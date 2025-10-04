use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};

pub fn manipulators() -> Vec<Manipulator> {
    manipulators_with_app(&BundleIdentifier::VSCode)
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
    let super_modifiers = vec![Ctrl, Shift, Opt, Cmd];
    let super_without_cmd_modifiers = vec![Ctrl, Shift, Opt];
    let keys = vec![
        (K::A, &super_modifiers),             // execute command
        (K::B, &super_modifiers),             // show bookmarks
        (K::C, &super_without_cmd_modifiers), // GitHub Copilot: Open Completions Panel
        (K::D, &super_modifiers),             // Toggle sidebar
        (K::E, &super_modifiers),             // エクスプローラーを表示
        (K::F, &super_modifiers),             // search file
        (K::G, &super_modifiers),             // GitLens: Open File on Remote
        (K::H, &super_modifiers),             // Go Back
        (K::I, &super_modifiers),             // Go to implementation
        (K::J, &super_modifiers),             // Codex: Focus on Codex View
        (K::K, &super_modifiers),             // find in path
        (K::L, &super_modifiers),             // Go Forward
        (K::M, &super_without_cmd_modifiers), // Toggle bookmarks
        (K::N, &super_without_cmd_modifiers), // 次の問題へ移動
        (K::O, &super_modifiers),             // open recent
        (K::P, &super_without_cmd_modifiers), // 表示: 問題の切り替え
        (K::R, &super_modifiers),             // reload window
        (K::S, &super_modifiers),             // go to symbol
        (K::T, &super_without_cmd_modifiers), // ワークスペース内のシンボルへ移動
        (K::U, &super_without_cmd_modifiers), // 次の参照へ移動
        (K::V, &super_modifiers),             // Copy active file relative path
        (K::W, &super_modifiers),             // Claude Code: Open in Side Bar
        (K::Y, &super_modifiers),             // Toggle File Blame
        (K::Z, &super_modifiers),             // Focus on Source Control: Changes view
        (K::Key9, &super_modifiers),          // Zoom in
        (K::Key0, &super_modifiers),          // Zoom out
        (K::ReturnOrEnter, &super_modifiers), // workbench.action.tasks.reRunTask
    ];

    keys.into_iter()
        .map(|(key, modifiers)| create_vk4_self_mapping(bundle_id, key, modifiers.clone()))
        .collect()
}

fn manipulators_with_app(bundle_identifier: &BundleIdentifier) -> Vec<Manipulator> {
    vec![
        create_universal_command_mappings(bundle_identifier),
        vec![create_vk1_mapping(bundle_identifier, K::W, K::S, vec![Cmd])],
    ]
    .into_iter()
    .flatten()
    .collect()
}
