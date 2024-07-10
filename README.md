# serde-versioning

`serde-versioning` is a basic Rust crate that implements a naive solution for struct and enum versioning by extending the capabilities of `serde_derive`. This crate maintains 100% compatibility with `serde` while introducing a new container attribute `versioning` that provides versioning support for deserialization.

## What it does for you

`serde-versioning` allows you to turn a versioned struct (or enum) code that might look like this:

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct FooV0 { name: String }
#[derive(Debug, Deserialize)]
struct FooV1 { name: String, age: u8 }
#[derive(Debug, Deserialize)]
struct Foo { name: String, age: u8, country: String }

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum FooVersions {
    V0(FooV0),
    V1(FooV1),
    Latest(Foo),
}

impl TryFrom<FooV0> for FooV1 { /* ... */ }
impl TryFrom<FooV1> for Foo { /* ... */ }
impl TryFrom<FooVersions> for Foo { /* ... */ }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let foo_v0_str = r#"{ "name": "vic1707" }"#;
    let foo: Foo = serde_json::from_str::<FooVersions>(foo_v0_str)?.try_into()?;
    println!("{foo:#?}");
    Ok(())
}
```

Into

```rust
use serde_versioning::Deserialize;

#[derive(Debug, Deserialize)]
struct FooV0 { name: String }
#[derive(Debug, Deserialize)]
#[versioning(previous_version = FooV0)]
struct FooV1 { name: String, age: u8 }
#[derive(Debug, Deserialize)]
#[versioning(previous_version = FooV1)]
struct Foo { name: String, age: u8, country: String }

impl TryFrom<FooV0> for FooV1 { /* ... */ }
impl TryFrom<FooV1> for Foo { /* ... */ }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let foo_v0_str = r#"{ "name": "vic1707" }"#;
    let foo= serde_json::from_str::<Foo>(foo_v0_str)?;
    println!("{foo:#?}");
    Ok(())
}
````

**Note**: internally `serde_versioning` **doesn't** generate an untagged enum.

## Features

- **Optimistic/Pessimistic Deserialization**: Choose whether to attempt deserialization using previous versions first (pessimistic, which is the default) or the latest version first (optimistic).
- **Previous Version**: Specify a type name (either directly or as a string) representing a previous version.
- **Previous Versions**: Specify multiple previous versions using an array of type names.

## Installation

Add the following to your Cargo.toml:

```toml
[dependencies]
serde-versioning = { git = "https://github.com/vic1707/serde-versioning.git" }
serde = "1.0.204"
```

Ensure you have `serde` listed as a dependency to be able to import the `Deserialize` trait itself.

## Usage

`serde-versioning` is designed to be a drop-in replacement for `serde`. Simply replace:

```rust
use serde::Deserialize;
```

or

```rust
use serde_derive::Deserialize;
```

with

```rust
use serde_versioning::Deserialize;
```

And you'll be able to benefit from the `versioning` attribute while the original `Deserialize` capabilities stays as is.

## Example

```rust
#[derive(Deserialize)]
#[versioning(pessimistic, previous_versions = [FooV0, "FooV1"])]
struct Foo {
    name: String,
    age: u8,
    placeholder: String,
}
```

In this example, `Foo` will first attempt to deserialize as `FooV0`, then as `FooV1`, and finally as `Foo`, following the pessimistic strategy.
Feel free to look at the [usage](./examples/usage.rs) example.

## Implementation Details

Internally, `serde-versioning` manually invokes the original derive implementation from `serde`, which is imported via a personal fork (associated with a PR [#2765](https://github.com/serde-rs/serde/pull/2765) aimed at integrating this feature into the official `serde`).
The crate modifies the output to add versioning support, incorporating a few if-let-ok statements to handle the versioning logic.
The implementation is heavily inspired by the untagged enum approach commonly used for versioning, but `serde-versioning` attempts to make this process more transparent and straightforward.

Basically once derived it looks like this:

#### Optimistic

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
where
    __D: _serde::Deserializer<'de>,
{
    // imports and logic stolen from untagged enum derived implementation
    use _serde::__private::de::{Content, ContentRefDeserializer};
    let __content = Content::deserialize(__deserializer)?;
    let __deserializer = ContentRefDeserializer::<__D::Error>::new(&__content);

    if let Ok(Ok(__ok)) = { /* Original output from serde */ } {
        return Ok(__ok);
    }

    // as many of these as previous_versions you gave
    if let Ok(Ok(__ok)) = #version_ty ::deserialize(__deserializer).map(Self::try_from)
    {
        return Ok(__ok);
    }

    return Err(
        _serde::de::Error::custom("data did not match any version of (enum/struct) #ty"),
    );
}
```

#### Pessimistic

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
where
    __D: _serde::Deserializer<'de>,
{
    // imports and logic stolen from untagged enum derived implementation
    use _serde::__private::de::{Content, ContentRefDeserializer};
    let __content = Content::deserialize(__deserializer)?;
    let __deserializer = ContentRefDeserializer::<__D::Error>::new(&__content);

    // as many of these as previous_versions you gave
    if let Ok(Ok(__ok)) = #version_ty ::deserialize(__deserializer).map(Self::try_from)
    {
        return Ok(__ok);
    }

    /* Original output from serde */
}
```

Everything else is left as derived by serde.
If you don't use the new `versioning` attribute then I simply return what `serde` derives, nothing less, nothing more.

## Contributing

Feel free to open issues or submit pull requests. Contributions are welcome!
If you have an opinion on whether we should be pessimistic or optimistic by default I'd love to read your thoughts!

## License

This project is licensed under the "Do Whatever the Fuck You Want" License.
