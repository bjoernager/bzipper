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

use crate::{
	Deserialise,
	Dstream,
	Error,
	FixedStringIter,
	Serialise,
	Sstream,
};

use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter, Write};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

/// Owned string with maximum size.
///
/// This is in contrast to [String], which has no size limit is practice, and [str], which is unsized.
#[derive(Clone)]
pub struct FixedString<const N: usize> {
	buf: [char; N],
	len: usize,
}

impl<const N: usize> FixedString<N> {
	/// Constructs a new fixed string.
	///
	/// The contents of the provided string are copied into the internal buffer.
	/// All residual characters are instanced as U+0000 `NULL`.
	///
	/// # Errors
	///
	/// If the given string `s` cannot fit into `N` characters, an [`ArrayTooShort`](Error::ArrayTooShort) error is returned.
	pub fn new(s: &str) -> Result<Self, Error> {
		let mut buf = ['\0'; N];
		let     len = s.chars().count();

		for (i, c) in s.chars().enumerate() {
			if i >= N { return Err(Error::ArrayTooShort { req: len, len: N }) }

			buf[i] = c;
		}

		Ok(Self { buf, len })
	}

	/// Returns the length of the string.
	///
	/// This does not necessarily equal the value of `N`, as the internal buffer is not required to be used fully.
	#[inline(always)]
	#[must_use]
	pub const fn len(&self) -> usize { self.len }

	/// Checks if the string is empty, i.e. `self.len() == 0x0`.
	#[inline(always)]
	#[must_use]
	pub const fn is_empty(&self) -> bool { self.len == 0x0 }

	/// Borrows the character at the specified index.
	///
	/// If no element exists at that position, [`None`] is returned instead.
	#[inline]
	#[must_use]
	pub const fn get(&self, index: usize) -> Option<&char> {
		if index >= self.len {
			None
		} else {
			Some(&self.buf[index])
		}
	}

	/// Mutably borrows the character at the specified index.
	///
	/// If no element exists at that position, [`None`] is returned instead.
	#[inline]
	#[must_use]
	pub fn get_mut(&mut self, index: usize) -> Option<&mut char> {
		if index >= self.len {
			None
		} else {
			Some(&mut self.buf[index])
		}
	}

	/// Returns an iterator to the contained characters.
	#[inline(always)]
	pub fn iter(&self) -> std::slice::Iter<'_, char> { self.buf[0x0..self.len].iter() }

	/// Returns a mutable iterator to the contained characters.
	#[inline(always)]
	pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, char> { self.buf[0x0..self.len].iter_mut() }
}

impl<const N: usize> Debug for FixedString<N> {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		f.write_char('"')?;
		for c in self { write!(f, "{}", c.escape_debug())? }
		f.write_char('"')?;

		Ok(())
	}
}

impl<const N: usize> Deserialise for FixedString<N> {
	type Error = Error;

	fn deserialise(stream: &mut Dstream) -> Result<Self, Self::Error> {
		let len = usize::try_from(u64::deserialise(stream)?).unwrap();

		let data = stream.take(len)?;
		let s = std::str::from_utf8(data)
			.map_err(|e| Error::BadString { source: e })?;

		let len = s.chars().count();
		if len > N {
			return Err(Error::ArrayTooShort { req: len, len: N });
		}

		let mut buf = ['\0'; N];
		for (i, c) in s.chars().enumerate() {
			buf[i] = c;
		}

		Ok(Self { buf, len })
	}
}

impl<const N: usize> Default for FixedString<N> {
	#[inline(always)]
	fn default() -> Self { Self {
		buf: ['\0'; N],
		len: 0x0,
	} }
}

impl<const N: usize> Display for FixedString<N> {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		for c in self { write!(f, "{c}")? }

		Ok(())
	}
}

impl<const N: usize> Eq for FixedString<N> { }

impl<const N: usize> From<[char; N]> for FixedString<N> {
	fn from(value: [char; N]) -> Self { Self {
		buf: value,
		len: N,
	} }
}

impl<const N: usize> FromStr for FixedString<N> {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Error> { Self::new(s) }
}

impl<const N: usize> Index<usize> for FixedString<N> {
	type Output = char;

	fn index(&self, index: usize) -> &Self::Output { self.get(index).unwrap() }
}

impl<const N: usize> IndexMut<usize> for FixedString<N> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output { self.get_mut(index).unwrap() }
}

impl<const N: usize> IntoIterator for FixedString<N> {
	type Item = char;

	type IntoIter = FixedStringIter<N>;

	fn into_iter(self) -> Self::IntoIter {
		FixedStringIter {
			buf: self.buf,
			len: self.len,

			pos: Some(0x0),
		}
	}
}

impl<'a, const N: usize> IntoIterator for &'a FixedString<N> {
	type Item = &'a char;

	type IntoIter = std::slice::Iter<'a, char>;

	fn into_iter(self) -> Self::IntoIter { self.iter() }
}

impl<'a, const N: usize> IntoIterator for &'a mut FixedString<N> {
	type Item = &'a mut char;

	type IntoIter = std::slice::IterMut<'a, char>;

	fn into_iter(self) -> Self::IntoIter { self.iter_mut() }
}

impl<const N: usize> Ord for FixedString<N> {
	fn cmp(&self, other: &Self) -> Ordering { self.partial_cmp(other).unwrap() }
}

impl<const N: usize, const M: usize> PartialEq<FixedString<M>> for FixedString<N> {
	fn eq(&self, other: &FixedString<M>) -> bool {
		if self.len() != other.len() { return false };

		for i in 0x0..self.len() {
			if self.buf[i] != other.buf[i] { return false };
		}

		true
	}
}

impl<const N: usize> PartialEq<&str> for FixedString<N> {
	fn eq(&self, other: &&str) -> bool {
		for (i, c) in other.chars().enumerate() {
			if self.buf.get(i) != Some(&c) { return false };
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
					let ordering = lc.cmp(rc);

					if ordering != Ordering::Equal { return Some(ordering) };
				}
			}
		}

		Some(Ordering::Equal)
	}
}

impl<const N: usize> Serialise for FixedString<N> {
	const SERIALISE_LIMIT: usize = 0x4 * N;

	fn serialise(&self, stream: &mut Sstream) {
		let s: String = self.iter().collect();

		let len = u64::try_from(s.len()).unwrap();

		stream.append(&len.to_be_bytes());
		stream.append(&s.into_bytes());
	}
}

impl<const N: usize> TryFrom<&str> for FixedString<N> {
	type Error = Error;

	#[inline(always)]
	fn try_from(value: &str) -> Result<Self, Self::Error> { Self::new(value) }
}
