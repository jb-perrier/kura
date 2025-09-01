use std::fs;
use std::process::Command;

use crate::config::{load_config, save_config};
use crate::package::{detect_package_type, extract_package_name};

pub fn install_package(source: &str) {
    let mut config = load_config();

    // Check if package already exists and update or add
    if !config.packages.iter().any(|p| p == source) {
        config.packages.push(source.to_string());
    }

    save_config(&config);
    let package_type = detect_package_type(source);
    let package_name = extract_package_name(source, &package_type);

    match package_type {
        crate::package::PackageType::Github => {
            let dep_dir = dirs::data_dir()
                .unwrap_or_else(|| dirs::home_dir().expect("Could not find home directory"))
                .join("kura")
                .join("crates");
            fs::create_dir_all(&dep_dir).expect("Failed to create directory");
            Command::new("git")
                .args(["clone", source, package_name.as_str()])
                .current_dir(dep_dir)
                .output()
                .expect("Failed to clone GitHub repository");
        }
        crate::package::PackageType::Name => {
            unimplemented!("Installing packages by name is not implemented yet.");
        }
        crate::package::PackageType::Local => {}
    }
    println!("Installed package: {package_name} ({package_type:?})");
}

pub fn remove_package(name: &str) {
    let mut config = load_config();
    let initial_len = config.packages.len();
    config.packages.retain(|package_source| {
        let package_type = detect_package_type(package_source);
        let package_name = extract_package_name(package_source, &package_type);
        package_name != name
    });

    if config.packages.len() < initial_len {
        save_config(&config);
        println!("Removed package: {name}");
    } else {
        eprintln!("Package '{name}' not found.");
    }
}

pub fn clean_project() -> Result<(), Box<dyn std::error::Error>> {
    let project_path = dirs::data_dir()
        .unwrap_or_else(|| dirs::home_dir().expect("Could not find home directory"))
        .join("kura")
        .join("crates")
        .join("kura-koto");

    if project_path.exists() {
        fs::remove_dir_all(&project_path)?;
    }

    println!("Cleaned project 'kura-koto' at: {}", project_path.display());
    Ok(())
}

pub fn list_packages() -> () {
    let config = load_config();

    if config.packages.is_empty() {
        println!("No packages installed.");
    } else {
        println!("Installed packages:");
        for package in config.packages {
            let package_type = detect_package_type(&package);
            let package_name = extract_package_name(&package, &package_type);
            println!("- {package_name} ({package_type:?})");
        }
    }
}
