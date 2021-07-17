mod gitversion;

use anyhow::Result;
use gitversion::GitVersion;
use quote::quote;
use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::Path;
use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] std::io::Error),

    #[error("environment variable is missing")]
    MissingEnvVar,
}

fn same_content_as(path: &Path, content: &str) -> Result<bool> {
    let mut f = File::open(path)?;
    let mut current = String::new();
    f.read_to_string(&mut current)?;

    Ok(current == content)
}

#[allow(unused)]
macro_rules! include_gitversion_from_path {
    ($path: tt) => {
        include!($path);
    };
}

/// Calls `dotnet-gitversion`, converts the JSON output and generates a `gitversion.rs`
/// file in the `OUT_DIR` directory.
pub fn build() -> Result<GitVersion> {
    let path = env::var_os("OUT_DIR").ok_or(Error::MissingEnvVar)?;
    let path: &Path = path.as_ref();
    let path = path.join("gitversion.rs");
    write_version_file(path.as_path())
}

fn dotnet_gitversion() -> Option<String> {
    Command::new("dotnet-gitversion")
        .args(&["/nofetch"])
        .output()
        .ok()
        .and_then(|out| {
            std::str::from_utf8(&out.stdout[..])
                .map(str::trim)
                .map(str::to_owned)
                .ok()
        })
}

/// Write version.rs file to OUT_DIR
#[allow(deprecated)]
fn write_version_file(path: &Path) -> Result<GitVersion> {
    let content = if let Some(json) = dotnet_gitversion() {
        json.to_owned()
    } else {
        "{}".to_owned()
    };

    let gv: GitVersion = serde_json::from_str(content.as_str())?;

    let major = gv.major;
    let minor = gv.minor;
    let patch = gv.patch;
    println!("cargo:rustc-env=GITVERSION_MAJOR={}", major.to_string());
    println!("cargo:rustc-env=GITVERSION_MINOR={}", minor.to_string());
    println!("cargo:rustc-env=GITVERSION_PATCH={}", patch.to_string());

    let pre_release_tag = gv.pre_release_tag.clone();
    let pre_release_tag_with_dash = gv.pre_release_tag_with_dash.clone();
    println!(
        "cargo:rustc-env=GITVERSION_PRE_RELEASE_TAG={}",
        pre_release_tag.clone()
    );
    println!(
        "cargo:rustc-env=GITVERSION_PRE_RELEASE_TAG_WITH_DASH={}",
        pre_release_tag_with_dash.clone()
    );

    let pre_release_label = gv.pre_release_label.clone();
    let pre_release_label_with_dash = gv.pre_release_label_with_dash.clone();
    println!(
        "cargo:rustc-env=GITVERSION_PRE_RELEASE_LABEL={}",
        pre_release_label.clone()
    );
    println!(
        "cargo:rustc-env=GITVERSION_PRE_RELEASE_LABEL_WITH_DASH={}",
        pre_release_label_with_dash.clone()
    );

    let has_pre_release_number = gv.pre_release_number != None;
    let pre_release_number = gv.pre_release_number.unwrap_or(0);
    if let Some(number) = gv.pre_release_number {
        println!(
            "cargo:rustc-env=GITVERSION_PRE_RELEASE_NUMBER={}",
            number.to_string()
        );
    }

    let weighted_pre_release_number = gv.weighted_pre_release_number;
    println!(
        "cargo:rustc-env=GITVERSION_WEIGHTED_PRE_RELEASE_NUMBER={}",
        weighted_pre_release_number.to_string()
    );

    let has_build_meta_data = gv.build_meta_data != None;
    let build_meta_data = gv.build_meta_data.unwrap_or(0);
    if let Some(number) = gv.build_meta_data {
        println!(
            "cargo:rustc-env=GITVERSION_BUILD_META_DATA={}",
            number.to_string()
        );
    }

    let build_meta_data_padded = gv.build_meta_data_padded.clone();
    println!(
        "cargo:rustc-env=GITVERSION_BUILD_META_DATA_PADDED={}",
        build_meta_data_padded.clone()
    );

    let full_build_meta_data = gv.full_build_meta_data.clone();
    println!(
        "cargo:rustc-env=GITVERSION_FULL_BUILD_META_DATA={}",
        full_build_meta_data.clone()
    );

    let major_minor_patch = gv.major_minor_patch.clone();
    println!(
        "cargo:rustc-env=GITVERSION_MAJOR_MINOR_PATCH={}",
        major_minor_patch.clone()
    );

    let semver = gv.semver.clone();
    println!("cargo:rustc-env=GITVERSION_SEMVER={}", semver.clone());

    let legacy_semver = gv.legacy_semver.clone();
    let legacy_semver_padded = gv.legacy_semver_padded.clone();
    println!(
        "cargo:rustc-env=GITVERSION_LEGACY_SEMVER={}",
        legacy_semver.clone()
    );
    println!(
        "cargo:rustc-env=GITVERSION_LEGACY_SEMVER_PADDED={}",
        legacy_semver_padded.clone()
    );

    let assembly_semver = gv.assembly_semver.clone();
    println!(
        "cargo:rustc-env=GITVERSION_ASSEMBLY_SEMVER={}",
        assembly_semver.clone()
    );

    let assembly_sem_file_version = gv.assembly_sem_file_version.clone();
    println!(
        "cargo:rustc-env=GITVERSION_ASSEMBLY_SEM_FILE_VERSION={}",
        assembly_sem_file_version.clone()
    );

    let informational_version = gv.informational_version.clone();
    println!(
        "cargo:rustc-env=GITVERSION_INFORMATIONAL_VERSION={}",
        informational_version.clone()
    );

    let full_semver = gv.full_semver.clone();
    println!(
        "cargo:rustc-env=GITVERSION_FULL_SEMVER={}",
        full_semver.clone()
    );

    let branch_name = gv.branch_name.clone();
    println!(
        "cargo:rustc-env=GITVERSION_BRANCH_NAME={}",
        branch_name.clone()
    );

    let escaped_branch_name = gv.escaped_branch_name.clone();
    println!(
        "cargo:rustc-env=GITVERSION_ESCAPED_BRANCH_NAME={}",
        escaped_branch_name.clone()
    );

    let sha = gv.sha.clone();
    println!("cargo:rustc-env=GITVERSION_SHA={}", sha.clone());

    let short_sha = gv.short_sha.clone();
    println!("cargo:rustc-env=GITVERSION_SHORT_SHA={}", short_sha.clone());

    let nuget_version_v2 = gv.nuget_version_v2.clone();
    println!(
        "cargo:rustc-env=GITVERSION_NUGET_VERSION_V2={}",
        nuget_version_v2.clone()
    );

    let nuget_version = gv.nuget_version.clone();
    println!(
        "cargo:rustc-env=GITVERSION_NUGET_VERSION={}",
        nuget_version.clone()
    );

    let nuget_prerelease_tag_v2 = gv.nuget_prerelease_tag_v2.clone();
    println!(
        "cargo:rustc-env=GITVERSION_NUGET_PRERELEASE_TAG_V2={}",
        nuget_prerelease_tag_v2.clone()
    );

    let nuget_prerelease_tag = gv.nuget_prerelease_tag.clone();
    println!(
        "cargo:rustc-env=GITVERSION_NUGET_PRERELEASE_TAG={}",
        nuget_prerelease_tag.clone()
    );

    let version_source_sha = gv.version_source_sha.clone();
    println!(
        "cargo:rustc-env=GITVERSION_VERSION_SOURCE_SHA={}",
        version_source_sha.clone()
    );

    let commits_since_version_source = gv.commits_since_version_source;
    println!(
        "cargo:rustc-env=GITVERSION_COMMITS_SINCE_VERSION_SOURCE={}",
        commits_since_version_source.clone()
    );

    let commits_since_version_source_padded = gv.commits_since_version_source_padded.clone();
    println!(
        "cargo:rustc-env=GITVERSION_COMMITS_SINCE_VERSION_SOURCE_PADDED={}",
        commits_since_version_source_padded.clone()
    );

    let uncommitted_changes = gv.uncommitted_changes;
    println!(
        "cargo:rustc-env=GITVERSION_UNCOMMITTED_CHANGES={}",
        uncommitted_changes.to_string()
    );

    let commit_date = gv.commit_date.clone();
    println!(
        "cargo:rustc-env=GITVERSION_COMMIT_DATE={}",
        commit_date.clone()
    );

    let tokens = quote! {
        #[allow(dead_code)]
        pub struct GitVersion {
            /// The major version. Should be incremented on breaking changes.
            pub major: u32,
            /// The minor version. Should be incremented on new features.
            pub minor: u32,
            /// The patch version. Should be incremented on bug fixes.
            pub patch: u32,
            /// The pre-release tag is the pre-release label suffixed by the `pre_release_number`.
            pub pre_release_tag: &'static str,
            /// The pre-release tag prefixed with a dash.
            pub pre_release_tag_with_dash: &'static str,
            /// The pre-release label.
            pub pre_release_label: &'static str,
            /// The pre-release label prefixed with a dash.
            pub pre_release_label_with_dash: &'static str,
            /// The pre-release number.
            pub pre_release_number: Option<u32>,
            /// A summation of branch specific `pre-release-weight` and the `pre_release_number`.
            /// Can be used to obtain a monotonically increasing version number across the branches.
            pub weighted_pre_release_number: u32,
            /// The build metadata, usually representing number of commits since the `version_source_sha`.
            pub build_meta_data: Option<u32>,
            /// The `build_meta_data` padded with `0` up to `4` digits.
            pub build_meta_data_padded: &'static str,
            /// The `build_meta_data` suffixed with `branch_name` and `sha`.
            pub full_build_meta_data: &'static str,
            /// `major`, `minor` and `patch` joined together, separated by `.`.
            pub major_minor_patch: &'static str,
            /// The semantical version number, including `pre_release_tag_with_dash` for pre-release version numbers.
            pub semver: &'static str,
            /// Equal to `semver`, but without a `.` separating `pre_release_label` and `pre_release_number`.
            #[deprecated]
            pub legacy_semver: &'static str,
            /// Equal to `legacy_semver`, but with `pre_release_number` padded with `0` up to `4` digits.
            #[deprecated]
            pub legacy_semver_padded: &'static str,
            /// Defaults to `major.minor.0.0` to allow the assembly to be hotfixed without breaking
            /// existing applications that may be referencing it.
            /// (Suitable for .NET `AssemblyVersion`.)
            #[deprecated]
            pub assembly_semver: &'static str,
            /// Defaults to `major.minor.patch.0`.
            /// (Suitable for .NET `AssemblyFileVersion`.)
            #[deprecated]
            pub assembly_sem_file_version: &'static str,
            /// Defaults to `full_semver` suffixed by `full_build_meta_data`.
            /// (Suitable for .NET `AssemblyInformationalVersion`. )
            pub informational_version: &'static str,
            /// The full, SemVer 2.0 compliant version number.
            pub full_semver: &'static str,
            /// The name of the checked out Git branch.
            pub branch_name: &'static str,
            /// Equal to `branch_name`, but with `/` replaced with `-`.
            pub escaped_branch_name: &'static str,
            /// The SHA of the Git commit.
            pub sha: &'static str,
            /// The `sha` limited to `7` characters.
            pub short_sha: &'static str,
            /// A NuGet 2.0 compatible version number.
            #[deprecated]
            pub nuget_version_v2: &'static str,
            /// A NuGet 1.0 compatible version number.
            #[deprecated]
            pub nuget_version: &'static str,
            /// A NuGet 2.0 compatible `pre_release_tag`.
            #[deprecated]
            pub nuget_prerelease_tag_v2: &'static str,
            /// A NuGet 1.0 compatible `pre_release_tag`.
            #[deprecated]
            pub nuget_prerelease_tag: &'static str,
            /// The SHA of the commit used as version source.
            pub version_source_sha: &'static str,
            /// The number of commits since the version source.
            pub commits_since_version_source: u32,
            /// The `commits_since_version_source` padded with `0` up to `4` digits.
            pub commits_since_version_source_padded: &'static str,
            /// The ISO-8601 formatted date of the commit identified by `sha`.
            pub uncommitted_changes: u32,
            /// The number of uncommitted changes present in the repository.
            pub commit_date: &'static str,
        }

        #[allow(dead_code)]
        impl GitVersion {
            /// Builds a `GitVersion` instance.
            #[allow(deprecated)]
            pub const fn new() -> GitVersion {
                GitVersion {
                    major: #major,
                    minor: #minor,
                    patch: #patch,
                    pre_release_tag: #pre_release_tag,
                    pre_release_tag_with_dash: #pre_release_tag_with_dash,
                    pre_release_label: #pre_release_label,
                    pre_release_label_with_dash: #pre_release_label_with_dash,
                    pre_release_number: if #has_pre_release_number { Some( #pre_release_number ) } else { None },
                    weighted_pre_release_number: #weighted_pre_release_number,
                    build_meta_data: if #has_build_meta_data { Some( #build_meta_data ) } else { None },
                    build_meta_data_padded: #build_meta_data_padded,
                    full_build_meta_data: #full_build_meta_data,
                    major_minor_patch: #major_minor_patch,
                    semver: #semver,
                    legacy_semver: #legacy_semver,
                    legacy_semver_padded: #legacy_semver_padded,
                    assembly_semver: #assembly_semver,
                    assembly_sem_file_version: #assembly_sem_file_version,
                    informational_version: #informational_version,
                    full_semver: #full_semver,
                    branch_name: #branch_name,
                    escaped_branch_name: #escaped_branch_name,
                    sha: #sha,
                    short_sha: #short_sha,
                    nuget_version_v2: #nuget_version_v2,
                    nuget_version: #nuget_version,
                    nuget_prerelease_tag_v2: #nuget_prerelease_tag_v2,
                    nuget_prerelease_tag: #nuget_prerelease_tag,
                    version_source_sha: #version_source_sha,
                    commits_since_version_source: #commits_since_version_source,
                    commits_since_version_source_padded: #commits_since_version_source_padded,
                    uncommitted_changes: #uncommitted_changes,
                    commit_date: #commit_date
                }
            }
        }

        #[allow(dead_code)]
        impl Default for GitVersion {
            fn default() -> Self {
                GitVersion::new()
            }
        }

        #[allow(dead_code)]
        pub const GIT_VERSION: GitVersion = GitVersion::new();

        impl std::fmt::Display for GitVersion {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.semver)
            }
        }

        impl std::fmt::Debug for GitVersion {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.informational_version)
            }
        }
    };

    let code = tokens.to_string();
    let is_fresh = if path.exists() {
        same_content_as(&path, &code)?
    } else {
        false
    };

    if !is_fresh {
        let mut file = BufWriter::new(File::create(&path)?);
        write!(file, "{}", code)?;
    }
    Ok(gv)
}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    pub fn json_is_created() {
        let result = dotnet_gitversion().expect("dotnet_gitversion");
        assert!(result.len() > 8);
    }

    #[test]
    pub fn write_file() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        write_version_file(file.path());
        Ok(())
    }
}
