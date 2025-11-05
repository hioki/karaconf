use crate::karabiner_data::{
    BundleIdentifier::VSCode,
    Condition,
    KeyCode::{self, *},
    Manipulator,
    ModifierKey::{self, Cmd, Ctrl, Opt, Shift},
    VirtualKey,
};
use std::collections::BTreeMap;

const CTRL_SHIFT_OPT_CMD: &[ModifierKey] = &[Ctrl, Shift, Opt, Cmd];
const CTRL_SHIFT_OPT: &[ModifierKey] = &[Ctrl, Shift, Opt];

pub fn manipulators() -> Vec<Manipulator> {
    let m: BTreeMap<VirtualKey, Vec<(KeyCode, &[ModifierKey])>> = BTreeMap::from([
        (
            VirtualKey::Vk1,
            vec![
                (W, CTRL_SHIFT_OPT), // Save file
            ],
        ),
        (
            VirtualKey::Vk4,
            vec![
                (A, CTRL_SHIFT_OPT_CMD),             // execute command
                (B, CTRL_SHIFT_OPT_CMD),             // show bookmarks
                (C, CTRL_SHIFT_OPT),                 // GitHub Copilot: Open Completions Panel
                (D, CTRL_SHIFT_OPT_CMD),             // Toggle sidebar
                (E, CTRL_SHIFT_OPT_CMD),             // Show Explorer
                (F, CTRL_SHIFT_OPT_CMD),             // search file
                (G, CTRL_SHIFT_OPT_CMD),             // GitLens: Open File on Remote
                (H, CTRL_SHIFT_OPT_CMD),             // Go Back
                (I, CTRL_SHIFT_OPT_CMD),             // Go to implementation
                (J, CTRL_SHIFT_OPT_CMD),             // Codex: Focus on Codex View
                (K, CTRL_SHIFT_OPT_CMD),             // find in path
                (L, CTRL_SHIFT_OPT_CMD),             // Go Forward
                (M, CTRL_SHIFT_OPT),                 // Toggle bookmarks
                (N, CTRL_SHIFT_OPT),                 // Go to next problem
                (O, CTRL_SHIFT_OPT_CMD),             // open recent
                (P, CTRL_SHIFT_OPT),                 // Toggle Problems view
                (R, CTRL_SHIFT_OPT_CMD),             // reload window
                (S, CTRL_SHIFT_OPT_CMD),             // go to symbol
                (T, CTRL_SHIFT_OPT),                 // Go to workspace symbol
                (U, CTRL_SHIFT_OPT),                 // Go to next reference
                (V, CTRL_SHIFT_OPT_CMD),             // Copy active file relative path
                (W, CTRL_SHIFT_OPT_CMD),             // Claude Code: Open in Side Bar
                (Y, CTRL_SHIFT_OPT_CMD),             // Toggle File Blame
                (Z, CTRL_SHIFT_OPT_CMD),             // Focus on Source Control: Changes view
                (Key9, CTRL_SHIFT_OPT_CMD),          // Zoom in
                (Key0, CTRL_SHIFT_OPT_CMD),          // Zoom out
                (ReturnOrEnter, CTRL_SHIFT_OPT_CMD), // workbench.action.tasks.reRunTask
                (Period, CTRL_SHIFT_OPT),            // quick fix
                (Key1, CTRL_SHIFT_OPT),              // Split the editor to the left
                (Key2, CTRL_SHIFT_OPT),              // Split the editor to the bottom
                (Key3, CTRL_SHIFT_OPT),              // Split the editor to the top
                (Key4, CTRL_SHIFT_OPT),              // Split the editor to the right
            ],
        ),
    ]);
    m.iter()
        .flat_map(|(vk, mappings)| {
            mappings.iter().map(|(key, modifiers)| {
                Manipulator::builder()
                    .condition(Condition::on_app(VSCode))
                    .condition(Condition::with_virtual_key(vk.clone()))
                    .from_key(key.clone())
                    .to_key(key.clone(), Some(modifiers.to_vec()))
                    .build()
            })
        })
        .collect()
}
