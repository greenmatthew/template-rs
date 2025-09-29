use crate::path::resolve_path;
use crate::file::{ensure_template_storage_dir};
use crate::template::{Template, TEMPLATE_CONFIG_FILE};

use std::env;
use std::process::Command;

#[allow(clippy::fn_params_excessive_bools)]
pub fn handle_init(
    template: &str,
    path: Option<String>,
    dry_run: bool,
    force: bool,
    delete: bool,
    create_dir: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing...");
    println!("Using template: {template}");
    
    // Resolve the target path - use current directory if none provided
    let target_path = match path {
        Some(p) => resolve_path(&p, None)?,
        None => env::current_dir()?,
    };
    
    // Create directory if requested (for 'new' command)
    if create_dir {
        if !target_path.exists() {
            println!("Creating directory: {}", target_path.display());
            if !dry_run {
                std::fs::create_dir_all(&target_path)?;
            }
        } else if !target_path.is_dir() {
            return Err(format!("Path exists but is not a directory: {}", target_path.display()).into());
        }
    } else {
        // Check if the target path exists (for 'init' command)
        if !target_path.exists() {
            return Err(format!("Target path does not exist: {}", target_path.display()).into());
        }
    }
    
    println!("Target path: {}", target_path.display());
    
    // Ensure template storage exists
    let template_dir = ensure_template_storage_dir()?;

    // Find the template
    let template_info = Template::find(template)?
        .ok_or_else(|| format!("Template '{template}' not found. Use 'template-rs list' to see available templates."))?;
    
    println!("Found template: {}", template_info.path.display());
    if let Some(description) = template_info.description() {
        println!("Description: {description}");
    }
    
    // Rest of the function remains the same, but use template_info.path instead of source_template
    let source_template = &template_info.path;
    
    if !source_template.exists() {
        return Err(format!("Template '{template}' not found in {}", template_dir.display()).into());
    }
    
    // Build rsync command
    let mut cmd = Command::new("rsync");
    // -r recursive, -l copy symlinks, -p preserve permissions, -v verbose
    // Omit -t to NOT preserve timestamps (files get current time)
    cmd.arg("-rlpv");

    // Exclude the template configuration file
    cmd.arg(format!("--exclude={TEMPLATE_CONFIG_FILE}"));
    
    if dry_run {
        cmd.arg("--dry-run");
        cmd.arg("--itemize-changes");
    }
    
    if !force {
        cmd.arg("--ignore-existing");
    }
    
    if delete {
        cmd.arg("--delete");
    }
    
    // Add trailing slash to source for proper rsync behavior
    let source_str = format!("{}/", source_template.display());
    cmd.arg(&source_str);
    cmd.arg(&target_path);
    
    // Show user what's happening
    if dry_run {
        println!("🔍 Dry run - showing what would be copied:");
    } else if force && delete {
        println!("⚠️  Force + delete mode - destination will match template exactly");
    } else if force {
        println!("⚠️  Force mode - overwriting existing files");
    } else if delete {
        println!("⚠️  Delete mode - removing files not in template");
    } else {
        println!("📁 Copying template files (skipping existing files)");
    }
    
    // Execute rsync
    let output = cmd.output()?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("rsync failed: {stderr}").into());
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    if !stdout.trim().is_empty() {
        println!("{stdout}");
    }
    
    if !dry_run {
        println!("✅ Template initialization complete!");
    }
    
    Ok(())
}
