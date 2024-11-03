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

use crate::error::{SizeError, Utf8Error};

use core::error::Error;
use core::fmt::{self, Display, Formatter};

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

/// Decode error variants.
///
/// These errors may be returned from implementation of [`Decode`](crate::Decode).
#[derive(Debug)]
#[non_exhaustive]
pub enum DecodeError {
	/// Bytes were requested on an empty stream.
	///
	/// This variant is different from [`SmallBuffer`](Self::SmallBuffer) in that this is exclusively for use by the stream types, whilst `SmallBuffer` is for any other array-like type.
	BadString(Utf8Error),

	/// An unspecified error.
	///
	/// This is mainly useful by third-party implementors if none of the other predefined variants are appropriate.
	#[cfg(feature = "alloc")]
	#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
	CustomError(Box<dyn core::error::Error>),

	/// A boolean encountered a value outside `0` and `1`.
	InvalidBoolean(u8),

	/// An invalid code point was encountered.
	///
	/// This includes surrogate points in the inclusive range `U+D800` to `U+DFFF`, as well as all values larger than `U+10FFFF`.
	InvalidCodePoint(u32),

	/// An invalid enumeration descriminant was provided.
	InvalidDiscriminant(isize),

	/// The [`SystemTime`](std::time::SystemTime) type could not represent a UNIX timestamp.
	///
	/// This error should not occur on systems that represent timestamps with at least a signed 64-bits seconds counter.
	#[cfg(feature = "std")]
	#[cfg_attr(doc, doc(cfg(feature = "std")))]
	NarrowSystemTime {
		/// The unrepresentable timestamp.
		timestamp: i64,
	},

	/// A non-zero integer had the value `0`.
	NullInteger,

	/// A C-like string encountered a null value within bounds.
	#[cfg(feature = "alloc")]
	#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
	NullCString {
		/// The index of the null value.
		index: usize,
	},

	/// An array could not hold the requested amount of elements.
	SmallBuffer(SizeError),
}

impl Display for DecodeError {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		use DecodeError::*;

		match *self {
			BadString(ref source)
			=> write!(f, "bad string: {source}"),

			#[cfg(feature = "alloc")]
			CustomError(ref source)
			=> write!(f, "{source}"),

			InvalidBoolean(value)
			=> write!(f, "expected boolean but got `{value:#02X}`"),

			InvalidCodePoint(value)
			=> write!(f, "code point U+{value:04X} is not defined"),

			InvalidDiscriminant(value)
			=> write!(f, "discriminant `{value}` is not valid for the given enumeration"),

			#[cfg(feature = "std")]
			NarrowSystemTime { timestamp }
			=> write!(f, "could not represent `{timestamp}` as a system timestamp"),

			NullInteger
			=> write!(f, "expected non-zero integer but got `0`"),

			#[cfg(feature = "alloc")]
			NullCString { index }
			=> write!(f, "expected c string but found null value at '{index}`"),

			SmallBuffer(ref source)
			=> write!(f, "buffer too small: {source}"),
		}
	}
}

impl Error for DecodeError {
	#[inline]
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		use DecodeError::*;

		match *self {
			BadString(ref source) => Some(source),

			#[cfg(feature = "alloc")]
			CustomError(ref source) => Some(source.as_ref()),

			SmallBuffer(ref source) => Some(source),

			_ => None,
		}
	}
}
