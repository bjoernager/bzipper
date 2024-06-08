// Copyright 2022-2024 Gabriel Bjørnager Jensen.

use crate::serialise::Serialise;

use std::fmt::{Debug, Formatter};
use std::mem::size_of;

#[derive(Clone, Eq, PartialEq)]
pub struct SStream(Vec<u8>);

impl SStream {
	#[must_use]
	pub const fn new() -> Self { Self(Vec::new()) }

	pub fn append(&mut self, extra: &[u8]) {
		self.0.extend(extra);
	}
}

impl AsRef<[u8]> for SStream {
	#[inline(always)]
	fn as_ref(&self) -> &[u8] { self.0.as_ref() }
}

impl Debug for SStream {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		write!(f, "[")?;

		for v in &self.0 { write!(f, "{v:#02X},")? };

		write!(f, "]")?;

		Ok(())
	}
}

impl Default for SStream {
	#[inline(always)]
	fn default() -> Self { Self::new() }
}

impl<T: Serialise> From<&T> for SStream {
	fn from(value: &T) -> Self {
		let mut stream = Self(Vec::with_capacity(size_of::<T>()));
		value.serialise(&mut stream);

		stream
	}
}

impl From<SStream> for Box<[u8]> {
	#[inline(always)]
	fn from(value: SStream) -> Self { value.0.into_boxed_slice() }
}
