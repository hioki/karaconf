use crate::karabiner_data::{Condition, FromModifier, KeyCode::*, Manipulator, ModifierKey::*};

pub fn manipulators() -> Vec<Manipulator> {
    let mut manipulators = Vec::new();

    for (from, to, to_modifiers) in [
        (F, Tab, Some(vec![Cmd])),           // move to next app
        (D, Tab, Some(vec![Cmd, Shift])),    // move to previous app
        (S, Tab, Some(vec![Ctrl])),          // move to next tab
        (A, Tab, Some(vec![Ctrl, Shift])),   // move to previous tab
        (Key9, KeypadPlus, Some(vec![Cmd])), // zoom in
        (Key0, Hyphen, Some(vec![Cmd])),     // zoom out
        (Key1, VolumeDecrement, None),
        (Key2, VolumeIncrement, None),
        (Key3, DisplayBrightnessDecrement, None),
        (Key4, DisplayBrightnessIncrement, None),
    ] {
        manipulators.push(
            Manipulator::builder()
                .condition(Condition::with_vk2())
                .from_key(from)
                .to_key(to, to_modifiers)
                .build(),
        );
    }

    // for Magnet.app
    for (from, to) in [
        (H, LeftArrow),
        (O, RightArrow),
        (N, DownArrow),
        (P, UpArrow),
        (U, Key1),
        (I, Key2),
        (M, Key3),
        (Comma, Key4),
        (J, P),
        (K, N),
    ] {
        manipulators.push(
            Manipulator::builder()
                .condition(Condition::with_vk2())
                .from_key_with_modifiers(from, FromModifier::Mandatory(vec![Ctrl]))
                .to_key(to, Some(vec![Cmd, Ctrl, Opt, Shift]))
                .build(),
        );
    }

    manipulators
}
