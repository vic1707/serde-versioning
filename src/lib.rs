#![no_std]
/* Modules */
mod attributes;
mod compile_error;
mod deserialize_impl;
mod from_default;
mod utils;
/* Crate imports */
use attributes::Attributes;
use deserialize_impl::DeserializeImpl;
use from_default::DefaultImpls;
use utils::tri;
/* Built-in imports */
extern crate alloc;
use alloc::format;
/* Dependencies */
use quote::quote;
use syn::Data;

#[proc_macro_derive(Deserialize, attributes(serde, versioning))]
pub fn derive_deserialize_versioned(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let derive_input: syn::DeriveInput = syn::parse_macro_input!(input);

    let Some(versioning_attr) = derive_input
        .attrs
        .iter()
        .find(|meta| meta.path().is_ident("versioning"))
    else {
        return tri!(DeserializeImpl::try_from(derive_input)).into();
    };

    let Attributes {
        versioning,
        optimistic,
        from_default,
    } = tri!(Attributes::try_from(versioning_attr));
    let ds_ident = derive_input.ident.clone();
    let ds_data = derive_input.data.clone();
    let ds_kind_name = match ds_data {
        Data::Enum(_) => "enum",
        Data::Struct(_) => "struct",
        Data::Union(_) => "union",
    };
    let no_match_message =
        format!("data did not match any version of {ds_kind_name} {ds_ident}.");

    let mut deserialize_impl = tri!(DeserializeImpl::try_from(derive_input));

    tri!(deserialize_impl.replace_body(|de_stmts|{
        let deserialization = if optimistic {
            quote! {
                if let Ok(__ok) = { #(#de_stmts)* } { return Ok(__ok) }
                #versioning
                return Err(_serde::de::Error::custom(#no_match_message));
            }
        } else {
            quote! {
                #versioning
                #(#de_stmts)*
            }
        };

        quote! {{
            use _serde::__private::de::{Content, ContentRefDeserializer};
            let __content = Content::deserialize(__deserializer)?;
            let __deserializer = ContentRefDeserializer::<__D::Error>::new(&__content);

            #deserialization
        }}
    }));

    #[allow(clippy::if_then_some_else_none)]
    let default_impl: Option<DefaultImpls> = if from_default {
        Some(tri!(DefaultImpls::build(versioning.names(), &ds_ident, ds_data)))
    } else {
        None
    };

    let derived = quote! {
        #deserialize_impl
        #default_impl
    };
    proc_macro::TokenStream::from(derived)
}
