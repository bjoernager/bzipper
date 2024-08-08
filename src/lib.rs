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

#![doc(html_logo_url = "https://gitlab.com/bjoernager/bzipper/-/raw/master/doc-icon.svg?ref_type=heads")]

//! Binary (de)serialisation.
//!
//! Contrary to [Serde](https://crates.io/crates/serde/)/[Bincode](https://crates.io/crates/bincode/), the goal of `bzipper` is to serialise with a known size constraint.
//! Therefore, this crate may be more suited for networking or other cases where a fixed-sized buffer is needed.
//!
//! Keep in mind that this project is still work-in-progress.
//!
//! This crate does not require any dependencies at the moment.
//! It is also compatible with `no_std`.
//!
//! # Data model
//!
//! Most primitive types serialise losslessly, with the exception being [`usize`] and [`isize`].
//! These serialise as [`u16`] and [`i16`], respectively, for portability reasons.
//!
//! Unsized types, such as [`str`] and [slices](slice), are not supported.
//! Instead, [arrays](array) should be used.
//! For strings, the [`FixedString`] type is also provided.
//!
//! # Usage
//!
//! This crate revolves around the [`Serialise`] and [`Deserialise`] traits, both of which work around streams (more specifically, [d-streams](Dstream) and [s-streams](Sstream)).
//!
//! Many core types come implemented with `bzipper`, including primitives as well as some standard library types such as [`Option`] and [`Result`](core::result::Result).
//!
//! ## Serialisation
//!
//! To serialise an object implementing `Serialise`, simply allocate a so-called "s-stream" (short for *serialisation stream*) with the [`Sstream`] type:
//!
//! ```
//! let mut buf: [u8; 16] = Default::default();
//!
//! let mut stream = bzipper::Sstream::new(&mut buf);
//! ```
//!
//! The resulting stream is immutable in the sense that it cannot grow its buffer, altough it does keep track of the buffer's state.
//!
//! A byte sequence can be added to our new stream by passing the stream to a call to the [`serialise`](Serialise::serialise) method:
//!
//! ```
//! use bzipper::Serialise;
//!
//! let mut buf: [u8; 2] = Default::default();
//! let mut stream = bzipper::Sstream::new(&mut buf);
//!
//! 0x4554_u16.serialise(&mut stream).unwrap();
//! ```
//!
//! The ammount of bytes used by the serialiser (that is, the ammount of bytes written to the stream) is indicated by its return value (i.e. it has the type `Result<usize, Serialise::Error>`).
//!
//! Whilst the *maximum* ammount of bytes is specified by the [`SERIALISE_LIMIT`](Serialise::SERIALISE_LIMIT) constant, this can in cases be lower (for example with [`None`] variants which are always encoded as a single, null byte).
//!
//! When serialising primitives, the resulting byte stream is in big endian (a.k.a. network endian).
//! It is recommended for implementors to adhere to this convention as well.
//!
//! After serialisation, the s-stream records the new write-to position of the buffer. This allows for *chaining* of serialisations, which can prove useful when implementing the trait for custom types.
//!
//! ## Deserialisation
//!
//! As with serialisation, deserialisation uses streams (just with the [`Dstream`] type; short for *deserialisation stream*):
//!
//! ```
//! let data = [0x45, 0x54];
//!
//! let mut stream = bzipper::Dstream::new(&data);
//! ```
//!
//! Using these streams is also just as simple as with s-streams:
//!
//! ```
//! use bzipper::Deserialise;
//!
//! let data = [0x45, 0x54];
//! let mut stream = bzipper::Dstream::new(&data);
//!
//! assert_eq!(u16::deserialise(&mut stream).unwrap(), 0x4554);
//! ```
//!
//! When chaining serialisations, keep in mind that appropriate deserialisations should come in **reverse order** (streams function similarly to stacks in this sense).

#![no_std]

extern crate alloc;

macro_rules! use_mod {
	($vis:vis $name:ident) => {
		mod $name;
		$vis use $name::*;
	};
}
pub(in crate) use use_mod;

use_mod!(pub buffer);
use_mod!(pub deserialise);
use_mod!(pub dstream);
use_mod!(pub error);
use_mod!(pub fixed_string);
use_mod!(pub fixed_string_iter);
use_mod!(pub serialise);
use_mod!(pub sstream);
