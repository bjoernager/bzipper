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
use syn::{Ident, Token};

/// A field capture list.
///
/// This is used for capturing fields of structures or enumeration variants.
#[derive(Clone)]
pub struct Capture {
	pub ref_token: Token![ref],
	pub ident:     Ident,
}

impl ToTokens for Capture {
	#[inline(always)]
	fn to_tokens(&self, tokens: &mut TokenStream) {
		self.ref_token.to_tokens(tokens);
		self.ident.to_tokens(tokens);
	}
}
