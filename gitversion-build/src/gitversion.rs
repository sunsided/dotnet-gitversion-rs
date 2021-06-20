use serde::Deserialize;
use std::fmt::{Debug, Display, Formatter};

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct GitVersion {
    /// The major version. Should be incremented on breaking changes.
    #[serde(rename = "Major")]
    pub major: u32,

    /// The minor version. Should be incremented on new features.
    #[serde(rename = "Minor")]
    pub minor: u32,

    /// The patch version. Should be incremented on bug fixes.
    #[serde(rename = "Patch")]
    pub patch: u32,

    /// The pre-release tag is the pre-release label suffixed by the `pre_release_number`.
    #[serde(rename = "PreReleaseTag")]
    pub pre_release_tag: String,

    /// The pre-release tag prefixed with a dash.
    #[serde(rename = "PreReleaseTagWithDash")]
    pub pre_release_tag_with_dash: String,

    /// The pre-release label.
    #[serde(rename = "PreReleaseLabel")]
    pub pre_release_label: String,

    /// The pre-release label prefixed with a dash.
    #[serde(rename = "PreReleaseLabelWithDash")]
    pub pre_release_label_with_dash: String,

    /// The pre-release number.
    #[serde(rename = "PreReleaseNumber")]
    pub pre_release_number: Option<u32>,

    /// A summation of branch specific `pre-release-weight` and the `pre_release_number`.
    /// Can be used to obtain a monotonically increasing version number across the branches.
    #[serde(rename = "WeightedPreReleaseNumber")]
    pub weighted_pre_release_number: u32,

    /// The build metadata, usually representing number of commits since the `version_source_sha`.
    #[serde(rename = "BuildMetaData")]
    pub build_meta_data: Option<u32>,

    /// The `build_meta_data` padded with `0` up to `4` digits.
    #[serde(rename = "BuildMetaDataPadded")]
    pub build_meta_data_padded: String,

    /// The `build_meta_data` suffixed with `branch_name` and `sha`.
    #[serde(rename = "FullBuildMetaData")]
    pub full_build_meta_data: String,

    /// `major`, `minor` and `patch` joined together, separated by `.`.
    #[serde(rename = "MajorMinorPatch")]
    pub major_minor_patch: String,

    /// The semantical version number, including `pre_release_tag_with_dash` for pre-release version numbers.
    #[serde(rename = "SemVer")]
    pub semver: String,

    /// Equal to `semver`, but without a `.` separating `pre_release_label` and `pre_release_number`.
    #[serde(rename = "LegacySemVer")]
    #[deprecated]
    pub legacy_semver: String,

    /// Equal to `legacy_semver`, but with `pre_release_number` padded with `0` up to `4` digits.
    #[serde(rename = "LegacySemVerPadded")]
    #[deprecated]
    pub legacy_semver_padded: String,

    /// Defaults to `major.minor.0.0` to allow the assembly to be hotfixed without breaking
    /// existing applications that may be referencing it.
    /// (Suitable for .NET `AssemblyVersion`.)
    #[serde(rename = "AssemblySemVer")]
    #[deprecated]
    pub assembly_semver: String,

    /// Defaults to `major.minor.patch.0`.
    /// (Suitable for .NET `AssemblyFileVersion`.)
    #[serde(rename = "AssemblySemFileVer")]
    #[deprecated]
    pub assembly_sem_file_version: String,

    /// Defaults to `full_semver` suffixed by `full_build_meta_data`.
    /// (Suitable for .NET `AssemblyInformationalVersion`. )
    #[serde(rename = "InformationalVersion")]
    pub informational_version: String,

    /// The full, SemVer 2.0 compliant version number.
    #[serde(rename = "FullSemVer")]
    pub full_semver: String,

    /// The name of the checked out Git branch.
    #[serde(rename = "BranchName")]
    pub branch_name: String,

    /// Equal to `branch_name`, but with `/` replaced with `-`.
    #[serde(rename = "EscapedBranchName")]
    pub escaped_branch_name: String,

    /// The SHA of the Git commit.
    #[serde(rename = "Sha")]
    pub sha: String,

    /// The `sha` limited to `7` characters.
    #[serde(rename = "ShortSha")]
    pub short_sha: String,

    /// A NuGet 2.0 compatible version number.
    #[serde(rename = "NuGetVersionV2")]
    #[deprecated]
    pub nuget_version_v2: String,

    /// A NuGet 1.0 compatible version number.
    #[serde(rename = "NuGetVersion")]
    #[deprecated]
    pub nuget_version: String,

    /// A NuGet 2.0 compatible `pre_release_tag`.
    #[serde(rename = "NuGetPreReleaseTagV2")]
    #[deprecated]
    pub nuget_prerelease_tag_v2: String,

    /// A NuGet 1.0 compatible `pre_release_tag`.
    #[serde(rename = "NuGetPreReleaseTag")]
    #[deprecated]
    pub nuget_prerelease_tag: String,

    /// The SHA of the commit used as version source.
    #[serde(rename = "VersionSourceSha")]
    pub version_source_sha: String,

    /// The number of commits since the version source.
    #[serde(rename = "CommitsSinceVersionSource")]
    pub commits_since_version_source: u32,

    /// The `commits_since_version_source` padded with `0` up to `4` digits.
    #[serde(rename = "CommitsSinceVersionSourcePadded")]
    pub commits_since_version_source_padded: String,

    /// The ISO-8601 formatted date of the commit identified by `sha`.
    #[serde(rename = "UncommittedChanges")]
    pub uncommitted_changes: u32,

    /// The number of uncommitted changes present in the repository.
    #[serde(rename = "CommitDate")]
    pub commit_date: String,
}

impl Display for GitVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.full_semver)
    }
}

impl Debug for GitVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.informational_version)
    }
}
