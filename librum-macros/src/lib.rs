// Copyright 2024 Gabriel Bjørnager Jensen.
//
// This file is part of Librum.
//
// Librum is free software: you can redistribute it
// and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// Librum is distributed in the hope that it will
// be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Less-
// er General Public License along with Librum. If
// not, see <https://www.gnu.org/licenses/>.

#![doc(html_logo_url = "https://gitlab.com/bjoernager/librum/-/raw/master/doc-icon.svg")]

//! This crate implements procedural macros for [`Librum`](https://crates.io/crates/librum/).

// For use in macros:
extern crate self as librum_macros;

macro_rules! use_mod {
	($vis:vis $name:ident) => {
		mod $name;
		$vis use $name::*;
	};
}
pub(crate) use use_mod;

use_mod!(discriminants);
use_mod!(generic_name);
use_mod!(impl_derive_macro);
use_mod!(repr);

mod impls;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse2};

#[proc_macro_derive(Decode)]
pub fn derive_decode(input: TokenStream) -> TokenStream {
	let input = syn::parse_macro_input!(input as DeriveInput);

	let output = impl_derive_macro(
		input,
		parse2(quote! { ::librum::Decode }).unwrap(),
		None,
		impls::decode_struct,
		impls::decode_enum,
	);

	//panic!("{output}");

	output.into()
}

#[proc_macro_derive(Encode)]
pub fn derive_encode(input: TokenStream) -> TokenStream {
	let input = syn::parse_macro_input!(input as DeriveInput);

	let output = impl_derive_macro(
		input,
		parse2(quote! { ::librum::Encode }).unwrap(),
		None,
		impls::encode_struct,
		impls::encode_enum,
	);

	//panic!("{output}");

	output.into()
}

#[proc_macro_derive(SizedEncode)]
pub fn derive_sized_encode(input: TokenStream) -> TokenStream {
	let input = syn::parse_macro_input!(input as DeriveInput);

	let output = impl_derive_macro(
		input,
		parse2(quote! { ::librum::SizedEncode }).unwrap(),
		None,
		impls::sized_encode_struct,
		impls::sized_encode_enum,
	);

	//panic!("{output}");

	output.into()
}
