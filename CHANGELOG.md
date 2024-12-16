# CHANGELOG for the `auto-traits`-crate
This file keeps track of notable changes to the `auto-traits` crate.

The project uses [semantic versioning](https://semver.org). As such, breaking changes are
indicated with **(BREAKING CHANGE)**.


## v0.1.0 - 2024-12-16
Initial release!

### Added
- The `pointer_impls` attribute macro, which can make a trait have blanket implementations for a
  bunch of standard library types (and `parking_lot`, if the feature is enabled).
