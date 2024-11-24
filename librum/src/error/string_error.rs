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

use crate::error::{SizeError, Utf16Error, Utf8Error};

use core::error::Error;
use core::fmt::{self, Display, Formatter};

/// String error variants.
#[derive(Debug)]
#[non_exhaustive]
#[must_use]
pub enum StringError {
	/// An invalid UTF-16 sequence was encountered.
	BadUtf16(Utf16Error),

	/// An invalid UTF-8 sequence was encountered.
	BadUtf8(Utf8Error),

	/// A fixed-size buffer was too small.
	SmallBuffer(SizeError),
}

impl Display for StringError {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		use StringError::*;

		match *self {
			BadUtf16(ref e)
			=> write!(f, "bad utf-16: {e}"),

			BadUtf8(ref e)
			=> write!(f, "bad utf-8: {e}"),

			SmallBuffer(ref e)
			=> write!(f, "buffer too small: {e}"),
		}
	}
}

impl Error for StringError {
	#[inline]
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		use StringError::*;

		match *self {
			BadUtf16(ref e) => Some(e),

			BadUtf8(ref e) => Some(e),

			SmallBuffer(ref e) => Some(e),
		}
	}
}
