#![no_std]
/* Dependencies */
use serde_derive::Deserialize;
/* Built-in imports */
extern crate alloc;
use alloc::string::String;

#[derive(Deserialize)]
struct FooV0 {
    name: String,
}

fn main() {}