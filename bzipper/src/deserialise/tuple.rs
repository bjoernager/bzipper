// Copyright 2024 Gabriel Bjørnager Jensen.
//
// This file is part of bzipper.
//
// bzipper is free software: you can redistribute
// it and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// bzipper is distributed in the hope that it will
// be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Less-
// er General Public License along with bzipper. If
// not, see <https://www.gnu.org/licenses/>.

use crate::{Deserialise, Dstream, Result};

impl<T0> Deserialise for (T0, )
where
	T0: Deserialise, {
	fn deserialise(stream: &Dstream) -> Result<Self> {
		let value = (
			Deserialise::deserialise(stream)?,
		);

		Ok(value)
	}
}

impl<T0, T1> Deserialise for (T0, T1)
where
	T0: Deserialise,
	T1: Deserialise, {
	fn deserialise(stream: &Dstream) -> Result<Self> {
		let value = (
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
		);

		Ok(value)
	}
}

impl<T0, T1, T2> Deserialise for (T0, T1, T2)
where
	T0: Deserialise,
	T1: Deserialise,
	T2: Deserialise, {
	fn deserialise(stream: &Dstream) -> Result<Self> {
		let value = (
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
		);

		Ok(value)
	}
}

impl<T0, T1, T2, T3> Deserialise for (T0, T1, T2, T3)
where
	T0: Deserialise,
	T1: Deserialise,
	T2: Deserialise,
	T3: Deserialise, {
	fn deserialise(stream: &Dstream) -> Result<Self> {
		let value = (
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
		);

		Ok(value)
	}
}

impl<T0, T1, T2, T3, T4> Deserialise for (T0, T1, T2, T3, T4)
where
	T0: Deserialise,
	T1: Deserialise,
	T2: Deserialise,
	T3: Deserialise,
	T4: Deserialise, {
	fn deserialise(stream: &Dstream) -> Result<Self> {
		let value = (
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
		);

		Ok(value)
	}
}

impl<T0, T1, T2, T3, T4, T5> Deserialise for (T0, T1, T2, T3, T4, T5)
where
	T0: Deserialise,
	T1: Deserialise,
	T2: Deserialise,
	T3: Deserialise,
	T4: Deserialise,
	T5: Deserialise, {
	fn deserialise(stream: &Dstream) -> Result<Self> {
		let value = (
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
		);

		Ok(value)
	}
}

impl<T0, T1, T2, T3, T4, T5, T6> Deserialise for (T0, T1, T2, T3, T4, T5, T6)
where
	T0: Deserialise,
	T1: Deserialise,
	T2: Deserialise,
	T3: Deserialise,
	T4: Deserialise,
	T5: Deserialise,
	T6: Deserialise, {
	fn deserialise(stream: &Dstream) -> Result<Self> {
		let value = (
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
		);

		Ok(value)
	}
}

impl<T0, T1, T2, T3, T4, T5, T6, T7> Deserialise for (T0, T1, T2, T3, T4, T5, T6, T7)
where
	T0: Deserialise,
	T1: Deserialise,
	T2: Deserialise,
	T3: Deserialise,
	T4: Deserialise,
	T5: Deserialise,
	T6: Deserialise,
	T7: Deserialise, {
	fn deserialise(stream: &Dstream) -> Result<Self> {
		let value = (
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
		);

		Ok(value)
	}
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Deserialise for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
	T0: Deserialise,
	T1: Deserialise,
	T2: Deserialise,
	T3: Deserialise,
	T4: Deserialise,
	T5: Deserialise,
	T6: Deserialise,
	T7: Deserialise,
	T8: Deserialise, {
	fn deserialise(stream: &Dstream) -> Result<Self> {
		let value = (
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
		);

		Ok(value)
	}
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Deserialise for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
	T0: Deserialise,
	T1: Deserialise,
	T2: Deserialise,
	T3: Deserialise,
	T4: Deserialise,
	T5: Deserialise,
	T6: Deserialise,
	T7: Deserialise,
	T8: Deserialise,
	T9: Deserialise, {
	fn deserialise(stream: &Dstream) -> Result<Self> {
		let value = (
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
		);

		Ok(value)
	}
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Deserialise for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
	T0:  Deserialise,
	T1:  Deserialise,
	T2:  Deserialise,
	T3:  Deserialise,
	T4:  Deserialise,
	T5:  Deserialise,
	T6:  Deserialise,
	T7:  Deserialise,
	T8:  Deserialise,
	T9:  Deserialise,
	T10: Deserialise, {
	fn deserialise(stream: &Dstream) -> Result<Self> {
		let value = (
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
		);

		Ok(value)
	}
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Deserialise for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
	T0:  Deserialise,
	T1:  Deserialise,
	T2:  Deserialise,
	T3:  Deserialise,
	T4:  Deserialise,
	T5:  Deserialise,
	T6:  Deserialise,
	T7:  Deserialise,
	T8:  Deserialise,
	T9:  Deserialise,
	T10: Deserialise,
	T11: Deserialise, {
	fn deserialise(stream: &Dstream) -> Result<Self> {
		let value = (
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
			Deserialise::deserialise(stream)?,
		);

		Ok(value)
	}
}
