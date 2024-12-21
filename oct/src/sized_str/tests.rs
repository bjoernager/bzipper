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
use oct::error::{StringError, Utf8Error};
use std::cmp::Ordering;

#[test]
fn test_fixed_str_from_iter() {
	let s: SizedStr::<0x4> = "hello world".chars().collect();
	assert_eq!(s, "hell")
}

#[test]
fn test_fixed_str_size() {
	let string0 = SizedStr::<0x0C>::try_from("Hello there!").unwrap();
	let string1 = SizedStr::<0x12>::try_from("MEIN_GRO\u{1E9E}_GOTT").unwrap();
	let string2 = SizedStr::<0x05>::try_from("Hello").unwrap();

	assert_eq!(string0.partial_cmp(&string0), Some(Ordering::Equal));
	assert_eq!(string0.partial_cmp(&string1), Some(Ordering::Less));
	assert_eq!(string0.partial_cmp(&string2), Some(Ordering::Greater));

	assert_eq!(string1.partial_cmp(&string0), Some(Ordering::Greater));
	assert_eq!(string1.partial_cmp(&string1), Some(Ordering::Equal));
	assert_eq!(string1.partial_cmp(&string2), Some(Ordering::Greater));

	assert_eq!(string2.partial_cmp(&string0), Some(Ordering::Less));
	assert_eq!(string2.partial_cmp(&string1), Some(Ordering::Less));
	assert_eq!(string2.partial_cmp(&string2), Some(Ordering::Equal));

	assert_eq!(string0, "Hello there!");
	assert_eq!(string1, "MEIN_GRO\u{1E9E}_GOTT");
	assert_eq!(string2, "Hello");
}

#[test]
fn test_fixed_str_from_utf8() {
	macro_rules! test_utf8 {
		{
			len: $len:literal,
			utf8: $utf8:expr,
			result: $result:pat$(,)?
		 } => {{
			let utf8: &[u8] = $utf8.as_ref();

			assert!(matches!(
				SizedStr::<$len>::from_utf8(utf8),
				$result,
			));
		}};
	}

	test_utf8!(
		len:    0x3,
		utf8:   b"A\xF7c",
		result: Err(StringError::BadUtf8(Utf8Error { value: 0xF7, index: 0x1 })),
	);

	test_utf8!(
		len:    0x4,
		utf8:   "A\u{00F7}c",
		result: Ok(..),
	);

	test_utf8!(
		len:    0x4,
		utf8:   b"20\x20\xAC",
		result: Err(StringError::BadUtf8(Utf8Error { value: 0xAC, index: 0x3 })),
	);

	test_utf8!(
		len:    0x5,
		utf8:   "20\u{20AC}",
		result: Ok(..),
	);
}
