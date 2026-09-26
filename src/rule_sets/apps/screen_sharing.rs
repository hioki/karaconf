use crate::{
    display,
    karabiner_data::{BundleIdentifier::ScreenSharing, Condition, Manipulator},
    rule_sets::common::VK2_SHELL_COMMANDS,
};

/// The Mac controlled through Screen Sharing (an ssh host alias). Karabiner
/// runs `shell_command` on the local Mac, and the remote Mac's Karabiner never
/// sees Screen Sharing input, so VK2 commands are forwarded over ssh instead.
const REMOTE_HOST: &str = "h-ms";

pub fn manipulators() -> Vec<Manipulator> {
    VK2_SHELL_COMMANDS
        .iter()
        // Launching Screen Sharing on the remote Mac is pointless; this falls
        // through to the local command, which keeps Screen Sharing frontmost.
        .filter(|(_, command)| !command.contains("Screen Sharing.app"))
        .map(|(key_code, command)| {
            Manipulator::builder()
                .description(format!(
                    "[{}] {}",
                    REMOTE_HOST,
                    display::command_summary(command)
                ))
                .conditions(vec![
                    Condition::on_app(ScreenSharing),
                    Condition::with_vk2(),
                ])
                .from_key(key_code.clone())
                .to_command(&remote_command(command))
                .build()
        })
        .collect()
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
