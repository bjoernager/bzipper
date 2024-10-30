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

use crate::{Encode, OStream};
use crate::error::EncodeError;

/// Implemented for tuples with up to twelve members.
#[cfg_attr(doc, doc(fake_variadic))]
impl<T> Encode for (T, )
where
	T: Encode, {

	#[doc(hidden)]
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		self.0.encode(stream)?;

		Ok(())
	}
}

#[doc(hidden)]
impl<T0, T1> Encode for (T0, T1)
where
	T0: Encode,
	T1: Encode, {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		self.0.encode(stream)?;
		self.1.encode(stream)?;

		Ok(())
	}
}

#[doc(hidden)]
impl<T0, T1, T2> Encode for (T0, T1, T2)
where
	T0: Encode,
	T1: Encode,
	T2: Encode, {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		self.0.encode(stream)?;
		self.1.encode(stream)?;
		self.2.encode(stream)?;

		Ok(())
	}
}

#[doc(hidden)]
impl<T0, T1, T2, T3> Encode for (T0, T1, T2, T3)
where
	T0: Encode,
	T1: Encode,
	T2: Encode,
	T3: Encode, {
	#[inline(always)]
		fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		self.0.encode(stream)?;
		self.1.encode(stream)?;
		self.2.encode(stream)?;
		self.3.encode(stream)?;

		Ok(())
	}
}

#[doc(hidden)]
impl<T0, T1, T2, T3, T4> Encode for (T0, T1, T2, T3, T4)
where
	T0: Encode,
	T1: Encode,
	T2: Encode,
	T3: Encode,
	T4: Encode, {
	#[inline(always)]
		fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		self.0.encode(stream)?;
		self.1.encode(stream)?;
		self.2.encode(stream)?;
		self.3.encode(stream)?;
		self.4.encode(stream)?;

		Ok(())
	}
}

#[doc(hidden)]
impl<T0, T1, T2, T3, T4, T5> Encode for (T0, T1, T2, T3, T4, T5)
where
	T0: Encode,
	T1: Encode,
	T2: Encode,
	T3: Encode,
	T4: Encode,
	T5: Encode, {
	#[inline(always)]
		fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		self.0.encode(stream)?;
		self.1.encode(stream)?;
		self.2.encode(stream)?;
		self.3.encode(stream)?;
		self.4.encode(stream)?;
		self.5.encode(stream)?;

		Ok(())
	}
}

#[doc(hidden)]
impl<T0, T1, T2, T3, T4, T5, T6> Encode for (T0, T1, T2, T3, T4, T5, T6)
where
	T0: Encode,
	T1: Encode,
	T2: Encode,
	T3: Encode,
	T4: Encode,
	T5: Encode,
	T6: Encode, {
	#[inline(always)]
		fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		self.0.encode(stream)?;
		self.1.encode(stream)?;
		self.2.encode(stream)?;
		self.3.encode(stream)?;
		self.4.encode(stream)?;
		self.5.encode(stream)?;
		self.6.encode(stream)?;

		Ok(())
	}
}

#[doc(hidden)]
impl<T0, T1, T2, T3, T4, T5, T6, T7> Encode for (T0, T1, T2, T3, T4, T5, T6, T7)
where
	T0: Encode,
	T1: Encode,
	T2: Encode,
	T3: Encode,
	T4: Encode,
	T5: Encode,
	T6: Encode,
	T7: Encode, {
	#[inline(always)]
		fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		self.0.encode(stream)?;
		self.1.encode(stream)?;
		self.2.encode(stream)?;
		self.3.encode(stream)?;
		self.4.encode(stream)?;
		self.5.encode(stream)?;
		self.6.encode(stream)?;
		self.7.encode(stream)?;

		Ok(())
	}
}

#[doc(hidden)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Encode for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
	T0: Encode,
	T1: Encode,
	T2: Encode,
	T3: Encode,
	T4: Encode,
	T5: Encode,
	T6: Encode,
	T7: Encode,
	T8: Encode, {
	#[inline(always)]
		fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		self.0.encode(stream)?;
		self.1.encode(stream)?;
		self.2.encode(stream)?;
		self.3.encode(stream)?;
		self.4.encode(stream)?;
		self.5.encode(stream)?;
		self.6.encode(stream)?;
		self.7.encode(stream)?;
		self.8.encode(stream)?;

		Ok(())
	}
}

#[doc(hidden)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Encode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
	T0: Encode,
	T1: Encode,
	T2: Encode,
	T3: Encode,
	T4: Encode,
	T5: Encode,
	T6: Encode,
	T7: Encode,
	T8: Encode,
	T9: Encode, {
	#[inline(always)]
		fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		self.0.encode(stream)?;
		self.1.encode(stream)?;
		self.2.encode(stream)?;
		self.3.encode(stream)?;
		self.4.encode(stream)?;
		self.5.encode(stream)?;
		self.6.encode(stream)?;
		self.7.encode(stream)?;
		self.8.encode(stream)?;
		self.9.encode(stream)?;

		Ok(())
	}
}

#[doc(hidden)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Encode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
	T0:  Encode,
	T1:  Encode,
	T2:  Encode,
	T3:  Encode,
	T4:  Encode,
	T5:  Encode,
	T6:  Encode,
	T7:  Encode,
	T8:  Encode,
	T9:  Encode,
	T10: Encode, {
	#[inline(always)]
		fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		self.0.encode(stream)?;
		self.1.encode(stream)?;
		self.2.encode(stream)?;
		self.3.encode(stream)?;
		self.4.encode(stream)?;
		self.5.encode(stream)?;
		self.6.encode(stream)?;
		self.7.encode(stream)?;
		self.8.encode(stream)?;
		self.9.encode(stream)?;
		self.10.encode(stream)?;

		Ok(())
	}
}

#[doc(hidden)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Encode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
	T0:  Encode,
	T1:  Encode,
	T2:  Encode,
	T3:  Encode,
	T4:  Encode,
	T5:  Encode,
	T6:  Encode,
	T7:  Encode,
	T8:  Encode,
	T9:  Encode,
	T10: Encode,
	T11: Encode, {
	#[inline(always)]
		fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		self.0.encode(stream)?;
		self.1.encode(stream)?;
		self.2.encode(stream)?;
		self.3.encode(stream)?;
		self.4.encode(stream)?;
		self.5.encode(stream)?;
		self.6.encode(stream)?;
		self.7.encode(stream)?;
		self.8.encode(stream)?;
		self.9.encode(stream)?;
		self.10.encode(stream)?;
		self.11.encode(stream)?;

		Ok(())
	}
}
