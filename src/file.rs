use crate::path::{PERSISTENT_STORAGE, TEMPLATE_STORAGE, resolve_path};
use crate::template::{Template, TemplateConfig};
use std::fs;
use std::path::{Path, PathBuf};

const TEMPLATE_CONFIG_FILE: &str = ".template.toml";

/// Creates a directory and all parent directories if they don't exist
pub fn create_dir_if_missing<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
    let path = path.as_ref();
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Ensures the persistent storage directory exists, returns the resolved path
pub fn ensure_persistent_storage_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let storage_dir = resolve_path(PERSISTENT_STORAGE, None)?;
    create_dir_if_missing(&storage_dir)?;
    Ok(storage_dir)
}

/// Ensures the template storage directory exists, returns the resolved path
pub fn ensure_template_storage_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let template_dir = resolve_path(TEMPLATE_STORAGE, None)?;
    create_dir_if_missing(&template_dir)?;
    Ok(template_dir)
}

/// Ensures both storage directories exist
pub fn ensure_all_storage_dirs() -> Result<(), Box<dyn std::error::Error>> {
    ensure_persistent_storage_dir()?;
    ensure_template_storage_dir()?;
    Ok(())
}

/// Checks if a directory is a valid template (contains .template.toml)
fn is_valid_template<P: AsRef<Path>>(dir_path: P) -> bool {
    dir_path.as_ref().join(TEMPLATE_CONFIG_FILE).exists()
}

/// Serializes a TemplateConfig to a TOML file
pub fn save_template_config<P: AsRef<Path>>(
    config: &TemplateConfig, 
    file_path: P,
    ensure_dir: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = file_path.as_ref();
    
    // Create parent directories if requested
    if ensure_dir {
        if let Some(parent) = file_path.parent() {
            create_dir_if_missing(parent)?;
        }
    }
    
    let toml_content = toml::to_string_pretty(config)?;
    fs::write(file_path, toml_content)?;
    
    Ok(())
}

/// Deserializes a .template.toml file for template metadata
fn parse_template_config<P: AsRef<Path>>(config_path: P) -> Result<TemplateConfig, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(config_path)?;
    let config: TemplateConfig = toml::from_str(&content)?;
    Ok(config)
}

/// Discovers all available templates in the template storage directory
pub fn discover_templates() -> Result<Vec<Template>, Box<dyn std::error::Error>> {
    let template_dir = ensure_template_storage_dir()?;
    let mut templates = Vec::new();
    
    // Read all entries in the template directory
    for entry in fs::read_dir(&template_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        // Skip if not a directory
        if !path.is_dir() {
            continue;
        }
        
        // Get the directory name as template name
        let name = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name.to_string(),
            None => continue,
        };
        
        // Check if it's a valid template
        if !is_valid_template(&path) {
            continue;
        }
        
        // Try to parse template config
        let config_path = path.join(TEMPLATE_CONFIG_FILE);
        let config = match parse_template_config(&config_path) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Warning: Failed to parse {}: {e}", config_path.display());
                continue;
            }
        };
        
        templates.push(Template {
            name,
            path,
            config,
        });
    }
    
    // Sort templates by display name
    templates.sort_by(|a, b| a.display_name().cmp(b.display_name()));
    
    Ok(templates)
}

/// Finds a specific template by name (matches both directory name and config name)
pub fn find_template(template_name: &str) -> Result<Option<Template>, Box<dyn std::error::Error>> {
    let templates = discover_templates()?;
    Ok(templates.into_iter().find(|t| {
        t.name == template_name || 
        t.config.name.as_deref() == Some(template_name)
    }))
}

/// Creates a sample .template.toml file in the specified directory
pub fn create_sample_template_config<P: AsRef<Path>>(dir_path: P, template_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = dir_path.as_ref().join(TEMPLATE_CONFIG_FILE);
    
    let sample_config = TemplateConfig {
        name: Some(template_name.to_string()),
        description: Some(format!("A template for {template_name}")),
        author: Some("Your Name".to_string()),
        version: Some("1.0.0".to_string()),
        tags: Some(vec!["project".to_string(), "template".to_string()]),
        min_tool_version: Some("0.1.0".to_string()),
        metadata: None,
    };
    
    let toml_content = toml::to_string_pretty(&sample_config)?;
    fs::write(config_path, toml_content)?;
    
    Ok(())
}