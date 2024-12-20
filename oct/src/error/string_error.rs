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

use crate::error::{LengthError, Utf16Error, Utf8Error};

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
	SmallBuffer(LengthError),
}

impl Display for StringError {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match *self {
			Self::BadUtf16(ref e)
			=> write!(f, "bad utf-16: {e}"),

			Self::BadUtf8(ref e)
			=> write!(f, "bad utf-8: {e}"),

			Self::SmallBuffer(ref e)
			=> write!(f, "buffer too small: {e}"),
		}
	}
}

impl Error for StringError {
	#[inline]
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match *self {
			Self::BadUtf16(ref e) => Some(e),

			Self::BadUtf8(ref e) => Some(e),

			Self::SmallBuffer(ref e) => Some(e),
		}
	}
}
