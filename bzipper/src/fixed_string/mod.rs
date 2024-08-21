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

#[cfg(test)]
mod test;

use crate::{Deserialise, Error, FixedIter, Serialise};

use core::cmp::Ordering;
use core::fmt::{Debug, Display, Formatter};
use core::mem::MaybeUninit;
use core::ops::{Deref, DerefMut, Index, IndexMut};
use core::slice::SliceIndex;
use core::str::FromStr;

#[cfg(feature = "alloc")]
use alloc::string::{String, ToString};

/// Owned string with maximum size.
///
/// This is in contrast to [String] -- which has no size limit in practice -- and [str], which is unsized.
///
/// # Examples
///
/// All instances of this type have the same size if the value of `N` is also the same.
/// This size can be found through
///
/// `size_of::<char>() * N + size_of::<usize>()`.
///
/// Therefore, the following four strings have -- despite their different contents -- the same total size.
///
/// ```
/// use bzipper::FixedString;
/// use std::str::FromStr;
///
/// let str0 = FixedString::<0xF>::new(); // Empty string.
/// let str1 = FixedString::<0xF>::from_str("Hello there!");
/// let str2 = FixedString::<0xF>::from_str("أنا من أوروپا");
/// let str3 = FixedString::<0xF>::from_str("COGITO ERGO SUM");
///
/// assert_eq!(size_of_val(&str0), size_of_val(&str1));
/// assert_eq!(size_of_val(&str0), size_of_val(&str2));
/// assert_eq!(size_of_val(&str0), size_of_val(&str3));
/// assert_eq!(size_of_val(&str1), size_of_val(&str2));
/// assert_eq!(size_of_val(&str1), size_of_val(&str3));
/// assert_eq!(size_of_val(&str2), size_of_val(&str3));
/// ```
///
/// These three strings can---by extend in theory---also interchange their contents between each other.
#[derive(Clone, Deserialise, Serialise)]
pub struct FixedString<const N: usize> {
	buf: [char; N],
	len: usize,
}

impl<const N: usize> FixedString<N> {
	/// Constructs a new, fixed-size string.
	///
	/// Note that it is only the internal buffer that is size-constrained.
	/// The string internally keeps track of the amount of used characters and acts accordingly.
	/// One must therefore only see the value of `N` as a size *limit*.
	///
	/// The constructed string will have a null length.
	/// All characters inside the internal buffer are instanced as `U+0000 NULL`.
	///
	/// For constructing a string with an already defined buffer, see [`from_chars`](Self::from_chars) and [`from_raw_parts`](Self::from_raw_parts).
	#[inline(always)]
	#[must_use]
	pub const fn new() -> Self { Self { buf: ['\0'; N], len: 0x0 } }

	/// Consumes the buffer into a fixed string.
	///
	/// The internal length is to `N`.
	/// For a similar function but with an explicit size, see [`from_raw_parts`](Self::from_raw_parts).
	#[inline(always)]
	#[must_use]
	pub const fn from_chars(buf: [char; N]) -> Self { Self { buf, len: N } }

	/// Constructs a fixed string from raw parts.
	#[inline(always)]
	#[must_use]
	pub const fn from_raw_parts(buf: [char; N], len: usize) -> Self { Self { buf, len } }

	/// Deconstructs a fixed string into its raw parts.
	#[inline(always)]
	#[must_use]
	pub const fn into_raw_parts(self) -> ([char; N], usize) { (self.buf, self.len) }

	/// Gets a pointer to the first character.
	#[inline(always)]
	#[must_use]
	pub const fn as_ptr(&self) -> *const char { self.buf.as_ptr() }

	/// Gets a mutable pointer to the first character.
	///
	/// This function can only be marked as `const` when `const_mut_refs` is implemented.
	/// See tracking issue [`#57349`](https://github.com/rust-lang/rust/issues/57349/) for more information.
	#[inline(always)]
	#[must_use]
	pub fn as_mut_ptr(&mut self) -> *mut char { self.buf.as_mut_ptr() }

	/// Borrows the string as a character slice.
	///
	/// The range of the returned slice only includes characters that are "used."
	/// For borrowing the entire internal buffer, see [`as_mut_slice`](Self::as_mut_slice).
	#[inline(always)]
	#[must_use]
	pub const fn as_slice(&self) -> &[char] {
		// We need to use `from_raw_parts` to mark this
		// function `const`.

		unsafe { core::slice::from_raw_parts(self.as_ptr(), self.len()) }
	}

	/// Mutably borrows the string as a character slice.
	///
	/// The range of the returned slice includes the entire internal buffer.
	/// For borrowing only the "used" characters, see [`as_slice`](Self::as_slice).
	#[inline(always)]
	#[must_use]
	pub fn as_mut_slice(&mut self) -> &mut [char] { &mut self.buf[0x0..self.len] }

	/// Returns the length of the string.
	///
	/// This does not necessarily equate to the value of `N`, as the internal buffer may be used but partially.
	#[inline(always)]
	#[must_use]
	pub const fn len(&self) -> usize { self.len }

	/// Checks if the string is empty, i.e. `self.len() == 0x0`.
	#[inline(always)]
	#[must_use]
	pub const fn is_empty(&self) -> bool { self.len() == 0x0 }

	/// Checks if the string is full, i.e. `self.len() == N`.
	#[inline(always)]
	#[must_use]
	pub const fn is_full(&self) -> bool { self.len() == N }

	/// Sets the internal length.
	///
	/// The length is compared with `N` to guarantee that bounds are honoured.
	///
	/// # Panics
	///
	/// This method panics if the value of `len` is greater than that of `N`.
	#[inline(always)]
	pub fn set_len(&mut self, len: usize) {
		assert!(self.len <= N, "cannot set length longer than the fixed size");
		self.len = len;
	}

	/// Borrows characters at the specified index.
	///
	/// If no element can be retrieved using the given index, [`None`] is returned instead.
	#[inline(always)]
	#[must_use]
	pub fn get<I: SliceIndex<[char]>>(&self, index: I) -> Option<&I::Output> { self.buf.get(index) }

	/// Borrows characters at the specified index *without* checking bounds.
	///
	/// For performing a similar operation *with* bounds checks, see [`get`](Self::get).
	///
	/// # Safety
	///
	/// If the given index points out of the bounds of the string, behaviour is undefined.
	#[inline(always)]
	#[must_use]
	pub unsafe fn get_unchecked<I: SliceIndex<[char]>>(&self, index: I) -> &I::Output { self.buf.get_unchecked(index) }

	/// Mutably borrows characters at the specified index.
	///
	/// If no element can be retrieved using the given index, [`None`] is returned instead.
	#[inline(always)]
	#[must_use]
	pub fn get_mut<I: SliceIndex<[char]>>(&mut self, index: I) -> Option<&mut I::Output> { self.buf.get_mut(index) }

	/// Mutably borrows characters at the specified index *without* checking bounds.
	///
	/// For performing a similar operation *with* bounds checks, see [`get_mut`](Self::get_mut)
	///
	/// # Safety
	///
	/// If the given index points out of the bounds of the string, behaviour is undefined.
	#[inline(always)]
	#[must_use]
	pub unsafe fn get_unchecked_mut<I: SliceIndex<[char]>>(&mut self, index: I) -> &I::Output { self.buf.get_unchecked_mut(index) }

	/// Pushes a character into the string.
	///
	/// The internal length is updated accordingly.
	///
	/// # Panics
	///
	/// If the string cannot hold any more character (i.e. it is full), this method will panic.
	#[inline(always)]
	pub fn push(&mut self, c: char) {
		assert!(!self.is_full(), "cannot push character to full string");

		self.buf[self.len] = c;
		self.len += 0x1;
	}

	/// Pops a character from the string.
	///
	/// The internal length is updated accordingly.
	///
	/// If no characters are left (i.e. the string is empty), an instance of [`None`] is returned.
	#[inline(always)]
	pub fn pop(&mut self) -> Option<char> {
		self.len
			.checked_sub(0x1)
			.map(|len| {
				let c = self.buf[self.len];
				self.len = len;

				c
			})
	}

	/// Returns an iterator to the contained characters.
	///
	/// This iterator only covers "used" character.
	/// See [`iter_mut`](Self::iter_mut) for borrowing the entire buffer.
	#[inline(always)]
	pub fn iter(&self) -> core::slice::Iter<'_, char> { self.as_slice().iter() }

	/// Returns a mutable iterator to the contained characters.
	///
	/// This iterator covers the entire internal buffer.
	/// See [`iter`](Self::iter) for borrowing only "used" characters.
	#[inline(always)]
	pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, char> { self.as_mut_slice().iter_mut() }
}

impl<const N: usize> AsMut<[char]> for FixedString<N> {
	#[inline(always)]
	fn as_mut(&mut self) -> &mut [char] { self.as_mut_slice() }
}

impl<const N: usize> AsRef<[char]> for FixedString<N> {
	#[inline(always)]
	fn as_ref(&self) -> &[char] { self.as_slice() }
}

impl<const N: usize> Debug for FixedString<N> {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
		write!(f, "\"")?;
		for c in self { write!(f, "{}", c.escape_debug())? }
		write!(f, "\"")?;

		Ok(())
	}
}

impl<const N: usize> Default for FixedString<N> {
	#[inline(always)]
	fn default() -> Self { Self { buf: [Default::default(); N], len: 0x0 } }
}

/// See [`as_slice`](Self::as_slice).
impl<const N: usize> Deref for FixedString<N> {
	type Target = [char];

	#[inline(always)]
	fn deref(&self) -> &Self::Target { self.as_slice() }
}

/// See [`as_mut_slice`](Self::as_mut_slice).
impl<const N: usize> DerefMut for FixedString<N> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target { self.as_mut_slice() }
}

impl<const N: usize> Display for FixedString<N> {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
		for c in self { write!(f, "{c}")? }

		Ok(())
	}
}

impl<const N: usize> Eq for FixedString<N> { }

impl<const N: usize> From<[char; N]> for FixedString<N> {
	#[inline(always)]
	fn from(value: [char; N]) -> Self { Self::from_chars(value) }
}

impl<const N: usize> FromStr for FixedString<N> {
	type Err = Error;

	#[inline]
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut buf = [Default::default(); N];
		let     len = s.chars().count();

		for (i, c) in s.chars().enumerate() {
			if i >= N { return Err(Error::ArrayTooShort { req: len, len: N }) }

			buf[i] = c;
		}

		Ok(Self { buf, len })
	}
}

impl<I: SliceIndex<[char]>, const N: usize> Index<I> for FixedString<N> {
	type Output = I::Output;

	#[inline(always)]
	fn index(&self, index: I) -> &Self::Output { self.get(index).unwrap() }
}

impl<I: SliceIndex<[char]>, const N: usize> IndexMut<I> for FixedString<N> {
	#[inline(always)]
	fn index_mut(&mut self, index: I) -> &mut Self::Output { self.get_mut(index).unwrap() }
}

impl<const N: usize> IntoIterator for FixedString<N> {
	type Item = char;

	type IntoIter = FixedIter<char, N>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		FixedIter {
			buf: unsafe { self.buf.as_ptr().cast::<[MaybeUninit<char>; N]>().read() },

			pos: 0x0,
			len: self.len,
		}
	}
}

impl<'a, const N: usize> IntoIterator for &'a FixedString<N> {
	type Item = &'a char;

	type IntoIter = core::slice::Iter<'a, char>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter { self.iter() }
}

impl<'a, const N: usize> IntoIterator for &'a mut FixedString<N> {
	type Item = &'a mut char;

	type IntoIter = core::slice::IterMut<'a, char>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter { self.iter_mut() }
}

impl<const N: usize> Ord for FixedString<N> {
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering { self.partial_cmp(other).unwrap() }
}

impl<const N: usize, const M: usize> PartialEq<FixedString<M>> for FixedString<N> {
	#[inline]
	fn eq(&self, other: &FixedString<M>) -> bool {
		if self.len() != other.len() { return false };

		for i in 0x0..self.len() {
			if self.buf[i] != other.buf[i] { return false };
		}

		true
	}
}

impl<const N: usize> PartialEq<&str> for FixedString<N> {
	#[inline]
	fn eq(&self, other: &&str) -> bool {
		for (i, c) in other.chars().enumerate() {
			if self.get(i) != Some(&c) { return false };
		}

		true
	}
}

impl<const N: usize, const M: usize> PartialOrd<FixedString<M>> for FixedString<N> {
	fn partial_cmp(&self, other: &FixedString<M>) -> Option<Ordering> {
		let len = self.len().max(other.len());

		for i in 0x0..len {
			let lc = self.get(i);
			let rc = other.get(i);

			match (lc, rc) {
				(None, None)    => return Some(Ordering::Equal),
				(Some(_), None) => return Some(Ordering::Greater),
				(None, Some(_)) => return Some(Ordering::Less),

				(Some(lc), Some(rc)) => {
					match lc.partial_cmp(rc) {
						Some(Ordering::Equal) => {},
						ordering              => return ordering
					}
				}
			}
		}

		Some(Ordering::Equal)
	}
}

impl<const N: usize> TryFrom<&str> for FixedString<N> {
	type Error = <Self as FromStr>::Err;

	#[inline(always)]
	fn try_from(value: &str) -> Result<Self, Self::Error> { Self::from_str(value) }
}

#[cfg(feature = "alloc")]
impl<const N: usize> TryFrom<String> for FixedString<N> {
	type Error = <Self as FromStr>::Err;

	#[inline(always)]
	fn try_from(value: String) -> Result<Self, Self::Error> { Self::from_str(&value) }
}

#[cfg(feature = "alloc")]
impl<const N: usize> From<FixedString<N>> for String {
	#[inline(always)]
	fn from(value: FixedString<N>) -> Self { value.to_string() }
}
