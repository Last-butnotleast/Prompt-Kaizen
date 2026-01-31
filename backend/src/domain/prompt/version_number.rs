use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        let parts: Vec<&str> = s.split('.').collect();

        if parts.len() != 3 {
            return Err("Version must be in format 'major.minor.patch'".to_string());
        }

        let major = parts[0].parse::<u32>()
            .map_err(|_| "Invalid major version number")?;
        let minor = parts[1].parse::<u32>()
            .map_err(|_| "Invalid minor version number")?;
        let patch = parts[2].parse::<u32>()
            .map_err(|_| "Invalid patch version number")?;

        Ok(Self::new(major, minor, patch))
    }

    pub fn bump_major(&self) -> Self {
        Self::new(self.major + 1, 0, 0)
    }

    pub fn bump_minor(&self) -> Self {
        Self::new(self.major, self.minor + 1, 0)
    }

    pub fn bump_patch(&self) -> Self {
        Self::new(self.major, self.minor, self.patch + 1)
    }

    pub fn major(&self) -> u32 {
        self.major
    }

    pub fn minor(&self) -> u32 {
        self.minor
    }

    pub fn patch(&self) -> u32 {
        self.patch
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.major.cmp(&other.major)
            .then(self.minor.cmp(&other.minor))
            .then(self.patch.cmp(&other.patch))
    }
}