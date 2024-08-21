// Copyright 2024 Gabriel Bjørnager Jensen.
//
// This file is part of bzipper.
//
// bzipper is free software: you can redistribute
// it and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// bzipper is distributed in the hope that it will
// be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Less-
// er General Public License along with bzipper. If
// not, see <https://www.gnu.org/licenses/>.

use proc_macro2::TokenStream;
use quote::ToTokens;

/// An enumeration discriminant.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Discriminant(u32);

impl Discriminant {
	/// Constructs a new discriminant.
	#[inline(always)]
	#[must_use]
	pub const fn new(value: u32) -> Self { Self(value) }

	/// Retrieves the raw discriminant value.
	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> u32 { self.0 }

	/// Unwraps the given value as a discriminant.
	///
	/// # Panics
	///
	/// If the given value cannot be represented as an `u32`, this function will panic.
	#[inline(always)]
	#[must_use]
	pub fn unwrap_from<T: TryInto<Self>>(value: T) -> Self {
		value
			.try_into()
			.unwrap_or_else(|_| panic!("enumeration discriminants must be representable in `u32`"))
	}

	/// Unsafely unwraps the given value as a discriminant.
	///
	/// This function assumes that this conversion is infallible for the given value.
	/// If this is a false guarantee, the [`unwrap_from`](Self::unwrap_from) function should be used instead.
	///
	/// # Safety
	///
	/// Behaviour is undefined if the given value cannot be represented as an object of `u32`.
	#[inline(always)]
	#[must_use]
	pub unsafe fn unwrap_from_unchecked<T: TryInto<Self>>(value: T) -> Self {
		value
			.try_into()
			.unwrap_unchecked()
	}
}

impl ToTokens for Discriminant {
	#[inline(always)]
	fn to_tokens(&self, tokens: &mut TokenStream) { self.0.to_tokens(tokens) }
}

impl From<u32> for Discriminant {
	#[inline(always)]
	fn from(value: u32) -> Self { Self(value) }
}

impl TryFrom<usize> for Discriminant {
	type Error = <u32 as TryFrom<usize>>::Error;

	#[inline(always)]
	fn try_from(value: usize) -> Result<Self, Self::Error> { value.try_into().map(Self) }
}

impl From<Discriminant> for u32 {
	#[inline(always)]
	fn from(value: Discriminant) -> Self { value.0 }
}

impl TryFrom<Discriminant> for usize {
	type Error = <Self as TryFrom<u32>>::Error;

	#[inline(always)]
	fn try_from(value: Discriminant) -> Result<Self, Self::Error> { value.0.try_into() }
}
