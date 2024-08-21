# bzipper

[`bzipper`](https://crates.io/crates/bzipper) is a binary (de)serialiser for the Rust language.

Contrary to [Serde](https://crates.io/crates/serde/)/[Bincode](https://crates.io/crates/bincode/), the goal of bzipper is to serialise with a known size constraint.
Therefore, this crate may be more suited for networking or other cases where a fixed-sized buffer is needed.

Keep in mind that this project is still work-in-progress.

This crate is compatible with `no_std`.

## Data model

Most primitive types serialise losslessly, with the exception being `usize` and `isize`.
These serialise as `u32` and `i32`, respectively, for portability reasons.

Unsized types, such as `str` and slices, are not supported.
Instead, arrays should be used.
For strings, the `FixedString` type is also provided.

## Usage

This crate revolves around the `Serialise` and `Deserialise` traits, both of which are commonly used in conjunction with streams (more specifically, s-streams and d-streams).

Many core types come implemented with bzipper, including primitives as well as some standard library types such as `Option` and `Result`.

It is recommended in most cases to just derive these traits for custom types (enumerations and structures only).
Here, each field is chained in declaration order:

```rs
use bzipper::{Deserialise, Serialise};

#[derive(Debug, Deserialise, PartialEq, Serialise)]
struct IoRegister {
    addr:  u32,
    value: u16,
}

let mut buf: [u8; IoRegister::SERIALISED_SIZE] = Default::default();
IoRegister { addr: 0x04000000, value: 0x0402 }.serialise(&mut buf).unwrap();

assert_eq!(buf, [0x04, 0x00, 0x00, 0x00, 0x04, 0x02]);

assert_eq!(IoRegister::deserialise(&buf).unwrap(), IoRegister { addr: 0x04000000, value: 0x0402 });
```

### Serialisation

To serialise an object implementing `Serialise`, simply allocate a buffer for the serialisation.
The required size of any given serialisation is specified by the `SERIALISED_SIZE` constant:

```rs
use bzipper::Serialise;

let mut buf: [u8; char::SERIALISED_SIZE] = Default::default();
'Ж'.serialise(&mut buf).unwrap();

assert_eq!(buf, [0x00, 0x00, 0x04, 0x16]);
```

The only special requirement of the `serialise` method is that the provided byte slice has an element count of exactly `SERIALISED_SIZE`.

We can also use streams to *chain* multiple elements together.

```rs
use bzipper::Serialise;

let mut buf: [u8; char::SERIALISED_SIZE * 5] = Default::default();
let mut stream = bzipper::Sstream::new(&mut buf);

stream.append(&'ل');
stream.append(&'ا');
stream.append(&'م');
stream.append(&'د');
stream.append(&'ا');

assert_eq!(buf, [0x00, 0x00, 0x06, 0x44, 0x00, 0x00, 0x06, 0x27, 0x00, 0x00, 0x06, 0x45, 0x00, 0x00, 0x06, 0x2F, 0x00, 0x00, 0x06, 0x27]);
```

When serialising primitives, the resulting byte stream is in big endian (a.k.a. network endian).
It is recommended for implementors to adhere to this convention as well.

### Deserialisation

Deserialisation works with an almost identical syntax to serialisation.

To deserialise a buffer, simply call the `deserialise` method:

```rs
use bzipper::Deserialise;

let data = [0x45, 0x54];
assert_eq!(<u16>::deserialise(&data).unwrap(), 0x4554);
```

Just like with serialisations, the `Dstream` can be used to deserialise chained elements:

```rs
use bzipper::Deserialise;

let data = [0x45, 0x54];
let stream = bzipper::Dstream::new(&data);

assert_eq!(stream.take::<u8>().unwrap(), 0x45);
assert_eq!(stream.take::<u8>().unwrap(), 0x54);
```
