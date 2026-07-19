//! Human-readable labels for keyboard cheatsheet and lint messages.
//! Key legends follow the JIS layout (keyboard_type_v2 = "jis").

use crate::karabiner_data::{
    Condition, From, FromModifier, KeyCode, Manipulator, ModifierKey, PointingButton, To,
    ToIfAlone, VirtualKey,
};

pub fn key_label(key: &KeyCode) -> String {
    use KeyCode::*;
    let s = match key {
        Key1 => "1",
        Key2 => "2",
        Key3 => "3",
        Key4 => "4",
        Key5 => "5",
        Key6 => "6",
        Key7 => "7",
        Key8 => "8",
        Key9 => "9",
        Key0 => "0",
        Hyphen => "-",
        EqualSign => "^",
        International3 => "¥",
        OpenBracket => "@",
        CloseBracket => "[",
        Semicolon => ";",
        Quote => ":",
        Backslash | NonUsPound => "]",
        International1 => "_",
        Comma => ",",
        Period => ".",
        Slash => "/",
        GraveAccentAndTilde => "`",
        ReturnOrEnter => "⏎",
        Escape => "Esc",
        DeleteOrBackspace => "⌫",
        DeleteForward => "⌦",
        Tab => "Tab",
        Spacebar => "Space",
        CapsLock => "Caps",
        LeftArrow => "←",
        RightArrow => "→",
        UpArrow => "↑",
        DownArrow => "↓",
        Home => "Home",
        End => "End",
        PageUp => "PgUp",
        PageDown => "PgDn",
        JapaneseEisuu | Lang2 => "英数",
        JapaneseKana | Lang1 => "かな",
        International2 => "Intl2",
        International4 => "変換",
        International5 => "無変換",
        RightGui => "右⌘",
        LeftGui => "左⌘",
        KeypadPlus => "+",
        KeypadHyphen => "-",
        VolumeIncrement => "音量+",
        VolumeDecrement => "音量-",
        Rewind => "⏪",
        PlayOrPause => "⏯",
        Fastforward => "⏩",
        DisplayBrightnessIncrement => "輝度+",
        DisplayBrightnessDecrement => "輝度-",
        VkNone => "∅",
        MissionControl => "Mission Control",
        Launchpad => "Launchpad",
        _ => return format!("{:?}", key),
    };
    s.to_string()
}

pub fn modifier_symbol(modifier: &ModifierKey) -> &'static str {
    match modifier {
        ModifierKey::Any => "任意+",
        ModifierKey::Ctrl => "⌃",
        ModifierKey::Shift => "⇧",
        ModifierKey::Opt => "⌥",
        ModifierKey::Cmd => "⌘",
    }
}

pub fn mods_prefix(modifiers: &[ModifierKey]) -> String {
    modifiers.iter().map(modifier_symbol).collect()
}

pub fn virtual_key_label(virtual_key: &VirtualKey) -> &'static str {
    match virtual_key {
        VirtualKey::Vk1 => "VK1",
        VirtualKey::Vk2 => "VK2",
        VirtualKey::Vk3 => "VK3",
        VirtualKey::Vk4 => "VK4",
        VirtualKey::ShingetaMode => "新下駄",
    }
}

/// Label for the `from` part, e.g. "⌘W", "K+1". Optional modifiers are omitted.
pub fn from_label(from: &From) -> String {
    match from {
        From::Single {
            key_code,
            modifiers,
        } => {
            let base = key_label(key_code);
            match modifiers {
                Some(FromModifier::Mandatory(mods)) => format!("{}{}", mods_prefix(mods), base),
                _ => base,
            }
        }
        From::Simultaneous { simultaneous, .. } => simultaneous
            .iter()
            .map(|s| key_label(&s.key_code))
            .collect::<Vec<_>>()
            .join("+"),
    }
}

/// Description used for the generated Rule, e.g. "VSCode: VK4+A -> execute command".
/// The effect part is the manual description when present, otherwise the
/// mechanical output summary.
pub fn rule_description(manipulator: &Manipulator) -> String {
    let effect = manipulator
        .description
        .clone()
        .unwrap_or_else(|| to_summary(manipulator));
    format!("{} -> {}", context_label(manipulator), effect)
}

/// "VSCode: VK4+A" style context: conditions followed by the `from` key.
fn context_label(manipulator: &Manipulator) -> String {
    let mut prefix = String::new();
    for condition in manipulator.conditions.iter().flatten() {
        match condition {
            Condition::OnApplication {
                bundle_identifiers, ..
            } => {
                prefix.push_str(
                    &bundle_identifiers
                        .iter()
                        .map(|b| format!("{:?}", b))
                        .collect::<Vec<_>>()
                        .join("/"),
                );
                prefix.push_str(": ");
            }
            Condition::WithVirtualKey { name, value, .. } => {
                if *value == 1 {
                    match name {
                        VirtualKey::ShingetaMode => prefix.push_str("新下駄: "),
                        _ => {
                            prefix.push_str(virtual_key_label(name));
                            prefix.push('+');
                        }
                    }
                } else {
                    prefix.push_str(virtual_key_label(name));
                    prefix.push_str("オフ: ");
                }
            }
            Condition::InputSource { input_sources, .. } => {
                let languages = input_sources
                    .iter()
                    .map(|s| s.language.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                prefix.push_str(&format!("({}) ", languages));
            }
        }
    }
    format!("{}{}", prefix, from_label(&manipulator.from))
}

/// One-line summary of everything a manipulator emits.
pub fn to_summary(manipulator: &Manipulator) -> String {
    let mut summary = manipulator
        .to
        .iter()
        .map(to_part)
        .collect::<Vec<_>>()
        .join(" ");
    if let Some(alone) = &manipulator.to_if_alone {
        let alone_desc = alone
            .iter()
            .filter_map(|a| match a {
                ToIfAlone::Key { key_code } => Some(key_label(key_code)),
                ToIfAlone::Variable { .. } => None,
            })
            .collect::<Vec<_>>()
            .join(" ");
        if !alone_desc.is_empty() {
            if summary.is_empty() {
                summary = format!("単独: {}", alone_desc);
            } else {
                summary = format!("{} ／ 単独: {}", summary, alone_desc);
            }
        }
    }
    summary
}

fn to_part(to: &To) -> String {
    match to {
        To::Key {
            key_code,
            modifiers,
            ..
        } => format!(
            "{}{}",
            modifiers.as_deref().map(mods_prefix).unwrap_or_default(),
            key_label(key_code)
        ),
        To::Variable { set_variable } => format!(
            "[{}={}]",
            virtual_key_label(&set_variable.name),
            set_variable.value
        ),
        To::Command { shell_command } => command_summary(shell_command),
        To::Mouse { mouse_key } => {
            let mut parts = Vec::new();
            if let Some(x) = mouse_key.x {
                parts.push(format!("マウス{}", if x < 0 { "←" } else { "→" }));
            }
            if let Some(y) = mouse_key.y {
                parts.push(format!("マウス{}", if y < 0 { "↑" } else { "↓" }));
            }
            if let Some(v) = mouse_key.vertical_wheel {
                parts.push(format!("ホイール{}", if v < 0 { "↑" } else { "↓" }));
            }
            if let Some(h) = mouse_key.horizontal_wheel {
                parts.push(format!("横ホイール{}", if h < 0 { "←" } else { "→" }));
            }
            parts.join(" ")
        }
        To::Click { pointing_button } => match pointing_button {
            PointingButton::Button1 => "左クリック".to_string(),
            PointingButton::Button2 => "右クリック".to_string(),
        },
    }
}

fn command_summary(shell_command: &str) -> String {
    if let Some(app) = launched_app_name(shell_command) {
        return format!("{} 起動", app);
    }
    if shell_command.contains("shingeta_mode") {
        return "新下駄モード記録".to_string();
    }
    if shell_command.contains("osascript") && shell_command.contains("volume") {
        return "マイク音量調整".to_string();
    }
    if shell_command.contains("keystroke") && shell_command.contains("date +%Y%m%d%H%M%S") {
        return "タイムスタンプを入力".to_string();
    }
    if shell_command.contains("keystroke") && shell_command.contains("date +") {
        return "今日の日付を入力".to_string();
    }
    if shell_command.contains("pbpaste") {
        return "クリップボード整形".to_string();
    }
    if let Some(rest) = shell_command.strip_prefix("open '")
        && let Some(url) = rest.split('\'').next()
    {
        let host = url
            .strip_prefix("https://")
            .or_else(|| url.strip_prefix("http://"))
            .and_then(|u| u.split('/').next())
            .unwrap_or(url);
        return format!("{} を開く", host);
    }
    let short: String = shell_command.chars().take(28).collect();
    if short.len() < shell_command.len() {
        format!("$ {}…", short)
    } else {
        format!("$ {}", short)
    }
}

fn launched_app_name(shell_command: &str) -> Option<String> {
    let rest = shell_command.split("open -a ").nth(1)?;
    let rest = rest.trim_start();
    let quote = rest.chars().next()?;
    let name = if quote == '\'' || quote == '"' {
        rest[1..].split(quote).next()?
    } else {
        rest.split_whitespace().next()?
    };
    Some(name.trim_end_matches(".app").to_string())
}
