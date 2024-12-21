// Copyright 2024 Gabriel Bjørnager Jensen.
//
// This file is part of Oct.
//
// Oct is free software: you can redistribute it
// and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// Oct is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even
// the implied warranty of MERCHANTABILITY or FIT-
// NESS FOR A PARTICULAR PURPOSE. See the GNU Less-
// er General Public License for more details.
//
// You should have received a copy of the GNU Less-
// er General Public License along with Oct. If
// not, see <https://www.gnu.org/licenses/>.

//! Error variants.
//!
//! This module defines the error types used by oct.
//! All of these types define (at least conditionally) the [`Error`](core::error::Error) trait.

use crate::use_mod;

use_mod!(pub encode);
use_mod!(pub output);
use_mod!(pub sized_encode);

/// Implements [`Encode`] for the provided type.
///
/// This derive macro assumes that all fields implement <code>Encode&lt;[Error]: [Into]&lt;[GenericEncodeError]&gt;&gt;</code>.
/// If this is **not** the case, then the trait should be implemented manually instead.
///
/// [Error]: Encode::Error
/// [GenericEncodeError]: crate::error::GenericEncodeError
///
/// Do also consider deriving [`SizedEncode`](derive@SizedEncode) -- if possible.
///
/// # Structs
///
/// For structures, each element is chained in **order of declaration.**
/// If the structure is a unit structure (i.e. it has *no* fields) then it is encoded equivalently to the [unit] type.
///
/// For example, the following struct will encode its field `foo` followed by `bar`:
///
/// ```
/// use oct::encode::Encode;
///
/// #[derive(Encode)]
/// struct FooBar {
///     pub foo: char,
///     pub bar: char,
/// }
/// ```
///
/// This should be kept in mind when changing the structure's declarationm as doing so may invalidate previous encodings.
///
/// The [`Error`](Encode::Error) type will in all cases just be `GenericEncodeError`.
///
/// # Enums
///
/// Enumerations encode like structures except that each variant additionally encodes a unique discriminant.
///
/// By default, each discriminant is assigned from the range 0 to infinite, to the extend allowed by the [`isize`] type and its encoding (as which **all** discriminants are encoded).
/// A custom discriminant may be set instead by assigning the variant an integer constant.
/// Unspecified discriminants then increment the previous variant's discriminant:
///
/// ```
/// use oct::Slot;
/// use oct::encode::Encode;
///
/// #[derive(Encode)]
/// enum Num {
///     Two = 0x2,
///
///     Three,
///
///     Zero = 0x0,
///
///     One,
/// }
///
/// let mut buf = Slot::with_capacity(size_of::<i16>());
///
/// buf.write(Num::Zero).unwrap();
/// assert_eq!(buf, [0x00, 0x00].as_slice());
///
/// buf.write(Num::One).unwrap();
/// assert_eq!(buf, [0x01, 0x00].as_slice());
///
/// buf.write(Num::Two).unwrap();
/// assert_eq!(buf, [0x02, 0x00].as_slice());
///
/// buf.write(Num::Three).unwrap();
/// assert_eq!(buf, [0x03, 0x00].as_slice());
/// ```
///
/// Variants with fields are encoded exactly like structures.
/// That is, each field is chained in order of declaration.
///
/// For error handling, the `Error` type is defined as:
///
/// <code>[EnumEncodeError]&lt;&lt;Repr as Encode&gt;::Error, GenericEncodeError&gt;</code>,
///
/// [EnumEncodeError]: crate::error::GenericEncodeError
///
/// wherein `Repr` is the enumeration's representation.
///
/// # Unions
///
/// Unions cannot derive `Encode` due to the uncertainty of their contents.
/// The trait should therefore be implemented manually for such types.
#[cfg(feature = "proc-macro")]
#[cfg_attr(doc, doc(cfg(feature = "proc-macro")))]
#[doc(inline)]
pub use oct_macros::Encode;

/// Implements [`Encode`](trait@Encode) using the default implementation.
///
/// For simple structures, the value of [`MAX_ENCODED_SIZE`](SizedEncode::MAX_ENCODED_SIZE) is set as the combined value of <code>T*n*::MAX_ENCODED_SIZE</code> wherein <code>T*n*</code> is the type of each field.
///
/// For enumerations, the value is set such that each variant is treated like a structure (with the discriminant as an extra field) and where the variant that produces the largest `MAX_ENCODED_SIZE` is chosen.
///
/// As untagged unions cannot derive `Encode`, `SizedEncode` also cannot be derived for them.
///
/// Do remember that deriving this trait is only recommended
#[cfg(feature = "proc-macro")]
#[cfg_attr(doc, doc(cfg(feature = "proc-macro")))]
#[doc(inline)]
pub use oct_macros::SizedEncode;
