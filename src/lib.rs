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

//! Binary (de)serialisation.
//!
//! Contrary to [Serde](https://crates.io/crates/serde/)/[Bincode](https://crates.io/crates/bincode/), the goal of `bzipper` is to serialise with a known size constraint.
//! Therefore, this crate may be more suited for networking or other cases where a fixed-sized buffer is needed.
//!
//! Keep in mind that this project is still work-in-progress.
//!
//! This crate does not require any dependencies at the moment.

macro_rules! use_mod {
	($vis:vis $name:ident) => {
		mod $name;
		$vis use $name::*;
	};
}
pub(in crate) use use_mod;

use_mod!(pub deserialise);
use_mod!(pub dstream);
use_mod!(pub error);
use_mod!(pub fixed_string);
use_mod!(pub fixed_string_iter);
use_mod!(pub serialise);
use_mod!(pub sstream);
