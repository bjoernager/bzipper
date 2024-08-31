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

use crate::{Error, Result, Sstream};

use core::{convert::Infallible, hint::unreachable_unchecked, marker::PhantomData};

mod tuple;

/// Denotes a type capable of serialisation.
///
/// It is recommended to simply derive this trait for custom types.
/// It can, however, also be manually implemented:
///
/// ```rust
/// // Manual implementation of custom type. This im-
/// // plementation is equivalent to what would have
/// // been derived.
///
/// use bzipper::{Result, Serialise, Sstream};
///
/// struct Foo {
///     bar: u16,
///     baz: f32,
/// }
///
/// impl Serialise for Foo {
///     const MAX_SERIALISED_SIZE: usize = u16::MAX_SERIALISED_SIZE + f32::MAX_SERIALISED_SIZE;
///
///     fn serialise(&self, stream: &mut Sstream) -> Result<()> {
///         // Serialise fields using chaining.
///
///         self.bar.serialise(stream)?;
///         self.baz.serialise(stream)?;
///
///         Ok(())
///     }
/// }
/// ```
///
/// Implementors of this trait should make sure that [`MAX_SERIALISED_SIZE`](Self::MAX_SERIALISED_SIZE) is properly defined.
/// This value indicates the definitively largest size of any serialisation of `Self`.
pub trait Serialise: Sized {
	/// The maximum amount of bytes that can result from a serialisation.
	///
	/// Implementors of this trait should make sure that no serialisation (or deserialisation) uses more than the amount specified by this constant.
	const MAX_SERIALISED_SIZE: usize;

	/// Serialises `self` into the given s-stream.
	///
	/// This method must **never** write more bytes than specified by [`MAX_SERIALISED_SIZE`](Self::MAX_SERIALISED_SIZE).
	/// Doing so is considered a logic error.
	///
	/// # Errors
	///
	/// If serialisation fails, e.g. by an unencodable value being provided, an error is returned.
	fn serialise(&self, stream: &mut Sstream) -> Result<()>;
}

macro_rules! impl_numeric {
	($ty:ty) => {
		impl ::bzipper::Serialise for $ty {
			const MAX_SERIALISED_SIZE: usize = size_of::<$ty>();

			#[inline]
			fn serialise(&self, stream: &mut Sstream) -> Result<()> {
				stream.write(&self.to_be_bytes())?;

				Ok(())
			}
		}
	};
}

macro_rules! impl_non_zero {
	($ty:ty) => {
		impl ::bzipper::Serialise for ::core::num::NonZero<$ty> {
			const MAX_SERIALISED_SIZE: usize = ::core::mem::size_of::<$ty>();

			#[inline(always)]
			fn serialise(&self, stream: &mut Sstream) -> Result<()> { self.get().serialise(stream) }
		}
	};
}

impl<T: Serialise, const N: usize> Serialise for [T; N] {
	const MAX_SERIALISED_SIZE: usize = T::MAX_SERIALISED_SIZE * N;

	fn serialise(&self, stream: &mut Sstream) -> Result<()> {
		for v in self { v.serialise(stream)? }

		Ok(())
	}
}

impl Serialise for bool {
	const MAX_SERIALISED_SIZE: usize = u8::MAX_SERIALISED_SIZE;

	#[inline(always)]
	fn serialise(&self, stream: &mut Sstream) -> Result<()> {
		u8::from(*self).serialise(stream)
	}
}

impl Serialise for char {
	const MAX_SERIALISED_SIZE: usize = u32::MAX_SERIALISED_SIZE;

	#[inline(always)]
	fn serialise(&self, stream: &mut Sstream) -> Result<()> {
		u32::from(*self).serialise(stream)
	}

}

// Especially useful for `Result<T, Infallible>`.
// *If* that is even needed, of course.
impl Serialise for Infallible {
	const MAX_SERIALISED_SIZE: usize = 0x0;

	#[inline(always)]
	fn serialise(&self, _stream: &mut Sstream) -> Result<()> { unsafe { unreachable_unchecked() } }

}

impl Serialise for isize {
	const MAX_SERIALISED_SIZE: usize = i32::MAX_SERIALISED_SIZE;

	#[inline]
	fn serialise(&self, stream: &mut Sstream) -> Result<()> {
		let value = i32::try_from(*self)
			.map_err(|_| Error::IsizeOutOfRange(*self))?;

		value.serialise(stream)
	}
}

impl<T: Serialise> Serialise for Option<T> {
	const MAX_SERIALISED_SIZE: usize = bool::MAX_SERIALISED_SIZE + T::MAX_SERIALISED_SIZE;

	fn serialise(&self, stream: &mut Sstream) -> Result<()> {
		// The first element is of type `bool` and is
		// called the "sign." It signifies whether there is
		// a following element or not.

		match *self {
			None => {
				false.serialise(stream)?;
				// No need to zero-fill.
			},

			Some(ref v) => {
				true.serialise(stream)?;
				v.serialise(stream)?;
			},
		};

		Ok(())
	}
}

impl<T> Serialise for PhantomData<T> {
	const MAX_SERIALISED_SIZE: usize = size_of::<Self>();

	#[inline(always)]
	fn serialise(&self, _stream: &mut Sstream) -> Result<()> { Ok(()) }
}

impl<T, E> Serialise for core::result::Result<T, E>
where
	T: Serialise,
	E: Serialise, {
	const MAX_SERIALISED_SIZE: usize = bool::MAX_SERIALISED_SIZE + if size_of::<T>() > size_of::<E>() { size_of::<T>() } else { size_of::<E>() };

	fn serialise(&self, stream: &mut Sstream) -> Result<()> {
		// Remember the descriminant.

		match *self {
			Ok(ref v) => {
				false.serialise(stream)?;
				v.serialise(stream)?;
			},

			Err(ref e) => {
				true.serialise(stream)?;
				e.serialise(stream)?;
			},
		};

		Ok(())
	}
}

impl Serialise for () {
	const MAX_SERIALISED_SIZE: usize = 0x0;

	#[inline(always)]
	fn serialise(&self, _stream: &mut Sstream) -> Result<()> { Ok(()) }
}

impl Serialise for usize {
	const MAX_SERIALISED_SIZE: Self = u32::MAX_SERIALISED_SIZE;

	fn serialise(&self, stream: &mut Sstream) -> Result<()> {
		let value = u32::try_from(*self)
			.map_err(|_| Error::UsizeOutOfRange(*self))?;

		value.serialise(stream)
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
