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

/// A fixed-size buffer was too small.
///
/// Some data types use a statically-sized buffer whilst still allowing for partial usage of this buffer (e.g. [`SizedSlice`](crate::SizedSlice)).
///
/// Taking `SizedSlice` as an example, it encodes its actual length before encoding each of its elements.
/// It is allowed for any smaller-sized `SizedSlice` instance to decode a larger-sized encoding **if** the actual length still fits.
/// If not, then this error type is used to denote the error state.
#[derive(Debug)]
#[must_use]
pub struct SizeError {
	/// The total capacity of the buffer.
	pub cap: usize,

	/// The required amount of elements.
	pub len: usize,
}

impl Display for SizeError {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "collection of size ({}) cannot hold ({}) elements", self.cap, self.len)
	}
}

impl Error for SizeError { }
