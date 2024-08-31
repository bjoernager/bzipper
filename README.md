# bzipper

[bzipper](https://crates.io/crates/bzipper/) is a binary (de)serialiser for the Rust language.

In contrast to [Serde](https://crates.io/crates/serde/)/[Bincode](https://crates.io/crates/bincode/), the primary goal of bzipper is to serialise with a known size constraint.
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

This crate revolves around the `Serialise` and `Deserialise` traits, both of which use *streams* &ndash; or more specifically &ndash; s-streams and d-streams.

Many core types come implemented with bzipper, including primitives as well as some standard library types such as `Option` and `Result`.

It is recommended in most cases to just derive these two traits for custom types (although this is only supported with enumerations and structures).
Here, each field is *chained* according to declaration order:

```rust
use bzipper::{Buffer, Deserialise, Serialise};

#[derive(Debug, Deserialise, PartialEq, Serialise)]
struct IoRegister {
    addr:  u32,
    value: u16,
}

let mut buf = Buffer::new();

buf.write(IoRegister { addr: 0x04000000, value: 0x0402 }).unwrap();

assert_eq!(buf.len(), 0x6);
assert_eq!(buf, [0x04, 0x00, 0x00, 0x00, 0x04, 0x02]);

assert_eq!(buf.read().unwrap(), IoRegister { addr: 0x04000000, value: 0x0402 });
```

### Serialisation

To serialise an object implementing `Serialise`, simply allocate a buffer for the serialisation and wrap it in an s-stream (*serialisation stream*) with the `Sstream` type.

```rust
use bzipper::{Serialise, Sstream};

let mut buf = [Default::default(); char::MAX_SERIALISED_SIZE];
let mut stream = Sstream::new(&mut buf);

'Ж'.serialise(&mut stream).unwrap();

assert_eq!(stream, [0x00, 0x00, 0x04, 0x16]);
```

The maximum size of any given serialisation is specified by the `MAX_SERIALISED_SIZE` constant.

We can also use streams to chain multiple elements together:

```rust
use bzipper::{Serialise, Sstream};

let mut buf = [Default::default(); char::MAX_SERIALISED_SIZE * 0x5];
let mut stream = Sstream::new(&mut buf);

// Note: For serialising multiple characters, the
// `FixedString` type is usually preferred.

'ل'.serialise(&mut stream).unwrap();
'ا'.serialise(&mut stream).unwrap();
'م'.serialise(&mut stream).unwrap();
'د'.serialise(&mut stream).unwrap();
'ا'.serialise(&mut stream).unwrap();

assert_eq!(buf, [
    0x00, 0x00, 0x06, 0x44, 0x00, 0x00, 0x06, 0x27,
    0x00, 0x00, 0x06, 0x45, 0x00, 0x00, 0x06, 0x2F,
    0x00, 0x00, 0x06, 0x27
]);
```

When serialising primitives, the resulting byte stream is in big endian (a.k.a. network endian).
It is recommended for implementors to adhere to this convention as well.

### Deserialisation

Deserialisation works with a similar syntax to serialisation.

D-streams (*deserialisation streams*) use the `Dstream` type and are constructed in a manner similar to s-streams.
To deserialise a buffer, simply call the `deserialise` method with the strema:

```rust
use bzipper::{Deserialise, Dstream};

let data = [0x45, 0x54];
let stream = Dstream::new(&data);
assert_eq!(u16::deserialise(&stream).unwrap(), 0x4554);
```

And just like s-streams, d-streams can also be used to handle chaining:

```rust
use bzipper::{Deserialise, Dstream};

let data = [0x45, 0x54];
let stream = Dstream::new(&data);

assert_eq!(u8::deserialise(&stream).unwrap(), 0x45);
assert_eq!(u8::deserialise(&stream).unwrap(), 0x54);

// The data can also be deserialised as a tuple (up
// to twelve elements).

let stream = Dstream::new(&data);
assert_eq!(<(u8, u8)>::deserialise(&stream).unwrap(), (0x45, 0x54));
```
