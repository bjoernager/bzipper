# `bzipper`

[`bzipper`](https://crates.io/crates/bzipper) is a binary (de)serialiser for the Rust language.

Contrary to [Serde](https://crates.io/crates/serde/)/[Bincode](https://crates.io/crates/bincode/), the goal of this crate is to serialise data with a known size limit.
Therefore, this crate may be more suited for networking or other cases where a fixed-sized buffer is needed.

Keep in mind that this project is still work-in-progress.

This crate does not require any dependencies at the moment.

See [Docs.rs](https://docs.rs/bzipper/latest/bzipper/) for documentation.

## Data Model

Most primitive types serialise losslessly, with the exception being `usize` and `isize`.
These serialise as `u16` and `i16`, respectively, for portability reasons.

Unsized types, such as `str` and slices, are not supported.
Instead, array should be used.
For strings, the `FixedString` type is also provided.

## Copyright & Licensing

Copyright 2024 Gabriel Bjørnager Jensen.

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
