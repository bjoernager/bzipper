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

#![doc(html_logo_url = "https://gitlab.com/bjoernager/bzipper/-/raw/master/doc-icon.svg")]

//! This crate implements procedural macros for [`bZipper`](https://crates.io/crates/bzipper/).

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

macro_rules! use_mod {
	($vis:vis $name:ident) => {
		mod $name;
		$vis use $name::*;
	};
}
pub(crate) use use_mod;

use_mod!(discriminant);
use_mod!(discriminant_iter);
use_mod!(generic_name);

mod impls;

#[proc_macro_derive(Decode)]
pub fn derive_decode(input: TokenStream) -> TokenStream {
	let input = syn::parse_macro_input!(input as DeriveInput);

	let impl_body = match input.data {
		Data::Enum(  ref data) => impls::decode_enum(  data),
		Data::Struct(ref data) => impls::decode_struct(data),

		Data::Union(..) => panic!("unions cannot derive `Decode`"),
	};

	let ty_name = &input.ident;

	let generic_params = &input.generics.params;
	let generic_where  = &input.generics.where_clause;

	let generic_names = GenericName::extract_from(&input.generics);

	let output = quote! {
		impl<#generic_params> ::bzipper::Decode for #ty_name<#generic_names>
		#generic_where {
			#impl_body
		}
	};

	//panic!("{output}");

	output.into()
}

#[proc_macro_derive(Encode)]
pub fn derive_encode(input: TokenStream) -> TokenStream {
	let input = syn::parse_macro_input!(input as DeriveInput);

	let impl_body = match input.data {
		Data::Enum(  ref data) => impls::encode_enum(  data),
		Data::Struct(ref data) => impls::encode_struct(data),

		Data::Union(..) => panic!("unions cannot derive `Encode`"),
	};

	let ty_name = &input.ident;

	let generic_params = &input.generics.params;
	let generic_where  = &input.generics.where_clause;

	let generic_names = GenericName::extract_from(&input.generics);

	let output = quote! {
		impl<#generic_params> ::bzipper::Encode for #ty_name<#generic_names>
		#generic_where {
			#impl_body
		}
	};

	//panic!("{output}");

	output.into()
}

#[proc_macro_derive(SizedEncode)]
pub fn derive_sized_encode(input: TokenStream) -> TokenStream {
	let input = syn::parse_macro_input!(input as DeriveInput);

	let encode_impl_body = match input.data {
		Data::Enum(  ref data) => impls::encode_enum(  data),
		Data::Struct(ref data) => impls::encode_struct(data),

		Data::Union(..) => panic!("unions can neither derive `Encode` nor `SizedEncode`"),
	};

	let sized_encode_impl_body = match input.data {
		Data::Enum(  ref data) => impls::sized_encode_enum(  data),
		Data::Struct(ref data) => impls::sized_encode_struct(data),

		Data::Union(..) => unreachable!(),
	};

	let ty_name = &input.ident;

	let generic_params = &input.generics.params;
	let generic_where  = &input.generics.where_clause;

	let generic_names = GenericName::extract_from(&input.generics);

	let output = quote! {
		impl<#generic_params> ::bzipper::Encode for #ty_name<#generic_names>
		#generic_where {
			#encode_impl_body
		}

		unsafe impl<#generic_params> ::bzipper::SizedEncode for #ty_name<#generic_names>
		#generic_where {
			#sized_encode_impl_body
		}
	};

	//panic!("{output}");

	output.into()
}
