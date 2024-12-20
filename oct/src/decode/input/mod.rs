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

use crate::error::InputError;

use core::ptr::copy_nonoverlapping;
use core::slice;

/// Byte stream suitable for reading.
pub struct Input<'a> {
	buf: &'a [u8],
	pos: usize,
}

impl<'a> Input<'a> {
	/// Constructs a new input stream.
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
	pub const fn read(&mut self, count: usize) -> Result<&'a [u8], InputError> {
		let remaining = self.buf.len() - self.pos;

		if remaining < count {
			return Err(InputError {
				capacity: self.buf.len(),
				position: self.pos,
				count,
			});
		}

		let data = unsafe {
			let ptr = self.buf.as_ptr().add(self.pos);

			slice::from_raw_parts(ptr, count)
		};

		self.pos += count;

		Ok(data)
	}

	/// Reads bytes from the stream into a predefined buffer.
	///
	/// This method may be preferred over [`read`](Self::read) if the read data **is** directly needed, e.g. if all required transformations can be done in-place.
	///
	/// # Panics
	///
	/// If the provided buffer could not be completely filled, then this method will panic.
	#[inline]
	pub const fn read_into(&mut self, buf: &mut [u8]) -> Result<(), InputError> {
		let count     = buf.len();
		let remaining = self.remaining();

		if remaining < count {
			return Err(InputError {
				capacity: self.buf.len(),
				position: self.pos,
				count,
			});
		}

		unsafe {
			let src = self.buf.as_ptr().add(self.pos);
			let dst = buf.as_mut_ptr();

			copy_nonoverlapping(src, dst, count);
		}

		self.pos += count;

		Ok(())
	}

	/// Retrieves the maximum capacity of the input stream.
	#[inline(always)]
	#[must_use]
	pub const fn capacity(&self) -> usize {
		self.buf.len()
	}

	/// Retrieves the remaining, free capacity of the input stream.
	#[inline(always)]
	#[must_use]
	pub const fn remaining(&self) -> usize {
		// SAFETY: The cursor position can never exceed the
		// stream's capacity.
		unsafe { self.capacity().unchecked_sub(self.position()) }
	}

	/// Retrieves the current cursor position of the input stream.
	#[inline(always)]
	#[must_use]
	pub const fn position(&self) -> usize {
		self.pos
	}
}
