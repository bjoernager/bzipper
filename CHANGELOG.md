# Changelog

This is the changelog of bzipper.
See `"README.md"` for more information.

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
