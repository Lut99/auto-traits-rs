//  LIB.rs
//    by Lut99
//
//  Created:
//    13 Dec 2024, 14:22:24
//  Last edited:
//    16 Dec 2024, 14:22:30
//  Auto updated?
//    Yes
//
//  Description:
//!   A Rust crate containing procedural macros for common auto-implementations for traits.
//!   
//!   
//!   # Installation
//!   To use the crate, first add it to your `Cargo.toml`, e.g.:
//!   ```toml
//!   [dependencies]
//!   auto-traits = { git = "https://github.com/Lut99/auto-traits-rs" }
//!   ```
//!   
//!   You can commit yourself to a particular version with:
//!   ```toml
//!   [dependencies]
//!   auto-traits = { git = "https://github.com/Lut99/auto-traits-rs", tag = "v0.1.0" }
//!   ```
//!   
//!   Optionally, you can also enable features with:
//!   ```toml
//!   auto-traits = { git = "https://github.com/Lut99/auto-traits-rs", features = ["parking_lot"] }
//!   ```
//!   
//!   
//!   # Usage
//!   Using the crate is pretty straightforward. Go to any trait that you want to commute over standard
//!   pointer-like types, and add:
//!   ```rust
//!   #[auto_traits::pointer_impls]
//!   trait Foo {
//!       fn foo(&self) -> &str;
//!   }
//!   ```
//!   and a transparent implementation of your trait is generated for quite a few standard library types.
//!   See the documentation of the `pointer_impl`-attribute macro for the full specification of which
//!   types are supported and how to use it.
//!   
//!   ## Features
//!   The crate supports the following features:
//!   - `parking_lot`: Adds blanket implementations for
//!     [`parking_lot`](https://crates.io/crates/parking_lot)'s `MutexGuard`, `RwLockReadGuard` and
//!     `RwLockWriteGuard` types to every invocation of the `pointer_impls` macro.
//!   
//!   ## Documentation
//!   You can generate the code documentation by running:
//!   ```sh
//!   cargo doc --open --no-deps
//!   ```
//!   which will automatically open the generated HTML in your system's default browser.
//!   
//!   
//!   # Contribution
//!   If you're interested in constributing to the project, welcome! Simply
//!   [create an issue](https://github.com/Lut99/auto-traits-rs/issues) or
//!   [open a pull request](https://github.com/Lut99/auto-traits-rs/pulls).
//!   
//!   
//!   # License
//!   This project is licensed under [Apache 2.0](./LICENSE).
//

// Modules
mod pointer_impls;

// Imports
use proc_macro::TokenStream;


/***** LIBRARY *****/
/// A procedural macro that will automatically implement pointer-like types for your traits.
///
/// It is often useful if your trait doesn't hold for the original object, but also for references,
/// boxes, reference-counter pointers, etc. This attribute macro allows you to quickly provide
/// these blanket implementations for the types you specify.
///
/// # Usage
/// To use the macro attribute, simply add it to any trait definition:
/// ```rust
/// use auto_traits::pointer_impls;
///
/// #[pointer_impls]
/// trait Foo {
///     fn foo(&self) -> &str;
///     fn bar(&self) -> &str { "bar" }
/// }
/// ```
///
/// This will automatically implement a blanket implementation that forwards everything to the
/// implementation of object `T` for:
/// - `&T` *
/// - `&mut T`
/// - `Box<T>`
/// - `Rc<T>` *
/// - `Arc<T>` *
/// - `Ref` *
/// - `RefMut`
/// - `MutexGuard`
/// - `RwLockReadGuard` *
/// - `RwLockWriteGuard`
/// - (`parking_lot` feature) `parking_lot::MutexGuard`
/// - (`parking_lot` feature) `parking_lot::RwLockReadGuard` *
/// - (`parking_lot` feature) `parking_lot::RwLockWriteGuard`
///
/// where types marked with an aterisk (*) are only implemented if the trait has no methods with
/// `&mut self`.
///
///
/// ## Specifying types
/// You can tweak the attribute to change for which types your trait is implemented.
///
/// You can call it like:
/// ```rust
/// use auto_traits::pointer_impls;
///
/// #[pointer_impls(...)]
/// trait Foo {
///     fn foo(&self) -> &str;
///     fn bar(&self) -> &str { "bar" }
/// }
/// ```
/// where `...` is a comma-separated list of:
/// - `impl[<T1, T2, ...>] [mut] Foo<_>`, which adds an implementation for the given type.
///     - Any generics given are generics _added_ to the implementation that aren't already in the
///       trait definition. This is usually used for lifetimes.
///     - Specifying `mut` indicates that your type supports interior mutability. This is only
///       relevant if the trait has methods with mutable access to `self`.
///     - By default, the implementation will rely on a [`Deref`]/[`DerefMut`]-implementation to
///       coerce the pointers to `self`; however, you can specify an expression over `self` after
///       an equals sign to change how this is accessed (e.g., `impl Foo<_> = &self.0`).
///     - You can use `_` to refer to the original object (e.g., `&_`).
/// - `unimpl Foo<_>`, which removes generating an implementation for a certain type. This is
///   mostly useful for excluding types that are defaultly generated.
///     - Note that types are referred to by absolute path, e.g., `Box` should be
///       `::std::boxed::Box<_>`.
///     - Use `*` instead of a typename to remove ALL currently marked-for-implementation types.
///       This is useful for when you only want to implement your own types.
///
/// For examples on how to use these patterns, see the
/// [`examples/`](https://github.com/Lut99/auto-traits-rs/tree/main/examples) in the repository.
///
///
/// ## Default implementations
/// Some traits have methods with default implementations. In these cases, by default, the macro
/// will not generate anything in the blanket implementations. However, you can include them as any
/// other by attaching the `#[include_impl]`-attribute to them:
/// ```rust
/// use auto_traits::pointer_impls;
///
/// #[pointer_impls]
/// trait Foo {
///     fn foo(&self) -> &str;
///     #[include_impl]
///     fn bar(&self) -> &str { "bar" }
/// }
/// ```
///
///
/// # Considerations
/// This macro has a few implementations. Currently:
/// - The impls only work for concrete types - and then specifically, paths (e.g., identifiers with
///   generics).
/// - Macro invocations in traits are ignored in the `impls`, because I'm unsure how those would
///   generally translate to impls.
#[proc_macro_attribute]
pub fn pointer_impls(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Pass to the actual implementation
    match pointer_impls::pointer_impls(attr.into(), item.into()) {
        Ok(res) => res.into(),
        Err(err) => err.into_compile_error().into(),
    }
}
