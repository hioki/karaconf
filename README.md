# Karaconf

This is a tool for writing the configuration file (`$HOME/.config/karabiner/karabiner.json`) for [Karabiner-Elements](https://karabiner-elements.pqrs.org/) in Rust.

Managing the numerous Karabiner settings by directly editing `karabiner.json` is cumbersome, so I created a tool to generate and update the configuration in Rust.

## Usage

```sh
# Lint the rules, update the Karabiner configuration, and generate cheatsheet.html
cargo run

# Lint only (no files are written; exits with code 1 if problems are found)
cargo run -- --check
```

## What gets written to karabiner.json

`cargo run` only replaces the first profile's `complex_modifications.rules` and
sets `basic.simultaneous_threshold_milliseconds`; every other setting (devices,
simple modifications, fields added by newer Karabiner-Elements, ...) is kept as is.
Before overwriting, the previous file is saved to
`~/.config/karabiner/karabiner.json.karaconf-bak`. If `karabiner.json` cannot be
parsed, karaconf aborts and leaves the file untouched.

## Lint

Karabiner-Elements evaluates manipulators in order and the first match wins.
The built-in lint reports manipulators that can never fire because an earlier
manipulator matches a superset of their key events (same `from` key, subset of
conditions, covering modifiers), as well as fully duplicated definitions.

## Cheatsheet

`cargo run` also generates `cheatsheet.html`: per-layer (VK1-VK4) JIS keyboard
diagrams with unused keys dimmed, per-app rule tables, and the shingeta layout
rendered as kana (letter-key outputs are interpreted as romaji).

## App rules over Screen Sharing

Keys typed into Screen Sharing are handled by Karabiner-Elements on the local
Mac only (the remote Mac's Karabiner never sees them), and there the frontmost
app is Screen Sharing itself. So karaconf also generates a copy of every app
rule for "Screen Sharing is frontmost and the app is frontmost on the remote
Mac", checking the Karabiner variable `remote_frontmost_app`.

`screen_sharing/sync_frontmost_app.sh` keeps that variable up to date: it runs
`screen_sharing/frontmost_app.swift` on the remote Mac over ssh (the remote Mac
needs Swift from the Xcode Command Line Tools) and passes each frontmost app
change to `karabiner_cli`. Install it as a LaunchAgent on the Mac that runs
Screen Sharing:

```sh
# HOST defaults to h-ms, REMOTE_HOST in src/rule_sets/apps/screen_sharing.rs
screen_sharing/install.sh [HOST]

# Uninstall
launchctl bootout gui/$(id -u)/com.github.hioki.karaconf.remote-frontmost-app
rm ~/Library/LaunchAgents/com.github.hioki.karaconf.remote-frontmost-app.plist
```

The log goes to `~/Library/Logs/karaconf-remote-frontmost-app.log`.
