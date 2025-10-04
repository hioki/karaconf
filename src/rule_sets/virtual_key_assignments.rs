use crate::karabiner_data::{
    FromModifier, KeyCode, KeyCode::*, Manipulator, ModifierKey::*, SetVariable, VirtualKey,
    VirtualKey::*,
};
use std::collections::BTreeMap;

pub fn manipulators() -> Vec<Manipulator> {
    let m: BTreeMap<(VirtualKey, Option<KeyCode>), Vec<KeyCode>> = BTreeMap::from([
        ((Vk1, Some(JapaneseKana)), vec![Lang1, International4]),
        ((Vk2, Some(JapaneseEisuu)), vec![Lang2, International5]),
        ((Vk3, None), vec![RightGui, International2]),
        ((Vk4, Some(Tab)), vec![Tab]),
    ]);
    m.iter()
        .flat_map(|((virtual_key, to_if_alone), key_codes)| {
            key_codes.iter().map(move |key_code| {
                let mut builder = Manipulator::builder()
                    .from_key_with_modifiers(key_code.clone(), FromModifier::Optional(vec![Any]))
                    .to_variable(SetVariable {
                        name: virtual_key.clone(),
                        value: 1,
                    })
                    .to_after_key_up(SetVariable {
                        name: virtual_key.clone(),
                        value: 0,
                    });
                if let Some(to_if_alone) = to_if_alone {
                    builder = builder.to_if_alone(to_if_alone.clone());
                }
                builder.build()
            })
        })
        .collect()
}
