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

use crate::Capture;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{DataEnum, Fields, Ident, Token};
use syn::punctuated::Punctuated;

#[must_use]
pub fn serialise_enum(data: &DataEnum) -> TokenStream {
	let mut sizes = Vec::new();

	let mut match_arms = Punctuated::<TokenStream, Token![,]>::new();

	for (index, variant) in data.variants.iter().enumerate() {
		let mut serialised_size = Punctuated::<TokenStream, Token![+]>::new();

		let variant_name = &variant.ident;

		let discriminant = u32::try_from(index)
			.expect("enumeration discriminants must be representable in `u32`");

		// Discriminant size:
		serialised_size.push(quote! { <u32 as ::bzipper::Serialise>::SERIALISED_SIZE });

		let mut captures = Punctuated::<Capture, Token![,]>::new();

		let mut chain_commands = Punctuated::<TokenStream, Token![;]>::new();
		chain_commands.push(quote! { stream.append(&#discriminant)? });

		for (index, field) in variant.fields.iter().enumerate() {
			let field_ty = &field.ty;

			let field_name = field.ident
				.as_ref()
				.map_or_else(|| Ident::new(&format!("v{index}"), Span::call_site()), Clone::clone);

			serialised_size.push(quote! { <#field_ty as ::bzipper::Serialise>::SERIALISED_SIZE });

			captures.push(Capture {
				ref_token: Token![ref](Span::call_site()),
				ident:     field_name.clone(),
			});

			chain_commands.push(quote! { stream.append(#field_name)? });
		}

		chain_commands.push_punct(Token![;](Span::call_site()));

		let arm = match variant.fields {
			Fields::Named(  ..) => quote! { Self::#variant_name { #captures } => { #chain_commands } },
			Fields::Unnamed(..) => quote! { Self::#variant_name(#captures)    => { #chain_commands } },
			Fields::Unit        => quote! { Self::#variant_name               => { #chain_commands } },
		};

		sizes.push(serialised_size);
		match_arms.push(arm);
	}

	let mut size_tests = Punctuated::<TokenStream, Token![else]>::new();

	for size in &sizes {
		let mut test = Punctuated::<TokenStream, Token![&&]>::new();

		for other_size in &sizes { test.push(quote! { #size >= #other_size }) }

		size_tests.push(quote! { if #test { #size } });
	}

	size_tests.push(quote! { { core::unreachable!(); } });

	quote! {
		const SERIALISED_SIZE: usize = const { #size_tests };

		fn serialise(&self, buf: &mut [u8]) -> ::bzipper::Result<()> {
			::core::debug_assert_eq!(buf.len(), Self::SERIALISED_SIZE);

			let mut stream = ::bzipper::Sstream::new(buf);

			match (*self) { #match_arms }
			Ok(())
		}
	}
}
