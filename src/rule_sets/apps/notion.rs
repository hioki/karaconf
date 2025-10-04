use crate::karabiner_data::{KeyCode::*, ModifierKey::*, *};
use std::collections::BTreeMap;

pub fn manipulators() -> Vec<Manipulator> {
    let m = BTreeMap::from([
        (
            VirtualKey::Vk2,
            vec![
                (Key9, KeypadPlus, vec![Cmd]),
                (Key9, EqualSign, vec![Cmd]),
                (Key0, Hyphen, vec![Cmd]),
            ],
        ),
        (
            VirtualKey::Vk4,
            vec![
                (E, International3, vec![Cmd]),
                (F, P, vec![Cmd]),
                (H, CloseBracket, vec![Cmd]),
                (L, NonUsPound, vec![Cmd]),
                (U, U, vec![Cmd, Shift]),
                (N, J, vec![Ctrl, Shift]),
                (P, K, vec![Ctrl, Shift]),
            ],
        ),
    ]);
    m.iter()
        .flat_map(|(vk, mappings)| {
            mappings.iter().map(move |(from, to, modifiers)| {
                Manipulator::builder()
                    .from_key(from.clone())
                    .to_key(to.clone(), Some(modifiers.clone()))
                    .conditions(vec![
                        Condition::on_app(BundleIdentifier::Notion),
                        Condition::with_virtual_key(vk.clone()),
                    ])
                    .build()
            })
        })
        .collect()
}
