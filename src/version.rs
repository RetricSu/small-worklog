pub fn read_version_from_toml() -> Option<String> {
    // Read the contents of Cargo.toml
    let cargo_toml_content = match std::fs::read_to_string("Cargo.toml") {
        Ok(content) => content,
        Err(_) => return None, // Unable to read Cargo.toml
    };

    // Parse Cargo.toml content to extract version information
    let toml: toml::Value = match cargo_toml_content.parse() {
        Ok(t) => t,
        Err(_) => return None, // Unable to parse Cargo.toml
    };

    // Extract version information
    let version = match toml["package"]["version"].as_str() {
        Some(v) => v.to_string(),
        None => return None, // Version information not found in Cargo.toml
    };

    Some(version)
}
