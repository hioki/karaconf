use crate::{
    display,
    karabiner_data::{
        BundleIdentifier::ScreenSharing, Condition, FromModifier, Manipulator, ModifierKey::*,
    },
    rule_sets::common::{
        MAC_STUDIO_MAGNET_MODIFIERS, MAC_STUDIO_MAGNET_SHORTCUTS, VK2_SHELL_COMMANDS,
    },
};

/// The Mac controlled through Screen Sharing (an ssh host alias). Karabiner
/// runs `shell_command` on the local Mac, and the remote Mac's Karabiner never
/// sees Screen Sharing input, so VK2 commands are forwarded over ssh instead.
const REMOTE_HOST: &str = "h-ms";

/// Hides Screen Sharing on the local Mac so the previously used app gets focus,
/// like ⌘H. Matched by bundle id because the process name is localized.
const HIDE_SCREEN_SHARING_COMMAND: &str = r#"osascript -e 'tell application "System Events" to set visible of (first process whose bundle identifier is "com.apple.ScreenSharing") to false'"#;

pub fn manipulators() -> Vec<Manipulator> {
    VK2_SHELL_COMMANDS
        .iter()
        .map(|(key_code, command)| {
            let builder = Manipulator::builder()
                .conditions(vec![
                    Condition::on_app(ScreenSharing),
                    Condition::with_vk2(),
                ])
                .from_key(key_code.clone());
            if command.contains("Screen Sharing.app") {
                // The same key launches Screen Sharing elsewhere, so it
                // toggles in and out of Screen Sharing.
                builder
                    .description("Screen Sharing を隠す")
                    .to_command(HIDE_SCREEN_SHARING_COMMAND)
            } else {
                builder
                    .description(format!(
                        "[{}] {}",
                        REMOTE_HOST,
                        display::command_summary(command)
                    ))
                    .to_command(&remote_command(command))
            }
            .build()
        })
        .chain(magnet_manipulators())
        .collect()
}

/// Screen Sharing forwards the keys Karabiner sends, so VK2+⌃ sends the remote
/// Mac's Magnet shortcuts. The local Magnet must not use the same shortcuts,
/// or it takes them before Screen Sharing does.
fn magnet_manipulators() -> impl Iterator<Item = Manipulator> {
    MAC_STUDIO_MAGNET_SHORTCUTS
        .iter()
        .map(|(description, from, to)| {
            Manipulator::builder()
                .description(format!("[{}] [Magnet] {}", REMOTE_HOST, description))
                .conditions(vec![
                    Condition::on_app(ScreenSharing),
                    Condition::with_vk2(),
                ])
                .from_key_with_modifiers(from.clone(), FromModifier::Mandatory(vec![Ctrl]))
                .to_key(to.clone(), Some(MAC_STUDIO_MAGNET_MODIFIERS.to_vec()))
                .build()
        })
}

/// - BatchMode: fail instead of waiting for a password prompt that nobody can
///   answer from Karabiner.
/// - Control*: reuse one connection for 10 minutes after the last use, so a key
///   press skips the handshake and 1Password signing (~0.5s per connection).
///   Only these commands use this socket; interactive ssh is unaffected.
/// - ServerAlive*: drop a dead master (e.g. after sleep or a network change)
///   within ~10s instead of hanging on it.
const SSH_OPTIONS: &[&str] = &[
    "BatchMode=yes",
    "ConnectTimeout=5",
    "ControlMaster=auto",
    "ControlPath=~/.ssh/karaconf-%C",
    "ControlPersist=10m",
    "ServerAliveInterval=5",
    "ServerAliveCountMax=2",
];

fn remote_command(command: &str) -> String {
    let options: String = SSH_OPTIONS
        .iter()
        .map(|option| format!("-o {} ", option))
        .collect();
    format!("ssh {}{} {}", options, REMOTE_HOST, shell_quote(command))
}

fn shell_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', r"'\''"))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The remote shell must receive each command exactly as written.
    #[test]
    fn shell_quote_round_trips_through_sh() {
        for (_, command) in VK2_SHELL_COMMANDS {
            let output = std::process::Command::new("/bin/sh")
                .arg("-c")
                .arg(format!("printf %s {}", shell_quote(command)))
                .output()
                .unwrap();
            assert_eq!(String::from_utf8(output.stdout).unwrap(), *command);
        }
    }
}
