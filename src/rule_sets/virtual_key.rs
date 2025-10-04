use crate::karabiner_data::{KeyCode as K, ModifierKey::*, VirtualKey as VK, *};
use std::collections::BTreeMap;

pub fn manipulators() -> Vec<Manipulator> {
    let m: BTreeMap<(VirtualKey, Option<KeyCode>), Vec<KeyCode>> = BTreeMap::from([
        (
            (VK::Vk1, Some(K::JapaneseKana)),
            vec![K::Lang1, K::International4],
        ),
        (
            (VK::Vk2, Some(K::JapaneseEisuu)),
            vec![K::Lang2, K::International5],
        ),
        ((VK::Vk3, None), vec![K::RightGui, K::International2]),
        ((VK::Vk4, Some(K::Tab)), vec![K::Tab]),
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
