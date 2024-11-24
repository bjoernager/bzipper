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

use crate::{Discriminants, Repr};

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{DataEnum, Fields, Ident, LitInt};

#[must_use]
pub fn encode_enum(data: DataEnum, repr: Repr) -> TokenStream {
	let discriminants: Vec<LitInt> = Discriminants::new(&data.variants).collect();

	let captures: Vec<Vec<Ident>> = data
		.variants
		.iter()
		.map(|variant| {
			variant
				.fields
				.iter()
				.enumerate()
				.map(|(index, _)| Ident::new(&format!("value{index}"), Span::call_site()))
				.collect()
		})
		.collect();

	let patterns = data
		.variants
		.into_iter()
		.zip(&captures)
		.map(|(variant, captures)| {
			let variant_name = variant.ident;

			match variant.fields {
				Fields::Unit => quote! { Self::#variant_name },

				Fields::Unnamed(_fields) => quote! { Self::#variant_name (#(ref #captures, )*) },

				Fields::Named(fields) => {
					let field_names = fields
						.named
						.into_iter()
						.map(|field| field.ident.unwrap());

					quote! { Self::#variant_name { #(#field_names: ref #captures, )* } }
				},
			}
		});

	quote! {
		type Error = ::librum::error::EnumEncodeError<#repr, ::librum::error::GenericEncodeError>;

		#[allow(unreachable_patterns)]
		#[inline]
		fn encode(&self, stream: &mut ::librum::OStream) -> ::core::result::Result<(), Self::Error> {
			match *self {
				#(
					#patterns => {
						<#repr as ::librum::Encode>::encode(&#discriminants, stream)
							.map_err(::librum::error::EnumEncodeError::Discriminant)?;

						#(
							::librum::Encode::encode(#captures, stream)
								.map_err(::core::convert::Into::<::librum::error::GenericEncodeError>::into)
								.map_err(::librum::error::EnumEncodeError::Field)?;
						)*
					}
				)*

				_ => ::core::unreachable!("no variants defined for this enumeration"),
			}

			::core::result::Result::Ok(())
		}
	}
}
