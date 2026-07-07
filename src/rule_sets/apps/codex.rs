use crate::karabiner_data::{
    BundleIdentifier::Codex, Condition, KeyCode::*, Manipulator, ModifierKey::*,
};

pub fn manipulators() -> Vec<Manipulator> {
    vec![
        Manipulator::builder()
            .conditions(vec![Condition::on_app(Codex), Condition::with_vk4()])
            .from_key(E)
            .to_key(B, Some(vec![Cmd]))
            .build(),
        Manipulator::builder()
            .conditions(vec![Condition::on_app(Codex), Condition::with_vk4()])
            .from_key(J)
            .to_key(B, Some(vec![Opt, Cmd]))
            .build(),
    ]
}
