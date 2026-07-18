use crate::karabiner_data::{
    BundleIdentifier, Condition, KeyCode::*, Manipulator, ModifierKey::*, VirtualKey,
};
use std::collections::BTreeMap;

pub fn manipulators() -> Vec<Manipulator> {
    let mut manipulators = Vec::new();
    let m = BTreeMap::from([
        (
            VirtualKey::Vk2,
            vec![(Key9, KeypadPlus, vec![Cmd]), (Key0, Hyphen, vec![Cmd])],
        ),
        (
            VirtualKey::Vk4,
            vec![
                (C, L, vec![Cmd]),
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
    for (vk, mappings) in &m {
        for (from, to, modifiers) in mappings {
            let manipulator = Manipulator::builder()
                .from_key(from.clone())
                .to_key(to.clone(), Some(modifiers.clone()))
                .conditions(vec![
                    Condition::on_app(BundleIdentifier::Notion),
                    Condition::with_virtual_key(vk.clone()),
                ])
                .build();
            manipulators.push(manipulator);
        }
    }

    manipulators
}
