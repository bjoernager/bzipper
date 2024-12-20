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

/// A collection buffer was too small to contain all of its elements.
///
/// Some data types use a statically-sized buffer whilst still allowing for partial usage of this buffer (e.g. [`SizedSlice`](crate::SizedSlice)).
/// These types should return this error in cases where their size limit has exceeded.
///
/// Taking [`SizedSlice`](crate::SizedSlice) as an example, it encodes its actual length before encoding its elements.
/// It is allowed for any smaller-sized `SizedSlice` instance to decode a larger-sized encoding **if** the actual length is still within bounds.
/// Otherwise, this error type is used to denote the error state.
#[derive(Debug)]
#[must_use]
pub struct LengthError {
	/// The total capacity of the buffer.
	pub capacity: usize,

	/// The required amount of elements.
	pub len: usize,
}

impl Display for LengthError {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "collection of size ({}) cannot hold ({}) elements", self.capacity, self.len)
	}
}

impl Error for LengthError { }
