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

use crate::SizedEncode;

/// Implemented for tuples with up to twelve members.
#[cfg_attr(doc, doc(fake_variadic))]
unsafe impl<T> SizedEncode for (T, )
where
	T: SizedEncode, {

	#[doc(hidden)]
	const MAX_ENCODED_SIZE: usize =
		T::MAX_ENCODED_SIZE;
}

#[doc(hidden)]
unsafe impl<T0, T1> SizedEncode for (T0, T1)
where
	T0: SizedEncode,
	T1: SizedEncode, {
	const MAX_ENCODED_SIZE: usize =
		T0::MAX_ENCODED_SIZE
		+ T1::MAX_ENCODED_SIZE;
}

#[doc(hidden)]
unsafe impl<T0, T1, T2> SizedEncode for (T0, T1, T2)
where
	T0: SizedEncode,
	T1: SizedEncode,
	T2: SizedEncode, {
	const MAX_ENCODED_SIZE: usize =
		T0::MAX_ENCODED_SIZE
		+ T1::MAX_ENCODED_SIZE
		+ T2::MAX_ENCODED_SIZE;
}

#[doc(hidden)]
unsafe impl<T0, T1, T2, T3> SizedEncode for (T0, T1, T2, T3)
where
	T0: SizedEncode,
	T1: SizedEncode,
	T2: SizedEncode,
	T3: SizedEncode, {
	const MAX_ENCODED_SIZE: usize =
		T0::MAX_ENCODED_SIZE
		+ T1::MAX_ENCODED_SIZE
		+ T2::MAX_ENCODED_SIZE
		+ T3::MAX_ENCODED_SIZE;
}

#[doc(hidden)]
unsafe impl<T0, T1, T2, T3, T4> SizedEncode for (T0, T1, T2, T3, T4)
where
	T0: SizedEncode,
	T1: SizedEncode,
	T2: SizedEncode,
	T3: SizedEncode,
	T4: SizedEncode, {
	const MAX_ENCODED_SIZE: usize =
		T0::MAX_ENCODED_SIZE
		+ T1::MAX_ENCODED_SIZE
		+ T2::MAX_ENCODED_SIZE
		+ T3::MAX_ENCODED_SIZE
		+ T4::MAX_ENCODED_SIZE;
}

#[doc(hidden)]
unsafe impl<T0, T1, T2, T3, T4, T5> SizedEncode for (T0, T1, T2, T3, T4, T5)
where
	T0: SizedEncode,
	T1: SizedEncode,
	T2: SizedEncode,
	T3: SizedEncode,
	T4: SizedEncode,
	T5: SizedEncode, {
	const MAX_ENCODED_SIZE: usize =
		T0::MAX_ENCODED_SIZE
		+ T1::MAX_ENCODED_SIZE
		+ T2::MAX_ENCODED_SIZE
		+ T3::MAX_ENCODED_SIZE
		+ T4::MAX_ENCODED_SIZE
		+ T5::MAX_ENCODED_SIZE;
}

#[doc(hidden)]
unsafe impl<T0, T1, T2, T3, T4, T5, T6> SizedEncode for (T0, T1, T2, T3, T4, T5, T6)
where
	T0: SizedEncode,
	T1: SizedEncode,
	T2: SizedEncode,
	T3: SizedEncode,
	T4: SizedEncode,
	T5: SizedEncode,
	T6: SizedEncode, {
	const MAX_ENCODED_SIZE: usize =
		T0::MAX_ENCODED_SIZE
		+ T1::MAX_ENCODED_SIZE
		+ T2::MAX_ENCODED_SIZE
		+ T3::MAX_ENCODED_SIZE
		+ T4::MAX_ENCODED_SIZE
		+ T5::MAX_ENCODED_SIZE
		+ T6::MAX_ENCODED_SIZE;
}

#[doc(hidden)]
unsafe impl<T0, T1, T2, T3, T4, T5, T6, T7> SizedEncode for (T0, T1, T2, T3, T4, T5, T6, T7)
where
	T0: SizedEncode,
	T1: SizedEncode,
	T2: SizedEncode,
	T3: SizedEncode,
	T4: SizedEncode,
	T5: SizedEncode,
	T6: SizedEncode,
	T7: SizedEncode, {
	const MAX_ENCODED_SIZE: usize =
		T0::MAX_ENCODED_SIZE
		+ T1::MAX_ENCODED_SIZE
		+ T2::MAX_ENCODED_SIZE
		+ T3::MAX_ENCODED_SIZE
		+ T4::MAX_ENCODED_SIZE
		+ T5::MAX_ENCODED_SIZE
		+ T6::MAX_ENCODED_SIZE
		+ T7::MAX_ENCODED_SIZE;
}

#[doc(hidden)]
unsafe impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> SizedEncode for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
	T0: SizedEncode,
	T1: SizedEncode,
	T2: SizedEncode,
	T3: SizedEncode,
	T4: SizedEncode,
	T5: SizedEncode,
	T6: SizedEncode,
	T7: SizedEncode,
	T8: SizedEncode, {
	const MAX_ENCODED_SIZE: usize =
		T0::MAX_ENCODED_SIZE
		+ T1::MAX_ENCODED_SIZE
		+ T2::MAX_ENCODED_SIZE
		+ T3::MAX_ENCODED_SIZE
		+ T4::MAX_ENCODED_SIZE
		+ T5::MAX_ENCODED_SIZE
		+ T6::MAX_ENCODED_SIZE
		+ T7::MAX_ENCODED_SIZE
		+ T8::MAX_ENCODED_SIZE;
}

#[doc(hidden)]
unsafe impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> SizedEncode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
	T0: SizedEncode,
	T1: SizedEncode,
	T2: SizedEncode,
	T3: SizedEncode,
	T4: SizedEncode,
	T5: SizedEncode,
	T6: SizedEncode,
	T7: SizedEncode,
	T8: SizedEncode,
	T9: SizedEncode, {
	const MAX_ENCODED_SIZE: usize =
		T0::MAX_ENCODED_SIZE
		+ T1::MAX_ENCODED_SIZE
		+ T2::MAX_ENCODED_SIZE
		+ T3::MAX_ENCODED_SIZE
		+ T4::MAX_ENCODED_SIZE
		+ T5::MAX_ENCODED_SIZE
		+ T6::MAX_ENCODED_SIZE
		+ T7::MAX_ENCODED_SIZE
		+ T8::MAX_ENCODED_SIZE
		+ T9::MAX_ENCODED_SIZE;
}

#[doc(hidden)]
unsafe impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> SizedEncode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
	T0:  SizedEncode,
	T1:  SizedEncode,
	T2:  SizedEncode,
	T3:  SizedEncode,
	T4:  SizedEncode,
	T5:  SizedEncode,
	T6:  SizedEncode,
	T7:  SizedEncode,
	T8:  SizedEncode,
	T9:  SizedEncode,
	T10: SizedEncode, {
	const MAX_ENCODED_SIZE: usize =
		T0::MAX_ENCODED_SIZE
		+ T1::MAX_ENCODED_SIZE
		+ T2::MAX_ENCODED_SIZE
		+ T3::MAX_ENCODED_SIZE
		+ T4::MAX_ENCODED_SIZE
		+ T5::MAX_ENCODED_SIZE
		+ T6::MAX_ENCODED_SIZE
		+ T7::MAX_ENCODED_SIZE
		+ T8::MAX_ENCODED_SIZE
		+ T9::MAX_ENCODED_SIZE
		+ T10::MAX_ENCODED_SIZE;
}

#[doc(hidden)]
unsafe impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> SizedEncode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
	T0:  SizedEncode,
	T1:  SizedEncode,
	T2:  SizedEncode,
	T3:  SizedEncode,
	T4:  SizedEncode,
	T5:  SizedEncode,
	T6:  SizedEncode,
	T7:  SizedEncode,
	T8:  SizedEncode,
	T9:  SizedEncode,
	T10: SizedEncode,
	T11: SizedEncode, {
	const MAX_ENCODED_SIZE: usize =
		T0::MAX_ENCODED_SIZE
		+ T1::MAX_ENCODED_SIZE
		+ T2::MAX_ENCODED_SIZE
		+ T3::MAX_ENCODED_SIZE
		+ T4::MAX_ENCODED_SIZE
		+ T5::MAX_ENCODED_SIZE
		+ T6::MAX_ENCODED_SIZE
		+ T7::MAX_ENCODED_SIZE
		+ T8::MAX_ENCODED_SIZE
		+ T9::MAX_ENCODED_SIZE
		+ T10::MAX_ENCODED_SIZE
		+ T11::MAX_ENCODED_SIZE;
}
