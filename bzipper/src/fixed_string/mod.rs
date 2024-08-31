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
	Serialise,
	Sstream,
};

use core::borrow::{Borrow, BorrowMut};
use core::cmp::Ordering;
use core::fmt::{Debug, Display, Formatter};
use core::hash::{Hash, Hasher};
use core::ops::{Add, AddAssign, Deref, DerefMut, Index, IndexMut};
use core::slice::SliceIndex;
use core::str::{Chars, CharIndices, FromStr};

#[cfg(feature = "alloc")]
use alloc::string::String;

#[cfg(feature = "std")]
use std::ffi::OsStr;

#[cfg(feature = "std")]
use std::net::ToSocketAddrs;

#[cfg(feature = "std")]
use std::path::Path;

/// Heap-allocated string with maximum size.
///
/// This is in contrast to [String] -- which has no size limit in practice -- and [str], which is unsized.
///
/// The string itself is encoded in UTF-8 for interoperability wtih Rust's standard string facilities, as well as for memory concerns.
///
/// Keep in mind that the size limit specified by `N` denotes *bytes* and not *characters* -- i.e. a value of `8` may translate to between two and eight characters, depending on their codepoints.
///
/// # Examples
///
/// All instances of this type have the same size if the value of `N` is also the same.
/// Therefore, the following four strings have -- despite their different contents -- the same total size.
///
/// ```rust
/// use bzipper::FixedString;
/// use std::str::FromStr;
///
/// let str0 = FixedString::<0x40>::new(); // Empty string.
/// let str1 = FixedString::<0x40>::from_str("Hello there!").unwrap();
/// let str2 = FixedString::<0x40>::from_str("أنا من أوروپا").unwrap();
/// let str3 = FixedString::<0x40>::from_str("COGITO ERGO SUM").unwrap();
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
#[derive(Clone)]
pub struct FixedString<const N: usize> {
	buf: [u8; N],
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
	/// For constructing a string with an already defined buffer, see [`from_raw_parts`](Self::from_raw_parts) and [`from_str`](Self::from_str).
	#[inline(always)]
	#[must_use]
	pub const fn new() -> Self { Self { buf: [0x00; N], len: 0x0 } }

	/// Constructs a new, fixed-size string from raw parts.
	///
	/// The provided parts are not tested in any way.
	///
	/// # Safety
	///
	/// The value of `len` may not exceed that of `N`.
	/// Additionally, the octets in `buf` (from index zero up to the value of `len`) must be valid UTF-8 codepoints.
	///
	/// If any of these requirements are violated, behaviour is undefined.
	#[inline(always)]
	#[must_use]
	pub const unsafe fn from_raw_parts(buf: [u8; N], len: usize) -> Self { Self { buf, len } }

	/// Destructs the provided string into its raw parts.
	///
	/// The returned values are valid to pass on to [`from_raw_parts`](Self::from_raw_parts).
	///
	/// The returned byte array is guaranteed to be fully initialised.
	/// However, only octets up to an index of [`len`](Self::len) are also guaranteed to be valid UTF-8 codepoints.
	#[inline(always)]
	#[must_use]
	pub const fn into_raw_parts(self) -> ([u8; N], usize) { (self.buf, self.len) }

	/// Gets a pointer to the first octet.
	#[inline(always)]
	#[must_use]
	pub const fn as_ptr(&self) -> *const u8 { self.buf.as_ptr() }

	// This function can only be marked as `const` when
	// `const_mut_refs` is implemented. See tracking
	// issue #57349 for more information.
	/// Gets a mutable pointer to the first octet.
	///
	#[inline(always)]
	#[must_use]
	pub fn as_mut_ptr(&mut self) -> *mut u8 { self.buf.as_mut_ptr() }

	/// Borrows the string as a byte slice.
	///
	/// The range of the returned slice only includes characters that are "used."
	#[inline(always)]
	#[must_use]
	pub const fn as_bytes(&self) -> &[u8] {
		// We need to use `from_raw_parts` to mark this
		// function `const`.

		unsafe { core::slice::from_raw_parts(self.as_ptr(), self.len()) }
	}

	/// Borrows the string as a string slice.
	///
	/// The range of the returned slice only includes characters that are "used."
	#[inline(always)]
	#[must_use]
	pub const fn as_str(&self) -> &str { unsafe { core::str::from_utf8_unchecked(self.as_bytes()) } }

	/// Mutably borrows the string as a string slice.
	///
	/// The range of the returned slice only includes characters that are "used."
	#[inline(always)]
	#[must_use]
	pub fn as_mut_str(&mut self) -> &mut str {
		let range = 0x0..self.len();

		unsafe { core::str::from_utf8_unchecked_mut(&mut self.buf[range]) }
	}

	/// Returns the length of the string.
	///
	/// This does not necessarily equate to the value of `N`, as the internal buffer may be used but partially.
	#[inline(always)]
	#[must_use]
	pub const fn len(&self) -> usize { self.len }

	/// Checks if the string is empty, i.e. no characters are contained.
	#[inline(always)]
	#[must_use]
	pub const fn is_empty(&self) -> bool { self.len() == 0x0 }

	/// Checks if the string is full, i.e. it cannot hold any more characters.
	#[inline(always)]
	#[must_use]
	pub const fn is_full(&self) -> bool { self.len() == N }

	/// Returns the total capacity of the string.
	///
	/// This is defined as being exactly the value of `N`.
	#[inline(always)]
	#[must_use]
	pub const fn capacity(&self) -> usize { N }

	/// Gets a substring of the string.
	#[inline(always)]
	#[must_use]
	pub fn get<I: SliceIndex<str>>(&self, index: I) -> Option<&I::Output> { self.as_str().get(index) }

	/// Gets a mutable substring of the string.
	#[inline(always)]
	#[must_use]
	pub fn get_mut<I: SliceIndex<str>>(&mut self, index: I) -> Option<&mut I::Output> { self.as_mut_str().get_mut(index) }

	/// Pushes a character into the string.
	///
	/// The internal length is updated accordingly.
	///
	/// # Panics
	///
	/// If the string cannot hold the provided character *after* encoding, this method will panic.
	#[inline(always)]
	pub fn push(&mut self, c: char) {
		let mut buf = [0x00; 0x4];
		let s = c.encode_utf8(&mut buf);

		self.push_str(s);
	}

	/// Pushes a string slice into the string.
	///
	/// The internal length is updated accordingly.
	///
	/// # Panics
	///
	/// If the string cannot hold the provided slice, this method will panic.
	#[inline(always)]
	pub fn push_str(&mut self, s: &str) {
		let rem = self.buf.len() - self.len;
		let req = s.len();

		assert!(rem >= req, "cannot push string beyond fixed length");

		let start = self.len;
		let stop  = start + req;

		let buf = &mut self.buf[start..stop];
		buf.copy_from_slice(s.as_bytes());
	}

	/// Pops a character from the string.
	///
	/// The internal length is updated accordingly.
	///
	/// If no characters are left (i.e. the string is empty), an instance of [`None`] is returned.
	///
	/// **Note that this method is currently unimplemented.**
	#[deprecated = "temporarily unimplemented"]
	#[inline(always)]
	pub fn pop(&mut self) -> Option<char> { todo!() }

	/// Returns an iterator of the string's characters.
	#[inline(always)]
	pub fn chars(&self) -> Chars { self.as_str().chars() }

	/// Returns an iterator of the string's characters along with their positions.
	#[inline(always)]
	pub fn char_indices(&self) -> CharIndices { self.as_str().char_indices() }
}

impl<const N: usize> Add<&str> for FixedString<N> {
	type Output = Self;

	fn add(mut self, rhs: &str) -> Self::Output {
		self.push_str(rhs);
		self
	}
}

impl<const N: usize> AddAssign<&str> for FixedString<N> {
	fn add_assign(&mut self, rhs: &str) { self.push_str(rhs) }
}

impl<const N: usize> AsMut<str> for FixedString<N> {
	#[inline(always)]
	fn as_mut(&mut self) -> &mut str { self.as_mut_str() }
}

#[cfg(feature = "std")]
#[cfg_attr(doc, doc(cfg(feature = "std")))]
impl<const N: usize> AsRef<OsStr> for FixedString<N> {
	#[inline(always)]
	fn as_ref(&self) -> &OsStr { self.as_str().as_ref() }
}

#[cfg(feature = "std")]
#[cfg_attr(doc, doc(cfg(feature = "std")))]
impl<const N: usize> AsRef<Path> for FixedString<N> {
	#[inline(always)]
	fn as_ref(&self) -> &Path { self.as_str().as_ref() }
}

impl<const N: usize> AsRef<str> for FixedString<N> {
	#[inline(always)]
	fn as_ref(&self) -> &str { self.as_str() }
}

impl<const N: usize> AsRef<[u8]> for FixedString<N> {
	#[inline(always)]
	fn as_ref(&self) -> &[u8] { self.as_bytes() }
}

impl<const N: usize> Borrow<str> for FixedString<N> {
	#[inline(always)]
	fn borrow(&self) -> &str { self.as_str() }
}

impl<const N: usize> BorrowMut<str> for FixedString<N> {
	#[inline(always)]
	fn borrow_mut(&mut self) -> &mut str { self.as_mut_str() }
}

impl<const N: usize> Debug for FixedString<N> {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> core::fmt::Result { Debug::fmt(self.as_str(), f) }
}

impl<const N: usize> Default for FixedString<N> {
	#[inline(always)]
	fn default() -> Self { Self { buf: [Default::default(); N], len: 0x0 } }
}

impl<const N: usize> Deref for FixedString<N> {
	type Target = str;

	#[inline(always)]
	fn deref(&self) -> &Self::Target { self.as_str() }
}

impl<const N: usize> DerefMut for FixedString<N> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target { self.as_mut_str() }
}

impl<const N: usize> Deserialise for FixedString<N> {
	#[inline]
	fn deserialise(stream: &Dstream) -> Result<Self, Error> {
		let len = Deserialise::deserialise(stream)?;
		if len > N { return Err(Error::ArrayTooShort { req: len, len: N }) };

		let bytes = stream.read(len)?;

		let s = core::str::from_utf8(bytes)
			.map_err(|e| Error::BadString { source: e })?;

		Self::from_str(s)
	}
}

impl<const N: usize> Display for FixedString<N> {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> core::fmt::Result { Display::fmt(self.as_str(), f) }
}

impl<const N: usize> Eq for FixedString<N> { }

impl<const N: usize> FromStr for FixedString<N> {
	type Err = Error;

	#[inline]
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let len = s.len();
		if len > N { return Err(Error::ArrayTooShort { req: len, len: N }) };

		let mut buf = [0x00; N];
		unsafe { core::ptr::copy_nonoverlapping(s.as_ptr(), buf.as_mut_ptr(), len) };

		// The remaining bytes are already initialised to
		// null.

		Ok(Self { buf, len })
	}
}

impl<const N: usize> Hash for FixedString<N> {
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H) { self.as_str().hash(state) }
}

impl<I: SliceIndex<str>, const N: usize> Index<I> for FixedString<N> {
	type Output	= I::Output;

	#[inline(always)]
	fn index(&self, index: I) -> &Self::Output { self.get(index).unwrap() }
}

impl<I: SliceIndex<str>, const N: usize> IndexMut<I> for FixedString<N> {
	#[inline(always)]
	fn index_mut(&mut self, index: I) -> &mut Self::Output { self.get_mut(index).unwrap() }
}

impl<const N: usize> Ord for FixedString<N> {
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering { self.as_str().cmp(other.as_str()) }
}

impl<const N: usize, const M: usize> PartialEq<FixedString<M>> for FixedString<N> {
	#[inline(always)]
	fn eq(&self, other: &FixedString<M>) -> bool { self.as_str() == other.as_str() }
}

impl<const N: usize> PartialEq<&str> for FixedString<N> {
	#[inline(always)]
	fn eq(&self, other: &&str) -> bool { self.as_str() == *other }
}

impl<const N: usize, const M: usize> PartialOrd<FixedString<M>> for FixedString<N> {
	#[inline(always)]
	fn partial_cmp(&self, other: &FixedString<M>) -> Option<Ordering> { self.as_str().partial_cmp(other.as_str()) }
}

impl<const N: usize> PartialOrd<&str> for FixedString<N> {
	#[inline(always)]
	fn partial_cmp(&self, other: &&str) -> Option<Ordering> { self.as_str().partial_cmp(*other) }
}

impl<const N: usize> Serialise for FixedString<N> {
	const MAX_SERIALISED_SIZE: usize = N + usize::MAX_SERIALISED_SIZE;

	fn serialise(&self, stream: &mut Sstream) -> Result<(), Error> {
		self.len().serialise(stream)?;
		stream.write(self.as_bytes())?;

		Ok(())
	}
}

#[cfg(feature = "std")]
#[cfg_attr(doc, doc(cfg(feature = "std")))]
impl<const N: usize> ToSocketAddrs for FixedString<N> {
	type Iter = <str as ToSocketAddrs>::Iter;

	#[inline(always)]
	fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> { self.as_str().to_socket_addrs() }
}

impl<const N: usize> TryFrom<char> for FixedString<N> {
	type Error = <Self as FromStr>::Err;

	#[inline(always)]
	fn try_from(value: char) -> Result<Self, Self::Error> {
		let mut buf = [0x00; 0x4];
		let s = value.encode_utf8(&mut buf);

		s.parse()
	}
}

impl<const N: usize> TryFrom<&str> for FixedString<N> {
	type Error = <Self as FromStr>::Err;

	#[inline(always)]
	fn try_from(value: &str) -> Result<Self, Self::Error> { Self::from_str(value) }
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<const N: usize> TryFrom<String> for FixedString<N> {
	type Error = <Self as FromStr>::Err;

	#[inline(always)]
	fn try_from(value: String) -> Result<Self, Self::Error> { Self::from_str(&value) }
}

/// Converts the fixed-size string into a dynamic string.
///
/// The capacity of the resulting [`String`] object is equal to the value of `N`.
#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<const N: usize> From<FixedString<N>> for String {
	#[inline(always)]
	fn from(value: FixedString<N>) -> Self {
		let mut s = Self::with_capacity(N);
		s.push_str(value.as_str());

		s
	}
}
