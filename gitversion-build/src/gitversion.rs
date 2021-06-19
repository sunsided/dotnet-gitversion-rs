use serde::Deserialize;
use std::fmt::{Debug, Formatter};

#[derive(Deserialize)]
pub struct GitVersion {
    #[serde(rename = "Major")]
    pub major: u32,
    #[serde(rename = "Minor")]
    pub minor: u32,
    #[serde(rename = "Patch")]
    pub patch: u32,
    #[serde(rename = "PreReleaseTag")]
    pub pre_release_tag: String,
    #[serde(rename = "PreReleaseTagWithDash")]
    pub pre_release_tag_with_dash: String,
    #[serde(rename = "PreReleaseLabel")]
    pub pre_release_label: String,
    #[serde(rename = "PreReleaseLabelWithDash")]
    pub pre_release_label_with_dash: String,
    #[serde(rename = "PreReleaseNumber")]
    pub pre_release_number: Option<u32>,
    #[serde(rename = "WeightedPreReleaseNumber")]
    pub weighted_pre_release_number: u32,
    #[serde(rename = "BuildMetaData")]
    pub build_meta_data: Option<u32>,
    #[serde(rename = "BuildMetaDataPadded")]
    pub build_meta_data_padded: String,
    #[serde(rename = "FullBuildMetaData")]
    pub full_build_meta_data: String,
    #[serde(rename = "MajorMinorPatch")]
    pub major_minor_patch: String,
    #[serde(rename = "SemVer")]
    pub semver: String,
    #[serde(rename = "LegacySemVer")]
    pub legacy_semver: String,
    #[serde(rename = "LegacySemVerPadded")]
    pub legacy_semver_padded: String,
    #[serde(rename = "AssemblySemVer")]
    pub assembly_semver: String,
    #[serde(rename = "AssemblySemFileVer")]
    pub assembly_sem_file_version: String,
    #[serde(rename = "InformationalVersion")]
    pub informational_version: String,
    #[serde(rename = "BranchName")]
    pub branch_name: String,
    #[serde(rename = "EscapedBranchName")]
    pub escaped_branch_name: String,
    #[serde(rename = "Sha")]
    pub sha: String,
    #[serde(rename = "ShortSha")]
    pub short_sha: String,
    #[serde(rename = "NuGetVersionV2")]
    pub nuget_version_v2: String,
    #[serde(rename = "NuGetVersion")]
    pub nuget_version: String,
    #[serde(rename = "NuGetPreReleaseTagV2")]
    pub nuget_prerelease_tag_v2: String,
    #[serde(rename = "NuGetPreReleaseTag")]
    pub nuget_prerelease_tag: String,
    #[serde(rename = "VersionSourceSha")]
    pub version_source_sha: String,
    #[serde(rename = "CommitsSinceVersionSource")]
    pub commits_since_version_source: u32,
    #[serde(rename = "CommitsSinceVersionSourcePadded")]
    pub commits_since_version_source_padded: String,
    #[serde(rename = "UncommittedChanges")]
    pub uncommitted_changes: u32,
    #[serde(rename = "CommitDate")]
    pub commit_date: String,
}

impl Debug for GitVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.informational_version)
    }
}
