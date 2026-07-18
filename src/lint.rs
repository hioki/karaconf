//! Detects manipulators that can never fire.
//!
//! Karabiner-Elements evaluates manipulators in order and the first match
//! wins. A manipulator is unreachable when an earlier one matches a superset
//! of its key events: same `from` key, condition set that is a subset of the
//! later one's, and a modifier spec that covers the later one's.

use crate::display;
use crate::karabiner_data::{From, FromModifier, Manipulator, ModifierKey};
use std::collections::BTreeSet;

#[derive(Debug, PartialEq, Eq)]
pub enum FindingKind {
    /// Identical match surface and identical output.
    Duplicate,
    /// The later manipulator can never fire because the earlier one wins.
    Shadowed,
}

pub struct Finding {
    pub kind: FindingKind,
    pub earlier: RuleRef,
    pub later: RuleRef,
    pub from_desc: String,
}

pub struct RuleRef {
    pub ruleset: String,
    pub index: usize,
}

impl std::fmt::Display for Finding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            FindingKind::Duplicate => write!(
                f,
                "[{}#{}] と [{}#{}] は同一の定義です ({})",
                self.earlier.ruleset,
                self.earlier.index,
                self.later.ruleset,
                self.later.index,
                self.from_desc,
            ),
            FindingKind::Shadowed => write!(
                f,
                "[{}#{}] ({}) は先行する [{}#{}] に覆われていて発火しません",
                self.later.ruleset,
                self.later.index,
                self.from_desc,
                self.earlier.ruleset,
                self.earlier.index,
            ),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
enum ModSpec {
    /// No modifiers field: matches only an unmodified key press.
    Unmodified,
    Optional(BTreeSet<String>),
    Mandatory(BTreeSet<String>),
}

impl ModSpec {
    fn of(from: &From) -> Option<ModSpec> {
        match from {
            From::Single { modifiers, .. } => Some(match modifiers {
                None => ModSpec::Unmodified,
                Some(FromModifier::Optional(mods)) => ModSpec::Optional(mod_names(mods)),
                Some(FromModifier::Mandatory(mods)) => ModSpec::Mandatory(mod_names(mods)),
            }),
            // Simultaneous keys have no modifier spec.
            From::Simultaneous { .. } => None,
        }
    }

    /// Does every key event matched by `later` also match `self`?
    fn covers(&self, later: &ModSpec) -> bool {
        if self == later {
            return true;
        }
        match self {
            ModSpec::Optional(mods) if mods.contains("any") => true,
            ModSpec::Optional(mods) => match later {
                ModSpec::Unmodified => true,
                ModSpec::Optional(other) | ModSpec::Mandatory(other) => other.is_subset(mods),
            },
            _ => false,
        }
    }
}

fn mod_names(modifiers: &[ModifierKey]) -> BTreeSet<String> {
    modifiers
        .iter()
        .map(|m| json_name(m).trim_matches('"').to_string())
        .collect::<BTreeSet<_>>()
}

fn json_name<T: serde::Serialize>(value: &T) -> String {
    serde_json::to_string(value).expect("serialization should not fail")
}

/// Identity of the `from` key(s), ignoring modifiers.
fn from_signature(from: &From) -> String {
    match from {
        From::Single { key_code, .. } => format!("single:{}", json_name(key_code)),
        From::Simultaneous { simultaneous, .. } => {
            let mut keys: Vec<String> = simultaneous
                .iter()
                .map(|s| json_name(&s.key_code))
                .collect();
            keys.sort();
            format!("simultaneous:{}", keys.join(","))
        }
    }
}

fn condition_set(manipulator: &Manipulator) -> BTreeSet<String> {
    manipulator
        .conditions
        .iter()
        .flatten()
        .map(json_name)
        .collect()
}

struct Entry<'a> {
    ruleset: &'a str,
    index: usize,
    manipulator: &'a Manipulator,
    from_signature: String,
    conditions: BTreeSet<String>,
    mod_spec: Option<ModSpec>,
}

pub fn lint(rulesets: &[(&str, Vec<Manipulator>)]) -> Vec<Finding> {
    let entries: Vec<Entry> = rulesets
        .iter()
        .flat_map(|(name, manipulators)| {
            manipulators
                .iter()
                .enumerate()
                .map(move |(index, m)| Entry {
                    ruleset: name,
                    index,
                    manipulator: m,
                    from_signature: from_signature(&m.from),
                    conditions: condition_set(m),
                    mod_spec: ModSpec::of(&m.from),
                })
        })
        .collect();

    let mut findings = Vec::new();
    for (j, later) in entries.iter().enumerate() {
        for earlier in &entries[..j] {
            if earlier.from_signature != later.from_signature {
                continue;
            }
            if !earlier.conditions.is_subset(&later.conditions) {
                continue;
            }
            let mods_covered = match (&earlier.mod_spec, &later.mod_spec) {
                (Some(e), Some(l)) => e.covers(l),
                (None, None) => true, // simultaneous vs simultaneous with same keys
                _ => false,
            };
            if !mods_covered {
                continue;
            }
            let identical = earlier.conditions == later.conditions
                && earlier.mod_spec == later.mod_spec
                && json_name(&earlier.manipulator.to) == json_name(&later.manipulator.to);
            findings.push(Finding {
                kind: if identical {
                    FindingKind::Duplicate
                } else {
                    FindingKind::Shadowed
                },
                earlier: RuleRef {
                    ruleset: earlier.ruleset.to_string(),
                    index: earlier.index,
                },
                later: RuleRef {
                    ruleset: later.ruleset.to_string(),
                    index: later.index,
                },
                from_desc: display::from_label(&later.manipulator.from),
            });
            break; // one finding per unreachable manipulator is enough
        }
    }
    findings
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::karabiner_data::{Condition, KeyCode::*, ModifierKey::*};

    fn ruleset(manipulators: Vec<Manipulator>) -> Vec<(&'static str, Vec<Manipulator>)> {
        vec![("test", manipulators)]
    }

    #[test]
    fn optional_any_shadows_later_mandatory() {
        let findings = lint(&ruleset(vec![
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key_with_modifiers(X, FromModifier::Optional(vec![Any]))
                .to_key(V, Some(vec![Cmd, Shift]))
                .build(),
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key_with_modifiers(X, FromModifier::Mandatory(vec![Cmd]))
                .to_key(V, None)
                .build(),
        ]));
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].kind, FindingKind::Shadowed);
    }

    #[test]
    fn unconditioned_rule_shadows_conditioned_one() {
        let findings = lint(&ruleset(vec![
            Manipulator::builder()
                .from_key(Semicolon)
                .to_key(ReturnOrEnter, None)
                .build(),
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key(Semicolon)
                .to_key(F, Some(vec![Ctrl]))
                .build(),
        ]));
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].kind, FindingKind::Shadowed);
    }

    #[test]
    fn more_specific_earlier_rule_does_not_shadow() {
        // App-specific rule first, global rule later: both reachable.
        let findings = lint(&ruleset(vec![
            Manipulator::builder()
                .conditions(vec![
                    Condition::on_app(crate::karabiner_data::BundleIdentifier::ITerm2),
                    Condition::with_vk1(),
                ])
                .from_key(O)
                .to_key(P, Some(vec![Ctrl]))
                .build(),
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key(O)
                .to_key(Tab, Some(vec![Ctrl, Shift]))
                .build(),
        ]));
        assert!(findings.is_empty());
    }

    #[test]
    fn unmodified_does_not_cover_mandatory() {
        let findings = lint(&ruleset(vec![
            Manipulator::builder()
                .condition(Condition::with_vk2())
                .from_key(Key1)
                .to_key(VolumeDecrement, None)
                .build(),
            Manipulator::builder()
                .condition(Condition::with_vk2())
                .from_key_with_modifiers(Key1, FromModifier::Mandatory(vec![Ctrl]))
                .to_command("echo hi")
                .build(),
        ]));
        assert!(findings.is_empty());
    }

    #[test]
    fn identical_rules_reported_as_duplicate() {
        let build = || {
            Manipulator::builder()
                .condition(Condition::with_vk1())
                .from_key(H)
                .to_key(LeftArrow, None)
                .build()
        };
        let findings = lint(&ruleset(vec![build(), build()]));
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].kind, FindingKind::Duplicate);
    }

    #[test]
    fn simultaneous_duplicate_detected_regardless_of_key_order() {
        let findings = lint(&ruleset(vec![
            Manipulator::builder()
                .condition(Condition::with_shingeta_mode())
                .from_simultaneous_keys(vec![K, Key1])
                .to_key(L, None)
                .build(),
            Manipulator::builder()
                .condition(Condition::with_shingeta_mode())
                .from_simultaneous_keys(vec![Key1, K])
                .to_key(A, None)
                .build(),
        ]));
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].kind, FindingKind::Shadowed);
    }
}
