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

#[derive(Debug)]
#[must_use]
/// An output-related error
///
/// This structure is mainly returned by the [`write`](crate::encode::Output::write) method in [`encode::Output`](crate::encode::Output).
pub struct OutputError {
	/// The total capacity of the output stream.
	pub capacity: usize,

	/// The cursor position of the requested write.
	pub position: usize,

	/// The requested amount of octets.
	pub count: usize,
}

impl Display for OutputError {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(
			f,
			"cannot write ({}) bytes at ({}) to output stream with capacity of ({})",
			self.count,
			self.position,
			self.capacity,
		)
	}
}

impl Error for OutputError { }
