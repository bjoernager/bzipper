// Copyright 2024 Gabriel Bjørnager Jensen.
//
// This file is part of bzipper.
//
// bzipper is free software: you can redistribute
// it and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// bzipper is distributed in the hope that it will
// be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Less-
// er General Public License along with bzipper. If
// not, see <https://www.gnu.org/licenses/>.

use crate::{Deserialise, Error, Result};

use core::cell::Cell;

/// Byte stream for deserialisation.
///
/// This type borrows a slice, keeping track internally of the used bytes.
pub struct Dstream<'a> {
	data: &'a [u8],
	pos:  Cell<usize>,
}

impl<'a> Dstream<'a> {
	/// Constructs a new byte stream.
	#[inline(always)]
	#[must_use]
	pub const fn new(data: &'a [u8]) -> Self { Self { data, pos: Cell::new(0x0) } }

	/// Deserialises an object from the stream.
	///
	/// # Errors
	///
	/// If the stream doesn't hold at least the amount of bytes specified by [`SERIALISED_SIZE`](crate::Serialise::SERIALISED_SIZE), an [`EndOfStream`](Error::EndOfStream) error is returned.
	#[inline]
	pub fn take<T: Deserialise>(&self) -> Result<T> {
		let rem = self.data.len() - self.pos.get();
		let req = T::SERIALISED_SIZE;

		if rem < req { return Err(Error::EndOfStream { req, rem }) };

		let start = self.pos.get();
		let stop  = start + req;

		self.pos.set(stop);
		T::deserialise(&self.data[start..stop])
	}
}
