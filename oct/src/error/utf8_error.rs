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

/// An invalid UTF-8 sequence was encountered.
#[derive(Debug)]
#[must_use]
pub struct Utf8Error {
	/// The invalid UTF-8 octet.
	pub value: u8,

	/// The index of the invalid octet.
	pub index: usize,
}

impl Display for Utf8Error {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "found invalid utf-8 octet {:#02X} at offset ({})", self.value, self.index)
	}
}

impl Error for Utf8Error { }
