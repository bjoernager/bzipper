// Copyright 2024 Gabriel Bjørnager Jensen.
//
// This file is part of Oct.
//
// Oct is free software: you can redistribute it
// and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// Oct is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even
// the implied warranty of MERCHANTABILITY or FIT-
// NESS FOR A PARTICULAR PURPOSE. See the GNU Less-
// er General Public License for more details.
//
// You should have received a copy of the GNU Less-
// er General Public License along with Oct. If
// not, see <https://www.gnu.org/licenses/>.

use crate::PrimitiveDiscriminant;
use crate::decode::Decode;
use crate::error::{
	CollectionDecodeError,
	EnumDecodeError,
	ItemDecodeError,
	NonZeroDecodeError,
	LengthError,
	Utf8Error,
	SystemTimeDecodeError,
};

#[cfg(feature = "alloc")]
use crate::error::CStringDecodeError;

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

	/// A C-like string contained a null byte.
	#[cfg(feature = "std")]
	#[cfg_attr(doc, doc(cfg(feature = "std")))]
	NullString(CStringDecodeError),

	/// A non-null integer was null.
	NullInteger(NonZeroDecodeError),

	/// A statically-sized buffer was too small.
	SmallBuffer(LengthError),

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
		match *self {
			Self::BadString(ref e)
			=> write!(f, "{e}"),

			Self::NullString(ref e)
			=> write!(f, "{e}"),

			Self::NullInteger(ref e)
			=> write!(f, "{e}"),

			Self::SmallBuffer(ref e)
			=> write!(f, "{e}"),

			Self::UnassignedDiscriminant { value }
			=> write!(f, "discriminant value `{value:#X} has not been assigned"),

			#[cfg(feature = "std")]
			Self::NarrowSystemTime(ref e)
			=> write!(f, "{e}"),
		}
	}
}

impl Error for GenericDecodeError {
	#[inline]
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match *self {
			Self::BadString(ref e) => Some(e),

			#[cfg(feature = "std")]
			Self::NullString(ref e) => Some(e),

			Self::NullInteger(ref e) => Some(e),

			Self::SmallBuffer(ref e) => Some(e),

			#[cfg(feature = "std")]
			Self::NarrowSystemTime(ref e) => Some(e),

			_ => None,
		}
	}
}

impl<L, I> From<CollectionDecodeError<L, I>> for GenericDecodeError
where
	L: Into<Self>,
	I: Into<Self>,
{
	#[inline(always)]
	fn from(value: CollectionDecodeError<L, I>) -> Self {
		use CollectionDecodeError as Error;

		match value {
			Error::BadLength(e) => e.into(),

			Error::BadItem(e) => e.into(),
		}
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl From<CStringDecodeError> for GenericDecodeError {
	#[inline(always)]
	fn from(value: CStringDecodeError) -> Self {
		Self::NullString(value)
	}
}

impl<D, F> From<EnumDecodeError<D, F>> for GenericDecodeError
where
	D: Decode<Error: Into<Self>> + PrimitiveDiscriminant,
	F: Into<Self>,
{
	#[inline(always)]
	fn from(value: EnumDecodeError<D, F>) -> Self {
		use EnumDecodeError as Error;

		match value {
			Error::InvalidDiscriminant(e) => e.into(),

			Error::UnassignedDiscriminant { value } => Self::UnassignedDiscriminant { value: value.to_u128() },

			Error::BadField(e) => e.into(),
		}
	}
}

impl From<Infallible> for GenericDecodeError {
	#[inline(always)]
	fn from(_value: Infallible) -> Self {
		// SAFETY: `Infallible` objects can never be con-
		// structed
		unsafe { unreachable_unchecked() }
	}
}

impl<I, E: Into<Self>> From<ItemDecodeError<I, E>> for GenericDecodeError {
	#[inline(always)]
	fn from(value: ItemDecodeError<I, E>) -> Self {
		value.error.into()
	}
}

impl From<NonZeroDecodeError> for GenericDecodeError {
	#[inline(always)]
	fn from(value: NonZeroDecodeError) -> Self {
		Self::NullInteger(value)
	}
}

impl From<LengthError> for GenericDecodeError {
	#[inline(always)]
	fn from(value: LengthError) -> Self {
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
