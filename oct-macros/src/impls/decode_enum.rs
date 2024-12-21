// Copyright 2024 Gabriel Bjørnager Jensen.
//
// This file is part of Oct.
//
// Oct is free software: you can redistribute it
// and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// Oct is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even
// the implied warranty of MERCHANTABILITY or FIT-
// NESS FOR A PARTICULAR PURPOSE. See the GNU Less-
// er General Public License for more details.
//
// You should have received a copy of the GNU Less-
// er General Public License along with Oct. If
// not, see <https://www.gnu.org/licenses/>.

use crate::{Discriminants, Repr};

use proc_macro2::TokenStream;
use quote::quote;
use std::iter;
use syn::{DataEnum, Fields};

#[must_use]
pub fn decode_enum(data: DataEnum, repr: Repr) -> TokenStream {
	let discriminants: Vec<_> = Discriminants::new(&data.variants).collect();

	let values = data
		.variants
		.into_iter()
		.map(|variant| {
			let variant_name = variant.ident;

			let commands = iter::repeat_n(
				quote! {
					::oct::decode::Decode::decode(stream)
						.map_err(::core::convert::Into::<::oct::error::GenericDecodeError>::into)
						.map_err(::oct::error::EnumDecodeError::BadField)?
				},
				variant.fields.len(),
			);

			match variant.fields {
				Fields::Unit => quote! { Self::#variant_name },

				Fields::Unnamed(_fields) => quote! { Self::#variant_name (#(#commands, )*) },

				Fields::Named(fields) => {
					let field_names = fields
						.named
						.into_iter()
						.map(|field| field.ident.unwrap());

					quote! { Self::#variant_name { #(#field_names: #commands, )* } }
				},
			}
		});

	quote! {
		type Error = ::oct::error::EnumDecodeError<#repr, ::oct::error::GenericDecodeError>;

		#[inline]
		fn decode(stream: &mut ::oct::decode::Input) -> ::core::result::Result<Self, Self::Error> {
			let discriminant = <#repr as ::oct::decode::Decode>::decode(stream)
				.map_err(::core::convert::Into::<::core::convert::Infallible>::into)
				.map_err(::oct::error::EnumDecodeError::InvalidDiscriminant)?;

			let this = match discriminant {
				#(#discriminants => #values,)*

				value => return ::core::result::Result::Err(::oct::error::EnumDecodeError::UnassignedDiscriminant { value }),
			};

			::core::result::Result::Ok(this)
		}
	}
}
