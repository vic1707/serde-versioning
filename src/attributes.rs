/* Crate imports */
use crate::compile_error::CompileError;
/* Dependencies */
use quote::quote;
use syn::{Expr, Lit, Type};

macro_rules! syn_err {
    ($expr:expr) => {
        return Err(syn::Error::new(proc_macro2::Span::call_site(), $expr))
    };
}

pub struct Attributes {
    pub versioning: PreviousVersions,
}

pub struct PreviousVersions(Vec<syn::Type>);

impl TryFrom<&syn::Attribute> for Attributes {
    type Error = CompileError;

    fn try_from(attribute: &syn::Attribute) -> Result<Self, Self::Error> {
        let mut opt_versioning = None;
        let meta_list = attribute.meta.require_list()?;

        meta_list.parse_nested_meta(|meta| {
            if meta.path.is_ident("previous_versions") {
                let expr: syn::Expr = meta.value()?.parse()?;
                let Expr::Array(type_arr) = expr else {
                    syn_err!("Expected an Array")
                };
                let type_names: Vec<syn::Type> = type_arr
                    .elems
                    .into_iter()
                    .map(require_syn_type)
                    .collect::<Result<Vec<_>, syn::Error>>()?;
                opt_versioning.replace(PreviousVersions(type_names));
            } else if meta.path.is_ident("previous_version") {
                let expr: syn::Expr = meta.value()?.parse()?;
                let type_name = require_syn_type(expr)?;
                opt_versioning.replace(PreviousVersions(vec![type_name]));
            } else {
                syn_err!(format!(
                    "Unknown attribute: '{:?}'",
                    meta.path.get_ident()
                ));
            }

            Ok(())
        })?;

        let Some(versioning) = opt_versioning else {
            return Err(CompileError::from("No previous versions found"));
        };

        Ok(Self { versioning })
    }
}

impl quote::ToTokens for PreviousVersions {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let if_lets = self.0.iter().map(|ty| {
            quote! {
                if let Ok(Ok(__ok)) = <#ty as _serde::Deserialize<
                    'de,
                >>::deserialize(__deserializer)
                    .map(Self::try_from)
                {
                    return Ok(__ok);
                }
            }
        });

        tokens.extend(quote! { #(#if_lets)* });
    }
}

fn require_syn_type(expr: syn::Expr) -> Result<syn::Type, syn::Error> {
    if let Expr::Path(syn::ExprPath { path, .. }) = expr {
        Ok(Type::Path(syn::TypePath { path, qself: None }))
    } else if let Expr::Lit(syn::ExprLit {
        lit: Lit::Str(type_name),
        ..
    }) = expr
    {
        type_name.parse()
    } else {
        syn_err!("Expected a TypeName")
    }
}
