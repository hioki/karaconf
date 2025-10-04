use crate::karabiner_data::{
    BundleIdentifier, Condition, KeyCode as K, Manipulator,
    ModifierKey::{self, Cmd, Ctrl, Opt, Shift},
    VirtualKey,
};

const CTRL_SHIFT_OPT_CMD: &[ModifierKey] = &[Ctrl, Shift, Opt, Cmd];
const CTRL_SHIFT_OPT: &[ModifierKey] = &[Ctrl, Shift, Opt];

pub fn manipulators() -> Vec<Manipulator> {
    let mut m: std::collections::HashMap<VirtualKey, Vec<(K, &[ModifierKey])>> =
        std::collections::HashMap::new();
    m.insert(
        VirtualKey::Vk1,
        vec![
            (K::W, CTRL_SHIFT_OPT), // Save file
        ],
    );
    m.insert(
        VirtualKey::Vk4,
        vec![
            (K::A, CTRL_SHIFT_OPT_CMD),             // execute command
            (K::B, CTRL_SHIFT_OPT_CMD),             // show bookmarks
            (K::C, CTRL_SHIFT_OPT),                 // GitHub Copilot: Open Completions Panel
            (K::D, CTRL_SHIFT_OPT_CMD),             // Toggle sidebar
            (K::E, CTRL_SHIFT_OPT_CMD),             // Show Explorer
            (K::F, CTRL_SHIFT_OPT_CMD),             // search file
            (K::G, CTRL_SHIFT_OPT_CMD),             // GitLens: Open File on Remote
            (K::H, CTRL_SHIFT_OPT_CMD),             // Go Back
            (K::I, CTRL_SHIFT_OPT_CMD),             // Go to implementation
            (K::J, CTRL_SHIFT_OPT_CMD),             // Codex: Focus on Codex View
            (K::K, CTRL_SHIFT_OPT_CMD),             // find in path
            (K::L, CTRL_SHIFT_OPT_CMD),             // Go Forward
            (K::M, CTRL_SHIFT_OPT),                 // Toggle bookmarks
            (K::N, CTRL_SHIFT_OPT),                 // Go to next problem
            (K::O, CTRL_SHIFT_OPT_CMD),             // open recent
            (K::P, CTRL_SHIFT_OPT),                 // Toggle Problems view
            (K::R, CTRL_SHIFT_OPT_CMD),             // reload window
            (K::S, CTRL_SHIFT_OPT_CMD),             // go to symbol
            (K::T, CTRL_SHIFT_OPT),                 // Go to workspace symbol
            (K::U, CTRL_SHIFT_OPT),                 // Go to next reference
            (K::V, CTRL_SHIFT_OPT_CMD),             // Copy active file relative path
            (K::W, CTRL_SHIFT_OPT_CMD),             // Claude Code: Open in Side Bar
            (K::Y, CTRL_SHIFT_OPT_CMD),             // Toggle File Blame
            (K::Z, CTRL_SHIFT_OPT_CMD),             // Focus on Source Control: Changes view
            (K::Key9, CTRL_SHIFT_OPT_CMD),          // Zoom in
            (K::Key0, CTRL_SHIFT_OPT_CMD),          // Zoom out
            (K::ReturnOrEnter, CTRL_SHIFT_OPT_CMD), // workbench.action.tasks.reRunTask
            (K::Period, CTRL_SHIFT_OPT),            // quick fix
        ],
    );
    m.iter()
        .flat_map(|(vk, mappings)| {
            mappings.iter().map(|(key, modifiers)| {
                Manipulator::builder()
                    .condition(Condition::on_app(BundleIdentifier::VSCode))
                    .condition(Condition::with_virtual_key(vk.clone()))
                    .from_key(key.clone())
                    .to_key(key.clone(), Some(modifiers.to_vec()))
                    .build()
            })
        })
        .collect()
}
