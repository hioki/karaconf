use crate::karabiner_data::{
    BundleIdentifier::BitWarden, Condition, KeyCode::*, Manipulator, ModifierKey::*, VirtualKey,
};

pub fn manipulators() -> Vec<Manipulator> {
    let mut manipulators = vec![];
    for (description, from_key, (to_key, modifiers)) in [
        ("Toggle sidebar", E, (S, Some(vec![Cmd, Ctrl]))),
        ("Copy username", U, (U, Some(vec![Cmd]))),
        ("Copy password", P, (P, Some(vec![Cmd]))),
        ("Copy TOTP", T, (T, Some(vec![Cmd]))),
        ("Search", K, (F, Some(vec![Cmd]))),
    ] {
        manipulators.push(
            Manipulator::builder()
                .description(description)
                .conditions(vec![
                    Condition::on_app(BitWarden),
                    Condition::with_virtual_key(VirtualKey::Vk4),
                ])
                .from_key(from_key)
                .to_key(to_key, modifiers)
                .build(),
        )
    }
    manipulators
}
