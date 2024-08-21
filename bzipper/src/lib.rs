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
//! Contrary to [Serde](https://crates.io/crates/serde/)/[Bincode](https://crates.io/crates/bincode/), the goal of bzipper is to serialise with a known size constraint.
//! Therefore, this crate may be more suited for networking or other cases where a fixed-sized buffer is needed.
//!
//! Keep in mind that this project is still work-in-progress.
//!
//! This crate is compatible with `no_std`.
//!
//! # Data model
//!
//! Most primitive types serialise losslessly, with the exception being [`usize`] and [`isize`].
//! These serialise as [`u32`] and [`i32`], respectively, for portability reasons.
//!
//! Unsized types, such as [`str`] and [slices](slice), are not supported.
//! Instead, [arrays](array) should be used.
//! For strings, the [`FixedString`] type is also provided.
//!
//! # Usage
//!
//! This crate revolves around the [`Serialise`] and [`Deserialise`] traits, both of which are commonly used in conjunction with streams (more specifically, [s-streams](Sstream) and [d-streams](Dstream)).
//!
//! Many core types come implemented with bzipper, including primitives as well as some standard library types such as [`Option`] and [`Result`](core::result::Result).
//!
//! It is recommended in most cases to just derive these traits for custom types (enumerations and structures only).
//! Here, each field is chained in declaration order:
//!
//! ```
//! use bzipper::{Deserialise, Serialise};
//!
//! #[derive(Debug, Deserialise, PartialEq, Serialise)]
//! struct IoRegister {
//!     addr:  u32,
//!     value: u16,
//! }
//!
//! let mut buf: [u8; IoRegister::SERIALISED_SIZE] = Default::default();
//! IoRegister { addr: 0x04000000, value: 0x0402 }.serialise(&mut buf).unwrap();
//!
//! assert_eq!(buf, [0x04, 0x00, 0x00, 0x00, 0x04, 0x02]);
//!
//! assert_eq!(IoRegister::deserialise(&buf).unwrap(), IoRegister { addr: 0x04000000, value: 0x0402 });
//! ```
//!
//! ## Serialisation
//!
//! To serialise an object implementing `Serialise`, simply allocate a buffer for the serialisation.
//! The required size of any given serialisation is specified by the [`SERIALISED_SIZE`](Serialise::SERIALISED_SIZE) constant:
//!
//! ```
//! use bzipper::Serialise;
//!
//! let mut buf: [u8; char::SERIALISED_SIZE] = Default::default();
//! 'Ж'.serialise(&mut buf).unwrap();
//!
//! assert_eq!(buf, [0x00, 0x00, 0x04, 0x16]);
//! ```
//!
//! The only special requirement of the [`serialise`](Serialise::serialise) method is that the provided byte slice has an element count of exactly `SERIALISED_SIZE`.
//!
//! We can also use streams to *chain* multiple elements together.
//!
//! ```
//! use bzipper::Serialise;
//!
//! let mut buf: [u8; char::SERIALISED_SIZE * 5] = Default::default();
//! let mut stream = bzipper::Sstream::new(&mut buf);
//!
//! stream.append(&'ل');
//! stream.append(&'ا');
//! stream.append(&'م');
//! stream.append(&'د');
//! stream.append(&'ا');
//!
//! assert_eq!(buf, [0x00, 0x00, 0x06, 0x44, 0x00, 0x00, 0x06, 0x27, 0x00, 0x00, 0x06, 0x45, 0x00, 0x00, 0x06, 0x2F, 0x00, 0x00, 0x06, 0x27]);
//! ```
//!
//! When serialising primitives, the resulting byte stream is in big endian (a.k.a. network endian).
//! It is recommended for implementors to adhere to this convention as well.
//!
//! ## Deserialisation
//!
//! Deserialisation works with an almost identical syntax to serialisation.
//!
//! To deserialise a buffer, simply call the [`deserialise`](Deserialise::deserialise) method:
//!
//! ```
//! use bzipper::Deserialise;
//!
//! let data = [0x45, 0x54];
//! assert_eq!(<u16>::deserialise(&data).unwrap(), 0x4554);
//! ```
//!
//! Just like with serialisations, the [`Dstream`] can be used to deserialise chained elements:
//!
//! ```
//! use bzipper::Deserialise;
//!
//! let data = [0x45, 0x54];
//! let stream = bzipper::Dstream::new(&data);
//!
//! assert_eq!(stream.take::<u8>().unwrap(), 0x45);
//! assert_eq!(stream.take::<u8>().unwrap(), 0x54);
//! ```

#![no_std]

#![cfg_attr(doc, feature(doc_cfg))]

extern crate self as bzipper;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
extern crate std;

/// Implements [`Deserialise`] for the provided type.
///
/// This macro assumes that `Serialise` was also derived, although this is not strictly required as it is unenforceable.
#[doc(inline)]
pub use bzipper_macros::Deserialise;

/// Implements [`Serialise`] for the provided type.
///
/// # Structs
///
/// For structures, each element is chained in **order of declaration.**
/// For example, the following struct will serialise its field `foo` before `bar`:
///
/// ```
/// use bzipper::Serialise;
///
/// #[derive(Serialise)]
/// pub struct FooBar {
///     pub foo: char,
///     pub bar: char,
/// }
/// ```
///
/// Should the order of declaration change, then most of---if not all---previous dervied serialisations become void.
///
/// The value of [`SERIALISED_SIZE`](Serialise::SERIALISED_SIZE) is set to the combined value of all fields.
///
/// If the structure is a unit structure (i.e. it has *no* fields), it is serialised equivalently to the [unit] type.
///
/// # Enums
///
/// Enumerations are serialised by first assigning each variant its own discriminant.
/// By default, each discriminant is assigned from the range 0 to infinite, to the extend allowed by the `u32` type (as which the discriminant is encoded).
/// In the future, however, custom representations and assigned discriminants will be honoured.
///
/// Variants with fields are serialised exactly like structures.
/// That is, each field is chained in order of declaration.
///
/// Each variant has its own serialised size, and the largest of these values is chosen as the serialised size of the enumeration type.
///
/// # Unions
///
/// Unions cannot derive `Serialise` due to the uncertainty of their contents.
/// The trait should therefore be implemented manually.
#[doc(inline)]
pub use bzipper_macros::Serialise;

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
use_mod!(pub fixed_iter);
use_mod!(pub fixed_string);
use_mod!(pub serialise);
use_mod!(pub sstream);
