// Copyright 2024 Gabriel Bjørnager Jensen.
//
// This file is part of oct.
//
// oct is free software: you can redistribute it
// and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// oct is distributed in the hope that it will
// be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Less-
// er General Public License along with oct. If
// not, see <https://www.gnu.org/licenses/>.

use crate::encode::Encode;
use crate::error::{
	CollectionEncodeError,
	EnumEncodeError,
	IsizeEncodeError,
	ItemEncodeError,
	UsizeEncodeError,
};

use core::cell::BorrowError;
use core::convert::Infallible;
use core::error::Error;
use core::fmt::{self, Display, Formatter};
use core::hint::unreachable_unchecked;

/// A decoding failed.
///
/// The intended use of this type is by [derived](derive@crate::encode::Encode) implementations of [`Encode`](crate::encode::Encode).
/// Manual implementors are recommended to use a custom or less generic type for the sake of efficiency.
#[derive(Debug)]
#[must_use]
#[non_exhaustive]
pub enum GenericEncodeError {
	/// A [`RefCell`](core::cell::RefCell) object could not be borrowed.
	BadBorrow(BorrowError),

	/// An `isize` object was outside the allowed domain.
	LargeIsize(IsizeEncodeError),

	/// A `usize` object was outside the allowed domain.
	LargeUsize(UsizeEncodeError),
}

impl Display for GenericEncodeError {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match *self {
			Self::BadBorrow(ref e)
			=> write!(f, "{e}"),

			Self::LargeIsize(ref e)
			=> write!(f, "{e}"),

			Self::LargeUsize(ref e)
			=> write!(f, "{e}"),

		}
	}
}

impl Error for GenericEncodeError {
	#[inline]
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match *self {
			Self::BadBorrow(ref e) => Some(e),

			Self::LargeIsize(ref e) => Some(e),

			Self::LargeUsize(ref e) => Some(e),
		}
	}
}

impl From<BorrowError> for GenericEncodeError {
	#[inline(always)]
	fn from(value: BorrowError) -> Self {
		Self::BadBorrow(value)
	}
}

impl<L, I> From<CollectionEncodeError<L, I>> for GenericEncodeError
where
	L: Into<Self>,
	I: Into<Self>,
{
	#[inline(always)]
	fn from(value: CollectionEncodeError<L, I>) -> Self {
		use CollectionEncodeError as Error;

		match value {
			Error::BadLength(e) => e.into(),

			Error::BadItem(e) => e.into(),
		}
	}
}

impl<D, F> From<EnumEncodeError<D, F>> for GenericEncodeError
where
	D: Encode<Error: Into<Self>>,
	F: Into<Self>,
{
	#[inline(always)]
	fn from(value: EnumEncodeError<D, F>) -> Self {
		use EnumEncodeError as Error;

		match value {
			Error::BadDiscriminant(e) => e.into(),

			Error::BadField(e) => e.into(),
		}
	}
}

impl From<Infallible> for GenericEncodeError {
	#[inline(always)]
	fn from(_value: Infallible) -> Self {
		// SAFETY: `Infallible` objects can never be con-
		// structed
		unsafe { unreachable_unchecked() }
	}
}

impl From<IsizeEncodeError> for GenericEncodeError {
	#[inline(always)]
	fn from(value: IsizeEncodeError) -> Self {
		Self::LargeIsize(value)
	}
}

impl<I, E: Into<Self>> From<ItemEncodeError<I, E>> for GenericEncodeError {
	#[inline(always)]
	fn from(value: ItemEncodeError<I, E>) -> Self {
		value.error.into()
	}
}

impl From<UsizeEncodeError> for GenericEncodeError {
	#[inline(always)]
	fn from(value: UsizeEncodeError) -> Self {
		Self::LargeUsize(value)
	}
}
