// Copyright 2022-2024 Gabriel Bjørnager Jensen.

use crate::serialise::{SStream, Serialise};

#[test]
fn test_serialise() {
	let mut stream = SStream::new();

	0x00_u8.serialise(&mut stream);
	0xFF_u8.serialise(&mut stream);
	0x7F_u8.serialise(&mut stream);

	0x0F_7E_u16.serialise(&mut stream);

	0x00_2F_87_E7_u32.serialise(&mut stream);

	0xF3_37_CF_8B_DB_03_2B_39_u64.serialise(&mut stream);

	0x45_A0_15_6A_36_77_17_8A_83_2E_3C_2C_84_10_58_1A_u128.serialise(&mut stream);

	['\u{03B4}', '\u{0190}', '\u{03BB}', '\u{03A4}', '\u{03B1}'].serialise(&mut stream);

	Result::<u16, char>::Ok(0x45_45).serialise(&mut stream);
	Result::<u16, char>::Err(char::REPLACEMENT_CHARACTER).serialise(&mut stream);

	Option::<()>::None.serialise(&mut stream);
	Option::<()>::Some(()).serialise(&mut stream);

	let data: Box<[u8]> = stream.into();

	assert_eq!(
		data.as_ref(),
		[
			0x00, 0xFF, 0x7F, 0x0F, 0x7E, 0x00, 0x2F, 0x87,
			0xE7, 0xF3, 0x37, 0xCF, 0x8B, 0xDB, 0x03, 0x2B,
			0x39, 0x45, 0xA0, 0x15, 0x6A, 0x36, 0x77, 0x17,
			0x8A, 0x83, 0x2E, 0x3C, 0x2C, 0x84, 0x10, 0x58,
			0x1A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x05, 0x00, 0x00, 0x03, 0xB4, 0x00, 0x00, 0x01,
			0x90, 0x00, 0x00, 0x03, 0xBB, 0x00, 0x00, 0x03,
			0xA4, 0x00, 0x00, 0x03, 0xB1, 0x00, 0x45, 0x45,
			0x01, 0x00, 0x00, 0xFF, 0xFD, 0x00, 0x01,
		]
	);
}
