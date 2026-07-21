use crate::karabiner_data::{
    BundleIdentifier::GoogleChrome, Condition, KeyCode::*, Manipulator, ModifierKey::*,
    VirtualKey::Vk4,
};

pub fn manipulators() -> Vec<Manipulator> {
    vec![
        Manipulator::builder()
            .description("Mute google meets")
            .conditions(vec![
                Condition::on_app(GoogleChrome),
                Condition::with_virtual_key(Vk4),
            ])
            .from_key(Spacebar)
            .to_key(D, Some(vec![Cmd]))
            .build(),
        Manipulator::builder()
            .description("List profiles")
            .conditions(vec![
                Condition::on_app(GoogleChrome),
                Condition::with_virtual_key(Vk4),
            ])
            .from_key(M)
            .to_key(M, Some(vec![Cmd, Shift]))
            .build(),
        Manipulator::builder()
            .description("Toggle bookmarks")
            .conditions(vec![
                Condition::on_app(GoogleChrome),
                Condition::with_virtual_key(Vk4),
            ])
            .from_key(B)
            .to_key(B, Some(vec![Cmd, Shift]))
            .build(),
        Manipulator::builder()
            .description("Search tabs")
            .conditions(vec![
                Condition::on_app(GoogleChrome),
                Condition::with_virtual_key(Vk4),
            ])
            .from_key(R)
            .to_key(A, Some(vec![Cmd, Shift]))
            .build(),
        Manipulator::builder()
            .description("Developer tools")
            .conditions(vec![
                Condition::on_app(GoogleChrome),
                Condition::with_virtual_key(Vk4),
            ])
            .from_key(F)
            .to_key(F12, None)
            .build(),
    ]
}
