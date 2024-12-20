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

use_mod!(pub decode);
use_mod!(pub decode_borrowed);
use_mod!(pub input);

/// Implements [`Decode`] for the provided type.
///
/// This macro assumes the same format used by the equivalent [`Encode`](derive@crate::encode::Encode) macro.
#[cfg(feature = "proc-macro")]
#[cfg_attr(doc, doc(cfg(feature = "proc-macro")))]
#[doc(inline)]
pub use oct_macros::Decode;
