// Copyright 2024 Gabriel Bjørnager Jensen.
//
// This file is part of bZipper.
//
// bZipper is free software: you can redistribute
// it and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// bZipper is distributed in the hope that it will
// be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Less-
// er General Public License along with bZipper. If
// not, see <https://www.gnu.org/licenses/>.

#[cfg(test)]
mod test;

use crate::OStream;
use crate::error::EncodeError;

use core::cell::{Cell, LazyCell, RefCell};
use core::convert::Infallible;
use core::hash::BuildHasher;
use core::hint::unreachable_unchecked;
use core::marker::{PhantomData, PhantomPinned};
use core::net::{
	IpAddr,
	Ipv4Addr,
	Ipv6Addr,
	SocketAddr,
	SocketAddrV4,
	SocketAddrV6,
};
use core::num::{Saturating, Wrapping};
use core::ops::{
	Bound,
	Range,
	RangeFrom,
	RangeFull,
	RangeInclusive,
	RangeTo,
	RangeToInclusive,
};

#[cfg(feature = "alloc")]
use alloc::borrow::{Cow, ToOwned};

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

#[cfg(feature = "alloc")]
use alloc::collections::LinkedList;

#[cfg(feature = "alloc")]
use alloc::string::String;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "alloc")]
use alloc::rc::Rc;

#[cfg(feature = "alloc")]
use alloc::sync::Arc;

#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};

#[cfg(feature = "std")]
use std::sync::{LazyLock, Mutex, RwLock};

/// Denotes a type capable of being encoded.
///
/// It is recommended to simply derive this trait for custom types.
/// It can, however, also be manually implemented.
///
/// If all possible encodings have a known maximum size, then the [`SizedEncode`](crate::SizedEncode) trait should additionally be implemented.
///
/// # Examples
///
/// A manual implementation of `Encode`:
///
/// ```
/// // Manual implementation of custom type. This im-
/// // plementation is equivalent to what would have
/// // been derived.
///
/// use bzipper::{Encode, OStream};
/// use bzipper::error::EncodeError;
///
/// struct Foo {
///     bar: u16,
///     baz: f32,
/// }
///
/// impl Encode for Foo {
///     fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
///         // Encode fields using chaining.
///
///         self.bar.encode(stream)?;
///         self.baz.encode(stream)?;
///
///         Ok(())
///     }
/// }
/// ```
pub trait Encode {
	/// Encodes `self` into the provided stream.
	///
	/// # Errors
	///
	/// If encoding fails, such as if `self` is unencodable, an error is returned.
	///
	/// # Panics
	///
	/// If `stream` cannot contain the entirety of the resulting encoding, then this method should panic.
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError>;
}

impl<T: Encode> Encode for &T {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		T::encode(self, stream)
	}
}

impl<T: Encode> Encode for &mut T {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		T::encode(self, stream)
	}
}

/// Implemented for tuples with up to twelve members.
#[cfg_attr(doc, doc(fake_variadic))]
impl<T: Encode> Encode for (T, ) {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		self.0.encode(stream)
	}
}

impl<T: Encode, const N: usize> Encode for [T; N] {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		for value in self {
			value.encode(stream)?;
		}

		Ok(())
	}
}

impl<T: Encode> Encode for [T] {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		self.len().encode(stream)?;

		for value in self {
			value.encode(stream)?;
		}

		Ok(())
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: Encode> Encode for Arc<T> {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		T::encode(self, stream)
	}
}

impl Encode for bool {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		u8::from(*self).encode(stream)
	}
}

impl<T: Encode> Encode for Bound<T> {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		match *self {
			Self::Included(ref bound) => {
				0x0u8.encode(stream)?;
				bound.encode(stream)?;
			}

			Self::Excluded(ref bound) => {
				0x1u8.encode(stream)?;
				bound.encode(stream)?;
			}

			Self::Unbounded => {
				0x2u8.encode(stream)?;
			}
		}

		Ok(())
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: Encode> Encode for Box<T> {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		T::encode(self, stream)
	}
}

impl<T: Encode + Copy> Encode for Cell<T> {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		self.get().encode(stream)
	}
}

impl Encode for char {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		u32::from(*self).encode(stream)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: Encode + ToOwned> Encode for Cow<'_, T> {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		T::encode(self, stream)
	}
}

#[cfg(feature = "std")]
#[cfg_attr(doc, doc(cfg(feature = "std")))]
impl<K, V, S> Encode for HashMap<K, V, S>
where
	K: Encode,
	V: Encode,
	S: BuildHasher,
	{
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		for (key, value) in self {
			key.encode(stream)?;
			value.encode(stream)?;
		}

		Ok(())
	}
}

#[cfg(feature = "std")]
#[cfg_attr(doc, doc(cfg(feature = "std")))]
impl<T, S> Encode for HashSet<T, S>
where
	T: Encode,
	S: BuildHasher,
	{
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		for key in self {
			key.encode(stream)?;
		}

		Ok(())
	}
}

// Especially useful for `Result<T, Infallible>`.
// **If** that is even needed, of course.
impl Encode for Infallible {
	#[inline(always)]
	fn encode(&self, _stream: &mut OStream) -> Result<(), EncodeError> {
		// SAFETY: `Infallible` can **never** be construct-
		// ed.
		unsafe { unreachable_unchecked() }
	}
}

/// This implementation encoded as discriminant denoting the IP version of the address (i.e. `4` for IPv4 and `6` for IPv6).
/// This is then followed by the respective address' own encoding (either [`Ipv4Addr`] or [`Ipv6Addr`]).
impl Encode for IpAddr {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		// The discriminant here is the IP version.

		match *self {
			Self::V4(ref addr) => {
				0x4u8.encode(stream)?;
				addr.encode(stream)?;
			}

			Self::V6(ref addr) => {
				0x6u8.encode(stream)?;
				addr.encode(stream)?;
			}
		}

		Ok(())
	}
}

/// This implementation encodes the address's bits in big-endian.
impl Encode for Ipv4Addr {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		let value = self.to_bits();
		value.encode(stream)
	}
}

/// This implementation encodes the address's bits in big-endian.
impl Encode for Ipv6Addr {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		let value = self.to_bits();
		value.encode(stream)
	}
}

/// This implementation casts `self` to `i16` before encoding.
/// If this conversion isn't possible for the given value, then the [`IsizeOutOfRange`](EncodeError::IsizeOutOfRange) error is returned.
impl Encode for isize {
	#[inline]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		let value = i16::try_from(*self)
			.map_err(|_| EncodeError::IsizeOutOfRange(*self))?;

		value.encode(stream)
	}
}

impl<T: Encode> Encode for LazyCell<T> {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		T::encode(self, stream)
	}
}

#[cfg(feature = "std")]
#[cfg_attr(doc, doc(cfg(feature = "std")))]
impl<T: Encode> Encode for LazyLock<T> {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		T::encode(self, stream)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: Encode> Encode for LinkedList<T> {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		for value in self {
			value.encode(stream)?;
		}

		Ok(())
	}
}

#[cfg(feature = "std")]
#[cfg_attr(doc, doc(cfg(feature = "std")))]
impl<T: Encode> Encode for Mutex<T> {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		self
			.lock()
			.unwrap_or_else(std::sync::PoisonError::into_inner)
			.encode(stream)
	}
}

/// This implementation encodes a sign denoting the optional's variant.
/// The sign is `false` for `None` instances and `true` for `Some` instances.
/// The contained value is encoded proceeding the sign.
impl<T: Encode> Encode for Option<T> {
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		match *self {
			None => false.encode(stream)?,

			Some(ref v) => {
				true.encode(stream)?;
				v.encode(stream)?;
			}
		};

		Ok(())
	}
}

impl<T> Encode for PhantomData<T> {
	#[inline(always)]
	fn encode(&self, _stream: &mut OStream) -> Result<(), EncodeError> {
		Ok(())
	}
}

impl Encode for PhantomPinned {
	#[inline(always)]
	fn encode(&self, _stream: &mut OStream) -> Result<(), EncodeError> {
		Ok(())
	}
}

impl<T: Encode> Encode for Range<T> {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		self.start.encode(stream)?;
		self.end.encode(stream)?;

		Ok(())
	}
}

impl<T: Encode> Encode for RangeFrom<T> {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		self.start.encode(stream)
	}
}

impl Encode for RangeFull {
	#[inline(always)]
	fn encode(&self, _stream: &mut OStream) -> Result<(), EncodeError> {
		Ok(())
	}
}

impl<T: Encode> Encode for RangeInclusive<T> {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		self.start().encode(stream)?;
		self.end().encode(stream)?;

		Ok(())
	}
}

impl<T: Encode> Encode for RangeTo<T> {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		self.end.encode(stream)
	}
}

impl<T: Encode> Encode for RangeToInclusive<T> {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		self.end.encode(stream)?;

		Ok(())
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: Encode> Encode for Rc<T> {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		T::encode(self, stream)
	}
}

impl<T: Encode> Encode for RefCell<T> {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		let value = self
			.try_borrow()
			.map_err(EncodeError::BadBorrow)?;

		T::encode(&value, stream)
	}
}

/// This implementation encodes a sign denoting the optional's variant.
/// The sign is `false` for denoting `Ok` and `true` for denoting `Err`.
/// The contained value is encoded proceeding the sign.
impl<T: Encode, E: Encode> Encode for core::result::Result<T, E> {
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		// The sign here is `false` for `Ok` objects and
		// `true` for `Err` objects.

		match *self {
			Ok(ref v) => {
				false.encode(stream)?;
				v.encode(stream)?;
			}

			Err(ref e) => {
				true.encode(stream)?;
				e.encode(stream)?;
			}
		};

		Ok(())
	}
}

#[cfg(feature = "std")]
#[cfg_attr(doc, doc(cfg(feature = "std")))]
impl<T: Encode> Encode for RwLock<T> {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		self
			.read()
			.or_else(|e| Ok(e.into_inner()))?
			.encode(stream)
	}
}

impl<T: Encode> Encode for Saturating<T> {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		self.0.encode(stream)
	}
}

/// This implementation encoded as discriminant denoting the IP version of the address (i.e. `4` for IPv4 and `6` for IPv6).
/// This is then followed by the respective address' own encoding (either [`SocketAddrV4`] or [`SocketAddrV6`]).
impl Encode for SocketAddr {
	#[inline]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		// The discriminant here is the IP version.

		match *self {
			Self::V4(ref addr) => {
				0x4u8.encode(stream)?;
				addr.encode(stream)?;
			}

			Self::V6(ref addr) => {
				0x6u8.encode(stream)?;
				addr.encode(stream)?;
			}
		}

		Ok(())
	}
}

/// This implementation encodes the address's bits followed by the port number, all of which in big-endian.
impl Encode for SocketAddrV4 {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		self.ip().encode(stream)?;
		self.port().encode(stream)?;

		Ok(())
	}
}

/// This implementation encodes the address's bits followed by the port number, all of which in big-endian.
impl Encode for SocketAddrV6 {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		self.ip().encode(stream)?;
		self.port().encode(stream)?;
		self.flowinfo().encode(stream)?;
		self.scope_id().encode(stream)?;

		Ok(())
	}
}

impl Encode for str {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		// Optimised encode. Don't just rely on `[char]`.

		self.len().encode(stream)?;
		stream.write(self.as_bytes());

		Ok(())
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl Encode for String {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		self.as_str().encode(stream)
	}
}

impl Encode for () {
	#[inline(always)]
	fn encode(&self, _stream: &mut OStream) -> Result<(), EncodeError> {
		Ok(())
	}
}

/// This implementation casts `self` to `u16` before encoding.
/// If this conversion isn't possible for the given value, then the [`IsizeOutOfRange`](EncodeError::IsizeOutOfRange) error is returned.
impl Encode for usize {
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		let value = u16::try_from(*self)
			.map_err(|_| EncodeError::UsizeOutOfRange(*self))?;

		value.encode(stream)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: Encode> Encode for Vec<T> {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		self.as_slice().encode(stream)
	}
}

impl<T: Encode> Encode for Wrapping<T> {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		self.0.encode(stream)
	}
}

macro_rules! impl_numeric {
	($ty:ty$(,)?) => {
		impl ::bzipper::Encode for $ty {
			#[inline]
			fn encode(&self, stream: &mut OStream) -> ::core::result::Result<(), ::bzipper::error::EncodeError> {
				stream.write(&self.to_be_bytes());

				Ok(())
			}
		}
	};
}

macro_rules! impl_tuple {
	{
		$($captures:ident: $tys:ident),+$(,)?
	} => {
		#[doc(hidden)]
		impl<$($tys: ::bzipper::Encode, )*> ::bzipper::Encode for ($($tys, )*) {
			#[inline(always)]
			fn encode(&self, stream: &mut ::bzipper::OStream) -> ::core::result::Result<(), ::bzipper::error::EncodeError> {
				let ($(ref $captures, )*) = *self;

				$(
					$captures.encode(stream)?;
				)*

				Ok(())
			}
		}
	};
}

macro_rules! impl_non_zero {
	($ty:ty$(,)?) => {
		impl ::bzipper::Encode for ::core::num::NonZero<$ty> {
			#[inline(always)]
			fn encode(&self, stream: &mut OStream) -> ::core::result::Result<(), ::bzipper::error::EncodeError> {
				self.get().encode(stream)
			}
		}
	};
}

macro_rules! impl_atomic {
	{
		width: $width:literal,
		ty: $ty:ty,
		atomic_ty: $atomic_ty:ty$(,)?
	} => {
		/// This implementation uses the same format as the atomic's primitive counterpart.
		/// The atomic object itself is read with the [`Relaxed`](core::sync::atomic::Ordering) ordering scheme.
		#[cfg(target_has_atomic = $width)]
		#[cfg_attr(doc, doc(cfg(target_has_atomic = $width)))]
		impl ::bzipper::Encode for $atomic_ty {
			#[inline(always)]
			fn encode(&self, stream: &mut ::bzipper::OStream) -> ::core::result::Result<(), ::bzipper::error::EncodeError> {
				self.load(::std::sync::atomic::Ordering::Relaxed).encode(stream)
			}
		}
	};
}

//impl_numeric!(f128);
//impl_numeric!(f16);
impl_numeric!(f32);
impl_numeric!(f64);
impl_numeric!(i128);
impl_numeric!(i16);
impl_numeric!(i32);
impl_numeric!(i64);
impl_numeric!(i8);
impl_numeric!(u128);
impl_numeric!(u16);
impl_numeric!(u32);
impl_numeric!(u64);
impl_numeric!(u8);

impl_tuple! {
	value0: T0,
	value1: T1,
}

impl_tuple! {
	value0: T0,
	value1: T1,
	value2: T2,
}

impl_tuple! {
	value0: T0,
	value1: T1,
	value2: T2,
	value3: T3,
}

impl_tuple! {
	value0: T0,
	value1: T1,
	value2: T2,
	value3: T3,
	value4: T4,
}

impl_tuple! {
	value0: T0,
	value1: T1,
	value2: T2,
	value3: T3,
	value4: T4,
	value5: T5,
}

impl_tuple! {
	value0: T0,
	value1: T1,
	value2: T2,
	value3: T3,
	value4: T4,
	value5: T5,
	value6: T6,
}

impl_tuple! {
	value0: T0,
	value1: T1,
	value2: T2,
	value3: T3,
	value4: T4,
	value5: T5,
	value6: T6,
	value7: T7,
}

impl_tuple! {
	value0: T0,
	value1: T1,
	value2: T2,
	value3: T3,
	value4: T4,
	value5: T5,
	value6: T6,
	value7: T7,
	value8: T8,
}

impl_tuple! {
	value0: T0,
	value1: T1,
	value2: T2,
	value3: T3,
	value4: T4,
	value5: T5,
	value6: T6,
	value7: T7,
	value8: T8,
	value9: T9,
}

impl_tuple! {
	value0:  T0,
	value1:  T1,
	value2:  T2,
	value3:  T3,
	value4:  T4,
	value5:  T5,
	value6:  T6,
	value7:  T7,
	value8:  T8,
	value9:  T9,
	value10: T10,
}

impl_tuple! {
	value0:  T0,
	value1:  T1,
	value2:  T2,
	value3:  T3,
	value4:  T4,
	value5:  T5,
	value6:  T6,
	value7:  T7,
	value8:  T8,
	value9:  T9,
	value10: T10,
	value11: T11,
}

impl_non_zero!(i128);
impl_non_zero!(i16);
impl_non_zero!(i32);
impl_non_zero!(i64);
impl_non_zero!(i8);
impl_non_zero!(isize);
impl_non_zero!(u128);
impl_non_zero!(u16);
impl_non_zero!(u32);
impl_non_zero!(u64);
impl_non_zero!(u8);
impl_non_zero!(usize);

impl_atomic! {
	width: "8",
	ty: bool,
	atomic_ty: std::sync::atomic::AtomicBool,
}

impl_atomic! {
	width: "16",
	ty: i16,
	atomic_ty: std::sync::atomic::AtomicI16,
}

impl_atomic! {
	width: "32",
	ty: i32,
	atomic_ty: std::sync::atomic::AtomicI32,
}

impl_atomic! {
	width: "64",
	ty: i64,
	atomic_ty: std::sync::atomic::AtomicI64,
}

impl_atomic! {
	width: "8",
	ty: i8,
	atomic_ty: std::sync::atomic::AtomicI8,
}

impl_atomic! {
	width: "ptr",
	ty: isize,
	atomic_ty: std::sync::atomic::AtomicIsize,
}

impl_atomic! {
	width: "16",
	ty: u16,
	atomic_ty: std::sync::atomic::AtomicU16,
}

impl_atomic! {
	width: "32",
	ty: u32,
	atomic_ty: std::sync::atomic::AtomicU32,
}

impl_atomic! {
	width: "64",
	ty: u64,
	atomic_ty: std::sync::atomic::AtomicU64,
}

impl_atomic! {
	width: "8",
	ty: u8,
	atomic_ty: std::sync::atomic::AtomicU8,
}

impl_atomic! {
	width: "ptr",
	ty: usize,
	atomic_ty: std::sync::atomic::AtomicUsize,
}
