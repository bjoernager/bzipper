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

use core::cell::Cell;
use core::fmt::{Debug, Formatter};

/// Byte stream suitable for deserialisation.
///
/// This type borrows a buffer, keeping track internally of the used bytes.
pub struct Dstream<'a> {
	pub(in crate) data: &'a [u8],
	pub(in crate) pos:  Cell<usize>,
}

impl<'a> Dstream<'a> {
	/// Constructs a new byte stream.
	#[inline(always)]
	#[must_use]
	pub const fn new(data: &'a [u8]) -> Self { Self { data, pos: Cell::new(0x0) } }

	/// Takes (borrows) raw bytes from the stream.
	#[inline]
	pub fn read(&self, count: usize) -> Result<&[u8]> {
		let rem = self.data.len() - self.pos.get();
		let req = count;

		if rem < req { return Err(Error::EndOfStream { req, rem }) }

		let start = self.pos.get();
		let stop  = start + req;

		self.pos.set(stop);

		let data = &self.data[start..stop];
		Ok(data)
	}

	/// Gets a pointer to the first byte in the stream.
	#[inline(always)]
	#[must_use]
	pub const fn as_ptr(&self) -> *const u8 { self.data.as_ptr() }

	/// Gets a slice of the stream.
	#[inline(always)]
	#[must_use]
	pub const fn as_slice(&self) -> &[u8] {
		let ptr = self.as_ptr();
		let len = self.len();

		unsafe { core::slice::from_raw_parts(ptr, len) }
	}

	/// Gets the length of the stream.
	#[inline(always)]
	#[must_use]
	pub const fn len(&self) -> usize { unsafe { self.pos.as_ptr().read() } }

	/// Tests if the stream is empty.
	///
	/// If no deserialisations have been made at the time of calling, this method returns `false`.
	#[inline(always)]
	#[must_use]
	pub const fn is_empty(&self) -> bool { self.len() == 0x0 }

	/// Tests if the stream is full.
	///
	/// Note that zero-sized types such as [`()`](unit) can still be deserialised from this stream.
	#[inline(always)]
	#[must_use]
	pub const fn is_full(&self) -> bool { self.len() == self.data.len() }
}

impl Debug for Dstream<'_> {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> core::fmt::Result { Debug::fmt(self.as_slice(), f) }
}

impl<'a> From<&'a [u8]> for Dstream<'a> {
	#[inline(always)]
	fn from(value: &'a [u8]) -> Self { Self::new(value) }
}

impl<'a> From<&'a mut [u8]> for Dstream<'a> {
	#[inline(always)]
	fn from(value: &'a mut [u8]) -> Self { Self::new(value) }
}

impl PartialEq for Dstream<'_> {
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool { self.as_slice() == other.as_slice() }
}

impl PartialEq<&[u8]> for Dstream<'_> {
	#[inline(always)]
	fn eq(&self, other: &&[u8]) -> bool { self.as_slice() == *other }
}

impl<const N: usize> PartialEq<[u8; N]> for Dstream<'_> {
	#[inline(always)]
	fn eq(&self, other: &[u8; N]) -> bool { self.as_slice() == other.as_slice() }
}
