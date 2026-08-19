use crate::karabiner_data::{
    BundleIdentifier::GitHubCopilot, Condition, KeyCode::*, Manipulator, ModifierKey::*, VirtualKey,
};

pub fn manipulators() -> Vec<Manipulator> {
    let mut manipulators = vec![];
    for (description, from_key, (to_key, modifiers)) in [
        ("View keyboard shortcuts", Slash, (Slash, Some(vec![Shift]))),
        ("Command palette", A, (K, Some(vec![Cmd]))),
        ("Find in file", F, (F, Some(vec![Cmd]))),
        ("Next session", N, (G, Some(vec![Cmd]))),
        ("Previous session", P, (P, Some(vec![Cmd, Shift]))),
        ("Go back", H, (CloseBracket, Some(vec![Cmd]))),
        ("Go forward", H, (NonUsPound, Some(vec![Cmd]))),
        ("Toggle sidebar", E, (B, Some(vec![Cmd]))),
        ("Go to file", K, (P, Some(vec![Cmd]))),
        ("New chat", N, (O, Some(vec![Cmd, Shift]))),
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
