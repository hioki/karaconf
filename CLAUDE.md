# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust tool for generating Karabiner-Elements configuration files. It programmatically creates the complex `karabiner.json` configuration by organizing keyboard rules into separate modules, making it easier to manage large numbers of keyboard customizations.

## Common Commands

### Build and Run
```bash
cargo run
```
This builds the project and executes the main binary, which generates the Karabiner configuration files.

### Standard Rust Development
```bash
cargo build        # Build the project
cargo check        # Check for compilation errors
cargo test         # Run tests
cargo fmt          # Format code
cargo clippy       # Run lints
```

## Architecture

### Core Components

- **`src/main.rs`**: Entry point that orchestrates the configuration generation process:
  1. Collects manipulators from all rule sets
  2. Writes `custom.json` to the project root
  3. Copies the file to `~/.config/karabiner/assets/complex_modifications/`
  4. Updates the main `karabiner.json` file

- **`src/karabiner_data.rs`**: Core data structures and types that model the Karabiner-Elements JSON schema:
  - `Manipulator`: Represents a single keyboard mapping rule
  - `ManipulatorBuilder`: Fluent builder pattern for creating manipulators
  - `Condition`: Defines when rules should apply (app-specific, virtual key states)
  - `KeyCode`, `ModifierKey`, `VirtualKey`: Enums for keyboard inputs
  - `BundleIdentifier`: Enum for specific applications

- **`src/rule_sets/`**: Modular rule definitions organized by application or functionality:
  - Each module exports a `manipulators()` function returning `Vec<Manipulator>`
  - Applications: `vscode.rs`, `iterm2.rs`, `slack.rs`, `notion.rs`, etc.
  - Virtual key layers: `vk1.rs`, `vk2.rs`, `vk3.rs` for complex key combinations
  - Special functions: `open_apps.rs`, `capslock.rs`, `semicolon.rs`, `singlequote.rs`

### Key Design Patterns

1. **Builder Pattern**: `ManipulatorBuilder` provides a fluent API for constructing keyboard rules
2. **Virtual Key Layers**: Uses virtual keys (VK1-VK4) to create complex, stateful key combinations
3. **Application-Specific Rules**: Uses bundle identifiers to apply rules only in specific applications
4. **Modular Organization**: Each rule set is isolated in its own module for maintainability

### Virtual Key System

The project uses a virtual key system for complex key combinations:
- Virtual keys (VK1-VK4) act as toggles that can be enabled/disabled
- Rules can be conditioned on virtual key states using `Condition::with_vk1()` etc.
- Enables creation of vim-like modes and complex key sequences

## File Generation Process

1. All rule sets are collected and flattened into a single `Vec<Manipulator>`
2. A `ComplexModifications` struct is created with all rules
3. The configuration is serialized to `custom.json` in the project root
4. The file is copied to Karabiner's assets directory
5. The main `karabiner.json` is updated with the new rules

## Dependencies

- `serde`: JSON serialization/deserialization
- `serde_json`: JSON handling
- `anyhow`: Error handling