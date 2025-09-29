pub mod init;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new template or project
    Init {
        /// Template to use for initialization
        #[arg(help = "Template to use for initialization")]
        template: String,
        
        /// Path where to initialize (defaults to current directory)
        #[arg(help = "Path where to initialize the template")]
        path: Option<String>,

        /// Show what would be copied without actually doing it
        #[arg(short = 'n', long, help = "Preview changes without copying files")]
        dry_run: bool,

        /// Force initialization, overwriting existing files
        #[arg(short, long, help = "Overwrite existing files without prompting")]
        force: bool,
    
        /// Delete files in destination that don't exist in template (dangerous!)
        #[arg(long, help = "Remove destination files not present in template")]
        delete: bool,
    },
}

pub fn handle_command(command: Commands) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Commands::Init { template, path, dry_run, force, delete } => {
            init::handle_init(template, path, dry_run, force, delete)
        }
    }
}
