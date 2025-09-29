use std::collections::HashMap;

/// Language definition with display name and aliases
pub struct Language {
    pub display_name: &'static str,
    pub aliases: &'static [&'static str],
}

impl Language {
    const fn new(display_name: &'static str, aliases: &'static [&'static str]) -> Self {
        Self { display_name, aliases }
    }
}

/// Map of all known languages with their aliases
/// Based on GitHub Linguist languages.yml
pub fn get_language_map() -> HashMap<&'static str, &'static Language> {
    // Define all languages
    static LANGUAGES: &[Language] = &[
        Language::new("Bash", &["bash", "sh", "shell", "zsh"]),
        Language::new("C", &["c"]),
        Language::new("C#", &["csharp", "c#", "cake", "cakescript"]),
        Language::new("C++", &["cpp", "c++"]),
        Language::new("Clojure", &["clojure", "clj"]),
        Language::new("CMake", &["cmake"]),
        Language::new("CoffeeScript", &["coffeescript", "coffee", "coffee-script"]),
        Language::new("CSS", &["css"]),
        Language::new("Dart", &["dart"]),
        Language::new("Dockerfile", &["dockerfile", "containerfile"]),
        Language::new("Elixir", &["elixir"]),
        Language::new("Erlang", &["erlang"]),
        Language::new("F#", &["fsharp", "f#"]),
        Language::new("Go", &["go", "golang"]),
        Language::new("Groovy", &["groovy"]),
        Language::new("Haskell", &["haskell", "hs"]),
        Language::new("HTML", &["html", "xhtml"]),
        Language::new("Java", &["java"]),
        Language::new("JavaScript", &["javascript", "js", "node"]),
        Language::new("JSON", &["json", "geojson", "jsonl", "topojson"]),
        Language::new("Just", &["just", "justfile"]),
        Language::new("Kotlin", &["kotlin", "kt"]),
        Language::new("Lua", &["lua"]),
        Language::new("Makefile", &["makefile", "make", "mf", "bsdmake"]),
        Language::new("Markdown", &["markdown", "md", "pandoc"]),
        Language::new("Nix", &["nix", "nixos"]),
        Language::new("Objective-C", &["objective-c", "objc", "obj-c", "objectivec"]),
        Language::new("Objective-C++", &["objective-c++", "objc++", "obj-c++", "objectivec++"]),
        Language::new("OCaml", &["ocaml"]),
        Language::new("Perl", &["perl", "cperl"]),
        Language::new("PHP", &["php"]),
        Language::new("PowerShell", &["powershell", "posh", "pwsh"]),
        Language::new("Python", &["python", "py", "python3", "rusthon"]),
        Language::new("R", &["r", "rscript", "splus"]),
        Language::new("Ruby", &["ruby", "rb", "jruby", "macruby", "rake", "rbx"]),
        Language::new("Rust", &["rust", "rs"]),
        Language::new("Sass", &["sass"]),
        Language::new("Scala", &["scala"]),
        Language::new("SCSS", &["scss"]),
        Language::new("Shell", &["shell", "sh", "bash", "zsh"]),
        Language::new("SQL", &["sql"]),
        Language::new("Svelte", &["svelte"]),
        Language::new("Swift", &["swift"]),
        Language::new("TOML", &["toml"]),
        Language::new("TypeScript", &["typescript", "ts"]),
        Language::new("Vue", &["vue"]),
        Language::new("XML", &["xml"]),
        Language::new("YAML", &["yaml", "yml"]),
    ];

    // Build the map
    let mut map = HashMap::new();
    for lang in LANGUAGES {
        for &alias in lang.aliases {
            map.insert(alias, lang);
        }
    }
    map
}

/// Get the display name for a language identifier (case-insensitive)
/// Returns the canonical display name if recognized, otherwise returns the original identifier
pub fn get_display_name(identifier: &str) -> String {
    let map = get_language_map();
    let lower = identifier.to_lowercase();
    map.get(lower.as_str())
        .map(|lang| lang.display_name.to_string())
        .unwrap_or_else(|| identifier.to_string())
}

/// Check if a language identifier is known (case-insensitive)
pub fn is_known_language(identifier: &str) -> bool {
    let map = get_language_map();
    let lower = identifier.to_lowercase();
    map.contains_key(lower.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_aliases() {
        assert_eq!(get_display_name("js"), "JavaScript");
        assert_eq!(get_display_name("javascript"), "JavaScript");
        assert_eq!(get_display_name("node"), "JavaScript");
        
        assert_eq!(get_display_name("cpp"), "C++");
        assert_eq!(get_display_name("c++"), "C++");
        
        assert_eq!(get_display_name("py"), "Python");
        assert_eq!(get_display_name("python3"), "Python");
    }
    
    #[test]
    fn test_case_insensitive() {
        assert_eq!(get_display_name("JavaScript"), "JavaScript");
        assert_eq!(get_display_name("JAVASCRIPT"), "JavaScript");
        assert_eq!(get_display_name("jAvAsCrIpT"), "JavaScript");
    }
    
    #[test]
    fn test_unknown_language() {
        assert_eq!(get_display_name("unknown-lang"), "unknown-lang");
        assert!(!is_known_language("unknown-lang"));
    }
}
