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

use core::iter::{DoubleEndedIterator, ExactSizeIterator, FusedIterator};
use core::mem::MaybeUninit;
use core::slice;

/// Iterator to a sized slice.
#[must_use]
pub struct SizedIter<T, const N: usize> {
	buf: [MaybeUninit<T>; N],

	pos: usize,
	len: usize,
}

impl<T, const N: usize> SizedIter<T, N> {
	/// Constructs a new, fixed-size iterator.
	#[inline(always)]
	pub(crate) const unsafe fn new(buf: [MaybeUninit<T>; N], len: usize) -> Self {
		debug_assert!(len <= N, "cannot construct iterator longer than its capacity");

		Self { buf, pos: 0x0, len }
	}

	/// Gets a slice of the remaining elements.
	#[inline(always)]
	pub const fn as_slice(&self) -> &[T] {
		unsafe {
			let ptr = self.buf
				.as_ptr()
				.add(self.pos)
				.cast();

			slice::from_raw_parts(ptr, self.len)
		}
	}

	/// Gets a mutable slice of the remaining elements.
	#[inline(always)]
	pub const fn as_mut_slice(&mut self) -> &mut [T] {
		unsafe {
			let ptr = self.buf
				.as_mut_ptr()
				.add(self.pos)
				.cast();

			slice::from_raw_parts_mut(ptr, self.len)
		}
	}
}

impl<T, const N: usize> AsMut<[T]> for SizedIter<T, N> {
	#[inline(always)]
	fn as_mut(&mut self) -> &mut [T] {
		self.as_mut_slice()
	}
}

impl<T, const N: usize> AsRef<[T]> for SizedIter<T, N> {
	#[inline(always)]
	fn as_ref(&self) -> &[T] {
		self.as_slice()
	}
}

impl<T: Clone, const N: usize> Clone for SizedIter<T, N> {
	#[inline]
	fn clone(&self) -> Self {
		unsafe {
			let mut buf: [MaybeUninit<T>; N] = MaybeUninit::uninit().assume_init();
			let Self { pos, len, .. } = *self;

			let start = pos;
			let stop  = start.unchecked_add(len);

			for i in start..stop {
				let value = &*self.buf
					.as_ptr()
					.add(i)
					.cast::<T>();

				buf
					.get_unchecked_mut(i)
					.write(value.clone());
			}

			Self { buf, pos, len }
		}
	}
}

impl<T, const N: usize> DoubleEndedIterator for SizedIter<T, N> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		if self.len == 0x0 { return None };

		unsafe {
			let index = self.pos.unchecked_add(self.len);

			let item = self.buf
				.get_unchecked(index)
				.assume_init_read();

			self.len = self.len.unchecked_sub(0x1);

			Some(item)
		}
	}
}

impl<T, const N: usize> ExactSizeIterator for SizedIter<T, N> { }

impl<T, const N: usize> FusedIterator for SizedIter<T, N> { }

impl<T, const N: usize> Iterator for SizedIter<T, N> {
	type Item = T;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		if self.len == 0x0 { return None };

		unsafe {
			let index = self.pos;

			let item = self.buf
				.get_unchecked(index)
				.assume_init_read();

			self.pos = self.pos.unchecked_add(0x1);
			self.len = self.len.unchecked_sub(0x1);

			Some(item)
		}
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let rem = unsafe { self.len.unchecked_sub(self.pos) };

		(rem, Some(rem))
	}
}
