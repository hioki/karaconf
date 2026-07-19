use crate::karabiner_data::{
    BundleIdentifier::ITerm2,
    Condition, FromModifier,
    KeyCode::*,
    Manipulator,
    ModifierKey::{self, *},
    VirtualKey::{Vk1, Vk2, Vk4},
};

const CTRL: &[ModifierKey] = &[Ctrl];
const SHIFT: &[ModifierKey] = &[Shift];
const CMD: &[ModifierKey] = &[Cmd];

pub fn manipulators() -> Vec<Manipulator> {
    let mut manipulators = Vec::new();

    // Disable default Cmd+W behavior in iTerm2
    manipulators.push(
        Manipulator::builder()
            .description("disable default Cmd+W")
            .condition(Condition::on_app(ITerm2))
            .from_key_with_modifiers(W, FromModifier::Mandatory(CMD.to_vec()))
            .to_key(VkNone, None)
            .build(),
    );

    // save file in vim
    manipulators.push(
        Manipulator::builder()
            .description("vim: save file (:w)")
            .conditions(vec![
                Condition::on_app(ITerm2),
                Condition::with_virtual_key(Vk1),
            ])
            .from_key(W)
            .to_key(Escape, None)
            .to_key(Quote, None)
            .to_key(W, None)
            .to_key(ReturnOrEnter, None)
            .build(),
    );
    // quit vim
    manipulators.push(
        Manipulator::builder()
            .description("vim: quit (:q)")
            .conditions(vec![
                Condition::on_app(ITerm2),
                Condition::with_virtual_key(Vk1),
            ])
            .from_key(Q)
            .to_key(Escape, None)
            .to_key(Quote, None)
            .to_key(Q, None)
            .to_key(ReturnOrEnter, None)
            .build(),
    );
    // go to line head/tail in vim
    for (from, to, description) in [
        (U, Key0, "vim: go to line head (0)"),
        (I, Key4, "vim: go to line tail ($)"),
    ] {
        manipulators.push(
            Manipulator::builder()
                .description(description)
                .conditions(vec![
                    Condition::on_app(ITerm2),
                    Condition::with_virtual_key(Vk1),
                ])
                .from_key(from)
                .to_key(to, Some(SHIFT.to_vec()))
                .build(),
        )
    }

    //
    // for tmux
    //
    for key_code in [C, J, K, N, P, S, V] {
        manipulators.push(
            Manipulator::builder()
                .conditions(vec![
                    Condition::on_app(ITerm2),
                    Condition::with_virtual_key(Vk4),
                ])
                .from_key(key_code.clone())
                .to_key(T, Some(CTRL.to_vec()))
                .to_key(key_code, Some(CTRL.to_vec()))
                .build(),
        )
    }
    // Pane movement with O and P in tmux
    for (from, to) in [(O, P), (P, N)] {
        manipulators.push(
            Manipulator::builder()
                .description("tmux: pane movement")
                .conditions(vec![
                    Condition::on_app(ITerm2),
                    Condition::with_virtual_key(Vk1),
                ])
                .from_key(from)
                .to_key(T, Some(CTRL.to_vec()))
                .to_key(to, Some(CTRL.to_vec()))
                .build(),
        )
    }
    // Pane movement with A and S in tmux
    for (from, to) in [(A, P), (S, N)] {
        manipulators.push(
            Manipulator::builder()
                .description("tmux: pane movement")
                .conditions(vec![
                    Condition::on_app(ITerm2),
                    Condition::with_virtual_key(Vk2),
                ])
                .from_key(from)
                .to_key(T, Some(CTRL.to_vec()))
                .to_key(to, Some(CTRL.to_vec()))
                .build(),
        )
    }
    // Enter copy mode in tmux
    manipulators.push(
        Manipulator::builder()
            .description("tmux: enter copy mode")
            .conditions(vec![
                Condition::on_app(ITerm2),
                Condition::with_virtual_key(Vk1),
            ])
            .from_key(Z)
            .to_key(T, Some(CTRL.to_vec()))
            .to_key(B, Some(CTRL.to_vec()))
            .build(),
    );

    manipulators
}
