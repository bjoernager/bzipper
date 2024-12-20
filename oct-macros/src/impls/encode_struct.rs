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

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{DataStruct, Fields, Ident};

#[must_use]
pub fn encode_struct(data: DataStruct) -> TokenStream {
	let captures: Vec<_> = data
		.fields
		.iter()
		.enumerate()
		.map(|(index, _)| Ident::new(&format!("value{index}"), Span::call_site()))
		.collect();

	let pattern = match data.fields {
		Fields::Unit => quote! { Self },

		Fields::Unnamed(_fields) => quote! { Self(#(ref #captures, )*) },

		Fields::Named(fields) => {
			let field_names = fields
				.named
				.into_iter()
				.map(|field| field.ident.unwrap());

			quote! { Self { #(#field_names: ref #captures, )* } }
		},
	};

	quote! {
		type Error = ::oct::error::GenericEncodeError;

		#[inline]
		fn encode(&self, stream: &mut ::oct::encode::Output) -> ::core::result::Result<(), Self::Error> {
			let #pattern = self;

			#(
				::oct::encode::Encode::encode(#captures, stream)
					.map_err(::core::convert::Into::<::oct::error::GenericEncodeError>::into)?;
			)*

			::core::result::Result::Ok(())
		}
	}
}
