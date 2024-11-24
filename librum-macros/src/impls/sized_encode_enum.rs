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

use crate::Repr;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::iter;
use syn::DataEnum;

#[must_use]
pub fn sized_encode_enum(data: DataEnum, repr: Repr) -> TokenStream {
	let tys: Vec<Vec<_>> = data
		.variants
		.iter()
		.map(|variant| {
			variant
				.fields
				.iter()
				.map(|field| field.ty.clone())
				.chain(iter::once(repr.to_type(Span::call_site())))
				.collect()
		})
		.collect();

	quote! {
		const MAX_ENCODED_SIZE: usize = {
			let mut total_size = 0x0usize;

			let mut current_size = 0x0usize;

			#(
				current_size = 0x0 #(+ <#tys as ::librum::SizedEncode>::MAX_ENCODED_SIZE)*;

				if current_size > total_size { total_size = current_size };
			)*

			total_size
		};
	}
}
