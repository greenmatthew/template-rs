pub mod author;
pub mod list;
pub mod init;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new template for reuse
    Author {
        /// Path where the template will be created
        #[arg(help = "Path where the template will be created")]
        path: String,
        
        /// Name for the template (defaults to directory name)
        #[arg(short, long, help = "Name for the template (defaults to directory name)")]
        name: Option<String>,
    },

    /// List all available templates
    List {
        /// Show detailed information for each template
        #[arg(short, long, help = "Show detailed information for each template")]
        verbose: bool,

        /// Filter templates by programming language
        #[arg(short, long, help = "Filter templates by programming language")]
        language: Option<String>,
    },

    /// Initialize existing directory using an existing template
    Init {
        /// Name of the template to use
        #[arg(help = "Name of the template to use")]
        template: String,
        
        /// Target directory (defaults to current directory)
        #[arg(help = "Target directory (defaults to current directory)")]
        path: Option<String>,

        /// Preview changes without copying files
        #[arg(short = 'n', long, help = "Preview changes without copying files")]
        dry_run: bool,

        /// Overwrite existing files
        #[arg(short, long, help = "Overwrite existing files")]
        force: bool,
    
        /// Remove files not present in template
        #[arg(long, help = "Remove files not present in template")]
        delete: bool,
    },

    /// Create a new directory using an existing template
    New {
        /// Name of the template to use
        #[arg(help = "Name of the template to use")]
        template: String,
        
        /// Path where the new directory will be created
        #[arg(help = "Path where the new directory will be created")]
        path: String,

        /// Preview changes without copying files
        #[arg(short = 'n', long, help = "Preview changes without copying files")]
        dry_run: bool,

        /// Overwrite existing files
        #[arg(short, long, help = "Overwrite existing files")]
        force: bool,
    
        /// Remove files not present in template
        #[arg(long, help = "Remove files not present in template")]
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
