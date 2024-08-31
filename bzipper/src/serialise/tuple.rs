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

use crate::{Result, Serialise, Sstream};

impl<T0> Serialise for (T0, )
where
	T0: Serialise, {
	const MAX_SERIALISED_SIZE: usize =
		T0::MAX_SERIALISED_SIZE;

	fn serialise(&self, stream: &mut Sstream) -> Result<()> {
		self.0.serialise(stream)?;

		Ok(())
	}
}

impl<T0, T1> Serialise for (T0, T1)
where
	T0: Serialise,
	T1: Serialise, {
	const MAX_SERIALISED_SIZE: usize =
		T0::MAX_SERIALISED_SIZE
		+ T1::MAX_SERIALISED_SIZE;

	fn serialise(&self, stream: &mut Sstream) -> Result<()> {
		self.0.serialise(stream)?;
		self.1.serialise(stream)?;

		Ok(())
	}
}

impl<T0, T1, T2> Serialise for (T0, T1, T2)
where
	T0: Serialise,
	T1: Serialise,
	T2: Serialise, {
	const MAX_SERIALISED_SIZE: usize =
		T0::MAX_SERIALISED_SIZE
		+ T1::MAX_SERIALISED_SIZE
		+ T2::MAX_SERIALISED_SIZE;

	fn serialise(&self, stream: &mut Sstream) -> Result<()> {
		self.0.serialise(stream)?;
		self.1.serialise(stream)?;
		self.2.serialise(stream)?;

		Ok(())
	}
}

impl<T0, T1, T2, T3> Serialise for (T0, T1, T2, T3)
where
	T0: Serialise,
	T1: Serialise,
	T2: Serialise,
	T3: Serialise, {
	const MAX_SERIALISED_SIZE: usize =
		T0::MAX_SERIALISED_SIZE
		+ T1::MAX_SERIALISED_SIZE
		+ T2::MAX_SERIALISED_SIZE
		+ T3::MAX_SERIALISED_SIZE;

		fn serialise(&self, stream: &mut Sstream) -> Result<()> {
		self.0.serialise(stream)?;
		self.1.serialise(stream)?;
		self.2.serialise(stream)?;
		self.3.serialise(stream)?;

		Ok(())
	}
}

impl<T0, T1, T2, T3, T4> Serialise for (T0, T1, T2, T3, T4)
where
	T0: Serialise,
	T1: Serialise,
	T2: Serialise,
	T3: Serialise,
	T4: Serialise, {
	const MAX_SERIALISED_SIZE: usize =
		T0::MAX_SERIALISED_SIZE
		+ T1::MAX_SERIALISED_SIZE
		+ T2::MAX_SERIALISED_SIZE
		+ T3::MAX_SERIALISED_SIZE
		+ T4::MAX_SERIALISED_SIZE;

		fn serialise(&self, stream: &mut Sstream) -> Result<()> {
		self.0.serialise(stream)?;
		self.1.serialise(stream)?;
		self.2.serialise(stream)?;
		self.3.serialise(stream)?;
		self.4.serialise(stream)?;

		Ok(())
	}
}

impl<T0, T1, T2, T3, T4, T5> Serialise for (T0, T1, T2, T3, T4, T5)
where
	T0: Serialise,
	T1: Serialise,
	T2: Serialise,
	T3: Serialise,
	T4: Serialise,
	T5: Serialise, {
	const MAX_SERIALISED_SIZE: usize =
		T0::MAX_SERIALISED_SIZE
		+ T1::MAX_SERIALISED_SIZE
		+ T2::MAX_SERIALISED_SIZE
		+ T3::MAX_SERIALISED_SIZE
		+ T4::MAX_SERIALISED_SIZE
		+ T5::MAX_SERIALISED_SIZE;

		fn serialise(&self, stream: &mut Sstream) -> Result<()> {
		self.0.serialise(stream)?;
		self.1.serialise(stream)?;
		self.2.serialise(stream)?;
		self.3.serialise(stream)?;
		self.4.serialise(stream)?;
		self.5.serialise(stream)?;

		Ok(())
	}
}

impl<T0, T1, T2, T3, T4, T5, T6> Serialise for (T0, T1, T2, T3, T4, T5, T6)
where
	T0: Serialise,
	T1: Serialise,
	T2: Serialise,
	T3: Serialise,
	T4: Serialise,
	T5: Serialise,
	T6: Serialise, {
	const MAX_SERIALISED_SIZE: usize =
		T0::MAX_SERIALISED_SIZE
		+ T1::MAX_SERIALISED_SIZE
		+ T2::MAX_SERIALISED_SIZE
		+ T3::MAX_SERIALISED_SIZE
		+ T4::MAX_SERIALISED_SIZE
		+ T5::MAX_SERIALISED_SIZE
		+ T6::MAX_SERIALISED_SIZE;

		fn serialise(&self, stream: &mut Sstream) -> Result<()> {
		self.0.serialise(stream)?;
		self.1.serialise(stream)?;
		self.2.serialise(stream)?;
		self.3.serialise(stream)?;
		self.4.serialise(stream)?;
		self.5.serialise(stream)?;
		self.6.serialise(stream)?;

		Ok(())
	}
}

impl<T0, T1, T2, T3, T4, T5, T6, T7> Serialise for (T0, T1, T2, T3, T4, T5, T6, T7)
where
	T0: Serialise,
	T1: Serialise,
	T2: Serialise,
	T3: Serialise,
	T4: Serialise,
	T5: Serialise,
	T6: Serialise,
	T7: Serialise, {
	const MAX_SERIALISED_SIZE: usize =
		T0::MAX_SERIALISED_SIZE
		+ T1::MAX_SERIALISED_SIZE
		+ T2::MAX_SERIALISED_SIZE
		+ T3::MAX_SERIALISED_SIZE
		+ T4::MAX_SERIALISED_SIZE
		+ T5::MAX_SERIALISED_SIZE
		+ T6::MAX_SERIALISED_SIZE
		+ T7::MAX_SERIALISED_SIZE;

		fn serialise(&self, stream: &mut Sstream) -> Result<()> {
		self.0.serialise(stream)?;
		self.1.serialise(stream)?;
		self.2.serialise(stream)?;
		self.3.serialise(stream)?;
		self.4.serialise(stream)?;
		self.5.serialise(stream)?;
		self.6.serialise(stream)?;
		self.7.serialise(stream)?;

		Ok(())
	}
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Serialise for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
	T0: Serialise,
	T1: Serialise,
	T2: Serialise,
	T3: Serialise,
	T4: Serialise,
	T5: Serialise,
	T6: Serialise,
	T7: Serialise,
	T8: Serialise, {
	const MAX_SERIALISED_SIZE: usize =
		T0::MAX_SERIALISED_SIZE
		+ T1::MAX_SERIALISED_SIZE
		+ T2::MAX_SERIALISED_SIZE
		+ T3::MAX_SERIALISED_SIZE
		+ T4::MAX_SERIALISED_SIZE
		+ T5::MAX_SERIALISED_SIZE
		+ T6::MAX_SERIALISED_SIZE
		+ T7::MAX_SERIALISED_SIZE
		+ T8::MAX_SERIALISED_SIZE;

		fn serialise(&self, stream: &mut Sstream) -> Result<()> {
		self.0.serialise(stream)?;
		self.1.serialise(stream)?;
		self.2.serialise(stream)?;
		self.3.serialise(stream)?;
		self.4.serialise(stream)?;
		self.5.serialise(stream)?;
		self.6.serialise(stream)?;
		self.7.serialise(stream)?;
		self.8.serialise(stream)?;

		Ok(())
	}
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Serialise for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
	T0: Serialise,
	T1: Serialise,
	T2: Serialise,
	T3: Serialise,
	T4: Serialise,
	T5: Serialise,
	T6: Serialise,
	T7: Serialise,
	T8: Serialise,
	T9: Serialise, {
	const MAX_SERIALISED_SIZE: usize =
		T0::MAX_SERIALISED_SIZE
		+ T1::MAX_SERIALISED_SIZE
		+ T2::MAX_SERIALISED_SIZE
		+ T3::MAX_SERIALISED_SIZE
		+ T4::MAX_SERIALISED_SIZE
		+ T5::MAX_SERIALISED_SIZE
		+ T6::MAX_SERIALISED_SIZE
		+ T7::MAX_SERIALISED_SIZE
		+ T8::MAX_SERIALISED_SIZE
		+ T9::MAX_SERIALISED_SIZE;

		fn serialise(&self, stream: &mut Sstream) -> Result<()> {
		self.0.serialise(stream)?;
		self.1.serialise(stream)?;
		self.2.serialise(stream)?;
		self.3.serialise(stream)?;
		self.4.serialise(stream)?;
		self.5.serialise(stream)?;
		self.6.serialise(stream)?;
		self.7.serialise(stream)?;
		self.8.serialise(stream)?;
		self.9.serialise(stream)?;

		Ok(())
	}
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Serialise for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
	T0:  Serialise,
	T1:  Serialise,
	T2:  Serialise,
	T3:  Serialise,
	T4:  Serialise,
	T5:  Serialise,
	T6:  Serialise,
	T7:  Serialise,
	T8:  Serialise,
	T9:  Serialise,
	T10: Serialise, {
	const MAX_SERIALISED_SIZE: usize =
		T0::MAX_SERIALISED_SIZE
		+ T1::MAX_SERIALISED_SIZE
		+ T2::MAX_SERIALISED_SIZE
		+ T3::MAX_SERIALISED_SIZE
		+ T4::MAX_SERIALISED_SIZE
		+ T5::MAX_SERIALISED_SIZE
		+ T6::MAX_SERIALISED_SIZE
		+ T7::MAX_SERIALISED_SIZE
		+ T8::MAX_SERIALISED_SIZE
		+ T9::MAX_SERIALISED_SIZE
		+ T10::MAX_SERIALISED_SIZE;

		fn serialise(&self, stream: &mut Sstream) -> Result<()> {
		self.0.serialise(stream)?;
		self.1.serialise(stream)?;
		self.2.serialise(stream)?;
		self.3.serialise(stream)?;
		self.4.serialise(stream)?;
		self.5.serialise(stream)?;
		self.6.serialise(stream)?;
		self.7.serialise(stream)?;
		self.8.serialise(stream)?;
		self.9.serialise(stream)?;
		self.10.serialise(stream)?;

		Ok(())
	}
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Serialise for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
	T0:  Serialise,
	T1:  Serialise,
	T2:  Serialise,
	T3:  Serialise,
	T4:  Serialise,
	T5:  Serialise,
	T6:  Serialise,
	T7:  Serialise,
	T8:  Serialise,
	T9:  Serialise,
	T10: Serialise,
	T11: Serialise, {
	const MAX_SERIALISED_SIZE: usize =
		T0::MAX_SERIALISED_SIZE
		+ T1::MAX_SERIALISED_SIZE
		+ T2::MAX_SERIALISED_SIZE
		+ T3::MAX_SERIALISED_SIZE
		+ T4::MAX_SERIALISED_SIZE
		+ T5::MAX_SERIALISED_SIZE
		+ T6::MAX_SERIALISED_SIZE
		+ T7::MAX_SERIALISED_SIZE
		+ T8::MAX_SERIALISED_SIZE
		+ T9::MAX_SERIALISED_SIZE
		+ T10::MAX_SERIALISED_SIZE
		+ T11::MAX_SERIALISED_SIZE;

		fn serialise(&self, stream: &mut Sstream) -> Result<()> {
		self.0.serialise(stream)?;
		self.1.serialise(stream)?;
		self.2.serialise(stream)?;
		self.3.serialise(stream)?;
		self.4.serialise(stream)?;
		self.5.serialise(stream)?;
		self.6.serialise(stream)?;
		self.7.serialise(stream)?;
		self.8.serialise(stream)?;
		self.9.serialise(stream)?;
		self.10.serialise(stream)?;
		self.11.serialise(stream)?;

		Ok(())
	}
}
