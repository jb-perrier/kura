use std::process::Command;

use crate::build::build_rust_project;

pub fn run_project(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let project_path = dirs::data_dir()
        .unwrap_or_else(|| dirs::home_dir().expect("Could not find home directory"))
        .join("kura")
        .join("crates")
        .join("kura-koto");

    let executable_path = if cfg!(target_os = "windows") {
        project_path
            .join("target")
            .join("release")
            .join("kura-koto.exe")
    } else {
        project_path
            .join("target")
            .join("release")
            .join("kura-koto")
    };

    if !executable_path.exists() {
        build_rust_project()?;
    }

    let filename = std::fs::canonicalize(filename)?;
    let mut child = Command::new(executable_path)
        .arg(filename)
        .spawn()?;

    child.wait()?;
    Ok(())
}
