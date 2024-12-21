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

use crate::decode::Decode;

use core::borrow::Borrow;

#[cfg(feature = "alloc")]
use core::ffi::CStr;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

#[cfg(feature = "alloc")]
use alloc::ffi::CString;

#[cfg(feature = "alloc")]
use alloc::rc::Rc;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "alloc")]
use alloc::string::String;

#[cfg(all(feature = "alloc", target_has_atomic = "ptr"))]
use alloc::sync::Arc;

/// Indicates a scheme relationship between borrowed and owned types.
///
/// Implementing this trait is specifically a promise that <code>&lt;Self as [Decode]&gt;::[decode](Decode::decode)</code> can handle any encoding of `B`.
///
/// This trait is mainly useful for types that implement [`Encode`](crate::encode::Encode::encode) but do not implement `Decode` for whatever reason (mostly due to being unsized).
/// The primary user of this trait is the `Decode` implementation of [`Cow`](alloc::borrow::Cow).
///
/// # Arrays
///
/// This trait in the form <code>DecodeBorrowed&lt;[\[T\]]&gt;</code> is not implemented for [`[T; N]`](array) due to the fact that arrays do not encode their length, instead having it hard-coded into the type, thus rendering their scheme incompatible with that of slices.
///
/// [\[T\]]: slice
///
/// An alternative to using arrays would be to use [`SizedSlice`](crate::SizedSlice), which *do* encode its length.
pub trait DecodeBorrowed<B: ?Sized>: Borrow<B> + Decode { }

impl<T: Decode> DecodeBorrowed<T> for T { }

#[cfg(all(feature = "alloc", target_has_atomic = "ptr"))]
#[cfg_attr(doc, doc(cfg(all(feature = "alloc", target_has_atomic = "ptr"))))]
impl<T: Decode> DecodeBorrowed<T> for Arc<T> { }

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: Decode> DecodeBorrowed<T> for Box<T> { }

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl DecodeBorrowed<CStr> for CString { }

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: Decode> DecodeBorrowed<T> for Rc<T> { }

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl DecodeBorrowed<str> for String { }

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: Decode> DecodeBorrowed<[T]> for Vec<T> { }
