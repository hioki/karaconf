pub mod karabiner_data;
pub mod rule_sets;

use std::{io::Seek as _, path::Path};

const CUSTOM_JSON_FILENAME: &str = "custom.json";

fn main() -> anyhow::Result<()> {
    let rules = collect_all_rules();
    let complex_modifications = karabiner_data::ComplexModifications {
        title: "Personal rules",
        rules: &rules,
    };

    let config_dir = get_karabiner_config_dir()?;
    ensure_karabiner_directories(&config_dir)?;

    write_custom_json(&complex_modifications)?;
    copy_to_karabiner_assets(&config_dir)?;
    update_karabiner_config(&config_dir, &rules)?;

    println!("✅ Karabiner configuration updated successfully!");
    Ok(())
}

/// Collect all manipulators from rule sets
fn collect_all_rules() -> Vec<karabiner_data::Rule> {
    vec![karabiner_data::Rule {
        description: "Personal rules".to_string(),
        manipulators: vec![
            rule_sets::virtual_key_assignments::manipulators(),
            rule_sets::iterm2::manipulators(),
            rule_sets::vscode::manipulators(),
            rule_sets::dynalist::manipulators(),
            rule_sets::slack::manipulators(),
            rule_sets::google_chrome::manipulators(),
            rule_sets::notion::manipulators(),
            rule_sets::chatgpt::manipulators(),
            rule_sets::common::manipulators(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<karabiner_data::Manipulator>>(),
    }]
}

/// Get the Karabiner-Elements configuration directory
fn get_karabiner_config_dir() -> anyhow::Result<std::path::PathBuf> {
    let home = std::env::var("HOME").map_err(|_| {
        anyhow::anyhow!(
            "HOME environment variable is not set. Please set it to your home directory."
        )
    })?;
    Ok(std::path::PathBuf::from(home).join(".config/karabiner"))
}

/// Ensure required Karabiner directories exist
fn ensure_karabiner_directories(config_dir: &Path) -> anyhow::Result<()> {
    if !config_dir.is_dir() {
        anyhow::bail!(
            "Karabiner-Elements configuration directory {:?} does not exist.\n\
            Please install and run Karabiner-Elements first to create the directory structure.",
            config_dir
        );
    }

    let assets_dir = config_dir.join("assets/complex_modifications");
    if !assets_dir.is_dir() {
        anyhow::bail!(
            "Karabiner-Elements assets directory {:?} does not exist.\n\
            Please run Karabiner-Elements at least once to create the directory structure.",
            assets_dir
        );
    }

    Ok(())
}

/// Write custom.json to the project root
fn write_custom_json(
    complex_modifications: &karabiner_data::ComplexModifications,
) -> anyhow::Result<()> {
    let custom_json_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .read(true)
        .open(CUSTOM_JSON_FILENAME)
        .map_err(|e| anyhow::anyhow!("Failed to create {}: {}", CUSTOM_JSON_FILENAME, e))?;

    serde_json::to_writer_pretty(&custom_json_file, &complex_modifications)
        .map_err(|e| anyhow::anyhow!("Failed to write JSON to {}: {}", CUSTOM_JSON_FILENAME, e))?;

    Ok(())
}

/// Copy custom.json to Karabiner assets directory
fn copy_to_karabiner_assets(config_dir: &Path) -> anyhow::Result<()> {
    let assets_path = config_dir
        .join("assets/complex_modifications")
        .join(CUSTOM_JSON_FILENAME);

    let mut custom_json_file = std::fs::File::open(CUSTOM_JSON_FILENAME)
        .map_err(|e| anyhow::anyhow!("Failed to open {}: {}", CUSTOM_JSON_FILENAME, e))?;

    let mut karabiner_assets_file = std::fs::File::create(&assets_path)
        .map_err(|e| anyhow::anyhow!("Failed to create assets file {:?}: {}", assets_path, e))?;

    custom_json_file.seek(std::io::SeekFrom::Start(0))?;
    std::io::copy(&mut custom_json_file, &mut karabiner_assets_file)
        .map_err(|e| anyhow::anyhow!("Failed to copy to assets directory: {}", e))?;

    Ok(())
}

/// Update karabiner.json with type-safe operations
fn update_karabiner_config(
    config_dir: &Path,
    rules: &[karabiner_data::Rule],
) -> anyhow::Result<()> {
    let karabiner_json_path = config_dir.join("karabiner.json");

    // Read and parse the existing karabiner.json
    let mut karabiner_config: karabiner_data::KarabinerConfig =
        serde_json::from_reader(&std::fs::File::open(&karabiner_json_path)?).unwrap_or_else(|_| {
            eprintln!(
                "⚠️  Warning: Unable to parse existing karabiner.json, using default structure"
            );
            karabiner_data::KarabinerConfig::default()
        });

    // Update the first profile's complex modifications
    if let Some(profile) = karabiner_config.profiles.get_mut(0) {
        profile.complex_modifications.rules = rules.to_vec();
    } else {
        anyhow::bail!("No profile found in karabiner.json");
    }

    // Write back the updated configuration
    let karabiner_json_data = serde_json::to_vec_pretty(&karabiner_config)
        .map_err(|e| anyhow::anyhow!("Failed to serialize karabiner.json: {}", e))?;

    std::fs::write(&karabiner_json_path, karabiner_json_data)
        .map_err(|e| anyhow::anyhow!("Failed to write karabiner.json: {}", e))?;

    Ok(())
}
