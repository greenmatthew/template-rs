use crate::path::{PERSISTENT_STORAGE, TEMPLATE_STORAGE, resolve_path};
use std::fs;
use std::path::{Path, PathBuf};

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
    let storage_dir = resolve_path(PERSISTENT_STORAGE, None)?; // Use the const, not a string
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