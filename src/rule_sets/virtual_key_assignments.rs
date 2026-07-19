use crate::karabiner_data::{
    FromModifier, KeyCode, KeyCode::*, Manipulator, ModifierKey::*, SetVariable, VirtualKey,
    VirtualKey::*,
};

pub fn manipulators() -> Vec<Manipulator> {
    #[allow(clippy::type_complexity)]
    let configs: Vec<(VirtualKey, Vec<KeyCode>, Option<KeyCode>, Option<u8>)> = vec![
        (
            Vk1,
            vec![Lang1, International4],
            Some(JapaneseKana),
            Some(1),
        ),
        (
            Vk2,
            vec![Lang2, International5],
            Some(JapaneseEisuu),
            Some(0),
        ),
        (Vk3, vec![RightGui, International2], None, None),
        (Vk4, vec![Tab], Some(Tab), None),
    ];

    configs
        .into_iter()
        .flat_map(|(virtual_key, key_codes, to_if_alone, shingeta_value)| {
            key_codes.into_iter().map(move |key_code| {
                let mut builder = Manipulator::builder()
                    .from_key_with_modifiers(key_code, FromModifier::Optional(vec![Any]))
                    .to_variable(SetVariable {
                        name: virtual_key.clone(),
                        value: 1,
                    })
                    .to_after_key_up(SetVariable {
                        name: virtual_key.clone(),
                        value: 0,
                    });
                if let Some(to_if_alone) = to_if_alone.clone() {
                    builder = builder.to_if_alone(to_if_alone);
                }
                if let Some(value) = shingeta_value {
                    builder = builder.to_if_alone_variable(SetVariable {
                        name: VirtualKey::ShingetaMode,
                        value,
                    });
                }
                builder.build()
            })
        })
        .collect()
}
