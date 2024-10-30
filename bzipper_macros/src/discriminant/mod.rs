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

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Expr, Lit};

/// An enumeration discriminant.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Discriminant(pub isize);

impl Discriminant {
	/// Parses the expression as a discriminant value.
	///
	/// # Panics
	///
	/// This constructor will panic if the provided expression is not a valid `isize` literal.
	#[inline]
	#[must_use]
	pub fn parse(expr: &Expr) -> Self {
		let Expr::Lit(ref expr) = *expr else {
			panic!("expected literal expression for discriminant value");
		};

		let Lit::Int(ref expr) = expr.lit else {
			panic!("expected integer literal for discriminant value");
		};

		let value = expr.base10_parse::<isize>()
			.expect("expected `isize` literal for discriminant value");

		Self(value)
	}
}

impl ToTokens for Discriminant {
	#[inline(always)]
	fn to_tokens(&self, tokens: &mut TokenStream) {
		self.0.to_tokens(tokens);
	}
}
