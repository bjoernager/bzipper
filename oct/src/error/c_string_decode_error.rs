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

use core::error::Error;
use core::fmt::{self, Display, Formatter};

/// A C-like string could not be decoded.
///
/// This error is generatead when <code>&lt;[CString](alloc::ffi::CString) as [Decode](crate::decode::Decode)&gt;::[decode](crate::decode::Decode::decode)</code> encounteres a null byte within bounds.
///
/// Note that although any null value is *theoretically* also the string's null terminator, the implementations for [`CStr`](core::ffi::CStr) and `CString` use the same encoding scheme as [`[u8]`](slice).
/// This is mainly for efficiency's sake (as to allow the entire stream to be read at once), but this also allows for the aforementioned case to happen.
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
#[derive(Debug)]
#[must_use]
pub struct CStringDecodeError {
	/// The index of the null value.
	pub index: usize,
}

impl Display for CStringDecodeError {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "expected c string but found null value within bounds at '{}`", self.index)
	}
}

impl Error for CStringDecodeError { }
