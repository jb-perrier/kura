use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Package {
    name: String,
    kind: PackageKind,
}

impl Package {
    pub fn from_source(source: &str) -> anyhow::Result<Self> {
        let kind = PackageKind::from_source(source)?;
        let name = find_package_name(&kind);
        Ok(Self { name, kind })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn kind(&self) -> &PackageKind {
        &self.kind
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum PackageKind {
    Github(String),
    Local(String),
}

impl PackageKind {
    pub fn from_source(source: &str) -> anyhow::Result<Self> {
        if source.starts_with("https://github.com/") {
            Ok(Self::Github(source.to_string()))
        } else if source.contains('/') || source.contains('\\') || PathBuf::from(source).exists() {
            Ok(Self::Local(source.to_string()))
        } else {
            Err(anyhow::anyhow!(
                "Unknown package type for source: {}",
                source
            ))
        }
    }

    pub const fn name(&self) -> &'static str {
        match self {
            Self::Github(_) => "GitHub",
            Self::Local(_) => "Local",
        }
    }
}

pub fn find_package_name(package_type: &PackageKind) -> String {
    match package_type {
        PackageKind::Github(source) => {
            // Extract repo name from URL (e.g., "https://github.com/user/repo" -> "repo")
            source
                .trim_end_matches(".git")
                .split('/')
                .next_back()
                .unwrap_or(source)
                .to_string()
        }
        PackageKind::Local(source) => {
            // Extract directory name from path
            PathBuf::from(source)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or(source)
                .to_string()
        }
    }
}
