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

#![doc(html_logo_url = "https://gitlab.com/bjoernager/bzipper/-/raw/master/doc-icon.svg?ref_type=heads")]

//! Binary (de)serialisation.
//!
//! This crate implements macros for the [`bzipper`](https://crates.io/crates/bzipper/) crate.

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

macro_rules! use_mod {
	($vis:vis $name:ident) => {
		mod $name;
		$vis use $name::*;
	};
}
pub(in crate) use use_mod;

use_mod!(closure);
use_mod!(discriminant);
use_mod!(generic_name);

mod impls;

#[proc_macro_derive(Deserialise)]
pub fn derive_deserialise(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let impl_body = match input.data {
		Data::Enum(  ref data) => impls::deserialise_enum(  data),
		Data::Struct(ref data) => impls::deserialise_struct(data),

		Data::Union(..) => panic!("unions cannot derive `Deserialise`"),
	};

	let type_name = &input.ident;

	let generic_params = &input.generics.params;
	let generic_where  = &input.generics.where_clause;

	let generic_names = GenericName::extract_from(&input.generics);

	let output = quote! {
		impl<#generic_params> ::bzipper::Deserialise for #type_name<#generic_names>
		#generic_where {
			#impl_body
		}
	};

	output.into()
}

#[proc_macro_derive(Serialise)]
pub fn derive_serialise(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let impl_body = match input.data {
		Data::Enum(  ref data) => impls::serialise_enum(  data),
		Data::Struct(ref data) => impls::serialise_struct(data),

		Data::Union(..) => panic!("unions cannot derive `Serialise`"),
	};

	let type_name = &input.ident;

	let generic_params = &input.generics.params;
	let generic_where  = &input.generics.where_clause;

	let generic_names = GenericName::extract_from(&input.generics);

	let output = quote! {
		impl<#generic_params> ::bzipper::Serialise for #type_name<#generic_names>
		#generic_where {
			#impl_body
		}
	};

	//if let Data::Enum(..) = input.data { panic!("{output}") };

	output.into()
}
