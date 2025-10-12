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

    /// Discovers all available templates in the template storage directory (recursively)
    pub fn discover_all() -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        let template_dir = ensure_template_storage_dir()?;
        let mut templates = Vec::new();
        
        // Helper function to recursively search for templates
        fn search_templates(
            base_dir: &Path,
            current_dir: &Path,
            templates: &mut Vec<Template>,
        ) -> Result<(), Box<dyn std::error::Error>> {
            for entry in fs::read_dir(current_dir)? {
                let entry = entry?;
                let path = entry.path();
                
                // Skip if not a directory
                if !path.is_dir() {
                    continue;
                }
                
                // Check if this directory is a valid template
                if Template::is_valid_template(&path) {
                    // Calculate the relative path from base_dir as the template name
                    let name = path.strip_prefix(base_dir)
                        .ok()
                        .and_then(|p| p.to_str())
                        .map(|s| s.replace('\\', "/")) // Normalize path separators
                        .unwrap_or_else(|| {
                            path.file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("unknown")
                                .to_string()
                        });
                    
                    // Try to parse template config
                    let config_path = path.join(TEMPLATE_CONFIG_FILE);
                    let config = match Template::parse_config(&config_path) {
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
                } else {
                    // If not a template, recursively search its subdirectories
                    search_templates(base_dir, &path, templates)?;
                }
            }
            Ok(())
        }
        
        search_templates(&template_dir, &template_dir, &mut templates)?;
        
        // Sort templates by name (which is now the path)
        templates.sort_by(|a, b| a.name.cmp(&b.name));
        
        Ok(templates)
    }

    /// Finds a specific template by name (matches both path and config name)
    pub fn find(template_name: &str) -> Result<Option<Self>, Box<dyn std::error::Error>> {
        let templates = Self::discover_all()?;
        
        // Normalize the search name (convert backslashes to forward slashes)
        let normalized_search = template_name.replace('\\', "/");
        
        Ok(templates.into_iter().find(|t| {
            // Match against the path (template.name)
            t.name == normalized_search || 
            // Also match against the config name if it exists
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
        if ensure_dir && let Some(parent) = file_path.parent() {
            create_dir_if_missing(parent)?;
        }
        
        let toml_content = toml::to_string_pretty(config)?;
        fs::write(file_path, toml_content)?;
        
        Ok(())
    }
}
