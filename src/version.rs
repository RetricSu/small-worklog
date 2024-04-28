pub const VERSION: &str = "0.1.1";

pub fn read_version_from_toml() -> String {
    #[cfg(feature = "check_version")]
    {
        let cargo_toml_content = match std::fs::read_to_string("Cargo.toml") {
            Ok(content) => content,
            Err(_) => panic!("Unable to read Cargo.toml"), // Unable to read Cargo.toml
        };

        let toml: toml::Value = match cargo_toml_content.parse() {
            Ok(t) => t,
            Err(_) => panic!("Unable to parse Cargo.toml"), // Unable to parse Cargo.toml
        };

        let version_from_toml = match toml["package"]["version"].as_str() {
            Some(v) => v.to_string(),
            None => panic!("Unable to get version toml section as string"), // Version information not found in Cargo.toml
        };

        // Compare with the constant VERSION
        if version_from_toml.eq(VERSION) {
            return version_from_toml;
        } else {
            panic!(
                "Version from Cargo.toml ({}) does not match the constant VERSION ({})",
                version_from_toml, VERSION
            );
        }
    }

    #[cfg(not(feature = "check_version"))]
    {
        VERSION.to_string()
    }
}
