use crate as mvutils;
use mvutils_proc_macro::Savable;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

#[derive(Eq, PartialEq, Copy, Clone, Savable)]
pub struct Version {
    variant: u16,
    major: u16,
    minor: u16,
    patch: u16,
}

impl Version {
    pub fn new(variant: u16, major: u16, minor: u16, patch: u16) -> Self {
        Version {
            variant,
            major,
            minor,
            patch,
        }
    }

    pub fn parse_vulkan(version: u32) -> Self {
        Version {
            variant: (version >> 29) as u16,
            major: ((version >> 22) & 0x7f) as u16,
            minor: ((version >> 12) & 0x3FF) as u16,
            patch: (version & 0xFFF) as u16,
        }
    }

    pub fn parse(version: &str) -> Option<Self> {
        if version.is_empty() {
            return None;
        }
        let digits = if version.starts_with("#version") {
            version
                .replace("#version", "")
                .replace(' ', "")
                .chars()
                .map(|c| c as u16 - 48)
                .collect::<Vec<_>>()
        } else {
            let results = version
                .replace(['v', ' '], "")
                .split('.')
                .map(u16::from_str)
                .collect::<Vec<_>>();
            if results.iter().any(Result::is_err) {
                return None;
            }
            results.into_iter().map(|e| e.unwrap()).collect::<Vec<_>>()
        };

        if digits.len() == 1 {
            Some(Version {
                variant: 0,
                major: digits[0],
                minor: 0,
                patch: 0,
            })
        } else if digits.len() == 2 {
            Some(Version {
                variant: 0,
                major: digits[0],
                minor: digits[1],
                patch: 0,
            })
        } else if digits.len() == 3 {
            Some(Version {
                variant: 0,
                major: digits[0],
                minor: digits[1],
                patch: digits[2],
            })
        } else {
            None
        }
    }

    pub fn to_glsl_string(&self) -> String {
        format!("#version {}{}{}", self.major, self.minor, self.patch)
    }

    pub fn as_vulkan_version(&self) -> u32 {
        ((self.variant as u32) << 29 | (self.major as u32) << 22)
            | ((self.minor as u32) << 12)
            | (self.patch as u32)
    }

    pub fn variant(&self) -> u16 {
        self.variant
    }

    pub fn major(&self) -> u16 {
        self.major
    }

    pub fn minor(&self) -> u16 {
        self.minor
    }

    pub fn patch(&self) -> u16 {
        self.patch
    }

    pub fn set_variant(&mut self, variant: u16) {
        self.variant = variant;
    }

    pub fn set_major(&mut self, major: u16) {
        self.major = major;
    }

    pub fn set_minor(&mut self, minor: u16) {
        self.minor = minor;
    }

    pub fn set_patch(&mut self, patch: u16) {
        self.patch = patch;
    }
}

impl Default for Version {
    fn default() -> Self {
        Version {
            variant: 0,
            major: 1,
            minor: 0,
            patch: 0,
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}.{}.{}", self.major, self.minor, self.patch).as_str())
    }
}

impl Debug for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "Version {{ major: {}, minor: {}, patch: {} }}",
                self.major, self.minor, self.patch
            )
            .as_str(),
        )
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.major
            .cmp(&other.major)
            .then(self.minor.cmp(&other.minor))
            .then(self.patch.cmp(&other.patch))
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Version {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u16(self.major);
        state.write_u16(self.minor);
        state.write_u16(self.patch);
    }
}

impl FromStr for Version {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Version::parse(s).ok_or(())
    }
}

impl TryFrom<String> for Version {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Version::parse(&value).ok_or(())
    }
}

impl From<u32> for Version {
    fn from(value: u32) -> Self {
        Version::parse_vulkan(value)
    }
}

impl From<Version> for u32 {
    fn from(value: Version) -> Self {
        value.as_vulkan_version()
    }
}
