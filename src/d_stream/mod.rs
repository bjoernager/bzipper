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

use crate::{Error, Result, SStream};

use std::fmt::{Debug, Formatter};

/// Byte stream for deserialisation.
///
/// This type borrows a byte slice (hence [`new`](DStream::new)), keeping track internally of the used bytes.
#[derive(Clone)]
pub struct DStream<'a> {
	data: &'a [u8],
	len:  usize,
}

impl<'a> DStream<'a> {
	/// Constructs a new byte stream.
	#[inline(always)]
	#[must_use]
	pub fn new<T: AsRef<[u8]> + ?Sized>(buf: &'a T) -> Self { Self {
		data: buf.as_ref(),
		len:  buf.as_ref().len(),
	} }

	/// Takes bytes from the stream.
	///
	/// # Errors
	///
	/// If the internal buffer doesn't hold at least the requested ammount of bytes, an [`EndOfDStream`](Error::EndOfDStream) error is returned.
	pub fn take(&mut self, len: usize) -> Result<&[u8]> {
		if self.len < len { return Err(Error::EndOfDStream { len: self.len, ok_len: len } ) }

		let start = self.data.len() - self.len;
		let stop  = start + len;

		self.len -= len;

		Ok(&self.data[start..stop])
	}

	/// Takes a slice of the remaining data.
	#[must_use]
	pub fn as_slice(&self) -> &[u8] {
		let stop  = self.data.len();
		let start = stop - self.len;

		&self.data[start..stop]
	}

	/// Converts the stream to a `SStream` object.
	///
	/// The returned object owns a copy of the remaining data.
	#[inline(always)]
	#[must_use]
	pub fn to_s_stream(&self) -> SStream {
		SStream(self.as_slice().to_vec())
	}
}

impl Debug for DStream<'_> {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		write!(f, "[")?;

		for v in self.as_slice() { write!(f, "{v:#02X},")? };

		write!(f, "]")?;

		Ok(())
	}
}

impl<'a> From<&'a [u8]> for DStream<'a> {
	fn from(value: &'a [u8]) -> Self { Self::new(value) }
}

impl<'a, const N: usize> From<&'a [u8; N]> for DStream<'a> {
	fn from(value: &'a [u8; N]) -> Self { Self::new(value) }
}
