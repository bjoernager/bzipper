// Copyright 2024 Gabriel Bjørnager Jensen.
//
// This file is part of Librum.
//
// Librum is free software: you can redistribute it
// and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// Librum is distributed in the hope that it will
// be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Less-
// er General Public License along with Librum. If
// not, see <https://www.gnu.org/licenses/>.

#[cfg(test)]
mod tests;

use crate::{DecodeBorrowed, IStream, SizedEncode};
use crate::error::{
	BoolDecodeError,
	CStringDecodeError,
	CharDecodeError,
	CollectionDecodeError,
	EnumDecodeError,
	ItemDecodeError,
	SystemTimeDecodeError,
	Utf8Error,
};

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
use core::num::{Saturating, Wrapping};
use core::ops::{
	Bound,
	Range,
	RangeFrom,
	RangeFull,
	RangeInclusive,
	RangeTo,
	RangeToInclusive,
};
use core::ptr::copy_nonoverlapping;
use core::str;
use core::time::Duration;

#[cfg(feature = "alloc")]
use alloc::borrow::{Cow, ToOwned};

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

#[cfg(feature = "alloc")]
use alloc::collections::LinkedList;

#[cfg(feature = "alloc")]
use alloc::ffi::CString;

#[cfg(feature = "alloc")]
use alloc::string::String;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "alloc")]
use alloc::rc::Rc;

#[cfg(all(feature = "alloc", target_has_atomic = "ptr"))]
use alloc::sync::Arc;

#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};

#[cfg(feature = "std")]
use std::hash::BuildHasher;

#[cfg(feature = "std")]
use std::sync::{Mutex, RwLock};

#[cfg(feature = "std")]
use std::time::{SystemTime, UNIX_EPOCH};

// Should we require `Encode` for `Decode`?

/// Denotes a type capable of being decoded.
pub trait Decode: Sized {
	/// The type returned in case of error.
	type Error;

	/// Decodes an object from the provided stream.
	///
	/// # Errors
	///
	/// If decoding fails due to e.g. an invalid byte sequence in the stream, then an error should be returned.
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error>;
}

/// Implemented for tuples with up to twelve members.
#[cfg_attr(doc, doc(fake_variadic))]
impl<T: Decode> Decode for (T, ) {
	type Error = T::Error;

	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let this = (Decode::decode(stream)?, );
		Ok(this)
	}
}

impl<T: Decode, const N: usize> Decode for [T; N] {
	type Error = CollectionDecodeError<Infallible, ItemDecodeError<usize, T::Error>>;

	#[inline]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		// Initialise the array incrementally.

 		let mut buf: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

		for (i, item) in buf.iter_mut().enumerate() {
			let value = Decode::decode(stream)
				.map_err(|e| CollectionDecodeError::Item(ItemDecodeError { index: i, error: e }))?;

			item.write(value);
		}

		// SAFETY: This should be safe as `MaybeUninit<T>`
		// is transparent to `T` and we have initialised
		// every element. The original buffer is NOT
		// dropped automatically, so we can just forget
		// about it from this point on. `transmute` cannot
		// be used here, and `transmute_unchecked` is re-
		// served for the greedy rustc devs.
		let this = unsafe { buf.as_ptr().cast::<[T; N]>().read() };
		Ok(this)
	}
}

#[cfg(all(feature = "alloc", target_has_atomic = "ptr"))]
#[cfg_attr(doc, doc(cfg(all(feature = "alloc", target_has_atomic = "ptr"))))]
impl<T: Decode> Decode for Arc<T> {
	type Error = T::Error;

	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let value = Decode::decode(stream)?;

		let this = Self::new(value);
		Ok(this)
	}
}

impl Decode for bool {
	type Error = BoolDecodeError;

	#[inline]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let value = u8::decode(stream).unwrap();

		match value {
			0x0 => Ok(false),
			0x1 => Ok(true),
			_   => Err(BoolDecodeError { value })
		}
	}
}

impl<T: Decode> Decode for Bound<T> {
	type Error = EnumDecodeError<u8, T::Error>;

	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let discriminant = u8::decode(stream).unwrap();

		let this = match discriminant {
			0x0 => {
				let bound = Decode::decode(stream)
					.map_err(EnumDecodeError::Field)?;

				Self::Included(bound)
			}

			0x1 => {
				let bound = Decode::decode(stream)
					.map_err(EnumDecodeError::Field)?;

				Self::Excluded(bound)
			}

			0x2 => Self::Unbounded,

			value => return Err(EnumDecodeError::UnassignedDiscriminant { value }),
		};

		Ok(this)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: Decode> Decode for Box<T> {
	type Error = T::Error;

	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let value = Decode::decode(stream)?;

		let this = Self::new(value);
		Ok(this)
	}
}

impl<T: Decode> Decode for Cell<T> {
	type Error = T::Error;

	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let value = Decode::decode(stream)?;

		let this = Self::new(value);
		Ok(this)
	}
}

impl Decode for char {
	type Error = CharDecodeError;

	#[inline]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let code_point = u32::decode(stream).unwrap();

		let this = code_point
			.try_into()
			.map_err(|_| CharDecodeError { code_point })?;

		Ok(this)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T, B> Decode for Cow<'_, B>
where
	T: DecodeBorrowed<B>,
	B: ToOwned<Owned = T>,
{
	type Error = T::Error;

	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let value = Decode::decode(stream)?;

		let this = Self::Owned(value);
		Ok(this)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl Decode for CString {
	type Error = CStringDecodeError;

	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let len = Decode::decode(stream).unwrap();

		let data = stream.read(len);

		for (i, c) in data.iter().enumerate() {
			if *c == b'\x00' { return Err(CStringDecodeError { index: i }) };
		}

		let mut buf = Vec::with_capacity(len);

		unsafe {
			let src = data.as_ptr();
			let dst = buf.as_mut_ptr();

			copy_nonoverlapping(src, dst, len);
			buf.set_len(len);
		}

		// SAFETY: We have already tested the data.
		let this = unsafe { Self::from_vec_unchecked(buf) };
		Ok(this)
	}
}

impl Decode for Duration {
	type Error = Infallible;

	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let secs  = Decode::decode(stream)?;
		let nanos = Decode::decode(stream)?;

		let this = Self::new(secs, nanos);
		Ok(this)
	}
}

#[cfg(feature = "std")]
#[cfg_attr(doc, doc(cfg(feature = "std")))]
impl<K, V, S, E> Decode for HashMap<K, V, S>
where
	K: Decode<Error = E> + Eq + Hash,
	V: Decode<Error = E>,
	S: BuildHasher + Default,
{
	type Error = CollectionDecodeError<Infallible, ItemDecodeError<usize, E>>;

	#[inline]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let len = Decode::decode(stream).unwrap();

		let mut this = Self::with_capacity_and_hasher(len, Default::default());

		for i in 0x0..len {
			let key= Decode::decode(stream)
				.map_err(|e| CollectionDecodeError::Item(ItemDecodeError { index: i, error: e }))?;

			let value = Decode::decode(stream)
				.map_err(|e| CollectionDecodeError::Item(ItemDecodeError { index: i, error: e }))?;

			this.insert(key, value);
		}

		Ok(this)
	}
}

#[cfg(feature = "std")]
#[cfg_attr(doc, doc(cfg(feature = "std")))]
impl<K, S> Decode for HashSet<K, S>
where
	K: Decode + Eq + Hash,
	S: BuildHasher + Default,
{
	type Error = CollectionDecodeError<Infallible, ItemDecodeError<usize, K::Error>>;

	#[inline]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let len = Decode::decode(stream).unwrap();

		let mut this = Self::with_capacity_and_hasher(len, Default::default());

		for i in 0x0..len {
			let key = Decode::decode(stream)
				.map_err(|e| CollectionDecodeError::Item(ItemDecodeError { index: i, error: e }) )?;

			this.insert(key);
		}

		Ok(this)
	}
}

impl Decode for Infallible {
	type Error = Self;

	#[inline(always)]
	fn decode(_stream: &mut IStream) -> Result<Self, Self::Error> {
		panic!("cannot deserialise `Infallible` as it cannot be serialised to begin with")
	}
}

impl Decode for IpAddr {
	type Error = EnumDecodeError<u8, Infallible>;

	#[inline]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let discriminant = u8::decode(stream)
			.map_err(EnumDecodeError::InvalidDiscriminant)?;

		let this = match discriminant {
			0x4 => Self::V4(Decode::decode(stream).unwrap()),
			0x6 => Self::V6(Decode::decode(stream).unwrap()),

			value => return Err(EnumDecodeError::UnassignedDiscriminant { value })
		};

		Ok(this)
	}
}

impl Decode for Ipv4Addr {
	type Error = Infallible;

	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let value = Decode::decode(stream)?;
		Ok(Self::from_bits(value))
	}
}

impl Decode for Ipv6Addr {
	type Error = Infallible;

	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let value = Decode::decode(stream)?;
		Ok(Self::from_bits(value))
	}
}

impl Decode for isize {
	type Error = Infallible;

	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let value = i16::decode(stream)?;
		Ok(value as Self)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: Decode> Decode for LinkedList<T> {
	type Error = CollectionDecodeError<Infallible, ItemDecodeError<usize, T::Error>>;

	#[inline]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let len = usize::decode(stream).unwrap();

		let mut this = Self::new();

		for i in 0x0..len {
			let value = T::decode(stream)
				.map_err(|e| CollectionDecodeError::Item(ItemDecodeError { index: i, error: e }))?;

			this.push_back(value);
		}

		Ok(this)
	}
}

#[cfg(feature = "std")]
#[cfg_attr(doc, doc(cfg(feature = "std")))]
impl<T: Decode> Decode for Mutex<T> {
	type Error = T::Error;

	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		Ok(Self::new(Decode::decode(stream)?))
	}
}

impl<T: Decode> Decode for Option<T> {
	type Error = T::Error;

	#[expect(clippy::if_then_some_else_none)] // ???
	#[inline]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let sign = bool::decode(stream).unwrap();

		let this = if sign {
			Some(Decode::decode(stream)?)
		} else {
			None
		};

		Ok(this)
	}
}

impl<T> Decode for PhantomData<T> {
	type Error = Infallible;

	#[inline(always)]
	fn decode(_stream: &mut IStream) -> Result<Self, Self::Error> {
		Ok(Self)
	}
}

impl Decode for PhantomPinned {
	type Error = Infallible;

	#[inline(always)]
	fn decode(_stream: &mut IStream) -> Result<Self, Self::Error> {
		Ok(Self)
	}
}

impl<T: Decode> Decode for Range<T> {
	type Error = T::Error;

	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let start = Decode::decode(stream)?;
		let end   = Decode::decode(stream)?;

		Ok(start..end)
	}
}

impl<T: Decode> Decode for RangeFrom<T> {
	type Error = T::Error;

	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let start = Decode::decode(stream)?;

		Ok(start..)
	}
}

impl Decode for RangeFull {
	type Error = Infallible;

	#[inline(always)]
	fn decode(_stream: &mut IStream) -> Result<Self, Self::Error> {
		Ok(..)
	}
}

impl<T: Decode> Decode for RangeInclusive<T> {
	type Error = T::Error;

	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let start = Decode::decode(stream)?;
		let end   = Decode::decode(stream)?;

		Ok(start..=end)
	}
}

impl<T: Decode> Decode for RangeTo<T> {
	type Error = T::Error;

	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let end = Decode::decode(stream)?;

		Ok(..end)
	}
}

impl<T: Decode> Decode for RangeToInclusive<T> {
	type Error = T::Error;

	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let end = Decode::decode(stream)?;

		Ok(..=end)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: Decode> Decode for Rc<T> {
	type Error = T::Error;

	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		Ok(Self::new(Decode::decode(stream)?))
	}
}

impl<T: Decode> Decode for RefCell<T> {
	type Error = T::Error;

	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let value = Decode::decode(stream)?;

		let this = Self::new(value);
		Ok(this)
	}
}

impl<T, E, Err> Decode for core::result::Result<T, E>
where
	T: Decode<Error = Err>,
	E: Decode<Error = Err>,
{
	type Error = EnumDecodeError<bool, Err>;

	#[inline]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let sign = bool::decode(stream)
			.map_err(EnumDecodeError::InvalidDiscriminant)?;

		let this = if sign {
			let value = Decode::decode(stream)
				.map_err(EnumDecodeError::Field)?;

			Err(value)
		} else {
			let value = Decode::decode(stream)
				.map_err(EnumDecodeError::Field)?;

			Ok(value)
		};

		Ok(this)
	}
}

#[cfg(feature = "std")]
#[cfg_attr(doc, doc(cfg(feature = "std")))]
impl<T: Decode> Decode for RwLock<T> {
	type Error = T::Error;

	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let value = Decode::decode(stream)?;

		let this = Self::new(value);
		Ok(this)
	}
}

impl<T: Decode> Decode for Saturating<T> {
	type Error = T::Error;

	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let value = Decode::decode(stream)?;

		let this = Self(value);
		Ok(this)
	}
}

impl Decode for SocketAddr {
	type Error = EnumDecodeError<u8, Infallible>;

	#[inline]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let discriminant = u8::decode(stream).unwrap();

		let this = match discriminant {
			0x4 => Self::V4(Decode::decode(stream).unwrap()),
			0x6 => Self::V6(Decode::decode(stream).unwrap()),

			value => return Err(EnumDecodeError::UnassignedDiscriminant { value })
		};

		Ok(this)
	}
}

impl Decode for SocketAddrV4 {
	type Error = Infallible;

	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let ip   = Decode::decode(stream)?;
		let port = Decode::decode(stream)?;

		let this = Self::new(ip, port);
		Ok(this)
	}
}

impl Decode for SocketAddrV6 {
	type Error = Infallible;

	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
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
	type Error = CollectionDecodeError<Infallible, Utf8Error>;

	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let len = Decode::decode(stream).unwrap();

		let data = stream.read(len);

		str::from_utf8(data)
			.map_err(|e| {
				let i = e.valid_up_to();
				let c = data[i];

				CollectionDecodeError::Item(Utf8Error { value: c, index: i })
			})?;

		let mut v = Vec::with_capacity(len);

		unsafe {
			let src = data.as_ptr();
			let dst = v.as_mut_ptr();

			copy_nonoverlapping(src, dst, len);
			v.set_len(len);
		}

		// SAFETY: We have already tested the raw data.
		let this = unsafe { Self::from_utf8_unchecked(v) };
		Ok(this)
	}
}

#[cfg(feature = "std")]
#[cfg_attr(doc, doc(cfg(feature = "std")))]
impl Decode for SystemTime {
	type Error = SystemTimeDecodeError;

	#[inline]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let time = i64::decode(stream).unwrap();

		let this = if time.is_positive() {
			let time = time as u64;

			UNIX_EPOCH.checked_add(Duration::from_secs(time))
		} else {
			let time = time.unsigned_abs();

			UNIX_EPOCH.checked_sub(Duration::from_secs(time))
		};

		this.ok_or(SystemTimeDecodeError { timestamp: time })
	}
}

impl Decode for () {
	type Error = Infallible;

	#[inline(always)]
	fn decode(_stream: &mut IStream) -> Result<Self, Self::Error> {
		Ok(())
	}
}

impl Decode for usize {
	type Error = Infallible;

	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let value = u16::decode(stream)?;
		Ok(value as Self)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: Decode> Decode for Vec<T> {
	type Error = CollectionDecodeError<Infallible, ItemDecodeError<usize, T::Error>>;

	#[inline]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let len = Decode::decode(stream).unwrap();

		let mut this = Self::with_capacity(len);

		let buf = this.as_mut_ptr();
		for i in 0x0..len {
			let value = Decode::decode(stream)
				.map_err(|e| CollectionDecodeError::Item(ItemDecodeError { index: i, error: e }))?;

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
	type Error = T::Error;

	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let value = Decode::decode(stream)?;

		let this = Self(value);
		Ok(this)
	}
}

macro_rules! impl_numeric {
	($ty:ty$(,)?) => {
		impl ::librum::Decode for $ty {
			type Error = ::core::convert::Infallible;

			#[inline]
			fn decode(stream: &mut ::librum::IStream) -> ::core::result::Result<Self, Self::Error> {
				let mut data = [::core::default::Default::default(); Self::MAX_ENCODED_SIZE];
				stream.read_into(&mut data);

				let this = Self::from_le_bytes(data);
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
		impl<$($tys, )* E> ::librum::Decode for ($($tys, )*)
		where
			$($tys: Decode<Error = E>, )*
		{
			type Error = E;

			#[inline(always)]
			fn decode(stream: &mut ::librum::IStream) -> ::core::result::Result<Self, Self::Error> {
				let this = (
					$( <$tys as ::librum::Decode>::decode(stream)?, )*
				);

				Ok(this)
			}
		}
	};
}

macro_rules! impl_non_zero {
	($ty:ty$(,)?) => {
		impl ::librum::Decode for ::core::num::NonZero<$ty> {
			type Error = ::librum::error::NonZeroDecodeError;

			#[inline]
			fn decode(stream: &mut ::librum::IStream) -> ::core::result::Result<Self, Self::Error> {
				let Ok(value) = <$ty as ::librum::Decode>::decode(stream);

				let this = ::core::num::NonZero::new(value)
					.ok_or(::librum::error::NonZeroDecodeError)?;

				Ok(this)
			}
		}
	};
}

macro_rules! impl_atomic {
	{
		width: $width:literal,
		ty: $ty:ty,
		atomic_ty: $atomic_ty:ty$(,)?
	} => {
		#[cfg(target_has_atomic = $width)]
		#[cfg_attr(doc, doc(cfg(target_has_atomic = $width)))]
		impl ::librum::Decode for $atomic_ty {
			type Error = <$ty as ::librum::Decode>::Error;

			#[inline(always)]
			fn decode(stream: &mut ::librum::IStream) -> ::core::result::Result<Self, Self::Error> {
				let value = ::librum::Decode::decode(stream)?;

				let this = Self::new(value);
				Ok(this)
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
	ty: bool,
	atomic_ty: std::sync::atomic::AtomicBool,
}

impl_atomic! {
	width: "16",
	ty: i16,
	atomic_ty: std::sync::atomic::AtomicI16,
}

impl_atomic! {
	width: "32",
	ty: i32,
	atomic_ty: std::sync::atomic::AtomicI32,
}

impl_atomic! {
	width: "64",
	ty: i64,
	atomic_ty: std::sync::atomic::AtomicI64,
}

impl_atomic! {
	width: "8",
	ty: i8,
	atomic_ty: std::sync::atomic::AtomicI8,
}

impl_atomic! {
	width: "ptr",
	ty: isize,
	atomic_ty: std::sync::atomic::AtomicIsize,
}

impl_atomic! {
	width: "16",
	ty: u16,
	atomic_ty: std::sync::atomic::AtomicU16,
}

impl_atomic! {
	width: "32",
	ty: u32,
	atomic_ty: std::sync::atomic::AtomicU32,
}

impl_atomic! {
	width: "64",
	ty: u64,
	atomic_ty: std::sync::atomic::AtomicU64,
}

impl_atomic! {
	width: "8",
	ty: u8,
	atomic_ty: std::sync::atomic::AtomicU8,
}

impl_atomic! {
	width: "ptr",
	ty: usize,
	atomic_ty: std::sync::atomic::AtomicUsize,
}
