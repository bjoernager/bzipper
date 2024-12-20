// Copyright 2024 Gabriel Bjørnager Jensen.
//
// This file is part of oct.
//
// oct is free software: you can redistribute it
// and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// oct is distributed in the hope that it will
// be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Less-
// er General Public License along with oct. If
// not, see <https://www.gnu.org/licenses/>.

#[cfg(test)]
mod tests;

use crate::decode::{DecodeBorrowed, Input};
use crate::error::{
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

	/// Decodes an object from the provided input.
	///
	/// # Errors
	///
	/// If decoding fails due to e.g. an invalid byte sequence in the input, then an error should be returned.
	fn decode(input: &mut Input) -> Result<Self, Self::Error>;
}

/// Implemented for tuples with up to twelve members.
#[cfg_attr(doc, doc(fake_variadic))]
impl<T: Decode> Decode for (T, ) {
	type Error = T::Error;

	#[inline(always)]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let this = (Decode::decode(input)?, );
		Result::Ok(this)
	}
}

impl<T: Decode, const N: usize> Decode for [T; N] {
	type Error = CollectionDecodeError<Infallible, ItemDecodeError<usize, T::Error>>;

	#[inline]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		// Initialise the array incrementally.

 		let mut buf = [const { MaybeUninit::<T>::uninit() }; N];

		for (i, item) in buf.iter_mut().enumerate() {
			let value = Decode::decode(input)
				.map_err(|e| CollectionDecodeError::BadItem(ItemDecodeError { index: i, error: e }))?;

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
		Result::Ok(this)
	}
}

#[cfg(all(feature = "alloc", target_has_atomic = "ptr"))]
#[cfg_attr(doc, doc(cfg(all(feature = "alloc", target_has_atomic = "ptr"))))]
impl<T: Decode> Decode for Arc<T> {
	type Error = T::Error;

	#[inline(always)]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let value = Decode::decode(input)?;

		let this = Self::new(value);
		Result::Ok(this)
	}
}

impl Decode for bool {
	type Error = Infallible;

	/// Lossily reinterprets a byte value as a boolean.
	///
	/// Whilst <code>[Encode](crate::encode::Encode)::[encode](crate::encode::Encode::encode)</code> will only yield the values `0` and `1`, this method clamps all values above `1`.
	#[inline]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(value) = u8::decode(input);

		let this = value != 0x0;
		Result::Ok(this)
	}
}

impl<T: Decode> Decode for Bound<T> {
	type Error = EnumDecodeError<u8, T::Error>;

	#[inline(always)]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(discriminant) = u8::decode(input);

		let this = match discriminant {
			0x0 => {
				let bound = Decode::decode(input)
					.map_err(EnumDecodeError::BadField)?;

				Self::Included(bound)
			}

			0x1 => {
				let bound = Decode::decode(input)
					.map_err(EnumDecodeError::BadField)?;

				Self::Excluded(bound)
			}

			0x2 => Self::Unbounded,

			value => return Err(EnumDecodeError::UnassignedDiscriminant { value }),
		};

		Result::Ok(this)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: Decode> Decode for Box<T> {
	type Error = T::Error;

	#[inline(always)]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let value = Decode::decode(input)?;

		let this = Self::new(value);
		Result::Ok(this)
	}
}

impl<T: Decode> Decode for Cell<T> {
	type Error = T::Error;

	#[inline(always)]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let value = Decode::decode(input)?;

		let this = Self::new(value);
		Result::Ok(this)
	}
}

impl Decode for char {
	type Error = CharDecodeError;

	#[inline]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(code_point) = u32::decode(input);

		match code_point {
			code_point @ (0x0000..=0xD7FF | 0xDE00..=0x10FFFF) => {
				let this = unsafe { Self::from_u32_unchecked(code_point) };
				Result::Ok(this)
			},

			code_point => Err(CharDecodeError { code_point }),
		}
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
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let value = Decode::decode(input)?;

		let this = Self::Owned(value);
		Result::Ok(this)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl Decode for CString {
	type Error = CStringDecodeError;

	#[inline(always)]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(len) = Decode::decode(input);

		let data = input.read(len).unwrap();

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
		Result::Ok(this)
	}
}

impl Decode for Duration {
	type Error = Infallible;

	#[inline(always)]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(secs)  = Decode::decode(input);
		let Ok(nanos) = Decode::decode(input);

		let this = Self::new(secs, nanos);
		Result::Ok(this)
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
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(len) = Decode::decode(input);

		let mut this = Self::with_capacity_and_hasher(len, Default::default());

		for i in 0x0..len {
			let key= Decode::decode(input)
				.map_err(|e| CollectionDecodeError::BadItem(ItemDecodeError { index: i, error: e }))?;

			let value = Decode::decode(input)
				.map_err(|e| CollectionDecodeError::BadItem(ItemDecodeError { index: i, error: e }))?;

			this.insert(key, value);
		}

		Result::Ok(this)
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
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(len) = Decode::decode(input);

		let mut this = Self::with_capacity_and_hasher(len, Default::default());

		for i in 0x0..len {
			let key = Decode::decode(input)
				.map_err(|e| CollectionDecodeError::BadItem(ItemDecodeError { index: i, error: e }) )?;

			this.insert(key);
		}

		Result::Ok(this)
	}
}

impl Decode for Infallible {
	type Error = Self;

	#[inline(always)]
	fn decode(_input: &mut Input) -> Result<Self, Self::Error> {
		panic!("cannot deserialise `Infallible` as it cannot be serialised to begin with")
	}
}

impl Decode for IpAddr {
	type Error = EnumDecodeError<u8, Infallible>;

	#[inline]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let discriminant = u8::decode(input)
			.map_err(EnumDecodeError::InvalidDiscriminant)?;

		let this = match discriminant {
			0x4 => Self::V4(Decode::decode(input).unwrap()),
			0x6 => Self::V6(Decode::decode(input).unwrap()),

			value => return Err(EnumDecodeError::UnassignedDiscriminant { value })
		};

		Result::Ok(this)
	}
}

impl Decode for Ipv4Addr {
	type Error = Infallible;

	#[inline(always)]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(value) = Decode::decode(input);

		let this = Self::from_bits(value);
		Result::Ok(this)
	}
}

impl Decode for Ipv6Addr {
	type Error = Infallible;

	#[inline(always)]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(value) = Decode::decode(input);

		let this = Self::from_bits(value);
		Result::Ok(this)
	}
}

impl Decode for isize {
	type Error = Infallible;

	#[inline(always)]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(value) = i16::decode(input);

		Result::Ok(value as Self)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: Decode> Decode for LinkedList<T> {
	type Error = CollectionDecodeError<Infallible, ItemDecodeError<usize, T::Error>>;

	#[inline]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(len) = usize::decode(input);

		let mut this = Self::new();

		for i in 0x0..len {
			let value = T::decode(input)
				.map_err(|e| CollectionDecodeError::BadItem(ItemDecodeError { index: i, error: e }))?;

			this.push_back(value);
		}

		Result::Ok(this)
	}
}

#[cfg(feature = "std")]
#[cfg_attr(doc, doc(cfg(feature = "std")))]
impl<T: Decode> Decode for Mutex<T> {
	type Error = T::Error;

	#[inline(always)]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let value = Decode::decode(input)?;

		let this = Self::new(value);
		Result::Ok(this)
	}
}

impl<T: Decode> Decode for Option<T> {
	type Error = T::Error;

	#[expect(clippy::if_then_some_else_none)] // ???
	#[inline]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let sign = bool::decode(input)
			.map_err::<T::Error, _>(|_e| unreachable!())?;

		let this = if sign {
			Some(Decode::decode(input)?)
		} else {
			None
		};

		Result::Ok(this)
	}
}

impl<T> Decode for PhantomData<T> {
	type Error = Infallible;

	#[inline(always)]
	fn decode(_input: &mut Input) -> Result<Self, Self::Error> {
		Result::Ok(Self)
	}
}

impl Decode for PhantomPinned {
	type Error = Infallible;

	#[inline(always)]
	fn decode(_input: &mut Input) -> Result<Self, Self::Error> {
		Result::Ok(Self)
	}
}

impl<T: Decode> Decode for Range<T> {
	type Error = T::Error;

	#[inline(always)]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let start = Decode::decode(input)?;
		let end   = Decode::decode(input)?;

		Result::Ok(start..end)
	}
}

impl<T: Decode> Decode for RangeFrom<T> {
	type Error = T::Error;

	#[inline(always)]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let start = Decode::decode(input)?;

		Result::Ok(start..)
	}
}

impl Decode for RangeFull {
	type Error = Infallible;

	#[inline(always)]
	fn decode(_input: &mut Input) -> Result<Self, Self::Error> {
		Result::Ok(..)
	}
}

impl<T: Decode> Decode for RangeInclusive<T> {
	type Error = T::Error;

	#[inline(always)]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let start = Decode::decode(input)?;
		let end   = Decode::decode(input)?;

		Result::Ok(start..=end)
	}
}

impl<T: Decode> Decode for RangeTo<T> {
	type Error = T::Error;

	#[inline(always)]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let end = Decode::decode(input)?;

		Result::Ok(..end)
	}
}

impl<T: Decode> Decode for RangeToInclusive<T> {
	type Error = T::Error;

	#[inline(always)]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let end = Decode::decode(input)?;

		Result::Ok(..=end)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: Decode> Decode for Rc<T> {
	type Error = T::Error;

	#[inline(always)]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		Result::Ok(Self::new(Decode::decode(input)?))
	}
}

impl<T: Decode> Decode for RefCell<T> {
	type Error = T::Error;

	#[inline(always)]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let value = Decode::decode(input)?;

		let this = Self::new(value);
		Result::Ok(this)
	}
}

impl<T, E, Err> Decode for Result<T, E>
where
	T: Decode<Error = Err>,
	E: Decode<Error = Err>,
{
	type Error = EnumDecodeError<bool, Err>;

	#[inline]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let sign = bool::decode(input)
			.map_err(EnumDecodeError::InvalidDiscriminant)?;

		let this = if sign {
			let value = Decode::decode(input)
				.map_err(EnumDecodeError::BadField)?;

			Err(value)
		} else {
			let value = Decode::decode(input)
				.map_err(EnumDecodeError::BadField)?;

			Ok(value)
		};

		Result::Ok(this)
	}
}

#[cfg(feature = "std")]
#[cfg_attr(doc, doc(cfg(feature = "std")))]
impl<T: Decode> Decode for RwLock<T> {
	type Error = T::Error;

	#[inline(always)]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let value = Decode::decode(input)?;

		let this = Self::new(value);
		Result::Ok(this)
	}
}

impl<T: Decode> Decode for Saturating<T> {
	type Error = T::Error;

	#[inline(always)]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let value = Decode::decode(input)?;

		let this = Self(value);
		Result::Ok(this)
	}
}

impl Decode for SocketAddr {
	type Error = EnumDecodeError<u8, Infallible>;

	#[inline]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(discriminant) = u8::decode(input);

		match discriminant {
			0x4 => Result::Ok(Self::V4(Decode::decode(input).unwrap())),
			0x6 => Result::Ok(Self::V6(Decode::decode(input).unwrap())),

			value => Err(EnumDecodeError::UnassignedDiscriminant { value }),
		}
	}
}

impl Decode for SocketAddrV4 {
	type Error = Infallible;

	#[inline(always)]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let ip   = Decode::decode(input)?;
		let port = Decode::decode(input)?;

		let this = Self::new(ip, port);
		Result::Ok(this)
	}
}

impl Decode for SocketAddrV6 {
	type Error = Infallible;

	#[inline(always)]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let ip        = Decode::decode(input)?;
		let port      = Decode::decode(input)?;
		let flow_info = Decode::decode(input)?;
		let scope_id  = Decode::decode(input)?;

		let this = Self::new(ip, port, flow_info, scope_id);
		Result::Ok(this)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl Decode for String {
	type Error = CollectionDecodeError<Infallible, Utf8Error>;

	#[inline(always)]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(len) = Decode::decode(input);

		let data = input.read(len).unwrap();

		if let Err(e) = str::from_utf8(data) {
			let i = e.valid_up_to();
			let c = data[i];

			return Err(
				CollectionDecodeError::BadItem(
					Utf8Error { value: c, index: i },
				),
			);
		};

		let mut v = Vec::with_capacity(len);

		unsafe {
			let src = data.as_ptr();
			let dst = v.as_mut_ptr();

			copy_nonoverlapping(src, dst, len);
			v.set_len(len);
		}

		// SAFETY: We have already tested the raw data.
		let this = unsafe { Self::from_utf8_unchecked(v) };
		Result::Ok(this)
	}
}

#[cfg(feature = "std")]
#[cfg_attr(doc, doc(cfg(feature = "std")))]
impl Decode for SystemTime {
	type Error = SystemTimeDecodeError;

	#[inline]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(time) = i64::decode(input);

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
	fn decode(_input: &mut Input) -> Result<Self, Self::Error> {
		Result::Ok(())
	}
}

impl Decode for usize {
	type Error = Infallible;

	#[inline(always)]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let value = u16::decode(input)?;
		Result::Ok(value as Self)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: Decode> Decode for Vec<T> {
	type Error = CollectionDecodeError<Infallible, ItemDecodeError<usize, T::Error>>;

	#[inline]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(len) = Decode::decode(input);

		let mut this = Self::with_capacity(len);

		let buf = this.as_mut_ptr();
		for i in 0x0..len {
			let value = Decode::decode(input)
				.map_err(|e| CollectionDecodeError::BadItem(ItemDecodeError { index: i, error: e }))?;

			// SAFETY: Each index is within bounds (i.e. capac-
			// ity).
			unsafe { buf.add(i).write(value) };
		}

		// SAFETY: We have initialised the buffer.
		unsafe { this.set_len(len); }

		Result::Ok(this)
	}
}

impl<T: Decode> Decode for Wrapping<T> {
	type Error = T::Error;

	#[inline(always)]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let value = Decode::decode(input)?;

		let this = Self(value);
		Result::Ok(this)
	}
}

macro_rules! impl_numeric {
	($ty:ty$(,)?) => {
		impl ::oct::decode::Decode for $ty {
			type Error = ::core::convert::Infallible;

			#[inline]
			fn decode(input: &mut ::oct::decode::Input) -> ::core::result::Result<Self, Self::Error> {
				let mut data = [::core::default::Default::default(); <Self as ::oct::encode::SizedEncode>::MAX_ENCODED_SIZE];
				input.read_into(&mut data).unwrap();

				let this = Self::from_le_bytes(data);
				::core::result::Result::Ok(this)
			}
		}
	};
}

macro_rules! impl_tuple {
	{
		$($tys:ident),+$(,)?
	} => {
		#[doc(hidden)]
		impl<$($tys, )* E> ::oct::decode::Decode for ($($tys, )*)
		where
			$($tys: Decode<Error = E>, )*
		{
			type Error = E;

			#[inline(always)]
			fn decode(input: &mut ::oct::decode::Input) -> ::core::result::Result<Self, Self::Error> {
				let this = (
					$( <$tys as ::oct::decode::Decode>::decode(input)?, )*
				);

				::core::result::Result::Ok(this)
			}
		}
	};
}

macro_rules! impl_non_zero {
	($ty:ty$(,)?) => {
		impl ::oct::decode::Decode for ::core::num::NonZero<$ty> {
			type Error = ::oct::error::NonZeroDecodeError;

			#[inline]
			fn decode(input: &mut ::oct::decode::Input) -> ::core::result::Result<Self, Self::Error> {
				use ::core::result::Result;

				let Result::Ok(value) = <$ty as ::oct::decode::Decode>::decode(input);

				match value {
					0x0 => Result::Err(::oct::error::NonZeroDecodeError),

					value => {
						let this = unsafe { ::core::num::NonZero::new_unchecked(value) };
						Result::Ok(this)
					},
				}
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
		impl ::oct::decode::Decode for $atomic_ty {
			type Error = <$ty as ::oct::decode::Decode>::Error;

			#[inline(always)]
			fn decode(input: &mut ::oct::decode::Input) -> ::core::result::Result<Self, Self::Error> {
				let value = ::oct::decode::Decode::decode(input)?;

				let this = Self::new(value);
				::core::result::Result::Ok(this)
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
