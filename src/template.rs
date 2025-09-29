use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Template configuration from .template.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    /// Template name (optional, defaults to directory name)
    pub name: Option<String>,
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
}
