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
	/// This method may be preferred over [`read_into`](Self::read_into) if the read data isn't directly needed, e.g. if an iterator is applied anyway to map the data.
	///
	/// # Panics
	///
	/// If the requested amount of bytes could not exactly be read, then this method will panic.
	#[inline]
	pub fn read(&mut self, count: usize) -> &'a [u8] {
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

	/// Reads bytes from the stream into a predefined buffer.
	///
	/// This method may be preferred over [`read`](Self::read) if the read data **is** directly needed, e.g. if all required transformations can be done in-place.
	///
	/// # Panics
	///
	/// If the provided buffer could not be completely filled, then this method will panic.
	#[inline]
	pub fn read_into(&mut self, buf: &mut [u8]) {
		let count     = buf.len();
		let remaining = self.buf.len() - self.pos;

		assert!(
			remaining >= count,
			"cannot read ({count}) bytes at ({}) from stream with capacity of ({})",
			self.pos,
			self.buf.len(),
		);

		unsafe {
			let src = self.buf.as_ptr().add(self.pos);
			let dst = buf.as_mut_ptr();

			copy_nonoverlapping(src, dst, count);
		}

		self.pos += count;
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
