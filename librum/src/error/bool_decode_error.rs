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

use core::error::Error;
use core::fmt::{self, Display, Formatter};

/// A boolean could not be decoded.
///
/// The encoding scheme for [`bool`] only defines the 8-bit values `0` and `1` (as `false` and `true`, respectively).
/// If any other 8-bit is read by <code>&lt;bool as [Decode](crate::Decode)&gt;::[decode](crate::Decode::decode)</code>, then an instance of this type is returned.
#[derive(Debug)]
#[must_use]
pub struct BoolDecodeError {
	/// The invalid value.
	pub value: u8,
}

impl Display for BoolDecodeError {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "expected boolean but got `{:#02X}`", self.value)
	}
}

impl Error for BoolDecodeError { }
