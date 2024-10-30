// Copyright 2024 Gabriel Bjørnager Jensen.
//
// This file is part of bZipper.
//
// bZipper is free software: you can redistribute
// it and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// bZipper is distributed in the hope that it will
// be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Less-
// er General Public License along with bZipper. If
// not, see <https://www.gnu.org/licenses/>.

use crate::{IStream, Decode};
use crate::error::DecodeError;

/// Implemented for tuples with up to twelve members.
#[cfg_attr(doc, doc(fake_variadic))]
impl<T> Decode for (T, )
where
	T: Decode, {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let value = (
			Decode::decode(stream)?,
		);

		Ok(value)
	}
}

#[doc(hidden)]
impl<T0, T1> Decode for (T0, T1)
where
	T0: Decode,
	T1: Decode, {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let value = (
			Decode::decode(stream)?,
			Decode::decode(stream)?,
		);

		Ok(value)
	}
}

#[doc(hidden)]
impl<T0, T1, T2> Decode for (T0, T1, T2)
where
	T0: Decode,
	T1: Decode,
	T2: Decode, {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let value = (
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
		);

		Ok(value)
	}
}

#[doc(hidden)]
impl<T0, T1, T2, T3> Decode for (T0, T1, T2, T3)
where
	T0: Decode,
	T1: Decode,
	T2: Decode,
	T3: Decode, {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let value = (
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
		);

		Ok(value)
	}
}

#[doc(hidden)]
impl<T0, T1, T2, T3, T4> Decode for (T0, T1, T2, T3, T4)
where
	T0: Decode,
	T1: Decode,
	T2: Decode,
	T3: Decode,
	T4: Decode, {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let value = (
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
		);

		Ok(value)
	}
}

#[doc(hidden)]
impl<T0, T1, T2, T3, T4, T5> Decode for (T0, T1, T2, T3, T4, T5)
where
	T0: Decode,
	T1: Decode,
	T2: Decode,
	T3: Decode,
	T4: Decode,
	T5: Decode, {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let value = (
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
		);

		Ok(value)
	}
}

#[doc(hidden)]
impl<T0, T1, T2, T3, T4, T5, T6> Decode for (T0, T1, T2, T3, T4, T5, T6)
where
	T0: Decode,
	T1: Decode,
	T2: Decode,
	T3: Decode,
	T4: Decode,
	T5: Decode,
	T6: Decode, {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let value = (
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
		);

		Ok(value)
	}
}

#[doc(hidden)]
impl<T0, T1, T2, T3, T4, T5, T6, T7> Decode for (T0, T1, T2, T3, T4, T5, T6, T7)
where
	T0: Decode,
	T1: Decode,
	T2: Decode,
	T3: Decode,
	T4: Decode,
	T5: Decode,
	T6: Decode,
	T7: Decode, {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let value = (
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
		);

		Ok(value)
	}
}

#[doc(hidden)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Decode for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
	T0: Decode,
	T1: Decode,
	T2: Decode,
	T3: Decode,
	T4: Decode,
	T5: Decode,
	T6: Decode,
	T7: Decode,
	T8: Decode, {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let value = (
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
		);

		Ok(value)
	}
}

#[doc(hidden)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Decode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
	T0: Decode,
	T1: Decode,
	T2: Decode,
	T3: Decode,
	T4: Decode,
	T5: Decode,
	T6: Decode,
	T7: Decode,
	T8: Decode,
	T9: Decode, {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let value = (
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
		);

		Ok(value)
	}
}

#[doc(hidden)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Decode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
	T0:  Decode,
	T1:  Decode,
	T2:  Decode,
	T3:  Decode,
	T4:  Decode,
	T5:  Decode,
	T6:  Decode,
	T7:  Decode,
	T8:  Decode,
	T9:  Decode,
	T10: Decode, {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let value = (
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
		);

		Ok(value)
	}
}

#[doc(hidden)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Decode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
	T0:  Decode,
	T1:  Decode,
	T2:  Decode,
	T3:  Decode,
	T4:  Decode,
	T5:  Decode,
	T6:  Decode,
	T7:  Decode,
	T8:  Decode,
	T9:  Decode,
	T10: Decode,
	T11: Decode, {
	#[inline(always)]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let value = (
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
			Decode::decode(stream)?,
		);

		Ok(value)
	}
}
