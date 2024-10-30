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

use crate::Discriminant;

use std::borrow::Borrow;
use syn::Variant;

pub struct DiscriminantIter<I: IntoIterator<Item: Borrow<Variant>>> {
	variants: I::IntoIter,
	prev:     Option<Discriminant>,
}

impl<I: IntoIterator<Item: Borrow<Variant>>> DiscriminantIter<I> {
	#[inline(always)]
	#[must_use]
	pub fn new(variants: I) -> Self {
		Self {
			variants: variants.into_iter(),
			prev: None,
		}
	}
}

impl<I: IntoIterator<Item: Borrow<Variant>>> Iterator for DiscriminantIter<I> {
	type Item = (Discriminant, I::Item);

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		let variant = self.variants.next()?;

		let discriminant = if let Some((_, ref discriminant)) = variant.borrow().discriminant {
			Discriminant::parse(discriminant)
		} else if let Some(discriminant) = self.prev {
			let value = discriminant.0
				.checked_add(0x1)
				.unwrap_or_else(|| panic!("overflow following discriminant `{discriminant:?}`"));

			Discriminant(value)
		} else {
			Default::default()
		};

		self.prev = Some(discriminant);

		Some((discriminant, variant))
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.variants.size_hint()
	}
}
