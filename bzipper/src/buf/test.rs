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

use bzipper::Buf;
use bzipper::error::DecodeError;

#[test]
fn test_buf_write_read() {
	let mut buf = Buf::<char>::new();

	macro_rules! test_read {
		($pattern:pat$(,)?) => {{
			match buf.read() {
				$pattern => { }

				value => panic!("value `{value:?}` does not match pattern `{}`", stringify!($pattern)),
			}
		}};
	}

	buf.write('\u{1F44D}').unwrap();
	assert_eq!(buf, [0x00, 0x01, 0xF4, 0x4D].as_slice());

	buf.copy_from_slice(&[0x00, 0x00, 0xD8, 0x00]);
	test_read!(Err(DecodeError::InvalidCodePoint(0xD800)));

	buf.copy_from_slice(&[0x00, 0x00, 0xFF, 0x3A]);
	test_read!(Ok('\u{FF3A}'));
}
