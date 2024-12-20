// Copyright 2024 Gabriel Bjørnager Jensen.
//
// This file is part of oct.
//
// oct is free software: you can redistribute it
// and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// oct is distributed in the hope that it will
// be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Less-
// er General Public License along with oct. If
// not, see <https://www.gnu.org/licenses/>.

#![doc(html_logo_url = "https://gitlab.com/bjoernager/oct/-/raw/master/doc-icon.svg")]

//! oct is a Rust crate for cheaply serialising (encoding) and deserialising (decoding) data structures into binary streams
//!
//! What separates this crate from others such as [Bincode](https://crates.io/crates/bincode/) or [Postcard](https://crates.io/crates/postcard/) is that this crate is extensively optimised for directly translating into binary encodings (whilst the mentioned crates specifically use Serde as a middle layer).
//! The original goal of this project was specifically to guarantee size constraints for encodings on a per-type basis at compile-time.
//! Therefore, this crate may be more suited for networking or other cases where many allocations are unwanted.
//!
//! Keep in mind that this project is still work-in-progress.
//! Until the interfaces are stabilised, different facilities may be replaced, removed, or altered in a breaking way.
//!
//! This crate is compatible with `no_std`.
//!
//! # Performance
//!
//! As oct is optimised exclusively for a single, binary format, it *may* outperform other libraries that are more generic in nature.
//!
//! The `oct-benchmarks` binary compares multiple scenarios using oct and other, similar crates.
//! According to my runs on an AMD Ryzen 7 3700X with default settings, these benchmarks indicate that oct usually outperforms the other tested crates -- as demonstrated in the following table:
//!
//! | Benchmark                          | [Bincode] | [Borsh] | oct     | [Postcard] |
//! | :--------------------------------- | --------: | ------: | ------: | ---------: |
//! | `encode_u8`                        |     0.968 |   0.857 |   0.733 |      0.979 |
//! | `encode_u32`                       |     1.065 |   0.999 |   0.730 |      2.727 |
//! | `encode_u128`                      |     2.168 |   2.173 |   1.510 |      6.246 |
//! | `encode_struct_unit`               |     0.000 |   0.000 |   0.000 |      0.000 |
//! | `encode_struct_unnamed`            |     1.241 |   1.173 |   0.823 |      3.350 |
//! | `encode_struct_named`              |     3.079 |   1.507 |   0.973 |      3.082 |
//! | `encode_enum_unit`                 |     0.246 |   0.297 |   0.000 |      0.295 |
//! | `decode_u8`                        |     0.942 |   0.962 |   0.922 |      0.923 |
//! | `decode_non_zero_u8`               |     1.126 |   1.159 |   1.127 |      1.160 |
//! | `decode_bool`                      |     1.040 |   1.099 |   1.055 |      1.177 |
//! | **Total time** &#8594;             |    11.873 |  10.225 |   7.873 |     18.939 |
//! | **Total deviation (p.c.)** &#8594; |       +51 |     +30 |      ±0 |       +141 |
//!
//! [Bincode]: https://crates.io/crates/bincode/
//! [Borsh]: https://crates.io/crates/borsh/
//! [Postcard]: https://crates.io/crates/postcard/
//!
//! All quantities are measured in seconds unless otherwise noted.
//!
//! Currently, oct's weakest point seems to be decoding.
//! Please note that I myself find large (relatively speaking) inconsistencies between runs in these last two benchmarks.
//! Do feel free to conduct your own tests of oct.
//!
//! # Data model
//!
//! Most primitives encode losslessly, with the main exceptions being [`usize`] and [`isize`].
//! These are instead first cast as [`u16`] and [`i16`], respectively, due to portability concerns (with respect to embedded systems).
//!
//! Numerical primitives in general encode as little endian (and **not** ["network order"](https://en.wikipedia.org/wiki/Endianness#Networking)).
//! It is recommended for implementors to follow this convention as well.
//!
//! See specific types' implementations for notes on their data models.
//!
//! **Note that the data model is currently not stabilised,** and may not necessarily be in the near future (at least before [specialisation](https://github.com/rust-lang/rust/issues/31844/)).
//! It may therefore be undesired to store encodings long-term.
//!
//! # Usage
//!
//! This crate revolves around the [`Encode`](encode::Encode) and [`Decode`](decode::Decode) traits, both of which handle conversions to and from byte streams.
//!
//! Many standard types come implemented with oct, including most primitives as well as some standard library types such as [`Option`] and [`Result`].
//! Some [features](#feature-flags) enable an extended set of implementations.
//!
//! It is recommended in most cases to simply derive these two traits for user-defined types (although this is only supported with enumerations and structures -- not untagged unions).
//! Here, each field is *chained* according to declaration order:
//!
//! ```
//! use oct::Slot;
//! use oct::decode::Decode;
//! use oct::encode::Encode;
//!
//! #[derive(Debug, Decode, Encode, PartialEq)]
//! struct Ints {
//!     value0: u8,
//!     value1: u16,
//!     value2: u32,
//!     value3: u64,
//!     value4: u128,
//! }
//!
//! const VALUE: Ints = Ints {
//!     value0: 0x00,
//!     value1: 0x02_01,
//!     value2: 0x06_05_04_03,
//!     value3: 0x0E_0D_0C_0B_0A_09_08_07,
//!     value4: 0x1E_1D_1C_1B_1A_19_18_17_16_15_14_13_12_11_10_0F,
//! };
//!
//! let mut buf = Slot::with_capacity(0x100);
//!
//! buf.write(VALUE).unwrap();
//!
//! assert_eq!(buf.len(), 0x1F);
//!
//! assert_eq!(
//!     buf,
//!     [
//!         0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
//!         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
//!         0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
//!         0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E
//!     ].as_slice(),
//! );
//!
//! assert_eq!(buf.read().unwrap(), VALUE);
//! ```
//!
//! ## Buffer types
//!
//! The [`Encode`](encode::Encode) and [`Decode`](decode::Decode) traits both rely on streams for carrying the manipulated bytes.
//!
//! These streams are separated into two type: [*output streams*](encode::Output) and [*input streams*](decode::Input).
//! The [`Slot`] type can be used to handle these streams.
//!
//! ## Encoding
//!
//! To encode an object directly using the `Encode` trait, simply allocate a buffer for the encoding and wrap it in an `Output` object:
//!
//! ```
//! use oct::encode::{Encode, Output, SizedEncode};
//!
//! let mut buf = [0x00; char::MAX_ENCODED_SIZE];
//! let mut stream = Output::new(&mut buf);
//!
//! 'Ж'.encode(&mut stream).unwrap();
//!
//! assert_eq!(buf, [0x16, 0x04, 0x00, 0x00].as_slice());
//! ```
//!
//! Streams can also be used to chain multiple objects together:
//!
//! ```
//! use oct::encode::{Encode, Output, SizedEncode};
//!
//! let mut buf = [0x0; char::MAX_ENCODED_SIZE * 0x5];
//! let mut stream = Output::new(&mut buf);
//!
//! // Note: For serialising multiple characters, the
//! // `String` and `SizedStr` types are usually
//! // preferred.
//!
//! 'ل'.encode(&mut stream).unwrap();
//! 'ا'.encode(&mut stream).unwrap();
//! 'م'.encode(&mut stream).unwrap();
//! 'د'.encode(&mut stream).unwrap();
//! 'ا'.encode(&mut stream).unwrap();
//!
//! assert_eq!(buf, [
//!     0x44, 0x06, 0x00, 0x00, 0x27, 0x06, 0x00, 0x00,
//!     0x45, 0x06, 0x00, 0x00, 0x2F, 0x06, 0x00, 0x00,
//!     0x27, 0x06, 0x00, 0x00
//! ]);
//! ```
//!
//! If the encoded type additionally implements [`SizedEncode`](encode::SizedEncode), then the maximum size of any encoding is guaranteed with the [`MAX_ENCODED_SIZE`](encode::SizedEncode::MAX_ENCODED_SIZE) constant.
//!
//! ## Decoding
//!
//! Decoding works with a similar syntax to encoding.
//! To decode a byte array, simply call the [`decode`](decode::Decode::decode) method with an [`Input`](decode::Input) object:
//!
//! ```
//! use oct::decode::{Decode, Input};
//!
//! let data = [0x54, 0x45];
//! let mut stream = Input::new(&data);
//!
//! assert_eq!(u16::decode(&mut stream).unwrap(), 0x4554);
//!
//! // Data can theoretically be reinterpretred:
//!
//! stream = Input::new(&data);
//!
//! assert_eq!(u8::decode(&mut stream).unwrap(), 0x54);
//! assert_eq!(u8::decode(&mut stream).unwrap(), 0x45);
//!
//! // Including as tuples:
//!
//! stream = Input::new(&data);
//!
//! assert_eq!(<(u8, u8)>::decode(&mut stream).unwrap(), (0x54, 0x45));
//! ```
//!
//! # Examples
//!
//! A UDP server/client for geographic data:
//!
//! ```
//! use oct::Slot;
//! use oct::decode::Decode;
//! use oct::encode::{Encode, SizedEncode};
//! use std::io;
//! use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
//! use std::thread::spawn;
//!
//! // City, region, etc.:
//! #[derive(Clone, Copy, Debug, Decode, Encode, Eq, PartialEq, SizedEncode)]
//! enum Area {
//!     AlQuds,
//!     Byzantion,
//!     Cusco,
//!     Tenochtitlan,
//!     // ...
//! }
//!
//! // Client-to-server message:
//! #[derive(Debug, Decode, Encode, PartialEq, SizedEncode)]
//! enum Request {
//!     AtmosphericHumidity { area: Area },
//!     AtmosphericPressure { area: Area },
//!     AtmosphericTemperature { area: Area },
//!     // ...
//! }
//!
//! // Server-to-client message:
//! #[derive(Debug, Decode, Encode, PartialEq, SizedEncode)]
//! enum Response {
//!     AtmosphericHumidity(f64),
//!     AtmosphericPressure(f64), // Pascal
//!     AtmosphericTemperature(f64), // Kelvin
//!     // ...
//! }
//!
//! struct Party {
//!     pub socket: UdpSocket,
//!
//!     pub request_buf:  Slot::<Request>,
//!     pub response_buf: Slot::<Response>,
//! }
//!
//! impl Party {
//!     pub fn new<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
//!         let socket = UdpSocket::bind(addr)?;
//!
//!         let this = Self {
//!             socket,
//!
//!             request_buf:  Slot::new(),
//!             response_buf: Slot::new(),
//!         };
//!
//!         Ok(this)
//!     }
//! }
//!
//! let mut server = Party::new("127.0.0.1:27015").unwrap();
//!
//! let mut client = Party::new("0.0.0.0:0").unwrap();
//!
//! spawn(move || {
//!     let Party { socket, mut request_buf, mut response_buf } = server;
//!
//!     // Recieve initial request from client.
//!
//!     let (len, addr) = socket.recv_from(&mut request_buf).unwrap();
//!     request_buf.set_len(len);
//!
//!     let request = request_buf.read().unwrap();
//!     assert_eq!(request, Request::AtmosphericTemperature { area: Area::AlQuds });
//!
//!     // Handle request and respond back to client.
//!
//!     let response = Response::AtmosphericTemperature(44.4); // For demonstration's sake.
//!
//!     response_buf.write(response).unwrap();
//!     socket.send_to(&response_buf, addr).unwrap();
//! });
//!
//! spawn(move || {
//!     let Party { socket, mut request_buf, mut response_buf } = client;
//!
//!     // Send initial request to server.
//!
//!     socket.connect("127.0.0.1:27015").unwrap();
//!
//!     let request = Request::AtmosphericTemperature { area: Area::AlQuds };
//!
//!     request_buf.write(request);
//!     socket.send(&request_buf).unwrap();
//!
//!     // Recieve final response from server.
//!
//!     socket.recv(&mut response_buf).unwrap();
//!
//!     let response = response_buf.read().unwrap();
//!     assert_eq!(response, Response::AtmosphericTemperature(44.4));
//! });
//! ```
//!
//! # Feature flags
//!
//! oct defines the following, default features:
//!
//! * `alloc`: Enables the [`Slot`] type and implementations for e.g. [`Box`](alloc::boxed::Box) and [`Arc`](alloc::sync::Arc)
//! * `proc-macro`: Pulls the procedural macros from the [`oct-macros`](https://crates.io/crates/oct-macros/) crate
//! * `std`: Enables implementations for types such as [`Mutex`](std::sync::Mutex) and [`RwLock`](std::sync::RwLock)
//!
//! # Documentation
//!
//! oct has its documentation written in-source for use by `rustdoc`.
//! See [Docs.rs](https://docs.rs/oct/latest/oct/) for an on-line, rendered instance.
//!
//! Currently, these docs make use of some unstable features for the sake of readability.
//! The nightly toolchain is therefore required when rendering them.
//!
//! # Contribution
//!
//! oct does not accept source code contributions at the moment.
//! This is a personal choice by the maintainer and may be undone in the future.
//!
//! Do however feel free to open up an issue on [GitLab](https://gitlab.com/bjoernager/oct/issues/) or (preferably) [GitHub](https://github.com/bjoernager/oct/issues/) if you feel the need to express any concerns over the project.
//!
//! # Copyright & Licence
//!
//! Copyright 2024 Gabriel Bjørnager Jensen.
//!
//! This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
//! See the GNU Lesser General Public License for more details.
//!
//! You should have received a copy of the GNU Lesser General Public License along with this program.
//! If not, see <https://www.gnu.org/licenses/>.

#![no_std]

#![warn(missing_docs)]
#![cfg_attr(doc, allow(internal_features))]

#![cfg_attr(doc, feature(doc_cfg, rustdoc_internals))]

// For use in macros:
extern crate self as oct;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

macro_rules! use_mod {
	($vis:vis $name:ident$(,)?) => {
		mod $name;
		$vis use $name::*;
	};
}
pub(crate) use use_mod;

use_mod!(pub primitive_discriminant);
use_mod!(pub sized_iter);
use_mod!(pub sized_slice);
use_mod!(pub sized_str);

#[cfg(feature = "alloc")]
use_mod!(pub slot);

pub mod decode;
pub mod encode;
pub mod error;
