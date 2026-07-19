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
    #[allow(clippy::type_complexity)]
    let m: BTreeMap<VirtualKey, Vec<(KeyCode, &[ModifierKey], &str)>> = BTreeMap::from([
        (VirtualKey::Vk1, vec![(W, CTRL_SHIFT_OPT, "Save file")]),
        (
            VirtualKey::Vk4,
            vec![
                (A, CTRL_SHIFT_OPT_CMD, "Execute command"),
                (B, CTRL_SHIFT_OPT_CMD, "Show bookmarks"),
                (C, CTRL_SHIFT_OPT, "GitHub Copilot: Open Completions Panel"),
                (D, CTRL_SHIFT_OPT_CMD, "Toggle sidebar"),
                (E, CTRL_SHIFT_OPT_CMD, "Show Explorer"),
                (F, CTRL_SHIFT_OPT_CMD, "Search file"),
                (G, CTRL_SHIFT_OPT_CMD, "GitLens: Open File on Remote"),
                (H, CTRL_SHIFT_OPT_CMD, "Go Back"),
                (I, CTRL_SHIFT_OPT_CMD, "Go to implementation"),
                (J, CTRL_SHIFT_OPT_CMD, "Toggle secondary sidebar"),
                (K, CTRL_SHIFT_OPT_CMD, "Search"),
                (L, CTRL_SHIFT_OPT_CMD, "Go Forward"),
                (M, CTRL_SHIFT_OPT, "Bookmarks: Toggle bookmarks"),
                (N, CTRL_SHIFT_OPT, "Go to next problem"),
                (O, CTRL_SHIFT_OPT_CMD, "Open recent"),
                (P, CTRL_SHIFT_OPT, "Toggle Problems view"),
                // (Q, None)
                (R, CTRL_SHIFT_OPT_CMD, "Search all references"),
                (S, CTRL_SHIFT_OPT_CMD, "Go to symbol"),
                (T, CTRL_SHIFT_OPT, "Go to workspace symbol"),
                (U, CTRL_SHIFT_OPT, "Go to next reference"),
                (V, CTRL_SHIFT_OPT_CMD, "Copy active file relative path"),
                (W, CTRL_SHIFT_OPT_CMD, "Focus on changes view"),
                (X, CTRL_SHIFT_OPT_CMD, "Close editor"),
                (Y, CTRL_SHIFT_OPT_CMD, "Toggle File Blame"),
                // (Z, CTRL_SHIFT_OPT_CMD, ""),
                (Period, CTRL_SHIFT_OPT, "Quick Fix"),
                (Key1, CTRL_SHIFT_OPT, "Split the editor to the left"),
                (Key2, CTRL_SHIFT_OPT, "Split the editor to the bottom"),
                (Key3, CTRL_SHIFT_OPT, "Split the editor to the top"),
                (Key4, CTRL_SHIFT_OPT, "Split the editor to the right"),
            ],
        ),
    ]);
    m.iter()
        .flat_map(|(vk, mappings)| {
            mappings.iter().map(|(key, modifiers, description)| {
                Manipulator::builder()
                    .description(*description)
                    .condition(Condition::on_app(VSCode))
                    .condition(Condition::with_virtual_key(vk.clone()))
                    .from_key(key.clone())
                    .to_key(key.clone(), Some(modifiers.to_vec()))
                    .build()
            })
        })
        .collect()
}
