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
                ("拡大", Key9, KeypadPlus, vec![Cmd]),
                ("縮小", Key0, Hyphen, vec![Cmd]),
            ],
        ),
        (
            VirtualKey::Vk4,
            vec![
                ("サイドバーを表示/非表示", E, International3, vec![Cmd]),
                ("検索", F, P, vec![Cmd]),
                ("ページのリンクをコピー", Y, L, vec![Cmd]),
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
