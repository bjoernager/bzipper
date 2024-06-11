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

/// Byte stream for serialisation.
///
/// This type borrows a byte slice (hence [`new`](Sstream::new)), keeping track internally of the used bytes.
#[derive(Eq, PartialEq)]
pub struct Sstream<'a> {
	data: &'a mut [u8],
	len:  usize
}

impl<'a> Sstream<'a> {
	/// Constructs a new byte stream.
	///
	/// If the borrowed slice already contains data, this may overwritten by subsequent serialisations.
	#[inline(always)]
	#[must_use]
	pub fn new(data: &'a mut [u8]) -> Self { Self { data, len: 0x0 } }

	/// Extends the byte stream.
	///
	/// # Errors
	///
	/// If the stream cannot hold the requested bytes, an [`EndOfStream`](Error::EndOfStream) instance is returned.
	pub fn add(&mut self, extra: &[u8]) -> Result<usize> {
		let rem = self.data.len() - self.len;
		let req = extra.len();

		if rem.checked_sub(req).is_none() {
			return Err(Error::EndOfStream { req, rem });
		}

		let start = self.len;
		let stop  = self.len + req;

		self.len += req;
		self.data[start..stop].copy_from_slice(extra);

		Ok(req)
	}

	/// Extends the byte stream by a single byte.
	///
	/// # Errors
	///
	/// If the stream cannot hold the byte, an [`EndOfStream`](Error::EndOfStream) instance is returned.
	pub fn add_byte(&mut self, extra: u8) -> Result<usize> {
		self.add(&[extra])
	}

	/// Yields the length of the stream.
	///
	/// That is, the amount of bytes written so far.
	#[inline(always)]
	#[must_use]
	pub const fn len(&self) -> usize { self.len }

	/// Tests if the stream is empty.
	#[inline(always)]
	#[must_use]
	pub const fn is_empty(&self) -> bool { self.len == 0x0 }

	/// Returns a slice to the stream contents.
	///
	/// This includes all previously written bytes.
	#[inline(always)]
	#[must_use]
	pub fn as_slice(&self) -> &[u8] { &self.data[0x0..self.len] }
}

impl AsRef<[u8]> for Sstream<'_> {
	#[inline(always)]
	fn as_ref(&self) -> &[u8] { self.as_slice() }
}

impl Debug for Sstream<'_> {
	fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
		self.data.fmt(f)
	}
}
