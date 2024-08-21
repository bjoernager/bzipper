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

use core::mem::MaybeUninit;

/// Iterator to a fixed vector.
///
/// This type is used by the [`FixedString`](crate::FixedString) type for iterating over an owned string.
#[must_use]
pub struct FixedIter<T, const N: usize> {
	pub(in crate) buf: [MaybeUninit<T>; N],

	pub(in crate) pos: usize,
	pub(in crate) len: usize,
}

impl<T, const N: usize> Iterator for FixedIter<T, N> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		if self.pos >= self.len { return None };

		let item = unsafe { self.buf[self.pos].assume_init_read() };
		self.pos += 0x1;

		Some(item)
	}
}
