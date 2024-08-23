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

use crate::{Deserialise, Result, Serialise};

use alloc::vec;
use alloc::boxed::Box;
use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

// We cannot use arrays for the `Buffer` type as
// that would require `generic_const_exprs`.

/// Typed (de)serialisation buffer.
///
/// This structure is intended as a lightweight wrapper around byte buffers for specific (de)serialisations of specific types.
///
/// The methods [`write`](Self::write) and [`read`](Self::read) can be used to <interpreting> the internal buffer.
/// Other methods exist for accessing the internal buffer directly.
///
/// # Examples
///
/// Create a buffer for holding a `Request` enumeration:
///
/// ```
/// use bzipper::{Buffer, FixedString, Serialise};
///
/// #[derive(Serialise)]
/// enum Request {
///     Join { username: FixedString<0x10> },
///
///     Quit { username: FixedString<0x10> },
///
///     SendMessage { message: FixedString<0x20> },
/// }
///
/// use Request::*;
///
/// let join_request = Join { username: FixedString::try_from("epsiloneridani").unwrap() };
///
/// let mut buf = Buffer::<Request>::new();
/// buf.write(&join_request);
///
/// // Do something with the buffer...
/// ```
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
#[derive(Clone, Eq, PartialEq)]
pub struct Buffer<T: Serialise> {
	buf: Box<[u8]>,

	_phanton: PhantomData<T>
}

impl<T: Serialise> Buffer<T> {
	/// Allocates a new buffer suitable for (de)serialisation.
	#[must_use]
	pub fn new() -> Self { Self { buf: vec![0x00; T::SERIALISED_SIZE].into(), _phanton: PhantomData } }

	/// Serialises into the contained buffer.
	#[inline(always)]
	pub fn write(&mut self, value: &T) -> Result<()> { value.serialise(&mut self.buf) }

	/// Retrieves a pointer to the first byte.
	#[inline(always)]
	#[must_use]
	pub const fn as_ptr(&self) -> *const u8 { self.buf.as_ptr() }

	/// Retrieves a mutable pointer to the first byte.
	#[inline(always)]
	#[must_use]
	pub fn as_mut_ptr(&mut self) -> *mut u8 { self.buf.as_mut_ptr() }

	/// Gets a slice of the internal buffer.
	#[inline(always)]
	#[must_use]
	pub const fn as_slice(&self) -> &[u8] { unsafe { core::slice::from_raw_parts(self.as_ptr(), self.len()) } }

	/// Gets a mutable slice of the internal buffer.
	#[inline(always)]
	#[must_use]
	pub fn as_mut_slice(&mut self) -> &mut [u8] { &mut self.buf }

	/// Gets the length of the buffer.
	///
	/// This is defined as (and therefore always equal to) the value of [SERIALISED_SIZE](Serialise::SERIALISED_SIZE) as specified by `T`.
	#[allow(clippy::len_without_is_empty)]
	#[inline(always)]
	#[must_use]
	pub const fn len(&self) -> usize { T::SERIALISED_SIZE }
}

impl<T: Deserialise> Buffer<T> {
	/// Deserialises from the contained buffer.
	#[inline(always)]
	pub fn read(&self) -> Result<T> { T::deserialise(&self.buf) }
}

impl<T: Serialise> AsMut<[u8]> for Buffer<T> {
	#[inline(always)]
	fn as_mut(&mut self) -> &mut [u8] { self.as_mut_slice() }
}

impl<T: Serialise> AsRef<[u8]> for Buffer<T> {
	#[inline(always)]
	fn as_ref(&self) -> &[u8] { self.as_slice() }
}

impl<T: Serialise> Debug for Buffer<T> {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> core::fmt::Result { write!(f, "{:?}", self.as_slice()) }
}

impl<T: Serialise> Default for Buffer<T> {
	#[inline(always)]
	fn default() -> Self { Self::new() }
}

impl<T: Serialise> Deref for Buffer<T> {
	type Target = [u8];

	#[inline(always)]
	fn deref(&self) -> &Self::Target { self.as_slice() }
}

impl<T: Serialise> DerefMut for Buffer<T> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target { self.as_mut_slice() }
}

impl<T: Serialise> PartialEq<&[u8]> for Buffer<T> {
	#[inline(always)]
	fn eq(&self, other: &&[u8]) -> bool { self.as_slice() == *other }
}
