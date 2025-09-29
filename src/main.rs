use clap::Parser;

// Declare modules
mod path;
mod file;
mod commands;
mod template;
mod languages;

// Import from modules
use file::ensure_all_storage_dirs;
use commands::{Commands, handle_command};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const LICENSE: &str = include_str!("../LICENSE");

#[derive(Parser)]
#[command(name = "template-rs")]
#[command(version = VERSION)]
#[command(author = AUTHORS)]
#[command(about = "A Rust template CLI application")]
#[command(arg_required_else_help = true)]
struct Cli {
    /// Display the license information
    #[arg(long)]
    license: bool,
    
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    // Ensure storage directories exist at startup
    if let Err(e) = ensure_all_storage_dirs() {
        eprintln!("Error creating storage directories: {e}");
        std::process::exit(1);
    }

    let cli = Cli::parse();

    // Handle license flag first
    if cli.license {
        println!("{LICENSE}");
        return;
    }

    // Handle subcommands
    if let Some(command) = cli.command {
        if let Err(e) = handle_command(command) {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}
