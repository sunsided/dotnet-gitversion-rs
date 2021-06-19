mod gitversion;

use anyhow::Result;
use gitversion::GitVersion;
use quote::quote;
use std::env;
use std::fmt::{Debug, Formatter};
use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter, Read, Write};
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

macro_rules! include_gitversion_from_path {
    ($path: tt) => {
        include!($path);
    };
}

fn read_version(path: String) -> Result<GitVersion> {
    let path: &Path = path.as_ref();
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let gv: GitVersion = serde_json::from_reader(reader)?;
    Ok(gv)
}

fn dotnet_gitversion() -> Option<String> {
    Command::new("dotnet-gitversion")
        // .args(&["describe", "--tags", "--always"])
        .output()
        .ok()
        .and_then(|out| {
            std::str::from_utf8(&out.stdout[..])
                .map(str::trim)
                .map(str::to_owned)
                .ok()
        })
}

pub fn build() -> Result<()> {
    let path = env::var_os("OUT_DIR").ok_or(Error::MissingEnvVar)?;
    let path: &Path = path.as_ref();
    let path = path.join("gitversion.rs");
    write_version_file(path.as_path())
}

/// Write version.rs file to OUT_DIR
fn write_version_file(path: &Path) -> Result<()> {
    let content = if let Some(json) = dotnet_gitversion() {
        json.to_owned()
    } else {
        "{}".to_owned()
    };

    let gv: GitVersion = serde_json::from_str(content.as_str())?;

    let major = gv.major;
    let minor = gv.minor;
    let patch = gv.patch;
    let pre_release_tag = gv.pre_release_tag;
    let pre_release_tag_with_dash = gv.pre_release_tag_with_dash;
    let pre_release_label = gv.pre_release_label;
    let pre_release_label_with_dash = gv.pre_release_label_with_dash;

    let has_pre_release_number = gv.pre_release_number != None;
    let pre_release_number = gv.pre_release_number.unwrap_or(0);

    let weighted_pre_release_number = gv.weighted_pre_release_number;

    let has_build_meta_data = gv.build_meta_data != None;
    let build_meta_data = gv.build_meta_data.unwrap_or(0);

    let build_meta_data_padded = gv.build_meta_data_padded;
    let full_build_meta_data = gv.full_build_meta_data;
    let major_minor_patch = gv.major_minor_patch;
    let semver = gv.semver;
    let legacy_semver = gv.legacy_semver;
    let legacy_semver_padded = gv.legacy_semver_padded;
    let assembly_semver = gv.assembly_semver;
    let assembly_sem_file_version = gv.assembly_sem_file_version;
    let informational_version = gv.informational_version;
    let branch_name = gv.branch_name;
    let escaped_branch_name = gv.escaped_branch_name;
    let sha = gv.sha;
    let short_sha = gv.short_sha;
    let nuget_version_v2 = gv.nuget_version_v2;
    let nuget_version = gv.nuget_version;
    let nuget_prerelease_tag_v2 = gv.nuget_prerelease_tag_v2;
    let nuget_prerelease_tag = gv.nuget_prerelease_tag;
    let version_source_sha = gv.version_source_sha;
    let commits_since_version_source = gv.commits_since_version_source;
    let commits_since_version_source_padded = gv.commits_since_version_source_padded;
    let uncommitted_changes = gv.uncommitted_changes;
    let commit_date = gv.commit_date;

    let tokens = quote! {
        #[allow(dead_code)]
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

        #[allow(dead_code, deprecated)]
        pub static GIT_VERSION: GitVersion = GitVersion {
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
        };

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
    Ok(())
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
