use crate::karabiner_data::{FromModifier, KeyCode::*, Manipulator, ModifierKey::*};

pub fn manipulators() -> Vec<Manipulator> {
    vec![
        // Ctrl+: -> '
        Manipulator::builder()
            .from_key_with_modifiers(Quote, FromModifier::Mandatory(vec![Ctrl]))
            .to_key(Key7, Some(vec![Shift]))
            .build(),
    ]
}
