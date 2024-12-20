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

use crate::SizedStr;
use crate::decode::{Decode, DecodeBorrowed, Input};
use crate::encode::{Encode, Output, SizedEncode};
use crate::error::{CollectionDecodeError, LengthError, StringError, Utf8Error};

impl<const N: usize> Decode for SizedStr<N> {
	type Error = CollectionDecodeError<LengthError, Utf8Error>;

	#[inline]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let len = Decode::decode(input).unwrap();

		let data = input.read(len).unwrap();

		Self::from_utf8(data)
			.map_err(|e| match e {
				StringError::BadUtf8(e) => CollectionDecodeError::BadItem(e),

				StringError::SmallBuffer(e) => CollectionDecodeError::BadLength(e),

				_ => unreachable!(),
			})
	}
}

impl<const N: usize> DecodeBorrowed<str> for SizedStr<N> { }

impl<const N: usize> Encode for SizedStr<N> {
	type Error = <str as Encode>::Error;

	#[inline(always)]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		self.as_str().encode(output)
	}
}

impl<const N: usize> SizedEncode for SizedStr<N> {
	const MAX_ENCODED_SIZE: usize =
		usize::MAX_ENCODED_SIZE
		+ u8::MAX_ENCODED_SIZE * N;
}
