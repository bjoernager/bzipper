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

use core::error::Error;
use core::fmt::{self, Display, Formatter};

/// An input-related error.
///
/// This structure is mainly returned by the [`read`](crate::decode::Input::read) and [`read_into`](crate::decode::Input::read_into) methods in [`decode::Input`](crate::decode::Input).
#[derive(Debug)]
#[must_use]
pub struct InputError {
	/// The total capacity of the output stream.
	pub capacity: usize,

	/// The cursor position of the requested read.
	pub position: usize,

	/// The requested amount of octets.
	pub count: usize,
}

impl Display for InputError {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(
			f,
			"cannot read ({}) bytes at ({}) from input stream with capacity of ({})",
			self.count,
			self.position,
			self.capacity,
		)
	}
}

impl Error for InputError { }
