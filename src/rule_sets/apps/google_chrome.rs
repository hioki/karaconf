use crate::karabiner_data::{
    BundleIdentifier::GoogleChrome, Condition, KeyCode::*, Manipulator, ModifierKey::*,
    VirtualKey::Vk4,
};

pub fn manipulators() -> Vec<Manipulator> {
    vec![
        // Mute google meets
        Manipulator::builder()
            .conditions(vec![
                Condition::on_app(GoogleChrome),
                Condition::with_virtual_key(Vk4),
            ])
            .from_key(Spacebar)
            .to_key(D, Some(vec![Cmd]))
            .build(),
        // List profiles
        Manipulator::builder()
            .conditions(vec![
                Condition::on_app(GoogleChrome),
                Condition::with_virtual_key(Vk4),
            ])
            .from_key(M)
            .to_key(M, Some(vec![Cmd, Shift]))
            .build(),
        // Switch profile
        Manipulator::builder()
            .conditions(vec![
                Condition::on_app(GoogleChrome),
                Condition::with_virtual_key(Vk4),
            ])
            .from_key(N)
            .to_key(M, Some(vec![Cmd, Shift]))
            .to_key(DownArrow, None)
            .to_key(DownArrow, None)
            .to_key(DownArrow, None)
            .to_key(ReturnOrEnter, None)
            .build(),
    ]
}
