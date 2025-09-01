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
    Build,
    Run {
        #[arg(help = "Path to the file to run")]
        filename: String,
    },
    Clean,
    List,
}


fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Install { source } => install_package(&source),
        Commands::Remove { name } => remove_package(&name),
        Commands::Build => build_rust_project(),
        Commands::Run { filename } => run_project(&filename),
        Commands::Clean => clean_project(),
        Commands::List => list_packages(),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
    }
}
