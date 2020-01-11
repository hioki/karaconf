mod bundle_identifier;
mod condition;
mod config;
mod from;
mod key_code;
mod manipulator;
mod modifier_key;
mod mouse_key;
mod rule;
mod set_variable;
mod to;
mod value;
mod virtual_key;

use serde_json;

use bundle_identifier::BundleIdentifier;
use condition::Condition;
use config::Config;
use from::{From, FromModifier};
use key_code::*;
use manipulator::{Manipulator, ManipulatorType, ToAfterKeyUp, ToIfAlone};
use modifier_key::ModifierKey;
use mouse_key::MouseKey;
use rule::Rule;
use set_variable::SetVariable;
use to::{PointingButton, To};
use value::Value;
use virtual_key::VirtualKey;

fn main() {
    let rules = vec![
        Rule {
            description: "Virtual Keys",
            manipulators: vec![
                (KeyCode::Lang1, VirtualKey::Vk1, Some(KeyCode::JapaneseKana)),
                (
                    KeyCode::International4,
                    VirtualKey::Vk1,
                    Some(KeyCode::JapaneseKana),
                ),
                (
                    KeyCode::Lang2,
                    VirtualKey::Vk2,
                    Some(KeyCode::JapaneseEisuu),
                ),
                (
                    KeyCode::International5,
                    VirtualKey::Vk2,
                    Some(KeyCode::JapaneseEisuu),
                ),
                (KeyCode::RightGui, VirtualKey::Vk3, None),
                (KeyCode::International2, VirtualKey::Vk3, None),
                (KeyCode::Tab, VirtualKey::Vk4, Some(KeyCode::Tab)),
            ]
            .into_iter()
            .map(|(key_code, virtual_key, to_if_alone)| Manipulator {
                r#type: ManipulatorType::default(),
                conditions: None,
                from: From {
                    key_code,
                    modifiers: Some(FromModifier::Optional(vec![ModifierKey::Any])),
                },
                to: vec![To {
                    set_variable: Some(SetVariable {
                        name: virtual_key.clone(),
                        value: Value::On.value(),
                    }),
                    key_code: None,
                    modifiers: None,
                    mouse_key: None,
                    pointing_button: None,
                    shell_command: None,
                }],
                to_after_key_up: Some(vec![ToAfterKeyUp {
                    set_variable: SetVariable {
                        name: virtual_key,
                        value: Value::Off.value(),
                    },
                }]),
                to_if_alone: to_if_alone.map(|key_code| vec![ToIfAlone { key_code }]),
            })
            .collect::<Vec<Manipulator>>(),
        },
        Rule {
            description: "Substitute TMUX prefix with VK4 on iTerm2",
            manipulators: vec![
                KeyCode::C,
                KeyCode::V,
                KeyCode::H,
                KeyCode::J,
                KeyCode::K,
                KeyCode::L,
                KeyCode::N,
                KeyCode::P,
            ]
            .into_iter()
            .map(|key_code| Manipulator {
                r#type: ManipulatorType::default(),
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::ITerm2),
                    Condition::with_virtual_key(VirtualKey::Vk4),
                ]),
                from: From {
                    key_code: key_code.clone(),
                    modifiers: None,
                },
                to: vec![
                    tmux_prefix(),
                    To {
                        set_variable: None,
                        key_code: Some(key_code),
                        modifiers: Some(vec![ModifierKey::Control]),
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    },
                ],
                to_after_key_up: None,
                to_if_alone: None,
            })
            .collect::<Vec<Manipulator>>(),
        },
        Rule {
            description: "VK4 on VSCode",
            manipulators: vec![
                KeyCode::Key1,
                KeyCode::Key2,
                KeyCode::Key3,
                KeyCode::Key4,
                KeyCode::A,
                KeyCode::H,
                KeyCode::J,
                KeyCode::E,
                KeyCode::L,
                KeyCode::S,
                KeyCode::P,
                KeyCode::O,
                KeyCode::C,
                KeyCode::M,
                KeyCode::K,
                KeyCode::R,
                KeyCode::X,
                KeyCode::I,
                KeyCode::Y,
                KeyCode::CloseBracket,
                KeyCode::NonUsPound,
            ]
            .into_iter()
            .map(|key_code| Manipulator {
                r#type: ManipulatorType::default(),
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::VSCode),
                    Condition::with_virtual_key(VirtualKey::Vk4),
                ]),
                from: From {
                    key_code: key_code.clone(),
                    modifiers: None,
                },
                to: vec![To {
                    set_variable: None,
                    key_code: Some(key_code),
                    modifiers: Some(vec![
                        ModifierKey::Control,
                        ModifierKey::Shift,
                        ModifierKey::Option,
                        ModifierKey::Command,
                    ]),
                    mouse_key: None,
                    pointing_button: None,
                    shell_command: None,
                }],
                to_after_key_up: None,
                to_if_alone: None,
            })
            .collect::<Vec<Manipulator>>(),
        },
        Rule {
            description: "[Terminal] o/p -> control+t control+p / control+t control+n",
            manipulators: vec![
                Manipulator {
                    conditions: Some(vec![
                        Condition::on_app(BundleIdentifier::ITerm2),
                        Condition::with_virtual_key(VirtualKey::Vk1),
                    ]),
                    from: From {
                        key_code: KeyCode::O,
                        modifiers: None,
                    },
                    to: vec![
                        tmux_prefix(),
                        To {
                            key_code: Some(KeyCode::P),
                            modifiers: Some(vec![ModifierKey::Control]),
                            set_variable: None,
                            mouse_key: None,
                            pointing_button: None,
                            shell_command: None,
                        },
                    ],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    conditions: Some(vec![
                        Condition::on_app(BundleIdentifier::ITerm2),
                        Condition::with_virtual_key(VirtualKey::Vk1),
                    ]),
                    from: From {
                        key_code: KeyCode::P,
                        modifiers: None,
                    },
                    to: vec![
                        tmux_prefix(),
                        To {
                            key_code: Some(KeyCode::N),
                            modifiers: Some(vec![ModifierKey::Control]),
                            set_variable: None,
                            mouse_key: None,
                            pointing_button: None,
                            shell_command: None,
                        },
                    ],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
            ],
        },
        Rule {
            description: "[Terminal] VK2 + a/s -> control+t control+p / control+t control+n",
            manipulators: vec![
                Manipulator {
                    conditions: Some(vec![
                        Condition::on_app(BundleIdentifier::ITerm2),
                        Condition::with_virtual_key(VirtualKey::Vk2),
                    ]),
                    from: From {
                        key_code: KeyCode::A,
                        modifiers: None,
                    },
                    to: vec![
                        tmux_prefix(),
                        To {
                            key_code: Some(KeyCode::P),
                            modifiers: Some(vec![ModifierKey::Control]),
                            set_variable: None,
                            mouse_key: None,
                            pointing_button: None,
                            shell_command: None,
                        },
                    ],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    conditions: Some(vec![
                        Condition::on_app(BundleIdentifier::ITerm2),
                        Condition::with_virtual_key(VirtualKey::Vk2),
                    ]),
                    from: From {
                        key_code: KeyCode::S,
                        modifiers: None,
                    },
                    to: vec![
                        tmux_prefix(),
                        To {
                            key_code: Some(KeyCode::N),
                            modifiers: Some(vec![ModifierKey::Control]),
                            set_variable: None,
                            mouse_key: None,
                            pointing_button: None,
                            shell_command: None,
                        },
                    ],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
            ],
        },
        Rule {
            description: "[Terminal] VK2 + h -> backspace",
            manipulators: vec![Manipulator {
                conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk2)]),
                from: From {
                    key_code: KeyCode::H,
                    modifiers: None,
                },
                to: vec![To {
                    key_code: Some(KeyCode::DeleteOrBackspace),
                    modifiers: None,
                    set_variable: None,
                    mouse_key: None,
                    pointing_button: None,
                    shell_command: None,
                }],
                r#type: ManipulatorType::default(),
                to_after_key_up: None,
                to_if_alone: None,
            }],
        },
        Rule {
            description: "[Terminal] z/y -> copy",
            manipulators: vec![
                Manipulator {
                    conditions: Some(vec![
                        Condition::on_app(BundleIdentifier::ITerm2),
                        Condition::with_virtual_key(VirtualKey::Vk1),
                    ]),
                    from: From {
                        key_code: KeyCode::Z,
                        modifiers: None,
                    },
                    to: vec![
                        tmux_prefix(),
                        To {
                            key_code: Some(KeyCode::CloseBracket),
                            modifiers: Some(vec![ModifierKey::Control]),
                            set_variable: None,
                            mouse_key: None,
                            pointing_button: None,
                            shell_command: None,
                        },
                    ],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    conditions: Some(vec![
                        Condition::on_app(BundleIdentifier::ITerm2),
                        Condition::with_virtual_key(VirtualKey::Vk1),
                    ]),
                    from: From {
                        key_code: KeyCode::Y,
                        modifiers: None,
                    },
                    to: vec![
                        To {
                            key_code: Some(KeyCode::ReturnOrEnter),
                            modifiers: None,
                            set_variable: None,
                            mouse_key: None,
                            pointing_button: None,
                            shell_command: None,
                        },
                        tmux_prefix(),
                        To {
                            key_code: Some(KeyCode::M),
                            modifiers: Some(vec![ModifierKey::Control]),
                            set_variable: None,
                            mouse_key: None,
                            pointing_button: None,
                            shell_command: None,
                        },
                    ],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
            ],
        },
        Rule {
            description: "[Terminal] u/i -> shift+0 / shift+4",
            manipulators: vec![
                Manipulator {
                    conditions: Some(vec![
                        Condition::on_app(BundleIdentifier::ITerm2),
                        Condition::with_virtual_key(VirtualKey::Vk1),
                    ]),
                    from: From {
                        key_code: KeyCode::U,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::Key0),
                        modifiers: Some(vec![ModifierKey::Shift]),
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    conditions: Some(vec![
                        Condition::on_app(BundleIdentifier::ITerm2),
                        Condition::with_virtual_key(VirtualKey::Vk1),
                    ]),
                    from: From {
                        key_code: KeyCode::I,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::Key4),
                        modifiers: Some(vec![ModifierKey::Shift]),
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
            ],
        },
        Rule {
            description: "[VK1] h/j/k/l -> cursor move",
            manipulators: vec![
                (KeyCode::H, KeyCode::LeftArrow),
                (KeyCode::J, KeyCode::DownArrow),
                (KeyCode::K, KeyCode::UpArrow),
                (KeyCode::L, KeyCode::RightArrow),
            ]
            .into_iter()
            .map(|(from, to)| {
                build_manipulator(
                    VirtualKey::Vk1,
                    from,
                    Some(FromModifier::Optional(vec![ModifierKey::Any])),
                    to,
                    None,
                )
            })
            .collect::<Vec<Manipulator>>(),
        },
        Rule {
            description: "[VK1] f -> escape",
            manipulators: vec![build_manipulator(
                VirtualKey::Vk1,
                KeyCode::F,
                Some(FromModifier::Optional(vec![ModifierKey::Any])),
                KeyCode::Escape,
                None,
            )],
        },
        Rule {
            description: "[VK1] s/d -> shift+control+j/shift+control+; (Google Japanese Input)",
            manipulators: vec![
                build_manipulator(
                    VirtualKey::Vk1,
                    KeyCode::S,
                    None,
                    KeyCode::J,
                    Some(vec![ModifierKey::Shift, ModifierKey::Control]),
                ),
                build_manipulator(
                    VirtualKey::Vk1,
                    KeyCode::D,
                    None,
                    KeyCode::Semicolon,
                    Some(vec![ModifierKey::Shift, ModifierKey::Control]),
                ),
            ],
        },
        Rule {
            description: "[VK1] a/z -> f10/f7",
            manipulators: vec![
                build_manipulator(VirtualKey::Vk1, KeyCode::A, None, KeyCode::F10, None),
                build_manipulator(VirtualKey::Vk1, KeyCode::Z, None, KeyCode::F7, None),
            ],
        },
        Rule {
            description: "[VK1] u/i -> command+left/command+right",
            manipulators: vec![
                build_manipulator(
                    VirtualKey::Vk1,
                    KeyCode::U,
                    Some(FromModifier::Optional(vec![ModifierKey::Any])),
                    KeyCode::LeftArrow,
                    Some(vec![ModifierKey::Command]),
                ),
                build_manipulator(
                    VirtualKey::Vk1,
                    KeyCode::I,
                    Some(FromModifier::Optional(vec![ModifierKey::Any])),
                    KeyCode::RightArrow,
                    Some(vec![ModifierKey::Command]),
                ),
            ],
        },
        Rule {
            description: "[VK1] g -> tab",
            manipulators: vec![build_manipulator(
                VirtualKey::Vk1,
                KeyCode::G,
                Some(FromModifier::Optional(vec![ModifierKey::Any])),
                KeyCode::Tab,
                None,
            )],
        },
        Rule {
            description: "[VK1] o/p -> control+shift+tab/control+tab",
            manipulators: vec![
                build_manipulator(
                    VirtualKey::Vk1,
                    KeyCode::O,
                    None,
                    KeyCode::Tab,
                    Some(vec![ModifierKey::Control, ModifierKey::Shift]),
                ),
                build_manipulator(
                    VirtualKey::Vk1,
                    KeyCode::P,
                    None,
                    KeyCode::Tab,
                    Some(vec![ModifierKey::Control]),
                ),
            ],
        },
        Rule {
            description: "[VK1] y/t/x -> command+c/command+x/command+shift+v",
            manipulators: vec![
                build_manipulator(
                    VirtualKey::Vk1,
                    KeyCode::Y,
                    None,
                    KeyCode::C,
                    Some(vec![ModifierKey::Command]),
                ),
                build_manipulator(
                    VirtualKey::Vk1,
                    KeyCode::T,
                    None,
                    KeyCode::X,
                    Some(vec![ModifierKey::Command]),
                ),
                build_manipulator(
                    VirtualKey::Vk1,
                    KeyCode::X,
                    None,
                    KeyCode::V,
                    Some(vec![
                        ModifierKey::Command,
                        ModifierKey::Shift,
                        ModifierKey::Option,
                    ]),
                ),
            ],
        },
        Rule {
            description: "[VK1] c/e -> backspace/delete",
            manipulators: vec![
                build_manipulator(
                    VirtualKey::Vk1,
                    KeyCode::C,
                    None,
                    KeyCode::DeleteOrBackspace,
                    None,
                ),
                build_manipulator(
                    VirtualKey::Vk1,
                    KeyCode::E,
                    None,
                    KeyCode::DeleteForward,
                    None,
                ),
            ],
        },
        Rule {
            description: "[VK1] [ -> command+z",
            manipulators: vec![build_manipulator(
                VirtualKey::Vk1,
                KeyCode::CloseBracket,
                Some(FromModifier::Optional(vec![ModifierKey::Any])),
                KeyCode::Z,
                Some(vec![ModifierKey::Command]),
            )],
        },
        Rule {
            description: "[VK1] colon -> command+h",
            manipulators: vec![build_manipulator(
                VirtualKey::Vk1,
                KeyCode::Quote,
                None,
                KeyCode::H,
                Some(vec![ModifierKey::Command]),
            )],
        },
        Rule {
            description: "[VK1] n/m/comma/dot -> mouse move",
            manipulators: vec![
                (
                    KeyCode::N,
                    Some(FromModifier::Mandatory(vec![ModifierKey::Shift])),
                    Some(-1536),
                    None,
                ),
                (
                    KeyCode::M,
                    Some(FromModifier::Mandatory(vec![ModifierKey::Shift])),
                    None,
                    Some(1536),
                ),
                (
                    KeyCode::Comma,
                    Some(FromModifier::Mandatory(vec![ModifierKey::Shift])),
                    None,
                    Some(-1536),
                ),
                (
                    KeyCode::Period,
                    Some(FromModifier::Mandatory(vec![ModifierKey::Shift])),
                    Some(1536),
                    None,
                ),
                (KeyCode::N, None, Some(-3072), None),
                (KeyCode::M, None, None, Some(3072)),
                (KeyCode::Comma, None, None, Some(-3072)),
                (KeyCode::Period, None, Some(3072), None),
            ]
            .into_iter()
            .map(|(key_code, modifiers, x, y)| Manipulator {
                conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                from: From {
                    key_code,
                    modifiers,
                },
                to: vec![To {
                    key_code: None,
                    modifiers: None,
                    set_variable: None,
                    mouse_key: Some(MouseKey {
                        x,
                        y,
                        vertical_wheel: None,
                    }),
                    pointing_button: None,
                    shell_command: None,
                }],
                r#type: ManipulatorType::default(),
                to_after_key_up: None,
                to_if_alone: None,
            })
            .collect::<Vec<Manipulator>>(),
        },
        Rule {
            description: "[VK1] / -> left click, _ -> right click",
            manipulators: vec![
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                    from: From {
                        key_code: KeyCode::Slash,
                        modifiers: Some(FromModifier::Optional(vec![ModifierKey::Any])),
                    },
                    to: vec![To {
                        key_code: None,
                        modifiers: None,
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: Some(PointingButton::Button1),
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                    from: From {
                        key_code: KeyCode::International1,
                        modifiers: Some(FromModifier::Optional(vec![ModifierKey::Any])),
                    },
                    to: vec![To {
                        key_code: None,
                        modifiers: None,
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: Some(PointingButton::Button2),
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
            ],
        },
        Rule {
            description: "[VK1] @/] -> scroll",
            manipulators: vec![
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                    from: From {
                        key_code: KeyCode::OpenBracket,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: None,
                        modifiers: None,
                        set_variable: None,
                        mouse_key: Some(MouseKey {
                            x: None,
                            y: None,
                            vertical_wheel: Some(-64),
                        }),
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                    from: From {
                        key_code: KeyCode::NonUsPound,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: None,
                        modifiers: None,
                        set_variable: None,
                        mouse_key: Some(MouseKey {
                            x: None,
                            y: None,
                            vertical_wheel: Some(64),
                        }),
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                    from: From {
                        key_code: KeyCode::Backslash,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: None,
                        modifiers: None,
                        set_variable: None,
                        mouse_key: Some(MouseKey {
                            x: None,
                            y: None,
                            vertical_wheel: Some(64),
                        }),
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
            ],
        },
        Rule {
            description: "[VK1] numbers -> function keys",
            manipulators: vec![
                (KeyCode::Key1, KeyCode::F1),
                (KeyCode::Key2, KeyCode::F2),
                (KeyCode::Key3, KeyCode::F3),
                (KeyCode::Key4, KeyCode::F4),
                (KeyCode::Key5, KeyCode::F5),
                (KeyCode::Key6, KeyCode::F6),
                (KeyCode::Key7, KeyCode::F7),
                (KeyCode::Key8, KeyCode::F8),
                (KeyCode::Key9, KeyCode::F9),
                (KeyCode::Key0, KeyCode::F10),
                (KeyCode::Hyphen, KeyCode::F11),
                (KeyCode::EqualSign, KeyCode::F12),
            ]
            .into_iter()
            .map(|(from, to)| Manipulator {
                conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                from: From {
                    key_code: from,
                    modifiers: None,
                },
                to: vec![To {
                    key_code: Some(to),
                    modifiers: None,
                    set_variable: None,
                    mouse_key: None,
                    pointing_button: None,
                    shell_command: None,
                }],
                r#type: ManipulatorType::default(),
                to_after_key_up: None,
                to_if_alone: None,
            })
            .collect::<Vec<Manipulator>>(),
        },
        Rule {
            description: "[VK1] b -> window maximize (ShiftIt)",
            manipulators: vec![Manipulator {
                conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                from: From {
                    key_code: KeyCode::B,
                    modifiers: None,
                },
                to: vec![To {
                    key_code: Some(KeyCode::M),
                    modifiers: Some(vec![
                        ModifierKey::Control,
                        ModifierKey::Option,
                        ModifierKey::Command,
                    ]),
                    set_variable: None,
                    mouse_key: None,
                    pointing_button: None,
                    shell_command: None,
                }],
                r#type: ManipulatorType::default(),
                to_after_key_up: None,
                to_if_alone: None,
            }],
        },
        Rule {
            description: "[VK1] \\ -> command+option+d (Hide the Dock)",
            manipulators: vec![Manipulator {
                conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                from: From {
                    key_code: KeyCode::International3,
                    modifiers: None,
                },
                to: vec![To {
                    key_code: Some(KeyCode::D),
                    modifiers: Some(vec![ModifierKey::Command, ModifierKey::Option]),
                    set_variable: None,
                    mouse_key: None,
                    pointing_button: None,
                    shell_command: None,
                }],
                r#type: ManipulatorType::default(),
                to_after_key_up: None,
                to_if_alone: None,
            }],
        },
        Rule {
            description: "[VK2] f/d -> command+tab/command+shift+tab",
            manipulators: vec![
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk2)]),
                    from: From {
                        key_code: KeyCode::F,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::Tab),
                        modifiers: Some(vec![ModifierKey::Command]),
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk2)]),
                    from: From {
                        key_code: KeyCode::D,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::Tab),
                        modifiers: Some(vec![ModifierKey::Command, ModifierKey::Shift]),
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
            ],
        },
        Rule {
            description: "[VK2] s/a -> control+tab/control+shift+tab",
            manipulators: vec![
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk2)]),
                    from: From {
                        key_code: KeyCode::S,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::Tab),
                        modifiers: Some(vec![ModifierKey::Control]),
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk2)]),
                    from: From {
                        key_code: KeyCode::A,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::Tab),
                        modifiers: Some(vec![ModifierKey::Control, ModifierKey::Shift]),
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
            ],
        },
        Rule {
            description: "[VK2] 9/0 -> command+shift+;/command+hyphen",
            manipulators: vec![
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk2)]),
                    from: From {
                        key_code: KeyCode::Key9,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::Semicolon),
                        modifiers: Some(vec![ModifierKey::Command, ModifierKey::Shift]),
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk2)]),
                    from: From {
                        key_code: KeyCode::Key0,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::Hyphen),
                        modifiers: Some(vec![ModifierKey::Command]),
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
            ],
        },
        Rule {
            description: "[VK2] 1/2 -> volume decrement/increment",
            manipulators: vec![
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk2)]),
                    from: From {
                        key_code: KeyCode::Key1,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::VolumeDecrement),
                        modifiers: None,
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk2)]),
                    from: From {
                        key_code: KeyCode::Key2,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::VolumeIncrement),
                        modifiers: None,
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
            ],
        },
        Rule {
            description: "[VK2] 3/4 -> display brightness decrement/increment",
            manipulators: vec![
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk2)]),
                    from: From {
                        key_code: KeyCode::Key3,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::DisplayBrightnessDecrement),
                        modifiers: None,
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk2)]),
                    from: From {
                        key_code: KeyCode::Key4,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::DisplayBrightnessIncrement),
                        modifiers: None,
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
            ],
        },
        Rule {
            description: "[VK2] ShiftIt",
            manipulators: vec![
                (KeyCode::H, KeyCode::LeftArrow),
                (KeyCode::O, KeyCode::RightArrow),
                (KeyCode::N, KeyCode::DownArrow),
                (KeyCode::P, KeyCode::UpArrow),
                (KeyCode::U, KeyCode::Key1),
                (KeyCode::I, KeyCode::Key2),
                (KeyCode::M, KeyCode::Key3),
                (KeyCode::Comma, KeyCode::Key4),
            ]
            .into_iter()
            .map(|(from, to)| Manipulator {
                conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk2)]),
                from: From {
                    key_code: from,
                    modifiers: Some(FromModifier::Mandatory(vec![ModifierKey::Control])),
                },
                to: vec![To {
                    key_code: Some(to),
                    modifiers: Some(vec![
                        ModifierKey::Command,
                        ModifierKey::Control,
                        ModifierKey::Option,
                    ]),
                    set_variable: None,
                    mouse_key: None,
                    pointing_button: None,
                    shell_command: None,
                }],
                r#type: ManipulatorType::default(),
                to_after_key_up: None,
                to_if_alone: None,
            })
            .collect::<Vec<Manipulator>>(),
        },
        Rule {
            description: "Open apps",
            manipulators: vec![
                (KeyCode::J, "open -a 'Google Chrome.app'"),
                (KeyCode::L, "open -a 'Alfred 4.app'"),
                (KeyCode::K, "open -a 'iTerm.app'"),
                (KeyCode::L, "open -a 'Alfred 4.app'"),
                (KeyCode::I, "open -a 'IntelliJ IDEA.app'"),
                (
                    KeyCode::E,
                    r#"osascript -e "tell application \"Alfred 4\" to search \"snip \"""#,
                ),
                (KeyCode::Slash, "open -a 'Slack.app'"),
                (KeyCode::OpenBracket, "open -a 'Mail.app'"),
                (KeyCode::T, "open -a 'Microsoft To Do.app'"),
                (KeyCode::G, "open -a 'Atom.app'"),
                (KeyCode::B, "open -a 'Tweetbot.app'"),
                (KeyCode::M, "open -a 'Skim.app'"),
                (KeyCode::R, "open -a 'Notes.app'"),
                (KeyCode::V, "open -a 'Visual Studio Code.app'"),
                (KeyCode::W, "open -a '1Password.app'"),
            ]
            .into_iter()
            .map(|(key_code, shell_command)| Manipulator {
                conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk2)]),
                from: From {
                    key_code,
                    modifiers: None,
                },
                to: vec![To {
                    key_code: None,
                    modifiers: None,
                    set_variable: None,
                    mouse_key: None,
                    pointing_button: None,
                    shell_command: Some(shell_command),
                }],
                r#type: ManipulatorType::default(),
                to_after_key_up: None,
                to_if_alone: None,
            })
            .collect::<Vec<Manipulator>>(),
        },
        Rule {
            description: "[VK3] a..: -> 1..-",
            manipulators: vec![
                (KeyCode::A, KeyCode::Key1),
                (KeyCode::S, KeyCode::Key2),
                (KeyCode::D, KeyCode::Key3),
                (KeyCode::F, KeyCode::Key4),
                (KeyCode::G, KeyCode::Key5),
                (KeyCode::H, KeyCode::Key6),
                (KeyCode::J, KeyCode::Key7),
                (KeyCode::K, KeyCode::Key8),
                (KeyCode::L, KeyCode::Key9),
                (KeyCode::Semicolon, KeyCode::Key0),
                (KeyCode::Quote, KeyCode::Hyphen),
            ]
            .into_iter()
            .map(|(from, to)| Manipulator {
                r#type: ManipulatorType::default(),
                conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk3)]),
                from: From {
                    key_code: from,
                    modifiers: Some(FromModifier::Optional(vec![ModifierKey::Any])),
                },
                to: vec![To {
                    set_variable: None,
                    key_code: Some(to),
                    modifiers: None,
                    mouse_key: None,
                    pointing_button: None,
                    shell_command: None,
                }],
                to_after_key_up: None,
                to_if_alone: None,
            })
            .collect::<Vec<Manipulator>>(),
        },
        Rule {
            description: "; -> enter",
            manipulators: vec![
                Manipulator {
                    r#type: ManipulatorType::default(),
                    conditions: None,
                    from: From {
                        key_code: KeyCode::Semicolon,
                        modifiers: Some(FromModifier::Mandatory(vec![ModifierKey::Control])),
                    },
                    to: vec![To {
                        set_variable: None,
                        key_code: Some(KeyCode::Semicolon),
                        modifiers: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    r#type: ManipulatorType::default(),
                    conditions: None,
                    from: From {
                        key_code: KeyCode::Semicolon,
                        modifiers: Some(FromModifier::Mandatory(vec![ModifierKey::Shift])),
                    },
                    to: vec![To {
                        set_variable: None,
                        key_code: Some(KeyCode::Semicolon),
                        modifiers: Some(vec![ModifierKey::Shift]),
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    r#type: ManipulatorType::default(),
                    conditions: None,
                    from: From {
                        key_code: KeyCode::Semicolon,
                        modifiers: Some(FromModifier::Optional(vec![ModifierKey::Any])),
                    },
                    to: vec![To {
                        set_variable: None,
                        key_code: Some(KeyCode::ReturnOrEnter),
                        modifiers: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    to_after_key_up: None,
                    to_if_alone: None,
                },
            ],
        },
        Rule {
            description: "control+: -> '",
            manipulators: vec![Manipulator {
                r#type: ManipulatorType::default(),
                conditions: None,
                from: From {
                    key_code: KeyCode::Quote,
                    modifiers: Some(FromModifier::Mandatory(vec![ModifierKey::Control])),
                },
                to: vec![To {
                    set_variable: None,
                    key_code: Some(KeyCode::Key7),
                    modifiers: Some(vec![ModifierKey::Shift]),
                    mouse_key: None,
                    pointing_button: None,
                    shell_command: None,
                }],
                to_after_key_up: None,
                to_if_alone: None,
            }],
        },
        Rule {
            description: "caps_lock -> vk_none",
            manipulators: vec![Manipulator {
                r#type: ManipulatorType::default(),
                conditions: None,
                from: From {
                    key_code: KeyCode::CapsLock,
                    modifiers: Some(FromModifier::Optional(vec![ModifierKey::Any])),
                },
                to: vec![To {
                    set_variable: None,
                    key_code: Some(KeyCode::VkNone),
                    modifiers: None,
                    mouse_key: None,
                    pointing_button: None,
                    shell_command: None,
                }],
                to_after_key_up: None,
                to_if_alone: None,
            }],
        },
    ];

    let config = Config {
        title: "Personal rules",
        rules,
    };

    println!("{}", serde_json::to_string(&config).unwrap());
}

fn tmux_prefix() -> To {
    To {
        set_variable: None,
        key_code: Some(KeyCode::T),
        modifiers: Some(vec![ModifierKey::Control]),
        mouse_key: None,
        pointing_button: None,
        shell_command: None,
    }
}

fn build_manipulator(
    virtual_key: VirtualKey,
    from: KeyCode,
    from_modifiers: Option<FromModifier>,
    to: KeyCode,
    to_modifiers: Option<Vec<ModifierKey>>,
) -> Manipulator {
    Manipulator {
        conditions: Some(vec![Condition::with_virtual_key(virtual_key)]),
        from: From {
            key_code: from,
            modifiers: from_modifiers,
        },
        to: vec![To {
            key_code: Some(to),
            modifiers: to_modifiers,
            set_variable: None,
            mouse_key: None,
            pointing_button: None,
            shell_command: None,
        }],
        r#type: ManipulatorType::default(),
        to_after_key_up: None,
        to_if_alone: None,
    }
}
