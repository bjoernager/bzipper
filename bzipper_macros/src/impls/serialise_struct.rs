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

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
	DataStruct,
	Fields,
	Index,
	Token,
	punctuated::Punctuated
};

#[must_use]
pub fn serialise_struct(data: &DataStruct) -> TokenStream {
	if matches!(data.fields, Fields::Unit) {
		quote! {
			const MAX_SERIALISED_SIZE: usize = 0x0;

			#[inline(always)]
			fn serialise(&self, stream: &mut ::bzipper::Sstream) -> ::bzipper::Result<()> { Ok(()) }
		}
	} else {
		let mut serialised_size = Punctuated::<TokenStream, Token![+]>::new();
		let mut chain_commands  = Punctuated::<TokenStream, Token![;]>::new();

		for (index, field) in data.fields.iter().enumerate() {
			let ty = &field.ty;

			let name = field.ident
				.as_ref()
				.map_or_else(|| Index::from(index).to_token_stream(), ToTokens::to_token_stream);

			serialised_size.push(quote! { <#ty as ::bzipper::Serialise>::MAX_SERIALISED_SIZE });

			chain_commands.push(quote! { self.#name.serialise(stream)? });
		}

		chain_commands.push_punct(Token![;](Span::call_site()));

		quote! {
			const MAX_SERIALISED_SIZE: usize = #serialised_size;

			fn serialise(&self, stream: &mut ::bzipper::Sstream) -> ::bzipper::Result<()> {
				#chain_commands

				Ok(())
			}
		}
	}
}
