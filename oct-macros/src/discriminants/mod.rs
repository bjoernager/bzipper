// Copyright 2024 Gabriel Bjørnager Jensen.
//
// This file is part of oct.
//
// oct is free software: you can redistribute it
// and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// oct is distributed in the hope that it will
// be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Less-
// er General Public License along with oct. If
// not, see <https://www.gnu.org/licenses/>.

use std::borrow::Borrow;
use proc_macro2::Span;
use syn::{Expr, Lit, LitInt, Variant};

pub struct Discriminants<I: IntoIterator<Item: Borrow<Variant>>> {
	variants: I::IntoIter,
	prev: Option<u128>,
}

impl<I: IntoIterator<Item: Borrow<Variant>>> Discriminants<I> {
	#[inline(always)]
	#[must_use]
	pub fn new(variants: I) -> Self {
		Self {
			variants: variants.into_iter(),
			prev: None,
		}
	}
}

impl<I: IntoIterator<Item: Borrow<Variant>>> Iterator for Discriminants<I> {
	type Item = LitInt;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		let variant = self.variants.next()?;

		let discriminant = if let Some((_, ref expr)) = variant.borrow().discriminant {
			let Expr::Lit(ref expr) = *expr else {
				panic!("expected literal expression for discriminant value");
			};

			let Lit::Int(ref expr) = expr.lit else {
				panic!("expected (potentially signed) integer literal for discriminant value`");
			};

			let expr = expr.base10_digits();

			let value: u128 = expr
				.parse()
				.or_else(|_| expr.parse::<i128>().map(|v| v as u128))
				.unwrap();

			value
		} else if let Some(prev) = self.prev {
			prev
				.checked_add(0x1)
				.unwrap_or_else(|| panic!("overflow following discriminant `{prev:?}`"))
		} else {
			Default::default()
		};

		self.prev = Some(discriminant);

		let discriminant = LitInt::new(&discriminant.to_string(), Span::call_site());

		Some(discriminant)
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.variants.size_hint()
	}
}
