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

/// The [`SystemTime`](std::time::SystemTime) type could not represent a UNIX timestamp.
///
/// Note that a UNIX timestamp is here defined as a signed, 64-bit integer denoting a difference of time to 1 january 1970, as measured in Greenwich using seconds.
/// This error should therefore not occur on systems that use the same or a more precise counter.
#[cfg_attr(doc, doc(cfg(feature = "std")))]
#[derive(Debug)]
#[must_use]
pub struct SystemTimeDecodeError {
	/// The unrepresentable timestamp.
	pub timestamp: i64,
}

#[cfg_attr(doc, doc(cfg(feature = "std")))]
impl Display for SystemTimeDecodeError {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "could not represent `{}` as a system timestamp", self.timestamp)
	}
}

#[cfg_attr(doc, doc(cfg(feature = "std")))]
impl Error for SystemTimeDecodeError { }
