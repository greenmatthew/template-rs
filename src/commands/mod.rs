pub mod init;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new template or project
    Init {
        /// Name of the template or project to initialize
        #[arg(help = "Template or project name")]
        name: Option<String>,
        
        /// Force initialization even if directory exists
        #[arg(short, long, help = "Force initialization, overwriting existing files")]
        force: bool,
        
        /// Initialize from a specific template
        #[arg(short, long, help = "Template to use for initialization")]
        template: Option<String>,
    },
}

pub fn handle_command(command: Commands) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Commands::Init { name, force, template } => {
            init::handle_init(name, force, template)
        }
    }
}
