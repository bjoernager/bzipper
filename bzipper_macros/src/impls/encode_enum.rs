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

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
	DataEnum,
	Fields,
	Ident,
	Variant,
};

#[must_use]
pub fn encode_enum(data: &DataEnum) -> TokenStream {
	let mut match_arms = Vec::new();

	// Iterate over each variant and give it a unique
	// encoding scheme.
	for (discriminant, variant) in DiscriminantIter::new(&data.variants) {
		// The original identifiers of the fields:
		let mut field_names = Vec::new();

		// The captured field identifiers:
		let mut field_captures = Vec::new();

		for (index, field) in variant.fields.iter().enumerate() {
			let capture = Ident::new(&format!("v{index}"), Span::call_site());

			field_names.push(&field.ident);
			field_captures.push(capture);
		}

		let pattern = match *variant {
			Variant { ident: ref variant_name, fields: Fields::Named(  ..), .. } => quote! { Self::#variant_name { #(#field_names: ref #field_captures, )* } },
			Variant { ident: ref variant_name, fields: Fields::Unnamed(..), .. } => quote! { Self::#variant_name(#(ref #field_captures)*) },
			Variant { ident: ref variant_name, fields: Fields::Unit, ..        } => quote! { Self::#variant_name },
		};

		match_arms.push(quote! {
			#pattern => {
				::bzipper::Encode::encode(&#discriminant, stream)?;
				#(::bzipper::Encode::encode(#field_captures, stream)?;)*
			}
		});
	}

	quote! {
		#[inline]
		fn encode(&self, stream: &mut ::bzipper::OStream) -> ::core::result::Result<(), ::bzipper::error::EncodeError> {
			match *self {
				#(#match_arms)*
			}

			Ok(())
		}
	}
}
