# Changelog

This is the changelog of bzipper.
See `README.md` for more information.

## 0.10.1

* Clean up and refactor code
* Add more tests
* Fix `DoubleEndedIterator` implementation for `SizedIter`

## 0.10.0

* Clean up code
* Implement `Encode` and `Decode` for `Cell` and `HashSet`
* Implement `SizedEncode` for `Cell`
* Add missing `SizedEncode` implementations for `Cow`, `LazyCell`, and `LazyLock`
* Unimplement `Decode` for `Cow`, `LazyCell`, and `LazyLock`
* Add missing `Decode` implementations for `RefCell`
* Fix feature flags for `SizedEncode` implementations of `Rc` and `Arc`

## 0.9.0

* Implement `Encode` and `Decode` for `LinkedList`, `HashMap`, `Cow`, `PhantomPinned`, `LazyCell`, `LazyLock`
* Add missing `Decode` implementation for `Box`
* Update inline rules
* Implement traits for tuples using macros
* Implement `SizedEncode` for `PhantomPinned`, `Cow`, `LazyCell`, `LazyLock`, `&_`, `&mut _`
* Implement `Encode` for `&_` and `&mut _`
* Update docs

## 0.8.1

* Update package metadata

## 0.8.0

* Rename `FixedString` to `SizedStr`
* Implement `PartialEq<String>` and `PartialOrd<String>` for `SizedStr`
* Add constructors `from_utf8` and `from_utf8_unchecked` to `SizedStr`
* Remove `pop`, `push_str`, and `push` from `SizedStr`
* Implement `FromIterator<char>` for `SizedStr`
* Rename `Serialise` to `Encode`
* Rename `Deserialise` to `Decode`
* Remove `Sized` requirement for `Encode`
* Add benchmarks
* Update package metadata
* Rename `Sstream` to `OStream`
* Rename `Dstream` to `IStream`
* Update readme
* Refactor code
* Update lints
* Implement `Encode` and `Decode` for `IpAddr`, `Ipv4Addr`, `Ipv6Addr`, `Mutex`, `Box`, `RwLock`, `Rc`, `Arc`, `Wrapping`, `Saturating`, `AtomicBool`, `AtomicU8`, `AtomicU16`, `AtomicU32`, `AtomicU64`, `AtomicI8`, `AtomicI16`, `AtomicI32`, `AtomicI64`, `AtomicUsize`, `AtomicIsize`, `SocketAddrV4`, `SocketAddrV6`, `SocketAddr`, `Range`, `RangeFrom`, `RangeFull`, `RangeInclusive`, `RangeTo`, `RangeToInclusive`, `Bound`, `RefCell`, `String`, and `Vec`
* Update docs
* Add `SizedSlice` type
* Add `SizedIter` type
* Rename `Buffer` type to `Buf`
* Remove `Add` and `AddAssign` implementations from `SizedStr`
* Add *Features* section to readme
* Honour explicit enumeration discriminants
* Encode enumeration discriminants as `isize`
* Add `SizedEncode` trait
* Outsource `MAX_SERIALISED_SIZE` to `SizedEncode` as `MAX_ENCODED_SIZE`
* Implement `Iterator`, `ExactSizeIterator`, `FusedIterator`, and `DoubleEndedIterator` for `SizedIter`
* Implement `AsRef<[T]>` and `AsMut<[T]>` for `SizedIter<T, ..>`
* Implement `Clone` for `SizedIter`
* Add `as_slice` and `as_mut_slice` methods to `SizedIter`
* Add `from_raw_parts` constructor and `into_raw_parts` destructor to `SizedSlice`
* Add `set_len` method to `SizedSlice`
* Add `len`, `is_empty`, `is_full`, and `capacity` methods to `SizedSlice`
* Add `as_slice` and `as_mut_slice` methods to `SizedSlice`
* Add `as_ptr` and `as_mut_ptr` methods to `SizedSlice`
* Implement `AsMut<[T]>` and `AsRef<[T]>` for `SizedSlice<T, ..>`
* Implement `Borrow<[T]>` and `BorrowMut<[T]>` for `SizedSlice<T, ..>`
* Implement `Deref<[T]>` and `DerefMut<[T]>` for `SizedSlice<T, ..>`
* Implement `Debug` for `SizedSlice`
* Implement `Default` for `SizedSlice`
* Implement `Clone` for `SizedSlice`
* Implement `Encode`, `Decode`, and `SizedEncode` for `SizedSlice`
* Implement `Eq` and `PartialEq` for `SizedSlice`
* Implement `Ord` and `PartialOrd` for `SizedSlice`
* Implement `From<[T; N]>` for `SizedSlice<T, N>`
* Implement `Hash` for `SizedSlice`
* Implement `Index` and `IndexMut` for `SizedSlice`
* Implement `IntoIterator` for `SizedSlice` (including references hereto)
* Implement `TryFrom<&[T]>` for `SizedSlice<T, ..>`
* Implement `From<SizedSlice<T, ..>>` for `Vec<[T]>`
* Implement `From<SizedSlice<T, ..>>` for `Box<[T]>`
* Add `into_boxed_slice` and `into_vec` destructors to `SizedSlice`
* Add `into_boxed_str` and `into_string` destructors to `SizedStr`
* Bump Rust version to `1.83` for `bzipper`
* Mark `SizedStr::as_mut_ptr` as const
* Implement `FromIterator<T>` for `SizedSlice<T, ..>`
* Make `SizedStr::new` take a `&str` object
* Add `is_empty` and `is_full` methods to `Buf`
* Disallow non-empty single-line functions
* Add `SAFETY` comments
* Implement `PartialEq<&mut [u8]>` and `PartialEq<[u8]>` for `Buf`
* Implement `Index` and `IndexMut` for `Buf`
* Add `from_raw_parts` constructor and `into_raw_parts` destructor to `Buf`
* Add *Documentation* and *Contribution* sections to readme
* Add *Copyright & Licence* section to readme
* Add Clippy configuration file
* Add more unit tests
* Add debug assertions
* Remove `as_ptr` and `as_slice` methods from `IStream` and `OStream`
* Remove `len`, `is_empty`, and `is_full` methods from `IStream` and `OStream`
* Unimplement all manually-implemented traits from `IStream` and `OStream`
* Mark `new` and `write` in `OStream` as const
* Mark the `read` method in `IStream` as const
* Add `close` destructor to `OStream` and `IStream`
* Implement `Encode` for `[T]` and `str`
* Encode `usize` and `isize` as `u16` and `i16` again
* Split `Error` type into `EncodeError`, `DecodeError`, `Utf8Error`, `Utf16Error`, `SizeError`, and `StringError`
* Remove `Result` type
* Add `error` module
* Make `IStream::read` and `OSream::write` panic on error
* Update logo
* Add more examples to docs
* Unmark all functions in `Buf` as const
* Implement `From<SizedStr>` for `Box<str>`
* Always implement `Freeze`, `RefUnwindSafe`, `Send`, `Sync`, `Unpin`, and `UnwindSafe` for `Buf`
* Add *Examples* section to readme
* Implement `SizedEncode` for all previous `Encode` types
* Bump dependency versions
* Implement `SizedEncode` for `IpAddr`, `Ipv4Addr`, `Ipv6Addr`, `Mutex`, `Box`, `RwLock`, `Rc`, `Arc`, `Wrapping`, `Saturating`, `AtomicBool`, `AtomicU8`, `AtomicU16`, `AtomicU32`, `AtomicU64`, `AtomicI8`, `AtomicI16`, `AtomicI32`, `AtomicI64`, `AtomicUsize`, `AtomicIsize`, `SocketAddrV4`, `SocketAddrV6`, `SocketAddr`, `Range`, `RangeFrom`, `RangeFull`, `RangeInclusive`, `RangeTo`, `RangeToInclusive`, `Bound`, and `RefCell`

## 0.7.0

* Make `alloc` and `std` default features
* Make serialisations variably sized again
* Refactor derive implementations
* Completely rework streams
* Fix tuple deserialisation
* Encode `FixedString` in UTF-8
* Remove methods `from_chars` and `set_len` from `FixedString`
* Rename `as_slice` and `as_mut_slice` methods in `FixedString` to `as_st` and `as_mut_str`
* Add methods `as_bytes`, `push_str`, `chars`, `capacity`, and `char_indices` to `FixedString`
* Rework `FixedString` traits
* Remove `FixedIter`
* Update lints
* Add methods `set_len` and `set_len_unchecked` to `Buffer`
* Elaborate docs
* Update readme
* Do not require `Serialise` for `Deserialise`
* Rename `SERIALISED_SIZE` in `Serialise` to `MAX_SERIALISED_SIZE`
* Use streams in `Serialise` and `Deserialise`
* Drop `Serialise` requirement for `Buffer`
* Add methods `with_capacity` and `capacity` to `Buffer`

## 0.6.2

* Fix `Deserialise` derive for unit variants
* Refactor `Serialise` derive for enumerations

## 0.6.1

* Bump dependency version
* Update docs
* Add more examples

## 0.6.0

* Update readme
* Add `Buffer` type
* Bump minor version
* Implement `PartialEq<&[char]>` for `FixedString`
* Update tests
* Implement `PartialOrd<&[char]>` and `PartialOrd<&str>` for `FixedString`
* Remove custom methods `get`, `get_unchecked`, `get_mut`, and  `get_unchecked_mut`, `iter`, and `iter_mut` from `FixedString`

## 0.5.2

* Respecify version numbers

## 0.5.1

* Specify `bzipper_macros` version

## 0.5.0

* Bump minor version
* Add macros crate
* Add derive macros
* Update package metadata
* Update readme
* Expand docs
* Require fixed size (de)serialisations
* Add more error variants
* Require `bzipper::Error` for (de)serialisation
* Reworks streams
* Remove `Buffer`
* Rework `FixedString`
* Serialise `usize` and `isize` as `u32` and `i32`, respectively
* Rework arrays (de)serialisation
* Fix `Result` serialisations
* Add new logo
* Add features `alloc` and `std`
* Specify rustc version
* Rename `FixedStringIter` to `FixedIter`
* Implement `Serialise` and `Deserialise` for single tuples and `PhantomData`

## 0.4.7

* Extensively elaborate docs
* Update readme

## 0.4.6

* Fix docs logo (again)
* Update docs (add examples)

## 0.4.5

* Fix package metadata :(

## 0.4.4

* Fix docs logo

## 0.4.3

* Reformat changelog
* Update logo
* Add docs logo

## 0.4.2

* Update package metadata

## 0.4.1

* Update readme

## 0.4.0

* Add logo
* Clean up code
* Fix array deserialisation (require `Default`)
* Bump minor
* Update commenting
* Make serialisations fallible
* Impl `Serialise` and `Deserialise` for `usize` and `isize` (restrict to 16 bits)
* Add new errors: `UsizeOutOfRange`, `IsizeOutOfRange`
* Rework sstreams
* Add buffer type
* Fix serialisation of `Option<T>`
* Disable `std`
* Rename error: `EndOfDStream` -> `EndOfStream`
* Update documentation
* Update readme
* Reformat changelog

## 0.3.0

* Bump minor
* Document errors
* Rename: `ArrayLengthMismatch` -> `ArrayTooShort`
* Remove error `FixedStringTooShort`
* Rename: `InvalidUtf8` -> `BadString`
* Rework errors
* Rename methods: `as_d_stream` -> `as_dstream`, `to_s_stream` -> `to_sstream`
* Add `SERIALISATION_LIMIT` constant to `Serialise`
* Make some deserialisations infallible
* Add method `append_byte` to `SStream`
* Add method `take_byte` to `DStream`
* Rename `SStream` -> `Sstream`, `DStream` -> `Dstream`
* Update readme
* Update documentation
* Make `Deserialise` require `Serialise`
* Fix copyright/license notice in `"src/serialise/test.rs"`

## 0.2.0

* Clean up code
* Implement `Ord` and `PartialOrd` for `FixedString`
* Implement `Index` and `IndexMut` for `FixedString`
* Add `get` and `get_mut` methods to `FixedString`
* Implement `From<[char; N]>` for `FixedString`
* Bump minor
* Implement `Serialise` and `Deserialise` for tuples

## 0.1.0

* Bump minor
* Export all in crate root
* Add fixed string type
* Add new errors
* Update documentation
* Add `as_d_stream` method to `SStream`
* Add `to_s_stream` and `as_slice` methods to `DStream`

## 0.0.2

* Add license files

## 0.0.1

* Fix copyright notices
* Add license notices
* Update readme

## 0.0.0

* Add changelog
* Fork from `backspace`
* Add gitignore
* Add documentation
* Add tests
* License under LGPL-3
* Configure lints
* Add readme
