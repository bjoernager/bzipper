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
use quote::quote;
use syn::{DataStruct, Fields, Token};
use syn::punctuated::Punctuated;

#[must_use]
pub fn deserialise_struct(data: &DataStruct) -> TokenStream {
	if matches!(data.fields, Fields::Unit) {
		quote! {
			#[inline(always)]
			fn deserialise(_stream: &::bzipper::Dstream) -> ::bzipper::Result<Self> { Ok(Self) }
		}
	} else {
		let mut chain_commands = Punctuated::<TokenStream, Token![,]>::new();

		for field in &data.fields {
			let command = field.ident
				.as_ref()
				.map_or_else(
					||           quote! { Deserialise::deserialise(stream)? },
					|field_name| quote! { #field_name: Deserialise::deserialise(stream)? }
				);

			chain_commands.push(command);
		}

		let value = if let Fields::Named(..) = data.fields {
			quote! { Self { #chain_commands } }
		} else {
			quote! { Self(#chain_commands) }
		};

		quote! {
			fn deserialise(stream: &::bzipper::Dstream) -> ::bzipper::Result<Self> {
				let value = #value;
				Ok(value)
			}
		}
	}
}
