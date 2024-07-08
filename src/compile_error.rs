/* Built-in imports */
use core::fmt;
use std::error;

#[derive(Debug)]
#[repr(transparent)]
pub struct CompileError(proc_macro::TokenStream);

impl From<syn::Error> for CompileError {
    fn from(err: syn::Error) -> Self {
        Self(err.to_compile_error().into())
    }
}

impl From<&str> for CompileError {
    fn from(msg: &str) -> Self {
        syn::Error::new(proc_macro2::Span::call_site(), msg).into()
    }
}

impl From<CompileError> for proc_macro::TokenStream {
    fn from(val: CompileError) -> Self {
        val.0
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "TODO")
    }
}

impl error::Error for CompileError {}
