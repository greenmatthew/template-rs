use clap::{Arg, Command};

// Declare modules
mod path;
mod file;

// Import from modules
use file::ensure_all_storage_dirs;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const LICENSE: &str = include_str!("../LICENSE");

fn main() {
    // Ensure storage directories exist at startup
    if let Err(e) = ensure_all_storage_dirs() {
        eprintln!("Error creating storage directories: {e}");
        std::process::exit(1);
    }

    let matches = Command::new("template-rs")
        .version(VERSION)
        .author(AUTHORS)
        .about("A Rust template CLI application")
        .arg(
            Arg::new("license")
                .long("license")
                .help("Display the license information")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if matches.get_flag("license") {
        println!("{LICENSE}");
    }
}