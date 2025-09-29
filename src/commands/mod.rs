pub mod author;
pub mod list;
pub mod init;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Author a new template
    Author {
        /// Path where to create the new template
        #[arg(help = "Path where to create the new template")]
        path: String,
        
        /// Template name (defaults to directory name)
        #[arg(short, long, help = "Template name (defaults to directory name)")]
        name: Option<String>,
    },

    /// List available templates
    List {
        /// Show detailed information about templates
        #[arg(short, long, help = "Show detailed template information")]
        verbose: bool,

        #[arg(short, long, help = "Filter by language")]
        language: Option<String>,
    },

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

    /// Create a new project from a template
    New {
        /// Template to use for the new project
        #[arg(help = "Template to use for the new project")]
        template: String,
        
        /// Path where to create the new project
        #[arg(help = "Path where to create the new project")]
        path: String,

        /// Show what would be copied without actually doing it
        #[arg(short = 'n', long, help = "Preview changes without copying files")]
        dry_run: bool,

        /// Force creation, overwriting existing files
        #[arg(short, long, help = "Overwrite existing files without prompting")]
        force: bool,
    
        /// Delete files in destination that don't exist in template (dangerous!)
        #[arg(long, help = "Remove destination files not present in template")]
        delete: bool,
    },
}

pub fn handle_command(command: Commands) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Commands::Author { path, name } => {
            author::handle_author(&path, name)
        }
        Commands::List { verbose, language} => {
            list::handle_list(verbose, language.as_deref())
        }
        Commands::Init { template, path, dry_run, force, delete } => {
            init::handle_init(&template, path, dry_run, force, delete, false)
        }
        Commands::New { template, path, dry_run, force, delete } => {
            init::handle_init(&template, Some(path), dry_run, force, delete, true)
        }
    }
}
