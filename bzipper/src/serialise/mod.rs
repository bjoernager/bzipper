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

use core::{convert::Infallible, marker::PhantomData};

mod tuple;

/// Denotes a type capable of being serialised.
///
/// It is recommended to simply derive this trait for custom types.
/// It can, however, be manually implemented:
///
/// ```
/// use bzipper::{Result, Serialise, Sstream};
///
/// struct Foo {
///     bar: u16,
///     baz: f32,
/// }
///
/// impl Serialise for Foo {
///     const SERIALISED_SIZE: usize = u16::SERIALISED_SIZE + f32::SERIALISED_SIZE;
///
///     fn serialise(&self, buf: &mut [u8]) -> Result<()> {
///         debug_assert_eq!(buf.len(), Self::SERIALISED_SIZE);
///
///         // Serialise fields using chaining.
///
///         let mut stream = Sstream::new(buf);
///
///         stream.append(&self.bar)?;
///         stream.append(&self.baz)?;
///
///         Ok(())
///     }
/// }
/// ```
///
/// Implementors of this trait should make sure that [`SERIALISED_SIZE`](Serialise::SERIALISED_SIZE) is properly defined.
/// This value indicates the definitive size of any serialisation of the `Self` type.
pub trait Serialise: Sized {
	/// The amount of bytes that result from a serialisation.
	///
	/// Implementors of this trait should make sure that no serialisation (or deserialisation) uses more than the amount specified by this constant.
	/// When using these traits, always assume that exactly this amount has or will be used.
	const SERIALISED_SIZE: usize;

	/// Serialises `self` into a slice.
	///
	/// In most cases it is wiser to chain serialisations using [`Sstream`] instead of using this method directly.
	///
	/// # Errors
	///
	/// If serialisation failed, e.g. by an unencodable value being provided, an error is returned.
	///
	/// # Panics
	///
	/// This method will usually panic if the provided slice has a length *less* than the value of `SERIALISED_SIZE`.
	/// Official implementations of this trait (including those that are derived) always panic in debug mode if the provided slice has a length that is different at all.
	fn serialise(&self, buf: &mut [u8]) -> Result<()>;
}

macro_rules! impl_numeric {
	($ty:ty) => {
		impl ::bzipper::Serialise for $ty {
			const SERIALISED_SIZE: usize = size_of::<$ty>();

			#[inline]
			fn serialise(&self, buf: &mut [u8]) -> Result<()> {
				debug_assert_eq!(buf.len(), Self::SERIALISED_SIZE);

				::core::debug_assert_eq!(buf.len(), Self::SERIALISED_SIZE);

				let data = self.to_be_bytes();
				buf.copy_from_slice(&data);

				Ok(())
			}
		}
	};
}

macro_rules! impl_non_zero {
	($ty:ty) => {
		impl ::bzipper::Serialise for ::core::num::NonZero<$ty> {
			const SERIALISED_SIZE: usize = ::core::mem::size_of::<$ty>();

			#[inline(always)]
			fn serialise(&self, buf: &mut [u8]) -> Result<()> {
				debug_assert_eq!(buf.len(), Self::SERIALISED_SIZE);

				self.get().serialise(buf)
			}
		}
	};
}

impl<T: Serialise, const N: usize> Serialise for [T; N] {
	const SERIALISED_SIZE: usize = T::SERIALISED_SIZE * N;

	fn serialise(&self, buf: &mut [u8]) -> Result<()> {
		debug_assert_eq!(buf.len(), Self::SERIALISED_SIZE);

		let mut stream = Sstream::new(buf);

		for v in self { stream.append(v)? }

		Ok(())
	}
}

impl Serialise for bool {
	const SERIALISED_SIZE: usize = u8::SERIALISED_SIZE;

	#[inline(always)]
	fn serialise(&self, buf: &mut [u8]) -> Result<()> {
		debug_assert_eq!(buf.len(), Self::SERIALISED_SIZE);

		u8::from(*self).serialise(buf)
	}
}

impl Serialise for char {
	const SERIALISED_SIZE: usize = u32::SERIALISED_SIZE;

	#[inline(always)]
	fn serialise(&self, buf: &mut [u8]) -> Result<()> {
		debug_assert_eq!(buf.len(), Self::SERIALISED_SIZE);

		u32::from(*self).serialise(buf)
	}

}

// Especially useful for `Result<T, Infallible>`.
// *If* that is needed, of course.
impl Serialise for Infallible {
	const SERIALISED_SIZE: usize = 0x0;

	#[inline(always)]
	fn serialise(&self, _buf: &mut [u8]) -> Result<()> { unreachable!() }

}

impl Serialise for isize {
	const SERIALISED_SIZE: usize = i32::SERIALISED_SIZE;

	#[inline]
	fn serialise(&self, buf: &mut [u8]) -> Result<()> {
		debug_assert_eq!(buf.len(), Self::SERIALISED_SIZE);

		let value = i32::try_from(*self)
			.map_err(|_| Error::IsizeOutOfRange { value: *self })?;

		value.serialise(buf)
	}
}

impl<T: Serialise> Serialise for Option<T> {
	const SERIALISED_SIZE: usize = bool::SERIALISED_SIZE + T::SERIALISED_SIZE;

	fn serialise(&self, buf: &mut [u8]) -> Result<()> {
		debug_assert_eq!(buf.len(), Self::SERIALISED_SIZE);

		// The first element is of type `bool` and is
		// called the "sign." It signifies whether there is
		// a following element or not. The remaining bytes
		// are preserved if `self` is `None`.

		let mut stream = Sstream::new(buf);

		match *self {
			None => {
				stream.append(&false)?;
				// No need to zero-fill.
			},

			Some(ref v) => {
				stream.append(&true)?;
				stream.append(v)?;
			},
		};

		Ok(())
	}
}

impl<T> Serialise for PhantomData<T> {
	const SERIALISED_SIZE: usize = size_of::<Self>();

	#[inline(always)]
	fn serialise(&self, buf: &mut [u8]) -> Result<()> {
		debug_assert_eq!(buf.len(), Self::SERIALISED_SIZE);

		Ok(())
	}
}

impl<T, E> Serialise for core::result::Result<T, E>
where
	T: Serialise,
	E: Serialise, {
	const SERIALISED_SIZE: usize = bool::SERIALISED_SIZE + if size_of::<T>() > size_of::<E>() { size_of::<T>() } else { size_of::<E>() };

	fn serialise(&self, buf: &mut [u8]) -> Result<()> {
		debug_assert_eq!(buf.len(), Self::SERIALISED_SIZE);

		let mut stream = Sstream::new(buf);

		// Remember the descriminant.
		match *self {
			Ok(ref v) => {
				stream.append(&false)?;
				stream.append(v)?;
			},

			Err(ref e) => {
				stream.append(&true)?;
				stream.append(e)?;
			},
		};

		Ok(())
	}
}

impl Serialise for () {
	const SERIALISED_SIZE: usize = size_of::<Self>();

	#[inline(always)]
	fn serialise(&self, buf: &mut [u8]) -> Result<()> {
		debug_assert_eq!(buf.len(), Self::SERIALISED_SIZE);

		Ok(())
	}
}

impl Serialise for usize {
	const SERIALISED_SIZE: Self = u32::SERIALISED_SIZE;

	fn serialise(&self, buf: &mut [u8]) -> Result<()> {
		debug_assert_eq!(buf.len(), Self::SERIALISED_SIZE);

		let value = u32::try_from(*self)
			.map_err(|_| Error::UsizeOutOfRange { value: *self })?;

		value.serialise(buf)
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
