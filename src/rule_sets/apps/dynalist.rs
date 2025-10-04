use crate::karabiner_data::{
    BundleIdentifier::Dynalist, Condition, KeyCode::*, Manipulator, ModifierKey::*, VirtualKey,
};
use std::collections::BTreeMap;

pub fn manipulators() -> Vec<Manipulator> {
    let m = BTreeMap::from([
        (
            VirtualKey::Vk1,
            vec![
                (U, A, Some(vec![Ctrl])), // go to line head
                (I, E, Some(vec![Ctrl])), // go to line tail
            ],
        ),
        (
            VirtualKey::Vk2,
            vec![
                (Key9, Hyphen, Some(vec![Cmd, Shift])), // zoom in
                (Key0, Hyphen, Some(vec![Cmd])),        // zoom out
            ],
        ),
        (
            VirtualKey::Vk4,
            vec![
                (E, F, Some(vec![Cmd, Shift])), // hide sidebar
                (F, O, Some(vec![Cmd])),        // search
            ],
        ),
    ]);
    m.iter()
        .flat_map(|(vk, mappings)| {
            mappings.iter().map(move |(from, to, to_modifiers)| {
                Manipulator::builder()
                    .conditions(vec![
                        Condition::on_app(Dynalist),
                        Condition::with_virtual_key(vk.clone()),
                    ])
                    .from_key(from.clone())
                    .to_key(to.clone(), to_modifiers.clone())
                    .build()
            })
        })
        .collect::<Vec<Manipulator>>()
}
