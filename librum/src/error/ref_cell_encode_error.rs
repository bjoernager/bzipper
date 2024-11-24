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

use core::cell::BorrowError;
use core::error::Error;
use core::fmt::{self, Display, Formatter};

/// A reference cell could not be encoded.
///
/// The implementation of <code>&lt;[RefCell](core::cell::RefCell)&lt;T&gt; as [Encode](crate::Encode)&gt;::[encode](crate::Encode::encode)</code> will first attempt to call <code>RefCell::[borrow](core::cell::RefCell::borrow)</code>.
/// If this call fails, then the returned error is again returned as a [`Borrow`](Self::Borrow) instance.
/// If the following call to <code>T::encode</code> fails instead, then the error returned from that call is passed on as a [`Value`](Self::Value) instance.
#[derive(Debug)]
#[must_use]
pub enum RefCellEncodeError<E> {
	/// The reference cell could not be borrowed.
	Borrow(BorrowError),

	/// The contained value could not be encoded.
	Value(E),
}

impl<E: Display> Display for RefCellEncodeError<E> {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		use RefCellEncodeError::*;

		let e: &dyn Display = match *self {
			Borrow(ref e) => e,

			Value(ref e) => e,
		};

		write!(f, "unable to encode reference cell: {e}")
	}
}

impl<E: Error + 'static> Error for RefCellEncodeError<E> {
	#[inline(always)]
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		use RefCellEncodeError::*;

		match *self {
			Borrow(ref e) => Some(e),

			Value(ref e) => Some(e)
		}
	}
}
