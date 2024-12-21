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

use crate::encode::Encode;

use core::convert::Infallible;
use core::error::Error;
use core::fmt::{self, Debug, Display, Formatter};

/// An invalid enumeration descriminant was provided.
#[derive(Debug)]
#[must_use]
pub enum EnumEncodeError<D: Encode, F> {
	/// The discriminant could not be encoded.
	BadDiscriminant(D::Error),

	/// A field could not be encoded.
	BadField(F),
}

impl<D, F> Display for EnumEncodeError<D, F>
where
	D: Display + Encode<Error: Display>,
	F: Display,
{
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match *self {
			Self::BadDiscriminant(ref e)
			=> write!(f, "discriminant could not be encoded: {e}"),

			Self::BadField(ref e)
			=> write!(f, "field could not be encoded: {e}"),
		}
	}
}

impl<D, F> Error for EnumEncodeError<D, F>
where
	D: Debug + Display + Encode<Error: Error + 'static>,
	F: Error + 'static,
{
	#[inline]
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match *self {
			Self::BadDiscriminant(ref e) => Some(e),

			Self::BadField(ref e) => Some(e),
		}
	}
}

impl<D, F> From<EnumEncodeError<D, F>> for Infallible
where
	D: Encode<Error: Into<Self>>,
	F: Into<Self>,
{
	#[inline(always)]
	fn from(_value: EnumEncodeError<D, F>) -> Self {
		unreachable!()
	}
}
