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
	SizedSlice
};
use crate::error::{CollectionDecodeError, ItemDecodeError, SizeError};

use core::mem::MaybeUninit;

impl<T: Decode, const N: usize> Decode for SizedSlice<T, N> {
	type Error = CollectionDecodeError<SizeError, ItemDecodeError<usize, T::Error>>;

	#[inline]
	fn decode(stream: &mut IStream) -> Result<Self, Self::Error> {
		let len = Decode::decode(stream).unwrap();
		if len > N { return Err(CollectionDecodeError::Length(SizeError { cap: N, len })) };

		let mut buf: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

		for (i, slot) in buf.iter_mut().enumerate() {
			let v = Decode::decode(stream)
				.map_err(|e| CollectionDecodeError::Item(ItemDecodeError { index: i, error: e }))?;

			slot.write(v);
		}

		Ok(Self { buf, len })
	}
}

impl<T: Decode, const N: usize> DecodeBorrowed<[T]> for SizedSlice<T, N> { }

impl<T: Encode, const N: usize> Encode for SizedSlice<T, N> {
	type Error = <[T] as Encode>::Error;

	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), Self::Error> {
		self.as_slice().encode(stream)
	}
}

impl<T: SizedEncode, const N: usize> SizedEncode for SizedSlice<T, N> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE * N;
}