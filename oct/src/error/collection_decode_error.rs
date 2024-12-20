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

use core::convert::Infallible;
use core::error::Error;
use core::fmt::{self, Display, Formatter};

/// A collection could not be decoded.
///
/// This type is intended as a partially-generic decode error for collections.
/// It supports denoting an error for when the collection's length is invalid -- see the [`BadLength`](Self::BadLength) variant -- and when an element is invalid -- see the [`Item`](Self::BadItem)) variant.
///
/// The most common form of this type is <code>CollectionDecodeError<[Infallible](core::convert::Infallible), [ItemDecodeError](crate::error::ItemDecodeError)<[usize], ..></code>, but this may not always necessarily be the preferred form.
///
/// An example of a type using a different form is [`SizedStr`](crate::SizedStr), which uses <code>CollectionDecodeError<[`LengthError`](crate::error::LengthError), [Utf8Error](crate::error::Utf8Error)></code>.
#[derive(Debug)]
#[must_use]
pub enum CollectionDecodeError<L, I> {
	/// The collection length could not be decoded or was invalid.
	///
	/// For most dynamically-sized collections, the suitable type here is [`Infallible`] due to there basically being no restriction on the collection's size (depending on the data type used for denoting lengths).
	///
	/// Sometimes the length isn't even encoded in the stream (instead lying in the type signature), and in these cases the appropriate type would also be `Infallible`.
	BadLength(L),

	/// A collection item could not be decoded.
	///
	/// Sometimes the index of the item may be desired.
	/// In these cases the [`ItemDecodeError`](crate::error::ItemDecodeError) could be used here.
	BadItem(I),
}

impl<L, I> Display for CollectionDecodeError<L, I>
where
	L: Display,
	I: Display,
{
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match *self {
			Self::BadLength(ref e)
			=> write!(f, "unable to decode collection length: {e}"),

			Self::BadItem(ref e)
			=> write!(f, "unable to decode collection item: {e}"),
		}
	}
}

impl<L, I> Error for CollectionDecodeError<L, I>
where
	L: Error + 'static,
	I: Error + 'static,
{
	#[inline]
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match *self {
			Self::BadLength(ref e) => Some(e),

			Self::BadItem(ref e) => Some(e),
		}
	}
}

impl<L, I> From<CollectionDecodeError<L, I>> for Infallible
where
	L: Into<Self>,
	I: Into<Self>,
{
	#[inline(always)]
	fn from(_value: CollectionDecodeError<L, I>) -> Self {
		unreachable!()
	}
}
