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

use proc_macro2::TokenStream;
use quote::ToTokens;
use std::fmt;
use std::fmt::{Debug, Formatter};
use syn::{
	GenericParam,
	Generics,
	Ident,
	Lifetime,
	Token,
	punctuated::Punctuated,
};

/// A name of a genric.
#[derive(Clone)]
pub enum GenericName {
	/// Denotes a generic constant.
	Const(Ident),

	/// Denotes a generic lifetime.
	Lifetime(Lifetime),

	/// Denotes a generic type.
	Ty(Ident),
}

impl GenericName {
	/// Extracts the names of the given generics.
	#[must_use]
	pub fn extract_from(generics: &Generics) -> Punctuated<Self, Token![,]> {
		let mut names = Punctuated::new();

		for generic in &generics.params {
			let name = match *generic {
				GenericParam::Const(   ref param) => Self::Const(   param.ident.clone()),
				GenericParam::Lifetime(ref param) => Self::Lifetime(param.lifetime.clone()),
				GenericParam::Type(    ref param) => Self::Ty(    param.ident.clone()),
			};

			names.push(name);
		}

		names
	}
}

impl Debug for GenericName {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		let ident = match *self {
			| Self::Const(ref ident)
			| Self::Lifetime(Lifetime { ref ident, .. })
			| Self::Ty(ref ident)
			=> ident,
		};

		Debug::fmt(ident, f)
	}
}

impl ToTokens for GenericName {
	#[inline(always)]
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match *self {
			| Self::Const(ref ident)
			| Self::Ty( ref ident)
			=> ident.to_tokens(tokens),

			Self::Lifetime(ref lifetime) => lifetime.to_tokens(tokens),
		}
	}
}
