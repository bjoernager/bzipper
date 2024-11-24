# Librum

Librum is a Rust crate for cheaply serialising (encoding) and deserialising (decoding) data structures into binary streams

What separates this crate from others such as [Bincode](https://crates.io/crates/bincode/) or [Postcard](https://crates.io/crates/postcard/) is that this crate is extensively optimised for *just* binary encodings (whilst the mentioned crates specifically use Serde and build on a more abstract data model).
The original goal of this project was specifically to guarantee size constraints for encodings on a per-type basis at compile-time.
Therefore, this crate may be more suited for networking or other cases where many allocations are unwanted.

Keep in mind that this project is still work-in-progress.
Until the interfaces are stabilised, different facilities may be replaced, removed, or altered in a breaking way.

This crate is compatible with `no_std`.

## Performance

As Librum is optimised exclusively for a single, binary format, it may outperform other libraries that are more generic in nature.

The `librum-benchmarks` binary compares multiple scenarios using Librum and other, similar crates.
According to my runs on an AMD Ryzen 7 3700X, these benchmarks indicate that Librum outperform all of the tested crates -- as demonstrated in the following table:

| Benchmark                          | [Bincode] | [Borsh] | Librum | [Postcard] |
| :--------------------------------- | --------: | ------: | ------: | ---------: |
| `encode_u8`                        |     1.306 |   1.315 |   1.150 |      1.304 |
| `encode_u32`                       |     1.321 |   1.317 |   1.146 |      3.016 |
| `encode_u128`                      |     2.198 |   2.103 |   1.509 |      6.376 |
| `encode_struct_unit`               |     0.000 |   0.000 |   0.000 |      0.000 |
| `encode_struct_unnamed`            |     1.362 |   1.448 |   1.227 |      2.659 |
| `encode_struct_named`              |     3.114 |   1.530 |   0.969 |      3.036 |
| `encode_enum_unit`                 |     0.252 |   0.297 |   0.000 |      0.299 |
| **Total time** &#8594;             |     9.553 |   8.010 |   6.001 |     16.691 |
| **Total deviation (p.c.)** &#8594; |       +59 |     +33 |      ±0 |       +178 |

[Bincode]: https://crates.io/crates/bincode/
[Borsh]: https://crates.io/crates/borsh/
[Postcard]: https://crates.io/crates/postcard/

All quantities are measured in seconds unless otherwise noted.
Please feel free to conduct your own tests of Librum.

## Data model

Most primitives encode losslessly, with the main exceptions being [`usize`] and [`isize`].
These are instead first cast as [`u16`] and [`i16`], respectively, due to portability concerns (with respect to embedded systems).

See specific types' implementations for notes on their data models.

**Note that the data model is currently not stabilised,** and may not necessarily be in the near future (at least before [specialisation](https://github.com/rust-lang/rust/issues/31844/)).
It may therefore be undesired to store encodings long-term.

## Usage

This crate revolves around the [`Encode`] and [`Decode`] traits which both handle conversions to and from byte streams.

Many standard types come implemented with Librum, including most primitives as well as some standard library types such as [`Option`] and [`Result`].
Some [features](#feature-flags) enable an extended set of implementations.

It is recommended in most cases to simply derive these two traits for custom types (although this is only supported with enumerations and structures -- not untagged unions).
Here, each field is *chained* according to declaration order:

```
use librum::{Buf, Decode, Encode};

##[derive(Debug, Decode, Encode, PartialEq)]
struct Ints {
    value0: u8,
    value1: u16,
    value2: u32,
    value3: u64,
    value4: u128,
}

const VALUE: Ints = Ints {
    value0: 0x00,
    value1: 0x02_01,
    value2: 0x06_05_04_03,
    value3: 0x0E_0D_0C_0B_0A_09_08_07,
    value4: 0x1E_1D_1C_1B_1A_19_18_17_16_15_14_13_12_11_10_0F,
};

let mut buf = Buf::with_capacity(0x100);

buf.write(VALUE).unwrap();

assert_eq!(buf.len(), 0x1F);

assert_eq!(
    buf,
    [
        0x00, 0x02, 0x01, 0x06, 0x05, 0x04, 0x03, 0x0E,
        0x0D, 0x0C, 0x0B, 0x0A, 0x09, 0x08, 0x07, 0x1E,
        0x1D, 0x1C, 0x1B, 0x1A, 0x19, 0x18, 0x17, 0x16,
        0x15, 0x14, 0x13, 0x12, 0x11, 0x10, 0x0F,
    ].as_slice(),
);

assert_eq!(buf.read().unwrap(), VALUE);
```

### Buffer types

The [`Encode`] and [`Decode`] traits both rely on streams for carrying the manipulated bytes.

These streams are separated into two type: [*O-streams*](OStream) (output streams) and [*i-streams*](IStream) (input streams).
The [`Buf`] type can be used to handle these streams.

### Encoding

To encode an object directly using the `Encode` trait, simply allocate a buffer for the encoding and wrap it in an [`OStream`] object:

```
use librum::{Encode, OStream, SizedEncode};

let mut buf = [0x00; char::MAX_ENCODED_SIZE];
let mut stream = OStream::new(&mut buf);

'Ж'.encode(&mut stream).unwrap();

assert_eq!(buf, [0x00, 0x00, 0x04, 0x16].as_slice());
```

Streams can also be used to chain multiple objects together:

```
use librum::{Encode, OStream, SizedEncode};

let mut buf = [0x0; char::MAX_ENCODED_SIZE * 0x5];
let mut stream = OStream::new(&mut buf);

// Note: For serialising multiple characters, the
// `String` and `SizedStr` types are usually
// preferred.

'ل'.encode(&mut stream).unwrap();
'ا'.encode(&mut stream).unwrap();
'م'.encode(&mut stream).unwrap();
'د'.encode(&mut stream).unwrap();
'ا'.encode(&mut stream).unwrap();

assert_eq!(buf, [
    0x00, 0x00, 0x06, 0x44, 0x00, 0x00, 0x06, 0x27,
    0x00, 0x00, 0x06, 0x45, 0x00, 0x00, 0x06, 0x2F,
    0x00, 0x00, 0x06, 0x27
]);
```

If the encoded type additionally implements [`SizedEncode`], then the maximum size of any encoding is guaranteed with the [`MAX_ENCODED_SIZE`](SizedEncode::MAX_ENCODED_SIZE) constant.

Numerical primitives are encoded in big endian (a.k.a. [network order](https://en.wikipedia.org/wiki/Endianness#Networking)) for... reasons.
It is recommended for implementors to follow this convention as well.

### Decoding

Decoding works with a similar syntax to encoding.
To decode a byte array, simply call the [`decode`](Decode::decode) method with an [`IStream`] object:

```
use librum::{Decode, IStream};

let data = [0x45, 0x54];
let mut stream = IStream::new(&data);

assert_eq!(u16::decode(&mut stream).unwrap(), 0x4554);

// Data can theoretically be reinterpretred:

stream = IStream::new(&data);

assert_eq!(u8::decode(&mut stream).unwrap(), 0x45);
assert_eq!(u8::decode(&mut stream).unwrap(), 0x54);

// Including as tuples:

stream = IStream::new(&data);

assert_eq!(<(u8, u8)>::decode(&mut stream).unwrap(), (0x45, 0x54));
```

## Examples

A UDP server/client for geographic data:

```
use librum::{Buf, Encode, Decode, SizedEncode};
use std::io;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::thread::spawn;

// City, region, etc.:
##[derive(Clone, Copy, Debug, Decode, Encode, Eq, PartialEq, SizedEncode)]
enum Area {
    AlQuds,
    Byzantion,
    Cusco,
    Tenochtitlan,
    // ...
}

// Client-to-server message:
##[derive(Debug, Decode, Encode, PartialEq, SizedEncode)]
enum Request {
    AtmosphericHumidity { area: Area },
    AtmosphericPressure { area: Area },
    AtmosphericTemperature { area: Area },
    // ...
}

// Server-to-client message:
##[derive(Debug, Decode, Encode, PartialEq, SizedEncode)]
enum Response {
    AtmosphericHumidity(f64),
    AtmosphericPressure(f64), // Pascal
    AtmosphericTemperature(f64), // Kelvin
    // ...
}

struct Party {
    pub socket: UdpSocket,

    pub request_buf:  Buf::<Request>,
    pub response_buf: Buf::<Response>,
}

impl Party {
    pub fn new<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let socket = UdpSocket::bind(addr)?;

        let this = Self {
            socket,

            request_buf:  Buf::new(),
            response_buf: Buf::new(),
        };

        Ok(this)
    }
}

let mut server = Party::new("127.0.0.1:27015").unwrap();

let mut client = Party::new("0.0.0.0:0").unwrap();

spawn(move || {
    let Party { socket, mut request_buf, mut response_buf } = server;

    // Recieve initial request from client.

    let (len, addr) = socket.recv_from(&mut request_buf).unwrap();
    request_buf.set_len(len);

    let request = request_buf.read().unwrap();
    assert_eq!(request, Request::AtmosphericTemperature { area: Area::AlQuds });

    // Handle request and respond back to client.

    let response = Response::AtmosphericTemperature(44.4); // For demonstration's sake.

    response_buf.write(response).unwrap();
    socket.send_to(&response_buf, addr).unwrap();
});

spawn(move || {
    let Party { socket, mut request_buf, mut response_buf } = client;

    // Send initial request to server.

    socket.connect("127.0.0.1:27015").unwrap();

    let request = Request::AtmosphericTemperature { area: Area::AlQuds };

    request_buf.write(request);
    socket.send(&request_buf).unwrap();

    // Recieve final response from server.

    socket.recv(&mut response_buf).unwrap();

    let response = response_buf.read().unwrap();
    assert_eq!(response, Response::AtmosphericTemperature(44.4));
});
```

## Feature flags

Librum defines the following features:

* *`alloc`: Enables the [`Buf`] type and implementations for e.g. [`Box`](alloc::boxed::Box) and [`Arc`](alloc::sync::Arc)
* *`proc-macro`: Pulls the procedural macros from the [`librum_macros`](https://crates.io/crates/librum_macros/) crate
* *`std`: Enables implementations for types such as [`Mutex`](std::sync::Mutex) and [`RwLock`](std::sync::RwLock)

Features marked with * are enabled by default.

## Documentation

Librum has its documentation written in-source for use by `rustdoc`.
See [Docs.rs](https://docs.rs/librum/latest/librum/) for an on-line, rendered instance.

Currently, these docs make use of some unstable features for the sake of readability.
The nightly toolchain is therefore required when rendering them.

## Contribution

Librum does not accept source code contributions at the moment.
This is a personal choice by the maintainer and may be undone in the future.

Do however feel free to open up an issue on [`GitLab`](https://gitlab.com/bjoernager/librum/issues/) or (preferably) [`GitHub`](https://github.com/bjoernager/librum/issues/) if you feel the need to express any concerns over the project.

## Copyright & Licence

Copyright 2024 Gabriel Bjørnager Jensen.

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
See the GNU Lesser General Public License for more details.

You should have received a copy of the GNU Lesser General Public License along with this program.
If not, see <https://www.gnu.org/licenses/>.