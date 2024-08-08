# Changelog

This is the changelog of `bzipper`.
See `"README.md"` for more information.

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
