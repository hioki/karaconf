use crate::karabiner_data::{KeyCode::*, ModifierKey::*, *};

const MOUSE_SPEED: i32 = 1536;
const WHEEL_SPEED: i32 = 64;

pub fn manipulators() -> Vec<Manipulator> {
    let mut manipulators = Vec::new();

    for (from, to, modifiers) in [
        (H, LeftArrow, None),
        (J, DownArrow, None),
        (K, UpArrow, None),
        (L, RightArrow, None),
        (F, Escape, None),
        (U, LeftArrow, Some(vec![Cmd])),  // go to line head
        (I, RightArrow, Some(vec![Cmd])), // go to line tail
        (G, Tab, None),
        (CloseBracket, Z, Some(vec![Cmd])), // undo
        //
        // F1-F12
        //
        (Key1, F1, None),
        (Key2, F2, None),
        (Key3, F3, None),
        (Key4, F4, None),
        (Key5, F5, None),
        (Key6, F6, None),
        (Key7, F7, None),
        (Key8, F8, None),
        (Key9, F9, None),
        (Key0, F10, None),
        (Hyphen, F11, None),
        (EqualSign, F12, None),
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
        (A, F10, None),                            // 英数に変換
        (Z, F7, None),                             // カタカナに変換
        (O, Tab, Some(vec![Ctrl, Shift])),         // move to previous tab
        (P, Tab, Some(vec![Ctrl])),                // move to next tab
        (Y, C, Some(vec![Cmd])),                   // copy
        (T, X, Some(vec![Cmd])),                   // cut
        (X, V, Some(vec![Cmd, Shift, Opt])),       // paste without formatting
        (C, DeleteOrBackspace, None),              // backspace
        (E, DeleteForward, None),                  // delete
        (Quote, H, Some(vec![Cmd])),               // hide current app
        (B, M, Some(vec![Ctrl, Opt, Cmd, Shift])), // maximize window
        (International3, D, Some(vec![Cmd, Opt])), // hide dock
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
        // what is this?
        Manipulator::builder()
            .condition(Condition::with_vk1())
            .from_key_with_modifiers(Y, FromModifier::Mandatory(vec![Shift]))
            .to_key(C, Some(vec![Cmd]))
            .to_command(
                "export LC_ALL=en_US.UTF-8; pbpaste | tr -d '\n' | sed 's/  */ /g' | pbcopy",
            )
            .build(),
    );

    // Mouse movement
    for (key_code, x, y) in [
        (N, Some(-MOUSE_SPEED), None),
        (M, None, Some(MOUSE_SPEED)),
        (Comma, None, Some(-MOUSE_SPEED)),
        (Period, Some(MOUSE_SPEED), None),
    ] {
        manipulators.push({
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key_with_modifiers(key_code.clone(), FromModifier::Mandatory(vec![Shift]))
                .to_mouse(MouseKey {
                    x,
                    y,
                    vertical_wheel: None,
                })
                .build()
        });
        manipulators.push({
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key(key_code)
                .to_mouse(MouseKey {
                    x: x.map(|n| n * 2),
                    y: y.map(|n| n * 2),
                    vertical_wheel: None,
                })
                .build()
        });
    }

    for (from_key, pointing_button) in [
        (Slash, PointingButton::Button1),          // left click
        (International1, PointingButton::Button2), // right click
    ] {
        manipulators.push(
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key_with_modifiers(from_key, FromModifier::Optional(vec![Any]))
                .to_click(pointing_button)
                .build(),
        );
    }

    for (key_code, vertical_wheel) in [
        (OpenBracket, -WHEEL_SPEED),
        (NonUsPound, WHEEL_SPEED),
        (Backslash, WHEEL_SPEED),
    ] {
        manipulators.push(
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key(key_code)
                .to_mouse(MouseKey {
                    x: None,
                    y: None,
                    vertical_wheel: Some(vertical_wheel),
                })
                .build(),
        );
    }

    manipulators
}
