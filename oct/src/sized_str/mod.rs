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

#[cfg(test)]
mod tests;

use crate::SizedSlice;
use crate::error::{LengthError, StringError};

use core::fmt::{self, Debug, Display, Formatter};
use core::hash::{Hash, Hasher};
use core::ops::{Index, IndexMut};
use core::slice::SliceIndex;
use core::str::{Chars, CharIndices};

// Comparison facilities:
mod cmp;

// Encode/decode facilities:
mod code;

// Conversion facilities:
mod conv;

/// Stack-allocated string with maximum length.
///
/// This is in contrast to [`String`](alloc::string::String) -- which has no size limit in practice -- and [`prim@str`], which is unsized.
///
/// The string itself is encoded in UTF-8 for interoperability wtih Rust's standard string facilities, and partly due to memory concerns.
///
/// Keep in mind that the size limit specified by `N` denotes *bytes* (octets) and **not** *characters* -- i.e. a value of `8` may translate to between two and eight characters due to variable-length encoding.
///
/// See [`SizedSlice`] for an equivalent alternative to [`Vec`](alloc::vec::Vec).
///
/// # Examples
///
/// All instances of this type have the same size if the value of `N` is also the same.
/// Therefore, the following four strings have -- despite their different contents -- the same total size.
///
/// ```
/// use oct::SizedStr;
/// use std::str::FromStr;
///
/// let str0 = SizedStr::<0x40>::default(); // Empty string.
/// let str1 = SizedStr::<0x40>::from_str("Hello there!").unwrap();
/// let str2 = SizedStr::<0x40>::from_str("أنا من أوروپا").unwrap();
/// let str3 = SizedStr::<0x40>::from_str("COGITO ERGO SUM").unwrap();
///
/// assert_eq!(size_of_val(&str0), size_of_val(&str1));
/// assert_eq!(size_of_val(&str0), size_of_val(&str2));
/// assert_eq!(size_of_val(&str0), size_of_val(&str3));
/// assert_eq!(size_of_val(&str1), size_of_val(&str2));
/// assert_eq!(size_of_val(&str1), size_of_val(&str3));
/// assert_eq!(size_of_val(&str2), size_of_val(&str3));
/// ```
///
/// These three strings can -- by extend in theory -- also interchange their contents between each other.
#[derive(Clone, Default)]
pub struct SizedStr<const N: usize>(SizedSlice<u8, N>);

impl<const N: usize> SizedStr<N> {
	/// Constructs a new, fixed-size string.
	///
	/// Note that string is not required to completely fill out its size-constraint.
	///
	/// # Errors
	///
	/// If the internal buffer cannot contain the entirety of `s`, then an error is returned.
	#[inline(always)]
	pub const fn new(s: &str) -> Result<Self, StringError> {
		if s.len() > N { return Err(StringError::SmallBuffer(LengthError { capacity: N, len: s.len() })) };

		let this = unsafe { Self::from_utf8_unchecked(s.as_bytes()) };
		Ok(this)
	}

	/// Returns the total capacity of the string.
	///
	/// This is defined as being exactly the value of `N`.
	#[inline(always)]
	#[must_use]
	pub const fn capacity(&self) -> usize {
		self.0.capacity()
	}

	/// Returns the length of the string.
	///
	/// This does not necessarily equate to the value of `N`, as the internal buffer may be used but partially.
	#[inline(always)]
	#[must_use]
	pub const fn len(&self) -> usize {
		self.0.len()
	}

	/// Checks if the string is empty, i.e. no characters are contained.
	#[inline(always)]
	#[must_use]
	pub const fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	/// Checks if the string is full, i.e. it cannot hold any more characters.
	#[inline(always)]
	#[must_use]
	pub const fn is_full(&self) -> bool {
		self.0.is_full()
	}

	/// Returns an iterator of the string's characters.
	#[inline(always)]
	pub fn chars(&self) -> Chars {
		self.as_str().chars()
	}

	/// Returns an iterator of the string's characters along with their positions.
	#[inline(always)]
	pub fn char_indices(&self) -> CharIndices {
		self.as_str().char_indices()
	}
}

impl<const N: usize> Debug for SizedStr<N> {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Debug::fmt(self.as_str(), f)
	}
}

impl<const N: usize> Display for SizedStr<N> {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(self.as_str(), f)
	}
}

impl<const N: usize> Eq for SizedStr<N> { }

impl<const N: usize> FromIterator<char> for SizedStr<N> {
	#[inline]
	fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
		let mut buf = [0x00; N];
		let mut len = 0x0;

		for c in iter {
			let rem = N - len;
			let req = c.len_utf8();

			if rem < req { break }

			let start = len;
			let stop  = start + req;

			c.encode_utf8(&mut buf[start..stop]);

			len += req;
		}

		// SAFETY: All octets are initialised and come from
		// `char::encode_utf8`.
		unsafe { Self::from_raw_parts(buf, len) }
	}
}

impl<const N: usize> Hash for SizedStr<N> {
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.as_str().hash(state)
	}
}

impl<I: SliceIndex<str>, const N: usize> Index<I> for SizedStr<N> {
	type Output	= I::Output;

	#[inline(always)]
	fn index(&self, index: I) -> &Self::Output {
		self.get(index).unwrap()
	}
}

impl<I: SliceIndex<str>, const N: usize> IndexMut<I> for SizedStr<N> {
	#[inline(always)]
	fn index_mut(&mut self, index: I) -> &mut Self::Output {
		self.get_mut(index).unwrap()
	}
}
