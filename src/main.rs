pub mod cheatsheet;
pub mod display;
pub mod karabiner_data;
pub mod lint;
pub mod rule_sets;

use std::{io::Seek as _, path::Path};

const SIMULTANEOUS_THRESHOLD_MILLISECONDS: u64 = 59;

const TITLE: &str = "Personal rules";

const CUSTOM_JSON_FILENAME: &str = "custom.json";

const CHEATSHEET_FILENAME: &str = "cheatsheet.html";

const KARABINER_JSON_FILENAME: &str = "karabiner.json";

const KARABINER_JSON_BACKUP_FILENAME: &str = "karabiner.json.karaconf-bak";

type RuleSet = (&'static str, fn() -> Vec<karabiner_data::Manipulator>);

/// Rule sets keyed on the frontmost app. Screen Sharing gets copies of their
/// rules for the remote Mac's frontmost app (see `rulesets`).
const APP_RULE_SETS: &[RuleSet] = &[
    ("apps/iterm2", rule_sets::apps::iterm2::manipulators),
    ("apps/vscode", rule_sets::apps::vscode::manipulators),
    ("apps/dynalist", rule_sets::apps::dynalist::manipulators),
    ("apps/slack", rule_sets::apps::slack::manipulators),
    (
        "apps/google_chrome",
        rule_sets::apps::google_chrome::manipulators,
    ),
    ("apps/firefox", rule_sets::apps::firefox::manipulators),
    ("apps/notion", rule_sets::apps::notion::manipulators),
    ("apps/bitwarden", rule_sets::apps::bitwarden::manipulators),
    // Claude,
    ("apps/claude", rule_sets::apps::claude::manipulators),
    (
        "apps/onepassword",
        rule_sets::apps::onepassword::manipulators,
    ),
    ("apps/finder", rule_sets::apps::finder::manipulators),
    (
        "apps/notion_calendar",
        rule_sets::apps::notion_calendar::manipulators,
    ),
    ("apps/calendar", rule_sets::apps::calendar::manipulators),
    ("apps/preview", rule_sets::apps::preview::manipulators),
    ("apps/mail", rule_sets::apps::mail::manipulators),
    ("apps/codex", rule_sets::apps::codex::manipulators),
    (
        "apps/microsoft_to_do",
        rule_sets::apps::microsoft_to_do::manipulators,
    ),
    (
        "apps/github_copilot",
        rule_sets::apps::github_copilot::manipulators,
    ),
];

/// All rule sets in evaluation order (the first matching manipulator wins).
fn rulesets() -> Vec<(&'static str, Vec<karabiner_data::Manipulator>)> {
    let apps: Vec<(&str, Vec<karabiner_data::Manipulator>)> =
        APP_RULE_SETS.iter().map(|(name, f)| (*name, f())).collect();
    // Right after the app rules, so that over Screen Sharing they win over the
    // other rules just as they do on the remote Mac itself.
    let remote_apps = rule_sets::apps::screen_sharing::remote_app_manipulators(
        apps.iter().flat_map(|(_, manipulators)| manipulators),
    );

    let mut rulesets = vec![(
        "virtual_key_assignments",
        rule_sets::virtual_key_assignments::manipulators(),
    )];
    rulesets.extend(apps);
    rulesets.extend([
        ("apps/screen_sharing/remote_apps", remote_apps),
        (
            "apps/screen_sharing",
            rule_sets::apps::screen_sharing::manipulators(),
        ),
        ("common", rule_sets::common::manipulators()),
        ("shingeta", rule_sets::shingeta::manipulators()),
    ]);
    rulesets
}

fn main() -> anyhow::Result<()> {
    let rulesets = rulesets();

    let findings = lint::lint(&rulesets);
    if !findings.is_empty() {
        eprintln!("⚠️  lint: {} 件の問題が見つかりました:", findings.len());
        for finding in &findings {
            eprintln!("  - {}", finding);
        }
    }
    if std::env::args().any(|arg| arg == "--check") {
        if findings.is_empty() {
            println!("✅ lint: 問題なし");
        }
        std::process::exit(if findings.is_empty() { 0 } else { 1 });
    }

    // One Rule per manipulator so each binding carries its own description.
    let rules: Vec<karabiner_data::Rule> = rulesets
        .iter()
        .flat_map(|(_, manipulators)| manipulators.iter())
        .map(|manipulator| karabiner_data::Rule {
            description: display::rule_description(manipulator),
            manipulators: vec![manipulator.clone()],
        })
        .collect();
    let complex_modifications = karabiner_data::ComplexModifications {
        title: TITLE,
        rules: &rules,
    };
    let config_dir = std::path::PathBuf::from(std::env::var("HOME")?).join(".config/karabiner");
    ensure_karabiner_directories(&config_dir)?;

    write_custom_json(&complex_modifications)?;
    copy_to_karabiner_assets(&config_dir)?;
    update_karabiner_config(&config_dir, &rules)?;

    std::fs::write(
        CHEATSHEET_FILENAME,
        cheatsheet::generate(&rulesets, &findings),
    )
    .map_err(|e| anyhow::anyhow!("Failed to write {}: {}", CHEATSHEET_FILENAME, e))?;

    println!("✅ Karabiner configuration updated successfully!");
    println!("📝 Cheatsheet written to {}", CHEATSHEET_FILENAME);
    Ok(())
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

/// Replace karaconf's rules in karabiner.json, keeping every other setting as it is.
///
/// The file is handled as an untyped `serde_json::Value`, so settings karaconf does not
/// model (devices, simple_modifications, fields added by newer Karabiner-Elements, ...)
/// and rules written by other karaconf versions never block the update or get dropped.
/// A file that cannot be parsed is left untouched instead of being replaced with a
/// default structure, which would silently wipe the user's settings. The previous
/// content is saved to `KARABINER_JSON_BACKUP_FILENAME` before the file is overwritten.
fn update_karabiner_config(
    config_dir: &Path,
    rules: &[karabiner_data::Rule],
) -> anyhow::Result<()> {
    let karabiner_json_path = config_dir.join(KARABINER_JSON_FILENAME);

    let original = std::fs::read_to_string(&karabiner_json_path)
        .map_err(|e| anyhow::anyhow!("Failed to read {:?}: {}", karabiner_json_path, e))?;
    let mut karabiner_config: serde_json::Value = serde_json::from_str(&original).map_err(|e| {
        anyhow::anyhow!(
            "Failed to parse {:?}: {}\n\
                The file was left untouched. Fix or restore it (e.g. from {} or \
                Karabiner-Elements' automatic_backups) and run again.",
            karabiner_json_path,
            e,
            KARABINER_JSON_BACKUP_FILENAME
        )
    })?;

    apply_rules(&mut karabiner_config, rules).map_err(|e| {
        anyhow::anyhow!(
            "Unexpected structure in {:?}: {}\nThe file was left untouched.",
            karabiner_json_path,
            e
        )
    })?;

    let updated = serde_json::to_string_pretty(&karabiner_config)
        .map_err(|e| anyhow::anyhow!("Failed to serialize karabiner.json: {}", e))?;
    // Skipping no-op writes also keeps the backup pointing at the last real change.
    if updated == original {
        return Ok(());
    }

    let backup_path = config_dir.join(KARABINER_JSON_BACKUP_FILENAME);
    std::fs::write(&backup_path, &original)
        .map_err(|e| anyhow::anyhow!("Failed to write backup {:?}: {}", backup_path, e))?;
    std::fs::write(&karabiner_json_path, updated)
        .map_err(|e| anyhow::anyhow!("Failed to write {:?}: {}", karabiner_json_path, e))?;

    Ok(())
}

/// Set the parts of karabiner.json that karaconf owns: the first profile's
/// complex_modifications rules (replaced wholesale) and simultaneous threshold.
///
/// Missing sections are created, since Karabiner-Elements omits sections that hold
/// only default values.
fn apply_rules(
    karabiner_config: &mut serde_json::Value,
    rules: &[karabiner_data::Rule],
) -> anyhow::Result<()> {
    let profile = karabiner_config
        .get_mut("profiles")
        .and_then(serde_json::Value::as_array_mut)
        .and_then(|profiles| profiles.first_mut())
        .and_then(serde_json::Value::as_object_mut)
        .ok_or_else(|| anyhow::anyhow!("No profile found"))?;

    let complex_modifications = profile
        .entry("complex_modifications")
        .or_insert_with(|| serde_json::json!({}))
        .as_object_mut()
        .ok_or_else(|| anyhow::anyhow!("profiles[0].complex_modifications is not an object"))?;
    complex_modifications.insert("rules".to_string(), serde_json::to_value(rules)?);

    // Update simultaneous threshold for the shingeta layout
    complex_modifications
        .entry("parameters")
        .or_insert_with(|| serde_json::json!({}))
        .as_object_mut()
        .ok_or_else(|| {
            anyhow::anyhow!("profiles[0].complex_modifications.parameters is not an object")
        })?
        .insert(
            "basic.simultaneous_threshold_milliseconds".to_string(),
            serde_json::json!(SIMULTANEOUS_THRESHOLD_MILLISECONDS),
        );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::karabiner_data::{KeyCode::*, Manipulator, Rule};
    use serde_json::json;

    fn rules() -> Vec<Rule> {
        vec![Rule {
            description: "test".to_string(),
            manipulators: vec![Manipulator::builder().from_key(A).to_key(B, None).build()],
        }]
    }

    fn scratch_dir(name: &str) -> std::path::PathBuf {
        let dir =
            std::env::temp_dir().join(format!("karaconf-test-{}-{}", std::process::id(), name));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn apply_rules_keeps_everything_but_rules_and_threshold() {
        // Rules from another karaconf version (unknown condition type) and fields karaconf
        // does not model must neither block the update nor be dropped.
        let mut config = json!({
            "global": { "ask_for_confirmation_before_quitting": false },
            "profiles": [
                {
                    "complex_modifications": {
                        "parameters": {
                            "basic.simultaneous_threshold_milliseconds": 50,
                            "basic.to_if_alone_timeout_milliseconds": 1000
                        },
                        "rules": [{
                            "description": "old",
                            "manipulators": [{ "conditions": [{ "type": "device_exists_if" }] }]
                        }]
                    },
                    "devices": [{ "identifiers": { "vendor_id": 1452 }, "ignore": true }],
                    "name": "Default profile",
                    "simple_modifications": [{
                        "from": { "key_code": "caps_lock" },
                        "to": [{ "key_code": "left_control" }]
                    }],
                    "unknown_future_field": 1
                },
                { "name": "Second profile" }
            ]
        });
        let mut expected = config.clone();
        let complex_modifications = &mut expected["profiles"][0]["complex_modifications"];
        complex_modifications["rules"] = serde_json::to_value(rules()).unwrap();
        complex_modifications["parameters"]["basic.simultaneous_threshold_milliseconds"] =
            json!(SIMULTANEOUS_THRESHOLD_MILLISECONDS);

        apply_rules(&mut config, &rules()).unwrap();

        // Compare the serialized text so that key order is checked too.
        assert_eq!(
            serde_json::to_string_pretty(&config).unwrap(),
            serde_json::to_string_pretty(&expected).unwrap()
        );
    }

    #[test]
    fn apply_rules_creates_sections_karabiner_omits() {
        let mut config = json!({ "profiles": [{ "name": "Default profile" }] });

        apply_rules(&mut config, &rules()).unwrap();

        assert_eq!(
            config,
            json!({ "profiles": [{
                "name": "Default profile",
                "complex_modifications": {
                    "rules": serde_json::to_value(rules()).unwrap(),
                    "parameters": {
                        "basic.simultaneous_threshold_milliseconds": SIMULTANEOUS_THRESHOLD_MILLISECONDS
                    }
                }
            }] })
        );
    }

    #[test]
    fn apply_rules_rejects_unexpected_structure() {
        for mut config in [
            json!({}),
            json!({ "profiles": [] }),
            json!({ "profiles": [{ "complex_modifications": [] }] }),
            json!({ "profiles": [{ "complex_modifications": { "parameters": [] } }] }),
        ] {
            assert!(apply_rules(&mut config, &rules()).is_err(), "{config}");
        }
    }

    #[test]
    fn unparsable_karabiner_json_is_left_untouched() {
        let dir = scratch_dir("unparsable");
        let path = dir.join(KARABINER_JSON_FILENAME);
        let broken = r#"{"profiles": [{"name": "#;
        std::fs::write(&path, broken).unwrap();

        let error = update_karabiner_config(&dir, &rules())
            .unwrap_err()
            .to_string();

        assert!(error.contains("Failed to parse"), "{error}");
        assert!(error.contains(&format!("{:?}", path)), "{error}");
        assert_eq!(std::fs::read_to_string(&path).unwrap(), broken);
        assert!(!dir.join(KARABINER_JSON_BACKUP_FILENAME).exists());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn previous_karabiner_json_is_backed_up_only_when_changed() {
        let dir = scratch_dir("backup");
        let path = dir.join(KARABINER_JSON_FILENAME);
        let backup_path = dir.join(KARABINER_JSON_BACKUP_FILENAME);
        let original = r#"{"profiles": [{"name": "Default profile"}]}"#;
        std::fs::write(&path, original).unwrap();

        update_karabiner_config(&dir, &rules()).unwrap();
        let updated = std::fs::read_to_string(&path).unwrap();
        assert_ne!(updated, original);
        assert_eq!(std::fs::read_to_string(&backup_path).unwrap(), original);

        // A run that changes nothing must not replace the backup with the current file.
        update_karabiner_config(&dir, &rules()).unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), updated);
        assert_eq!(std::fs::read_to_string(&backup_path).unwrap(), original);
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
