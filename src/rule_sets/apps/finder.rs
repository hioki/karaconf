use crate::karabiner_data::{
    BundleIdentifier::Finder, Condition, KeyCode::*, Manipulator, ModifierKey::*, VirtualKey,
};

pub fn manipulators() -> Vec<Manipulator> {
    let mut manipulators = vec![];
    for (description, from_key, (to_key, modifiers)) in [
        ("Toggle sidebar", E, (S, Some(vec![Cmd, Ctrl]))),
        ("Toggle preview", J, (P, Some(vec![Cmd, Shift]))),
        ("フォルダーへ移動", F, (G, Some(vec![Cmd, Shift]))),
    ] {
        manipulators.push(
            Manipulator::builder()
                .description(description)
                .conditions(vec![
                    Condition::on_app(Finder),
                    Condition::with_virtual_key(VirtualKey::Vk4),
                ])
                .from_key(from_key)
                .to_key(to_key, modifiers)
                .build(),
        )
    }
    manipulators
}
