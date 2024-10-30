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

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{DataStruct, Index};

#[must_use]
pub fn encode_struct(data: &DataStruct) -> TokenStream {
	let mut fields = Vec::new();

	for (index, field) in data.fields.iter().enumerate() {
		let name = field.ident
			.as_ref()
			.map_or_else(|| Index::from(index).to_token_stream(), ToTokens::to_token_stream);

		fields.push(name);
	}

	quote! {
		#[inline]
		fn encode(&self, stream: &mut ::bzipper::OStream) -> ::core::result::Result<(), ::bzipper::error::EncodeError> {
			#(::bzipper::Encode::encode(&self.#fields, stream)?;)*

			Ok(())
		}
	}
}
