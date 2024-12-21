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

#[cfg(test)]
mod tests;

use crate::error::LengthError;

use core::fmt::{self, Debug, Formatter};
use core::hash::{Hash, Hasher};
use core::mem::MaybeUninit;
use core::ops::{Index, IndexMut};
use core::ptr::{copy_nonoverlapping, null, null_mut};
use core::slice::SliceIndex;

// Encode/decode facilities:
mod code;

// Conversion facilities:
mod conv;

// Comparison facilities:
mod cmp;

// Iterator facilities:
mod iter;

/// Stack-allocated vector with maximum length.
///
/// This type is intended as a [sized-encodable](crate::encode::SizedEncode) alternative to [`Vec`](alloc::vec::Vec) -- for cases where [arrays](array) may not be wanted -- as well as a [decodable](crate::decode::Decode) alternative to normal [slices](slice).
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
/// use oct::SizedSlice;
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
	/// Copies elements from a slice.
	#[inline]
	pub const fn copy_from_slice(&mut self, data: &[T])
	where
		T: Copy,
	{
		unsafe {
			let src  = data.as_ptr();
			let dst   = self.buf.as_mut_ptr().cast();
			let count = data.len();

			// SAFETY: `T` implements `Copy`.
			copy_nonoverlapping(src, dst, count);

			self.set_len(count);
		}
	}

	/// Generates a sized slice referencing the elements of `self`.
	#[inline]
	pub const fn each_ref(&self) -> SizedSlice<&T, N> {
		let mut buf = [null::<T>(); N];
		let     len = self.len;

		let mut i = 0x0;
		while i < len {
			unsafe {
				let item = buf.as_mut_ptr().add(i);

				let value = self.as_ptr().add(i).cast();
				item.write(value);
			}

			i += 0x1;
		}

		// SAFETY: `*const T` has the same layout as
		// `MaybeUninit<&T>`, and every relavent pointer
		// has been initialised as a valid reference.
		let buf = unsafe { (&raw const buf).cast::<[MaybeUninit<&T>; N]>().read() };

		unsafe { SizedSlice::from_raw_parts(buf, len) }
	}

	/// Generates a sized slice mutably referencing the elements of `self`.
	#[inline]
	pub const fn each_mut(&mut self) -> SizedSlice<&mut T, N> {
		let mut buf = [null_mut::<T>(); N];
		let     len = self.len;

		let mut i = 0x0;
		while i < len {
			unsafe {
				let item = buf.as_mut_ptr().add(i);

				let value = self.as_mut_ptr().add(i).cast();
				item.write(value);
			}

			i += 0x1;
		}

		// SAFETY: `*mut T` has the same layout as
		// `MaybeUninit<&mut T>`, and every relavent point-
		// er has been initialised as a valid reference.
		let buf = unsafe { (&raw const buf).cast::<[MaybeUninit<&mut T>; N]>().read() };

		unsafe { SizedSlice::from_raw_parts(buf, len) }
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

	/// Returns the total capacity of the vector.
	///
	/// By definition, this is always exactly equal to the value of `N`.
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
}

impl<T: Clone, const N: usize> SizedSlice<T, N> {
	/// Constructs an empty, fixed-size vector.
	#[inline]
	pub fn new(data: &[T]) -> Result<Self, LengthError> {
		let mut buf = [const { MaybeUninit::<T>::uninit() };N];

		let len = data.len();
		if len > N { return Err(LengthError { capacity: N, len }) };

		for (item, value) in buf.iter_mut().zip(data.iter()) {
			item.write(value.clone());
		}

		Ok(Self { buf, len })
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
		unsafe {
			// SAFETY: Always safe.
			let buf = MaybeUninit::uninit().assume_init();

			// SAFETY: The resulting slice is zero lengthed.
			Self::from_raw_parts(buf, 0x0)
		}
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
