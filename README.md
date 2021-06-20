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

**Note that a `GitVersion.yml` configuration file might be required in your repo.**
See the [GitVersion.yml](GitVersion.yml) of this project for an example.

## Usage

Add `dotnet-gitversion` to your build dependencies:

```toml
[build-dependencies]
dotnet-gitversion-build = { git = "https://github.com/sunsided/dotnet-gitversion-rs" }
```

Create or update your `build.rs` to call `dotnet_gitversion_build::build()`.
This method creates the `gitversion.rs` file in the `OUT_DIR` directory 
and additionally returns the intermediate representation, allowing to 
access the GitVersion information in subsequent build steps:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _gv = dotnet_gitversion_build::build()?;
    Ok(())
}
```

After including the generated file you have access to the static `GIT_VERSION` constant.

```rust
include!(concat!(env!("OUT_DIR"), "/gitversion.rs"));

fn main() {
    // Use the "global" constant.
    println!("Display:      {}", GIT_VERSION);
    println!("Debug:        {:?}", GIT_VERSION);
    println!("SHA:          {}", GIT_VERSION.sha);
    println!("Commit:       {}", GIT_VERSION.commit_date);

    // The GitVersion::new() function allows you to obtain
    // the struct as a constant.
    const GV: GitVersion = GitVersion::new();
    println!("Branch name:  {}", GV.branch_name);

    // Alternatively you can use the Default trait to obtain a new instance.
    let gv = GitVersion::default();
    println!("Short commit: {}", gv.short_sha);
}
```

Example output of the above code:

```text
Display:      0.2.0
Debug:        0.2.0+Branch.main.Sha.2e3c96c6dbd30a0ca25e51d2fb10982042670a46
SHA:          2e3c96c6dbd30a0ca25e51d2fb10982042670a46
Commit:       2021-06-20
Branch name:  main
Short commit: 2e3c96c
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
    #[deprecated]
    pub nuget_version_v2: &'static str,
    #[deprecated]
    pub nuget_version: &'static str,
    #[deprecated]
    pub nuget_prerelease_tag_v2: &'static str,
    #[deprecated]
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
