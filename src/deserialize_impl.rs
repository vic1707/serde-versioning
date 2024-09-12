/* Crate imports */
use crate::compile_error::CompileError;
use quote::ToTokens;
/* Built-in imports */
extern crate alloc;
use alloc::vec::Vec;
/* Dependencies */
use serde_derive_implementation::de::expand_derive_deserialize;
use syn::{Expr, ImplItem, ImplItemFn, Item, ItemConst, Stmt};

pub struct DeserializeImpl(ItemConst);

impl DeserializeImpl {
    fn get_fn_impl(&mut self) -> Result<&mut ImplItemFn, CompileError> {
        let Expr::Block(ref mut const_block) = *self.0.expr else {
            return Err("Couldn't find Deserialize const block".into());
        };

        let Some(&mut Stmt::Item(Item::Impl(ref mut deserialize_impl))) =
            const_block.block.stmts.get_mut(1)
        else {
            return Err("Couldn't find Deserialize impl block".into());
        };

        let Some(&mut ImplItem::Fn(ref mut deserialize_fn_impl)) =
            deserialize_impl.items.get_mut(0)
        else {
            return Err("Couldn't find Deserialize::deserialize fn impl".into());
        };

        Ok(deserialize_fn_impl)
    }

    pub fn replace_body<F: FnOnce(&Vec<Stmt>) -> proc_macro2::TokenStream>(
        &mut self,
        func: F,
    ) -> Result<(), CompileError> {
        let de_fn_impl = self.get_fn_impl()?;
        // stmts to avoid unnecessary braces
        let de_fn_block_stmts = &de_fn_impl.block.stmts;

        de_fn_impl.block = syn::parse2(func(de_fn_block_stmts))?;

        Ok(())
    }
}

impl TryFrom<syn::DeriveInput> for DeserializeImpl {
    type Error = CompileError;

    fn try_from(mut input: syn::DeriveInput) -> Result<Self, Self::Error> {
        let deserialize_expanded = expand_derive_deserialize(&mut input)?;
        let deserialize_const_impl = syn::parse2(deserialize_expanded)?;

        Ok(Self(deserialize_const_impl))
    }
}

impl From<DeserializeImpl> for proc_macro::TokenStream {
    fn from(de_impl: DeserializeImpl) -> Self {
        de_impl.0.into_token_stream().into()
    }
}

impl quote::ToTokens for DeserializeImpl {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}
