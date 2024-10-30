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

use core::error::Error;
use core::fmt::{self, Display, Formatter};

/// An invalid UTF-16 sequence was encountered.
#[derive(Debug)]
pub struct Utf16Error {
	/// The invalid UTF-16 hextet.
	pub value: u16,

	/// The index of the invalid hextet.
	pub index: usize,
}

impl Display for Utf16Error {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "found invalid utf-16 hextet {:#04X} at offset ({})", self.value, self.index)
	}
}

impl Error for Utf16Error { }
