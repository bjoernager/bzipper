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

use crate::decode::Decode;

use core::convert::Infallible;
use core::error::Error;
use core::fmt::{self, Debug, Display, Formatter};

/// An invalid enumeration descriminant was provided.
#[derive(Debug)]
#[must_use]
pub enum EnumDecodeError<D: Decode, F> {
	/// The discriminant could not be decoded.
	InvalidDiscriminant(D::Error),

	/// An otherwise valid discriminant has not been assigned.
	///
	/// Remember that this error does **not** indicate that the discriminant couldn't be decoded, merely that it does not match with that of any variant.
	/// See also [`InvalidDiscriminant`](Self::InvalidDiscriminant).
	UnassignedDiscriminant {
		/// The unassigned discriminant value.
		value: D
	},

	/// A field could not be encoded.
	BadField(F),
}

impl<D, F> Display for EnumDecodeError<D, F>
where
	D: Decode<Error: Display> + Display,
	F: Display,
{
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match *self {
			Self::InvalidDiscriminant(ref e)
			=> write!(f, "discriminant could not be decoded: {e}"),

			Self::UnassignedDiscriminant { ref value }
			=> write!(f, "`{value}` is not an assigned discriminant for the given enumeration"),

			Self::BadField(ref e)
			=> write!(f, "variant could not be decoded: {e}"),
		}
	}
}

impl<D, F> Error for EnumDecodeError<D, F>
where
	D: Debug + Decode<Error: Error + 'static> + Display,
	F: Error + 'static,
{
	#[inline]
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match *self {
			Self::InvalidDiscriminant(ref e) => Some(e),

			Self::BadField(ref e) => Some(e),

			_ => None,
		}
	}
}

impl<D, F> From<EnumDecodeError<D, F>> for Infallible
where
	D: Decode<Error: Into<Self>>,
	F: Into<Self>,
{
	#[inline(always)]
	fn from(_value: EnumDecodeError<D, F>) -> Self {
		unreachable!()
	}
}
