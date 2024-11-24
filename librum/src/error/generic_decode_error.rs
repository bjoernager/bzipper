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

use crate::{Decode, PrimitiveDiscriminant};
use crate::error::{
	BoolDecodeError,
	CollectionDecodeError,
	CStringDecodeError,
	EnumDecodeError,
	ItemDecodeError,
	NonZeroDecodeError,
	SizeError,
	Utf8Error,
	SystemTimeDecodeError,
};

use core::convert::Infallible;
use core::error::Error;
use core::fmt::{self, Display, Formatter};
use core::hint::unreachable_unchecked;

/// A decoding failed.
///
/// The intended use of this type is by [derived](derive@Decode) implementations of [`Decode`].
/// Manual implementors are recommended to use a custom or less generic type for the sake of efficiency.
#[derive(Debug)]
#[must_use]
#[non_exhaustive]
pub enum GenericDecodeError {
	/// A string contained a non-UTF-8 sequence.
	BadString(Utf8Error),

	/// A boolean was neither `false` nor `true`.
	InvalidBool(BoolDecodeError),

	/// A C-like string contained a null byte.
	#[cfg(feature = "std")]
	#[cfg_attr(doc, doc(cfg(feature = "std")))]
	NullString(CStringDecodeError),

	/// A non-null integer was null.
	NullInteger(NonZeroDecodeError),

	/// A statically-sized buffer was too small.
	SmallBuffer(SizeError),

	/// An unassigned discriminant value was encountered.
	///
	/// The contained value denotes the raw, numerical value of the discriminant.
	UnassignedDiscriminant {
		/// The raw value of the discriminant.
		value: u128
	},

	/// The [`SystemTime`](std::time::SystemTime) type was too narrow.
	#[cfg(feature = "std")]
	#[cfg_attr(doc, doc(cfg(feature = "std")))]
	NarrowSystemTime(SystemTimeDecodeError),
}

impl Display for GenericDecodeError {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		use GenericDecodeError::*;

		match *self {
			BadString(ref e)
			=> write!(f, "{e}"),

			InvalidBool(ref e)
			=> write!(f, "{e}"),

			NullString(ref e)
			=> write!(f, "{e}"),

			NullInteger(ref e)
			=> write!(f, "{e}"),

			SmallBuffer(ref e)
			=> write!(f, "{e}"),

			UnassignedDiscriminant { value }
			=> write!(f, "discriminant value `{value:#X} has not been assigned"),

			#[cfg(feature = "std")]
			NarrowSystemTime(ref e)
			=> write!(f, "{e}"),
		}
	}
}

impl Error for GenericDecodeError {
	#[inline]
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		use GenericDecodeError::*;

		match *self {
			BadString(ref e) => Some(e),

			InvalidBool(ref e) => Some(e),

			#[cfg(feature = "std")]
			NullString(ref e) => Some(e),

			NullInteger(ref e) => Some(e),

			SmallBuffer(ref e) => Some(e),

			#[cfg(feature = "std")]
			NarrowSystemTime(ref e) => Some(e),

			_ => None,
		}
	}
}

impl From<BoolDecodeError> for GenericDecodeError {
	#[inline(always)]
	fn from(value: BoolDecodeError) -> Self {
		Self::InvalidBool(value)
	}
}

impl<L, I> From<CollectionDecodeError<L, I>> for GenericDecodeError
where
	L: Into<Self>,
	I: Into<Self>,
{
	#[inline(always)]
	fn from(value: CollectionDecodeError<L, I>) -> Self {
		use CollectionDecodeError::*;

		match value {
			Length(e) => e.into(),

			Item(e) => e.into(),
		}
	}
}

impl<D, F> From<EnumDecodeError<D, F>> for GenericDecodeError
where
	D: Decode<Error: Into<Self>> + PrimitiveDiscriminant,
	F: Into<Self>,
{
	#[inline(always)]
	fn from(value: EnumDecodeError<D, F>) -> Self {
		use EnumDecodeError::*;

		match value {
			InvalidDiscriminant(e) => e.into(),

			UnassignedDiscriminant { value } => Self::UnassignedDiscriminant { value: value.to_u128() },

			Field(e) => e.into(),
		}
	}
}

impl From<Infallible> for GenericDecodeError {
	#[inline(always)]
	fn from(_value: Infallible) -> Self {
		unsafe { unreachable_unchecked() }
	}
}

impl<I, E: Into<Self>> From<ItemDecodeError<I, E>> for GenericDecodeError {
	#[inline(always)]
	fn from(value: ItemDecodeError<I, E>) -> Self {
		value.error.into()
	}
}

#[cfg(feature = "std")]
#[cfg_attr(doc, doc(cfg(feature = "std")))]
impl From<CStringDecodeError> for GenericDecodeError {
	#[inline(always)]
	fn from(value: CStringDecodeError) -> Self {
		Self::NullString(value)
	}
}

impl From<NonZeroDecodeError> for GenericDecodeError {
	#[inline(always)]
	fn from(value: NonZeroDecodeError) -> Self {
		Self::NullInteger(value)
	}
}

impl From<SizeError> for GenericDecodeError {
	#[inline(always)]
	fn from(value: SizeError) -> Self {
		Self::SmallBuffer(value)
	}
}

#[cfg(feature = "std")]
#[cfg_attr(doc, doc(cfg(feature = "std")))]
impl From<SystemTimeDecodeError> for GenericDecodeError {
	#[inline(always)]
	fn from(value: SystemTimeDecodeError) -> Self {
		Self::NarrowSystemTime(value)
	}
}

impl From<Utf8Error> for GenericDecodeError {
	#[inline(always)]
	fn from(value: Utf8Error) -> Self {
		Self::BadString(value)
	}
}
