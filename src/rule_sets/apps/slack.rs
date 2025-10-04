use crate::karabiner_data::{
    BundleIdentifier::Slack, Condition, KeyCode::*, Manipulator, ModifierKey::*,
};

pub fn manipulators() -> Vec<Manipulator> {
    vec![
        (T, T, vec![Cmd, Shift]),               // Threads
        (U, A, vec![Cmd, Shift]),               // All Unreads
        (E, D, vec![Cmd, Shift]),               // Toggle Sidebar
        (K, G, vec![Cmd]),                      // Search
        (F, K, vec![Cmd]),                      // Jump
        (B, S, vec![Cmd, Shift]),               // Bookmarks
        (D, X, vec![Cmd, Shift]),               // Strike through
        (OpenBracket, C, vec![Cmd, Shift]),     // Code
        (C, C, vec![Cmd, Opt, Shift]),          // Code Block
        (Q, Key9, vec![Cmd, Shift]),            // Quote
        (Spacebar, Spacebar, vec![Cmd, Shift]), // Mute huddle
    ]
    .into_iter()
    .map(|(from, to, modifiers)| {
        Manipulator::builder()
            .conditions(vec![Condition::on_app(Slack), Condition::with_vk4()])
            .from_key(from)
            .to_key(to, Some(modifiers))
            .build()
    })
    .collect()
}
