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

use crate::{Error, Dstream};

use alloc::boxed::Box;
use core::convert::Infallible;
use core::error::Error as StdError;
use core::mem::{MaybeUninit, size_of};
use core::num::NonZero;
use core::ptr::read;

/// Types capable of being deserialised.
pub trait Deserialise: Sized {
	/// The error of deserialisation.
	///
	/// Use [`Infallible`] if **all** deserialisations are infallible, as is the case of zero-length types.
	type Error;

	/// Deserialises the byte stream to an object.
	///
	/// This function should **not** take more bytes than specified by [`T::SERIALISE_LIMIT`](crate::Serialise::SERIALISE_LIMIT).
	/// Doing so is considered a logic error.
	///
	/// # Errors
	///
	/// If deserialisation failed, e.g. by an invalid value being found, an error is returned.
	fn deserialise(stream: &mut Dstream) -> Result<Self, Self::Error>;
}

macro_rules! impl_float {
	($type:ty) => {
		impl Deserialise for $type {
			type Error = Error;

			fn deserialise(stream: &mut Dstream) -> Result<Self, Self::Error> {
				let data = stream
					.take(size_of::<Self>())?
					.try_into()
					.unwrap();

				Ok(Self::from_be_bytes(data))
			}
		}
	};
}

macro_rules! impl_int {
	($type:ty) => {
		impl Deserialise for $type {
			type Error = Error;

			fn deserialise(stream: &mut Dstream) -> Result<Self, Self::Error> {
				let data = stream
					.take(size_of::<Self>())?
					.try_into()
					.unwrap();

				Ok(Self::from_be_bytes(data))
			}
		}
	};
}

macro_rules! impl_non_zero {
	($type:ty) => {
		impl Deserialise for NonZero<$type> {
			type Error = Error;

			fn deserialise(stream: &mut Dstream) -> Result<Self, Self::Error> {
				let value = <$type>::deserialise(stream)?;

				NonZero::new(value)
					.ok_or(Error::NullInteger)
			}
		}
	};
}

impl<T0, T1> Deserialise for (T0, T1)
where
	T0: Deserialise<Error: StdError + 'static>,
	T1: Deserialise<Error: StdError + 'static>, {
	type Error = Box<dyn StdError>;

	fn deserialise(stream: &mut Dstream) -> Result<Self, Self::Error> {
		Ok((
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
		))
	}
}

impl<T0, T1, T2> Deserialise for (T0, T1, T2)
where
	T0: Deserialise<Error: StdError + 'static>,
	T1: Deserialise<Error: StdError + 'static>,
	T2: Deserialise<Error: StdError + 'static>, {
	type Error = Box<dyn StdError>;

	fn deserialise(stream: &mut Dstream) -> Result<Self, Self::Error> {
		Ok((
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
		))
	}
}

impl<T0, T1, T2, T3> Deserialise for (T0, T1, T2, T3)
where
	T0: Deserialise<Error: StdError + 'static>,
	T1: Deserialise<Error: StdError + 'static>,
	T2: Deserialise<Error: StdError + 'static>,
	T3: Deserialise<Error: StdError + 'static>, {
	type Error = Box<dyn StdError>;

	fn deserialise(stream: &mut Dstream) -> Result<Self, Self::Error> {
		Ok((
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
		))
	}
}

impl<T0, T1, T2, T3, T4> Deserialise for (T0, T1, T2, T3, T4)
where
	T0: Deserialise<Error: StdError + 'static>,
	T1: Deserialise<Error: StdError + 'static>,
	T2: Deserialise<Error: StdError + 'static>,
	T3: Deserialise<Error: StdError + 'static>,
	T4: Deserialise<Error: StdError + 'static>, {
	type Error = Box<dyn StdError>;

	fn deserialise(stream: &mut Dstream) -> Result<Self, Self::Error> {
		Ok((
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
		))
	}
}

impl<T0, T1, T2, T3, T4, T5> Deserialise for (T0, T1, T2, T3, T4, T5)
where
	T0: Deserialise<Error: StdError + 'static>,
	T1: Deserialise<Error: StdError + 'static>,
	T2: Deserialise<Error: StdError + 'static>,
	T3: Deserialise<Error: StdError + 'static>,
	T4: Deserialise<Error: StdError + 'static>,
	T5: Deserialise<Error: StdError + 'static>, {
	type Error = Box<dyn StdError>;

	fn deserialise(stream: &mut Dstream) -> Result<Self, Self::Error> {
		Ok((
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
		))
	}
}

impl<T0, T1, T2, T3, T4, T5, T6> Deserialise for (T0, T1, T2, T3, T4, T5, T6)
where
	T0: Deserialise<Error: StdError + 'static>,
	T1: Deserialise<Error: StdError + 'static>,
	T2: Deserialise<Error: StdError + 'static>,
	T3: Deserialise<Error: StdError + 'static>,
	T4: Deserialise<Error: StdError + 'static>,
	T5: Deserialise<Error: StdError + 'static>,
	T6: Deserialise<Error: StdError + 'static>, {
	type Error = Box<dyn StdError>;

	fn deserialise(stream: &mut Dstream) -> Result<Self, Self::Error> {
		Ok((
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
		))
	}
}

impl<T0, T1, T2, T3, T4, T5, T6, T7> Deserialise for (T0, T1, T2, T3, T4, T5, T6, T7)
where
	T0: Deserialise<Error: StdError + 'static>,
	T1: Deserialise<Error: StdError + 'static>,
	T2: Deserialise<Error: StdError + 'static>,
	T3: Deserialise<Error: StdError + 'static>,
	T4: Deserialise<Error: StdError + 'static>,
	T5: Deserialise<Error: StdError + 'static>,
	T6: Deserialise<Error: StdError + 'static>,
	T7: Deserialise<Error: StdError + 'static>, {
	type Error = Box<dyn StdError>;

	fn deserialise(stream: &mut Dstream) -> Result<Self, Self::Error> {
		Ok((
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
		))
	}
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Deserialise for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
	T0: Deserialise<Error: StdError + 'static>,
	T1: Deserialise<Error: StdError + 'static>,
	T2: Deserialise<Error: StdError + 'static>,
	T3: Deserialise<Error: StdError + 'static>,
	T4: Deserialise<Error: StdError + 'static>,
	T5: Deserialise<Error: StdError + 'static>,
	T6: Deserialise<Error: StdError + 'static>,
	T7: Deserialise<Error: StdError + 'static>,
	T8: Deserialise<Error: StdError + 'static>, {
	type Error = Box<dyn StdError>;

	fn deserialise(stream: &mut Dstream) -> Result<Self, Self::Error> {
		Ok((
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
		))
	}
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Deserialise for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
	T0: Deserialise<Error: StdError + 'static>,
	T1: Deserialise<Error: StdError + 'static>,
	T2: Deserialise<Error: StdError + 'static>,
	T3: Deserialise<Error: StdError + 'static>,
	T4: Deserialise<Error: StdError + 'static>,
	T5: Deserialise<Error: StdError + 'static>,
	T6: Deserialise<Error: StdError + 'static>,
	T7: Deserialise<Error: StdError + 'static>,
	T8: Deserialise<Error: StdError + 'static>,
	T9: Deserialise<Error: StdError + 'static>, {
	type Error = Box<dyn StdError>;

	fn deserialise(stream: &mut Dstream) -> Result<Self, Self::Error> {
		Ok((
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
		))
	}
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Deserialise for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
	T0:  Deserialise<Error: StdError + 'static>,
	T1:  Deserialise<Error: StdError + 'static>,
	T2:  Deserialise<Error: StdError + 'static>,
	T3:  Deserialise<Error: StdError + 'static>,
	T4:  Deserialise<Error: StdError + 'static>,
	T5:  Deserialise<Error: StdError + 'static>,
	T6:  Deserialise<Error: StdError + 'static>,
	T7:  Deserialise<Error: StdError + 'static>,
	T8:  Deserialise<Error: StdError + 'static>,
	T9:  Deserialise<Error: StdError + 'static>,
	T10: Deserialise<Error: StdError + 'static>, {
	type Error = Box<dyn StdError>;

	fn deserialise(stream: &mut Dstream) -> Result<Self, Self::Error> {
		Ok((
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
		))
	}
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Deserialise for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
	T0:  Deserialise<Error: StdError + 'static>,
	T1:  Deserialise<Error: StdError + 'static>,
	T2:  Deserialise<Error: StdError + 'static>,
	T3:  Deserialise<Error: StdError + 'static>,
	T4:  Deserialise<Error: StdError + 'static>,
	T5:  Deserialise<Error: StdError + 'static>,
	T6:  Deserialise<Error: StdError + 'static>,
	T7:  Deserialise<Error: StdError + 'static>,
	T8:  Deserialise<Error: StdError + 'static>,
	T9:  Deserialise<Error: StdError + 'static>,
	T10: Deserialise<Error: StdError + 'static>,
	T11: Deserialise<Error: StdError + 'static>, {
	type Error = Box<dyn StdError>;

	fn deserialise(stream: &mut Dstream) -> Result<Self, Self::Error> {
		Ok((
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
		))
	}
}

impl<T, const N: usize> Deserialise for [T; N]
where
	T: Default + Deserialise<Error: StdError + 'static>, {
	type Error = Box<dyn StdError>;

	fn deserialise(stream: &mut Dstream) -> Result<Self, Self::Error> {
		let len = usize::deserialise(stream)?;

		if len != N { return Err(Box::new(Error::ArrayTooShort { req: len, len: N })) };

		let mut buf: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

		// Deserialise t
		for item in buf.iter_mut().take(len) {
			item.write(Deserialise::deserialise(stream)?);
		}

		for item in buf.iter_mut().skip(len) {
			item.write(Default::default());
		}

		// This should be safe as `MaybeUninit<T>` is
		// transparent to `T`. The original buffer is
		// NOT dropped automatically, so we can just
		// forget about it from this point on.
		let buf = unsafe { read(core::ptr::from_ref(&buf).cast::<[T; N]>()) };
		Ok(buf)
	}
}

impl Deserialise for () {
	type Error = Infallible;

	fn deserialise(_stream: &mut Dstream) -> Result<Self, Self::Error> { Ok(()) }
}

impl Deserialise for bool {
	type Error = Error;

	fn deserialise(stream: &mut Dstream) -> Result<Self, Self::Error> {
		let value = u8::deserialise(stream)?;

		match value {
			0x00 => Ok(false),
			0x01 => Ok(true),
			_    => Err(Error::InvalidBoolean { value })
		}
	}
}

impl Deserialise for char {
	type Error = Error;

	fn deserialise(stream: &mut Dstream) -> Result<Self, Self::Error> {
		let value = u32::deserialise(stream)?;

		Self::from_u32(value)
			.ok_or(Error::InvalidCodePoint { value })
	}
}

impl Deserialise for Infallible {
	type Error = Self;

	fn deserialise(_stream: &mut Dstream) -> Result<Self, Self::Error> { unreachable!() }
}

impl Deserialise for isize {
	type Error = Error;

	fn deserialise(stream: &mut Dstream) -> Result<Self, Self::Error> {
		let value = i16::deserialise(stream)?
			.into();

		Ok(value)
	}
}

impl<T: Deserialise<Error: StdError + 'static>> Deserialise for Option<T> {
	type Error = Box<dyn StdError>;

	fn deserialise(stream: &mut Dstream) -> Result<Self, Self::Error> {
		let sign = bool::deserialise(stream)?;

		if sign {
			Ok(Some(T::deserialise(stream)?))
		} else {
			Ok(None)
		}
	}
}

impl<T: Deserialise, E: Deserialise> Deserialise for Result<T, E>
where
	<T as Deserialise>::Error: StdError + 'static,
	<E as Deserialise>::Error: StdError + 'static, {
	type Error = Box<dyn StdError>;

	fn deserialise(stream: &mut Dstream) -> Result<Self, Self::Error> {
		let sign = bool::deserialise(stream)?;

		let value = if sign {
			Err(E::deserialise(stream)?)
		} else {
			Ok(T::deserialise(stream)?)
		};

		Ok(value)
	}
}

impl Deserialise for usize {
	type Error = Error;

	fn deserialise(stream: &mut Dstream) -> Result<Self, Self::Error> {
		let value = u16::deserialise(stream)?
			.into();

		Ok(value)
	}
}

impl_float!(f32);
impl_float!(f64);

impl_int!(i128);
impl_int!(i16);
impl_int!(i32);
impl_int!(i64);
impl_int!(i8);
impl_int!(u128);
impl_int!(u16);
impl_int!(u32);
impl_int!(u64);
impl_int!(u8);

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
