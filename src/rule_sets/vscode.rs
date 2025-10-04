use std::collections::HashMap;

use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};

pub fn manipulators() -> Vec<Manipulator> {
    let ctrl_shft_opt_cmd = vec![Ctrl, Shift, Opt, Cmd];
    let ctrl_shft_opt = vec![Ctrl, Shift, Opt];

    let m: HashMap<VirtualKey, Vec<(K, &Vec<ModifierKey>)>> = HashMap::from([
        (
            VirtualKey::Vk1,
            vec![
                (K::W, &ctrl_shft_opt), // ファイルに保存
            ],
        ),
        (
            VirtualKey::Vk4,
            vec![
                (K::A, &ctrl_shft_opt_cmd),             // execute command
                (K::B, &ctrl_shft_opt_cmd),             // show bookmarks
                (K::C, &ctrl_shft_opt),                 // GitHub Copilot: Open Completions Panel
                (K::D, &ctrl_shft_opt_cmd),             // Toggle sidebar
                (K::E, &ctrl_shft_opt_cmd),             // エクスプローラーを表示
                (K::F, &ctrl_shft_opt_cmd),             // search file
                (K::G, &ctrl_shft_opt_cmd),             // GitLens: Open File on Remote
                (K::H, &ctrl_shft_opt_cmd),             // Go Back
                (K::I, &ctrl_shft_opt_cmd),             // Go to implementation
                (K::J, &ctrl_shft_opt_cmd),             // Codex: Focus on Codex View
                (K::K, &ctrl_shft_opt_cmd),             // find in path
                (K::L, &ctrl_shft_opt_cmd),             // Go Forward
                (K::M, &ctrl_shft_opt),                 // Toggle bookmarks
                (K::N, &ctrl_shft_opt),                 // 次の問題へ移動
                (K::O, &ctrl_shft_opt_cmd),             // open recent
                (K::P, &ctrl_shft_opt),                 // 表示: 問題の切り替え
                (K::R, &ctrl_shft_opt_cmd),             // reload window
                (K::S, &ctrl_shft_opt_cmd),             // go to symbol
                (K::T, &ctrl_shft_opt),                 // ワークスペース内のシンボルへ移動
                (K::U, &ctrl_shft_opt),                 // 次の参照へ移動
                (K::V, &ctrl_shft_opt_cmd),             // Copy active file relative path
                (K::W, &ctrl_shft_opt_cmd),             // Claude Code: Open in Side Bar
                (K::Y, &ctrl_shft_opt_cmd),             // Toggle File Blame
                (K::Z, &ctrl_shft_opt_cmd),             // Focus on Source Control: Changes view
                (K::Key9, &ctrl_shft_opt_cmd),          // Zoom in
                (K::Key0, &ctrl_shft_opt_cmd),          // Zoom out
                (K::ReturnOrEnter, &ctrl_shft_opt_cmd), // workbench.action.tasks.reRunTask
                (K::Period, &ctrl_shft_opt),            // quick fix
            ],
        ),
    ]);

    m.into_iter()
        .flat_map(|(vk, mappings)| {
            mappings.into_iter().map(move |(key, modifiers)| {
                Manipulator::builder()
                    .condition(Condition::on_app(BundleIdentifier::VSCode))
                    .condition(Condition::with_virtual_key(vk.clone()))
                    .from_key(key.clone())
                    .to_key(key.clone(), Some(modifiers.clone()))
                    .build()
            })
        })
        .collect()
}
