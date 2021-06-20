# dotnet-gitversion for Rust builds

This project provides a build-time wrapper to [GitTools/GitVersion]
that embeds [Semantic Version] information obtained
from the current repository state.

## Prerequisites

This library requires both the [.NET runtime] (e.g. .NET 5)
and the [GitVersion.Tool] package to be installed globally, e.g.

```console
$ dotnet tool install --global GitVersion.Tool --version 5.6.10
```

You can verify the installation by calling either

```console
$ dotnet gitversion
$ dotnet-gitversion
```

## Usage

Add `dotnet-gitversion` to your build dependencies:

```toml
[build-dependencies]
dotnet-gitversion-build = { git = "https://github.com/sunsided/dotnet-gitversion-rs" }
```

Create or update your `build.rs`:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotnet_gitversion_build::build()?;
  Ok(())
}
```

This generates the file `gitversion.rs` in the `OUT_DIR` build directory.
After including the file you have access to the static `GIT_VERSION` variable.

```rust
include!(concat!(env!("OUT_DIR"), "/gitversion.rs"));

fn main() {
  println!("Display: {}", GIT_VERSION);
  println!("Debug:   {:?}", GIT_VERSION);
  println!("SHA:     {}", GIT_VERSION.sha);
  println!("Commit:  {}", GIT_VERSION.commit_date);
}
```

Example output of the above code:

```text
Display: 0.1.0-metrics.18
Debug:   0.1.0-metrics.18+Branch.feature-metrics.Sha.defb2e23ce68c68d3bcf45333bd8718cd73368a3
SHA:     defb2e23ce68c68d3bcf45333bd8718cd73368a3
Commit:  2021-06-19
```

The imported `GitVersion` struct itself is defined as shown below. Please
see [GitTools/GitVersion](https://github.com/GitTools/GitVersion) for
documentation on the field values or run `dotnet gitversion`.

```rust
pub struct GitVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre_release_tag: &'static str,
    pub pre_release_tag_with_dash: &'static str,
    pub pre_release_label: &'static str,
    pub pre_release_label_with_dash: &'static str,
    pub pre_release_number: Option<u32>,
    pub weighted_pre_release_number: u32,
    pub build_meta_data: Option<u32>,
    pub build_meta_data_padded: &'static str,
    pub full_build_meta_data: &'static str,
    pub major_minor_patch: &'static str,
    pub semver: &'static str,
    #[deprecated]
    pub legacy_semver: &'static str,
    #[deprecated]
    pub legacy_semver_padded: &'static str,
    pub assembly_semver: &'static str,
    pub assembly_sem_file_version: &'static str,
    pub informational_version: &'static str,
    pub branch_name: &'static str,
    pub escaped_branch_name: &'static str,
    pub sha: &'static str,
    pub short_sha: &'static str,
    pub nuget_version_v2: &'static str,
    pub nuget_version: &'static str,
    pub nuget_prerelease_tag_v2: &'static str,
    pub nuget_prerelease_tag: &'static str,
    pub version_source_sha: &'static str,
    pub commits_since_version_source: u32,
    pub commits_since_version_source_padded: &'static str,
    pub uncommitted_changes: u32,
    pub commit_date: &'static str,
}
```

[GitTools/GitVersion]: https://github.com/GitTools/GitVersion
[Semantic Version]: http://semver.org/
[GitVersion.Tool]: https://www.nuget.org/packages/GitVersion.Tool/
[.NET runtime]: https://dot.net/
