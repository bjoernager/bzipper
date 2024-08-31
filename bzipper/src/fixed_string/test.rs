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

use crate::FixedString;

use core::cmp::Ordering;

#[test]
fn test_fixed_string() {
	let str0 = FixedString::<0x0C>::try_from("Hello there!").unwrap();
	let str1 = FixedString::<0x12>::try_from("MEIN_GRO\u{1E9E}_GOTT").unwrap();
	let str2 = FixedString::<0x05>::try_from("Hello").unwrap();

	assert_eq!(str0.partial_cmp(&str0), Some(Ordering::Equal));
	assert_eq!(str0.partial_cmp(&str1), Some(Ordering::Less));
	assert_eq!(str0.partial_cmp(&str2), Some(Ordering::Greater));

	assert_eq!(str1.partial_cmp(&str0), Some(Ordering::Greater));
	assert_eq!(str1.partial_cmp(&str1), Some(Ordering::Equal));
	assert_eq!(str1.partial_cmp(&str2), Some(Ordering::Greater));

	assert_eq!(str2.partial_cmp(&str0), Some(Ordering::Less));
	assert_eq!(str2.partial_cmp(&str1), Some(Ordering::Less));
	assert_eq!(str2.partial_cmp(&str2), Some(Ordering::Equal));

	assert_eq!(str0, "Hello there!");
	assert_eq!(str1, "MEIN_GRO\u{1E9E}_GOTT");
	assert_eq!(str2, "Hello");
}
