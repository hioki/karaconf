use crate::karabiner_data::{
    BundleIdentifier::Calendar, Condition, KeyCode::*, Manipulator, ModifierKey::*, VirtualKey,
};

pub fn manipulators() -> Vec<Manipulator> {
    let mut manipulators = vec![];
    for (description, from_key, (to_key, modifiers)) in [
        ("Today", T, (T, Some(vec![Cmd]))),
        ("Day", D, (Key1, Some(vec![Cmd]))),
        ("Week", W, (Key2, Some(vec![Cmd]))),
        ("Month", M, (Key3, Some(vec![Cmd]))),
        ("Year", Y, (Key4, Some(vec![Cmd]))),
        ("Next", N, (RightArrow, Some(vec![Cmd]))),
        ("Previous", P, (LeftArrow, Some(vec![Cmd]))),
    ] {
        manipulators.push(
            Manipulator::builder()
                .description(description)
                .conditions(vec![
                    Condition::on_app(Calendar),
                    Condition::with_virtual_key(VirtualKey::Vk4),
                ])
                .from_key(from_key)
                .to_key(to_key, modifiers)
                .build(),
        )
    }
    manipulators
}
