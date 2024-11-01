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

use bzipper::SizedSlice;

#[test]
fn test_sized_iter_double_ended() {
	let data = SizedSlice::from([
		'H', 'E', 'L', 'L', 'O', ' ', 'W', 'O',
		'R', 'L', 'D',
	]);

	let mut data = data.into_iter();

	assert_eq!(data.next(),      Some('H'));
	assert_eq!(data.next_back(), Some('D'));
	assert_eq!(data.next(),      Some('E'));
	assert_eq!(data.next_back(), Some('L'));
	assert_eq!(data.next(),      Some('L'));
	assert_eq!(data.next_back(), Some('R'));
	assert_eq!(data.next(),      Some('L'));
	assert_eq!(data.next_back(), Some('O'));
	assert_eq!(data.next(),      Some('O'));
	assert_eq!(data.next_back(), Some('W'));
	assert_eq!(data.next(),      Some(' '));
}
