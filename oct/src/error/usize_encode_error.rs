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

/// A [`usize`] value could not be decoded.
///
/// Any `usize` object that can fit in an [`u16`] can be encoded successfully.
#[derive(Debug)]
#[must_use]
pub struct UsizeEncodeError(
	/// The unencodable value.
	pub usize,
);

impl Display for UsizeEncodeError {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(
			f,
			"unsigned size value ({}) cannot be serialised: must be at most ({})",
			self.0,
			u16::MAX,
		)
	}
}

impl Error for UsizeEncodeError { }
