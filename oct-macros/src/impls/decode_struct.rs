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
use quote::quote;
use syn::{DataStruct, Fields};
use std::iter;

#[must_use]
pub fn decode_struct(data: DataStruct) -> TokenStream {
	let commands = iter::repeat_n(
		quote! {
			::oct::decode::Decode::decode(stream)
				.map_err(::core::convert::Into::<::oct::error::GenericDecodeError>::into)?
		},
		data.fields.len(),
	);

	let value = match data.fields {
		Fields::Unit => quote! { Self },

		Fields::Unnamed(_fields) => quote! { Self (#(#commands, )*) },

		Fields::Named(fields) => {
			let field_names = fields
				.named
				.into_iter()
				.map(|field| field.ident.unwrap());

			quote! { Self { #(#field_names: #commands, )* } }
		},
	};

	quote! {
		type Error = ::oct::error::GenericDecodeError;

		#[inline]
		fn decode(stream: &mut ::oct::decode::Input) -> ::core::result::Result<Self, Self::Error> {
			let this = #value;
			::core::result::Result::Ok(this)
		}
	}
}
