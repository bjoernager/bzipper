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

use crate::{Deserialise, Dstream, FixedString};

#[test]
fn test_deserialise() {
	let data = [
		0x00, 0xFF, 0xFF, 0x0F, 0xEF, 0x1F, 0xDF, 0x2F,
		0xCF, 0x3F, 0xBF, 0x4F, 0xAF, 0x5F, 0x9F, 0x6F,
		0x8F, 0x7F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		0x00, 0x09, 0x6D, 0xC3, 0xA1, 0x6E, 0x61, 0xC3,
		0xB0, 0x75, 0x72, 0x00, 0x00, 0x00, 0x00, 0x00,
		0x00, 0x00, 0x05, 0x00, 0x00, 0x03, 0xBB, 0x00,
		0x00, 0x03, 0x91, 0x00, 0x00, 0x03, 0xBC, 0x00,
		0x00, 0x03, 0x94, 0x00, 0x00, 0x03, 0xB1, 0x01,
		0x00, 0x00, 0x01, 0x80,
	];

	let mut stream = Dstream::from(&data);

	assert_eq!(
		u8::deserialise(&mut stream).unwrap(),
		0x00,
	);
	assert_eq!(
		u8::deserialise(&mut stream).unwrap(),
		0xFF,
	);

	assert_eq!(
		u128::deserialise(&mut stream).unwrap(),
		0xFF_0F_EF_1F_DF_2F_CF_3F_BF_4F_AF_5F_9F_6F_8F_7F,
	);

	assert_eq!(
		FixedString::<0x10>::deserialise(&mut stream).unwrap(),
		"m\u{00E1}na\u{00F0}ur",
	);

	assert_eq!(
		<[char; 0x5]>::deserialise(&mut stream).unwrap(),
		['\u{03BB}', '\u{0391}', '\u{03BC}', '\u{0394}', '\u{03B1}'],
	);

	assert_eq!(
		Option::<()>::deserialise(&mut stream).unwrap(),
		Some(()),
	);

	assert_eq!(
		Option::<()>::deserialise(&mut stream).unwrap(),
		None,
	);

	assert_eq!(
		Result::<(), i8>::deserialise(&mut stream).unwrap(),
		Ok(()),
	);

	assert_eq!(
		Result::<(), i8>::deserialise(&mut stream).unwrap(),
		Err(i8::MIN),
	);
}
