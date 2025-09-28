use shellexpand;
use std::env;
use std::path::{Path, PathBuf};

pub const PERSISTENT_STORAGE: &str = "~/.template-rs";
pub const TEMPLATE_STORAGE: &str = "~/.template-rs/templates";

/// Resolves a user-provided path string into an absolute PathBuf.
/// Handles relative paths, absolute paths, home directory expansion (~),
/// environment variable expansion ($VAR, ${VAR}), and current directory 
/// references (.) across Linux and Windows.
///
/// # Arguments
/// * `path_str` - The path string provided by the user
/// * `invocation_dir` - Optional directory context (if None, uses current working directory)
///
/// # Examples
/// ```
/// // Absolute path
/// let resolved = resolve_path("/home/user/file.txt", None);
/// 
/// // Home directory expansion
/// let resolved = resolve_path("~/Documents/file.txt", None);
/// 
/// // Environment variable expansion
/// let resolved = resolve_path("$HOME/config.toml", None);
/// let resolved = resolve_path("${XDG_CONFIG_HOME}/app/config.toml", None);
/// 
/// // Combined expansions
/// let resolved = resolve_path("~/projects/${PROJECT_NAME}/src", None);
/// 
/// // Relative path
/// let resolved = resolve_path("../config.toml", None);
/// 
/// // Current directory
/// let resolved = resolve_path(".", None);
/// ```
pub fn resolve_path(path_str: &str, invocation_dir: Option<&Path>) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let path_str = path_str.trim();
    
    // Handle current directory reference first (before expansion)
    if path_str == "." {
        return Ok(invocation_dir
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| PathBuf::from("."))));
    }
    
    // Perform shell-like expansion (tilde + environment variables)
    let expanded = shellexpand::full(path_str)?;
    let expanded_path = Path::new(expanded.as_ref());
    
    // If already absolute after expansion, return as-is
    if expanded_path.is_absolute() {
        return Ok(expanded_path.to_path_buf());
    }
    
    // Handle relative paths after expansion
    let base_dir = invocation_dir
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    
    // Handle ./path cases after expansion
    let final_path = if expanded.starts_with("./") {
        base_dir.join(&expanded[2..])
    } else if expanded.starts_with(".\\") {
        base_dir.join(&expanded[2..])
    } else {
        base_dir.join(expanded.as_ref())
    };
    
    Ok(final_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::env;
    
    #[test]
    fn test_current_directory() {
        let result = resolve_path(".", None).unwrap();
        assert!(result.is_absolute() || result == PathBuf::from("."));
    }
    
    #[test]
    fn test_absolute_path() {
        #[cfg(unix)]
        {
            let result = resolve_path("/home/user/file.txt", None).unwrap();
            assert_eq!(result, PathBuf::from("/home/user/file.txt"));
        }
        
        #[cfg(windows)]
        {
            let result = resolve_path("C:\\Users\\user\\file.txt", None).unwrap();
            assert_eq!(result, PathBuf::from("C:\\Users\\user\\file.txt"));
        }
    }
    
    #[test]
    fn test_home_expansion() {
        let result = resolve_path("~", None);
        assert!(result.is_ok());
        
        let result = resolve_path("~/Documents", None);
        assert!(result.is_ok());
        if let Ok(path) = result {
            assert!(path.to_string_lossy().contains("Documents"));
        }
    }
    
    #[test]
    fn test_env_var_expansion() {
        // Set a test environment variable
        env::set_var("TEST_PATH", "/test/directory");
        
        let result = resolve_path("$TEST_PATH/file.txt", None).unwrap();
        assert!(result.to_string_lossy().contains("/test/directory/file.txt"));
        
        // Test with braces
        let result = resolve_path("${TEST_PATH}/config.toml", None).unwrap();
        assert!(result.to_string_lossy().contains("/test/directory/config.toml"));
        
        // Clean up
        env::remove_var("TEST_PATH");
    }
    
    #[test]
    fn test_combined_expansion() {
        env::set_var("PROJECT_NAME", "my-project");
        
        // This would expand to something like ~/projects/my-project/src
        let result = resolve_path("~/projects/${PROJECT_NAME}/src", None);
        assert!(result.is_ok());
        if let Ok(path) = result {
            let path_str = path.to_string_lossy();
            assert!(path_str.contains("projects"));
            assert!(path_str.contains("my-project"));
            assert!(path_str.contains("src"));
        }
        
        env::remove_var("PROJECT_NAME");
    }
    
    #[test]
    fn test_relative_path() {
        let base = Path::new("/some/base/dir");
        let result = resolve_path("../config.toml", Some(base)).unwrap();
        assert_eq!(result, PathBuf::from("/some/base/dir/../config.toml"));
    }
    
    #[test]
    fn test_dot_slash_prefix() {
        let base = Path::new("/some/base/dir");
        let result = resolve_path("./file.txt", Some(base)).unwrap();
        assert_eq!(result, PathBuf::from("/some/base/dir/file.txt"));
    }
}
