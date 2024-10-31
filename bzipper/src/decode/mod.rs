// Copyright 2024 Gabriel Bjørnager Jensen.
//
// This file is part of bZipper.
//
// bZipper is free software: you can redistribute
// it and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// bZipper is distributed in the hope that it will
// be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Less-
// er General Public License along with bZipper. If
// not, see <https://www.gnu.org/licenses/>.

#[cfg(test)]
mod test;

use crate::{IStream, SizedEncode};
use crate::error::{DecodeError, Utf8Error};

use core::cell::{Cell, RefCell};
use core::convert::Infallible;
use core::hash::Hash;
use core::marker::{PhantomData, PhantomPinned};
use core::mem::MaybeUninit;
use core::net::{
	IpAddr,
	Ipv4Addr,
	Ipv6Addr,
	SocketAddr,
	SocketAddrV4,
	SocketAddrV6,
};
use core::num::{NonZero, Saturating, Wrapping};
use core::ops::{
	Bound,
	Range,
	RangeFrom,
	RangeFull,
	RangeInclusive,
	RangeTo,
	RangeToInclusive,
};

#[cfg(feature = "alloc")]
use std::boxed::Box;

#[cfg(feature = "alloc")]
use std::collections::LinkedList;

#[cfg(feature = "alloc")]
use alloc::string::String;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "alloc")]
use alloc::rc::Rc;

#[cfg(feature = "alloc")]
use alloc::sync::Arc;

#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};

#[cfg(feature = "std")]
use std::hash::BuildHasher;

#[cfg(feature = "std")]
use std::sync::{Mutex, RwLock};

// Should we require `Encode` for `Decode`?

/// Denotes a type capable of being decoded.
pub trait Decode: Sized {
	/// Decodes an object from the provided stream.
	///
	/// # Errors
	///
	/// If decoding fails due to e.g. an invalid byte sequence in the stream, then an error should be returned.
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError>;
}

/// Implemented for tuples with up to twelve members.
#[cfg_attr(doc, doc(fake_variadic))]
impl<T> Decode for (T, )
where
	T: Decode, {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let value = (Decode::decode(stream)?, );

		Ok(value)
	}
}

impl<T: Decode, const N: usize> Decode for [T; N] {
	#[inline]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		// Initialise the array incrementally.

 		let mut buf: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

		for item in &mut buf {
			let value = Decode::decode(stream)?;
			item.write(value);
		}

		// This should be safe as `MaybeUninit<T>` is
		// transparent to `T`, and we have initialised
		// every element. The original buffer is NOT
		// dropped automatically, so we can just forget
		// about it from this point on. `transmute` cannot
		// be used here, and `transmute_unchecked` is re-
		// served for the greedy rustc devs.
		let this = unsafe { buf.as_ptr().cast::<[T; N]>().read() };
		Ok(this)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: Decode> Decode for Arc<T> {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		Ok(Self::new(Decode::decode(stream)?))
	}
}

impl Decode for bool {
	#[inline]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let value = u8::decode(stream)?;

		match value {
			0x0 => Ok(false),
			0x1 => Ok(true),
			_   => Err(DecodeError::InvalidBoolean(value))
		}
	}
}

impl<T: Decode> Decode for Bound<T> {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let discriminant = u8::decode(stream)?;

		let this = match discriminant {
			0x0 => {
				let bound = Decode::decode(stream)?;
				Self::Included(bound)
			}

			0x1 => {
				let bound = Decode::decode(stream)?;
				Self::Excluded(bound)
			}

			0x2 => Self::Unbounded,

			_ => return Err(DecodeError::InvalidDiscriminant(discriminant.into())),
		};

		Ok(this)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: Decode> Decode for Box<T> {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let value = Decode::decode(stream)?;

		let this = Self::new(value);
		Ok(this)
	}
}

impl<T: Decode> Decode for Cell<T> {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let value = Decode::decode(stream)?;

		let this = Self::new(value);
		Ok(this)
	}
}

impl Decode for char {
	#[inline]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let value = u32::decode(stream)?;

		let this = value
			.try_into()
			.map_err(|_| DecodeError::InvalidCodePoint(value))?;

		Ok(this)
	}
}

#[cfg(feature = "std")]
#[cfg_attr(doc, doc(cfg(feature = "std")))]
impl<K, V, S> Decode for HashMap<K, V, S>
where
	K: Decode + Eq + Hash,
	V: Decode,
	S: BuildHasher + Default,
	{
	#[inline]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let len = Decode::decode(stream)?;

		let mut this = Self::with_capacity_and_hasher(len, Default::default());

		for _ in 0x0..len {
			let key   = Decode::decode(stream)?;
			let value = Decode::decode(stream)?;

			this.insert(key, value);
		}

		Ok(this)
	}
}

#[cfg(feature = "std")]
#[cfg_attr(doc, doc(cfg(feature = "std")))]
impl<T, S> Decode for HashSet<T, S>
where
	T: Decode + Eq + Hash,
	S: BuildHasher + Default,
	{
	#[inline]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let len = Decode::decode(stream)?;

		let mut this = Self::with_capacity_and_hasher(len, Default::default());

		for _ in 0x0..len {
			let key = Decode::decode(stream)?;

			this.insert(key);
		}

		Ok(this)
	}
}

impl Decode for Infallible {
	#[expect(clippy::panic_in_result_fn)]
	#[inline(always)]
	fn decode(_stream: &mut IStream) -> Result<Self, DecodeError> {
		panic!("cannot deserialise `Infallible` as it cannot be serialised to begin with")
	}
}

impl Decode for IpAddr {
	#[inline]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let discriminant = u8::decode(stream)?;

		let this = match discriminant {
			0x4 => Self::V4(Decode::decode(stream)?),
			0x6 => Self::V6(Decode::decode(stream)?),

			_ => return Err(DecodeError::InvalidDiscriminant(discriminant.into()))
		};

		Ok(this)
	}
}

impl Decode for Ipv4Addr {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let value = Decode::decode(stream)?;

		Ok(Self::from_bits(value))
	}
}

impl Decode for Ipv6Addr {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let value = Decode::decode(stream)?;

		Ok(Self::from_bits(value))
	}
}

impl Decode for isize {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let value = i16::decode(stream)?;
		Ok(value as Self)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: Decode> Decode for LinkedList<T> {
	#[inline]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let len = usize::decode(stream)?;

		let mut this = Self::new();

		for _ in 0x0..len {
			let value = T::decode(stream)?;

			this.push_back(value);
		}

		Ok(this)
	}
}

#[cfg(feature = "std")]
#[cfg_attr(doc, doc(cfg(feature = "std")))]
impl<T: Decode> Decode for Mutex<T> {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		Ok(Self::new(Decode::decode(stream)?))
	}
}

impl<T: Decode> Decode for Option<T> {
	#[expect(clippy::if_then_some_else_none)] // ???
	#[inline]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let sign = bool::decode(stream)?;

		let this = if sign {
			Some(Decode::decode(stream)?)
		} else {
			None
		};

		Ok(this)
	}
}

impl<T> Decode for PhantomData<T> {
	#[inline(always)]
	fn decode(_stream: &mut IStream) -> Result<Self, DecodeError> {
		Ok(Self)
	}
}

impl Decode for PhantomPinned {
	#[inline(always)]
	fn decode(_stream: &mut IStream) -> Result<Self, DecodeError> {
		Ok(Self)
	}
}

impl<T: Decode> Decode for Range<T> {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let start = Decode::decode(stream)?;
		let end   = Decode::decode(stream)?;

		Ok(start..end)
	}
}

impl<T: Decode> Decode for RangeFrom<T> {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let start = Decode::decode(stream)?;

		Ok(start..)
	}
}

impl Decode for RangeFull {
	#[inline(always)]
	fn decode(_stream: &mut IStream) -> Result<Self, DecodeError> {
		Ok(..)
	}
}

impl<T: Decode> Decode for RangeInclusive<T> {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let start = Decode::decode(stream)?;
		let end   = Decode::decode(stream)?;

		Ok(start..=end)
	}
}

impl<T: Decode> Decode for RangeTo<T> {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let end = Decode::decode(stream)?;

		Ok(..end)
	}
}

impl<T: Decode> Decode for RangeToInclusive<T> {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let end = Decode::decode(stream)?;

		Ok(..=end)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: Decode> Decode for Rc<T> {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		Ok(Self::new(Decode::decode(stream)?))
	}
}

impl<T: Decode> Decode for RefCell<T> {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let value = Decode::decode(stream)?;

		let this = Self::new(value);
		Ok(this)
	}
}

impl<T: Decode, E: Decode> Decode for core::result::Result<T, E> {
	#[inline]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let sign = bool::decode(stream)?;

		let this = if sign {
			Err(E::decode(stream)?)
		} else {
			Ok(Decode::decode(stream)?)
		};

		Ok(this)
	}
}

#[cfg(feature = "std")]
#[cfg_attr(doc, doc(cfg(feature = "std")))]
impl<T: Decode> Decode for RwLock<T> {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		Ok(Self::new(Decode::decode(stream)?))
	}
}

impl<T: Decode> Decode for Saturating<T> {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		Ok(Self(Decode::decode(stream)?))
	}
}

impl Decode for SocketAddr {
	#[inline]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let discriminant = u8::decode(stream)?;

		let this = match discriminant {
			0x4 => Self::V4(Decode::decode(stream)?),
			0x6 => Self::V6(Decode::decode(stream)?),

			_ => return Err(DecodeError::InvalidDiscriminant(discriminant.into()))
		};

		Ok(this)
	}
}

impl Decode for SocketAddrV4 {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let ip   = Decode::decode(stream)?;
		let port = Decode::decode(stream)?;

		let this = Self::new(ip, port);
		Ok(this)
	}
}

impl Decode for SocketAddrV6 {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let ip        = Decode::decode(stream)?;
		let port      = Decode::decode(stream)?;
		let flow_info = Decode::decode(stream)?;
		let scope_id  = Decode::decode(stream)?;

		let this = Self::new(ip, port, flow_info, scope_id);
		Ok(this)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl Decode for String {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let data = <Vec::<u8>>::decode(stream)?;

		Self::from_utf8(data)
			.map_err(|e| {
				let data = e.as_bytes();
				let i = e.utf8_error().valid_up_to();

				DecodeError::BadString(Utf8Error { value: data[i], index: i })
			})
	}
}

impl Decode for () {
	#[inline(always)]
	fn decode(_stream: &mut IStream) -> Result<Self, DecodeError> {
		Ok(())
	}
}

impl Decode for usize {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let value = u16::decode(stream)?;
		Ok(value as Self)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: Decode> Decode for Vec<T> {
	#[inline]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let len = Decode::decode(stream)?;

		let mut this = Self::with_capacity(len);

		let buf = this.as_mut_ptr();
		for i in 0x0..len {
			let value = Decode::decode(stream)?;

			// SAFETY: Each index is within bounds (i.e. capac-
			// ity).
			unsafe { buf.add(i).write(value) };
		}

		// SAFETY: We have initialised the buffer.
		unsafe { this.set_len(len); }

		Ok(this)
	}
}

impl<T: Decode> Decode for Wrapping<T> {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		Ok(Self(Decode::decode(stream)?))
	}
}

macro_rules! impl_numeric {
	($ty:ty$(,)?) => {
		impl ::bzipper::Decode for $ty {
			#[inline]
			fn decode(stream: &mut IStream) -> ::core::result::Result<Self, ::bzipper::error::DecodeError> {
				let data = stream
					.read(Self::MAX_ENCODED_SIZE)
					.try_into()
					.expect(concat!("mismatch between `", stringify!($ty), "::MAX_ENCODED_SIZE` and buffer needed by `", stringify!($ty), "::from_be_bytes`"));

				let this = Self::from_be_bytes(data);
				Ok(this)
			}
		}
	};
}

macro_rules! impl_tuple {
	{
		$($tys:ident),+$(,)?
	} => {
		#[doc(hidden)]
		impl<$($tys: ::bzipper::Decode, )*> ::bzipper::Decode for ($($tys, )*) {
			#[inline(always)]
			fn decode(stream: &mut ::bzipper::IStream) -> ::core::result::Result<Self, ::bzipper::error::DecodeError> {
				let this = (
					$( <$tys as ::bzipper::Decode>::decode(stream)?, )*
				);

				Ok(this)
			}
		}
	};
}

macro_rules! impl_non_zero {
	($ty:ty$(,)?) => {
		impl ::bzipper::Decode for NonZero<$ty> {
			#[inline]
			fn decode(stream: &mut IStream) -> ::core::result::Result<Self, ::bzipper::error::DecodeError> {
				let value = <$ty as ::bzipper::Decode>::decode(stream)?;

				let this = NonZero::new(value)
					.ok_or(::bzipper::error::DecodeError::NullInteger)?;

				Ok(this)
			}
		}
	};
}

macro_rules! impl_atomic {
	{
		width: $width:literal,
		ty: $ty:ty$(,)?
	} => {
		#[cfg(target_has_atomic = $width)]
		#[cfg_attr(doc, doc(cfg(target_has_atomic = $width)))]
		impl ::bzipper::Decode for $ty {
			#[inline(always)]
			fn decode(stream: &mut ::bzipper::IStream) -> ::core::result::Result<Self, ::bzipper::error::DecodeError> {
				Ok(Self::new(::bzipper::Decode::decode(stream)?))
			}
		}
	};
}

//impl_numeric!(f128);
//impl_numeric!(f16);
impl_numeric!(f32);
impl_numeric!(f64);
impl_numeric!(i128);
impl_numeric!(i16);
impl_numeric!(i32);
impl_numeric!(i64);
impl_numeric!(i8);
impl_numeric!(u128);
impl_numeric!(u16);
impl_numeric!(u32);
impl_numeric!(u64);
impl_numeric!(u8);

impl_tuple! {
	T0,
	T1,
}

impl_tuple! {
	T0,
	T1,
	T2,
}

impl_tuple! {
	T0,
	T1,
	T2,
	T3,
}

impl_tuple! {
	T0,
	T1,
	T2,
	T3,
	T4,
}

impl_tuple! {
	T0,
	T1,
	T2,
	T3,
	T4,
	T5,
}

impl_tuple! {
	T0,
	T1,
	T2,
	T3,
	T4,
	T5,
	T6,
}

impl_tuple! {
	T0,
	T1,
	T2,
	T3,
	T4,
	T5,
	T6,
	T7,
}

impl_tuple! {
	T0,
	T1,
	T2,
	T3,
	T4,
	T5,
	T6,
	T7,
	T8,
}

impl_tuple! {
	T0,
	T1,
	T2,
	T3,
	T4,
	T5,
	T6,
	T7,
	T8,
	T9,
}

impl_tuple! {
	 T0,
	 T1,
	 T2,
	 T3,
	 T4,
	 T5,
	 T6,
	 T7,
	 T8,
	 T9,
	 T10,
}

impl_tuple! {
	 T0,
	 T1,
	 T2,
	 T3,
	 T4,
	 T5,
	 T6,
	 T7,
	 T8,
	 T9,
	 T10,
	 T11,
}

impl_non_zero!(i128);
impl_non_zero!(i16);
impl_non_zero!(i32);
impl_non_zero!(i64);
impl_non_zero!(i8);
impl_non_zero!(isize);
impl_non_zero!(u128);
impl_non_zero!(u16);
impl_non_zero!(u32);
impl_non_zero!(u64);
impl_non_zero!(u8);
impl_non_zero!(usize);

impl_atomic! {
	width: "8",
	ty: std::sync::atomic::AtomicBool,
}

impl_atomic! {
	width: "16",
	ty: std::sync::atomic::AtomicI16,
}

impl_atomic! {
	width: "32",
	ty: std::sync::atomic::AtomicI32,
}

impl_atomic! {
	width: "64",
	ty: std::sync::atomic::AtomicI64,
}

impl_atomic! {
	width: "8",
	ty: std::sync::atomic::AtomicI8,
}

impl_atomic! {
	width: "ptr",
	ty: std::sync::atomic::AtomicIsize,
}

impl_atomic! {
	width: "16",
	ty: std::sync::atomic::AtomicU16,
}

impl_atomic! {
	width: "32",
	ty: std::sync::atomic::AtomicU32,
}

impl_atomic! {
	width: "64",
	ty: std::sync::atomic::AtomicU64,
}

impl_atomic! {
	width: "8",
	ty: std::sync::atomic::AtomicU8,
}

impl_atomic! {
	width: "ptr",
	ty: std::sync::atomic::AtomicUsize,
}
