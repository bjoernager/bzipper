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

use crate::{
	Decode,
	Encode,
	IStream,
	OStream,
	SizedEncode,
	SizedIter,
};
use crate::error::{DecodeError, EncodeError, SizeError};

use core::borrow::{Borrow, BorrowMut};
use core::cmp::Ordering;
use core::fmt::{self, Debug, Formatter};
use core::hash::{Hash, Hasher};
use core::mem::MaybeUninit;
use core::ops::{Deref, DerefMut, Index, IndexMut};
use core::ptr::copy_nonoverlapping;
use core::slice;
use core::slice::{Iter, IterMut, SliceIndex};

#[cfg(feature = "alloc")]
use alloc::alloc::{alloc, Layout};

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Stack-allocated vector with maximum length.
///
/// This type is intended as an [sized-encodable](SizedEncode) alternative to [`Vec`] for cases where [arrays](array) may not be wanted.
///
/// Note that this type is immutable in the sense that it does **not** define methods like `push` and `pop`, unlike `Vec`.
///
/// See [`SizedStr`](crate::SizedStr) for an equivalent alternative to [`String`](alloc::string::String).
///
/// # Examples
///
/// All instances of this type with the same `T` and `N` also have the exact same layout:
///
/// ```
/// use bzipper::SizedSlice;
///
/// let vec0 = SizedSlice::<u8, 0x4>::try_from([0x3].as_slice()).unwrap();
/// let vec1 = SizedSlice::<u8, 0x4>::try_from([0x3, 0x2].as_slice()).unwrap();
/// let vec2 = SizedSlice::<u8, 0x4>::try_from([0x3, 0x2, 0x4].as_slice()).unwrap();
/// let vec3 = SizedSlice::<u8, 0x4>::try_from([0x3, 0x2, 0x4, 0x3].as_slice()).unwrap();
///
/// assert_eq!(size_of_val(&vec0), size_of_val(&vec1));
/// assert_eq!(size_of_val(&vec0), size_of_val(&vec2));
/// assert_eq!(size_of_val(&vec0), size_of_val(&vec3));
/// assert_eq!(size_of_val(&vec1), size_of_val(&vec2));
/// assert_eq!(size_of_val(&vec1), size_of_val(&vec3));
/// assert_eq!(size_of_val(&vec2), size_of_val(&vec3));
/// ```
pub struct SizedSlice<T, const N: usize> {
	buf: [MaybeUninit<T>; N],
	len: usize,
}

impl<T, const N: usize> SizedSlice<T, N> {
	/// Constructs a fixed-size vector from raw parts.
	///
	/// The provided parts are not tested in any way.
	///
	/// # Safety
	///
	/// The value of `len` may not exceed that of `N`.
	/// Additionally, all elements of `buf` in the range specified by `len` must be initialised.
	///
	/// If any of these requirements are violated, behaviour is undefined.
	#[inline(always)]
	#[must_use]
	pub const unsafe fn from_raw_parts(buf: [MaybeUninit<T>; N], len: usize) -> Self {
		debug_assert!(len <= N, "cannot construct vector longer than its capacity");

		Self { buf, len }
	}

	/// Sets the length of the vector.
	///
	/// The provided length is not tested in any way.
	///
	/// # Safety
	///
	/// The new length `len` may not be larger than `N`.
	///
	/// It is only valid to enlarge vectors if `T` supports being in a purely uninitialised state.
	/// Such is permitted with e.g. [`MaybeUninit`].
	#[inline(always)]
	pub const unsafe fn set_len(&mut self, len: usize) {
		debug_assert!(len <= N, "cannot set length past bounds");

		self.len = len
	}

	/// Gets a pointer to the first element.
	///
	/// The pointed-to element may not necessarily be initialised.
	/// See [`len`](Self::len) for more information.
	#[inline(always)]
	#[must_use]
	pub const fn as_ptr(&self) -> *const T {
		self.buf.as_ptr().cast()
	}

	/// Gets a mutable pointer to the first element.
	///
	/// The pointed-to element may not necessarily be initialised.
	/// See [`len`](Self::len) for more information.
	#[inline(always)]
	#[must_use]
	pub const fn as_mut_ptr(&mut self) -> *mut T {
		self.buf.as_mut_ptr().cast()
	}

	/// Borrows the vector as a slice.
	///
	/// The range of the returned slice only includes the elements specified by [`len`](Self::len).
	#[inline(always)]
	#[must_use]
	pub const fn as_slice(&self) -> &[T] {
		let ptr = self.as_ptr();
		let len = self.len();

		unsafe { slice::from_raw_parts(ptr, len) }
	}

	/// Borrows the vector as a mutable slice.
	///
	/// The range of the returned slice only includes the elements specified by [`len`](Self::len).
	#[inline(always)]
	#[must_use]
	pub const fn as_mut_slice(&mut self) -> &mut [T] {
		let ptr = self.as_mut_ptr();
		let len = self.len();

		unsafe { slice::from_raw_parts_mut(ptr, len) }
	}

	/// Returns the total capacity of the vector.
	///
	/// By definition, this is always exactly equal to the value of `N`.
	#[expect(clippy::unused_self)]
	#[inline(always)]
	#[must_use]
	pub const fn capacity(&self) -> usize {
		N
	}

	/// Returns the length of the vector.
	///
	/// This value may necessarily be smaller than `N`.
	#[inline(always)]
	#[must_use]
	pub const fn len(&self) -> usize {
		self.len
	}

	/// Checks if the vector is empty, i.e. no elements are recorded.
	///
	/// Note that the internal buffer may still contain objects that have been "shadowed" by setting a smaller length with [`len`](Self::len).
	#[inline(always)]
	#[must_use]
	pub const fn is_empty(&self) -> bool {
		self.len() == 0x0
	}

	/// Checks if the vector is full, i.e. it cannot hold any more elements.
	#[inline(always)]
	#[must_use]
	pub const fn is_full(&self) -> bool {
		self.len() == self.capacity()
	}

	/// Destructs the vector into its raw parts.
	///
	/// The returned values are valid to pass on to [`from_raw_parts`](Self::from_raw_parts).
	#[inline(always)]
	#[must_use]
	pub const fn into_raw_parts(self) -> ([MaybeUninit<T>; N], usize) {
		let Self { buf, len } = self;
		(buf, len)
	}

	/// Converts the vector into a boxed slice.
	///
	/// The vector is reallocated using the global allocator.
	#[cfg(feature = "alloc")]
	#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
	#[must_use]
	pub fn into_boxed_slice(self) -> Box<[T]> {
		let (buf, len) = self.into_raw_parts();

		unsafe {
			let layout = Layout::array::<T>(len).unwrap();
			let ptr = alloc(layout).cast::<T>();

			assert!(!ptr.is_null(), "allocation failed");

			copy_nonoverlapping(buf.as_ptr().cast(), ptr, len);

			let slice = core::ptr::slice_from_raw_parts_mut(ptr, len);
			Box::from_raw(slice)

			// `self.buf` is dropped without destructors being
			// run.
		}
	}

	/// Converts the vector into a dynamic vector.
	///
	/// The vector is reallocated using the global allocator.
	#[cfg(feature = "alloc")]
	#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
	#[inline(always)]
	#[must_use]
	pub fn into_vec(self) -> Vec<T> {
		self.into_boxed_slice().into_vec()
	}
}

impl<T: Clone, const N: usize> SizedSlice<T, N> {
	/// Constructs an empty, fixed-size vector.
	#[inline]
	pub fn new(data: &[T]) -> Result<Self, SizeError> {
		let mut buf: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

		let len = data.len();
		if len > N { return Err(SizeError { req: len, len: N }) };

		for (item, value) in buf.iter_mut().zip(data.iter()) {
			item.write(value.clone());
		}

		Ok(Self { buf, len })
	}
}

impl<T, const N: usize> AsMut<[T]> for SizedSlice<T, N> {
	#[inline(always)]
	fn as_mut(&mut self) -> &mut [T] {
		self.as_mut_slice()
	}
}

impl<T, const N: usize> AsRef<[T]> for SizedSlice<T, N> {
	#[inline(always)]
	fn as_ref(&self) -> &[T] {
		self.as_slice()
	}
}

impl<T, const N: usize> Borrow<[T]> for SizedSlice<T, N> {
	#[inline(always)]
	fn borrow(&self) -> &[T] {
		self.as_slice()
	}
}

impl<T, const N: usize> BorrowMut<[T]> for SizedSlice<T, N> {
	#[inline(always)]
	fn borrow_mut(&mut self) -> &mut [T] {
		self.as_mut_slice()
	}
}

impl<T: Clone, const N: usize> Clone for SizedSlice<T, N> {
	#[inline]
	fn clone(&self) -> Self {
		unsafe {
			let mut buf: [MaybeUninit<T>; N] = MaybeUninit::uninit().assume_init();

			for i in 0x0..self.len() {
				let value = self.get_unchecked(i).clone();
				buf.get_unchecked_mut(i).write(value);
			}

			Self { buf, len: self.len }
		}
	}
}

impl<T: Debug, const N: usize> Debug for SizedSlice<T, N> {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Debug::fmt(self.as_slice(), f)
	}
}

impl<T, const N: usize> Default for SizedSlice<T, N> {
	#[inline(always)]
	fn default() -> Self {
		Self { buf: unsafe { MaybeUninit::uninit().assume_init() }, len: 0x0 }
	}
}

impl<T, const N: usize> Deref for SizedSlice<T, N> {
	type Target = [T];

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		self.as_slice()
	}
}

impl<T, const N: usize> DerefMut for SizedSlice<T, N> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.as_mut_slice()
	}
}

impl<T: Decode, const N: usize> Decode for SizedSlice<T, N> {
	#[inline]
	fn decode(stream: &mut IStream) -> Result<Self, DecodeError> {
		let len = Decode::decode(stream)?;
		if len > N { return Err(DecodeError::SmallBuffer(SizeError { req: len, len: N })) };

		let mut buf: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

		for item in &mut buf {
			let value = Decode::decode(stream)?;

			item.write(value);
		}

		Ok(Self { buf, len })
	}
}

impl<T: Encode, const N: usize> Encode for SizedSlice<T, N> {
	#[inline(always)]
	fn encode(&self, stream: &mut OStream) -> Result<(), EncodeError> {
		self.as_slice().encode(stream)
	}
}

impl<T: Eq, const N: usize> Eq for SizedSlice<T, N> { }

impl<T, const N: usize> From<[T; N]> for SizedSlice<T, N> {
	#[inline(always)]
	fn from(value: [T; N]) -> Self {
		unsafe {
			let buf = value.as_ptr().cast::<[MaybeUninit<T>; N]>().read();

			Self { buf, len: N }
		}
	}
}

impl<T, const N: usize> FromIterator<T> for SizedSlice<T, N> {
	#[inline]
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		let mut iter = iter.into_iter();

		let mut buf: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };
		let mut len = 0x0;

		for item in &mut buf {
			let Some(value) = iter.next() else { break };
			item.write(value);

			len += 0x1;
		}

		Self { buf, len }
	}
}

impl<T: Hash, const N: usize> Hash for SizedSlice<T, N> {
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H) {
		for v in self {
			v.hash(state);
		}
	}
}

impl<T, I: SliceIndex<[T]>, const N: usize> Index<I> for SizedSlice<T, N> {
	type Output	= I::Output;

	#[inline(always)]
	fn index(&self, index: I) -> &Self::Output {
		self.get(index).unwrap()
	}
}

impl<T, I: SliceIndex<[T]>, const N: usize> IndexMut<I> for SizedSlice<T, N> {
	#[inline(always)]
	fn index_mut(&mut self, index: I) -> &mut Self::Output {
		self.get_mut(index).unwrap()
	}
}

impl<T, const N: usize> IntoIterator for SizedSlice<T, N> {
	type Item = T;

	type IntoIter = SizedIter<T, N>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		let Self { buf, len } = self;

		unsafe { SizedIter::new(buf, len) }
	}
}

impl<'a, T, const N: usize> IntoIterator for &'a SizedSlice<T, N> {
	type Item = &'a T;

	type IntoIter = Iter<'a, T>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<'a, T, const N: usize> IntoIterator for &'a mut SizedSlice<T, N> {
	type Item = &'a mut T;

	type IntoIter = IterMut<'a, T>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		self.iter_mut()
	}
}

impl<T: Ord, const N: usize> Ord for SizedSlice<T, N> {
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_slice().cmp(other.as_slice())
	}
}

impl<T: PartialEq<U>, U: PartialEq<T>, const N: usize, const M: usize> PartialEq<SizedSlice<U, M>> for SizedSlice<T, N> {
	#[inline(always)]
	fn eq(&self, other: &SizedSlice<U, M>) -> bool {
		self.as_slice() == other.as_slice()
	}
}

impl<T: PartialEq<U>, U: PartialEq<T>, const N: usize, const M: usize> PartialEq<[U; M]> for SizedSlice<T, N> {
	#[inline(always)]
	fn eq(&self, other: &[U; M]) -> bool {
		self.as_slice() == other.as_slice()
	}
}

impl<T: PartialEq<U>, U: PartialEq<T>, const N: usize> PartialEq<&[U]> for SizedSlice<T, N> {
	#[inline(always)]
	fn eq(&self, other: &&[U]) -> bool {
		self.as_slice() == *other
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: PartialEq<U>, U: PartialEq<T>, const N: usize> PartialEq<Vec<U>> for SizedSlice<T, N> {
	#[inline(always)]
	fn eq(&self, other: &Vec<U>) -> bool {
		self.as_slice() == other.as_slice()
	}
}

impl<T: PartialOrd, const N: usize, const M: usize> PartialOrd<SizedSlice<T, M>> for SizedSlice<T, N> {
	#[inline(always)]
	fn partial_cmp(&self, other: &SizedSlice<T, M>) -> Option<Ordering> {
		self.as_slice().partial_cmp(other.as_slice())
	}
}

impl<T: PartialOrd, const N: usize, const M: usize> PartialOrd<[T; M]> for SizedSlice<T, N> {
	#[inline(always)]
	fn partial_cmp(&self, other: &[T; M]) -> Option<Ordering> {
		self.as_slice().partial_cmp(other.as_slice())
	}
}

impl<T: PartialOrd, const N: usize> PartialOrd<&[T]> for SizedSlice<T, N> {
	#[inline(always)]
	fn partial_cmp(&self, other: &&[T]) -> Option<Ordering> {
		self.as_slice().partial_cmp(*other)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: PartialOrd, const N: usize> PartialOrd<Vec<T>> for SizedSlice<T, N> {
	#[inline(always)]
	fn partial_cmp(&self, other: &Vec<T>) -> Option<Ordering> {
		self.as_slice().partial_cmp(other.as_slice())
	}
}

unsafe impl<T: SizedEncode, const N: usize> SizedEncode for SizedSlice<T, N> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE * N;
}

impl<T: Clone, const N: usize> TryFrom<&[T]> for SizedSlice<T, N> {
	type Error = SizeError;

	#[inline(always)]
	fn try_from(value: &[T]) -> Result<Self, Self::Error> {
		Self::new(value)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T, const N: usize> From<SizedSlice<T, N>> for Box<[T]> {
	#[inline(always)]
	fn from(value: SizedSlice<T, N>) -> Self {
		value.into_boxed_slice()
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T, const N: usize> From<SizedSlice<T, N>> for Vec<T> {
	#[inline(always)]
	fn from(value: SizedSlice<T, N>) -> Self {
		value.into_vec()
	}
}
