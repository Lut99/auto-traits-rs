# CHANGELOG for the `auto-traits`-crate
This file keeps track of notable changes to the `auto-traits` crate.

The project uses [semantic versioning](https://semver.org). As such, breaking changes are
indicated with **(BREAKING CHANGE)**.


## v0.2.0 - 2024-12-17
### Added
- The `#[pointer_impl(...)]` field macro on trait items.
    - Added `#[pointer_impl(generics = <...>)]` to specify which generics to pass to the
      implementation. This should generalise the problem of passing incorrect generics to the
      method call.

### Fixed
- A stray `println!()` in the `pointer_impls` attribute macro.
- The `pointer_impls` attribute macro failing to compile if it is implementing a method without
  `self`.
- The `pointer_impls` attribute macro failing to compile if it is implementing a method with
  lifetimes.



## v0.1.0 - 2024-12-16
Initial release!

### Added
- The `pointer_impls` attribute macro, which can make a trait have blanket implementations for a
  bunch of standard library types (and `parking_lot`, if the feature is enabled).
