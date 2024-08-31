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

use crate::{Deserialise, Dstream, Result, Serialise, Sstream};

use alloc::vec;
use alloc::boxed::Box;
use core::borrow::Borrow;
use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

/// Typed (de)serialisation buffer.
///
/// This structure is intended as a lightweight wrapper around byte buffers for specific (de)serialisations of specific types.
///
/// The methods [`write`](Self::write) and [`read`](Self::read) can be used to handle the internal buffer.
/// Other methods exist for accessing the internal buffer directly.
///
/// # Examples
///
/// Create a buffer for holding a `Request` enumeration:
///
/// ```rust
/// use bzipper::{Buffer, FixedString, Serialise};
///
/// #[derive(Serialise)]
/// enum Request {
///     Join { username: FixedString<0x40> },
///
///     Quit { username: FixedString<0x40> },
///
///     SendMessage { message: FixedString<0x80> },
/// }
///
/// use Request::*;
///
/// let join_request = Join { username: FixedString::try_from("epsiloneridani").unwrap() };
///
/// let mut buf = Buffer::new();
/// buf.write(join_request);
///
/// // Do something with the buffer...
/// ```
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
#[derive(Clone, Eq, PartialEq)]
pub struct Buffer<T> {
	buf: Box<[u8]>,
	len: usize,

	_phanton: PhantomData<T>
}

impl<T> Buffer<T> {
	/// Allocates a new buffer suitable for serialisation.
	///
	/// The given capacity should be large enough to hold any expected serialisation of `T`.
	/// Therefore, if `T` implements [`Serialise`], it is recommended to use [`new`](Self::new) instead, which is equivalent to passing [`MAX_SERIALISED_SIZE`](Serialise::MAX_SERIALISED_SIZE) to this function:
	#[inline]
	#[must_use]
	pub fn with_capacity(len: usize) -> Self {
		Self {
			buf: vec![0x00; len].into(),
			len: 0x0,

			_phanton: PhantomData,
		}
	}

	/// Sets the length of the used buffer.
	///
	/// The provided size is checked before being written.
	/// For the same operation *without* checks, see [`set_len_unchecked`](Self::set_len_unchecked).
	///
	/// # Panics
	///
	/// The provided size must not be greater than the buffer's capacity.
	/// If this is the case, however, this method will panic.
	#[inline(always)]
	pub fn set_len(&mut self, len: usize) {
		assert!(len <= self.capacity(), "cannot extend buffer beyond capacity");

		self.len = len;
	}

	/// Sets the length of the used buffer without checks.
	///
	/// The validity of the provided size is **not** checked before being written.
	/// For the same operation *with* checks, see [`set_len`](Self::set_len).
	///
	/// # Safety
	///
	/// If the value of `len` is greater than the buffer's capacity, behaviour is undefined.
	#[inline(always)]
	pub unsafe fn set_len_unchecked(&mut self, len: usize) { self.len = len }

	/// Retrieves a pointer to the first byte of the internal buffer.
	#[inline(always)]
	#[must_use]
	pub const fn as_ptr(&self) -> *const u8 { self.buf.as_ptr() }

	/// Retrieves a mutable pointer to the first byte of the internal buffer.
	#[inline(always)]
	#[must_use]
	pub fn as_mut_ptr(&mut self) -> *mut u8 { self.buf.as_mut_ptr() }

	/// Gets a slice of the internal buffer.
	///
	/// The returned slice will only include the used part of the buffer (as specified by [`len`](Self::len)).
	#[inline(always)]
	#[must_use]
	pub const fn as_slice(&self) -> &[u8] { unsafe { core::slice::from_raw_parts(self.as_ptr(), self.len()) } }

	/// Gets a mutable slice of the internal buffer.
	///
	/// In contrast to [`as_slice`](Self::as_slice), this method returns a slice of the **entire** internal buffer.
	///
	/// If the returned reference is written through, the new buffer length -- if different -- should be set using [`set_len`](Self::set_len).
	#[inline(always)]
	#[must_use]
	pub fn as_mut_slice(&mut self) -> &mut [u8] { &mut self.buf }

	/// Gets the length of the buffer.
	#[allow(clippy::len_without_is_empty)]
	#[inline(always)]
	#[must_use]
	pub const fn len(&self) -> usize { self.len }

	/// Gets the capacity of the buffer.
	///
	/// If the buffer was constructed using [`new`](Self::new), this value is exactly the same as [`MAX_SERIALISED_SIZE`](Serialise::MAX_SERIALISED_SIZE).
	#[inline(always)]
	#[must_use]
	pub const fn capacity(&self) -> usize { self.buf.len() }
}

impl<T: Serialise> Buffer<T> {
	/// Allocates a new buffer suitable for serialisation.
	///
	/// The capacity of the internal buffer is set so that any serialisation of `T` may be stored.
	///
	/// This is equivalent to calling [`with_capacity`](Self::with_capacity) with [`MAX_SERIALISED_SIZE`](Serialise::MAX_SERIALISED_SIZE).
	#[inline(always)]
	#[must_use]
	pub fn new() -> Self { Self::with_capacity(T::MAX_SERIALISED_SIZE) }

	/// Serialises into the contained buffer.
	///
	/// # Errors
	///
	/// Any error that occurs during serialisation is passed on and returned from this method.
	///
	/// # Panics
	///
	/// If the amount of bytes read by [`serialise`](Serialise::serialise) is greater than that specified by [`MAX_SERIALISED_SIZE`](Serialise::MAX_SERIALISED_SIZE), this method panics.
	///
	/// In reality, however, this error can only be detected if the buffer's capacity is set to a value greater than `MAX_SERIALISED_SIZE` to begin with (e.g. using [`with_capacity`](Self::with_capacity)).
	#[inline(always)]
	pub fn write<U: Borrow<T>>(&mut self, value: U) -> Result<()> {
		let mut stream = Sstream::new(&mut self.buf);
		value.borrow().serialise(&mut stream)?;

		assert!(stream.len() <= T::MAX_SERIALISED_SIZE);
		self.len = stream.len();

		Ok(())
	}
}

impl<T: Deserialise> Buffer<T> {
	/// Deserialises from the contained buffer.
	///
	/// # Errors
	///
	/// Any error that occurs during deserialisation is passed on and returned from this method.
	#[inline(always)]
	pub fn read(&self) -> Result<T> {
		// We should only pass the used part of the buffer
		// to `deserialise`.

		let stream = Dstream::new(&self.buf[0x0..self.len()]);
		let value = Deserialise::deserialise(&stream)?;

		Ok(value)
	}
}

impl<T> AsMut<[u8]> for Buffer<T> {
	#[inline(always)]
	fn as_mut(&mut self) -> &mut [u8] { self.as_mut_slice() }
}

impl<T> AsRef<[u8]> for Buffer<T> {
	#[inline(always)]
	fn as_ref(&self) -> &[u8] { self.as_slice() }
}

impl<T> Debug for Buffer<T> {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> core::fmt::Result { write!(f, "{:?}", self.as_slice()) }
}

impl<T: Serialise> Default for Buffer<T> {
	#[inline(always)]
	fn default() -> Self { Self::new() }
}

impl<T> Deref for Buffer<T> {
	type Target = [u8];

	#[inline(always)]
	fn deref(&self) -> &Self::Target { self.as_slice() }
}

impl<T> DerefMut for Buffer<T> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target { self.as_mut_slice() }
}

impl<T> PartialEq<&[u8]> for Buffer<T> {
	#[inline(always)]
	fn eq(&self, other: &&[u8]) -> bool { self.as_slice() == *other }
}

impl<T, const N: usize> PartialEq<[u8; N]> for Buffer<T> {
	#[inline(always)]
	fn eq(&self, other: &[u8; N]) -> bool { self.as_slice() == other.as_slice() }
}
