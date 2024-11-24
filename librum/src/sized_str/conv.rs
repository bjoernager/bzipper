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

use crate::{SizedSlice, SizedStr};
use crate::error::{SizeError, StringError, Utf8Error};

use core::borrow::{Borrow, BorrowMut};
use core::mem::{ManuallyDrop, MaybeUninit};
use core::ops::{Deref, DerefMut};
use core::ptr::copy_nonoverlapping;
use core::slice;
use core::str::{self, FromStr};

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

impl<const N: usize> SizedStr<N> {
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
		if data.len() > N { return Err(StringError::SmallBuffer(SizeError { cap: N, len: data.len() })) };

		let s = match str::from_utf8(data) {
			Ok(s) => s,

			Err(e) => {
				let i = e.valid_up_to();
				let c = data[i];

				return Err(StringError::BadUtf8(Utf8Error { value: c, index: i }));
			}
		};

		// SAFETY: `s` has been tested to only contain
		// valid octets.
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

		let buf = unsafe { buf.as_ptr().cast::<[MaybeUninit<u8>; N]>().read() };

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
		// SAFETY: We guarantee that all octets are valid
		// UTF-8 code points.
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

			let data = slice::from_raw_parts_mut(ptr, len);
			core::str::from_utf8_unchecked_mut(data)
		}
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
		let buf = unsafe { (&raw const init_buf).cast::<[u8; N]>().read() };

		(buf, len)
	}

	/// Deconstructs the string into a fixed-size byte slice.
	#[inline(always)]
	#[must_use]
	pub const fn into_bytes(self) -> SizedSlice<u8, N> {
		let Self(v) = self;
		v
	}

	/// Converts the fixed-size string into a boxed string slice.
	#[cfg(feature = "alloc")]
	#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
	#[inline(always)]
	#[must_use]
	pub fn into_boxed_str(self) -> Box<str> {
		let Self(v) = self;
		unsafe { alloc::str::from_boxed_utf8_unchecked(v.into_boxed_slice()) }
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

impl<const N: usize> FromStr for SizedStr<N> {
	type Err = StringError;

	#[inline]
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.len() > N { return Err(StringError::SmallBuffer(SizeError { cap: N, len: s.len() })) };

		let this = unsafe { Self::from_utf8_unchecked(s.as_bytes()) };
		Ok(this)
	}
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
