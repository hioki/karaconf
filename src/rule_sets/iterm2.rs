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

    // for Neovim
    // <VK4>+<Key> -> <Leader><Key>
    for key_code in [E, K, F, H, L, O] {
        manipulators.push(
            Manipulator::builder()
                .conditions(vec![
                    Condition::on_app(ITerm2),
                    Condition::with_virtual_key(Vk4),
                ])
                .from_key(key_code.clone())
                .to_key(International3, None)
                .to_key(key_code, None)
                .build(),
        );
    }

    // for tmux
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

    // Disable default Cmd+W behavior in iTerm2
    manipulators.push(
        Manipulator::builder()
            .condition(Condition::on_app(ITerm2))
            .from_key_with_modifiers(W, FromModifier::Mandatory(CMD.to_vec()))
            .to_key(VkNone, None)
            .build(),
    );

    // Pane movement
    for (from, to) in [(O, P), (P, N)] {
        manipulators.push(
            Manipulator::builder()
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
    for (from, to) in [(A, P), (S, N)] {
        manipulators.push(
            Manipulator::builder()
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

    // save file in vim
    manipulators.push(
        Manipulator::builder()
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

    // U -> Shift+0 (go to line head in vim)
    // I -> Shift+4 (go to line tail in vim)
    for (from, to) in [(U, Key0), (I, Key4)] {
        manipulators.push(
            Manipulator::builder()
                .conditions(vec![
                    Condition::on_app(ITerm2),
                    Condition::with_virtual_key(Vk1),
                ])
                .from_key(from)
                .to_key(to, Some(SHIFT.to_vec()))
                .build(),
        )
    }

    // what is this?
    manipulators.push(
        Manipulator::builder()
            .conditions(vec![
                Condition::on_app(ITerm2),
                Condition::with_virtual_key(Vk1),
            ])
            .from_key(Semicolon)
            .to_key(F, Some(CTRL.to_vec()))
            .build(),
    );

    // Enter copy mode in tmux
    manipulators.push(
        Manipulator::builder()
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
