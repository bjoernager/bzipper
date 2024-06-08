// Copyright 2022-2024 Gabriel Bjørnager Jensen.

use crate::serialise::SStream;

use std::convert::Infallible;
use std::mem::size_of;
use std::num::NonZero;

/// Denotes a type capable of being serialised.
pub trait Serialise: Sized {
	/// Serialises `self` into a byte stream.
	///
	/// One may assume that the resulting stream has at most the same ammount of bytes as before serialisation.
	/// Therefore, not observing this rule is a logic error.
	fn serialise(&self, stream: &mut SStream);
}

macro_rules! impl_float {
	($type:ty) => {
		impl Serialise for $type {
			fn serialise(&self, stream: &mut SStream) {
				stream.append(&self.to_be_bytes())
			}
		}
	};
}

macro_rules! impl_int {
	($type:ty) => {
		impl Serialise for $type {
			fn serialise(&self, stream: &mut SStream) {
				stream.append(&self.to_be_bytes())
			}
		}

		impl Serialise for NonZero<$type> {
			fn serialise(&self, stream: &mut SStream) {
				self.get().serialise(stream)
			}
		}
	};
}

impl<T: Serialise, const N: usize> Serialise for [T; N] {
	fn serialise(&self, stream: &mut SStream) {
		u64::try_from(self.len()).unwrap().serialise(stream);

		for v in self { v.serialise(stream) }
	}
}

impl Serialise for () {
	fn serialise(&self, _stream: &mut SStream) { }
}

impl Serialise for bool {
	fn serialise(&self, stream: &mut SStream) {
		u8::from(*self).serialise(stream)
	}
}

impl Serialise for char {
	fn serialise(&self, stream: &mut SStream) {
		u32::from(*self).serialise(stream)
	}
}

impl Serialise for Infallible {
	fn serialise(&self, _stream: &mut SStream) { unreachable!() }
}

impl<T: Serialise> Serialise for Option<T> {
	fn serialise(&self, stream: &mut SStream) {
		match *self {
			None => {
				stream.append(&[0x00]);
				stream.append(&vec![0x00; size_of::<T>()]);
			},

			Some(ref v) => {
				stream.append(&[0x01]);
				v.serialise(stream);
			},
		};
	}
}

impl<T: Serialise, E: Serialise> Serialise for Result<T, E> {
	fn serialise(&self, stream: &mut SStream) {
		match *self {
			Ok(ref v) => {
				stream.append(&[0x00]);
				v.serialise(stream);
			},

			Err(ref e) => {
				stream.append(&[0x01]);
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
