use crate::template::Template;
use crate::languages::{get_display_name, is_known_language};
use std::collections::BTreeMap;

pub fn handle_list(verbose: bool, language: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    // Print header
    println!("Templates are located in {}", crate::path::TEMPLATE_STORAGE);
    println!("They require a .template.toml file in their root dir");
    println!("Use `{} author --help` to learn how to create a template\n", env!("CARGO_BIN_NAME"));

    let mut templates = Template::discover_all()?;
    
    // Filter by language if specified
    if let Some(lang) = language {
        let lang_lower = lang.to_lowercase();
        
        // Handle special cases
        if lang_lower == "unknown" {
            templates.retain(|t| t.language().is_none());
        } else if lang_lower == "unrecognized" {
            templates.retain(|t| {
                t.language()
                    .is_some_and(|l| !is_known_language(l))
            });
        } else {
            // Normal language filtering
            templates.retain(|t| {
                t.language()
                    .is_some_and(|l| l.eq_ignore_ascii_case(lang))
            });
        }
        
        if templates.is_empty() {
            println!("No templates found for language filter '{lang}'.");
            println!("Use `{} list` to see all available templates.", env!("CARGO_BIN_NAME"));
            return Ok(());
        }
    }
    
    if templates.is_empty() {
        println!("No templates found.");
        println!("Templates should be directories in ~/.template-rs/templates/ with a .template.toml file.");
        return Ok(());
    }
    
    // Track if any unrecognized languages were found
    let has_unrecognized = templates.iter()
        .filter_map(|t| t.language())
        .any(|lang| !is_known_language(lang));
    
    if verbose {
        // Group by language for organized display
        let mut by_language: BTreeMap<String, Vec<&Template>> = BTreeMap::new();
        
        for template in &templates {
            let lang_key = match template.language() {
                Some(lang) => {
                    let display = get_display_name(lang);
                    if is_known_language(lang) {
                        display
                    } else {
                        format!("{display}*")
                    }
                }
                None => "\u{FFFF}Unknown".to_string(), // Unicode max char to sort last
            };
            by_language.entry(lang_key).or_default().push(template);
        }
        
        println!("Available templates:\n");
        
        for (lang, templates_in_lang) in by_language {
            // Strip the sorting prefix for display
            let display_lang = lang.trim_start_matches('\u{FFFF}');
            println!("  {display_lang}:");
            for template in templates_in_lang {
                println!("    Name: {}", template.display_name());
                if let Some(language) = template.language() {
                    println!("    Language: {language}");
                }
                if let Some(description) = template.description() {
                    println!("    Description: {description}");
                }
                if let Some(author) = template.author() {
                    println!("    Author: {author}");
                }
                if let Some(version) = template.version() {
                    println!("    Version: {version}");
                }
                if let Some(tags) = template.tags() {
                    println!("    Tags: {}", tags.join(", "));
                }
                println!();
            }
        }
    } else {
        // Simple grouped list
        let mut by_language: BTreeMap<String, Vec<&Template>> = BTreeMap::new();
        
        for template in &templates {
            let lang_key = match template.language() {
                Some(lang) => {
                    let display = get_display_name(lang);
                    if is_known_language(lang) {
                        display
                    } else {
                        format!("{display}*")
                    }
                }
                None => "\u{FFFF}Unknown".to_string(), // Unicode max char to sort last
            };
            by_language.entry(lang_key).or_default().push(template);
        }
        
        println!("Available templates:\n");
        
        for (lang, templates_in_lang) in by_language {
            // Strip the sorting prefix for display
            let display_lang = lang.trim_start_matches('\u{FFFF}');
            println!("  {display_lang}:");
            for template in templates_in_lang {
                println!("    Name: {}", template.display_name());
                if let Some(language) = template.language() {
                    println!("    Language: {language}");
                }
                if let Some(description) = template.description() {
                    println!("    Description: {description}");
                }
                println!();
            }
        }
    }
    
    if has_unrecognized {
        println!("* Unrecognized language (not in standard list)\n");
    }
    
    println!("Use `{} init <template-name>` to initialize a template.", env!("CARGO_BIN_NAME"));
    
    Ok(())
}
