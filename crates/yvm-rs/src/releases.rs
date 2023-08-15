use once_cell::sync::Lazy;
use semver::Version;
use serde::{
    de::{self, Deserializer},
    Deserialize, Serialize,
};
use std::{collections::BTreeMap, sync::Arc};
use url::Url;

use crate::{error::YlemVmError, platform::Platform};

const YLEM_RELEASES_URL: &str = "https://github.com/core-coin/ylem/releases/download";

static YLEM_LINUX_AARCH_RELEASES: Lazy<Arc<Releases>> = Lazy::new(|| {
    Arc::new(
        serde_json::from_str::<Releases>(include_str!("../list/LinuxAarchList.json"))
            .unwrap_or_else(|_| panic!("{}", parse_error_msg("Linux Aarch"))),
    )
});

static YLEM_LINUX_AMD_RELEASES: Lazy<Releases> = Lazy::new(|| {
    serde_json::from_str::<Releases>(include_str!("../list/LinuxAmdList.json"))
        .unwrap_or_else(|_| panic!("{}", parse_error_msg("Linux Amd")))
});

static YLEM_MAC_AMD_RELEASES: Lazy<Releases> = Lazy::new(|| {
    serde_json::from_str::<Releases>(include_str!("../list/MacAmdList.json"))
        .unwrap_or_else(|_| panic!("{}", parse_error_msg("Mac Amd")))
});

static YLEM_MAC_AARCH_RELEASES: Lazy<Releases> = Lazy::new(|| {
    serde_json::from_str::<Releases>(include_str!("../list/MacAarchList.json"))
        .unwrap_or_else(|_| panic!("{}", parse_error_msg("Mac Aarch")))
});

static YLEM_WINDOWS_RELEASES: Lazy<Releases> = Lazy::new(|| {
    serde_json::from_str::<Releases>(include_str!("../list/WindowsList.json"))
        .unwrap_or_else(|_| panic!("{}", parse_error_msg("Windows")))
});

fn parse_error_msg(platform: &str) -> String {
    format!(
        "Failed to parse ylem releases for {}. Please contact maintainers",
        platform
    )
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Releases {
    pub builds: Vec<BuildInfo>,
    pub releases: BTreeMap<Version, String>,
}

impl Releases {
    /// Get the checksum of a ylem version's binary if it exists.
    pub fn get_checksum(&self, v: &Version) -> Option<Vec<u8>> {
        for build in self.builds.iter() {
            if build.version.eq(v) {
                return Some(build.sha256.clone());
            }
        }
        None
    }

    /// Returns the artifact of the version if any
    pub fn get_artifact(&self, version: &Version) -> Option<&String> {
        self.releases.get(version)
    }

    /// Returns a sorted list of all versions
    pub fn into_versions(self) -> Vec<Version> {
        let mut versions = self.releases.into_keys().collect::<Vec<_>>();
        versions.sort_unstable();
        versions
    }
}

/// Build info contains the SHA256 checksum of a ylem binary.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildInfo {
    pub version: Version,
    #[serde(with = "hex_string")]
    pub sha256: Vec<u8>,
}

/// Helper serde module to serialize and deserialize bytes as hex.
mod hex_string {
    use super::*;
    use serde::Serializer;
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str_hex = String::deserialize(deserializer)?;
        let str_hex = str_hex.trim_start_matches("0x");
        hex::decode(str_hex).map_err(|err| de::Error::custom(err.to_string()))
    }

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: AsRef<[u8]>,
    {
        let value = hex::encode(value);
        serializer.serialize_str(&value)
    }
}

/// Fetch all releases available for the provided platform.
pub fn all_releases(platform: Platform) -> Result<&'static Releases, YlemVmError> {
    match platform {
        Platform::LinuxAarch64 => Ok(&*YLEM_LINUX_AARCH_RELEASES),
        Platform::LinuxAmd64 => Ok(&*YLEM_LINUX_AMD_RELEASES),
        Platform::MacOsAarch64 => Ok(&*YLEM_MAC_AARCH_RELEASES),
        Platform::MacOsAmd64 => Ok(&*YLEM_MAC_AMD_RELEASES),
        Platform::WindowsAmd64 => Ok(&*YLEM_WINDOWS_RELEASES),
        Platform::Unsupported => Err(YlemVmError::UnsupportedOs(platform.to_string())),
    }
}

/// Construct the URL to the Ylem binary for the specified release version and target platform.
pub fn artifact_url(
    _platform: Platform,
    version: &Version,
    artifact: &str,
) -> Result<Url, YlemVmError> {
    Ok(Url::parse(&format!(
        "{YLEM_RELEASES_URL}/{version}/{artifact}"
    ))?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_macos_aarch64() {
        let releases = all_releases(Platform::MacOsAarch64)
            .expect("could not fetch releases for macos-aarch64");
        let rosetta = Version::new(1, 0, 0);
        let url1 = artifact_url(
            Platform::MacOsAarch64,
            &rosetta,
            releases.get_artifact(&rosetta).unwrap(),
        )
        .expect("could not fetch artifact URL");

        assert!(url1.to_string().contains(YLEM_RELEASES_URL));
    }

    #[tokio::test]
    async fn test_all_releases_macos_amd64() {
        assert!(all_releases(Platform::MacOsAmd64).is_ok());
    }

    #[tokio::test]
    async fn test_all_releases_macos_aarch64() {
        assert!(all_releases(Platform::MacOsAarch64).is_ok());
    }

    #[tokio::test]
    async fn test_all_releases_linux_amd64() {
        assert!(all_releases(Platform::LinuxAmd64).is_ok());
    }

    #[tokio::test]
    async fn test_all_releases_linux_aarch64() {
        assert!(all_releases(Platform::LinuxAarch64).is_ok());
    }

    #[tokio::test]
    async fn releases_roundtrip() {
        let releases = all_releases(Platform::LinuxAmd64).unwrap();
        let s = serde_json::to_string(&releases).unwrap();
        let de_releases: Releases = serde_json::from_str(&s).unwrap();
        assert_eq!(*releases, de_releases);
    }
}
