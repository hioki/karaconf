use crate::karabiner_data::{
    BundleIdentifier::GitHubCopilot, Condition, KeyCode::*, Manipulator, ModifierKey::*, VirtualKey,
};

pub fn manipulators() -> Vec<Manipulator> {
    let mut manipulators = vec![];
    for (description, from_key, (to_key, modifiers)) in [
        ("View keyboard shortcuts", Slash, (Slash, Some(vec![Shift]))),
        ("Command palette", A, (K, Some(vec![Cmd]))),
        ("Find in file", K, (F, Some(vec![Cmd]))),
        ("Next session", Key2, (G, Some(vec![Cmd]))),
        ("Previous session", Key1, (P, Some(vec![Cmd, Shift]))),
        ("Go back", H, (CloseBracket, Some(vec![Cmd]))),
        ("Go forward", L, (NonUsPound, Some(vec![Cmd]))),
        ("Toggle sidebar", E, (B, Some(vec![Cmd]))),
        ("Go to file", F, (P, Some(vec![Cmd]))),
        ("New chat", N, (O, Some(vec![Cmd, Shift]))),
        ("Toggle review panel", J, (B, Some(vec![Cmd, Opt]))),
    ] {
        manipulators.push(
            Manipulator::builder()
                .description(description)
                .conditions(vec![
                    Condition::on_app(GitHubCopilot),
                    Condition::with_virtual_key(VirtualKey::Vk4),
                ])
                .from_key(from_key)
                .to_key(to_key, modifiers)
                .build(),
        )
    }
    manipulators
}
