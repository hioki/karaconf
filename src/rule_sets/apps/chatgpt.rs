use crate::karabiner_data::{KeyCode::*, ModifierKey::*, *};

pub fn manipulators() -> Vec<Manipulator> {
    vec![
        Manipulator::builder()
            .conditions(vec![
                Condition::on_app(BundleIdentifier::ChatGPT),
                Condition::with_vk4(),
            ])
            .from_key(E)
            .to_key(S, Some(vec![Cmd, Ctrl]))
            .build(),
    ]
}
