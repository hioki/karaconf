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

## Lint

Karabiner-Elements evaluates manipulators in order and the first match wins.
The built-in lint reports manipulators that can never fire because an earlier
manipulator matches a superset of their key events (same `from` key, subset of
conditions, covering modifiers), as well as fully duplicated definitions.

## Cheatsheet

`cargo run` also generates `cheatsheet.html`: per-layer (VK1-VK4) JIS keyboard
diagrams with unused keys dimmed, per-app rule tables, and the shingeta layout
rendered as kana (letter-key outputs are interpreted as romaji).
