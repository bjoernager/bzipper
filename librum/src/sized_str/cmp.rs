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

use crate::SizedStr;

use core::cmp::Ordering;

#[cfg(feature = "alloc")]
use alloc::string::String;

impl<const N: usize> Ord for SizedStr<N> {
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_str().cmp(other.as_str())
	}
}

impl<const N: usize, const M: usize> PartialEq<SizedStr<M>> for SizedStr<N> {
	#[inline(always)]
	fn eq(&self, other: &SizedStr<M>) -> bool {
		self.as_str() == other.as_str()
	}
}

impl<const N: usize> PartialEq<&str> for SizedStr<N> {
	#[inline(always)]
	fn eq(&self, other: &&str) -> bool {
		self.as_str() == *other
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<const N: usize> PartialEq<String> for SizedStr<N> {
	#[inline(always)]
	fn eq(&self, other: &String) -> bool {
		self.as_str() == other.as_str()
	}
}

impl<const N: usize, const M: usize> PartialOrd<SizedStr<M>> for SizedStr<N> {
	#[inline(always)]
	fn partial_cmp(&self, other: &SizedStr<M>) -> Option<Ordering> {
		self.as_str().partial_cmp(other.as_str())
	}
}

impl<const N: usize> PartialOrd<&str> for SizedStr<N> {
	#[inline(always)]
	fn partial_cmp(&self, other: &&str) -> Option<Ordering> {
		self.as_str().partial_cmp(*other)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<const N: usize> PartialOrd<String> for SizedStr<N> {
	#[inline(always)]
	fn partial_cmp(&self, other: &String) -> Option<Ordering> {
		self.as_str().partial_cmp(other.as_str())
	}
}
