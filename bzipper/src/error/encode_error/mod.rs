// Copyright 2024 Gabriel Bjørnager Jensen.
//
// This file is part of bZipper.
//
// bZipper is free software: you can redistribute
// it and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// bZipper is distributed in the hope that it will
// be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Less-
// er General Public License along with bZipper. If
// not, see <https://www.gnu.org/licenses/>.

use core::cell::BorrowError;
use core::error::Error;
use core::fmt::{self, Display, Formatter};

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

/// Encode error variants.
///
/// These errors may be returned from implementation of [`Encode`](crate::Encode).
#[derive(Debug)]
#[non_exhaustive]
pub enum EncodeError {
	/// A [`RefCell`](core::cell::RefCell) object could not be borrowed.
	BadBorrow(BorrowError),

	/// An unspecified error.
	///
	/// This is mainly useful by third-party implementors if none of the other predefined variants are appropriate.
	#[cfg(feature = "alloc")]
	#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
	CustomError(Box<dyn core::error::Error>),

	/// An `isize` value could not be cast as `i16`.
	IsizeOutOfRange(isize),

	/// A `usize` value could not be cast as `u16`.
	UsizeOutOfRange(usize),
}

impl Display for EncodeError {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		use EncodeError::*;

		match *self {
			BadBorrow(ref source)
			=> write!(f, "could not borrow reference cell: {source}"),

			#[cfg(feature = "alloc")]
			CustomError(ref source)
			=> write!(f, "{source}"),

			IsizeOutOfRange(value)
			=> write!(f, "signed size value ({value}) cannot be serialised: must be in the range ({}) to ({})", i16::MIN, i16::MAX),

			UsizeOutOfRange(value)
			=> write!(f, "unsigned size value ({value}) cannot be serialised: must be at most ({})", u16::MAX),
		}
	}
}

impl Error for EncodeError {
	#[inline]
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		use EncodeError::*;

		match *self {
			// In practice useless.
			BadBorrow(ref source) => Some(source),

			#[cfg(feature = "alloc")]
			CustomError(ref source) => Some(source.as_ref()),

			_ => None,
		}
	}
}
