use crate::karabiner_data::{
    BundleIdentifier::Mail, Condition, KeyCode::*, Manipulator, ModifierKey::*, VirtualKey,
};

pub fn manipulators() -> Vec<Manipulator> {
    let mut manipulators = vec![];
    for (description, from_key, (to_key, modifiers)) in [
        ("Toggle sidebar", E, (S, Some(vec![Cmd, Ctrl]))),
        ("Toggle toolbar", J, (H, Some(vec![Cmd, Opt, Shift]))),
        ("Search", K, (F, Some(vec![Cmd, Opt]))),
        ("Show Header", H, (H, Some(vec![Cmd, Shift]))),
        ("Toggle flag", M, (L, Some(vec![Cmd, Shift]))),
    ] {
        manipulators.push(
            Manipulator::builder()
                .description(description)
                .conditions(vec![
                    Condition::on_app(Mail),
                    Condition::with_virtual_key(VirtualKey::Vk4),
                ])
                .from_key(from_key)
                .to_key(to_key, modifiers)
                .build(),
        )
    }
    manipulators
}
