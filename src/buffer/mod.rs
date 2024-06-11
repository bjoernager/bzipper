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
// even the impl<T: Serialise>ied warranty of MERCHANTABILITY<T> or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Less-
// er General Public License along with bzipper. If
// not, see <https://www.gnu.org/licenses/>.

use crate::{Deserialise, Dstream, Serialise, Sstream};

use alloc::vec;
use alloc::boxed::Box;
use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;

/// Container type for (de)serialisations.
///
/// The purpose of this type is to easily hold a buffer than can fit any serialisation of a given type (hence the generic).
///
/// Do note that the internal buffer does not guarantee the state of any padding bytes that occur as a result of different serialisation sizes.
/// Deserialisations, however, are not affected by these.
#[derive(Clone, Eq, PartialEq)]
pub struct Buffer<T> {
	data: Box<[u8]>,
	len:  usize,

	_phantom: PhantomData<T>,
}

impl<T> Buffer<T> {
	/// Sets the internal length of the buffer without checks.
	///
	/// For a safe alternative, see [`set_len`](Self::set_len).
	///
	/// # Safety
	///
	/// The new length must **not** exceed [`T::SERIALISE_LIMIT`](Serialise::SERIALISE_LIMIT).
	#[inline(always)]
	pub unsafe fn set_len_unchecked(&mut self, len: usize) {
		self.len = len;
	}

	/// Returns a slice of the internal buffer.
	///
	/// This only includes bytes written by the last serialisation, or as set by [`set_len`](Self::set_len).
	#[inline(always)]
	#[must_use]
	pub fn as_slice(&self) -> &[u8] { &self.data[0x0..self.len] }

	/// Returns a mutable slice of the entire internal buffer.
	///
	/// This is in contrast to [`as_slice`](Self::as_slice), which only yields the last serialisation.
	///
	/// The length of bytes written to this slice should be set using [`set_len`](Self::set_len).
	#[inline(always)]
	#[must_use]
	pub fn as_mut_slice(&mut self) -> &mut [u8] { self.data.as_mut() }
}

impl<T: Serialise> Buffer<T> {
	/// Constructs a new, empty buffer.
	///
	/// The internal buffer is allocated on the heap instantly with the size [`T::SERIALISE_LIMIT`](Serialise::SERIALISE_LIMIT).
	#[inline(always)]
	#[must_use]
	pub fn new() -> Self { Self {
		data: vec![Default::default(); T::SERIALISE_LIMIT].into(),
		len:  0x0,

		_phantom: PhantomData
	} }

	/// Sets the length of the current serialisation.
	///
	/// This is specifically meant for cases where the buffer is set externally, as is the case for networking:
	///
	/// ```
	/// use bzipper::{Buffer, FixedString};
	/// use std::net::{SocketAddr, UdpSocket};
	/// use std::str::FromStr;
	///
	/// let destination = SocketAddr::from_str("127.0.0.1:37279")?;
	///
	/// let sender   = UdpSocket::bind("0.0.0.0:0")?;
	/// let reciever = UdpSocket::bind(destination)?;
	///
	/// // Create a buffer for holding a fixed string.
	/// let mut buffer = Buffer::<FixedString<0x10>>::new();
	///
	/// // Serialise and write the string:
	/// buffer.write(&FixedString::new("Hello there!")?);
	/// sender.send_to(buffer.as_ref(), destination);
	///
	/// // Recieve and deserialise the string:
	/// let (count, _source) = reciever.recv_from(buffer.as_mut_slice())?;
	/// buffer.set_len(count);
	///
	/// assert_eq!(buffer.read()?, "Hello there!");
	///
	/// # Ok::<(), Box<dyn std::error::Error>>(())
	/// ```
	///
	/// # Panics
	///
	/// Panics if `len` is larger than [`T::SERIALISE_LIMIT`](Serialise::SERIALISE_LIMIT).
	/// See [`set_len_unchecked`](Self::set_len_unchecked).
	#[inline]
	pub fn set_len(&mut self, len: usize) {
		assert!(len <= T::SERIALISE_LIMIT);
		self.len = len;
	}
}

impl<T: Serialise> Buffer<T> {
	/// Serialises into the buffer.
	///
	/// The result of [`serialise`](Serialise::serialise) is used as the length.
	///
	/// # Panics
	///
	/// Panics if the amount of written bytes exceeds [`SERIALISE_LIMIT`](Serialise::SERIALISE_LIMIT).
	/// This *should*, in theory, not occur, as the internal buffer can only fit up to this limit, making all writes past this limit fail.
	#[allow(clippy::panic_in_result_fn)]
	pub fn write(&mut self, value: &T) -> Result<(), <T as Serialise>::Error> {
		let mut stream = Sstream::new(&mut self.data);

		let count = value.serialise(&mut stream)?;
		assert!(count <= T::SERIALISE_LIMIT);

		self.len = count;
		Ok(())
	}
}

impl<T: Deserialise> Buffer<T> {
	/// Deserialises the contained buffer.
	///
	/// Only bytes from the last serialisation, or as set by [`set_len`](Self::set_len), are used.
	pub fn read(&self) -> Result<T, <T as Deserialise>::Error> {
		let mut stream = Dstream::new(self.as_ref());
		T::deserialise(&mut stream)
	}
}

impl<T> AsRef<[u8]> for Buffer<T> {
	#[inline(always)]
	fn as_ref(&self) -> &[u8] { self.as_slice() }
}

impl<T> Debug for Buffer<T> {
	fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
		self.data.fmt(f)
	}
}

impl<T: Serialise> Default for Buffer<T> {
	#[inline(always)]
	fn default() -> Self { Self::new() }
}
