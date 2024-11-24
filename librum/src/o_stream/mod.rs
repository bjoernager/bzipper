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

use core::ptr::copy_nonoverlapping;

/// Byte stream suitable for output.
pub struct OStream<'a> {
	buf: &'a mut [u8],
	pos: usize,
}

impl<'a> OStream<'a> {
	/// Constructs a new o-stream.
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
	pub fn write(&mut self, data: &[u8]) {
		let remaining = self.buf.len() - self.pos;
		let count     = data.len();

		assert!(
			remaining >= count,
			"cannot write ({count}) bytes at ({}) to stream with capacity of ({})",
			self.pos,
			self.buf.len(),
		);

		unsafe {
			let src = data.as_ptr();
			let dst = self.buf.as_mut_ptr().add(self.pos);

			copy_nonoverlapping(src, dst, count);
		}

		self.pos += count;
	}

	/// Closes the stream.
	///
	/// The total ammount of bytes written is returned.
	#[inline(always)]
	pub const fn close(self) -> usize {
		let Self { pos, .. } = self;

		pos
	}
}
