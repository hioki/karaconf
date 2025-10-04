use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};

pub fn manipulators() -> Vec<Manipulator> {
    let vscode_manipulators = manipulators_with_app(&BundleIdentifier::VSCode);
    let cursor_manipulators = manipulators_with_app(&BundleIdentifier::Cursor);
    vscode_manipulators
        .into_iter()
        .chain(cursor_manipulators)
        .collect()
}

/// Create a manipulator that maps a key to the same key with VK4 condition
fn create_vk4_self_mapping(
    bundle_id: &BundleIdentifier,
    from_key: K,
    modifiers: Vec<ModifierKey>,
) -> Manipulator {
    Manipulator::builder()
        .condition(Condition::on_app(bundle_id.clone()))
        .condition(Condition::with_vk4())
        .from_key(from_key.clone())
        .to_key(from_key, Some(modifiers))
        .build()
}

/// Create a manipulator that maps a key to a different key with VK4 condition
fn create_vk4_key_mapping(
    bundle_id: &BundleIdentifier,
    from_key: K,
    to_key: K,
    modifiers: Vec<ModifierKey>,
) -> Manipulator {
    Manipulator::builder()
        .condition(Condition::on_app(bundle_id.clone()))
        .condition(Condition::with_vk4())
        .from_key(from_key)
        .to_key(to_key, Some(modifiers))
        .build()
}

/// Create a manipulator with VK1 condition
fn create_vk1_mapping(
    bundle_id: &BundleIdentifier,
    from_key: K,
    to_key: K,
    modifiers: Vec<ModifierKey>,
) -> Manipulator {
    Manipulator::builder()
        .condition(Condition::on_app(bundle_id.clone()))
        .condition(Condition::with_vk1())
        .from_key(from_key)
        .to_key(to_key, Some(modifiers))
        .build()
}

/// Create multiple manipulators for keys that map to themselves with Ctrl+Shift+Opt+Cmd
fn create_universal_command_mappings(bundle_id: &BundleIdentifier) -> Vec<Manipulator> {
    let keys = vec![
        K::A,             // execute command
        K::B,             // show bookmarks
        K::D,             // Toggle sidebar
        K::E,             // エクスプローラーを表示
        K::F,             // search file
        K::G,             // GitLens: Open File on Remote
        K::H,             // Go Back
        K::I,             // Go to implementation
        K::J,             // Codex: Focus on Codex View
        K::K,             // find in path
        K::L,             // Go Forward
        K::O,             // open recent
        K::R,             // reload window
        K::S,             // go to symbol
        K::V,             // Copy active file relative path
        K::W,             // Claude Code: Open in Side Bar
        K::Y,             // Toggle File Blame
        K::Z,             // Focus on Source Control: Changes view
        K::Key9,          // Zoom in
        K::Key0,          // Zoom out
        K::ReturnOrEnter, // workbench.action.tasks.reRunTask
    ];

    keys.into_iter()
        .map(|key| create_vk4_self_mapping(bundle_id, key, vec![Ctrl, Shift, Opt, Cmd]))
        .collect()
}

fn manipulators_with_app(bundle_identifier: &BundleIdentifier) -> Vec<Manipulator> {
    vec![
        create_universal_command_mappings(bundle_identifier),
        vec![create_vk4_key_mapping(
            bundle_identifier,
            K::J,
            K::S,
            vec![Cmd],
        )],
        vec![create_vk1_mapping(bundle_identifier, K::W, K::S, vec![Cmd])],
        vec![create_vk4_key_mapping(
            bundle_identifier,
            K::M,
            K::K,
            vec![Opt, Cmd],
        )],
        vec![create_vk4_key_mapping(
            bundle_identifier,
            K::U,
            K::F12,
            vec![Shift],
        )],
        vec![create_vk4_key_mapping(
            bundle_identifier,
            K::N,
            K::F8,
            vec![Opt],
        )],
        vec![create_vk4_self_mapping(
            bundle_identifier,
            K::Period,
            vec![Cmd],
        )],
        vec![create_vk4_self_mapping(bundle_identifier, K::T, vec![Cmd])],
        vec![create_vk4_key_mapping(
            bundle_identifier,
            K::P,
            K::M,
            vec![Cmd, Shift],
        )],
    ]
    .into_iter()
    .flatten()
    .collect()
}
