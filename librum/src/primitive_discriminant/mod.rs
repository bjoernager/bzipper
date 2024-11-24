// Copyright 2024 Gabriel Bjørnager Jensen.
//
// This file is part of Librum.
//
// Librum is free software: you can redistribute it
// and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// Librum is distributed in the hope that it will
// be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Less-
// er General Public License along with Librum. If
// not, see <https://www.gnu.org/licenses/>.

mod sealed {
	/// Denotes a primitive, integral discriminant type.
	///
	/// See the public [`PrimitiveDiscriminant`](crate::PrimitiveDiscriminant) trait for more information.
	pub trait PrimitiveDiscriminant {
		/// Interprets the discriminant value as `u128`.
		///
		/// The returned value has exactly the same representation as the original value except that it is zero-extended to fit.
		#[must_use]
		fn to_u128(self) -> u128;
	}
}

pub(crate) use sealed::PrimitiveDiscriminant as SealedPrimitiveDiscriminant;

/// Denotes a primitive, integral discriminant type.
///
/// This type is specifically defined as a type which may be used as a representation in the `repr` attribute, i.e. [`u8`], [`i8`], [`u16`], [`i16`], [`u32`], [`i32`], [`u64`], [`i64`], [`usize`], and [`isize`].
///
/// On nightly, this additionally includes [`u128`] and [`i128`] (see [`repr128`](https://github.com/rust-lang/rust/issues/56071/)).
/// Note that this trait is implemented for these two types regardless.
///
/// Internally -- specifically in the [`GenericDecodeError`](crate::error::GenericDecodeError) enumeration -- this trait guarantees representability in the `u128` type.
pub trait PrimitiveDiscriminant: SealedPrimitiveDiscriminant + Sized { }

macro_rules! impl_primitive_discriminant {
	($ty:ty) => {
		impl ::librum::SealedPrimitiveDiscriminant for $ty {
			#[allow(clippy::cast_lossless)]
			#[inline(always)]
			#[must_use]
			fn to_u128(self) -> u128 {
				self as u128
			}
		}

		impl ::librum::PrimitiveDiscriminant for $ty { }
	};
}

impl_primitive_discriminant!(u8);
impl_primitive_discriminant!(i8);
impl_primitive_discriminant!(u16);
impl_primitive_discriminant!(i16);
impl_primitive_discriminant!(u32);
impl_primitive_discriminant!(i32);
impl_primitive_discriminant!(u64);
impl_primitive_discriminant!(i64);
impl_primitive_discriminant!(u128);
impl_primitive_discriminant!(i128);
impl_primitive_discriminant!(usize);
impl_primitive_discriminant!(isize);
