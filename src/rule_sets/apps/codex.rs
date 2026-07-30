use crate::karabiner_data::{
    BundleIdentifier::Codex, Condition, KeyCode::*, Manipulator, ModifierKey::*, VirtualKey,
};

pub fn manipulators() -> Vec<Manipulator> {
    let mut manipulators = vec![];
    for (description, from_key, (to_key, modifiers)) in [
        (
            "キーボードショートカットを表示",
            Slash,
            (K, Some(vec![Cmd, Opt, Shift, Ctrl])),
        ),
        ("新しいタスク", N, (N, Some(vec![Cmd]))),
        ("サイドバーを切り替える", E, (B, Some(vec![Cmd]))),
        (
            "レビューパネルの表示を切り替え",
            J,
            (B, Some(vec![Cmd, Opt])),
        ),
        ("サイドタスクを開く", S, (S, Some(vec![Cmd, Opt]))),
        ("MCP", P, (P, Some(vec![Cmd, Shift, Ctrl]))),
        (
            "リクエストを承認",
            A,
            (A, Some(vec![Cmd, Opt, Shift, Ctrl])),
        ),
        (
            "リクエストを拒否",
            D,
            (D, Some(vec![Cmd, Opt, Shift, Ctrl])),
        ),
        ("モデル選択を選ぶ", M, (M, Some(vec![Ctrl, Shift]))),
        (
            "音声入力を開始",
            Spacebar,
            (Spacebar, Some(vec![Cmd, Opt, Shift, Ctrl])),
        ),
        (
            "プランモードを切り替え",
            O,
            (O, Some(vec![Cmd, Opt, Shift, Ctrl])),
        ),
        (
            "Markdown形式でコピー",
            Y,
            (Y, Some(vec![Cmd, Opt, Shift, Ctrl])),
        ),
    ] {
        manipulators.push(
            Manipulator::builder()
                .description(description)
                .conditions(vec![
                    Condition::on_app(Codex),
                    Condition::with_virtual_key(VirtualKey::Vk4),
                ])
                .from_key(from_key)
                .to_key(to_key, modifiers)
                .build(),
        )
    }
    manipulators
}
