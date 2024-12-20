// Copyright 2024 Gabriel Bjørnager Jensen.
//
// This file is part of oct.
//
// oct is free software: you can redistribute it
// and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// oct is distributed in the hope that it will
// be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Less-
// er General Public License along with oct. If
// not, see <https://www.gnu.org/licenses/>.

use oct::Slot;
use oct::error::CharDecodeError;

#[test]
fn test_buf_write_read() {
	let mut buf = Slot::<char>::new();

	macro_rules! test_read {
		($pattern:pat$(,)?) => {{
			match buf.read() {
				$pattern => { }

				value => panic!("value `{value:?}` does not match pattern `{}`", stringify!($pattern)),
			}
		}};
	}

	buf.write('\u{1F44D}').unwrap();
	assert_eq!(buf, [0x4D, 0xF4, 0x01, 0x00].as_slice());

	buf.copy_from_slice(&[0x00, 0xD8, 0x00, 0x00]);
	test_read!(Err(CharDecodeError { code_point: 0xD800 }));

	buf.copy_from_slice(&[0x3A, 0xFF, 0x00, 0x00]);
	test_read!(Ok('\u{FF3A}'));
}
