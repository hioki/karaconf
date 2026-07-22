use crate::karabiner_data::{
    BundleIdentifier::NotionCalendar, Condition, KeyCode::*, Manipulator, ModifierKey::*,
    VirtualKey,
};

pub fn manipulators() -> Vec<Manipulator> {
    let mut manipulators = vec![];
    for (description, from_key, (to_key, modifiers)) in [
        (
            "Toggle sidebar",
            E,
            (International3, Some(vec![Cmd, Shift])),
        ),
        (
            "Toggle secondary sidebar",
            J,
            (International3, Some(vec![Opt])),
        ),
        ("次の日/週/月", N, (RightArrow, None)),
        ("前の日/週/月", P, (LeftArrow, None)),
    ] {
        manipulators.push(
            Manipulator::builder()
                .description(description)
                .conditions(vec![
                    Condition::on_app(NotionCalendar),
                    Condition::with_virtual_key(VirtualKey::Vk4),
                ])
                .from_key(from_key)
                .to_key(to_key, modifiers)
                .build(),
        )
    }
    manipulators
}
