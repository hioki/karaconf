use crate::karabiner_data::{
    Condition, FromModifier, KeyCode::*, Manipulator, ModifierKey::*, MouseKey, PointingButton,
    SetVariable, VirtualKey,
};

const MOUSE_SPEED: i32 = 1536;
const WHEEL_SPEED: i32 = 64;
const SHINGETA_MODE_ON_COMMAND: &str =
    "mkdir -p \"$HOME/.cache/karaconf\" && printf 'on' > \"$HOME/.cache/karaconf/shingeta_mode\"";
const SHINGETA_MODE_OFF_COMMAND: &str =
    "mkdir -p \"$HOME/.cache/karaconf\" && printf 'off' > \"$HOME/.cache/karaconf/shingeta_mode\"";

pub fn manipulators() -> Vec<Manipulator> {
    let mut manipulators = Vec::new();

    //
    // Virtual Key 1
    //
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

    // Mouse buttons
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

    // Mouse wheel
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

    //
    // Virtual Key 2
    //
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

    //
    // Virtual Key 2 - Shell Commands
    //
    for (key_code, shell_command) in [
        // (A, "Ctrl+Shift+Tab"),
        (B, "open -a 'Bitwarden.app'"),
        (C, "open -a 'Notion Calendar.app'"),
        // (D, "Command+Shift+Tab"),
        (
            E,
            r#"osascript -e "tell application \"Alfred 5\" to search \"snip \"""#,
        ),
        // (F, "Command+Tab"),
        (G, "open -a 'Visual Studio Code.app'"),
        // (H, None),
        (I, "open -a 'CLion.app'"),
        (J, "open -a \"Google Chrome\""),
        (K, "open -a 'iTerm.app'"),
        (L, "open -a 'Alfred 5.app'"),
        (M, "open -a 'Dynalist.app'"),
        (N, "open -a 'Notion.app'"),
        // (O, None),
        (P, "open -a '1Password.app'"),
        // (Q, None),
        // (R, None),
        // (S, "Ctrl+Tab"),
        (T, "open -a 'Visual Studio Code.app'"),
        (U, "open -a 'Microsoft To Do.app'"),
        (V, "open -a 'DeepL.app'"),
        (W, "open -a 'ChatGPT.app'"),
        (
            X,
            r#"osascript -e "tell application \"Alfred 5\" to search \"snip codeblocks\"""#,
        ),
        (Y, "open -a 'Slack.app'"),
        (Z, "open -a 'LICEcap.app'"),
        // (ReturnOrEnter, None),
        // (Quote, None), // :
        // (NonUsPound, None), // ]
        (OpenBracket, "open -a 'Mail.app'"), // @
        // (CloseBracket, None), // [
        (Comma, "open -a 'System Settings.app'"),
        (Period, "open -a 'Claude.app'"),
        (
            Slash,
            "open 'https://s2.kingtime.jp/independent/recorder2/personal/'",
        ),
        // (International1, None), // _
        // (NonUsPound, None),
        // (Backslash, None),
    ] {
        manipulators.push(
            Manipulator::builder()
                .condition(Condition::with_vk2())
                .from_key(key_code)
                .to_command(shell_command)
                .build(),
        )
    }

    //
    // Virtual Key 3
    //
    for (from, to) in [
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
    ] {
        manipulators.push(
            Manipulator::builder()
                .condition(Condition::with_vk3())
                .from_key_with_modifiers(from, FromModifier::Optional(vec![Any]))
                .to_key(to, None)
                .build(),
        )
    }

    manipulators.extend(vec![
        // Ctrl+; -> ;
        Manipulator::builder()
            .from_key_with_modifiers(Semicolon, FromModifier::Mandatory(vec![Ctrl]))
            .to_key(Semicolon, None)
            .build(),
        // Cmd+Shift+; -> Cmd+"+"
        Manipulator::builder()
            .from_key_with_modifiers(Semicolon, FromModifier::Mandatory(vec![Cmd, Shift]))
            .to_key(KeypadPlus, Some(vec![Cmd]))
            .build(),
        // ; -> Enter
        Manipulator::builder()
            .from_key(Semicolon)
            .to_key(ReturnOrEnter, None)
            .build(),
        // Ctrl+: -> '
        Manipulator::builder()
            .from_key_with_modifiers(Quote, FromModifier::Mandatory(vec![Ctrl]))
            .to_key(Key7, Some(vec![Shift]))
            .build(),
        // CapsLock -> (No Action)
        Manipulator::builder()
            .from_key_with_modifiers(CapsLock, FromModifier::Optional(vec![Any]))
            .to_key(VkNone, None)
            .build(),
    ]);

    // F12: Toggle between default romaji input and shingeta layout
    // When shingeta_mode is 0 (off), pressing F12 sets it to 1 (on)
    manipulators.push(
        Manipulator::builder()
            .condition(Condition::without_virtual_key(VirtualKey::ShingetaMode))
            .from_key(F12)
            .to_variable(SetVariable {
                name: VirtualKey::ShingetaMode,
                value: 1,
            })
            .to_command(SHINGETA_MODE_ON_COMMAND)
            .build(),
    );

    // When shingeta_mode is 1 (on), pressing F12 sets it to 0 (off)
    manipulators.push(
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .from_key(F12)
            .to_variable(SetVariable {
                name: VirtualKey::ShingetaMode,
                value: 0,
            })
            .to_command(SHINGETA_MODE_OFF_COMMAND)
            .build(),
    );

    // JapaneseEisuu (英数): Disable shingeta mode when switching to English input
    manipulators.push(
        Manipulator::builder()
            .from_key(JapaneseEisuu)
            .to_key(JapaneseEisuu, None)
            .to_variable(SetVariable {
                name: VirtualKey::ShingetaMode,
                value: 0,
            })
            .to_command(SHINGETA_MODE_OFF_COMMAND)
            .build(),
    );

    manipulators
}
