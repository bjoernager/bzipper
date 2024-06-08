// Copyright 2022-2024 Gabriel Bjørnager Jensen.

use crate::error::{Error, Result};

use std::fmt::{Debug, Formatter};

/// A byte stream for deserialisation.
///
/// This type borrows a byte slice (hence [`new`](DStream::new)), keeping track internally of the used bytes.
#[derive(Clone)]
pub struct DStream<'a> {
	data: &'a [u8],
	len:  usize,
}

impl<'a> DStream<'a> {
	/// Constructs a new byte stream.
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
}

impl Debug for DStream<'_> {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		let stop  = self.data.len();
		let start = self.data.len() - self.len;

		write!(f, "[")?;

		for v in &self.data[start..stop] { write!(f, "{v:#02X},")? };

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
