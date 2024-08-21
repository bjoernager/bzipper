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

use crate::{Error, Result, Serialise};

use core::cell::Cell;

/// Byte stream for deserialisation.
///
/// This type borrows a slice, keeping track internally of the used bytes.
pub struct Sstream<'a> {
	buf: &'a mut [u8],
	pos: Cell<usize>,
}

impl<'a> Sstream<'a> {
	/// Constructs a new byte stream.
	#[inline(always)]
	#[must_use]
	pub fn new(buf: &'a mut [u8]) -> Self { Self { buf, pos: Cell::new(0x0) } }

	/// Extends the stream by appending a new serialisation.
	///
	/// # Errors
	///
	/// If the stream cannot hold any arbitrary serialisation of `T`, an [`EndOfStream`](Error::EndOfStream) instance is returned.
	#[inline]
	pub fn append<T: Serialise>(&mut self, value: &T) -> Result<()> {
		let rem = self.buf.len() - self.pos.get();
		let req = T::SERIALISED_SIZE;

		if rem < req { return Err(Error::EndOfStream { req, rem }) };

		let start = self.pos.get();
		let stop  = start + req;

		self.pos.set(stop);
		value.serialise(&mut self.buf[start..stop])
	}
}
