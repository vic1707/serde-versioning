# serde-versioning

`serde-versioning` is a basic Rust crate that implements a naive solution for struct and enum versioning by extending the capabilities of `serde_derive`. This crate maintains 100% compatibility with `serde` while introducing a new container attribute `versioning` that provides versioning support for deserialization.

## Features

- **Optimistic/Pessimistic Deserialization**: Choose whether to attempt deserialization using previous versions first (pessimistic, which is the default) or the latest version first (optimistic).
- **Previous Version**: Specify a type name (either directly or as a string) representing a previous version.
- **Previous Versions**: Specify multiple previous versions using an array of type names.

## Installation

Add the following to your Cargo.toml:

```toml
[dependencies]
serde-versioning = { git = "https://github.com/vic1707/serde-versioning.git" }
serde = { version = "1.0", features = ["derive"] }
```

Ensure you have `serde` with the `derive` feature enabled for compatibility.

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

And you'll be able to benefit from the `versioning` attribute without affecting while the original `Deserialize` capabilities stays as is.

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

## Contributing

Feel free to open issues or submit pull requests. Contributions are welcome!
If you have an opinion on whether we should be pessimistic or optimistic by default I'd love to read your thoughts!

## License

This project is licensed under the "Do Whatever the Fuck You Want" License.
