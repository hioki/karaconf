use crate::karabiner_data::{
    BundleIdentifier::ChatGPT, Condition, KeyCode::*, Manipulator, ModifierKey::*,
};

pub fn manipulators() -> Vec<Manipulator> {
    vec![
        Manipulator::builder()
            .conditions(vec![Condition::on_app(ChatGPT), Condition::with_vk4()])
            .from_key(E)
            .to_key(S, Some(vec![Cmd, Ctrl]))
            .build(),
    ]
}
