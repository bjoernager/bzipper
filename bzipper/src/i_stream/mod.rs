// Copyright 2024 Gabriel Bjørnager Jensen.
//
// This file is part of bZipper.
//
// bZipper is free software: you can redistribute
// it and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// bZipper is distributed in the hope that it will
// be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Less-
// er General Public License along with bZipper. If
// not, see <https://www.gnu.org/licenses/>.

use core::slice;

/// Byte stream suitable for input.
pub struct IStream<'a> {
	buf: &'a [u8],
	pos: usize,
}

impl<'a> IStream<'a> {
	/// Constructs a new i-stream.
	#[inline(always)]
	#[must_use]
	pub fn new(buf: &'a [u8]) -> Self {
		Self { buf, pos: 0x0 }
	}

	/// Reads bytes from the stream.
	///
	/// # Panics
	///
	/// If the requested amount of bytes could not exactly be read, then this method will panic.
	#[inline]
	pub fn read(&mut self, count: usize) -> &[u8] {
		let remaining = self.buf.len() - self.pos;

		assert!(
			remaining >= count,
			"cannot read ({count}) bytes at ({}) from stream with capacity of ({})",
			self.pos,
			self.buf.len(),
		);

		let data = unsafe {
			let ptr = self.buf.as_ptr().add(self.pos);

			slice::from_raw_parts(ptr, count)
		};

		self.pos += count;

		data
	}

	/// Closes the stream.
	///
	/// The total ammount of bytes read is returned.
	#[inline(always)]
	pub const fn close(self) -> usize {
		let Self { pos, .. } = self;

		pos
	}
}
