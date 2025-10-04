use crate::karabiner_data::{
    BundleIdentifier::ITerm2,
    Condition, FromModifier,
    KeyCode::*,
    Manipulator,
    ModifierKey::{self, *},
    VirtualKey::{Vk1, Vk2, Vk4},
};

const CTRL: &[ModifierKey] = &[Ctrl];

pub fn manipulators() -> Vec<Manipulator> {
    vec![
        // for Neovim
        // <VK4>+<Key> -> <Leader><Key>
        vec![E, K, F, H, L, O]
            .into_iter()
            .map(|key_code| {
                Manipulator::builder()
                    .conditions(vec![
                        Condition::on_app(ITerm2),
                        Condition::with_virtual_key(Vk4),
                    ])
                    .from_key(key_code.clone())
                    .to_key(International3, None)
                    .to_key(key_code, None)
                    .build()
            })
            .collect(),
        // for tmux
        vec![C, J, K, N, P, S, V]
            .into_iter()
            .map(|key_code| {
                Manipulator::builder()
                    .conditions(vec![
                        Condition::on_app(ITerm2),
                        Condition::with_virtual_key(Vk4),
                    ])
                    .from_key(key_code.clone())
                    .to_key(T, Some(CTRL.to_vec()))
                    .to_key(key_code, Some(CTRL.to_vec()))
                    .build()
            })
            .collect(),
        vec![
            Manipulator::builder()
                .condition(Condition::on_app(ITerm2))
                .from_key_with_modifiers(W, FromModifier::Mandatory(vec![Cmd]))
                .to_key(VkNone, None)
                .build(),
        ],
        vec![(O, P), (P, N)]
            .into_iter()
            .map(|(from, to)| {
                Manipulator::builder()
                    .conditions(vec![
                        Condition::on_app(ITerm2),
                        Condition::with_virtual_key(Vk1),
                    ])
                    .from_key(from)
                    .to_key(T, Some(CTRL.to_vec()))
                    .to_key(to, Some(CTRL.to_vec()))
                    .build()
            })
            .collect(),
        vec![(A, P), (S, N)]
            .into_iter()
            .map(|(from, to)| {
                Manipulator::builder()
                    .conditions(vec![
                        Condition::on_app(ITerm2),
                        Condition::with_virtual_key(Vk2),
                    ])
                    .from_key(from)
                    .to_key(T, Some(CTRL.to_vec()))
                    .to_key(to, Some(CTRL.to_vec()))
                    .build()
            })
            .collect(),
        vec![
            Manipulator::builder()
                .conditions(vec![
                    Condition::on_app(ITerm2),
                    Condition::with_virtual_key(Vk1),
                ])
                .from_key(W)
                .to_key(Escape, None)
                .to_key(Quote, None)
                .to_key(W, None)
                .to_key(ReturnOrEnter, None)
                .build(),
            Manipulator::builder()
                .conditions(vec![
                    Condition::on_app(ITerm2),
                    Condition::with_virtual_key(Vk1),
                ])
                .from_key(Q)
                .to_key(Escape, None)
                .to_key(Quote, None)
                .to_key(Q, None)
                .to_key(ReturnOrEnter, None)
                .build(),
        ],
        vec![(U, Key0), (I, Key4)]
            .into_iter()
            .map(|(from, to)| {
                Manipulator::builder()
                    .conditions(vec![
                        Condition::on_app(ITerm2),
                        Condition::with_virtual_key(Vk1),
                    ])
                    .from_key(from)
                    .to_key(to, Some(vec![Shift]))
                    .build()
            })
            .collect(),
        vec![
            Manipulator::builder()
                .conditions(vec![
                    Condition::on_app(ITerm2),
                    Condition::with_virtual_key(Vk1),
                ])
                .from_key(Semicolon)
                .to_key(F, Some(CTRL.to_vec()))
                .build(),
            Manipulator::builder()
                .conditions(vec![
                    Condition::on_app(ITerm2),
                    Condition::with_virtual_key(Vk1),
                ])
                .from_key(Z)
                .to_key(T, Some(CTRL.to_vec()))
                .to_key(B, Some(CTRL.to_vec()))
                .build(),
        ],
    ]
    .into_iter()
    .flatten()
    .collect()
}
