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

/// Denotes a type capable of deserialisation.
pub trait Deserialise: Sized {
	/// Deserialises an object from the given d-stream.
	///
	/// This method must **never** read more bytes than specified by [`MAX_SERIALISED_SIZE`](crate::Serialise::MAX_SERIALISED_SIZE) (if [`Serialise`] is defined, that is).
	/// Doing so is considered a logic error.
	///
	/// # Errors
	///
	/// If deserialisation failed, e.g. by an illegal byte being found, an error is returned.
	///
	/// # Panics
	///
	/// This method will usually panic if the provided slice has a length *less* than the value of `MAX_SERIALISED_SIZE`.
	/// Official implementations of this trait (including those that are derived) always panic in debug mode if the provided slice has a length that is different at all.
	fn deserialise(stream: &Dstream) -> Result<Self>;
}

macro_rules! impl_numeric {
	($ty:ty) => {
		impl ::bzipper::Deserialise for $ty {
			#[inline]
			fn deserialise(stream: &Dstream) -> ::bzipper::Result<Self> {
				let data = stream
					.read(Self::MAX_SERIALISED_SIZE)
					.unwrap()
					//.ok_or(::bzipper::Error::EndOfStream { req: Self::MAX_SERIALISED_SIZE, rem: data.len() })?
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
			#[inline]
			fn deserialise(stream: &Dstream) -> ::bzipper::Result<Self> {
				let value = <$ty as ::bzipper::Deserialise>::deserialise(stream)?;

				let value = NonZero::new(value)
					.ok_or(Error::NullInteger)?;

				Ok(value)
			}
		}
	};
}

impl<T: Deserialise, const N: usize> Deserialise for [T; N] {
	#[inline]
	fn deserialise(stream: &Dstream) -> Result<Self> {
		// Initialise the array incrementally.

 		let mut buf: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

		for item in &mut buf {
			let value = T::deserialise(stream)?;
			item.write(value);
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
	#[inline]
	fn deserialise(stream: &Dstream) -> Result<Self> {
		let value = u8::deserialise(stream)?;

		match value {
			0x00 => Ok(false),
			0x01 => Ok(true),
			_    => Err(Error::InvalidBoolean(value))
		}
	}
}

impl Deserialise for char {
	#[inline]
	fn deserialise(stream: &Dstream) -> Result<Self> {
		let value = u32::deserialise(stream)?;

		let value = value
			.try_into()
			.map_err(|_| Error::InvalidCodePoint(value))?;

		Ok(value)
	}
}

impl Deserialise for Infallible {
	#[allow(clippy::panic_in_result_fn)]
	#[inline(always)]
	fn deserialise(_stream: &Dstream) -> Result<Self> { panic!("cannot deserialise `Infallible` as it cannot be serialised to begin with") }
}

impl Deserialise for isize {
	#[inline]
	fn deserialise(stream: &Dstream) -> Result<Self> {
		let value = i32::deserialise(stream)?;

		let value = value
			.try_into()
			.expect("unable to convert from `i32` to `isize`");

		Ok(value)
	}
}

impl<T: Deserialise> Deserialise for Option<T> {
	#[allow(clippy::if_then_some_else_none)]
	#[inline]
	fn deserialise(stream: &Dstream) -> Result<Self> {
		let sign = bool::deserialise(stream)?;

		let value = if sign {
			Some(T::deserialise(stream)?)
		} else {
			None
		};

		Ok(value)
	}
}

impl<T> Deserialise for PhantomData<T> {
	#[inline(always)]
	fn deserialise(_stream: &Dstream) -> Result<Self> { Ok(Self) }
}

impl<T: Deserialise, E: Deserialise> Deserialise for core::result::Result<T, E> {
	#[inline]
	fn deserialise(stream: &Dstream) -> Result<Self> {
		let sign = bool::deserialise(stream)?;

		let value = if sign {
			Err(E::deserialise(stream)?)
		} else {
			Ok(T::deserialise(stream)?)
		};

		Ok(value)
	}
}

impl Deserialise for () {
	#[inline(always)]
	fn deserialise(_stream: &Dstream) -> Result<Self> { Ok(()) }
}

impl Deserialise for usize {
	#[inline]
	fn deserialise(stream: &Dstream) -> Result<Self> {
		let value = u32::deserialise(stream)?;

		let value = value
			.try_into()
			.expect("must be able to convert from `u32` to `usize`");

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
