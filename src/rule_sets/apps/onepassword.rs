use crate::karabiner_data::{
    BundleIdentifier::OnePassword, Condition, KeyCode::*, Manipulator, ModifierKey::*, VirtualKey,
};

pub fn manipulators() -> Vec<Manipulator> {
    let mut manipulators = vec![];
    for (description, from_key, (to_key, modifiers)) in [
        ("Toggle sidebar", E, (D, Some(vec![Cmd, Shift]))),
        ("Copy username", U, (C, Some(vec![Cmd]))),
        ("Copy password", P, (C, Some(vec![Cmd, Shift]))),
        ("Copy TOTP", T, (C, Some(vec![Cmd, Opt]))),
        ("Open URL", X, (F, Some(vec![Cmd, Shift]))),
        ("Toggle masked field", M, (R, Some(vec![Cmd]))),
    ] {
        manipulators.push(
            Manipulator::builder()
                .description(description)
                .conditions(vec![
                    Condition::on_app(OnePassword),
                    Condition::with_virtual_key(VirtualKey::Vk4),
                ])
                .from_key(from_key)
                .to_key(to_key, modifiers)
                .build(),
        )
    }
    manipulators
}
