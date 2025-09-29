use crate::path::resolve_path;
use crate::template::{Template, TemplateConfig};
use std::fs;
use std::path::Path;

pub fn handle_author(
    path: String,
    name: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Authoring new template...");
    
    // Resolve the target path
    let target_path = resolve_path(&path, None)?;
    println!("Target path: {}", target_path.display());
    
    // Determine template name - use provided name or infer from directory
    let template_name = match name {
        Some(n) => n,
        None => {
            target_path
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or("Could not determine template name from path")?
                .to_string()
        }
    };
    
    // Create the directory if it doesn't exist
    if !target_path.exists() {
        println!("Creating directory: {}", target_path.display());
        fs::create_dir_all(&target_path)?;
    } else if !target_path.is_dir() {
        return Err(format!("Path exists but is not a directory: {}", target_path.display()).into());
    }
    
    // Check if .template.toml already exists
    let config_path = target_path.join(".template.toml");
    if config_path.exists() {
        return Err(format!("Template already exists at {}", target_path.display()).into());
    }
    
    // Create the .template.toml file
    println!("Creating .template.toml for template '{template_name}'");

    let sample_config = TemplateConfig {
        name: Some(template_name.to_string()),
        description: Some(format!("A template for {template_name}")),
        author: Some("Your Name".to_string()),
        version: Some("1.0.0".to_string()),
        tags: Some(vec!["project".to_string(), "template".to_string()]),
        min_tool_version: Some("0.1.0".to_string()),
        metadata: None,
    };

    Template::save_config(&sample_config, config_path, false)?;
    
    println!("✅ Template '{template_name}' created successfully!");
    println!("📝 Edit .template.toml to customize your template metadata");
    println!("📁 Add your template files to {}", target_path.display());
    
    Ok(())
}
