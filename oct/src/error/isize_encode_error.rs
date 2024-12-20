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

/// An [`isize`] value could not be decoded.
///
/// Any `isize` object that can fit in an [`i16`] can be encoded successfully.
#[derive(Debug)]
#[must_use]
pub struct IsizeEncodeError(
	/// The unencodable value.
	pub isize,
);

impl Display for IsizeEncodeError {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(
			f,
			"signed size value ({}) cannot be serialised: must be in the range ({}) to ({})",
			self.0,
			i16::MIN,
			i16::MAX,
		)
	}
}

impl Error for IsizeEncodeError { }
