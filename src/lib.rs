//  LIB.rs
//    by Lut99
//
//  Created:
//    13 Dec 2024, 14:22:24
//  Last edited:
//    13 Dec 2024, 14:51:04
//  Auto updated?
//    Yes
//
//  Description:
//!   A Rust crate containing procedural macros for common auto-implementations for traits.
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
/// - a type, which will add the implementation for that type.
///     - By default, the implementation will rely on a [`Deref`]-implementation to coerce the
///       pointers to `self`; however, you can specify a closure after an equals sign to change how
///       this is accessed.
///     - You can use `_` to refer to the original object (e.g., `&_`).
/// - `*`, which expands to all the default types. In combination with `not` (see below), this can
///   be used to exclude any standard type: `#[pointer_impls(!*)]`.
/// - `! ...`, meaning that that type will NOT be implemented instead of will. This can be used
///   to exclude any of the standard types, for example.
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
#[proc_macro_attribute]
pub fn pointer_impls(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Pass to the actual implementation
    match pointer_impls::pointer_impls(attr.into(), item.into()) {
        Ok(res) => res.into(),
        Err(err) => err.into_compile_error().into(),
    }
}
