use crate::karabiner_data::{Condition, FromModifier, KeyCode::*, Manipulator, ModifierKey::*};

// ============================================================================
// SHINGETA LAYOUT MAPPINGS - SIMULTANEOUS KEY MAPPINGS
// ============================================================================
// NOTE: Simultaneous key mappings MUST come before single key mappings
//       to ensure proper priority in Karabiner-Elements
pub fn manipulators() -> Vec<Manipulator> {
    let mut manipulators = vec![
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, Key1])
            .to_key(L, None)
            .to_key(A, None)
            .build(),
        // d+1 -> l -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, Key1])
            .to_key(L, None)
            .to_key(A, None)
            .build(),
        // s+1 -> l -> y -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, Key1])
            .to_key(L, None)
            .to_key(Y, None)
            .to_key(A, None)
            .build(),
        // l+1 -> l -> y -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, Key1])
            .to_key(L, None)
            .to_key(Y, None)
            .to_key(A, None)
            .build(),
        // k+2 -> l -> i
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, Key2])
            .to_key(L, None)
            .to_key(I, None)
            .build(),
        // d+2 -> l -> i
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, Key2])
            .to_key(L, None)
            .to_key(I, None)
            .build(),
        // s+2 -> m -> y -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, Key2])
            .to_key(M, None)
            .to_key(Y, None)
            .to_key(A, None)
            .build(),
        // l+2 -> m -> y -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, Key2])
            .to_key(M, None)
            .to_key(Y, None)
            .to_key(A, None)
            .build(),
        // k+3 -> l -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, Key3])
            .to_key(L, None)
            .to_key(U, None)
            .build(),
        // d+3 -> l -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, Key3])
            .to_key(L, None)
            .to_key(U, None)
            .build(),
        // s+3 -> m -> y -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, Key3])
            .to_key(M, None)
            .to_key(Y, None)
            .to_key(U, None)
            .build(),
        // l+3 -> m -> y -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, Key3])
            .to_key(M, None)
            .to_key(Y, None)
            .to_key(U, None)
            .build(),
        // k+4 -> l -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, Key4])
            .to_key(L, None)
            .to_key(E, None)
            .build(),
        // d+4 -> l -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, Key4])
            .to_key(L, None)
            .to_key(E, None)
            .build(),
        // s+4 -> m -> y -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, Key4])
            .to_key(M, None)
            .to_key(Y, None)
            .to_key(O, None)
            .build(),
        // l+4 -> m -> y -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, Key4])
            .to_key(M, None)
            .to_key(Y, None)
            .to_key(O, None)
            .build(),
        // k+5 -> l -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, Key5])
            .to_key(L, None)
            .to_key(O, None)
            .build(),
        // d+5 -> l -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, Key5])
            .to_key(L, None)
            .to_key(O, None)
            .build(),
        // s+5 -> l -> w -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, Key5])
            .to_key(L, None)
            .to_key(W, None)
            .to_key(A, None)
            .build(),
        // l+5 -> l -> w -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, Key5])
            .to_key(L, None)
            .to_key(W, None)
            .to_key(A, None)
            .build(),
        // k+7 -> option+comma
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, Key7])
            .to_key(Comma, Some(vec![Opt]))
            .build(),
        // d+7 -> option+comma
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, Key7])
            .to_key(Comma, Some(vec![Opt]))
            .build(),
        // s+7 -> option+period
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, Key7])
            .to_key(Period, Some(vec![Opt]))
            .build(),
        // l+7 -> option+period
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, Key7])
            .to_key(Period, Some(vec![Opt]))
            .build(),
        // k+8 -> close_bracket
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, Key8])
            .to_key(CloseBracket, None)
            .build(),
        // d+8 -> close_bracket
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, Key8])
            .to_key(CloseBracket, None)
            .build(),
        // s+8 -> shift+8
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, Key8])
            .to_key(Key8, Some(vec![Shift]))
            .build(),
        // l+8 -> shift+8
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, Key8])
            .to_key(Key8, Some(vec![Shift]))
            .build(),
        // k+9 -> backslash
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, Key9])
            .to_key(Backslash, None)
            .build(),
        // d+9 -> backslash
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, Key9])
            .to_key(Backslash, None)
            .build(),
        // s+9 -> shift+9
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, Key9])
            .to_key(Key9, Some(vec![Shift]))
            .build(),
        // l+9 -> shift+9
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, Key9])
            .to_key(Key9, Some(vec![Shift]))
            .build(),
        // k+0 -> semicolon
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, Key0])
            .to_key(Semicolon, None)
            .build(),
        // d+0 -> semicolon
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, Key0])
            .to_key(Semicolon, None)
            .build(),
        // s+0 -> quote
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, Key0])
            .to_key(Quote, None)
            .build(),
        // l+0 -> quote
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, Key0])
            .to_key(Quote, None)
            .build(),
        // k+hyphen -> open_bracket
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, Hyphen])
            .to_key(OpenBracket, None)
            .build(),
        // d+hyphen -> open_bracket
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, Hyphen])
            .to_key(OpenBracket, None)
            .build(),
        // s+hyphen -> shift+quote
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, Hyphen])
            .to_key(Quote, Some(vec![Shift]))
            .build(),
        // l+hyphen -> shift+quote
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, Hyphen])
            .to_key(Quote, Some(vec![Shift]))
            .build(),
        // k+q -> f -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, Q])
            .to_key(F, None)
            .to_key(A, None)
            .build(),
        // d+q -> f -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, Q])
            .to_key(F, None)
            .to_key(A, None)
            .build(),
        // s+q -> d -> i
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, Q])
            .to_key(D, None)
            .to_key(I, None)
            .build(),
        // l+q -> d -> i
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, Q])
            .to_key(D, None)
            .to_key(I, None)
            .build(),
        // k+w -> g -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, W])
            .to_key(G, None)
            .to_key(O, None)
            .build(),
        // d+w -> g -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, W])
            .to_key(G, None)
            .to_key(O, None)
            .build(),
        // s+w -> m -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, W])
            .to_key(M, None)
            .to_key(E, None)
            .build(),
        // l+w -> m -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, W])
            .to_key(M, None)
            .to_key(E, None)
            .build(),
        // k+e -> f -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, E])
            .to_key(F, None)
            .to_key(U, None)
            .build(),
        // d+e -> f -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, E])
            .to_key(F, None)
            .to_key(U, None)
            .build(),
        // s+e -> k -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, E])
            .to_key(K, None)
            .to_key(E, None)
            .build(),
        // l+e -> k -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, E])
            .to_key(K, None)
            .to_key(E, None)
            .build(),
        // k+r -> f -> i
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, R])
            .to_key(F, None)
            .to_key(I, None)
            .build(),
        // d+r -> f -> i
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, R])
            .to_key(F, None)
            .to_key(I, None)
            .build(),
        // s+r -> t -> h -> i
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, R])
            .to_key(T, None)
            .to_key(H, None)
            .to_key(I, None)
            .build(),
        // l+r -> t -> h -> i
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, R])
            .to_key(T, None)
            .to_key(H, None)
            .to_key(I, None)
            .build(),
        // k+t -> f -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, T])
            .to_key(F, None)
            .to_key(E, None)
            .build(),
        // d+t -> f -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, T])
            .to_key(F, None)
            .to_key(E, None)
            .build(),
        // s+t -> d -> h -> i
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, T])
            .to_key(D, None)
            .to_key(H, None)
            .to_key(I, None)
            .build(),
        // l+t -> d -> h -> i
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, T])
            .to_key(D, None)
            .to_key(H, None)
            .to_key(I, None)
            .build(),
        // k+y -> w -> i
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, Y])
            .to_key(W, None)
            .to_key(I, None)
            .build(),
        // d+y -> w -> i
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, Y])
            .to_key(W, None)
            .to_key(I, None)
            .build(),
        // s+y -> s -> h -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, Y])
            .to_key(S, None)
            .to_key(H, None)
            .to_key(E, None)
            .build(),
        // l+y -> s -> h -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, Y])
            .to_key(S, None)
            .to_key(H, None)
            .to_key(E, None)
            .build(),
        // k+u -> p -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, U])
            .to_key(P, None)
            .to_key(A, None)
            .build(),
        // d+u -> p -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, U])
            .to_key(P, None)
            .to_key(A, None)
            .build(),
        // s+u -> p -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, U])
            .to_key(P, None)
            .to_key(E, None)
            .build(),
        // l+u -> p -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, U])
            .to_key(P, None)
            .to_key(E, None)
            .build(),
        // k+i -> y -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, I])
            .to_key(Y, None)
            .to_key(O, None)
            .build(),
        // d+i -> y -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, I])
            .to_key(Y, None)
            .to_key(O, None)
            .build(),
        // s+i -> d -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, I])
            .to_key(D, None)
            .to_key(O, None)
            .build(),
        // l+i -> d -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, I])
            .to_key(D, None)
            .to_key(O, None)
            .build(),
        // k+o -> m -> i
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, O])
            .to_key(M, None)
            .to_key(I, None)
            .build(),
        // d+o -> m -> i
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, O])
            .to_key(M, None)
            .to_key(I, None)
            .build(),
        // s+o -> y -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, O])
            .to_key(Y, None)
            .to_key(A, None)
            .build(),
        // l+o -> y -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, O])
            .to_key(Y, None)
            .to_key(A, None)
            .build(),
        // k+p -> w -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, P])
            .to_key(W, None)
            .to_key(E, None)
            .build(),
        // d+p -> w -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, P])
            .to_key(W, None)
            .to_key(E, None)
            .build(),
        // s+p -> j -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, P])
            .to_key(J, None)
            .to_key(E, None)
            .build(),
        // l+p -> j -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, P])
            .to_key(J, None)
            .to_key(E, None)
            .build(),
        // k+open_bracket -> u -> l -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, OpenBracket])
            .to_key(U, None)
            .to_key(L, None)
            .to_key(O, None)
            .build(),
        // d+open_bracket -> u -> l -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, OpenBracket])
            .to_key(U, None)
            .to_key(L, None)
            .to_key(O, None)
            .build(),
        // k+a -> h -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, A])
            .to_key(H, None)
            .to_key(O, None)
            .build(),
        // d+a -> h -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, A])
            .to_key(H, None)
            .to_key(O, None)
            .build(),
        // s+a -> w -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, A])
            .to_key(W, None)
            .to_key(O, None)
            .build(),
        // l+a -> w -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, A])
            .to_key(W, None)
            .to_key(O, None)
            .build(),
        // k+s -> j -> i
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, S])
            .to_key(J, None)
            .to_key(I, None)
            .build(),
        // d+s -> j -> i
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, S])
            .to_key(J, None)
            .to_key(I, None)
            .build(),
        // s+l -> s -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, L])
            .to_key(S, None)
            .to_key(A, None)
            .build(),
        // k+d -> r -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, D])
            .to_key(R, None)
            .to_key(E, None)
            .build(),
        // s+d -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, D])
            .to_key(O, None)
            .build(),
        // l+d -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, D])
            .to_key(O, None)
            .build(),
        // k+f -> m -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, F])
            .to_key(M, None)
            .to_key(O, None)
            .build(),
        // d+f -> m -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, F])
            .to_key(M, None)
            .to_key(O, None)
            .build(),
        // s+f -> r -> i
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, F])
            .to_key(R, None)
            .to_key(I, None)
            .build(),
        // l+f -> r -> i
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, F])
            .to_key(R, None)
            .to_key(I, None)
            .build(),
        // k+g -> y -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, G])
            .to_key(Y, None)
            .to_key(U, None)
            .build(),
        // d+g -> y -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, G])
            .to_key(Y, None)
            .to_key(U, None)
            .build(),
        // s+g -> z -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, G])
            .to_key(Z, None)
            .to_key(U, None)
            .build(),
        // l+g -> z -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, G])
            .to_key(Z, None)
            .to_key(U, None)
            .build(),
        // k+h -> h -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, H])
            .to_key(H, None)
            .to_key(E, None)
            .build(),
        // d+h -> h -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, H])
            .to_key(H, None)
            .to_key(E, None)
            .build(),
        // s+h -> b -> i
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, H])
            .to_key(B, None)
            .to_key(I, None)
            .build(),
        // l+h -> b -> i
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, H])
            .to_key(B, None)
            .to_key(I, None)
            .build(),
        // k+j -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, J])
            .to_key(A, None)
            .build(),
        // d+j -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, J])
            .to_key(A, None)
            .build(),
        // s+j -> r -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, J])
            .to_key(R, None)
            .to_key(A, None)
            .build(),
        // l+j -> r -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, J])
            .to_key(R, None)
            .to_key(A, None)
            .build(),
        // k+l -> j -> i
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, L])
            .to_key(J, None)
            .to_key(I, None)
            .build(),
        // k+semicolon -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, Semicolon])
            .to_key(E, None)
            .build(),
        // // d+semicolon -> e
        //     Manipulator::builder()
        //         .condition(Condition::with_shingeta_mode())
        //         .condition(Condition::with_japanese_input())
        //         .from_simultaneous_keys(vec![D, Semicolon])
        //         .to_key(E, None)
        //         .build(),
        // );
        // d+colon -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, Quote])
            .to_key(E, None)
            .build(),
        // // s+semicolon -> s -> o
        //     Manipulator::builder()
        //         .condition(Condition::with_shingeta_mode())
        //         .condition(Condition::with_japanese_input())
        //         .from_simultaneous_keys(vec![S, Semicolon])
        //         .to_key(S, None)
        //         .to_key(O, None)
        //         .build(),
        // );
        // s+colon -> s -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, Quote])
            .to_key(S, None)
            .to_key(O, None)
            .build(),
        // l+semicolon -> s -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, Semicolon])
            .to_key(S, None)
            .to_key(O, None)
            .build(),
        // k+z -> d -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, Z])
            .to_key(D, None)
            .to_key(U, None)
            .build(),
        // d+z -> d -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, Z])
            .to_key(D, None)
            .to_key(U, None)
            .build(),
        // s+z -> z -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, Z])
            .to_key(Z, None)
            .to_key(E, None)
            .build(),
        // l+z -> z -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, Z])
            .to_key(Z, None)
            .to_key(E, None)
            .build(),
        // k+x -> z -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, X])
            .to_key(Z, None)
            .to_key(O, None)
            .build(),
        // d+x -> z -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, X])
            .to_key(Z, None)
            .to_key(O, None)
            .build(),
        // s+x -> z -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, X])
            .to_key(Z, None)
            .to_key(A, None)
            .build(),
        // l+x -> z -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, X])
            .to_key(Z, None)
            .to_key(A, None)
            .build(),
        // k+c -> b -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, C])
            .to_key(B, None)
            .to_key(O, None)
            .build(),
        // d+c -> b -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, C])
            .to_key(B, None)
            .to_key(O, None)
            .build(),
        // s+c -> g -> i
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, C])
            .to_key(G, None)
            .to_key(I, None)
            .build(),
        // l+c -> g -> i
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, C])
            .to_key(G, None)
            .to_key(I, None)
            .build(),
        // k+v -> m -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, V])
            .to_key(M, None)
            .to_key(U, None)
            .build(),
        // d+v -> m -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, V])
            .to_key(M, None)
            .to_key(U, None)
            .build(),
        // s+v -> r -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, V])
            .to_key(R, None)
            .to_key(O, None)
            .build(),
        // l+v -> r -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, V])
            .to_key(R, None)
            .to_key(O, None)
            .build(),
        // k+b -> f -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, B])
            .to_key(F, None)
            .to_key(O, None)
            .build(),
        // d+b -> f -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, B])
            .to_key(F, None)
            .to_key(O, None)
            .build(),
        // s+b -> n -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, B])
            .to_key(N, None)
            .to_key(U, None)
            .build(),
        // l+b -> n -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, B])
            .to_key(N, None)
            .to_key(U, None)
            .build(),
        // k+n -> s -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, N])
            .to_key(S, None)
            .to_key(E, None)
            .build(),
        // d+n -> s -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, N])
            .to_key(S, None)
            .to_key(E, None)
            .build(),
        // s+n -> w -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, N])
            .to_key(W, None)
            .to_key(A, None)
            .build(),
        // l+n -> w -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, N])
            .to_key(W, None)
            .to_key(A, None)
            .build(),
        // k+m -> n -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, M])
            .to_key(N, None)
            .to_key(E, None)
            .build(),
        // d+m -> n -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, M])
            .to_key(N, None)
            .to_key(E, None)
            .build(),
        // s+m -> d -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, M])
            .to_key(D, None)
            .to_key(A, None)
            .build(),
        // l+m -> d -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, M])
            .to_key(D, None)
            .to_key(A, None)
            .build(),
        // k+comma -> b -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, Comma])
            .to_key(B, None)
            .to_key(E, None)
            .build(),
        // d+comma -> b -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, Comma])
            .to_key(B, None)
            .to_key(E, None)
            .build(),
        // s+comma -> p -> i
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, Comma])
            .to_key(P, None)
            .to_key(I, None)
            .build(),
        // l+comma -> p -> i
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, Comma])
            .to_key(P, None)
            .to_key(I, None)
            .build(),
        // k+period -> p -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, Period])
            .to_key(P, None)
            .to_key(U, None)
            .build(),
        // d+period -> p -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, Period])
            .to_key(P, None)
            .to_key(U, None)
            .build(),
        // s+period -> p -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, Period])
            .to_key(P, None)
            .to_key(O, None)
            .build(),
        // l+period -> p -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, Period])
            .to_key(P, None)
            .to_key(O, None)
            .build(),
        // k+slash -> v -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![K, Slash])
            .to_key(V, None)
            .to_key(U, None)
            .build(),
        // d+slash -> v -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![D, Slash])
            .to_key(V, None)
            .to_key(U, None)
            .build(),
        // s+slash -> c -> h -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![S, Slash])
            .to_key(C, None)
            .to_key(H, None)
            .to_key(E, None)
            .build(),
        // l+slash -> c -> h -> e
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![L, Slash])
            .to_key(C, None)
            .to_key(H, None)
            .to_key(E, None)
            .build(),
        // i+1 -> l -> y -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![I, Key1])
            .to_key(L, None)
            .to_key(Y, None)
            .to_key(U, None)
            .build(),
        // i+2 -> b -> y -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![I, Key2])
            .to_key(B, None)
            .to_key(Y, None)
            .to_key(A, None)
            .build(),
        // i+3 -> b -> y -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![I, Key3])
            .to_key(B, None)
            .to_key(Y, None)
            .to_key(U, None)
            .build(),
        // i+4 -> b -> y -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![I, Key4])
            .to_key(B, None)
            .to_key(Y, None)
            .to_key(O, None)
            .build(),
        // i+q -> h -> y -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![I, Q])
            .to_key(H, None)
            .to_key(Y, None)
            .to_key(U, None)
            .build(),
        // i+w -> s -> h -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![I, W])
            .to_key(S, None)
            .to_key(H, None)
            .to_key(U, None)
            .build(),
        // i+e -> s -> h -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![I, E])
            .to_key(S, None)
            .to_key(H, None)
            .to_key(O, None)
            .build(),
        // i+r -> k -> y -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![I, R])
            .to_key(K, None)
            .to_key(Y, None)
            .to_key(U, None)
            .build(),
        // i+t -> c -> h -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![I, T])
            .to_key(C, None)
            .to_key(H, None)
            .to_key(U, None)
            .build(),
        // i+a -> h -> y -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![I, A])
            .to_key(H, None)
            .to_key(Y, None)
            .to_key(O, None)
            .build(),
        // i+f -> k -> y -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![I, F])
            .to_key(K, None)
            .to_key(Y, None)
            .to_key(O, None)
            .build(),
        // i+g -> c -> h -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![I, G])
            .to_key(C, None)
            .to_key(H, None)
            .to_key(O, None)
            .build(),
        // i+z -> h -> y -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![I, Z])
            .to_key(H, None)
            .to_key(Y, None)
            .to_key(A, None)
            .build(),
        // i+c -> s -> h -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![I, C])
            .to_key(S, None)
            .to_key(H, None)
            .to_key(A, None)
            .build(),
        // i+v -> k -> y -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![I, V])
            .to_key(K, None)
            .to_key(Y, None)
            .to_key(A, None)
            .build(),
        // i+b -> c -> h -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![I, B])
            .to_key(C, None)
            .to_key(H, None)
            .to_key(A, None)
            .build(),
        // o+1 -> l -> y -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![O, Key1])
            .to_key(L, None)
            .to_key(Y, None)
            .to_key(O, None)
            .build(),
        // o+2 -> p -> y -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![O, Key2])
            .to_key(P, None)
            .to_key(Y, None)
            .to_key(A, None)
            .build(),
        // o+3 -> p -> y -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![O, Key3])
            .to_key(P, None)
            .to_key(Y, None)
            .to_key(U, None)
            .build(),
        // o+4 -> p -> y -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![O, Key4])
            .to_key(P, None)
            .to_key(Y, None)
            .to_key(O, None)
            .build(),
        // o+q -> r -> y -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![O, Q])
            .to_key(R, None)
            .to_key(Y, None)
            .to_key(U, None)
            .build(),
        // o+w -> j -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![O, W])
            .to_key(J, None)
            .to_key(U, None)
            .build(),
        // o+e -> j -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![O, E])
            .to_key(J, None)
            .to_key(O, None)
            .build(),
        // o+r -> g -> y -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![O, R])
            .to_key(G, None)
            .to_key(Y, None)
            .to_key(U, None)
            .build(),
        // o+t -> n -> y -> u
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![O, T])
            .to_key(N, None)
            .to_key(Y, None)
            .to_key(U, None)
            .build(),
        // o+a -> r -> y -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![O, A])
            .to_key(R, None)
            .to_key(Y, None)
            .to_key(O, None)
            .build(),
        // o+f -> g -> y -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![O, F])
            .to_key(G, None)
            .to_key(Y, None)
            .to_key(O, None)
            .build(),
        // o+g -> n -> y -> o
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![O, G])
            .to_key(N, None)
            .to_key(Y, None)
            .to_key(O, None)
            .build(),
        // o+z -> r -> y -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![O, Z])
            .to_key(R, None)
            .to_key(Y, None)
            .to_key(A, None)
            .build(),
        // o+c -> j -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![O, C])
            .to_key(J, None)
            .to_key(A, None)
            .build(),
        // o+v -> g -> y -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![O, V])
            .to_key(G, None)
            .to_key(Y, None)
            .to_key(A, None)
            .build(),
        // o+b -> n -> y -> a
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![O, B])
            .to_key(N, None)
            .to_key(Y, None)
            .to_key(A, None)
            .build(),
        // r+g -> slash
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![R, G])
            .to_key(Slash, None)
            .build(),
        // r+f -> slash
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![R, F])
            .to_key(Slash, None)
            .build(),
        // f+v -> shift+1
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![F, V])
            .to_key(Key1, Some(vec![Shift]))
            .build(),
        // f+b -> shift+1
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![F, B])
            .to_key(Key1, Some(vec![Shift]))
            .build(),
        // f+g -> close_bracket -> backslash
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![F, G])
            .to_key(CloseBracket, None)
            .to_key(Backslash, None)
            .build(),
        // u+h -> international3
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![U, H])
            .to_key(International3, None)
            .build(),
        // j+n -> shift+slash
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![J, N])
            .to_key(Slash, Some(vec![Shift]))
            .build(),
        // h+j -> shift+8 -> shift+9
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_simultaneous_keys(vec![H, J])
            .to_key(Key8, Some(vec![Shift]))
            .to_key(Key9, Some(vec![Shift]))
            .build(),
        // ============================================================================
        // SHINGETA LAYOUT MAPPINGS - SINGLE KEY MAPPINGS
        // ============================================================================

        // ============================================================================
        // SHINGETA LAYOUT MAPPINGS - SINGLE KEY MAPPINGS
        // ============================================================================
        // Q -> "-" (hyphen)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(Q)
            .to_key(Hyphen, None)
            .build(),
        // W -> "ni" (n + i)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(W)
            .to_key(N, None)
            .to_key(I, None)
            .build(),
        // E -> "ha" (h + a)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(E)
            .to_key(H, None)
            .to_key(A, None)
            .build(),
        // R -> "," (comma)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(R)
            .to_key(Comma, None)
            .build(),
        // T -> "chi" (c + h + i)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(T)
            .to_key(C, None)
            .to_key(H, None)
            .to_key(I, None)
            .build(),
        // Y -> "gu" (g + u)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(Y)
            .to_key(G, None)
            .to_key(U, None)
            .build(),
        // U -> "ba" (b + a)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(U)
            .to_key(B, None)
            .to_key(A, None)
            .build(),
        // I -> "ko" (k + o)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(I)
            .to_key(K, None)
            .to_key(O, None)
            .build(),
        // O -> "ga" (g + a)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(O)
            .to_key(G, None)
            .to_key(A, None)
            .build(),
        // P -> "hi" (h + i)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(P)
            .to_key(H, None)
            .to_key(I, None)
            .build(),
        // OpenBracket -> "ge" (g + e)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(OpenBracket)
            .to_key(G, None)
            .to_key(E, None)
            .build(),
        // A -> "no" (n + o)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(A)
            .to_key(N, None)
            .to_key(O, None)
            .build(),
        // S -> "to" (t + o)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(S)
            .to_key(T, None)
            .to_key(O, None)
            .build(),
        // D -> "ka" (k + a)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(D)
            .to_key(K, None)
            .to_key(A, None)
            .build(),
        // F -> "nn" (n + n)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(F)
            .to_key(N, None)
            .to_key(N, None)
            .build(),
        // G -> "ltu" (small tsu: l + t + u)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(G)
            .to_key(L, None)
            .to_key(T, None)
            .to_key(U, None)
            .build(),
        // H -> "ku" (k + u)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(H)
            .to_key(K, None)
            .to_key(U, None)
            .build(),
        // J -> "u"
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(J)
            .to_key(U, None)
            .build(),
        // K -> "i"
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(K)
            .to_key(I, None)
            .build(),
        // L -> "si" (s + i)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(L)
            .to_key(S, None)
            .to_key(I, None)
            .build(),
        // // ; -> "na" (n + a)
        //     Manipulator::builder()
        //         .condition(Condition::with_shingeta_mode())
        //         .condition(Condition::with_japanese_input())
        //         .from_key(Semicolon)
        //         .to_key(N, None)
        //         .to_key(A, None)
        //         .build(),
        // );

        // // ' -> backspace
        //     Manipulator::builder()
        //         .condition(Condition::with_shingeta_mode())
        //         .condition(Condition::with_japanese_input())
        //         .from_key(Quote)
        //         .to_key(DeleteOrBackspace, None)
        //         .build(),
        // );

        // Cmd+: -> :
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key_with_modifiers(Quote, FromModifier::Mandatory(vec![Cmd]))
            .to_key(Quote, None)
            .build(),
        // ' -> "na" (n + a)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(Quote)
            .to_key(N, None)
            .to_key(A, None)
            .build(),
        // Z -> "su" (s + u)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(Z)
            .to_key(S, None)
            .to_key(U, None)
            .build(),
        // X -> "ma" (m + a)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(X)
            .to_key(M, None)
            .to_key(A, None)
            .build(),
        // C -> "ki" (k + i)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(C)
            .to_key(K, None)
            .to_key(I, None)
            .build(),
        // V -> "ru" (r + u)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(V)
            .to_key(R, None)
            .to_key(U, None)
            .build(),
        // B -> "tsu" (t + s + u)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(B)
            .to_key(T, None)
            .to_key(S, None)
            .to_key(U, None)
            .build(),
        // N -> "te" (t + e)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(N)
            .to_key(T, None)
            .to_key(E, None)
            .build(),
        // M -> "ta" (t + a)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(M)
            .to_key(T, None)
            .to_key(A, None)
            .build(),
        // , -> "de" (d + e)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(Comma)
            .to_key(D, None)
            .to_key(E, None)
            .build(),
        // . -> "."
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(Period)
            .to_key(Period, None)
            .build(),
        // / -> "bu" (b + u)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(Slash)
            .to_key(B, None)
            .to_key(U, None)
            .build(),
        // International1 (無変換) -> International3 + Option (katakana input mode)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .condition(Condition::with_japanese_input())
            .from_key(International1)
            .to_key(International3, Some(vec![Opt]))
            .build(),
    ];

    for (from, to) in [
        (LeftArrow, H),
        (DownArrow, J),
        (UpArrow, K),
        (RightArrow, L),
    ]
    .into_iter()
    {
        manipulators.push(
            Manipulator::builder()
                .condition(Condition::with_shingeta_mode())
                .condition(Condition::with_japanese_input())
                .from_key(from)
                .to_key(Z, None)
                .to_key(to, None)
                .build(),
        );
    }

    manipulators
}
