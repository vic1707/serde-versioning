/* Feature gates */
// ensure the new derive macro doesn't cause trouble with no_std support
#![no_std]
/* Clippy config */
#![allow(dead_code, clippy::unwrap_used)]
/* Built-in imports */
extern crate alloc;
use alloc::string::String;
/* Dependencies */
use serde_versioning::Deserialize;

const V0: &str = r#"{ "name": "vic1707" }"#;
const V1: &str = r#"{ "name": "vic1707", "age": 0 }"#;
const V2: &str = r#"{ "name": "vic1707", "age": 0, "placeholder": "hi" }"#;
const VF: &str = r#"{ "name": "vic1707", "age": 0, "place_holder": "hi" }"#;

#[derive(Deserialize)]
struct FooV0 {
    name: String,
}

#[derive(Deserialize)]
#[versioning(previous_version = "FooV0")]
struct FooV1 {
    name: String,
    age: u8,
}

#[derive(Deserialize)]
#[versioning(previous_versions = [FooV0, "FooV1"])]
struct FooV2 {
    name: String,
    age: u8,
    placeholder: String,
}

#[derive(Deserialize, Default)]
#[versioning(previous_version = FooV2)]
struct Foo {
    name: String,
    age: u8,
    #[serde(default)]
    place_holder: String,
}

fn main() {
    // FooV0
    serde_json::from_str::<FooV0>(V0).unwrap();
    // FooV1
    serde_json::from_str::<FooV0>(V1).unwrap();
    serde_json::from_str::<FooV1>(V1).unwrap();
    // FooV2
    serde_json::from_str::<FooV2>(V0).unwrap();
    serde_json::from_str::<FooV2>(V1).unwrap();
    serde_json::from_str::<FooV2>(V2).unwrap();
    // Foo
    serde_json::from_str::<Foo>(V0).unwrap();
    serde_json::from_str::<Foo>(V1).unwrap();
    serde_json::from_str::<Foo>(V2).unwrap();
    serde_json::from_str::<Foo>(VF).unwrap();
}

// all type conversions needed, will also work with TryFrom
impl From<FooV0> for FooV1 {
    fn from(v0: FooV0) -> Self {
        Self {
            name: v0.name,
            age: 0,
        }
    }
}
impl From<FooV1> for FooV2 {
    fn from(v1: FooV1) -> Self {
        Self {
            name: v1.name,
            age: v1.age,
            placeholder: String::new(),
        }
    }
}
impl From<FooV0> for FooV2 {
    fn from(v0: FooV0) -> Self {
        Self {
            name: v0.name,
            age: 0,
            placeholder: String::new(),
        }
    }
}
impl From<FooV2> for Foo {
    fn from(_: FooV2) -> Self {
        Self::default()
    }
}
