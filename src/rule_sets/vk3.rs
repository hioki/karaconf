use crate::karabiner_data::{KeyCode::*, ModifierKey::*, *};

pub fn manipulators() -> Vec<Manipulator> {
    vec![
        (A, Key1),
        (S, Key2),
        (D, Key3),
        (F, Key4),
        (G, Key5),
        (H, Key6),
        (J, Key7),
        (K, Key8),
        (L, Key9),
        (Semicolon, Key0),
        (Quote, Hyphen),
    ]
    .into_iter()
    .map(|(from, to)| {
        Manipulator::builder()
            .condition(Condition::with_vk3())
            .from_key_with_modifiers(from, FromModifier::Optional(vec![Any]))
            .to_key(to, None)
            .build()
    })
    .collect()
}
