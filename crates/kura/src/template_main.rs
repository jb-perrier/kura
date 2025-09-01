use std::env;
use std::fs;

use koto::{Result, prelude::*};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <script_file>", args[0]);
        std::process::exit(1);
    }
    let script_path = &args[1];
    let script = fs::read_to_string(script_path).expect("Failed to read script file");
    let mut koto = Koto::default();

    let prelude = koto.prelude();
    // <INSERT_PRELUDE_VALUES>

    koto.compile_and_run(&script)?;

    Ok(())
}
