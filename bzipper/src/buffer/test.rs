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

use crate::{Buffer, Error};

#[test]
fn test_buffer() {
	let mut buf = Buffer::<char>::new();

	buf.write(&'\u{1F44D}').unwrap();
	assert_eq!(buf, [0x00, 0x01, 0xF4, 0x4D].as_slice());

	buf.as_mut_slice().copy_from_slice(&[0x00, 0x00, 0xD8, 0x00]);
	assert!(matches!(buf.read(), Err(Error::InvalidCodePoint { value: 0xD800 })));

	buf.as_mut_slice().copy_from_slice(&[0x00, 0x00, 0xFF, 0x3A]);
	assert_eq!(buf.read().unwrap(), '\u{FF3A}');
}
