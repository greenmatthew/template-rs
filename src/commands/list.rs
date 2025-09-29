use crate::template::Template;

pub fn handle_list(verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    let templates = Template::discover_all()?;
    
    if templates.is_empty() {
        println!("No templates found.");
        println!("Templates should be directories in ~/.template-rs/templates/ with a .template.toml file.");
        return Ok(());
    }
    
    if verbose {
        println!("Available templates:\n");
        for template in &templates {
            println!("📋 {}", template.display_name());
            println!("   Directory: {}", template.name);
            println!("   Path: {}", template.path.display());
            
            if let Some(description) = template.description() {
                println!("   Description: {description}");
            }
            
            if let Some(author) = template.author() {
                println!("   Author: {author}");
            }
            
            if let Some(version) = template.version() {
                println!("   Version: {version}");
            }
            
            if let Some(tags) = template.tags() {
                println!("   Tags: {}", tags.join(", "));
            }
            
            println!();
        }
    } else {
        println!("Available templates:");
        for template in &templates {
            if let Some(description) = template.description() {
                println!("  {} - {description}", template.display_name());
            } else {
                println!("  {}", template.display_name());
            }
        }
    }
    
    println!("Use 'template-rs init <template-name>' to initialize a template.");
    
    Ok(())
}