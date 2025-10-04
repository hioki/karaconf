use crate::karabiner_data::{KeyCode::*, ModifierKey::*, *};

pub fn manipulators() -> Vec<Manipulator> {
    let mut manipulators = Vec::new();

    for (from, to, modifiers) in [
        (H, LeftArrow, None),
        (J, DownArrow, None),
        (K, UpArrow, None),
        (L, RightArrow, None),
        (F, Escape, None),
        (U, LeftArrow, Some(vec![Cmd])),
        (I, RightArrow, Some(vec![Cmd])),
        (G, Tab, None),
        (CloseBracket, Z, Some(vec![Cmd])),
    ] {
        manipulators.push(
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key_with_modifiers(from, FromModifier::Optional(vec![Any]))
                .to_key(to, modifiers)
                .build(),
        );
    }

    for (from, to, modifiers) in [
        (S, JapaneseKana, None),
        (D, JapaneseEisuu, None),
        (A, F10, None),
        (Z, F7, None),
        (O, Tab, Some(vec![Ctrl, Shift])),
        (P, Tab, Some(vec![Ctrl])),
        (Y, C, Some(vec![Cmd])),
        (T, X, Some(vec![Cmd])),
        (X, V, Some(vec![Cmd, Shift, Opt])),
        (C, DeleteOrBackspace, None),
        (E, DeleteForward, None),
        (Quote, H, Some(vec![Cmd])),
    ] {
        manipulators.push(
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key(from)
                .to_key(to, modifiers)
                .build(),
        );
    }

    manipulators.push(
        Manipulator::builder()
            .condition(Condition::with_vk1())
            .from_key_with_modifiers(Y, FromModifier::Mandatory(vec![Shift]))
            .to_key(C, Some(vec![Cmd]))
            .to_command(
                "export LC_ALL=en_US.UTF-8; pbpaste | tr -d '\n' | sed 's/  */ /g' | pbcopy",
            )
            .build(),
    );

    for (key_code, x, y) in [
        (N, Some(-1536), None),
        (M, None, Some(1536)),
        (Comma, None, Some(-1536)),
        (Period, Some(1536), None),
    ] {
        manipulators.push(
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key_with_modifiers(key_code, FromModifier::Mandatory(vec![Shift]))
                .to_mouse(MouseKey {
                    x,
                    y,
                    vertical_wheel: None,
                })
                .build(),
        );
    }

    for (key_code, x, y) in [
        (N, Some(-3072), None),
        (M, None, Some(3072)),
        (Comma, None, Some(-3072)),
        (Period, Some(3072), None),
    ] {
        manipulators.push(
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key(key_code)
                .to_mouse(MouseKey {
                    x,
                    y,
                    vertical_wheel: None,
                })
                .build(),
        )
    }

    vec![
        manipulators,
        vec![
            (Slash, PointingButton::Button1),
            (International1, PointingButton::Button2),
        ]
        .into_iter()
        .map(|(from_key, pointing_button)| {
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key_with_modifiers(from_key, FromModifier::Optional(vec![Any]))
                .to_click(pointing_button)
                .build()
        })
        .collect(),
        vec![(OpenBracket, -64), (NonUsPound, 64), (Backslash, 64)]
            .into_iter()
            .map(|(key_code, vertical_wheel)| {
                Manipulator::builder()
                    .condition(Condition::with_vk1())
                    .from_key(key_code)
                    .to_mouse(MouseKey {
                        x: None,
                        y: None,
                        vertical_wheel: Some(vertical_wheel),
                    })
                    .build()
            })
            .collect(),
        vec![
            (Key1, F1),
            (Key2, F2),
            (Key3, F3),
            (Key4, F4),
            (Key5, F5),
            (Key6, F6),
            (Key7, F7),
            (Key8, F8),
            (Key9, F9),
            (Key0, F10),
            (Hyphen, F11),
            (EqualSign, F12),
        ]
        .into_iter()
        .map(|(from, to)| {
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key_with_modifiers(from, FromModifier::Optional(vec![Any]))
                .to_key(to, None)
                .build()
        })
        .collect(),
        vec![
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key(B)
                .to_key(M, Some(vec![Ctrl, Opt, Cmd, Shift]))
                .build(),
        ],
        vec![
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key(International3)
                .to_key(D, Some(vec![Cmd, Opt]))
                .build(),
        ],
    ]
    .into_iter()
    .flatten()
    .collect()
}
