use crate::karabiner_data::{
    BundleIdentifier::Codex, Condition, KeyCode::*, Manipulator, ModifierKey::*,
};

pub fn manipulators() -> Vec<Manipulator> {
    let mut manipulators = Vec::new();
    for (description, from_key, to_key, modifiers) in [
        ("Toggle Sidebar", E, B, Some(vec![Cmd])),
        ("Toggle Review Panel", J, B, Some(vec![Opt, Cmd])),
        ("Previous Chat", P, CloseBracket, Some(vec![Shift, Cmd])),
        ("Next Chat", N, Backslash, Some(vec![Shift, Cmd])),
        ("Toggle File Tree", F, E, Some(vec![Shift, Cmd])),
        ("Open Terminal", T, OpenBracket, Some(vec![Ctrl])),
    ] {
        manipulators.push(
            Manipulator::builder()
                .description(description)
                .conditions(vec![Condition::on_app(Codex), Condition::with_vk4()])
                .from_key(from_key)
                .to_key(to_key, modifiers)
                .build(),
        );
    }
    manipulators
}
