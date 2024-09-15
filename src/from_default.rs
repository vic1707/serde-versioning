/* Crate imports */
use crate::compile_error::CompileError;
/* Built-in imports */
extern crate alloc;
use alloc::vec::Vec;
use proc_macro2::Ident;
use quote::quote;
use syn::{Data, Field, Fields, Type};

pub struct DefaultImpls(proc_macro2::TokenStream);

impl DefaultImpls {
    pub fn build<'froms, I : Iterator<Item = &'froms Type>>(
        froms: I,
        ds_ident: &Ident,
        ds_data: Data,
    ) -> Result<Self, CompileError> {
        let Data::Struct(ds) = ds_data else {
            return Err(CompileError::from(
                "`from_default` attribute is only available on structs.",
            ));
        };
        let Fields::Named(named_fields) = ds.fields else {
            return Err(CompileError::from(
                "`from_default` attribute on structs with named fields.",
            ));
        };

        #[allow(clippy::needless_borrowed_reference, clippy::ref_patterns)]
        let fs = named_fields
            .named
            .iter()
            .map(|&Field { ref ident, .. }| {
                quote! {
                    #ident : value. #ident,
                }
            })
            .collect::<Vec<_>>();

        Ok(Self(
            froms
                .map(|from| {
                    quote! {
                        impl From< #from > for #ds_ident {
                            fn from(value: #from) -> Self {
                                Self {
                                    #(#fs)*
                                    ..Default::default()
                                }
                            }
                        }
                    }
                })
                .collect(),
        ))
    }
}

impl quote::ToTokens for DefaultImpls {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ts = &self.0;

        tokens.extend(quote! { #ts });
    }
}
