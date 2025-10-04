pub mod capslock;
pub mod chatgpt;
pub mod commands;
pub mod dynalist;
pub mod google_chrome;
pub mod iterm2;
pub mod notion;
pub mod semicolon;
pub mod singlequote;
pub mod slack;
pub mod virtual_key;
pub mod vk1;
pub mod vk2;
pub mod vk3;
pub mod vscode;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::karabiner_data::*;

    #[test]
    fn test_all_rule_sets_return_manipulators() {
        let rule_sets = vec![
            ("virtual_key", virtual_key::manipulators()),
            ("iterm2", iterm2::manipulators()),
            ("vscode", vscode::manipulators()),
            ("dynalist", dynalist::manipulators()),
            ("slack", slack::manipulators()),
            ("google_chrome", google_chrome::manipulators()),
            ("notion", notion::manipulators()),
            ("chatgpt", chatgpt::manipulators()),
            ("vk1", vk1::manipulators()),
            ("vk2", vk2::manipulators()),
            ("commands", commands::manipulators()),
            ("vk3", vk3::manipulators()),
            ("semicolon", semicolon::manipulators()),
            ("singlequote", singlequote::manipulators()),
            ("capslock", capslock::manipulators()),
        ];

        for (name, manipulators) in rule_sets {
            println!("Testing rule set: {}", name);
            assert!(
                !manipulators.is_empty(),
                "Rule set {} should return at least one manipulator",
                name
            );

            // Test that each manipulator has required fields
            for (i, manipulator) in manipulators.iter().enumerate() {
                assert!(
                    !matches!(manipulator.from.key_code, KeyCode::Escape),
                    "Rule set {} manipulator {} should have a proper from key (not default Escape)",
                    name,
                    i
                );
            }
        }
    }

    #[test]
    fn test_vscode_manipulators_have_app_conditions() {
        let vscode_manipulators = vscode::manipulators();

        for manipulator in vscode_manipulators {
            if let Some(conditions) = &manipulator.conditions {
                let has_app_condition = conditions
                    .iter()
                    .any(|condition| matches!(condition, Condition::OnApplication { .. }));
                assert!(
                    has_app_condition,
                    "VSCode manipulator should have application condition"
                );
            }
        }
    }

    #[test]
    fn test_vk1_manipulators_have_virtual_key_conditions() {
        let vk1_manipulators = vk1::manipulators();

        for manipulator in vk1_manipulators {
            if let Some(conditions) = &manipulator.conditions {
                let has_vk1_condition = conditions.iter().any(|condition| {
                    matches!(
                        condition,
                        Condition::WithVirtualKey {
                            name: VirtualKey::Vk1,
                            ..
                        }
                    )
                });
                assert!(
                    has_vk1_condition,
                    "VK1 manipulator should have VK1 virtual key condition"
                );
            }
        }
    }

    #[test]
    fn test_manipulator_builder_type_safety() {
        // This should compile - proper usage
        let _manipulator = Manipulator::builder()
            .condition(Condition::with_vk1())
            .from_key(KeyCode::A)
            .to_key(KeyCode::B, None)
            .build();

        // This should also compile - with modifiers
        let _manipulator = Manipulator::builder()
            .from_key_with_modifiers(KeyCode::A, FromModifier::Optional(vec![ModifierKey::Shift]))
            .to_key(KeyCode::B, Some(vec![ModifierKey::Cmd]))
            .build();

        // The following would not compile due to type safety:
        // let _manipulator = Manipulator::builder()
        //     .condition(Condition::with_vk1())
        //     .to_key(KeyCode::B, None)
        //     .build(); // Error: missing from_key
    }
}
