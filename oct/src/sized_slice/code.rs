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

use crate::SizedSlice;
use crate::decode::{Decode, DecodeBorrowed, Input};
use crate::encode::{Encode, Output, SizedEncode};
use crate::error::{CollectionDecodeError, ItemDecodeError, LengthError};

use core::mem::MaybeUninit;

impl<T: Decode, const N: usize> Decode for SizedSlice<T, N> {
	type Error = CollectionDecodeError<LengthError, ItemDecodeError<usize, T::Error>>;

	#[inline]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let len = Decode::decode(input).unwrap();
		if len > N { return Err(CollectionDecodeError::BadLength(LengthError { capacity: N, len })) };

		let mut buf = [const { MaybeUninit::<T>::uninit() };N];

		for (i, slot) in buf.iter_mut().enumerate() {
			let v = Decode::decode(input)
				.map_err(|e| CollectionDecodeError::BadItem(ItemDecodeError { index: i, error: e }))?;

			slot.write(v);
		}

		Ok(Self { buf, len })
	}
}

impl<T: Decode, const N: usize> DecodeBorrowed<[T]> for SizedSlice<T, N> { }

impl<T: Encode, const N: usize> Encode for SizedSlice<T, N> {
	type Error = <[T] as Encode>::Error;

	#[inline(always)]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		self.as_slice().encode(output)
	}
}

impl<T: SizedEncode, const N: usize> SizedEncode for SizedSlice<T, N> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE * N;
}
