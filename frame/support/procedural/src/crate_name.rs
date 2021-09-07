// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Implementation of macros related to crate naming.

use proc_macro2::{Span, TokenStream};
use syn::{Error, Result};
use super::get_cargo_env_var;

/// Implementation of the `crate_to_crate_name!` macro.
pub fn crate_to_crate_name(input: proc_macro::TokenStream) -> Result<TokenStream> {
	if !input.is_empty() {
		return Err(Error::new(Span::call_site(), "No arguments expected!"))
	}

	let crate_name = get_cargo_env_var::<String>("CARGO_PKG_NAME")
		.map_err(|_| Error::new(Span::call_site(), "Major version needs to fit into `u16`"))?
        .replace("-", "_");

	Ok(quote::quote!(#crate_name))
}

