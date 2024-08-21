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

use crate::Discriminant;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, Fields, Token};
use syn::punctuated::Punctuated;

#[must_use]
pub fn deserialise_enum(data: &DataEnum) -> TokenStream {
	let mut match_arms = Punctuated::<TokenStream, Token![,]>::new();

	for (index, variant) in data.variants.iter().enumerate() {
		let variant_name = &variant.ident;

		let discriminant = Discriminant::unwrap_from(index);

		let block = if matches!(variant.fields, Fields::Unit) {
			quote! { Self }
		} else {
			let mut chain_commands = Punctuated::<TokenStream, Token![,]>::new();

			for field in &variant.fields {
				let field_ty = &field.ty;

				let command = field.ident
					.as_ref()
					.map_or_else(
						||           quote! { stream.take::<#field_ty>()? },
						|field_name| quote! { #field_name: stream.take::<#field_ty>()? }
					);

				chain_commands.push(command);
			}

			match variant.fields {
				Fields::Named(  ..) => quote! { Self::#variant_name { #chain_commands } },
				Fields::Unnamed(..) => quote! { Self::#variant_name(#chain_commands) },
				Fields::Unit        => unreachable!(),
			}
		};

		match_arms.push(quote! { #discriminant => #block });
	}

	match_arms.push(quote! { value => return Err(::bzipper::Error::InvalidDiscriminant { value }) });

	quote! {
		fn deserialise(data: &[u8]) -> ::bzipper::Result<Self> {
			::core::debug_assert_eq!(data.len(), <Self as ::bzipper::Serialise>::SERIALISED_SIZE);

			let mut stream = ::bzipper::Dstream::new(data);

			let value = match (stream.take::<u32>()?) { #match_arms };
			Ok(value)
		}
	}
}
