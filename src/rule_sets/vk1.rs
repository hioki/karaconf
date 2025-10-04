use crate::karabiner_data::{KeyCode::*, ModifierKey::*, *};

pub fn manipulators() -> Vec<Manipulator> {
    let mut manipulators = Vec::new();

    for (from, to, modifiers) in [
        (H, LeftArrow, None),
        (J, DownArrow, None),
        (K, UpArrow, None),
        (L, RightArrow, None),
        (F, Escape, None),
    ] {
        manipulators.push(
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key_with_modifiers(from, FromModifier::Optional(vec![Any]))
                .to_key(to, modifiers)
                .build(),
        );
    }

    for (from, to) in [(S, JapaneseKana), (D, JapaneseEisuu), (A, F10), (Z, F7)] {
        manipulators.push(
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key(from)
                .to_key(to, None)
                .build(),
        );
    }

    vec![
        manipulators,
        vec![
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key_with_modifiers(U, FromModifier::Optional(vec![Any]))
                .to_key(LeftArrow, Some(vec![Cmd]))
                .build(),
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key_with_modifiers(I, FromModifier::Optional(vec![Any]))
                .to_key(RightArrow, Some(vec![Cmd]))
                .build(),
        ],
        vec![
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key_with_modifiers(G, FromModifier::Optional(vec![Any]))
                .to_key(Tab, None)
                .build(),
        ],
        vec![
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key(O)
                .to_key(Tab, Some(vec![Ctrl, Shift]))
                .build(),
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key(P)
                .to_key(Tab, Some(vec![Ctrl]))
                .build(),
        ],
        vec![
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key_with_modifiers(Y, FromModifier::Mandatory(vec![Shift]))
                .to_key(C, Some(vec![Cmd]))
                .to_command(
                    "export LC_ALL=en_US.UTF-8; pbpaste | tr -d '\n' | sed 's/  */ /g' | pbcopy",
                )
                .build(),
        ],
        vec![
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key(Y)
                .to_key(C, Some(vec![Cmd]))
                .build(),
        ],
        vec![
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key(T)
                .to_key(X, Some(vec![Cmd]))
                .build(),
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key(X)
                .to_key(V, Some(vec![Cmd, Shift, Opt]))
                .build(),
        ],
        vec![
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key(C)
                .to_key(DeleteOrBackspace, None)
                .build(),
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key(E)
                .to_key(DeleteForward, None)
                .build(),
        ],
        vec![
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key_with_modifiers(CloseBracket, FromModifier::Optional(vec![Any]))
                .to_key(Z, Some(vec![Cmd]))
                .build(),
        ],
        vec![
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key(Quote)
                .to_key(H, Some(vec![Cmd]))
                .build(),
        ],
        {
            let shift_mappings = vec![
                (N, Some(-1536), None),
                (M, None, Some(1536)),
                (Comma, None, Some(-1536)),
                (Period, Some(1536), None),
            ]
            .into_iter()
            .map(|(key_code, x, y)| {
                Manipulator::builder()
                    .condition(Condition::with_vk1())
                    .from_key_with_modifiers(key_code, FromModifier::Mandatory(vec![Shift]))
                    .to_mouse(MouseKey {
                        x,
                        y,
                        vertical_wheel: None,
                    })
                    .build()
            });
            let normal_mappings = vec![
                (N, Some(-3072), None),
                (M, None, Some(3072)),
                (Comma, None, Some(-3072)),
                (Period, Some(3072), None),
            ]
            .into_iter()
            .map(|(key_code, x, y)| {
                Manipulator::builder()
                    .condition(Condition::with_vk1())
                    .from_key(key_code)
                    .to_mouse(MouseKey {
                        x,
                        y,
                        vertical_wheel: None,
                    })
                    .build()
            });
            shift_mappings.chain(normal_mappings).collect()
        },
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
