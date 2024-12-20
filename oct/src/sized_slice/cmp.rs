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

use crate::SizedSlice;

use core::cmp::Ordering;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

impl<T: Eq, const N: usize> Eq for SizedSlice<T, N> { }

impl<T: Ord, const N: usize> Ord for SizedSlice<T, N> {
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_slice().cmp(other.as_slice())
	}
}

impl<T: PartialEq<U>, U: PartialEq<T>, const N: usize, const M: usize> PartialEq<SizedSlice<U, M>> for SizedSlice<T, N> {
	#[inline(always)]
	fn eq(&self, other: &SizedSlice<U, M>) -> bool {
		self.as_slice() == other.as_slice()
	}
}

impl<T: PartialEq<U>, U: PartialEq<T>, const N: usize, const M: usize> PartialEq<[U; M]> for SizedSlice<T, N> {
	#[inline(always)]
	fn eq(&self, other: &[U; M]) -> bool {
		self.as_slice() == other.as_slice()
	}
}

impl<T: PartialEq<U>, U: PartialEq<T>, const N: usize> PartialEq<&[U]> for SizedSlice<T, N> {
	#[inline(always)]
	fn eq(&self, other: &&[U]) -> bool {
		self.as_slice() == *other
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: PartialEq<U>, U: PartialEq<T>, const N: usize> PartialEq<Vec<U>> for SizedSlice<T, N> {
	#[inline(always)]
	fn eq(&self, other: &Vec<U>) -> bool {
		self.as_slice() == other.as_slice()
	}
}

impl<T: PartialOrd, const N: usize, const M: usize> PartialOrd<SizedSlice<T, M>> for SizedSlice<T, N> {
	#[inline(always)]
	fn partial_cmp(&self, other: &SizedSlice<T, M>) -> Option<Ordering> {
		self.as_slice().partial_cmp(other.as_slice())
	}
}

impl<T: PartialOrd, const N: usize, const M: usize> PartialOrd<[T; M]> for SizedSlice<T, N> {
	#[inline(always)]
	fn partial_cmp(&self, other: &[T; M]) -> Option<Ordering> {
		self.as_slice().partial_cmp(other.as_slice())
	}
}

impl<T: PartialOrd, const N: usize> PartialOrd<&[T]> for SizedSlice<T, N> {
	#[inline(always)]
	fn partial_cmp(&self, other: &&[T]) -> Option<Ordering> {
		self.as_slice().partial_cmp(*other)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: PartialOrd, const N: usize> PartialOrd<Vec<T>> for SizedSlice<T, N> {
	#[inline(always)]
	fn partial_cmp(&self, other: &Vec<T>) -> Option<Ordering> {
		self.as_slice().partial_cmp(other.as_slice())
	}
}
