use crate::path::resolve_path;
use crate::template::{TemplateConfig, Template};
use std::fs;

pub fn handle_author(
    path: String,
    name: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Authoring new template...");
    
    // Resolve the target path
    let target_path = resolve_path(&path, None)?;
    println!("Target path: {}", target_path.display());
    
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
    
    // Determine template name for messaging and config
    let template_name = name.unwrap_or_else(|| {
        target_path
            .file_name()
            .and_then(|n| n.to_str())
            .map(String::from)
            .unwrap_or_else(|| "unknown".to_string())
    });
    
    println!("Creating .template.toml for template '{template_name}'");

    let sample_config = TemplateConfig {
        name: Some(template_name.clone()),
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
