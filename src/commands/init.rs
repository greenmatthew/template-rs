use crate::path::resolve_path;
use crate::file::ensure_template_storage_dir;
use std::env;
use std::process::Command;

pub fn handle_init(
    template: String,
    path: Option<String>,
    dry_run: bool,
    force: bool,
    delete: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing...");
    println!("Using template: {template}");
    
    // Resolve the target path - use current directory if none provided
    let target_path = match path {
        Some(p) => resolve_path(&p, None)?,
        None => env::current_dir()?,
    };
    
    // Check if the target path exists
    if !target_path.exists() {
        return Err(format!("Target path does not exist: {}", target_path.display()).into());
    }
    
    println!("Target path: {}", target_path.display());
    
    // Ensure template storage exists and get template path
    let template_dir = ensure_template_storage_dir()?;
    let source_template = template_dir.join(&template);
    
    if !source_template.exists() {
        return Err(format!("Template '{template}' not found in {}", template_dir.display()).into());
    }
    
    // Build rsync command
    let mut cmd = Command::new("rsync");
    cmd.arg("-av"); // archive + verbose
    
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
