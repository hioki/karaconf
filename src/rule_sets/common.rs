use crate::karabiner_data::{
    Condition, FromModifier, KeyCode::*, Manipulator, ModifierKey::*, MouseKey, PointingButton,
    SetVariable, VirtualKey,
};

const MOUSE_SPEED: i32 = 768;
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
    for (from, to, modifiers, description) in [
        (H, LeftArrow, None, "カーソルを左に移動"),
        (J, DownArrow, None, "カーソルを下に移動"),
        (K, UpArrow, None, "カーソルを上に移動"),
        (L, RightArrow, None, "カーソルを右に移動"),
        (U, LeftArrow, Some(vec![Cmd]), "行頭へ移動"),
        (I, RightArrow, Some(vec![Cmd]), "行末へ移動"),
        (G, Tab, None, "Tabを入力"),
        (CloseBracket, Z, Some(vec![Cmd]), "元に戻す"),
        (Q, Q, Some(vec![Cmd]), "アプリを終了"),
        //
        // F1-F12
        //
        (Key1, F1, None, ""),
        (Key2, F2, None, ""),
        (Key3, F3, None, ""),
        (Key4, F4, None, ""),
        (Key5, F5, None, ""),
        (Key6, F6, None, ""),
        (Key7, F7, None, ""),
        (Key8, F8, None, ""),
        (Key9, F9, None, ""),
        (Key0, F10, None, ""),
        (Hyphen, F11, None, ""),
        (EqualSign, F12, None, ""),
    ] {
        let mut builder = Manipulator::builder()
            .condition(Condition::with_vk1())
            .from_key_with_modifiers(from, FromModifier::Optional(vec![Any]))
            .to_key(to, modifiers);
        if !description.is_empty() {
            builder = builder.description(description);
        }
        manipulators.push(builder.build());
    }
    manipulators.push(
        Manipulator::builder()
            .description("ESC & 英数 & 新下駄モードOFF")
            .condition(Condition::with_vk1())
            .from_key_with_modifiers(F, FromModifier::Optional(vec![Any]))
            .to_key(Escape, None)
            .to_key(JapaneseEisuu, None)
            .to_variable(SetVariable {
                name: VirtualKey::ShingetaMode,
                value: 0,
            })
            .to_command(SHINGETA_MODE_OFF_COMMAND)
            .build(),
    );

    manipulators.push(
        Manipulator::builder()
            .description("かな & 新下駄モードON")
            .condition(Condition::with_vk1())
            .from_key(S)
            .to_key(JapaneseKana, None)
            .to_variable(SetVariable {
                name: VirtualKey::ShingetaMode,
                value: 1,
            })
            .to_command(SHINGETA_MODE_ON_COMMAND)
            .build(),
    );

    manipulators.push(
        Manipulator::builder()
            .description("英数 & 新下駄モードOFF")
            .condition(Condition::with_vk1())
            .from_key(D)
            .to_key(JapaneseEisuu, None)
            .to_variable(SetVariable {
                name: VirtualKey::ShingetaMode,
                value: 0,
            })
            .to_command(SHINGETA_MODE_OFF_COMMAND)
            .build(),
    );

    // VK1+R -> type today's date (e.g. 2026-07-18)
    manipulators.push(
        Manipulator::builder()
            .description("今日の日付を入力")
            .condition(Condition::with_vk1())
            .from_key(R)
            .to_command(
                r#"osascript -e "tell application \"System Events\" to keystroke \"$(date +%F)\"""#,
            )
            .build(),
    );

    // VK1+X -> type current timestamp (e.g. 20260718215230)
    manipulators.push(
        Manipulator::builder()
            .description("現在のタイムスタンプを入力")
            .condition(Condition::with_vk1())
            .from_key(X)
            .to_command(
                r#"osascript -e "tell application \"System Events\" to keystroke \"$(date +%Y%m%d%H%M%S)\"""#,
            )
            .build(),
    );

    for (from, to, modifiers, description) in [
        (A, F10, None, "変換中の文字を英数に変換"),
        (Z, F7, None, "変換中の文字をカタカナに変換"),
        (O, Tab, Some(vec![Ctrl, Shift]), "前のタブに切り替え"),
        (P, Tab, Some(vec![Ctrl]), "次のタブに切り替え"),
        (Y, C, Some(vec![Cmd]), "コピー"),
        (T, X, Some(vec![Cmd]), "切り取り"),
        (V, V, Some(vec![Cmd, Shift, Opt]), "書式なしで貼り付け"),
        (C, DeleteOrBackspace, None, "Backspace"),
        (E, DeleteForward, None, "Delete"),
        (Quote, H, Some(vec![Cmd]), "アプリを隠す"),
        (
            B,
            M,
            Some(vec![Ctrl, Opt, Cmd, Shift]),
            "ウィンドウの最大化",
        ),
        (International3, D, Some(vec![Cmd, Opt]), "Dockを表示/非表示"),
        // NOTE: must stay before the unconditioned `; -> Enter` rule below;
        // otherwise that rule matches first and this one never fires.
        (
            Semicolon,
            ReturnOrEnter,
            Some(vec![Shift]),
            "送信せずに改行",
        ),
    ] {
        let mut builder = Manipulator::builder()
            .condition(Condition::with_vk1())
            .from_key(from)
            .to_key(to, modifiers);
        if !description.is_empty() {
            builder = builder.description(description);
        }
        manipulators.push(builder.build());
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
                    horizontal_wheel: None,
                })
                .build()
        });
        manipulators.push({
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key(key_code)
                .to_mouse(MouseKey {
                    x: x.map(|n| n * 4),
                    y: y.map(|n| n * 4),
                    vertical_wheel: None,
                    horizontal_wheel: None,
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
    for (key_code, wheel) in [
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
                    vertical_wheel: Some(wheel),
                    horizontal_wheel: None,
                })
                .build(),
        );
    }
    for (key_code, wheel) in [
        (OpenBracket, WHEEL_SPEED),
        (NonUsPound, -WHEEL_SPEED),
        (Backslash, -WHEEL_SPEED),
    ] {
        manipulators.push(
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key_with_modifiers(key_code, FromModifier::Mandatory(vec![Ctrl]))
                .to_mouse(MouseKey {
                    x: None,
                    y: None,
                    vertical_wheel: None,
                    horizontal_wheel: Some(wheel),
                })
                .build(),
        );
    }

    //
    // Virtual Key 2
    //
    for (from, to, to_modifiers, description) in [
        (F, Tab, Some(vec![Cmd]), "次のアプリに切り替え"),
        (D, Tab, Some(vec![Cmd, Shift]), "前のアプリに切り替え"),
        (S, Tab, Some(vec![Ctrl]), "次のタブに切り替え"),
        (A, Tab, Some(vec![Ctrl, Shift]), "前のタブに切り替え"),
        (H, N, Some(vec![Ctrl, Shift, Cmd]), "通知センターを開く"),
        (
            Z,
            Key4,
            Some(vec![Shift, Cmd]),
            "スクリーンショット（選択範囲）",
        ),
        (Key8, Key0, Some(vec![Cmd]), "拡大/縮小をリセット"),
        (Key9, KeypadPlus, Some(vec![Cmd]), "拡大"),
        (Key0, Hyphen, Some(vec![Cmd]), "縮小"),
        (Key1, VolumeDecrement, None, "音量を下げる"),
        (Key2, VolumeIncrement, None, "音量を上げる"),
        (
            Key3,
            DisplayBrightnessDecrement,
            None,
            "画面の明るさを下げる",
        ),
        (
            Key4,
            DisplayBrightnessIncrement,
            None,
            "画面の明るさを上げる",
        ),
        (Key5, Rewind, None, "巻き戻し"),
        (Key6, PlayOrPause, None, "再生/一時停止"),
        (Key7, Fastforward, None, "早送り"),
        (Q, Q, Some(vec![Ctrl, Cmd]), "スクリーンロック"),
        (
            Semicolon,
            ReturnOrEnter,
            Some(vec![Cmd]),
            "複数行の文字列の送信など",
        ),
    ] {
        let mut builder = Manipulator::builder()
            .condition(Condition::with_vk2())
            .from_key(from)
            .to_key(to, to_modifiers);
        if !description.is_empty() {
            builder = builder.description(description);
        }
        manipulators.push(builder.build());
    }
    for (key_code, shell_command) in [
        (
            Key1,
            r#"osascript -e 'set currentVolume to input volume of (get volume settings)' -e 'set nextVolume to currentVolume - 10' -e 'if nextVolume < 0 then set nextVolume to 0' -e 'set volume input volume nextVolume'; open -g 'xbar://app.xbarapp.com/refreshPlugin?path=mic-slider.60s.sh'"#,
        ),
        (
            Key2,
            r#"osascript -e 'set currentVolume to input volume of (get volume settings)' -e 'set nextVolume to currentVolume + 10' -e 'if nextVolume > 100 then set nextVolume to 100' -e 'set volume input volume nextVolume'; open -g 'xbar://app.xbarapp.com/refreshPlugin?path=mic-slider.60s.sh'"#,
        ),
    ] {
        manipulators.push(
            Manipulator::builder()
                .description(if matches!(key_code, Key1) {
                    "マイク入力音量を下げる"
                } else {
                    "マイク入力音量を上げる"
                })
                .condition(Condition::with_vk2())
                .from_key_with_modifiers(key_code, FromModifier::Mandatory(vec![Ctrl]))
                .to_command(shell_command)
                .build(),
        );
    }

    for (description, from, to) in [
        ("Left", H, LeftArrow),
        ("Right", O, RightArrow),
        ("Down", N, DownArrow),
        ("Up", P, UpArrow),
        ("TopLeft", U, Key1),
        ("TopRight", I, Key2),
        ("BottomLeft", M, Key3),
        ("BottomRight", Comma, Key4),
    ] {
        manipulators.push(
            Manipulator::builder()
                .description(format!("[Magnet] {}", description))
                .condition(Condition::with_vk2())
                .from_key_with_modifiers(from, FromModifier::Mandatory(vec![Ctrl]))
                .to_key(to, Some(vec![Cmd, Ctrl, Opt, Shift]))
                .build(),
        );
    }
    manipulators.push(
        Manipulator::builder()
            .description("ウィンドウの最大化")
            .condition(Condition::with_vk2())
            .from_key_with_modifiers(B, FromModifier::Mandatory(vec![Ctrl]))
            .to_key(M, Some(vec![Ctrl, Opt, Cmd, Shift]))
            .build(),
    );

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
        // (H, "Open notification center"),
        (I, "open -a 'Claude.app'"),
        (J, "open -a 'Firefox.app'"),
        (K, "open -a 'iTerm.app'"),
        (L, "open -a 'Alfred 5.app'"),
        (M, "open -a 'Dynalist.app'"),
        (N, "open -a 'Notion.app'"),
        (O, "open -a \"Google Chrome\""),
        (P, "open -a '1Password.app'"),
        // (Q, "Ctrl+Cmd+Q - lock screen"),
        (R, "open -a 'Calculator.app'"),
        // (S, "Ctrl+Tab"),
        (T, "open -a 'Finder.app'"),
        (U, "open -a 'Microsoft To Do.app'"),
        (V, "open -a 'DeepL.app'"),
        (W, "open -a 'ChatGPT.app'"),
        (X, "open -a 'Calendar.app'"),
        (Y, "open -a 'Preview.app'"),
        // (Z, "Cmd+Shift+4 - screenshot selection"),
        // (ReturnOrEnter, None),
        (Quote, "open -a 'App Store.app'"),
        // (NonUsPound, None), // ]
        (OpenBracket, "open -a 'Mail.app'"), // @
        (CloseBracket, "open -a 'Karabiner-EventViewer.app'"), // [
        (Comma, "open -a 'System Settings.app'"),
        (Period, "open -a 'Slack.app'"),
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

    //
    // Virtual Key 3 - Bracket pairs (cursor ends up between the brackets)
    //
    manipulators.push(
        Manipulator::builder()
            .description("() を入力")
            .condition(Condition::with_vk3())
            .from_key(I)
            .to_key(Key8, Some(vec![Shift]))
            .to_key(Key9, Some(vec![Shift]))
            .to_key(LeftArrow, None)
            .build(),
    );
    manipulators.push(
        Manipulator::builder()
            .description("[] を入力")
            .condition(Condition::with_vk3())
            .condition(Condition::with_japanese_input())
            .from_key(O)
            .to_key(JapaneseEisuu, None)
            .to_key(CloseBracket, None)
            .to_key(NonUsPound, None)
            .to_key(JapaneseKana, None)
            .to_key(LeftArrow, None)
            .build(),
    );
    manipulators.push(
        Manipulator::builder()
            .description("[] を入力")
            .condition(Condition::with_vk3())
            .from_key(O)
            .to_key(CloseBracket, None)
            .to_key(NonUsPound, None)
            .to_key(LeftArrow, None)
            .build(),
    );
    manipulators.push(
        Manipulator::builder()
            .description("「」 を入力")
            .condition(Condition::with_vk3())
            .from_key(P)
            .to_key(CloseBracket, None)
            .to_key(NonUsPound, None)
            .to_key(LeftArrow, None)
            .build(),
    );
    for (from, description) in [(Q, "Play/Pause"), (W, "Reset"), (E, "Show")] {
        manipulators.push(
            Manipulator::builder()
                .description(format!("[Super Easy Timer] {}", description))
                .condition(Condition::with_vk3())
                .from_key(from.clone())
                .to_key(from, Some(vec![Cmd, Opt, Shift]))
                .build(),
        );
    }

    manipulators.extend(vec![
        Manipulator::builder()
            .description("; を入力")
            .from_key_with_modifiers(Semicolon, FromModifier::Mandatory(vec![Ctrl]))
            .to_key(Semicolon, None)
            .build(),
        Manipulator::builder()
            .description("Enter を入力")
            .from_key(Semicolon)
            .to_key(ReturnOrEnter, None)
            .build(),
        Manipulator::builder()
            .description("' を入力")
            .from_key_with_modifiers(Quote, FromModifier::Mandatory(vec![Ctrl]))
            .to_key(Key7, Some(vec![Shift]))
            .build(),
        Manipulator::builder()
            .description("CapsLock を無効化")
            .from_key_with_modifiers(CapsLock, FromModifier::Optional(vec![Any]))
            .to_key(VkNone, None)
            .build(),
    ]);

    manipulators.push(
        Manipulator::builder()
            .description("Enable shingeta layout")
            .condition(Condition::without_virtual_key(VirtualKey::ShingetaMode))
            .from_key(F12)
            .to_variable(SetVariable {
                name: VirtualKey::ShingetaMode,
                value: 1,
            })
            .to_command(SHINGETA_MODE_ON_COMMAND)
            .build(),
    );

    manipulators.push(
        Manipulator::builder()
            .description("新下駄モードを無効化")
            .condition(Condition::with_shingeta_mode())
            .from_key(F12)
            .to_variable(SetVariable {
                name: VirtualKey::ShingetaMode,
                value: 0,
            })
            .to_command(SHINGETA_MODE_OFF_COMMAND)
            .build(),
    );

    manipulators.push(
        Manipulator::builder()
            .description("英数 & 新下駄モードOFF")
            .from_key(JapaneseEisuu)
            .to_key(JapaneseEisuu, None)
            .to_variable(SetVariable {
                name: VirtualKey::ShingetaMode,
                value: 0,
            })
            .to_command(SHINGETA_MODE_OFF_COMMAND)
            .build(),
    );

    manipulators.push(
        Manipulator::builder()
            .description("かな & 新下駄モードON")
            .from_key(JapaneseKana)
            .to_key(JapaneseKana, None)
            .to_variable(SetVariable {
                name: VirtualKey::ShingetaMode,
                value: 1,
            })
            .to_command(SHINGETA_MODE_ON_COMMAND)
            .build(),
    );

    manipulators
}
