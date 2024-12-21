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

use core::error::Error;
use core::fmt::{self, Display, Formatter};

/// A non-zero integer could not be decoded.
///
/// The implementations of [`Decode`](crate::decode::Decode) for <code>[NonZero](core::num::NonZero)&lt;T&gt;</code> yield this error type if decoding `T` yields zero.
#[derive(Debug)]
pub struct NonZeroDecodeError;

impl Display for NonZeroDecodeError {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "expected non-zero integer but found `0`")
	}
}

impl Error for NonZeroDecodeError { }
