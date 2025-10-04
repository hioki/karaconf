use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};

pub fn manipulators() -> Vec<Manipulator> {
    let ctrl_shft_opt_cmd = vec![Ctrl, Shift, Opt, Cmd];
    let ctrl_shft_opt = vec![Ctrl, Shift, Opt];

    let keys = vec![
        // Vk1
        (VirtualKey::Vk1, K::W, &ctrl_shft_opt), // ファイルに保存
        // Vk4
        (VirtualKey::Vk4, K::A, &ctrl_shft_opt_cmd), // execute command
        (VirtualKey::Vk4, K::B, &ctrl_shft_opt_cmd), // show bookmarks
        (VirtualKey::Vk4, K::C, &ctrl_shft_opt),     // GitHub Copilot: Open Completions Panel
        (VirtualKey::Vk4, K::D, &ctrl_shft_opt_cmd), // Toggle sidebar
        (VirtualKey::Vk4, K::E, &ctrl_shft_opt_cmd), // エクスプローラーを表示
        (VirtualKey::Vk4, K::F, &ctrl_shft_opt_cmd), // search file
        (VirtualKey::Vk4, K::G, &ctrl_shft_opt_cmd), // GitLens: Open File on Remote
        (VirtualKey::Vk4, K::H, &ctrl_shft_opt_cmd), // Go Back
        (VirtualKey::Vk4, K::I, &ctrl_shft_opt_cmd), // Go to implementation
        (VirtualKey::Vk4, K::J, &ctrl_shft_opt_cmd), // Codex: Focus on Codex View
        (VirtualKey::Vk4, K::K, &ctrl_shft_opt_cmd), // find in path
        (VirtualKey::Vk4, K::L, &ctrl_shft_opt_cmd), // Go Forward
        (VirtualKey::Vk4, K::M, &ctrl_shft_opt),     // Toggle bookmarks
        (VirtualKey::Vk4, K::N, &ctrl_shft_opt),     // 次の問題へ移動
        (VirtualKey::Vk4, K::O, &ctrl_shft_opt_cmd), // open recent
        (VirtualKey::Vk4, K::P, &ctrl_shft_opt),     // 表示: 問題の切り替え
        (VirtualKey::Vk4, K::R, &ctrl_shft_opt_cmd), // reload window
        (VirtualKey::Vk4, K::S, &ctrl_shft_opt_cmd), // go to symbol
        (VirtualKey::Vk4, K::T, &ctrl_shft_opt),     // ワークスペース内のシンボルへ移動
        (VirtualKey::Vk4, K::U, &ctrl_shft_opt),     // 次の参照へ移動
        (VirtualKey::Vk4, K::V, &ctrl_shft_opt_cmd), // Copy active file relative path
        (VirtualKey::Vk4, K::W, &ctrl_shft_opt_cmd), // Claude Code: Open in Side Bar
        (VirtualKey::Vk4, K::Y, &ctrl_shft_opt_cmd), // Toggle File Blame
        (VirtualKey::Vk4, K::Z, &ctrl_shft_opt_cmd), // Focus on Source Control: Changes view
        (VirtualKey::Vk4, K::Key9, &ctrl_shft_opt_cmd), // Zoom in
        (VirtualKey::Vk4, K::Key0, &ctrl_shft_opt_cmd), // Zoom out
        (VirtualKey::Vk4, K::ReturnOrEnter, &ctrl_shft_opt_cmd), // workbench.action.tasks.reRunTask
    ];

    keys.into_iter()
        .map(|(vk, key, modifiers)| {
            Manipulator::builder()
                .condition(Condition::on_app(BundleIdentifier::VSCode))
                .condition(Condition::with_virtual_key(vk))
                .from_key(key.clone())
                .to_key(key, Some(modifiers.clone()))
                .build()
        })
        .collect()
}
