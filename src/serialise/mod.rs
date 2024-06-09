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

use crate::Sstream;

use std::convert::Infallible;
use std::mem::size_of;
use std::num::NonZero;

/// Denotes a type capable of being serialised.
pub trait Serialise: Sized {
	/// The maximum ammount of bytes that can result from serialisation.
	const SERIALISE_LIMIT: usize;

	/// Serialises `self` into a byte stream.
	///
	/// This function should not append *more* bytes than specified in [`SERIALISE_LIMIT`](Serialise::SERIALISE_LIMIT).
	/// Doing so is considered a logic error.
	fn serialise(&self, stream: &mut Sstream);
}

macro_rules! impl_float {
	($type:ty) => {
		impl Serialise for $type {
			const SERIALISE_LIMIT: usize = size_of::<$type>();

			fn serialise(&self, stream: &mut Sstream) {
				stream.append(&self.to_be_bytes())
			}
		}
	};
}

macro_rules! impl_int {
	($type:ty) => {
		impl Serialise for $type {
			const SERIALISE_LIMIT: usize = size_of::<$type>();

			fn serialise(&self, stream: &mut Sstream) {
				stream.append(&self.to_be_bytes())
			}
		}

		impl Serialise for NonZero<$type> {
			const SERIALISE_LIMIT: usize = size_of::<$type>();

			fn serialise(&self, stream: &mut Sstream) {
				self.get().serialise(stream)
			}
		}
	};
}

impl<T0, T1> Serialise for (T0, T1)
where
	T0: Serialise,
	T1: Serialise, {
	const SERIALISE_LIMIT: usize =
		T0::SERIALISE_LIMIT
		+ T1::SERIALISE_LIMIT;

	fn serialise(&self, stream: &mut Sstream) {
		self.0.serialise(stream);
		self.1.serialise(stream);
	}
}

impl<T0, T1, T2> Serialise for (T0, T1, T2)
where
	T0: Serialise,
	T1: Serialise,
	T2: Serialise, {
	const SERIALISE_LIMIT: usize =
		T0::SERIALISE_LIMIT
		+ T1::SERIALISE_LIMIT
		+ T2::SERIALISE_LIMIT;

	fn serialise(&self, stream: &mut Sstream) {
		self.0.serialise(stream);
		self.1.serialise(stream);
		self.2.serialise(stream);
	}
}

impl<T0, T1, T2, T3> Serialise for (T0, T1, T2, T3)
where
	T0: Serialise,
	T1: Serialise,
	T2: Serialise,
	T3: Serialise, {
	const SERIALISE_LIMIT: usize =
		T0::SERIALISE_LIMIT
		+ T1::SERIALISE_LIMIT
		+ T2::SERIALISE_LIMIT
		+ T3::SERIALISE_LIMIT;

		fn serialise(&self, stream: &mut Sstream) {
		self.0.serialise(stream);
		self.1.serialise(stream);
		self.2.serialise(stream);
		self.3.serialise(stream);
	}
}

impl<T0, T1, T2, T3, T4> Serialise for (T0, T1, T2, T3, T4)
where
	T0: Serialise,
	T1: Serialise,
	T2: Serialise,
	T3: Serialise,
	T4: Serialise, {
	const SERIALISE_LIMIT: usize =
		T0::SERIALISE_LIMIT
		+ T1::SERIALISE_LIMIT
		+ T2::SERIALISE_LIMIT
		+ T3::SERIALISE_LIMIT
		+ T4::SERIALISE_LIMIT;

		fn serialise(&self, stream: &mut Sstream) {
		self.0.serialise(stream);
		self.1.serialise(stream);
		self.2.serialise(stream);
		self.3.serialise(stream);
		self.4.serialise(stream);
	}
}

impl<T0, T1, T2, T3, T4, T5> Serialise for (T0, T1, T2, T3, T4, T5)
where
	T0: Serialise,
	T1: Serialise,
	T2: Serialise,
	T3: Serialise,
	T4: Serialise,
	T5: Serialise, {
	const SERIALISE_LIMIT: usize =
		T0::SERIALISE_LIMIT
		+ T1::SERIALISE_LIMIT
		+ T2::SERIALISE_LIMIT
		+ T3::SERIALISE_LIMIT
		+ T4::SERIALISE_LIMIT
		+ T5::SERIALISE_LIMIT;

		fn serialise(&self, stream: &mut Sstream) {
		self.0.serialise(stream);
		self.1.serialise(stream);
		self.2.serialise(stream);
		self.3.serialise(stream);
		self.4.serialise(stream);
		self.5.serialise(stream);
	}
}

impl<T0, T1, T2, T3, T4, T5, T6> Serialise for (T0, T1, T2, T3, T4, T5, T6)
where
	T0: Serialise,
	T1: Serialise,
	T2: Serialise,
	T3: Serialise,
	T4: Serialise,
	T5: Serialise,
	T6: Serialise, {
	const SERIALISE_LIMIT: usize =
		T0::SERIALISE_LIMIT
		+ T1::SERIALISE_LIMIT
		+ T2::SERIALISE_LIMIT
		+ T3::SERIALISE_LIMIT
		+ T4::SERIALISE_LIMIT
		+ T5::SERIALISE_LIMIT
		+ T6::SERIALISE_LIMIT;

		fn serialise(&self, stream: &mut Sstream) {
		self.0.serialise(stream);
		self.1.serialise(stream);
		self.2.serialise(stream);
		self.3.serialise(stream);
		self.4.serialise(stream);
		self.5.serialise(stream);
		self.6.serialise(stream);
	}
}

impl<T0, T1, T2, T3, T4, T5, T6, T7> Serialise for (T0, T1, T2, T3, T4, T5, T6, T7)
where
	T0: Serialise,
	T1: Serialise,
	T2: Serialise,
	T3: Serialise,
	T4: Serialise,
	T5: Serialise,
	T6: Serialise,
	T7: Serialise, {
	const SERIALISE_LIMIT: usize =
		T0::SERIALISE_LIMIT
		+ T1::SERIALISE_LIMIT
		+ T2::SERIALISE_LIMIT
		+ T3::SERIALISE_LIMIT
		+ T4::SERIALISE_LIMIT
		+ T5::SERIALISE_LIMIT
		+ T6::SERIALISE_LIMIT
		+ T7::SERIALISE_LIMIT;

		fn serialise(&self, stream: &mut Sstream) {
		self.0.serialise(stream);
		self.1.serialise(stream);
		self.2.serialise(stream);
		self.3.serialise(stream);
		self.4.serialise(stream);
		self.5.serialise(stream);
		self.6.serialise(stream);
		self.7.serialise(stream);
	}
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Serialise for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
	T0: Serialise,
	T1: Serialise,
	T2: Serialise,
	T3: Serialise,
	T4: Serialise,
	T5: Serialise,
	T6: Serialise,
	T7: Serialise,
	T8: Serialise, {
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

		fn serialise(&self, stream: &mut Sstream) {
		self.0.serialise(stream);
		self.1.serialise(stream);
		self.2.serialise(stream);
		self.3.serialise(stream);
		self.4.serialise(stream);
		self.5.serialise(stream);
		self.6.serialise(stream);
		self.7.serialise(stream);
		self.8.serialise(stream);
	}
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Serialise for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
	T0: Serialise,
	T1: Serialise,
	T2: Serialise,
	T3: Serialise,
	T4: Serialise,
	T5: Serialise,
	T6: Serialise,
	T7: Serialise,
	T8: Serialise,
	T9: Serialise, {
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

		fn serialise(&self, stream: &mut Sstream) {
		self.0.serialise(stream);
		self.1.serialise(stream);
		self.2.serialise(stream);
		self.3.serialise(stream);
		self.4.serialise(stream);
		self.5.serialise(stream);
		self.6.serialise(stream);
		self.7.serialise(stream);
		self.8.serialise(stream);
		self.9.serialise(stream);
	}
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Serialise for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
	T0:  Serialise,
	T1:  Serialise,
	T2:  Serialise,
	T3:  Serialise,
	T4:  Serialise,
	T5:  Serialise,
	T6:  Serialise,
	T7:  Serialise,
	T8:  Serialise,
	T9:  Serialise,
	T10: Serialise, {
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

		fn serialise(&self, stream: &mut Sstream) {
		self.0.serialise(stream);
		self.1.serialise(stream);
		self.2.serialise(stream);
		self.3.serialise(stream);
		self.4.serialise(stream);
		self.5.serialise(stream);
		self.6.serialise(stream);
		self.7.serialise(stream);
		self.8.serialise(stream);
		self.9.serialise(stream);
		self.10.serialise(stream);
	}
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Serialise for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
	T0:  Serialise,
	T1:  Serialise,
	T2:  Serialise,
	T3:  Serialise,
	T4:  Serialise,
	T5:  Serialise,
	T6:  Serialise,
	T7:  Serialise,
	T8:  Serialise,
	T9:  Serialise,
	T10: Serialise,
	T11: Serialise, {
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

		fn serialise(&self, stream: &mut Sstream) {
		self.0.serialise(stream);
		self.1.serialise(stream);
		self.2.serialise(stream);
		self.3.serialise(stream);
		self.4.serialise(stream);
		self.5.serialise(stream);
		self.6.serialise(stream);
		self.7.serialise(stream);
		self.8.serialise(stream);
		self.9.serialise(stream);
		self.10.serialise(stream);
		self.11.serialise(stream);
	}
}

impl<T: Serialise, const N: usize> Serialise for [T; N] {
	const SERIALISE_LIMIT: usize = T::SERIALISE_LIMIT * N;

	fn serialise(&self, stream: &mut Sstream) {
		u64::try_from(self.len()).unwrap().serialise(stream);

		for v in self { v.serialise(stream) }
	}
}

impl Serialise for () {
	const SERIALISE_LIMIT: usize = size_of::<Self>();

	fn serialise(&self, _stream: &mut Sstream) { }
}

impl Serialise for bool {
	const SERIALISE_LIMIT: usize = size_of::<Self>();

	fn serialise(&self, stream: &mut Sstream) {
		u8::from(*self).serialise(stream)
	}
}

impl Serialise for char {
	const SERIALISE_LIMIT: usize = size_of::<Self>();

	fn serialise(&self, stream: &mut Sstream) {
		u32::from(*self).serialise(stream)
	}
}

impl Serialise for Infallible {
	const SERIALISE_LIMIT: usize = size_of::<Self>();

	fn serialise(&self, _stream: &mut Sstream) { unreachable!() }
}

impl<T: Serialise> Serialise for Option<T> {
	const SERIALISE_LIMIT: usize = T::SERIALISE_LIMIT + 0x1;

	fn serialise(&self, stream: &mut Sstream) {
		match *self {
			None => {
				stream.append_byte(0x00);
				stream.append(&vec![0x00; size_of::<T>()]);
			},

			Some(ref v) => {
				stream.append_byte(0x01);
				v.serialise(stream);
			},
		};
	}
}

impl<T: Serialise, E: Serialise> Serialise for Result<T, E> {
	const SERIALISE_LIMIT: usize = const {
		if size_of::<T>() > size_of::<T>() {
			size_of::<T>()
		} else {
			size_of::<E>()
		}
	};

	fn serialise(&self, stream: &mut Sstream) {
		match *self {
			Ok(ref v) => {
				stream.append_byte(0x00);
				v.serialise(stream);
			},

			Err(ref e) => {
				stream.append_byte(0x01);
				e.serialise(stream);
			},
		};
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
