use crate::karabiner_data::{
    BundleIdentifier::Firefox, Condition, KeyCode::*, Manipulator, ModifierKey::*, VirtualKey::Vk4,
};

pub fn manipulators() -> Vec<Manipulator> {
    vec![
        Manipulator::builder()
            .conditions(vec![
                Condition::on_app(Firefox),
                Condition::with_virtual_key(Vk4),
            ])
            .from_key(B)
            .to_key(B, Some(vec![Cmd]))
            .build(),
        Manipulator::builder()
            .conditions(vec![
                Condition::on_app(Firefox),
                Condition::with_virtual_key(Vk4),
            ])
            .from_key(E)
            .to_key(Z, Some(vec![Ctrl]))
            .build(),
        Manipulator::builder()
            .conditions(vec![
                Condition::on_app(Firefox),
                Condition::with_virtual_key(Vk4),
            ])
            .from_key(R)
            .to_key(L, Some(vec![Cmd]))
            .to_key(EqualSign, None)
            .build(),
    ]
}
