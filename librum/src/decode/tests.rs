// Copyright 2024 Gabriel Bjørnager Jensen.
//
// This file is part of Librum.
//
// Librum is free software: you can redistribute it
// and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// Librum is distributed in the hope that it will
// be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Less-
// er General Public License along with Librum. If
// not, see <https://www.gnu.org/licenses/>.

use librum::{Decode, Encode, IStream, SizedEncode};
use std::char;
use std::vec::Vec;
use std::string::String;

macro_rules! test {
	($ty:ty: $data:expr => $value:expr) => {{
		let mut stream = IStream::new(&$data);

		let left  = <$ty as Decode>::decode(&mut stream).unwrap();
		let right = $value;

		assert_eq!(left, right);
	}};
}

#[test]
fn test_decode() {
	test!(i8: [0x00] =>  0x00);
	test!(i8: [0x7F] =>  0x7F);
	test!(i8: [0x80] => -0x80);
	test!(i8: [0xFF] => -0x01);

	test!(i16: [0x00, 0x00] =>  0x0000);
	test!(i16: [0xFF, 0x7F] =>  0x7FFF);
	test!(i16: [0x00, 0x80] => -0x8000);
	test!(i16: [0xFF, 0xFF] => -0x0001);

	test!(i32: [0x00, 0x00, 0x00, 0x00] =>  0x00000000);
	test!(i32: [0xFF, 0xFF, 0xFF, 0x7F] =>  0x7FFFFFFF);
	test!(i32: [0x00, 0x00, 0x00, 0x80] => -0x80000000);
	test!(i32: [0xFF, 0xFF, 0xFF, 0xFF] => -0x00000001);

	test!(i64: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] =>  0x0000000000000000);
	test!(i64: [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F] =>  0x7FFFFFFFFFFFFFFF);
	test!(i64: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80] => -0x8000000000000000);
	test!(i64: [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF] => -0x0000000000000001);

	test!(u128: [
		0x7F, 0x8F, 0x6F, 0x9F, 0x5F, 0xAF, 0x4F, 0xBF,
		0x3F, 0xCF, 0x2F, 0xDF, 0x1F, 0xEF, 0x0F, 0xFF,
	] => 0xFF_0F_EF_1F_DF_2F_CF_3F_BF_4F_AF_5F_9F_6F_8F_7F);

	test!(char: [0xFD, 0xFF, 0x00, 0x00] => char::REPLACEMENT_CHARACTER);

	test!([char; 0x5]: [
		0xBB, 0x03, 0x00, 0x00, 0x91, 0x03, 0x00, 0x00,
		0xBC, 0x03, 0x00, 0x00, 0x94, 0x03, 0x00, 0x00,
		0xB1, 0x03, 0x00, 0x00
	] => ['\u{03BB}', '\u{0391}', '\u{03BC}', '\u{0394}', '\u{03B1}']);

	test!(Option<()>: [0x00] => None);
	test!(Option<()>: [0x01] => Some(()));

	test!(Result<(), i8>: [0x00, 0x00] => Ok(()));
	test!(Result<(), i8>: [0x01, 0x7F] => Err(i8::MAX));

	test!(Vec<u16>: [0x02, 0x00, 0xBB, 0xAA, 0xDD, 0xCC] => [0xAA_BB, 0xCC_DD].as_slice());

	test!(String: [0x06, 0x00, 0xE6, 0x97, 0xA5, 0xE6, 0x9C, 0xAC] => "\u{65E5}\u{672C}");
}

#[test]
fn test_decode_derive() {
	#[derive(Debug, Decode, Encode, PartialEq, SizedEncode)]
	struct ProcExit {
		exit_code: i32,
		timestmap: u64,
	}

	#[derive(Debug, Decode, Encode, PartialEq, SizedEncode)]
	struct NewByte(u8);

	#[derive(Debug, Decode, Encode, PartialEq, SizedEncode)]
	struct Unit;

	#[derive(Debug, Decode, Encode, PartialEq, SizedEncode)]
	enum UnitOrFields {
		Unit,
		Unnamed(i32),
		Named { timestamp: u64 },
	}
	test!(ProcExit: [
		0x01, 0x00, 0x00, 0x00, 0x00, 0xE1, 0x0B, 0x5E,
		0x00, 0x00, 0x00, 0x00,
	] => ProcExit { exit_code: 0x1, timestmap: 1577836800 });

	test!(NewByte: [0x80] => NewByte(0x80));

	test!(Unit: [] => Unit);

	test!(UnitOrFields: [
		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		0x00, 0x00,
	] => UnitOrFields::Unit);

	test!(UnitOrFields: [
		0x01, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00,
		0x00, 0x00,
	] => UnitOrFields::Unnamed(-0x1));

	test!(UnitOrFields: [
		0x02, 0x00, 0x4C, 0xC8, 0xC5, 0x66, 0x00, 0x00,
		0x00, 0x00,
	] => UnitOrFields::Named { timestamp: 1724237900 });
}
