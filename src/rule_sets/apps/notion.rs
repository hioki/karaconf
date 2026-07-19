use crate::karabiner_data::{
    BundleIdentifier, Condition, KeyCode::*, Manipulator, ModifierKey::*, VirtualKey,
};
use std::collections::BTreeMap;

pub fn manipulators() -> Vec<Manipulator> {
    let mut manipulators = Vec::new();
    let m = BTreeMap::from([
        (
            VirtualKey::Vk2,
            vec![
                ("Zoom in", Key9, KeypadPlus, vec![Cmd]),
                ("Zoom out", Key0, Hyphen, vec![Cmd]),
            ],
        ),
        (
            VirtualKey::Vk4,
            vec![
                ("Toggle sidebar", E, International3, vec![Cmd]),
                ("Search", F, P, vec![Cmd]),
                ("Copy link", Y, L, vec![Cmd]),
            ],
        ),
    ]);
    for (vk, mappings) in &m {
        for (description, from, to, modifiers) in mappings {
            let manipulator = Manipulator::builder()
                .description(*description)
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
