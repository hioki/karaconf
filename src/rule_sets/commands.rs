use crate::karabiner_data::{Condition, KeyCode::*, Manipulator};

pub fn manipulators() -> Vec<Manipulator> {
    vec![
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
        (H, "/Applications/Google\\ Chrome.app/Contents/MacOS/Google\\ Chrome --profile-directory=\"Default\""),
        (I, "open -a 'CLion.app'"),
        (J, "/Applications/Google\\ Chrome.app/Contents/MacOS/Google\\ Chrome --profile-directory=\"Profile 1\""),
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
    ]
    .into_iter()
    .map(|(key_code, shell_command)| {
        Manipulator::builder()
            .condition(Condition::with_vk2())
            .from_key(key_code)
            .to_command(shell_command)
            .build()
    })
    .collect()
}
