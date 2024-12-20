// Copyright 2024 Gabriel Bjørnager Jensen.
//
// This file is part of oct.
//
// oct is free software: you can redistribute it
// and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// oct is distributed in the hope that it will
// be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Less-
// er General Public License along with oct. If
// not, see <https://www.gnu.org/licenses/>.

use crate::{SizedIter, SizedSlice};

use core::mem::MaybeUninit;
use core::slice;

impl<T, const N: usize> FromIterator<T> for SizedSlice<T, N> {
	#[inline]
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		let mut iter = iter.into_iter();

		let mut buf = [const { MaybeUninit::<T>::uninit() };N];
		let mut len = 0x0;

		for item in &mut buf {
			let Some(value) = iter.next() else { break };
			item.write(value);

			len += 0x1;
		}

		Self { buf, len }
	}
}

impl<T, const N: usize> IntoIterator for SizedSlice<T, N> {
	type Item = T;

	type IntoIter = SizedIter<T, N>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		let Self { buf, len } = self;

		unsafe { SizedIter::new(buf, len) }
	}
}

impl<'a, T, const N: usize> IntoIterator for &'a SizedSlice<T, N> {
	type Item = &'a T;

	type IntoIter = slice::Iter<'a, T>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<'a, T, const N: usize> IntoIterator for &'a mut SizedSlice<T, N> {
	type Item = &'a mut T;

	type IntoIter = slice::IterMut<'a, T>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		self.iter_mut()
	}
}
