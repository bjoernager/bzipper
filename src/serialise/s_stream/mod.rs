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
