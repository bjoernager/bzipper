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

use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::str::Utf8Error;

/// Mapping of [`std::result::Result`].
pub type Result<T> = std::result::Result<T, Error>;

/// Denotes an error.
///
/// These variants are used when deserialisation fails.
/// Serialisations are assumed infallible.
#[derive(Debug)]
pub enum Error {
	/// An array could not hold the requested ammount of elements.
	ArrayTooShort { req: usize, len: usize },

	/// A string encountered an invalid UTF-8 sequence.
	BadString { source: Utf8Error },

	/// Bytes were requested on an empty stream.
	EndOfDStream { req: usize, rem: usize },

	/// A boolean encountered a value outside (0) and (1).
	InvalidBoolean { value: u8 },

	/// An invalid code point was encountered.
	///
	/// This includes surrogate points in the inclusive range `U+D800` to `U+DFFF`, as well as values larger than `U+10FFFF`.
	InvalidCodePoint { value: u32 },

	/// A non-zero integer encountered the value (0).
	NullInteger,
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		use Error::*;

		match *self {
			ArrayTooShort { req, len } => {
				write!(f, "array of ({len}) element(s) cannot hold ({req})")
			},

			BadString { ref source } =>{
				write!(f, "unable to parse utf8: \"{source}\"")
			},

			EndOfDStream { req, rem } => {
				write!(f, "({req}) byte(s) were requested but only ({rem}) byte(s) were left")
			},

			InvalidBoolean { value } => {
				write!(f, "expected boolean but got {value:#02X}")
			},

			InvalidCodePoint { value } => {
				write!(f, "code point U+{value:04X} is not valid")
			},

			NullInteger => {
				write!(f, "expected non-zero integer but got (0)")
			},
		}
	}
}

impl StdError for Error {
	fn source(&self) -> Option<&(dyn StdError + 'static)> {
		use Error::*;

		match *self {
			BadString { ref source } => Some(source),

			_ => None,
		}
	}
}
