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

use alloc::vec::Vec;
use bzipper::SizedSlice;

#[test]
fn test_sized_slice_from_iter() {
	let f = |x: u32| -> u32 {
		let x = f64::from(x);

		let y = x.sin().powi(0x2) * 1000.0;

		y as u32
	};

	let mut vec = Vec::new();

	for x in 0x0..0x8 {
		vec.push(f(x));
	}

	let vec: SizedSlice<_, 0x10> = vec.into_iter().collect();

	assert_eq!(
		vec,
		[0, 708, 826, 19, 572, 919, 78, 431],
	);
}
