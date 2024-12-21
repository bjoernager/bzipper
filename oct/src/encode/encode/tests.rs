// Copyright 2024 Gabriel Bjørnager Jensen.
//
// This file is part of Oct.
//
// Oct is free software: you can redistribute it
// and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// Oct is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even
// the implied warranty of MERCHANTABILITY or FIT-
// NESS FOR A PARTICULAR PURPOSE. See the GNU Less-
// er General Public License for more details.
//
// You should have received a copy of the GNU Less-
// er General Public License along with Oct. If
// not, see <https://www.gnu.org/licenses/>.

use oct::SizedStr;
use oct::encode::{Encode, Output, SizedEncode};
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use std::vec;
use std::vec::Vec;

macro_rules! test {
	($ty:ty: $value:expr => $data:expr) => {{
		let data = $data;

		let mut buf = vec![0x00; data.len()];

		let mut stream = Output::new(&mut buf);
		<$ty as Encode>::encode(&$value, &mut stream).unwrap();

		let len = stream.position();
		assert_eq!(&buf[..len], data.as_slice());
	}};
}

#[test]
fn test_encode() {
	test!(u8: 0x00 => [0x00]);
	test!(u8: 0xFF => [0xFF]);
	test!(u8: 0x7F => [0x7F]);

	test!(u16: 0x0F_7E => [0x7E, 0x0F]);

	test!(u32: 0x00_2F_87_E7 => [0xE7, 0x87, 0x2F, 0x00]);

	test!(u64: 0xF3_37_CF_8B_DB_03_2B_39 => [0x39, 0x2B, 0x03, 0xDB, 0x8B, 0xCF, 0x37, 0xF3]);

	test!(u128: 0x45_A0_15_6A_36_77_17_8A_83_2E_3C_2C_84_10_58_1A => [
		0x1A, 0x58, 0x10, 0x84, 0x2C, 0x3C, 0x2E, 0x83,
		0x8A, 0x17, 0x77, 0x36, 0x6A, 0x15, 0xA0, 0x45,
	]);

	test!(SizedStr::<0x1>: SizedStr::try_from("A").unwrap() => [0x01, 0x00, 0x41]);

	test!(SizedStr::<0x24>: SizedStr::try_from("l\u{00F8}gma\u{00F0}ur").unwrap() => [
		0x0A, 0x00, 0x6C, 0xC3, 0xB8, 0x67, 0x6D, 0x61,
		0xC3, 0xB0, 0x75, 0x72,
	]);

	test!([char; 0x5]: ['\u{03B4}', '\u{0190}', '\u{03BB}', '\u{03A4}', '\u{03B1}'] => [
		0xB4, 0x03, 0x00, 0x00, 0x90, 0x01, 0x00, 0x00,
		0xBB, 0x03, 0x00, 0x00, 0xA4, 0x03, 0x00, 0x00,
		0xB1, 0x03, 0x00, 0x00,
	]);

	test!(Result::<u16, char>: Ok(0x4545) => [0x00, 0x45, 0x45]);

	test!(Result::<u16, char>: Err(char::REPLACEMENT_CHARACTER) => [0x01, 0xFD, 0xFF, 0x00, 0x00]);

	test!(Option<()>: None     => [0x00]);
	test!(Option<()>: Some(()) => [0x01]);

	test!(Vec<u16>: From::from([0x8000, 0x8000, 0x8000]) => [0x03, 0x00, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80]);

	test!(SystemTime: UNIX_EPOCH => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

	test!(SystemTime: UNIX_EPOCH - Duration::from_secs(0x1) => [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);

	test!(SystemTime: UNIX_EPOCH + Duration::from_secs(0x1) => [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
}

#[test]
fn test_encode_derive() {
	#[derive(Encode, SizedEncode)]
	#[repr(transparent)]
	struct Foo(char);

	#[derive(Encode, SizedEncode)]
	#[repr(u8)]
	enum Bar {
		Unit = 0x45,

		Pretty(bool) = 127,

		Teacher { initials: [char; 0x3] },
	}

	test!(Foo: Foo('\u{FDF2}') => [0xF2, 0xFD, 0x00, 0x00]);

	test!(Bar: Bar::Unit => [0x45]);

	test!(Bar: Bar::Pretty(true) => [0x7F, 0x01]);

	test!(Bar: Bar::Teacher { initials: ['T', 'L', '\0'] } => [
		0x80, 0x54, 0x00, 0x00, 0x00, 0x4C, 0x00, 0x00,
		0x00, 0x00, 0x00, 0x00, 0x00,
	]);
}
