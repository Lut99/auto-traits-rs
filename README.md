# auto-traits-rs
A Rust crate containing procedural macros for common auto-implementations for traits.


## Installation
To use the crate, first add it to your `Cargo.toml`, e.g.:
```toml
[dependencies]
auto-traits = { git = "https://github.com/Lut99/auto-traits-rs" }
```

You can commit yourself to a particular version with:
```toml
[dependencies]
auto-traits = { git = "https://github.com/Lut99/auto-traits-rs", tag = "v0.2.1" }
```

Optionally, you can also enable features with:
```toml
auto-traits = { git = "https://github.com/Lut99/auto-traits-rs", features = ["parking_lot"] }
```


## Usage
Using the crate is pretty straightforward. Go to any trait that you want to commute over standard
pointer-like types, and add:
```rust
#[auto_traits::pointer_impls]
trait Foo {
    fn foo(&self) -> &str;
}
```
and a transparent implementation of your trait is generated for quite a few standard library types.
See the documentation of the `pointer_impl`-attribute macro for the full specification of which
types are supported and how to use it.

### Features
The crate supports the following features:
- `parking_lot`: Adds blanket implementations for
  [`parking_lot`](https://crates.io/crates/parking_lot)'s `MutexGuard`, `RwLockReadGuard` and
  `RwLockWriteGuard` types to every invocation of the `pointer_impls` macro.

### Documentation
You can generate the code documentation by running:
```sh
cargo doc --open --no-deps
```
which will automatically open the generated HTML in your system's default browser.


## Contribution
If you're interested in constributing to the project, welcome! Simply
[create an issue](https://github.com/Lut99/auto-traits-rs/issues) or
[open a pull request](https://github.com/Lut99/auto-traits-rs/pulls).


## License
This project is licensed under [Apache 2.0](./LICENSE).
