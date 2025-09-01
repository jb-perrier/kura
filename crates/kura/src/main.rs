mod build;
mod commands;
mod config;
mod package;
mod run;

use build::build_rust_project;
use clap::{Parser, Subcommand};
use commands::{install_package, remove_package};
use run::run_project;

use crate::commands::{clean_project, list_packages};

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

    match cli.command {
        Commands::Install { source } => install_package(&source),
        Commands::Remove { name } => remove_package(&name),
        Commands::Build => {
            if let Err(e) = build_rust_project() {
                eprintln!("Error: {e}");
            }
        }
        Commands::Run { filename } => {
            if let Err(e) = run_project(&filename) {
                eprintln!("Error: {e}");
            }
        }
        Commands::Clean => {
            if let Err(e) = clean_project() {
                eprintln!("Error: {e}");
            }
        }
        Commands::List => list_packages(),
    }
}
