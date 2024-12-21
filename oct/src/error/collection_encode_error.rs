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

use core::convert::Infallible;
use core::error::Error;
use core::fmt::{self, Display, Formatter};

/// A collection could not be encoded.
///
/// This type is intended as a partially-generic encode error for collections.
/// It supports denoting an error for when the collection's length is invalid -- see the [`BadLength`](Self::BadLength) variant -- and when an element is invalid -- see the [`Item`](Self::BadItem)) variant.
#[derive(Debug)]
#[must_use]
pub enum CollectionEncodeError<L, I> {
	/// The collection length could not be encoded.
	BadLength(L),

	/// A collection item could not be encoded.
	BadItem(I),
}

impl<L, I> Display for CollectionEncodeError<L, I>
where
	L: Display,
	I: Display,
{
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match *self {
			Self::BadLength(ref e)
			=> write!(f, "unable to encode collection length: {e}"),

			Self::BadItem(ref e)
			=> write!(f, "unable to encode collection item: {e}"),
		}
	}
}

impl<L, I> Error for CollectionEncodeError<L, I>
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

impl<L, I> From<CollectionEncodeError<L, I>> for Infallible
where
	L: Into<Self>,
	I: Into<Self>,
{
	#[inline(always)]
	fn from(_value: CollectionEncodeError<L, I>) -> Self {
		unreachable!()
	}
}
