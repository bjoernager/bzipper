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

//! Error variants.
//!
//! This module defines the error types used by bZipper.
//! All of these types define the [`Error`](core::error::Error) trait.

use crate::use_mod;

use_mod!(pub decode_error);
use_mod!(pub encode_error);
use_mod!(pub size_error);
use_mod!(pub string_error);
use_mod!(pub utf16_error);
use_mod!(pub utf8_error);
