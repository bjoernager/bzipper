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

use core::cell::BorrowError;
use core::error::Error;
use core::fmt::{self, Display, Formatter};

/// A reference cell could not be encoded.
///
/// The implementation of <code>&lt;[RefCell](core::cell::RefCell)&lt;T&gt; as [Encode](crate::encode::Encode)&gt;::[encode](crate::encode::Encode::encode)</code> will first attempt to call <code>RefCell::[borrow](core::cell::RefCell::borrow)</code>.
/// If this call fails, then the returned error is again returned as a [`BadBorrow`](Self::BadBorrow) instance.
/// If the following call to <code>T::encode</code> fails instead, then the error returned from that call is passed on as a [`BadValue`](Self::BadValue) instance.
#[derive(Debug)]
#[must_use]
pub enum RefCellEncodeError<E> {
	/// The reference cell could not be borrowed.
	BadBorrow(BorrowError),

	/// The contained value could not be encoded.
	BadValue(E),
}

impl<E: Display> Display for RefCellEncodeError<E> {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		let e: &dyn Display = match *self {
			Self::BadBorrow(ref e) => e,

			Self::BadValue(ref e) => e,
		};

		write!(f, "unable to encode reference cell: {e}")
	}
}

impl<E: Error + 'static> Error for RefCellEncodeError<E> {
	#[inline(always)]
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match *self {
			Self::BadBorrow(ref e) => Some(e),

			Self::BadValue(ref e) => Some(e)
		}
	}
}
