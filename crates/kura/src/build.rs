use crate::config::load_config;
use crate::package::{PackageType, detect_package_type, extract_package_name};
use std::fs;
use std::path::Path;
use std::process::Command;

const TEMPLATE_MAIN: &str = include_str!("template_main.rs");

pub fn load_cargo_toml<P: AsRef<Path>>(
    folder_path: P,
) -> Result<toml::Value, Box<dyn std::error::Error>> {
    let cargo_toml_path = folder_path.as_ref().join("Cargo.toml");

    if !cargo_toml_path.exists() {
        return Err(format!("Cargo.toml not found in {}", folder_path.as_ref().display()).into());
    }

    let content = fs::read_to_string(&cargo_toml_path)?;
    let toml_value = content.parse::<toml::Value>()?;

    Ok(toml_value)
}

pub fn build_rust_project() -> Result<(), Box<dyn std::error::Error>> {
    let project_path = dirs::data_dir()
        .unwrap_or_else(|| dirs::home_dir().expect("Could not find home directory"))
        .join("kura")
        .join("crates")
        .join("kura-koto");

    // Remove existing project if it exists
    if project_path.exists() {
        fs::remove_dir_all(&project_path)?;
    }

    // Create project directory
    fs::create_dir_all(&project_path)?;

    // Initialize Rust binary project
    let init_output = Command::new("cargo")
        .args(["init", "--bin", "--name", "kura-koto"])
        .current_dir(&project_path)
        .output()?;

    if !init_output.status.success() {
        return Err(format!(
            "Failed to initialize project: {}",
            String::from_utf8_lossy(&init_output.stderr)
        )
        .into());
    }

    // Add koto dependency
    let add_koto_output = Command::new("cargo")
        .args(["add", "koto"])
        .current_dir(&project_path)
        .output()?;

    if !add_koto_output.status.success() {
        return Err(format!(
            "Failed to add 'koto' dependency: {}",
            String::from_utf8_lossy(&add_koto_output.stderr)
        )
        .into());
    }

    // Load config and add path dependencies to Cargo.toml
    let config = load_config();
    let cargo_toml_path = project_path.join("Cargo.toml");
    let mut insert_prelude_values = Vec::new();

    if !config.packages.is_empty() {
        let mut cargo_toml = load_cargo_toml(&project_path)?;

        // Initialize dependencies section if it doesn't exist
        if !cargo_toml
            .as_table()
            .expect("Invalid TOML structure")
            .contains_key("dependencies")
        {
            cargo_toml
                .as_table_mut()
                .expect("Invalid TOML structure")
                .insert(
                    "dependencies".to_string(),
                    toml::Value::Table(toml::map::Map::new()),
                );
        }

        // Add path dependencies
        if let Some(dependencies) = cargo_toml
            .get_mut("dependencies")
            .and_then(|d| d.as_table_mut())
        {
            for package_source in &config.packages {
                let package_type = detect_package_type(package_source);
                let package_name = extract_package_name(package_source, &package_type);
                let dep_value = toml::Value::Table({
                    let mut table = toml::map::Map::new();
                    table.insert(
                        "path".to_string(),
                        toml::Value::String(package_source.clone()),
                    );
                    table
                });
                dependencies.insert(package_name.clone(), dep_value);
                let package_name_underscore = package_name.replace('-', "_");
                insert_prelude_values.push(format!(
                        "prelude.insert(\"{package_name_underscore}\", {package_name_underscore}::make_module());"
                    ));
            }
        }

        // Write updated Cargo.toml
        let updated_content = toml::to_string_pretty(&cargo_toml)?;
        fs::write(&cargo_toml_path, updated_content)?;
    }

    let template = TEMPLATE_MAIN.to_string();
    let inserts = insert_prelude_values.join("\n");
    let main_rs = template.replace("// <INSERT_PRELUDE_VALUES>", &inserts);

    let main_rs_path = project_path.join("src").join("main.rs");
    fs::write(&main_rs_path, main_rs)?;

    // Build the project
    let build_output = Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(&project_path)
        .output()?;

    if !build_output.status.success() {
        return Err(format!(
            "Failed to build project: {}",
            String::from_utf8_lossy(&build_output.stderr)
        )
        .into());
    }

    println!("Built project 'kura-koto' at: {}", project_path.display());
    Ok(())
}
