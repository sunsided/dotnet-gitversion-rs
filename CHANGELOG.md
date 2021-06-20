# Changelog

All notable changes to this project will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.2.0 - 2021-06-20

### Added

- The `GitVersion` struct now has a `pub const fn new()` function.
- The generated `GitVersion` now implements the `Default` trait.
- Added member documentation to the intermediate and generated structs.

### Changed

- The `dotnet_gitversion_build::build()` now returns the intermediate `GitVersion`
  instance.
- The generated `GIT_VERSION` is now a `pub const`. 
- Some NuGet related fields were marked with the `#[deprecated]` attribute to discourage their use
  in favor of the SemVer fields.

## 0.1.0 - 2021-06-19

### Added

- Initial release providing the `dotnet_gitversion_build::build()` function.
- The generated file contains a `pub static GIT_VERSION` instance. 
