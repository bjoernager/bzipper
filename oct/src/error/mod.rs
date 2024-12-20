// Copyright 2024 Gabriel Bjørnager Jensen.
//
// This file is part of oct.
//
// oct is free software: you can redistribute it
// and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// oct is distributed in the hope that it will
// be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Less-
// er General Public License along with oct. If
// not, see <https://www.gnu.org/licenses/>.

//! Error variants.
//!
//! This module defines the error types used by oct.
//! All of these types define (at least conditionally) the [`Error`](core::error::Error) trait.

use crate::use_mod;

use_mod!(pub char_decode_error);
use_mod!(pub collection_decode_error);
use_mod!(pub collection_encode_error);
use_mod!(pub enum_decode_error);
use_mod!(pub enum_encode_error);
use_mod!(pub generic_decode_error);
use_mod!(pub generic_encode_error);
use_mod!(pub input_error);
use_mod!(pub isize_encode_error);
use_mod!(pub item_decode_error);
use_mod!(pub item_encode_error);
use_mod!(pub length_error);
use_mod!(pub non_zero_decode_error);
use_mod!(pub output_error);
use_mod!(pub ref_cell_encode_error);
use_mod!(pub string_error);
use_mod!(pub usize_encode_error);
use_mod!(pub utf16_error);
use_mod!(pub utf8_error);

#[cfg(feature = "alloc")]
use_mod!(pub c_string_decode_error);

#[cfg(feature = "std")]
use_mod!(pub system_time_decode_error);
