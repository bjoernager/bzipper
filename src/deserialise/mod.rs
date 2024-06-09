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

use crate::{Dstream, Error, Serialise};

use std::convert::Infallible;
use std::error::Error as StdError;
use std::mem::size_of;
use std::num::NonZero;

/// Denotes a type capable of being deserialised.
pub trait Deserialise: Serialise + Sized {
	type Error;

	/// Deserialises the byte stream to an object.
	///
	/// This function should not take *more* bytes than specified by [`SERIALISE_LIMIT`](Serialise::SERIALISE_LIMIT).
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

impl<T: Deserialise<Error: StdError + 'static>, const N: usize> Deserialise for [T; N] {
	type Error = Box<dyn StdError>;

	fn deserialise(stream: &mut Dstream) -> Result<Self, Self::Error> {
		let len = usize::try_from(u64::deserialise(stream)?).unwrap();
		if len != N { return Err(Box::new(Error::ArrayTooShort { req: len, len: N })) };

		let mut buf = Vec::with_capacity(len);
		for _ in 0x0..len { buf.push(Deserialise::deserialise(stream)?); }

		// If we had used the checked unwrap, we would also
		// have to require `T: Debug`.
		Ok(unsafe { buf.try_into().unwrap_unchecked() })
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
