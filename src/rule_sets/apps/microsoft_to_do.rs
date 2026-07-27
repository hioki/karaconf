use crate::karabiner_data::{
    BundleIdentifier::MicrosoftToDo, Condition, KeyCode::*, Manipulator, ModifierKey::*, VirtualKey,
};

pub fn manipulators() -> Vec<Manipulator> {
    let mut manipulators = vec![];
    for (description, from_key, (to_key, modifiers)) in [
        ("新しいリスト", N, (L, Some(vec![Cmd]))),
        ("リスト名の変更", R, (E, Some(vec![Cmd, Shift]))),
        ("新しいタスク", L, (N, Some(vec![Cmd]))),
        ("タスクを完了", K, (D, Some(vec![Cmd]))),
        ("タスクの削除", D, (DeleteOrBackspace, Some(vec![Cmd]))),
        (
            "完了済みタスクを非表示にする",
            H,
            (D, Some(vec![Shift, Cmd])),
        ),
    ] {
        manipulators.push(
            Manipulator::builder()
                .description(description)
                .conditions(vec![
                    Condition::on_app(MicrosoftToDo),
                    Condition::with_virtual_key(VirtualKey::Vk4),
                ])
                .from_key(from_key)
                .to_key(to_key, modifiers)
                .build(),
        )
    }
    manipulators
}
