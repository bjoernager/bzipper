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
use quote::quote;
use syn::DataEnum;

#[must_use]
pub fn sized_encode_enum(data: &DataEnum) -> TokenStream {
	let mut sizes = Vec::new();

	// Iterate over each variant and give it a unique
	// encoding scheme.
	for variant in &data.variants {
		let mut field_tys = Vec::new();

		for field in &variant.fields {
			field_tys.push(&field.ty);
		}

		sizes.push(quote! {
			<isize as ::bzipper::SizedEncode>::MAX_ENCODED_SIZE
			#(+ <#field_tys as ::bzipper::SizedEncode>::MAX_ENCODED_SIZE)*
		});
	}

	quote! {
		const MAX_ENCODED_SIZE: usize = const {
			let mut max_encoded_size = 0x0usize;

			#(if #sizes > max_encoded_size { max_encoded_size = #sizes };)*

			max_encoded_size
		};
	}
}
