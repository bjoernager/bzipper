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

/// Iterator to a fixed string.
pub struct FixedStringIter<const N: usize> {
	pub(in crate) buf: [char; N],
	pub(in crate) len: usize,

	pub(in crate) pos: Option<usize>,
}

impl<const N: usize> Iterator for FixedStringIter<N> {
	type Item = char;

	fn next(&mut self) -> Option<Self::Item> {
		let pos = self.pos.as_mut()?;

		if *pos >= self.len { return None };

		let item = self.buf[*pos];
		*pos += 0x1;

		Some(item)
	}
}
