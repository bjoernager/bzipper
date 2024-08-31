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

use crate::{Dstream, Error, Result};

use core::cell::Cell;
use core::fmt::{Debug, Formatter};

/// Byte stream suitable for serialisation.
///
/// This type mutably borrows a buffer, keeping track internally of the used bytes.
pub struct Sstream<'a> {
	pub(in crate) buf: &'a mut [u8],
	pub(in crate) pos: Cell<usize>,
}

impl<'a> Sstream<'a> {
	/// Constructs a new byte stream.
	#[inline(always)]
	#[must_use]
	pub fn new(buf: &'a mut [u8]) -> Self { Self { buf, pos: Cell::new(0x0) } }

	/// Appends raw bytes to the stream.
	#[inline]
	pub fn write(&mut self, bytes: &[u8]) -> Result<()> {
		let rem = self.buf.len() - self.pos.get();
		let req = bytes.len();

		if rem < req { return Err(Error::EndOfStream { req, rem }) }

		let start = self.pos.get();
		let stop  = start + req;

		self.pos.set(stop);

		let buf = &mut self.buf[start..stop];
		buf.copy_from_slice(bytes);

		Ok(())
	}

	/// Gets a pointer to the first byte in the stream.
	#[inline(always)]
	#[must_use]
	pub const fn as_ptr(&self) -> *const u8 { self.buf.as_ptr() }

	/// Gets an immutable slice of the stream.
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
	/// If no serialisations have been made so far, this method returns `false`.
	#[inline(always)]
	#[must_use]
	pub const fn is_empty(&self) -> bool { self.len() == 0x0 }

	/// Tests if the stream is full.
	///
	/// Note that zero-sized types such as [`()`](unit) can still be serialised into this stream.
	#[inline(always)]
	#[must_use]
	pub const fn is_full(&self) -> bool { self.len() == self.buf.len() }
}

impl Debug for Sstream<'_> {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> core::fmt::Result { Debug::fmt(self.as_slice(), f) }
}

impl<'a> From<&'a mut [u8]> for Sstream<'a> {
	#[inline(always)]
	fn from(value: &'a mut [u8]) -> Self { Self::new(value) }
}

impl PartialEq for Sstream<'_> {
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool { self.as_slice() == other.as_slice() }
}

impl PartialEq<&[u8]> for Sstream<'_> {
	#[inline(always)]
	fn eq(&self, other: &&[u8]) -> bool { self.as_slice() == *other }
}

impl<const N: usize> PartialEq<[u8; N]> for Sstream<'_> {
	#[inline(always)]
	fn eq(&self, other: &[u8; N]) -> bool { self.as_slice() == other.as_slice() }
}

impl<'a> From<Sstream<'a>> for Dstream<'a> {
	#[inline(always)]
	fn from(value: Sstream<'a>) -> Self { Self { data: value.buf, pos: value.pos } }
}
