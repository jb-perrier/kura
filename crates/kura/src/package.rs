use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
pub enum PackageType {
    Github,
    Local,
    Name,
}

pub fn detect_package_type(source: &str) -> PackageType {
    if source.starts_with("https://github.com/") {
        PackageType::Github
    } else if source.contains('/') || source.contains('\\') || PathBuf::from(source).exists() {
        PackageType::Local
    } else {
        PackageType::Name
    }
}

pub fn extract_package_name(source: &str, package_type: &PackageType) -> String {
    match package_type {
        PackageType::Github => {
            // Extract repo name from URL (e.g., "https://github.com/user/repo" -> "repo")
            source
                .trim_end_matches(".git")
                .split('/')
                .next_back()
                .unwrap_or(source)
                .to_string()
        }
        PackageType::Local => {
            // Extract directory name from path
            PathBuf::from(source)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or(source)
                .to_string()
        }
        PackageType::Name => source.to_string(),
    }
}
