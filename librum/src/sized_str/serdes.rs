// Copyright 2024 Gabriel Bjørnager Jensen.
//
// This file is part of Librum.
//
// Librum is free software: you can redistribute it
// and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// Librum is distributed in the hope that it will
// be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Less-
// er General Public License along with Librum. If
// not, see <https://www.gnu.org/licenses/>.

use crate::{
	Decode,
	DecodeBorrowed,
	Encode,
	IStream,
	OStream,
	SizedEncode,
	SizedStr
};
use crate::error::{CollectionDecodeError, SizeError, StringError, Utf8Error};

impl<const N: usize> Decode for SizedStr<N> {
	type Error = CollectionDecodeError<SizeError, Utf8Error>;

	#[inline]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let len = Decode::decode(stream).unwrap();

		let data = stream.read(len);

		Self::from_utf8(data)
			.map_err(|e| match e {
				StringError::BadUtf8(e) => CollectionDecodeError::Item(e),

				StringError::SmallBuffer(e) => CollectionDecodeError::Length(e),

				_ => unreachable!(),
			})
	}
}

impl<const N: usize> DecodeBorrowed<str> for SizedStr<N> { }

impl<const N: usize> Encode for SizedStr<N> {
	type Error = <str as Encode>::Error;

	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), Self::Error> {
		self.as_str().encode(stream)
	}
}

impl<const N: usize> SizedEncode for SizedStr<N> {
	const MAX_ENCODED_SIZE: usize =
		usize::MAX_ENCODED_SIZE
		+ u8::MAX_ENCODED_SIZE * N;
}
