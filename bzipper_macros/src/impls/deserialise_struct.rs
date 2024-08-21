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
	if let Fields::Named(..) = data.fields {
		let mut chain_commands = Punctuated::<TokenStream, Token![,]>::new();

		for field in &data.fields {
			let name = field.ident.as_ref().unwrap();
			let ty   = &field.ty;

			chain_commands.push(quote! { #name: stream.take::<#ty>()? });
		}

		quote! {
			fn deserialise(data: &[u8]) -> ::bzipper::Result<Self> {
				::core::debug_assert_eq!(data.len(), <Self as ::bzipper::Serialise>::SERIALISED_SIZE);

				let stream = ::bzipper::Dstream::new(data);

				Ok(Self { #chain_commands })
			}
		}
	} else if let Fields::Unnamed(..) = data.fields {
		let mut chain_commands = Punctuated::<TokenStream, Token![,]>::new();

		for field in &data.fields {
			let ty = &field.ty;

			chain_commands.push(quote! { stream.take::<#ty>()? });
		}

		quote! {
			fn deserialise(data: &[u8]) -> ::bzipper::Result<Self> {
				::core::debug_assert_eq!(data.len(), <Self as ::bzipper::Serialise>::SERIALISED_SIZE);

				let stream = ::bzipper::Dstream::new(data);

				Ok(Self(#chain_commands))
			}
		}
	} else {
		// Fields::Unit

		quote! {
			#[inline(always)]
			fn deserialise(data: &[u8]) -> ::bzipper::Result<Self> {
				::core::debug_assert_eq!(data.len(), <Self as ::bzipper::Serialise>::SERIALISED_SIZE);

				Ok(Self)
			}
		}
	}
}
