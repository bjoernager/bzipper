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

//! Error handling.

use std::error::Error as StdError;
use std::fmt::{Display, Formatter};

/// Mapping of [`std::result::Result`].
pub type Result<T> = std::result::Result<T, Error>;

/// Denotes an error.
///
/// These variants are used when a deserialisation fails.
/// Serialisations are assumed infallible.
#[derive(Debug)]
pub enum Error {
	ArrayLengthMismatch { len: usize, ok_len: usize },

	EndOfDStream { len: usize, ok_len: usize },

	InvalidBoolean { value: u8 },

	InvalidCodePoint { value: u32 },

	NullInteger,
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		use Error::*;

		match *self {
			ArrayLengthMismatch { len, ok_len } => {
				write!(f, "expected array of length ({ok_len}) but got ({len}) elements")
			},

			EndOfDStream { len, ok_len } => {
				write!(f, "({ok_len}) byte(s) were requested but only ({len}) byte(s) were left")
			},

			InvalidBoolean { value } => {
				write!(f, "expected boolean but got {value:#02X}")
			},

			InvalidCodePoint { value } => {
				write!(f, "code point U+{value:04X} is not valid")
			}

			NullInteger => {
				write!(f, "expected non-zero integer but got (0)")
			},
		}
	}
}

impl StdError for Error { }
