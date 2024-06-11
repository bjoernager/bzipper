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

use crate::{FixedString, Serialise, Sstream};

use alloc::boxed::Box;
use alloc::vec;

#[test]
fn test_serialise() {
	let mut buf = vec![0x00; 0x50];
	let mut stream = Sstream::new(&mut buf);

	0x00_u8.serialise(&mut stream).unwrap();
	0xFF_u8.serialise(&mut stream).unwrap();
	0x7F_u8.serialise(&mut stream).unwrap();

	0x0F_7E_u16.serialise(&mut stream).unwrap();

	0x00_2F_87_E7_u32.serialise(&mut stream).unwrap();

	0xF3_37_CF_8B_DB_03_2B_39_u64.serialise(&mut stream).unwrap();

	0x45_A0_15_6A_36_77_17_8A_83_2E_3C_2C_84_10_58_1A_u128.serialise(&mut stream).unwrap();

	FixedString::<0x1>::new("A").unwrap().serialise(&mut stream).unwrap();
	FixedString::<0x8>::new("l\u{00F8}gma\u{00F0}ur").unwrap().serialise(&mut stream).unwrap();

	['\u{03B4}', '\u{0190}', '\u{03BB}', '\u{03A4}', '\u{03B1}'].serialise(&mut stream).unwrap();

	Ok::<u16, char>(0x45_45).serialise(&mut stream).unwrap();
	Err::<u16, char>(char::REPLACEMENT_CHARACTER).serialise(&mut stream).unwrap();

	None::<()>.serialise(&mut stream).unwrap();
	Some::<()>(()).serialise(&mut stream).unwrap();

	let data: Box<[u8]> = buf.into();

	assert_eq!(
		data.as_ref(),
		[
			0x00, 0xFF, 0x7F, 0x0F, 0x7E, 0x00, 0x2F, 0x87,
			0xE7, 0xF3, 0x37, 0xCF, 0x8B, 0xDB, 0x03, 0x2B,
			0x39, 0x45, 0xA0, 0x15, 0x6A, 0x36, 0x77, 0x17,
			0x8A, 0x83, 0x2E, 0x3C, 0x2C, 0x84, 0x10, 0x58,
			0x1A, 0x00, 0x01, 0x41, 0x00, 0x0A, 0x6C, 0xC3,
			0xB8, 0x67, 0x6D, 0x61, 0xC3, 0xB0, 0x75, 0x72,
			0x00, 0x05, 0x00, 0x00, 0x03, 0xB4, 0x00, 0x00,
			0x01, 0x90, 0x00, 0x00, 0x03, 0xBB, 0x00, 0x00,
			0x03, 0xA4, 0x00, 0x00, 0x03, 0xB1, 0x00, 0x45,
			0x45, 0x01, 0x00, 0x00, 0xFF, 0xFD, 0x00, 0x01,
		],
	);
}