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

use crate::{Error, Result};

use core::fmt::{Debug, Formatter};

/// Byte stream for deserialisation.
///
/// This type borrows a byte slice (hence [`new`](Dstream::new)), keeping track internally of the used bytes.
#[derive(Clone)]
pub struct Dstream<'a> {
	data: &'a [u8],
	len:  usize,
}

impl<'a> Dstream<'a> {
	/// Constructs a new byte stream.
	#[inline(always)]
	#[must_use]
	pub fn new<T: AsRef<[u8]> + ?Sized>(buf: &'a T) -> Self { Self {
		data: buf.as_ref(),
		len:  buf.as_ref().len(),
	} }

	/// Takes bytes from the stream.
	///
	/// # Errors
	///
	/// If the internal buffer doesn't hold at least the requested amount of bytes, an [`EndOfStream`](Error::EndOfStream) error is returned.
	pub fn take(&mut self, req: usize) -> Result<&[u8]> {
		let rem = self.len;

		if rem < req { return Err(Error::EndOfStream { req, rem } ) }

		let start = self.data.len() - rem;
		let stop  = start + req;

		self.len -= req;
		Ok(&self.data[start..stop])
	}

	/// Takes a single byte from the stream.
	///
	/// # Errors
	///
	/// If the internal buffer doesn't hold at least the requested amount of bytes, an [`EndOfStream`](Error::EndOfStream) error is returned.
	pub fn take_byte(&mut self) -> Result<u8> {
		const LEN: usize = 0x1;

		if self.len < LEN { return Err(Error::EndOfStream { req: LEN, rem: self.len } ) }

		self.len -= LEN;

		let index = self.data.len() - self.len;
		Ok(self.data[index])
	}
}

impl Debug for Dstream<'_> {
	fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
		self.data.fmt(f)
	}
}

impl<'a, T: AsRef<[u8]>> From<&'a T> for Dstream<'a> {
	fn from(value: &'a T) -> Self { Self::new(value) }
}
