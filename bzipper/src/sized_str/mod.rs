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

#[cfg(test)]
mod test;

use crate::{
	Decode,
	Encode,
	IStream,
	OStream,
	SizedEncode,
	SizedSlice,
};
use crate::error::{
	DecodeError,
	EncodeError,
	SizeError,
	StringError,
	Utf8Error,
};

use core::borrow::{Borrow, BorrowMut};
use core::cmp::Ordering;
use core::fmt::{self, Debug, Display, Formatter};
use core::hash::{Hash, Hasher};
use core::mem::{ManuallyDrop, MaybeUninit};
use core::ops::{Deref, DerefMut, Index, IndexMut};
use core::ptr::{addr_of, copy_nonoverlapping};
use core::slice;
use core::slice::SliceIndex;
use core::str;
use core::str::{Chars, CharIndices, FromStr};

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

#[cfg(feature = "alloc")]
use alloc::string::String;

#[cfg(feature = "std")]
use std::ffi::OsStr;

#[cfg(feature = "std")]
use std::net::ToSocketAddrs;

#[cfg(feature = "std")]
use std::path::Path;

/// Stack-allocated string with maximum length.
///
/// This is in contrast to [`String`] -- which has no size limit in practice -- and [`prim@str`], which is unsized.
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
/// use bzipper::SizedStr;
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
	/// Constructs an empty, fixed-size string.
	///
	/// Note that string is not required to completely fill out its  size-constraint.
	///
	/// The constructed string will have a null length.
	///
	/// For constructing a string with an already defined buffer, see [`from_raw_parts`](Self::from_raw_parts) and [`from_str`](Self::from_str).
	///
	/// # Errors
	///
	/// If the internal buffer cannot contain the entirety of `s`, then an error is returned.
	#[inline(always)]
	pub const fn new(s: &str) -> Result<Self, StringError> {
		if s.len() > N { return Err(StringError::SmallBuffer(SizeError { req: s.len(), len: N })) };

		let this = unsafe { Self::from_utf8_unchecked(s.as_bytes()) };
		Ok(this)
	}

	/// Constructs a fixed-size string from UTF-8 octets.
	///
	/// The passed slice is checked for its validity.
	/// For a similar function *without* these checks, see [`from_utf8_unchecked`](Self::from_utf8_unchecked).
	///
	/// # Errors
	///
	/// Each byte value must be a valid UTF-8 code point.
	#[inline]
	pub const fn from_utf8(data: &[u8]) -> Result<Self, StringError> {
		if data.len() > N { return Err(StringError::SmallBuffer(SizeError { req: data.len(), len: N })) };

		let s = match str::from_utf8(data) {
			Ok(s) => s,

			Err(e) => {
				let i = e.valid_up_to();

				return Err(StringError::BadUtf8(Utf8Error { value: data[i], index: i }));
			}
		};

		// SAFETY: `s` is guaranteed to only contain valid
		// octets.
		let this = unsafe { Self::from_utf8_unchecked(s.as_bytes()) };
		Ok(this)
	}

	/// Unsafely constructs a new, fixed-size string from UTF-8 octets.
	///
	/// # Safety
	///
	/// Each byte value must be a valid UTF-8 code point.
	/// The behaviour of a programme that passes invalid values to this function is undefined.
	#[inline]
	#[must_use]
	pub const unsafe fn from_utf8_unchecked(s: &[u8]) -> Self {
		// Should we assert the length?
		debug_assert!(s.len() <= N, "cannot construct string from utf-8 sequence that is longer");

		let mut buf = [0x00; N];
		copy_nonoverlapping(s.as_ptr(), buf.as_mut_ptr(), s.len());

		// SAFETY: `s` is guaranteed by the caller to only
		// contain valid octets. It has also been tested to
		// not exceed bounds.
		Self::from_raw_parts(buf, s.len())
	}

	/// Constructs a fixed-size string from raw parts.
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
	pub const unsafe fn from_raw_parts(buf: [u8; N], len: usize) -> Self {
		debug_assert!(len <= N, "cannot construct string that is longer than its capacity");

		let init_buf = ManuallyDrop::new(buf);
		let buf = unsafe { addr_of!(init_buf).cast::<[MaybeUninit<u8>; N]>().read() };

		Self(SizedSlice::from_raw_parts(buf, len))
	}

	/// Gets a pointer to the first octet.
	#[inline(always)]
	#[must_use]
	pub const fn as_ptr(&self) -> *const u8 {
		self.0.as_ptr()
	}

	// This function can only be marked as `const` when
	// `const_mut_refs` is implemented. See tracking
	// issue #57349 for more information.
	/// Gets a mutable pointer to the first octet.
	///
	#[inline(always)]
	#[must_use]
	pub const fn as_mut_ptr(&mut self) -> *mut u8 {
		self.0.as_mut_ptr()
	}

	/// Borrows the string as a byte slice.
	///
	/// The range of the returned slice only includes characters that are "used."
	#[inline(always)]
	#[must_use]
	pub const fn as_bytes(&self) -> &[u8] {
		// We need to use `from_raw_parts` to mark this
		// function `const`.

		let ptr = self.as_ptr();
		let len = self.len();

		unsafe { slice::from_raw_parts(ptr, len) }
	}

	/// Borrows the string as a string slice.
	///
	/// The range of the returned slice only includes characters that are "used."
	#[inline(always)]
	#[must_use]
	pub const fn as_str(&self) -> &str {
		unsafe { core::str::from_utf8_unchecked(self.as_bytes()) }
	}

	/// Mutably borrows the string as a string slice.
	///
	/// The range of the returned slice only includes characters that are "used."
	#[inline(always)]
	#[must_use]
	pub fn as_mut_str(&mut self) -> &mut str {
		unsafe {
			let ptr = self.as_mut_ptr();
			let len = self.len();

			let bytes = slice::from_raw_parts_mut(ptr, len);
			core::str::from_utf8_unchecked_mut(bytes)
		}
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

	/// Destructs the provided string into its raw parts.
	///
	/// The returned values are valid to pass on to [`from_raw_parts`](Self::from_raw_parts).
	///
	/// The returned byte array is guaranteed to be fully initialised.
	/// However, only octets up to an index of [`len`](Self::len) are also guaranteed to be valid UTF-8 codepoints.
	#[inline(always)]
	#[must_use]
	pub const fn into_raw_parts(self) -> ([u8; N], usize) {
		let Self(vec) = self;
		let (buf, len) = vec.into_raw_parts();

		let init_buf = ManuallyDrop::new(buf);
		let buf = unsafe { addr_of!(init_buf).cast::<[u8; N]>().read() };

		(buf, len)
	}

	/// Converts the fixed-size string into a boxed string slice.
	#[cfg(feature = "alloc")]
	#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
	#[inline(always)]
	#[must_use]
	pub fn into_boxed_str(self) -> Box<str> {
		let Self(vec) = self;

		unsafe { alloc::str::from_boxed_utf8_unchecked(vec.into_boxed_slice()) }
	}

	/// Converts the fixed-size string into a dynamic string.
	///
	/// The capacity of the resulting [`String`] object is equal to the value of `N`.
	#[cfg(feature = "alloc")]
	#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
	#[inline(always)]
	#[must_use]
	pub fn into_string(self) -> String {
		self.into_boxed_str().into_string()
	}
}

impl<const N: usize> AsMut<str> for SizedStr<N> {
	#[inline(always)]
	fn as_mut(&mut self) -> &mut str {
		self.as_mut_str()
	}
}

#[cfg(feature = "std")]
#[cfg_attr(doc, doc(cfg(feature = "std")))]
impl<const N: usize> AsRef<OsStr> for SizedStr<N> {
	#[inline(always)]
	fn as_ref(&self) -> &OsStr {
		self.as_str().as_ref()
	}
}

#[cfg(feature = "std")]
#[cfg_attr(doc, doc(cfg(feature = "std")))]
impl<const N: usize> AsRef<Path> for SizedStr<N> {
	#[inline(always)]
	fn as_ref(&self) -> &Path {
		self.as_str().as_ref()
	}
}

impl<const N: usize> AsRef<str> for SizedStr<N> {
	#[inline(always)]
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

impl<const N: usize> AsRef<[u8]> for SizedStr<N> {
	#[inline(always)]
	fn as_ref(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl<const N: usize> Borrow<str> for SizedStr<N> {
	#[inline(always)]
	fn borrow(&self) -> &str {
		self.as_str()
	}
}

impl<const N: usize> BorrowMut<str> for SizedStr<N> {
	#[inline(always)]
	fn borrow_mut(&mut self) -> &mut str {
		self.as_mut_str()
	}
}

impl<const N: usize> Debug for SizedStr<N> {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Debug::fmt(self.as_str(), f)
	}
}

impl<const N: usize> Deref for SizedStr<N> {
	type Target = str;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		self.as_str()
	}
}

impl<const N: usize> DerefMut for SizedStr<N> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.as_mut_str()
	}
}

impl<const N: usize> Decode for SizedStr<N> {
	#[inline]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let len = Decode::decode(stream)?;

		let data = stream.read(len);

		Self::from_utf8(data)
			.map_err(|e| match e {
				StringError::BadUtf8(e) => DecodeError::BadString(e),

				StringError::SmallBuffer(e) => DecodeError::SmallBuffer(e),

				_ => unreachable!(),
			})
	}
}

impl<const N: usize> Display for SizedStr<N> {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(self.as_str(), f)
	}
}

impl<const N: usize> Encode for SizedStr<N> {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		// Optimised encode. Don't just rely on `SizedSlice`.

		self.as_str().encode(stream)
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

impl<const N: usize> FromStr for SizedStr<N> {
	type Err = StringError;

	#[inline]
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.len() > N { return Err(StringError::SmallBuffer(SizeError { req: s.len(), len: N })) };

		let this = unsafe { Self::from_utf8_unchecked(s.as_bytes()) };
		Ok(this)
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

unsafe impl<const N: usize> SizedEncode for SizedStr<N> {
	const MAX_ENCODED_SIZE: usize =
		usize::MAX_ENCODED_SIZE
		+ SizedSlice::<u8, N>::MAX_ENCODED_SIZE;
}

#[cfg(feature = "std")]
#[cfg_attr(doc, doc(cfg(feature = "std")))]
impl<const N: usize> ToSocketAddrs for SizedStr<N> {
	type Iter = <str as ToSocketAddrs>::Iter;

	#[inline(always)]
	fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
		self.as_str().to_socket_addrs()
	}
}

impl<const N: usize> TryFrom<char> for SizedStr<N> {
	type Error = <Self as FromStr>::Err;

	#[inline(always)]
	fn try_from(value: char) -> Result<Self, Self::Error> {
		let mut buf = [0x00; 0x4];
		let s = value.encode_utf8(&mut buf);

		s.parse()
	}
}

impl<const N: usize> TryFrom<&str> for SizedStr<N> {
	type Error = <Self as FromStr>::Err;

	#[inline(always)]
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		Self::from_str(value)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<const N: usize> TryFrom<String> for SizedStr<N> {
	type Error = <Self as FromStr>::Err;

	#[inline(always)]
	fn try_from(value: String) -> Result<Self, Self::Error> {
		Self::from_str(&value)
	}
}

/// See [`into_boxed_str`](SizedStr::into_boxed_str).
#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<const N: usize> From<SizedStr<N>> for Box<str> {
	#[inline(always)]
	fn from(value: SizedStr<N>) -> Self {
		value.into_boxed_str()
	}
}

/// See [`into_string`](SizedStr::into_string).
#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<const N: usize> From<SizedStr<N>> for String {
	#[inline(always)]
	fn from(value: SizedStr<N>) -> Self {
		value.into_string()
	}
}
