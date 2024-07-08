macro_rules! tri {
    ($expr:expr) => {
        match $expr {
            Ok(el) => el,
            Err(err) => return proc_macro::TokenStream::from(err),
        }
    };
}

pub(crate) use tri;
