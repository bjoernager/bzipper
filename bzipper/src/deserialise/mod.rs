// Copyright 2024 Gabriel Bjørnager Jensen.
//
// This file is part of bzipper.
//
// bzipper is free software: you can redistribute
// it and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// bzipper is distributed in the hope that it will
// be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Less-
// er General Public License along with bzipper. If
// not, see <https://www.gnu.org/licenses/>.

#[cfg(test)]
mod test;

use crate::{Dstream, Error, Result, Serialise};

use core::convert::Infallible;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::num::NonZero;

mod tuple;

/// Types capable of being deserialised.
///
/// This trait requires [`Serialise`] also being implemented as it relies on the [`SERIALISED_SIZE`](crate::Serialise::SERIALISED_SIZE) constant.
pub trait Deserialise: Serialise + Sized {
	/// Deserialises a slice into an object.
	///
	/// This function must **never** take more bytes than specified by [`SERIALISED_SIZE`](crate::Serialise::SERIALISED_SIZE).
	/// Doing so is considered a logic error.
	/// Likewise, providing more than this amount is also disfavoured.
	///
	/// # Errors
	///
	/// If deserialisation failed, e.g. by an illegal byte being found, an error is returned.
	///
	/// # Panics
	///
	/// This method will usually panic if the provided slice has a length *less* than the value of `SERIALISED_SIZE`.
	/// Official implementations of this trait (including those that are derived) always panic in debug mode if the provided slice has a length that is different at all.
	fn deserialise(data: &[u8]) -> Result<Self>;
}

macro_rules! impl_numeric {
	($ty:ty) => {
		impl ::bzipper::Deserialise for $ty {
			fn deserialise(data: &[u8]) -> ::bzipper::Result<Self> {
				::core::debug_assert_eq!(data.len(), <Self as ::bzipper::Serialise>::SERIALISED_SIZE);

				const SIZE: usize = ::core::mem::size_of::<$ty>();

				let data = data
					.get(0x0..SIZE)
					.ok_or(::bzipper::Error::EndOfStream { req: SIZE, rem: data.len() })?
					.try_into()
					.unwrap();

				Ok(Self::from_be_bytes(data))
			}
		}
	};
}

macro_rules! impl_non_zero {
	($ty:ty) => {
		impl ::bzipper::Deserialise for NonZero<$ty> {
			fn deserialise(data: &[u8]) -> ::bzipper::Result<Self> {
				::core::debug_assert_eq!(data.len(), <Self as ::bzipper::Serialise>::SERIALISED_SIZE);

				let value = <$ty as ::bzipper::Deserialise>::deserialise(data)?;

				NonZero::new(value)
					.ok_or(Error::NullInteger)
			}
		}
	};
}

impl<T, const N: usize> Deserialise for [T; N]
where
	T: Deserialise {
	fn deserialise(data: &[u8]) -> Result<Self> {
		debug_assert_eq!(data.len(), Self::SERIALISED_SIZE);

		// Initialise the array incrementally.

 		let mut buf: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };
		let mut pos = 0x0;

		for item in &mut buf {
			let range = pos..pos + T::SERIALISED_SIZE;

			pos = range.end;
			item.write(Deserialise::deserialise(&data[range])?);
		}

		// This should be safe as `MaybeUninit<T>` is
		// transparent to `T`, and we have initialised
		// every element. The original buffer is NOT
		// dropped automatically, so we can just forget
		// about it from this point on. `transmute` cannot
		// be used here, and `transmute_unchecked` is re-
		// served for the greedy rustc devs.
		let value: [T; N] = unsafe { buf.as_ptr().cast::<[T; N]>().read() };
		Ok(value)
	}
}

impl Deserialise for bool {
	fn deserialise(data: &[u8]) -> Result<Self> {
		debug_assert_eq!(data.len(), Self::SERIALISED_SIZE);

		let value = u8::deserialise(data)?;

		match value {
			0x00 => Ok(false),
			0x01 => Ok(true),
			_    => Err(Error::InvalidBoolean { value })
		}
	}
}

impl Deserialise for char {
	fn deserialise(data: &[u8]) -> Result<Self> {
		debug_assert_eq!(data.len(), Self::SERIALISED_SIZE);

		let value = u32::deserialise(data)?;

		Self::from_u32(value)
			.ok_or(Error::InvalidCodePoint { value })
	}
}

impl Deserialise for Infallible {
	#[allow(clippy::panic_in_result_fn)]
	#[inline(always)]
	fn deserialise(_data: &[u8]) -> Result<Self> { panic!("cannot deserialise `Infallible` as it cannot be serialised to begin with") }
}

impl Deserialise for isize {
	fn deserialise(data: &[u8]) -> Result<Self> {
		debug_assert_eq!(data.len(), Self::SERIALISED_SIZE);

		let value = i32::deserialise(data)?
			.try_into().expect("unable to convert from `i32` to `isize`");

		Ok(value)
	}
}

impl<T: Deserialise> Deserialise for Option<T> {
	fn deserialise(data: &[u8]) -> Result<Self> {
		debug_assert_eq!(data.len(), Self::SERIALISED_SIZE);

		let stream = Dstream::new(data);

		let sign = stream.take::<bool>()?;

		if sign {
			Ok(Some(stream.take::<T>()?))
		} else {
			Ok(None)
		}
	}
}

impl<T> Deserialise for PhantomData<T> {
	fn deserialise(data: &[u8]) -> Result<Self> {
		debug_assert_eq!(data.len(), Self::SERIALISED_SIZE);

		Ok(Self)
	}
}

impl<T: Deserialise, E: Deserialise> Deserialise for core::result::Result<T, E> {
	fn deserialise(data: &[u8]) -> Result<Self> {
		debug_assert_eq!(data.len(), Self::SERIALISED_SIZE);

		let stream = Dstream::new(data);

		let sign = stream.take::<bool>()?;

		let value = if sign {
			Err(stream.take::<E>()?)
		} else {
			Ok(stream.take::<T>()?)
		};

		Ok(value)
	}
}

impl Deserialise for () {
	fn deserialise(_data: &[u8]) -> Result<Self> { Ok(()) }
}

impl Deserialise for usize {
	fn deserialise(data: &[u8]) -> Result<Self> {
		debug_assert_eq!(data.len(), Self::SERIALISED_SIZE);

		let value = u32::deserialise(data)?
			.try_into().expect("unable to convert from `u32` to `usize`");

		Ok(value)
	}
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
