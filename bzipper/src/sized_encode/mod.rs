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

use crate::Encode;

use core::cell::RefCell;
use core::convert::Infallible;
use core::marker::PhantomData;
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
use alloc::boxed::Box;

#[cfg(feature = "std")]
use std::rc::Rc;

#[cfg(feature = "std")]
use std::sync::{Arc, Mutex, RwLock};

mod tuple;

/// Denotes a size-constrained, encodable type.
///
/// When using [`Encode`], the size of the resulting encoding cannot always be known beforehand.
/// This trait defines an upper bound for these sizes.
///
/// # Safety
///
/// Users of the `Encode` and [`Decode`](crate::Decode) traits may assume that the [`MAX_ENCODED_SIZE`](Self::MAX_ENCODED_SIZE) constant is properly defined and that no encoding will be larger than this value.
/// Implementors must therefore guarantee that **no** call to [`encode`](Encode::encode) or [`decode`](bzipper::Decode::decode) consumes more bytes than specified by this constant.
pub unsafe trait SizedEncode: Encode + Sized {
	/// The maximum guaranteed amount of bytes that can result from an encoding.
	///
	/// Implementors of this trait should make sure that no encoding (or decoding) uses more than the amount specified by this constant.
	const MAX_ENCODED_SIZE: usize;
}

macro_rules! impl_numeric {
	($ty:ty$(,)?) => {
		unsafe impl ::bzipper::SizedEncode for $ty {
			const MAX_ENCODED_SIZE: usize = size_of::<$ty>();
		}
	};
}

macro_rules! impl_non_zero {
	($ty:ty$(,)?) => {
		unsafe impl ::bzipper::SizedEncode for ::core::num::NonZero<$ty> {
			const MAX_ENCODED_SIZE: usize = <$ty as ::bzipper::SizedEncode>::MAX_ENCODED_SIZE;
		}
	};
}

macro_rules! impl_atomic {
	{
		width: $width:literal,
		ty: $ty:ty,
		atomic_ty: $atomic_ty:ty$(,)?
	} => {
		#[cfg(target_has_atomic = $width)]
		#[cfg_attr(doc, doc(cfg(target_has_atomic = $width)))]
		unsafe impl ::bzipper::SizedEncode for $atomic_ty {
			const MAX_ENCODED_SIZE: usize = <$ty as ::bzipper::SizedEncode>::MAX_ENCODED_SIZE;
		}
	};
}

unsafe impl<T: SizedEncode, const N: usize> SizedEncode for [T; N] {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE * N;
}

#[cfg(feature = "std")]
#[cfg_attr(doc, doc(cfg(feature = "std")))]
unsafe impl<T: SizedEncode> SizedEncode for Arc<T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE;
}

unsafe impl SizedEncode for bool {
	const MAX_ENCODED_SIZE: usize = u8::MAX_ENCODED_SIZE;
}

unsafe impl<T: SizedEncode> SizedEncode for Bound<T> {
	const MAX_ENCODED_SIZE: usize = 0x0;
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
unsafe impl<T: SizedEncode> SizedEncode for Box<T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE;
}

unsafe impl SizedEncode for char {
	const MAX_ENCODED_SIZE: usize = u32::MAX_ENCODED_SIZE;

}

unsafe impl SizedEncode for Infallible {
	const MAX_ENCODED_SIZE: usize = 0x0;
}

unsafe impl SizedEncode for IpAddr {
	const MAX_ENCODED_SIZE: usize = u8::MAX_ENCODED_SIZE + Ipv6Addr::MAX_ENCODED_SIZE;
}

unsafe impl SizedEncode for Ipv4Addr {
	const MAX_ENCODED_SIZE: usize = u32::MAX_ENCODED_SIZE;
}

unsafe impl SizedEncode for Ipv6Addr {
	const MAX_ENCODED_SIZE: usize = u128::MAX_ENCODED_SIZE;
}

unsafe impl SizedEncode for isize {
	const MAX_ENCODED_SIZE: usize = i16::MAX_ENCODED_SIZE;
}

#[cfg(feature = "std")]
#[cfg_attr(doc, doc(cfg(feature = "std")))]
unsafe impl<T: SizedEncode> SizedEncode for Mutex<T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE;
}

unsafe impl<T: SizedEncode> SizedEncode for Option<T> {
	const MAX_ENCODED_SIZE: usize =
		bool::MAX_ENCODED_SIZE
		+ T::MAX_ENCODED_SIZE;
}

unsafe impl<T> SizedEncode for PhantomData<T> {
	const MAX_ENCODED_SIZE: usize = 0x0;
}

unsafe impl<T: SizedEncode> SizedEncode for Range<T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE * 0x2;
}

unsafe impl<T: SizedEncode> SizedEncode for RangeFrom<T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE;
}

unsafe impl SizedEncode for RangeFull {
	const MAX_ENCODED_SIZE: usize = 0x0;
}

unsafe impl<T: SizedEncode> SizedEncode for RangeInclusive<T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE * 0x2;
}

unsafe impl<T: SizedEncode> SizedEncode for RangeTo<T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE;
}

unsafe impl<T: SizedEncode> SizedEncode for RangeToInclusive<T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE;
}

#[cfg(feature = "std")]
#[cfg_attr(doc, doc(cfg(feature = "std")))]
unsafe impl<T: SizedEncode> SizedEncode for Rc<T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE;
}

unsafe impl<T: SizedEncode> SizedEncode for RefCell<T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE;
}

unsafe impl<T: SizedEncode, E: SizedEncode> SizedEncode for core::result::Result<T, E> {
	const MAX_ENCODED_SIZE: usize =
		bool::MAX_ENCODED_SIZE
		+ if size_of::<T>() > size_of::<E>() { size_of::<T>() } else { size_of::<E>() };
}

#[cfg(feature = "std")]
#[cfg_attr(doc, doc(cfg(feature = "std")))]
unsafe impl<T: SizedEncode> SizedEncode for RwLock<T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE;
}

unsafe impl<T: SizedEncode> SizedEncode for Saturating<T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE;
}

unsafe impl SizedEncode for SocketAddr {
	const MAX_ENCODED_SIZE: usize = u8::MAX_ENCODED_SIZE + SocketAddrV6::MAX_ENCODED_SIZE;
}

unsafe impl SizedEncode for SocketAddrV4 {
	const MAX_ENCODED_SIZE: usize = Ipv4Addr::MAX_ENCODED_SIZE + u16::MAX_ENCODED_SIZE;
}

/// This implementation encodes the address's bits followed by the port number, all of which in big-endian.
unsafe impl SizedEncode for SocketAddrV6 {
	const MAX_ENCODED_SIZE: usize =
		Ipv6Addr::MAX_ENCODED_SIZE
		+ u16::MAX_ENCODED_SIZE
		+ u32::MAX_ENCODED_SIZE
		+ u32::MAX_ENCODED_SIZE;
}

unsafe impl SizedEncode for () {
	const MAX_ENCODED_SIZE: usize = 0x0;
}

unsafe impl SizedEncode for usize {
	const MAX_ENCODED_SIZE: Self = u16::MAX_ENCODED_SIZE;
}

unsafe impl<T: SizedEncode> SizedEncode for Wrapping<T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE;
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
