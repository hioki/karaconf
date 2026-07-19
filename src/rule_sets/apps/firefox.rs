use crate::karabiner_data::{
    BundleIdentifier::Firefox, Condition, KeyCode::*, Manipulator, ModifierKey::*, VirtualKey::Vk4,
};

pub fn manipulators() -> Vec<Manipulator> {
    let mut manipulators = Vec::new();
    for (description, from_key, to_key, modifiers) in [
        ("Toggle bookmarks", B, B, Some(vec![Cmd])),
        ("Expand/Collapse the Tab List Area", E, Z, Some(vec![Ctrl])),
        ("Select from the history list", R, L, Some(vec![Cmd])),
        ("Open Private Window", N, P, Some(vec![Cmd, Shift])),
    ] {
        manipulators.push(
            Manipulator::builder()
                .description(description)
                .conditions(vec![
                    Condition::on_app(Firefox),
                    Condition::with_virtual_key(Vk4),
                ])
                .from_key(from_key)
                .to_key(to_key, modifiers)
                .build(),
        );
    }
    manipulators
}
