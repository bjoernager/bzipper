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

use crate::DiscriminantIter;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, Fields, Token, Variant};
use syn::punctuated::Punctuated;

#[must_use]
pub fn decode_enum(data: &DataEnum) -> TokenStream {
	let mut match_arms = Punctuated::<TokenStream, Token![,]>::new();

	for (discriminant, variant) in DiscriminantIter::new(&data.variants) {
		let mut chain_commands = Punctuated::<TokenStream, Token![,]>::new();

		for field in &variant.fields {
			let command = field.ident
				.as_ref()
				.map_or_else(
					||           quote! { ::bzipper::Decode::decode(stream)? },
					|field_name| quote! { #field_name: ::bzipper::Decode::decode(stream)? },
				);

			chain_commands.push(command);
		}

		let value = match *variant {
			Variant { ident: ref variant_name, fields: Fields::Named(  ..), .. } => quote! { Self::#variant_name { #chain_commands } },
			Variant { ident: ref variant_name, fields: Fields::Unnamed(..), .. } => quote! { Self::#variant_name(#chain_commands) },
			Variant { ident: ref variant_name, fields: Fields::Unit, .. }        => quote! { Self::#variant_name },
		};

		match_arms.push(quote! { #discriminant => #value });
	}

	match_arms.push(quote! {
		value => return ::core::result::Result::Err(::bzipper::error::DecodeError::InvalidDiscriminant(value))
	});

	quote! {
		#[inline]
		fn decode(stream: &mut ::bzipper::IStream) -> ::core::result::Result<Self, ::bzipper::error::DecodeError> {
			let value = match (<isize as ::bzipper::Decode>::decode(stream)?) { #match_arms };
			Ok(value)
		}
	}
}
