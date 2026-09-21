/* Dependencies */
use serde_derive::Deserialize;
/* Built-in imports */
extern crate alloc;
use alloc::string::String;

#[derive(Deserialize)]
#[allow(dead_code)]
struct FooV0 {
    name: String,
}

fn main() {}
