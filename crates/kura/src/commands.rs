use std::fs;
use std::process::Command;

use anyhow::anyhow;

use crate::build::build_rust_project;
use crate::config::{load_config, save_config};
use crate::package::{Package, PackageKind};

pub fn install_package(source: &str) -> anyhow::Result<()> {
    let mut config = load_config()?;
    let package = Package::from_source(source)?;

    if config
        .packages
        .iter()
        .any(|p| p.name() == package.name())
    {
        println!("Package '{}' is already installed.", package.name());
        return Ok(());
    }

    match package.kind() {
        PackageKind::Github(repo) => {
            let dep_dir = dirs::data_dir()
                .unwrap_or_else(|| dirs::home_dir().expect("Could not find home directory"))
                .join("kura")
                .join("crates");
            fs::create_dir_all(&dep_dir).expect("Failed to create directory");
            Command::new("git")
                .args(["clone", repo, package.name()])
                .current_dir(dep_dir)
                .output()
                .expect("Failed to clone GitHub repository");
        }
        PackageKind::Local(_) => {}
    }

    println!(
        "Installed package: {} ({})",
        package.name(),
        package.kind().name()
    );
    config.packages.push(package);
    save_config(&config)?;
    Ok(())
}

pub fn remove_package(name: &str) -> anyhow::Result<()> {
    let mut config = load_config()?;
    let initial_len = config.packages.len();
    config.packages.retain(|package| package.name() != name);

    if config.packages.len() < initial_len {
        save_config(&config)?;
        println!("Removed package: {name}");
    } else {
        eprintln!("Package '{name}' not found.");
    }
    Ok(())
}

pub fn clean_project() -> anyhow::Result<()> {
    let project_path = dirs::data_dir()
        .ok_or_else(|| anyhow!("Could not find app directory"))?
        .join("kura")
        .join("crates")
        .join("kura-koto");

    if project_path.exists() {
        fs::remove_dir_all(&project_path)?;
    }

    println!("Cleaned project 'kura-koto' at: {}", project_path.display());
    Ok(())
}

pub fn list_packages() -> anyhow::Result<()> {
    let config = load_config()?;

    if config.packages.is_empty() {
        println!("No packages installed.");
    } else {
        println!("Installed packages:");
        for package in config.packages {
            println!("- {} ({})", package.name(), package.kind().name());
        }
    }
    Ok(())
}

pub fn run_project(filename: &str) -> anyhow::Result<()> {
    let project_path = dirs::data_dir()
        .ok_or_else(|| anyhow!("Could not find app directory"))?
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
    let mut child = Command::new(executable_path).arg(filename).spawn()?;

    child.wait()?;
    Ok(())
}
