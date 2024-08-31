// Copyright 2024 Gabriel Bjørnager Jensen.
//
// This file is part of bzipper.
//test!(you can redistribut => []);
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

#[test]
fn test_serialise() {
	#[derive(Serialise)]
	struct Foo(char);

	#[derive(Serialise)]
	enum Bar {
		Unit,
		Pretty(bool),
		Teacher { initials: [char; 0x3] },
	}

	assert_eq!(Foo::MAX_SERIALISED_SIZE, 0x4);
	assert_eq!(Bar::MAX_SERIALISED_SIZE, 0x10);

	macro_rules! test {
		($ty:ty: $value:expr => $data:expr) => {{
			use ::bzipper::Serialise;

			let mut buf = [0x00; <$ty as Serialise>::MAX_SERIALISED_SIZE];

			let mut stream = Sstream::new(&mut buf);
			<$ty as Serialise>::serialise(&mut $value, &mut stream).unwrap();

			assert_eq!(stream, $data);
		}};
	}

	test!(u8: 0x00 => [0x00]);
	test!(u8: 0xFF => [0xFF]);
	test!(u8: 0x7F => [0x7F]);

	test!(u16: 0x0F_7E => [0x0F, 0x7E]);

	test!(u32: 0x00_2F_87_E7 => [0x00, 0x2F, 0x87, 0xE7]);

	test!(u64: 0xF3_37_CF_8B_DB_03_2B_39 => [0xF3, 0x37, 0xCF, 0x8B, 0xDB, 0x03, 0x2B, 0x39]);

	test!(u128: 0x45_A0_15_6A_36_77_17_8A_83_2E_3C_2C_84_10_58_1A => [
		0x45, 0xA0, 0x15, 0x6A, 0x36, 0x77, 0x17, 0x8A,
		0x83, 0x2E, 0x3C, 0x2C, 0x84, 0x10, 0x58, 0x1A,
	]);

	test!(FixedString::<0x1>: FixedString::try_from("A").unwrap() => [0x00, 0x00, 0x00, 0x01, 0x41]);

	test!(FixedString::<0x24>: FixedString::try_from("l\u{00F8}gma\u{00F0}ur").unwrap() => [
		0x00, 0x00, 0x00, 0x0A, 0x6C, 0xC3, 0xB8, 0x67,
		0x6D, 0x61, 0xC3, 0xB0, 0x75, 0x72,
	]);

	test!([char; 0x5]: ['\u{03B4}', '\u{0190}', '\u{03BB}', '\u{03A4}', '\u{03B1}'] => [
		0x00, 0x00, 0x03, 0xB4, 0x00, 0x00, 0x01, 0x90,
		0x00, 0x00, 0x03, 0xBB, 0x00, 0x00, 0x03, 0xA4,
		0x00, 0x00, 0x03, 0xB1,
	]);

	test!(Result::<u16, char>: Ok(0x45_45)                      => [0x00, 0x45, 0x45]);
	test!(Result::<u16, char>: Err(char::REPLACEMENT_CHARACTER) => [0x01, 0x00, 0x00, 0xFF, 0xFD]);

	test!(Option<()>: None     => [0x00]);
	test!(Option<()>: Some(()) => [0x01]);

	test!(Foo: Foo('\u{FDF2}') => [0x00, 0x00, 0xFD, 0xF2]);

	test!(Bar: Bar::Unit => [0x00, 0x00, 0x00, 0x00]);

	test!(Bar: Bar::Pretty(true) => [0x00, 0x00, 0x00, 0x01, 0x01]);

	test!(Bar: Bar::Teacher { initials: ['T', 'L', '\0'] } => [
		0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x54,
		0x00, 0x00, 0x00, 0x4C, 0x00, 0x00, 0x00, 0x00,
	]);
}