use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::config::get_kura_path;

#[derive(Serialize, Deserialize)]
pub struct Package {
    name: String,
    kind: PackageKind,
    local_path: String,
}

impl Package {
    pub fn from_source(source: &str) -> anyhow::Result<Self> {
        let kind = PackageKind::from_source(source)?;
        let name = find_package_name(source, &kind);
        let local_path = match &kind {
            PackageKind::Local => {
                let absolute_path = std::fs::canonicalize(source)
                    .map_err(|_| anyhow::anyhow!("Invalid local path: {}", source))?;
                absolute_path.to_string_lossy().to_string()
            }
            PackageKind::Github(_) => get_kura_path()?.join("crates").join(&name).to_string_lossy().to_string(),
        };
        Ok(Self { name, kind, local_path })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn kind(&self) -> &PackageKind {
        &self.kind
    }

    pub fn local_path(&self) -> &str {
        &self.local_path
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum PackageKind {
    Github(String),
    Local,
}

impl PackageKind {
    pub fn from_source(source: &str) -> anyhow::Result<Self> {
        if source.starts_with("https://github.com/") {
            Ok(Self::Github(source.to_string()))
        } else if source.contains('/') || source.contains('\\') || PathBuf::from(source).exists() {
            Ok(Self::Local)
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
            Self::Local => "Local",
        }
    }
}

pub fn find_package_name(source: &str, package_type: &PackageKind) -> String {
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
        PackageKind::Local => {
            // Extract directory name from path
            PathBuf::from(source)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or(source)
                .to_string()
        }
    }
}
