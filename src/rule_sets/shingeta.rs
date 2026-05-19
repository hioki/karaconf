use crate::karabiner_data::{
    Condition, FromModifier,
    KeyCode::{self, *},
    Manipulator,
    ModifierKey::{self, *},
};

fn dj(from: (KeyCode, KeyCode), to: Vec<(KeyCode, Option<Vec<ModifierKey>>)>) -> Manipulator {
    let mut builder = Manipulator::builder()
        .condition(Condition::with_shingeta_mode())
        .from_simultaneous_keys(vec![from.0, from.1]);
    for (key_code, modifiers) in to {
        builder = builder.to_key(key_code, modifiers);
    }
    builder.build()
}

fn sn(from: KeyCode, to: Vec<(KeyCode, Option<Vec<ModifierKey>>)>) -> Manipulator {
    let mut builder = Manipulator::builder()
        .condition(Condition::with_shingeta_mode())
        .from_key(from);
    for (key_code, modifiers) in to {
        builder = builder.to_key(key_code, modifiers);
    }
    builder.build()
}

// ============================================================================
// SHINGETA LAYOUT MAPPINGS - SIMULTANEOUS KEY MAPPINGS
// ============================================================================
// NOTE: Simultaneous key mappings MUST come before single key mappings
//       to ensure proper priority in Karabiner-Elements
pub fn manipulators() -> Vec<Manipulator> {
    let mut manipulators = vec![
        dj((K, Key1), vec![(L, None), (A, None)]),
        dj((L, Key1), vec![(L, None), (Y, None), (A, None)]),
        dj((K, Key2), vec![(L, None), (I, None)]),
        dj((L, Key2), vec![(M, None), (Y, None), (A, None)]),
        dj((K, Key3), vec![(L, None), (U, None)]),
        dj((L, Key3), vec![(M, None), (Y, None), (U, None)]),
        dj((K, Key4), vec![(L, None), (E, None)]),
        dj((L, Key4), vec![(M, None), (Y, None), (O, None)]),
        dj((K, Key5), vec![(L, None), (O, None)]),
        dj((L, Key5), vec![(L, None), (W, None), (A, None)]),
        // dj((D, Key7), vec![(Comma, Some(vec![Opt]))]),
        // dj((S, Key7), vec![(Period, Some(vec![Opt]))]),
        // dj((D, Key8), vec![(CloseBracket, None)]),
        // dj((S, Key8), vec![(Key8, Some(vec![Shift]))]),
        // dj((D, Key9), vec![(Backslash, None)]),
        // dj((S, Key9), vec![(Key9, Some(vec![Shift]))]),
        // dj((D, Key0), vec![(Semicolon, None)]),
        // dj((S, Key0), vec![(Quote, None)]),
        // dj((D, Hyphen), vec![(OpenBracket, None)]),
        // dj((S, Hyphen), vec![(Quote, Some(vec![Shift]))]),
        dj((K, Q), vec![(F, None), (A, None)]),
        dj((L, Q), vec![(D, None), (I, None)]),
        dj((K, W), vec![(G, None), (O, None)]),
        dj((L, W), vec![(M, None), (E, None)]),
        dj((K, E), vec![(F, None), (U, None)]),
        dj((L, E), vec![(K, None), (E, None)]),
        dj((K, R), vec![(F, None), (I, None)]),
        dj((L, R), vec![(T, None), (H, None), (I, None)]),
        dj((K, T), vec![(F, None), (E, None)]),
        dj((L, T), vec![(D, None), (H, None), (I, None)]),
        dj((D, Y), vec![(W, None), (I, None)]),
        dj((S, Y), vec![(S, None), (H, None), (E, None)]),
        dj((D, U), vec![(P, None), (A, None)]),
        dj((S, U), vec![(P, None), (E, None)]),
        dj((D, I), vec![(Y, None), (O, None)]),
        dj((S, I), vec![(D, None), (O, None)]),
        dj((D, O), vec![(M, None), (I, None)]),
        dj((S, O), vec![(Y, None), (A, None)]),
        dj((D, P), vec![(W, None), (E, None)]),
        dj((S, P), vec![(J, None), (E, None)]),
        dj((D, OpenBracket), vec![(U, None), (L, None), (O, None)]),
        dj((K, A), vec![(H, None), (O, None)]),
        dj((L, A), vec![(W, None), (O, None)]),
        dj((K, S), vec![(J, None), (I, None)]),
        dj((S, L), vec![(S, None), (A, None)]),
        dj((K, D), vec![(R, None), (E, None)]),
        dj((L, D), vec![(O, None)]),
        dj((K, F), vec![(M, None), (O, None)]),
        dj((L, F), vec![(R, None), (I, None)]),
        dj((K, G), vec![(Y, None), (U, None)]),
        dj((L, G), vec![(Z, None), (U, None)]),
        dj((D, H), vec![(H, None), (E, None)]),
        dj((S, H), vec![(B, None), (I, None)]),
        dj((D, J), vec![(A, None)]),
        dj((S, J), vec![(R, None), (A, None)]),
        dj((D, Quote), vec![(E, None)]),
        dj((S, Quote), vec![(S, None), (O, None)]),
        dj((K, Z), vec![(D, None), (U, None)]),
        dj((L, Z), vec![(Z, None), (E, None)]),
        dj((K, X), vec![(Z, None), (O, None)]),
        dj((L, X), vec![(Z, None), (A, None)]),
        dj((K, C), vec![(B, None), (O, None)]),
        dj((L, C), vec![(G, None), (I, None)]),
        dj((K, V), vec![(M, None), (U, None)]),
        dj((L, V), vec![(R, None), (O, None)]),
        dj((K, B), vec![(F, None), (O, None)]),
        dj((L, B), vec![(N, None), (U, None)]),
        dj((D, N), vec![(S, None), (E, None)]),
        dj((S, N), vec![(W, None), (A, None)]),
        dj((D, M), vec![(N, None), (E, None)]),
        dj((S, M), vec![(D, None), (A, None)]),
        dj((D, Comma), vec![(B, None), (E, None)]),
        dj((S, Comma), vec![(P, None), (I, None)]),
        dj((D, Period), vec![(P, None), (U, None)]),
        dj((S, Period), vec![(P, None), (O, None)]),
        dj((D, Slash), vec![(V, None), (U, None)]),
        dj((S, Slash), vec![(C, None), (H, None), (E, None)]),
        dj((I, Key1), vec![(L, None), (Y, None), (U, None)]),
        dj((I, Key2), vec![(B, None), (Y, None), (A, None)]),
        dj((I, Key3), vec![(B, None), (Y, None), (U, None)]),
        dj((I, Key4), vec![(B, None), (Y, None), (O, None)]),
        dj((I, Q), vec![(H, None), (Y, None), (U, None)]),
        dj((I, W), vec![(S, None), (H, None), (U, None)]),
        dj((I, E), vec![(S, None), (H, None), (O, None)]),
        dj((I, R), vec![(K, None), (Y, None), (U, None)]),
        dj((I, T), vec![(C, None), (H, None), (U, None)]),
        dj((I, A), vec![(H, None), (Y, None), (O, None)]),
        dj((I, F), vec![(K, None), (Y, None), (O, None)]),
        dj((I, G), vec![(C, None), (H, None), (O, None)]),
        dj((I, Z), vec![(H, None), (Y, None), (A, None)]),
        dj((I, X), vec![(V, None), (U, None), (L, None), (A, None)]),
        dj((I, C), vec![(S, None), (H, None), (A, None)]),
        dj((I, V), vec![(K, None), (Y, None), (A, None)]),
        dj((I, B), vec![(C, None), (H, None), (A, None)]),
        dj((O, Key1), vec![(L, None), (Y, None), (O, None)]),
        dj((O, Key2), vec![(P, None), (Y, None), (A, None)]),
        dj((O, Key3), vec![(P, None), (Y, None), (U, None)]),
        dj((O, Key4), vec![(P, None), (Y, None), (O, None)]),
        dj((O, Q), vec![(R, None), (Y, None), (U, None)]),
        dj((O, W), vec![(J, None), (U, None)]),
        dj((O, E), vec![(J, None), (O, None)]),
        dj((O, R), vec![(G, None), (Y, None), (U, None)]),
        dj((O, T), vec![(N, None), (Y, None), (U, None)]),
        dj((O, A), vec![(R, None), (Y, None), (O, None)]),
        dj((O, F), vec![(G, None), (Y, None), (O, None)]),
        dj((O, G), vec![(N, None), (Y, None), (O, None)]),
        dj((O, Z), vec![(R, None), (Y, None), (A, None)]),
        dj((O, X), vec![(D, None), (H, None), (U, None)]),
        dj((O, C), vec![(J, None), (A, None)]),
        dj((O, V), vec![(G, None), (Y, None), (A, None)]),
        dj((O, B), vec![(N, None), (Y, None), (A, None)]),
        // dj((R, G), vec![(Slash, None)]),
        // dj((R, F), vec![(Slash, None)]),
        // dj((F, V), vec![(Key1, Some(vec![Shift]))]),
        // dj((F, B), vec![(Key1, Some(vec![Shift]))]),
        // dj((F, G), vec![(CloseBracket, None), (Backslash, None)]),
        // dj((U, H), vec![(International3, None)]),
        // dj((J, N), vec![(Slash, Some(vec![Shift]))]),
        // dj((H, J), vec![(Key8, Some(vec![Shift])), (Key9, Some(vec![Shift]))]),
        // ============================================================================
        // SHINGETA LAYOUT MAPPINGS - SINGLE KEY MAPPINGS
        // ============================================================================

        // ============================================================================
        // SHINGETA LAYOUT MAPPINGS - SINGLE KEY MAPPINGS
        // ============================================================================
        sn(Q, vec![(Hyphen, None)]),
        // W -> "ni" (n + i)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .from_key(W)
            .to_key(N, None)
            .to_key(I, None)
            .build(),
        // E -> "ha" (h + a)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .from_key(E)
            .to_key(H, None)
            .to_key(A, None)
            .build(),
        // R -> "," (comma)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .from_key(R)
            .to_key(Comma, None)
            .build(),
        // T -> "chi" (c + h + i)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .from_key(T)
            .to_key(C, None)
            .to_key(H, None)
            .to_key(I, None)
            .build(),
        // Y -> "gu" (g + u)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .from_key(Y)
            .to_key(G, None)
            .to_key(U, None)
            .build(),
        // U -> "ba" (b + a)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .from_key(U)
            .to_key(B, None)
            .to_key(A, None)
            .build(),
        // I -> "ko" (k + o)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .from_key(I)
            .to_key(K, None)
            .to_key(O, None)
            .build(),
        // O -> "ga" (g + a)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .from_key(O)
            .to_key(G, None)
            .to_key(A, None)
            .build(),
        // P -> "hi" (h + i)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .from_key(P)
            .to_key(H, None)
            .to_key(I, None)
            .build(),
        // OpenBracket -> "ge" (g + e)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .from_key(OpenBracket)
            .to_key(G, None)
            .to_key(E, None)
            .build(),
        // A -> "no" (n + o)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .from_key(A)
            .to_key(N, None)
            .to_key(O, None)
            .build(),
        // S -> "to" (t + o)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .from_key(S)
            .to_key(T, None)
            .to_key(O, None)
            .build(),
        // D -> "ka" (k + a)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .from_key(D)
            .to_key(K, None)
            .to_key(A, None)
            .build(),
        // F -> "nn" (n + n)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .from_key(F)
            .to_key(N, None)
            .to_key(N, None)
            .build(),
        // G -> "ltu" (small tsu: l + t + u)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .from_key(G)
            .to_key(L, None)
            .to_key(T, None)
            .to_key(U, None)
            .build(),
        // H -> "ku" (k + u)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .from_key(H)
            .to_key(K, None)
            .to_key(U, None)
            .build(),
        // J -> "u"
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .from_key(J)
            .to_key(U, None)
            .build(),
        // K -> "i"
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .from_key(K)
            .to_key(I, None)
            .build(),
        // L -> "si" (s + i)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
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
            .from_key_with_modifiers(Quote, FromModifier::Mandatory(vec![Cmd]))
            .to_key(Quote, None)
            .build(),
        // ' -> "na" (n + a)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .from_key(Quote)
            .to_key(N, None)
            .to_key(A, None)
            .build(),
        // Z -> "su" (s + u)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .from_key(Z)
            .to_key(S, None)
            .to_key(U, None)
            .build(),
        // X -> "ma" (m + a)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .from_key(X)
            .to_key(M, None)
            .to_key(A, None)
            .build(),
        // C -> "ki" (k + i)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .from_key(C)
            .to_key(K, None)
            .to_key(I, None)
            .build(),
        // V -> "ru" (r + u)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .from_key(V)
            .to_key(R, None)
            .to_key(U, None)
            .build(),
        // B -> "tsu" (t + s + u)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .from_key(B)
            .to_key(T, None)
            .to_key(S, None)
            .to_key(U, None)
            .build(),
        // N -> "te" (t + e)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .from_key(N)
            .to_key(T, None)
            .to_key(E, None)
            .build(),
        // M -> "ta" (t + a)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .from_key(M)
            .to_key(T, None)
            .to_key(A, None)
            .build(),
        // , -> "de" (d + e)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .from_key(Comma)
            .to_key(D, None)
            .to_key(E, None)
            .build(),
        // . -> "."
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .from_key(Period)
            .to_key(Period, None)
            .build(),
        // / -> "bu" (b + u)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
            .from_key(Slash)
            .to_key(B, None)
            .to_key(U, None)
            .build(),
        // International1 (無変換) -> International3 + Option (katakana input mode)
        Manipulator::builder()
            .condition(Condition::with_shingeta_mode())
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
                .from_key(from)
                .to_key(Z, None)
                .to_key(to, None)
                .build(),
        );
    }

    manipulators
}
