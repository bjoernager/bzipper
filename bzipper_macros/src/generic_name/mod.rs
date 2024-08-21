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
	Const(   Ident),
	Lifetime(Lifetime),
	Type(    Ident),
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
				GenericParam::Type(    ref param) => Self::Type(    param.ident.clone()),
			};

			names.push(name);
		}

		names
		}
}

impl ToTokens for GenericName {
	#[inline(always)]
	fn to_tokens(&self, tokens: &mut TokenStream) {
		use GenericName::*;

		match *self {
			| Const(ref ident)
			| Type( ref ident)
			=> ident.to_tokens(tokens),

			Lifetime(ref lifetime) => lifetime.to_tokens(tokens),
		}
	}
}
