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

use crate::{Error, Sstream};

use alloc::boxed::Box;
use core::convert::Infallible;
use core::error::Error as StdError;
use core::mem::size_of;
use core::num::NonZero;

/// Types capable of being serialised.
pub trait Serialise: Sized {
	/// The error of serialisation.
	///
	/// Use [`Infallible`] if **all** deserialisations are infallible, as is the case of zero-length types.
	type Error;

	/// The maximum amount of bytes that can result from serialisation.
	const SERIALISE_LIMIT: usize;

	/// Serialises `self` into a byte stream.
	///
	/// The number of bytes written is returned.
	/// This should **not** exceed [`SERIALISE_LIMIT`](Serialise::SERIALISE_LIMIT), and doing so is considered a logic error.
	fn serialise(&self, stream: &mut Sstream) -> Result<usize, Self::Error>;
}

macro_rules! impl_float {
	($type:ty) => {
		impl Serialise for $type {
			type Error = Error;

			const SERIALISE_LIMIT: usize = size_of::<$type>();

			fn serialise(&self, stream: &mut Sstream) -> Result<usize, Self::Error> {
				let data = self.to_be_bytes();
				stream.add(&data)?;

				Ok(data.len())
			}
		}
	};
}

macro_rules! impl_int {
	($type:ty) => {
		impl Serialise for $type {
			type Error = Error;

			const SERIALISE_LIMIT: usize = size_of::<$type>();

			fn serialise(&self, stream: &mut Sstream) -> Result<usize, Self::Error> {
				let data = self.to_be_bytes();
				stream.add(&data)?;

				Ok(data.len())
			}
		}
	};
}

macro_rules! impl_non_zero {
	($type:ty) => {
		impl Serialise for NonZero<$type> {
			type Error = <$type as Serialise>::Error;

			const SERIALISE_LIMIT: usize = size_of::<$type>();

			fn serialise(&self, stream: &mut Sstream) -> Result<usize, Self::Error> {
				self.get().serialise(stream)
			}
		}
	};
}

impl<T0, T1> Serialise for (T0, T1)
where
	T0: Serialise<Error: StdError + 'static>,
	T1: Serialise<Error: StdError + 'static>, {
	type Error = Box<dyn StdError>;

	const SERIALISE_LIMIT: usize =
		T0::SERIALISE_LIMIT
		+ T1::SERIALISE_LIMIT;

	fn serialise(&self, stream: &mut Sstream) -> Result<usize, Self::Error> {
		let mut count = 0x0;

		count += self.0.serialise(stream)?;
		count += self.1.serialise(stream)?;

		Ok(count)
	}
}

impl<T0, T1, T2> Serialise for (T0, T1, T2)
where
	T0: Serialise<Error: StdError + 'static>,
	T1: Serialise<Error: StdError + 'static>,
	T2: Serialise<Error: StdError + 'static>, {
	type Error = Box<dyn StdError>;

	const SERIALISE_LIMIT: usize =
		T0::SERIALISE_LIMIT
		+ T1::SERIALISE_LIMIT
		+ T2::SERIALISE_LIMIT;

	fn serialise(&self, stream: &mut Sstream) -> Result<usize, Self::Error> {
		let mut count = 0x0;

		count += self.0.serialise(stream)?;
		count += self.1.serialise(stream)?;
		count += self.2.serialise(stream)?;

		Ok(count)
	}
}

impl<T0, T1, T2, T3> Serialise for (T0, T1, T2, T3)
where
	T0: Serialise<Error: StdError + 'static>,
	T1: Serialise<Error: StdError + 'static>,
	T2: Serialise<Error: StdError + 'static>,
	T3: Serialise<Error: StdError + 'static>, {
	type Error = Box<dyn StdError>;

	const SERIALISE_LIMIT: usize =
		T0::SERIALISE_LIMIT
		+ T1::SERIALISE_LIMIT
		+ T2::SERIALISE_LIMIT
		+ T3::SERIALISE_LIMIT;

		fn serialise(&self, stream: &mut Sstream) -> Result<usize, Self::Error> {
			let mut count = 0x0;

		count += self.0.serialise(stream)?;
		count += self.1.serialise(stream)?;
		count += self.2.serialise(stream)?;
		count += self.3.serialise(stream)?;

		Ok(count)
	}
}

impl<T0, T1, T2, T3, T4> Serialise for (T0, T1, T2, T3, T4)
where
	T0: Serialise<Error: StdError + 'static>,
	T1: Serialise<Error: StdError + 'static>,
	T2: Serialise<Error: StdError + 'static>,
	T3: Serialise<Error: StdError + 'static>,
	T4: Serialise<Error: StdError + 'static>, {
	type Error = Box<dyn StdError>;

	const SERIALISE_LIMIT: usize =
		T0::SERIALISE_LIMIT
		+ T1::SERIALISE_LIMIT
		+ T2::SERIALISE_LIMIT
		+ T3::SERIALISE_LIMIT
		+ T4::SERIALISE_LIMIT;

		fn serialise(&self, stream: &mut Sstream) -> Result<usize, Self::Error> {
			let mut count = 0x0;

		count += self.0.serialise(stream)?;
		count += self.1.serialise(stream)?;
		count += self.2.serialise(stream)?;
		count += self.3.serialise(stream)?;
		count += self.4.serialise(stream)?;

		Ok(count)
	}
}

impl<T0, T1, T2, T3, T4, T5> Serialise for (T0, T1, T2, T3, T4, T5)
where
	T0: Serialise<Error: StdError + 'static>,
	T1: Serialise<Error: StdError + 'static>,
	T2: Serialise<Error: StdError + 'static>,
	T3: Serialise<Error: StdError + 'static>,
	T4: Serialise<Error: StdError + 'static>,
	T5: Serialise<Error: StdError + 'static>, {
	type Error = Box<dyn StdError>;

	const SERIALISE_LIMIT: usize =
		T0::SERIALISE_LIMIT
		+ T1::SERIALISE_LIMIT
		+ T2::SERIALISE_LIMIT
		+ T3::SERIALISE_LIMIT
		+ T4::SERIALISE_LIMIT
		+ T5::SERIALISE_LIMIT;

		fn serialise(&self, stream: &mut Sstream) -> Result<usize, Self::Error> {
			let mut count = 0x0;

		count += self.0.serialise(stream)?;
		count += self.1.serialise(stream)?;
		count += self.2.serialise(stream)?;
		count += self.3.serialise(stream)?;
		count += self.4.serialise(stream)?;
		count += self.5.serialise(stream)?;

		Ok(count)
	}
}

impl<T0, T1, T2, T3, T4, T5, T6> Serialise for (T0, T1, T2, T3, T4, T5, T6)
where
	T0: Serialise<Error: StdError + 'static>,
	T1: Serialise<Error: StdError + 'static>,
	T2: Serialise<Error: StdError + 'static>,
	T3: Serialise<Error: StdError + 'static>,
	T4: Serialise<Error: StdError + 'static>,
	T5: Serialise<Error: StdError + 'static>,
	T6: Serialise<Error: StdError + 'static>, {
	type Error = Box<dyn StdError>;

	const SERIALISE_LIMIT: usize =
		T0::SERIALISE_LIMIT
		+ T1::SERIALISE_LIMIT
		+ T2::SERIALISE_LIMIT
		+ T3::SERIALISE_LIMIT
		+ T4::SERIALISE_LIMIT
		+ T5::SERIALISE_LIMIT
		+ T6::SERIALISE_LIMIT;

		fn serialise(&self, stream: &mut Sstream) -> Result<usize, Self::Error> {
			let mut count = 0x0;

		count += self.0.serialise(stream)?;
		count += self.1.serialise(stream)?;
		count += self.2.serialise(stream)?;
		count += self.3.serialise(stream)?;
		count += self.4.serialise(stream)?;
		count += self.5.serialise(stream)?;
		count += self.6.serialise(stream)?;

		Ok(count)
	}
}

impl<T0, T1, T2, T3, T4, T5, T6, T7> Serialise for (T0, T1, T2, T3, T4, T5, T6, T7)
where
	T0: Serialise<Error: StdError + 'static>,
	T1: Serialise<Error: StdError + 'static>,
	T2: Serialise<Error: StdError + 'static>,
	T3: Serialise<Error: StdError + 'static>,
	T4: Serialise<Error: StdError + 'static>,
	T5: Serialise<Error: StdError + 'static>,
	T6: Serialise<Error: StdError + 'static>,
	T7: Serialise<Error: StdError + 'static>, {
	type Error = Box<dyn StdError>;

	const SERIALISE_LIMIT: usize =
		T0::SERIALISE_LIMIT
		+ T1::SERIALISE_LIMIT
		+ T2::SERIALISE_LIMIT
		+ T3::SERIALISE_LIMIT
		+ T4::SERIALISE_LIMIT
		+ T5::SERIALISE_LIMIT
		+ T6::SERIALISE_LIMIT
		+ T7::SERIALISE_LIMIT;

		fn serialise(&self, stream: &mut Sstream) -> Result<usize, Self::Error> {
			let mut count = 0x0;

		count += self.0.serialise(stream)?;
		count += self.1.serialise(stream)?;
		count += self.2.serialise(stream)?;
		count += self.3.serialise(stream)?;
		count += self.4.serialise(stream)?;
		count += self.5.serialise(stream)?;
		count += self.6.serialise(stream)?;
		count += self.7.serialise(stream)?;

		Ok(count)
	}
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Serialise for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
	T0: Serialise<Error: StdError + 'static>,
	T1: Serialise<Error: StdError + 'static>,
	T2: Serialise<Error: StdError + 'static>,
	T3: Serialise<Error: StdError + 'static>,
	T4: Serialise<Error: StdError + 'static>,
	T5: Serialise<Error: StdError + 'static>,
	T6: Serialise<Error: StdError + 'static>,
	T7: Serialise<Error: StdError + 'static>,
	T8: Serialise<Error: StdError + 'static>, {
	type Error = Box<dyn StdError>;

	const SERIALISE_LIMIT: usize =
		T0::SERIALISE_LIMIT
		+ T1::SERIALISE_LIMIT
		+ T2::SERIALISE_LIMIT
		+ T3::SERIALISE_LIMIT
		+ T4::SERIALISE_LIMIT
		+ T5::SERIALISE_LIMIT
		+ T6::SERIALISE_LIMIT
		+ T7::SERIALISE_LIMIT
		+ T8::SERIALISE_LIMIT;

		fn serialise(&self, stream: &mut Sstream) -> Result<usize, Self::Error> {
			let mut count = 0x0;

		count += self.0.serialise(stream)?;
		count += self.1.serialise(stream)?;
		count += self.2.serialise(stream)?;
		count += self.3.serialise(stream)?;
		count += self.4.serialise(stream)?;
		count += self.5.serialise(stream)?;
		count += self.6.serialise(stream)?;
		count += self.7.serialise(stream)?;
		count += self.8.serialise(stream)?;

		Ok(count)
	}
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Serialise for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
	T0: Serialise<Error: StdError + 'static>,
	T1: Serialise<Error: StdError + 'static>,
	T2: Serialise<Error: StdError + 'static>,
	T3: Serialise<Error: StdError + 'static>,
	T4: Serialise<Error: StdError + 'static>,
	T5: Serialise<Error: StdError + 'static>,
	T6: Serialise<Error: StdError + 'static>,
	T7: Serialise<Error: StdError + 'static>,
	T8: Serialise<Error: StdError + 'static>,
	T9: Serialise<Error: StdError + 'static>, {
	type Error = Box<dyn StdError>;

	const SERIALISE_LIMIT: usize =
		T0::SERIALISE_LIMIT
		+ T1::SERIALISE_LIMIT
		+ T2::SERIALISE_LIMIT
		+ T3::SERIALISE_LIMIT
		+ T4::SERIALISE_LIMIT
		+ T5::SERIALISE_LIMIT
		+ T6::SERIALISE_LIMIT
		+ T7::SERIALISE_LIMIT
		+ T8::SERIALISE_LIMIT
		+ T9::SERIALISE_LIMIT;

		fn serialise(&self, stream: &mut Sstream) -> Result<usize, Self::Error> {
			let mut count = 0x0;

		count += self.0.serialise(stream)?;
		count += self.1.serialise(stream)?;
		count += self.2.serialise(stream)?;
		count += self.3.serialise(stream)?;
		count += self.4.serialise(stream)?;
		count += self.5.serialise(stream)?;
		count += self.6.serialise(stream)?;
		count += self.7.serialise(stream)?;
		count += self.8.serialise(stream)?;
		count += self.9.serialise(stream)?;

		Ok(count)
	}
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Serialise for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
	T0:  Serialise<Error: StdError + 'static>,
	T1:  Serialise<Error: StdError + 'static>,
	T2:  Serialise<Error: StdError + 'static>,
	T3:  Serialise<Error: StdError + 'static>,
	T4:  Serialise<Error: StdError + 'static>,
	T5:  Serialise<Error: StdError + 'static>,
	T6:  Serialise<Error: StdError + 'static>,
	T7:  Serialise<Error: StdError + 'static>,
	T8:  Serialise<Error: StdError + 'static>,
	T9:  Serialise<Error: StdError + 'static>,
	T10: Serialise<Error: StdError + 'static>, {
	type Error = Box<dyn StdError>;

	const SERIALISE_LIMIT: usize =
		T0::SERIALISE_LIMIT
		+ T1::SERIALISE_LIMIT
		+ T2::SERIALISE_LIMIT
		+ T3::SERIALISE_LIMIT
		+ T4::SERIALISE_LIMIT
		+ T5::SERIALISE_LIMIT
		+ T6::SERIALISE_LIMIT
		+ T7::SERIALISE_LIMIT
		+ T8::SERIALISE_LIMIT
		+ T9::SERIALISE_LIMIT
		+ T10::SERIALISE_LIMIT;

		fn serialise(&self, stream: &mut Sstream) -> Result<usize, Self::Error> {
			let mut count = 0x0;

		count += self.0.serialise(stream)?;
		count += self.1.serialise(stream)?;
		count += self.2.serialise(stream)?;
		count += self.3.serialise(stream)?;
		count += self.4.serialise(stream)?;
		count += self.5.serialise(stream)?;
		count += self.6.serialise(stream)?;
		count += self.7.serialise(stream)?;
		count += self.8.serialise(stream)?;
		count += self.9.serialise(stream)?;
		count += self.10.serialise(stream)?;

		Ok(count)
	}
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Serialise for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
	T0:  Serialise<Error: StdError + 'static>,
	T1:  Serialise<Error: StdError + 'static>,
	T2:  Serialise<Error: StdError + 'static>,
	T3:  Serialise<Error: StdError + 'static>,
	T4:  Serialise<Error: StdError + 'static>,
	T5:  Serialise<Error: StdError + 'static>,
	T6:  Serialise<Error: StdError + 'static>,
	T7:  Serialise<Error: StdError + 'static>,
	T8:  Serialise<Error: StdError + 'static>,
	T9:  Serialise<Error: StdError + 'static>,
	T10: Serialise<Error: StdError + 'static>,
	T11: Serialise<Error: StdError + 'static>, {
	type Error = Box<dyn StdError>;

	const SERIALISE_LIMIT: usize =
		T0::SERIALISE_LIMIT
		+ T1::SERIALISE_LIMIT
		+ T2::SERIALISE_LIMIT
		+ T3::SERIALISE_LIMIT
		+ T4::SERIALISE_LIMIT
		+ T5::SERIALISE_LIMIT
		+ T6::SERIALISE_LIMIT
		+ T7::SERIALISE_LIMIT
		+ T8::SERIALISE_LIMIT
		+ T9::SERIALISE_LIMIT
		+ T10::SERIALISE_LIMIT
		+ T11::SERIALISE_LIMIT;

		fn serialise(&self, stream: &mut Sstream) -> Result<usize, Self::Error> {
			let mut count = 0x0;

		count += self.0.serialise(stream)?;
		count += self.1.serialise(stream)?;
		count += self.2.serialise(stream)?;
		count += self.3.serialise(stream)?;
		count += self.4.serialise(stream)?;
		count += self.5.serialise(stream)?;
		count += self.6.serialise(stream)?;
		count += self.7.serialise(stream)?;
		count += self.8.serialise(stream)?;
		count += self.9.serialise(stream)?;
		count += self.10.serialise(stream)?;
		count += self.11.serialise(stream)?;

		Ok(count)
	}
}

impl<T: Serialise<Error: StdError + 'static>, const N: usize> Serialise for [T; N] {
	type Error = Box<dyn StdError>;

	const SERIALISE_LIMIT: usize = T::SERIALISE_LIMIT * N;

	fn serialise(&self, stream: &mut Sstream) -> Result<usize, Self::Error> {
		let mut count = 0x0;

		self.len().serialise(stream)?;
		for v in self { count += v.serialise(stream)? }

		Ok(count)
	}
}

impl Serialise for () {
	type Error = Infallible;

	const SERIALISE_LIMIT: usize = size_of::<Self>();

	#[inline(always)]
	fn serialise(&self, mut _stream: &mut Sstream) -> Result<usize, Self::Error> {
		Ok(Self::SERIALISE_LIMIT)
	}
}

impl Serialise for bool {
	type Error = Error;

	const SERIALISE_LIMIT: usize = size_of::<Self>();

	fn serialise(&self, stream: &mut Sstream) -> Result<usize, Self::Error> {
		u8::from(*self).serialise(stream)
	}
}

impl Serialise for char {
	type Error = Error;

	const SERIALISE_LIMIT: usize = size_of::<Self>();

	fn serialise(&self, stream: &mut Sstream) -> Result<usize, Self::Error> {
		u32::from(*self).serialise(stream)
	}
}

// Especially useful for `Result<T, Infallible>`.
impl Serialise for Infallible {
	type Error = Self;

	const SERIALISE_LIMIT: usize = size_of::<Self>();

	fn serialise(&self, mut _stream: &mut Sstream) -> Result<usize, Self::Error> { unreachable!() }
}

impl Serialise for isize {
	type Error = Error;

	const SERIALISE_LIMIT: usize = size_of::<i16>();

	fn serialise(&self, stream: &mut Sstream) -> Result<usize, Self::Error> {
		let value = i16::try_from(*self)
			.map_err(|_| Error::IsizeOutOfRange { value: *self })?;

		value.serialise(stream)
	}
}

impl<T: Serialise<Error: StdError + 'static>> Serialise for Option<T> {
	type Error = Box<dyn StdError>;

	const SERIALISE_LIMIT: usize = T::SERIALISE_LIMIT + 0x1;

	fn serialise(&self, stream: &mut Sstream) -> Result<usize, Self::Error> {
		let mut count = 0x0;

		match *self {
			None => {
				count += false.serialise(stream)?;
				// No need to zero-fill.
			},

			Some(ref v) => {
				count += true.serialise(stream)?;
				count += v.serialise(stream)?;
			},
		};

		Ok(count)
	}
}

impl<T, E> Serialise for core::result::Result<T, E>
where
	T: Serialise<Error: StdError + 'static>,
	E: Serialise<Error: StdError + 'static>, {
	type Error = Box<dyn StdError>;

	const SERIALISE_LIMIT: usize = const {
		if size_of::<T>() > size_of::<T>() {
			size_of::<T>()
		} else {
			size_of::<E>()
		}
	};

	fn serialise(&self, stream: &mut Sstream) -> Result<usize, Self::Error> {
		// Remember the descriminant.
		let mut count = 0x0;

		match *self {
			Ok(ref v) => {
				count += false.serialise(stream)?;
				count += v.serialise(stream)?;
			},

			Err(ref e) => {
				count += true.serialise(stream)?;
				count += e.serialise(stream)?;
			},
		};

		Ok(count)
	}
}

impl Serialise for usize {
	type Error = Error;

	const SERIALISE_LIMIT: Self = size_of::<u16>();

	fn serialise(&self, stream: &mut Sstream) -> Result<usize, Self::Error> {
		let value = u16::try_from(*self)
			.map_err(|_| Error::UsizeOutOfRange { value: *self })?;

		value.serialise(stream)
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
