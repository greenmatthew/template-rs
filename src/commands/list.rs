use crate::template::Template;
use crate::languages::{get_display_name, is_known_language};
use std::collections::BTreeMap;

pub fn handle_list(verbose: bool, language: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    print_header();
    
    let mut templates = Template::discover_all()?;
    filter_by_language(&mut templates, language);
    
    if templates.is_empty() {
        print_no_templates_message(language);
        return Ok(());
    }
    
    let has_unrecognized = check_for_unrecognized(&templates);
    
    if verbose {
        display_verbose(&templates);
    } else {
        display_simple(&templates);
    }
    
    if has_unrecognized {
        println!("* Unrecognized language (not in standard list)\n");
    }
    
    Ok(())
}

fn print_header() {
    println!("Templates are located in {}", crate::path::TEMPLATE_STORAGE);
    println!("They require a .template.toml file in their root dir");
    println!("Use `{} author --help` to learn how to create a template\n", env!("CARGO_BIN_NAME"));
}

fn filter_by_language(templates: &mut Vec<Template>, language: Option<&str>) {
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
            // Normal language filtering - check if input matches any alias for the template's language
            templates.retain(|t| {
                t.language().is_some_and(|template_lang| {
                    // Get display name of template's language
                    let template_display = get_display_name(template_lang);
                    // Get display name of user's input
                    let input_display = get_display_name(lang);
                    // Match if display names are the same (both resolve to same canonical name)
                    template_display.eq_ignore_ascii_case(&input_display)
                })
            });
        }
    }
}

fn print_no_templates_message(language: Option<&str>) {
    if let Some(lang) = language {
        println!("No templates found for language filter '{lang}'.");
        println!("Use `{} list` to see all available templates.", env!("CARGO_BIN_NAME"));
    } else {
        println!("No templates found.");
        println!("Templates should be directories in ~/.template-rs/templates/ with a .template.toml file.");
    }
}

fn check_for_unrecognized(templates: &[Template]) -> bool {
    templates.iter()
        .filter_map(|t| t.language())
        .any(|lang| !is_known_language(lang))
}

fn group_by_language(templates: &[Template]) -> BTreeMap<String, Vec<&Template>> {
    let mut by_language: BTreeMap<String, Vec<&Template>> = BTreeMap::new();
    
    for template in templates {
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
    
    by_language
}

fn display_verbose(templates: &[Template]) {
    let by_language = group_by_language(templates);
    
    println!("Available templates:\n");
    
    for (lang, templates_in_lang) in by_language {
        // Strip the sorting prefix for display
        let display_lang = lang.trim_start_matches('\u{FFFF}');
        println!("  {display_lang}:");
        for template in templates_in_lang {
            println!("    Path: {}", template.name); // Show the actual path/identifier
            if let Some(name) = &template.config.name {
                println!("    Name: {name}");
            }
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
}

fn display_simple(templates: &[Template]) {
    let by_language = group_by_language(templates);
    
    println!("Available templates:\n");
    
    for (lang, templates_in_lang) in by_language {
        // Strip the sorting prefix for display
        let display_lang = lang.trim_start_matches('\u{FFFF}');
        println!("  {display_lang}:");
        for template in templates_in_lang {
            println!("    Path: {}", template.name); // Show the actual path/identifier
            if let Some(name) = &template.config.name {
                println!("    Name: {name}");
            }
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
