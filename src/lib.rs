/* Modules */
mod attributes;
mod compile_error;
mod deserialize_impl;
mod utils;
/* Crate imports */
use attributes::Attributes;
use deserialize_impl::DeserializeImpl;
use utils::tri;
/* Dependencies */
use quote::quote;

#[proc_macro_derive(Deserialize, attributes(serde, versioning))]
pub fn derive_deserialize_versioned(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let derive_input: syn::DeriveInput = syn::parse_macro_input!(input);
    let derive_attrs = derive_input.attrs.clone();
    let mut deserialize_impl = tri!(DeserializeImpl::try_from(derive_input));

    let Attributes { versioning } = match derive_attrs
        .iter()
        .find(|meta| meta.path().is_ident("versioning"))
    {
        // if no `versioning` attribut found
        // simply return the default serde derived impl.
        None => return proc_macro::TokenStream::from(deserialize_impl),
        Some(attr) => tri!(Attributes::try_from(attr)),
    };

    tri!(deserialize_impl.replace_body(|de_stmts|
        quote! {{
            use _serde::__private::{
                Result as SerdeResult,
                de::{Content, ContentRefDeserializer}
            };
            let __content = <Content as _serde::Deserialize>::deserialize(__deserializer)?;
            let __deserializer = ContentRefDeserializer::<__D::Error,>::new(&__content);

            #versioning

            #(#de_stmts)*
        }}
    ));

    proc_macro::TokenStream::from(deserialize_impl)
}