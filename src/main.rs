use clap::Parser;
use clap::builder::styling::{AnsiColor, Effects, Styles};

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

fn custom_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Green.on_default() | Effects::BOLD)
        .usage(AnsiColor::Green.on_default() | Effects::BOLD)
        .literal(AnsiColor::Cyan.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Blue.on_default())
        .error(AnsiColor::Red.on_default() | Effects::BOLD)
        .valid(AnsiColor::Cyan.on_default() | Effects::BOLD)
        .invalid(AnsiColor::Yellow.on_default() | Effects::BOLD)
}

#[derive(Parser)]
#[command(name = env!("CARGO_BIN_NAME"))]
#[command(version = VERSION)]
#[command(author = AUTHORS)]
#[command(about = "A Rust CLI tool for managing programming templates")]
#[command(arg_required_else_help = true)]
#[command(styles = custom_styles())]
struct Cli {
    /// Display detailed information about this tool
    #[arg(long)]
    about: bool,

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

    // Handle about flag first
    if cli.about {
        println!("template-rs v{VERSION}");
        println!("Created by: {AUTHORS}\n");
        println!("🌐 Website: https://matthewgreen.gg/");
        println!("📦 GitHub: https://github.com/greenmatthew/template-rs");
        println!("🔧 Gitea: https://git.matthewgreen.gg/mgreen/template-rs\n");
        println!("Use `{} --license` to view the license", env!("CARGO_BIN_NAME"));
        return;
    }

    // Handle license flag first
    if cli.license {
        println!("{LICENSE}");
        return;
    }

    // Handle subcommands
    if let Some(command) = cli.command
        && let Err(e) = handle_command(command) {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
}
