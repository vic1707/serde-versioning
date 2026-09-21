#![allow(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::restriction,
    unexpected_cfgs,
    warnings,
)]

use proc_macro2::{Ident, Span};
use quote::{ToTokens, TokenStreamExt as _};

#[macro_use]
pub mod bound;
#[macro_use]
pub mod fragment;

pub mod de;
pub mod deprecated;
pub mod dummy;
pub mod internals;
pub mod pretend;
pub mod ser;
pub mod this;

#[allow(non_camel_case_types)]
pub struct private;

impl private {
    fn ident(&self) -> Ident {
        Ident::new(
            "__private229",
            Span::call_site(),
        )
    }
}

impl ToTokens for private {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append(self.ident());
    }
}
