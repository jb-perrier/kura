mod build;
mod commands;
mod config;
mod package;

use build::build_rust_project;
use clap::{Parser, Subcommand};
use commands::{install_package, remove_package};

use crate::commands::{clean_project, list_packages, run_project};

#[derive(Parser)]
#[command(name = "kura")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

use clap::ValueEnum;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum BuildMode {
    Debug,
    Release,
}

impl Default for BuildMode {
    fn default() -> Self {
        BuildMode::Release
    }
}

#[derive(Subcommand)]
pub enum Commands {
    Install {
        #[arg(help = "Package source (URL, path, or name)")]
        source: String,
    },
    Remove {
        #[arg(help = "Package name")]
        name: String,
    },
    Build {
        #[arg(long, value_enum, help = "Run mode (debug or release)")]
        build_mode: Option<BuildMode>,
    },
    Run {
        #[arg(help = "Path to the file to run")]
        filename: String,

        #[arg(long, value_enum, help = "Run mode (debug or release)")]
        build_mode: Option<BuildMode>,
    },
    Clean,
    List,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Install { source } => install_package(&source),
        Commands::Remove { name } => remove_package(&name),
        Commands::Build { build_mode: mode } => build_rust_project(mode.unwrap_or_else(BuildMode::default)),
        Commands::Run {
            filename,
            build_mode: mode,
        } => run_project(&filename, mode.unwrap_or_else(BuildMode::default)),
        Commands::Clean => clean_project(),
        Commands::List => list_packages(),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
    }
}
