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

use core::error::Error;
use core::fmt::{self, Display, Formatter};

/// A character could not be decoded.
///
/// Unicode defines only the code points inclusively between `U+0000` and `U+D7FFF` as well between `U+E0000` and `U+10FFFF` as being valid.
/// UTF-32 (the format used by the [`char`] data type) additionally specifies that these code points are padded to 32 bits.
///
/// The encoding scheme used by `char` yields an untransformed representation (disregarding endian corrections), but this regrettably also leads to many bit patterns being undefined with respect to UTF-32.
/// If any of these values is read by <code>&lt;char as [Decode](crate::decode::Decode)&gt;::[decode](crate::decode::Decode::decode)</code>, then an instance of this error type is returned.
#[derive(Debug)]
#[must_use]
pub struct CharDecodeError {
	/// The undefined code point.
	pub code_point: u32,
}

impl Display for CharDecodeError {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "code point U+{:04X} is not defined", self.code_point)
	}
}

impl Error for CharDecodeError { }
