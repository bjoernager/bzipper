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

use crate::error::OutputError;

use core::borrow::Borrow;
use core::ptr::copy_nonoverlapping;
use core::slice;

/// Byte stream suitable for writing.
#[derive(Eq)]
pub struct Output<'a> {
	buf: &'a mut [u8],
	pos: usize,
}

impl<'a> Output<'a> {
	/// Constructs a new output stream.
	#[inline(always)]
	#[must_use]
	pub const fn new(buf: &'a mut [u8]) -> Self {
		Self { buf, pos: 0x0 }
	}

	/// Writes bytes to the stream.
	///
	/// # Panics
	///
	/// If the requested amount of bytes could not exactly be written, then this method will panic.
	#[inline]
	pub const fn write(&mut self, data: &[u8]) -> Result<(), OutputError> {
		let remaining = self.buf.len() - self.pos;
		let count     = data.len();

		if remaining < count {
			return Err(OutputError {
				capacity: self.buf.len(),
				position: self.pos,
				count,
			});
		}

		unsafe {
			let src = data.as_ptr();
			let dst = self.buf.as_mut_ptr().add(self.pos);

			copy_nonoverlapping(src, dst, count);
		}

		self.pos += count;

		Ok(())
	}

	/// Gets a pointer to the first byte of the output stream.
	#[inline(always)]
	#[must_use]
	pub const fn as_ptr(&self) -> *const u8 {
		self.buf.as_ptr()
	}

	/// Gets a slice of the written bytes in the output stream.
	#[inline(always)]
	#[must_use]
	pub const fn as_slice(&self) -> &[u8] {
		unsafe {
			let ptr = self.as_ptr();
			let len = self.position();

			slice::from_raw_parts(ptr, len)
		}
	}

	/// Retrieves the maximum capacity of the output stream.
	#[inline(always)]
	#[must_use]
	pub const fn capacity(&self) -> usize {
		self.buf.len()
	}

	/// Retrieves the remaining, free capacity of the output stream.
	#[inline(always)]
	#[must_use]
	pub const fn remaining(&self) -> usize {
		// SAFETY: The cursor position can never exceed the
		// stream's capacity.
		unsafe { self.capacity().unchecked_sub(self.position()) }
	}

	/// Retrieves the current cursor position of the output stream.
	#[inline(always)]
	#[must_use]
	pub const fn position(&self) -> usize {
		self.pos
	}
}

impl AsRef<[u8]> for Output<'_> {
	#[inline(always)]
	fn as_ref(&self) -> &[u8] {
		self.as_slice()
	}
}

impl Borrow<[u8]> for Output<'_> {
	#[inline(always)]
	fn borrow(&self) -> &[u8] {
		self.as_slice()
	}
}

impl PartialEq for Output<'_> {
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool {
		self.as_slice() == other.as_slice()
	}
}

impl PartialEq<[u8]> for Output<'_> {
	#[inline(always)]
	fn eq(&self, other: &[u8]) -> bool {
		self.as_slice() == other
	}
}

impl PartialEq<&[u8]> for Output<'_> {
	#[inline(always)]
	fn eq(&self, other: &&[u8]) -> bool {
		self.as_slice() == *other
	}
}

impl PartialEq<&mut [u8]> for Output<'_> {
	#[inline(always)]
	fn eq(&self, other: &&mut [u8]) -> bool {
		self.as_slice() == *other
	}
}
