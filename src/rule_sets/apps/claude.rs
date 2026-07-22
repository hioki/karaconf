use crate::karabiner_data::{
    BundleIdentifier::Claude, Condition, KeyCode::*, Manipulator, ModifierKey::*, VirtualKey,
};

pub fn manipulators() -> Vec<Manipulator> {
    let mut manipulators = vec![];
    for (description, from_key, (to_key, modifiers)) in [
        (
            "キーボードショートカットを表示",
            Slash,
            (Slash, Some(vec![Cmd])),
        ),
        ("Home に切替", H, (LeftArrow, Some(vec![Opt, Cmd]))),
        ("Code に切替", L, (RightArrow, Some(vec![Opt, Cmd]))),
        ("次のセッションを表示", N, (Tab, Some(vec![Ctrl]))),
        ("前のセッションを表示", P, (Tab, Some(vec![Shift, Ctrl]))),
        ("モデル切替", M, (I, Some(vec![Shift, Cmd]))),
        ("モード切替", O, (M, Some(vec![Shift, Cmd]))),
        ("エフォート切替", E, (E, Some(vec![Shift, Cmd]))),
    ] {
        manipulators.push(
            Manipulator::builder()
                .description(description)
                .conditions(vec![
                    Condition::on_app(Claude),
                    Condition::with_virtual_key(VirtualKey::Vk4),
                ])
                .from_key(from_key)
                .to_key(to_key, modifiers)
                .build(),
        )
    }
    manipulators
}
