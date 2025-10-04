use crate::karabiner_data::{KeyCode::*, ModifierKey::*, *};

pub fn manipulators() -> Vec<Manipulator> {
    vec![
        Manipulator::builder()
            .from_key_with_modifiers(Semicolon, FromModifier::Mandatory(vec![Ctrl]))
            .to_key(Semicolon, None)
            .build(),
        Manipulator::builder()
            .from_key_with_modifiers(Semicolon, FromModifier::Mandatory(vec![Cmd, Shift]))
            .to_key(KeypadPlus, Some(vec![Cmd]))
            .build(),
        Manipulator::builder()
            .from_key(Semicolon)
            .to_key(ReturnOrEnter, None)
            .build(),
    ]
}
