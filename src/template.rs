use crate::file::{ensure_template_storage_dir, create_dir_if_missing};

use serde::{Deserialize, Serialize};

use std::path::PathBuf;
use std::fs;
use std::path::Path;

pub const TEMPLATE_CONFIG_FILE: &str = ".template.toml";

/// Template configuration from .template.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    /// Template name (optional, defaults to directory name)
    pub name: Option<String>,
    /// Template main programming language
    pub language: Option<String>,
    /// Template description
    pub description: Option<String>,
    /// Template author
    pub author: Option<String>,
    /// Template version
    pub version: Option<String>,
    /// Template tags for categorization
    pub tags: Option<Vec<String>>,
    /// Minimum required version of this tool
    pub min_tool_version: Option<String>,
    /// Additional metadata
    #[serde(flatten)]
    pub metadata: Option<toml::Table>,
}

/// Represents a discovered template
#[derive(Debug, Clone)]
pub struct Template {
    pub name: String,
    pub path: PathBuf,
    pub config: TemplateConfig,
}

impl Template {
    /// Get the display name (config name or directory name)
    pub fn display_name(&self) -> &str {
        self.config.name.as_deref().unwrap_or(&self.name)
    }

    /// Get the main programming language if available
    pub fn language(&self) -> Option<&str> {
        self.config.language.as_deref()
    }

    /// Get description if available
    pub fn description(&self) -> Option<&str> {
        self.config.description.as_deref()
    }
    
    /// Get author if available
    pub fn author(&self) -> Option<&str> {
        self.config.author.as_deref()
    }
    
    /// Get version if available
    pub fn version(&self) -> Option<&str> {
        self.config.version.as_deref()
    }
    
    /// Get tags if available
    pub fn tags(&self) -> Option<&[String]> {
        self.config.tags.as_deref()
    }

    /// Checks if a directory is a valid template (contains .template.toml)
    pub fn is_valid_template<P: AsRef<Path>>(dir_path: P) -> bool {
        dir_path.as_ref().join(TEMPLATE_CONFIG_FILE).exists()
    }

    /// Deserializes a .template.toml file for template metadata
    fn parse_config<P: AsRef<Path>>(config_path: P) -> Result<TemplateConfig, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(config_path)?;
        let config: TemplateConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// Discovers all available templates in the template storage directory
    pub fn discover_all() -> Result<Vec<Self>, Box<dyn std::error::Error>> {
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
            if !Self::is_valid_template(&path) {
                continue;
            }
            
            // Try to parse template config
            let config_path = path.join(TEMPLATE_CONFIG_FILE);
            let config = match Self::parse_config(&config_path) {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Warning: Failed to parse {}: {e}", config_path.display());
                    continue;
                }
            };
            
            templates.push(Self {
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
    pub fn find(template_name: &str) -> Result<Option<Self>, Box<dyn std::error::Error>> {
        let templates = Self::discover_all()?;
        Ok(templates.into_iter().find(|t| {
            t.name == template_name || 
            t.config.name.as_deref() == Some(template_name)
        }))
    }

    /// Serializes a `TemplateConfig` to a TOML file
    pub fn save_config<P: AsRef<Path>>(
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
}
