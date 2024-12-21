// Copyright 2024 Gabriel Bjørnager Jensen.
//
// This file is part of Oct.
//
// Oct is free software: you can redistribute it
// and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// Oct is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even
// the implied warranty of MERCHANTABILITY or FIT-
// NESS FOR A PARTICULAR PURPOSE. See the GNU Less-
// er General Public License for more details.
//
// You should have received a copy of the GNU Less-
// er General Public License along with Oct. If
// not, see <https://www.gnu.org/licenses/>.

//! Decoding-related facilities.

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
