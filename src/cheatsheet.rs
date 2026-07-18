//! Generates a self-contained HTML cheatsheet (cheatsheet.html) from the
//! manipulator definitions: per-layer JIS keyboard grids, per-app tables,
//! and the shingeta layout rendered as kana.

use crate::display::{from_label, key_label, mods_prefix, to_summary, virtual_key_label};
use crate::karabiner_data::{Condition, From, FromModifier, KeyCode, Manipulator, To, VirtualKey};
use crate::lint::Finding;
use std::collections::BTreeMap;

pub fn generate(rulesets: &[(&str, Vec<Manipulator>)], findings: &[Finding]) -> String {
    let mut buckets = Buckets::default();
    for (name, manipulators) in rulesets {
        for manipulator in manipulators {
            buckets.add(name, manipulator);
        }
    }

    let total: usize = rulesets.iter().map(|(_, m)| m.len()).sum();
    let mut html = String::with_capacity(256 * 1024);
    html.push_str(HEADER);
    html.push_str(&format!(
        "<h1>karaconf cheatsheet</h1>\n<p class=\"meta\">全 {} manipulators ／ {} rule sets{}</p>\n",
        total,
        rulesets.len(),
        if findings.is_empty() {
            " ／ lint: 問題なし ✅".to_string()
        } else {
            format!(" ／ <a href=\"#lint\">lint: {} 件 ⚠️</a>", findings.len())
        },
    ));

    render_layer_sections(&mut html, &buckets);
    render_global_section(&mut html, &buckets.global);
    render_app_sections(&mut html, &buckets.apps);
    render_shingeta_section(&mut html, &buckets);
    render_misc_section(&mut html, &buckets.misc);
    render_lint_section(&mut html, findings);

    html.push_str("</main></body>\n");
    html
}

// ---------------------------------------------------------------------------
// Classification
// ---------------------------------------------------------------------------

#[derive(Default)]
struct Buckets<'a> {
    /// No conditions at all.
    global: Vec<Entry<'a>>,
    /// Exactly one condition: vkN = 1.
    layers: BTreeMap<&'static str, Vec<Entry<'a>>>,
    /// Exactly one condition: shingeta_mode = 1.
    shingeta: Vec<Entry<'a>>,
    /// Has a frontmost-application condition, grouped by app name(s).
    apps: BTreeMap<String, Vec<Entry<'a>>>,
    /// Everything else (mixed conditions, input sources, ...).
    misc: Vec<Entry<'a>>,
}

struct Entry<'a> {
    ruleset: &'a str,
    manipulator: &'a Manipulator,
}

impl<'a> Buckets<'a> {
    fn add(&mut self, ruleset: &'a str, manipulator: &'a Manipulator) {
        let entry = Entry {
            ruleset,
            manipulator,
        };
        let conditions: Vec<&Condition> = manipulator.conditions.iter().flatten().collect();

        let app_names: Vec<String> = conditions
            .iter()
            .filter_map(|c| match c {
                Condition::OnApplication {
                    bundle_identifiers, ..
                } => Some(
                    bundle_identifiers
                        .iter()
                        .map(|b| format!("{:?}", b))
                        .collect::<Vec<_>>()
                        .join(" / "),
                ),
                _ => None,
            })
            .collect();
        if !app_names.is_empty() {
            self.apps
                .entry(app_names.join(", "))
                .or_default()
                .push(entry);
            return;
        }

        match conditions.as_slice() {
            [] => self.global.push(entry),
            [Condition::WithVirtualKey { name, value: 1, .. }] => match name {
                VirtualKey::Vk1 => self.layers.entry("VK1").or_default().push(entry),
                VirtualKey::Vk2 => self.layers.entry("VK2").or_default().push(entry),
                VirtualKey::Vk3 => self.layers.entry("VK3").or_default().push(entry),
                VirtualKey::Vk4 => self.layers.entry("VK4").or_default().push(entry),
                VirtualKey::ShingetaMode => self.shingeta.push(entry),
            },
            _ => self.misc.push(entry),
        }
    }
}

fn conditions_label(manipulator: &Manipulator) -> String {
    let parts: Vec<String> = manipulator
        .conditions
        .iter()
        .flatten()
        .map(|c| match c {
            Condition::OnApplication { .. } => String::new(),
            Condition::WithVirtualKey { name, value, .. } => {
                if *value == 1 {
                    virtual_key_label(name).to_string()
                } else {
                    format!("{}オフ", virtual_key_label(name))
                }
            }
            Condition::InputSource { input_sources, .. } => input_sources
                .iter()
                .map(|s| format!("入力={}", s.language))
                .collect::<Vec<_>>()
                .join(","),
        })
        .filter(|s| !s.is_empty())
        .collect();
    if parts.is_empty() {
        "常時".to_string()
    } else {
        parts.join(" + ")
    }
}

// ---------------------------------------------------------------------------
// JIS keyboard grid
// ---------------------------------------------------------------------------

fn keyboard_rows() -> Vec<Vec<KeyCode>> {
    use KeyCode::*;
    vec![
        vec![
            Key1,
            Key2,
            Key3,
            Key4,
            Key5,
            Key6,
            Key7,
            Key8,
            Key9,
            Key0,
            Hyphen,
            EqualSign,
            International3,
        ],
        vec![Q, W, E, R, T, Y, U, I, O, P, OpenBracket, CloseBracket],
        vec![A, S, D, F, G, H, J, K, L, Semicolon, Quote, Backslash],
        vec![Z, X, C, V, B, N, M, Comma, Period, Slash, International1],
    ]
}

fn key_id(key: &KeyCode) -> String {
    // NonUsPound shares the "]" cap with Backslash on JIS boards.
    if matches!(key, KeyCode::NonUsPound) {
        return key_id(&KeyCode::Backslash);
    }
    serde_json::to_string(key).expect("key code serializes")
}

/// cells: key id -> html lines rendered inside the key cap.
fn render_keyboard(
    html: &mut String,
    cells: &BTreeMap<String, Vec<String>>,
    kana_style: bool,
) -> Vec<String> {
    let indents = [0, 14, 22, 34];
    let mut free_keys = Vec::new();
    html.push_str(&format!(
        "<div class=\"kb{}\">\n",
        if kana_style { " kana" } else { "" }
    ));
    for (row, indent) in keyboard_rows().iter().zip(indents) {
        html.push_str(&format!(
            "<div class=\"krow\" style=\"margin-left:{}px\">\n",
            indent
        ));
        for key in row {
            let legend = esc(&key_label(key));
            match cells.get(&key_id(key)) {
                Some(lines) if !lines.is_empty() => {
                    html.push_str(&format!(
                        "<div class=\"key\"><span class=\"legend\">{}</span>{}</div>\n",
                        legend,
                        lines.join("")
                    ));
                }
                _ => {
                    free_keys.push(key_label(key));
                    html.push_str(&format!(
                        "<div class=\"key free\"><span class=\"legend\">{}</span></div>\n",
                        legend
                    ));
                }
            }
        }
        html.push_str("</div>\n");
    }
    html.push_str("</div>\n");
    free_keys
}

// ---------------------------------------------------------------------------
// Sections
// ---------------------------------------------------------------------------

fn render_layer_sections(html: &mut String, buckets: &Buckets) {
    for (layer, entries) in &buckets.layers {
        html.push_str(&format!(
            "<section><h2><span class=\"chip {}\">{}</span> レイヤー <span class=\"count\">{} rules</span></h2>\n",
            layer.to_lowercase(),
            layer,
            entries.len()
        ));

        let mut cells: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut overflow: Vec<&Entry> = Vec::new();
        for entry in entries {
            match &entry.manipulator.from {
                From::Single {
                    key_code,
                    modifiers,
                } if in_grid(key_code) => {
                    let badge = match modifiers {
                        Some(FromModifier::Mandatory(mods)) => {
                            format!("<span class=\"badge\">{}</span>", esc(&mods_prefix(mods)))
                        }
                        _ => String::new(),
                    };
                    cells.entry(key_id(key_code)).or_default().push(format!(
                        "<span class=\"out\">{}{}</span>",
                        badge,
                        esc(&to_summary(entry.manipulator))
                    ));
                }
                _ => overflow.push(entry),
            }
        }
        let free_keys = render_keyboard(html, &cells, false);
        if !free_keys.is_empty() {
            html.push_str(&format!(
                "<p class=\"free-list\">空きキー: {}</p>\n",
                esc(&free_keys.join(" "))
            ));
        }
        render_entry_table(html, &overflow, true);
        html.push_str("</section>\n");
    }
}

fn render_global_section(html: &mut String, entries: &[Entry]) {
    if entries.is_empty() {
        return;
    }
    html.push_str(&format!(
        "<section><h2>常時有効 <span class=\"count\">{} rules</span></h2>\n",
        entries.len()
    ));
    let refs: Vec<&Entry> = entries.iter().collect();
    render_entry_table(html, &refs, false);
    html.push_str("</section>\n");
}

fn render_app_sections(html: &mut String, apps: &BTreeMap<String, Vec<Entry>>) {
    if apps.is_empty() {
        return;
    }
    html.push_str("<section><h2>アプリ別</h2>\n");
    for (app, entries) in apps {
        html.push_str(&format!(
            "<h3>{} <span class=\"count\">{} rules</span></h3>\n",
            esc(app),
            entries.len()
        ));
        let refs: Vec<&Entry> = entries.iter().collect();
        render_entry_table(html, &refs, true);
    }
    html.push_str("</section>\n");
}

fn render_misc_section(html: &mut String, entries: &[Entry]) {
    if entries.is_empty() {
        return;
    }
    html.push_str(&format!(
        "<section><h2>その他の条件 <span class=\"count\">{} rules</span></h2>\n",
        entries.len()
    ));
    let refs: Vec<&Entry> = entries.iter().collect();
    render_entry_table(html, &refs, true);
    html.push_str("</section>\n");
}

fn render_entry_table(html: &mut String, entries: &[&Entry], show_conditions: bool) {
    if entries.is_empty() {
        return;
    }
    html.push_str("<table><thead><tr>");
    if show_conditions {
        html.push_str("<th>条件</th>");
    }
    html.push_str("<th>入力</th><th>出力</th><th>定義元</th></tr></thead><tbody>\n");
    for entry in entries {
        html.push_str("<tr>");
        if show_conditions {
            html.push_str(&format!(
                "<td class=\"cond\">{}</td>",
                esc(&conditions_label(entry.manipulator))
            ));
        }
        html.push_str(&format!(
            "<td class=\"from\">{}</td><td>{}</td><td class=\"src\">{}</td></tr>\n",
            esc(&from_label(&entry.manipulator.from)),
            esc(&to_summary(entry.manipulator)),
            esc(entry.ruleset),
        ));
    }
    html.push_str("</tbody></table>\n");
}

fn render_lint_section(html: &mut String, findings: &[Finding]) {
    if findings.is_empty() {
        return;
    }
    html.push_str(&format!(
        "<section id=\"lint\"><h2>⚠️ lint <span class=\"count\">{} 件</span></h2><ul class=\"lint\">\n",
        findings.len()
    ));
    for finding in findings {
        html.push_str(&format!("<li>{}</li>\n", esc(&finding.to_string())));
    }
    html.push_str("</ul></section>\n");
}

// ---------------------------------------------------------------------------
// Shingeta
// ---------------------------------------------------------------------------

fn render_shingeta_section(html: &mut String, buckets: &Buckets) {
    if buckets.shingeta.is_empty() {
        return;
    }
    html.push_str(&format!(
        "<section><h2><span class=\"chip shingeta\">新下駄</span> 配列 <span class=\"count\">{} rules</span></h2>\n",
        buckets.shingeta.len()
    ));

    let mut single_cells: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut combo_groups: BTreeMap<String, BTreeMap<String, Vec<String>>> = BTreeMap::new();
    let mut overflow: Vec<&Entry> = Vec::new();

    for entry in &buckets.shingeta {
        match &entry.manipulator.from {
            From::Single {
                key_code,
                modifiers: None,
            } if in_grid(key_code) => {
                single_cells
                    .entry(key_id(key_code))
                    .or_default()
                    .push(kana_cell(entry.manipulator));
            }
            From::Simultaneous { simultaneous, .. } if simultaneous.len() == 2 => {
                let shift_key = key_label(&simultaneous[0].key_code);
                let second = &simultaneous[1].key_code;
                if in_grid(second) {
                    combo_groups
                        .entry(shift_key)
                        .or_default()
                        .entry(key_id(second))
                        .or_default()
                        .push(kana_cell(entry.manipulator));
                } else {
                    overflow.push(entry);
                }
            }
            _ => overflow.push(entry),
        }
    }

    html.push_str("<h3>単打</h3>\n");
    render_keyboard(html, &single_cells, true);

    // Show combo groups in home-position order rather than alphabetically.
    let preferred = ["K", "L", "D", "S", "I", "O"];
    let mut group_names: Vec<&String> = combo_groups.keys().collect();
    group_names.sort_by_key(|name| {
        preferred
            .iter()
            .position(|p| p == name)
            .unwrap_or(preferred.len())
    });
    for name in group_names {
        let cells = &combo_groups[name];
        let count: usize = cells.values().map(Vec::len).sum();
        html.push_str(&format!(
            "<h3>{} + □ 同時押し <span class=\"count\">{} keys</span></h3>\n",
            esc(name),
            count
        ));
        render_keyboard(html, cells, true);
    }

    render_entry_table(html, &overflow, true);
    html.push_str("</section>\n");
}

fn kana_cell(manipulator: &Manipulator) -> String {
    match shingeta_output(manipulator) {
        Some(kana) => format!("<span class=\"out\">{}</span>", esc(&kana)),
        None => format!(
            "<span class=\"out fallback\">{}</span>",
            esc(&to_summary(manipulator))
        ),
    }
}

/// If every output is a bare letter key, interpret the sequence as romaji
/// and convert it to kana (e.g. [N, I] -> "に").
fn shingeta_output(manipulator: &Manipulator) -> Option<String> {
    // A single punctuation key typed while Japanese input is active.
    if let [
        To::Key {
            key_code,
            modifiers: None,
            ..
        },
    ] = manipulator.to.as_slice()
        && let Some(punctuation) = japanese_punctuation(key_code)
    {
        return Some(punctuation.to_string());
    }
    let mut romaji = String::new();
    for to in &manipulator.to {
        match to {
            To::Key {
                key_code,
                modifiers: None,
                ..
            } => romaji.push(letter_of(key_code)?),
            _ => return None,
        }
    }
    romaji_to_kana(&romaji)
}

fn letter_of(key_code: &KeyCode) -> Option<char> {
    let debug = format!("{:?}", key_code);
    let mut chars = debug.chars();
    match (chars.next(), chars.next()) {
        (Some(c), None) if c.is_ascii_uppercase() => Some(c.to_ascii_lowercase()),
        _ => None,
    }
}

fn japanese_punctuation(key_code: &KeyCode) -> Option<&'static str> {
    match key_code {
        KeyCode::Hyphen => Some("ー"),
        KeyCode::Comma => Some("、"),
        KeyCode::Period => Some("。"),
        KeyCode::Slash => Some("・"),
        _ => None,
    }
}

fn romaji_to_kana(romaji: &str) -> Option<String> {
    const TABLE: &[(&str, &str)] = &[
        // three letters
        ("lya", "ゃ"),
        ("lyu", "ゅ"),
        ("lyo", "ょ"),
        ("ltu", "っ"),
        ("lwa", "ゎ"),
        ("kya", "きゃ"),
        ("kyu", "きゅ"),
        ("kyo", "きょ"),
        ("gya", "ぎゃ"),
        ("gyu", "ぎゅ"),
        ("gyo", "ぎょ"),
        ("sha", "しゃ"),
        ("shu", "しゅ"),
        ("she", "しぇ"),
        ("sho", "しょ"),
        ("shi", "し"),
        ("cha", "ちゃ"),
        ("chu", "ちゅ"),
        ("che", "ちぇ"),
        ("cho", "ちょ"),
        ("chi", "ち"),
        ("tsu", "つ"),
        ("thi", "てぃ"),
        ("dhi", "でぃ"),
        ("dhu", "でゅ"),
        ("nya", "にゃ"),
        ("nyu", "にゅ"),
        ("nyo", "にょ"),
        ("hya", "ひゃ"),
        ("hyu", "ひゅ"),
        ("hyo", "ひょ"),
        ("bya", "びゃ"),
        ("byu", "びゅ"),
        ("byo", "びょ"),
        ("pya", "ぴゃ"),
        ("pyu", "ぴゅ"),
        ("pyo", "ぴょ"),
        ("mya", "みゃ"),
        ("myu", "みゅ"),
        ("myo", "みょ"),
        ("rya", "りゃ"),
        ("ryu", "りゅ"),
        ("ryo", "りょ"),
        // two letters
        ("la", "ぁ"),
        ("li", "ぃ"),
        ("lu", "ぅ"),
        ("le", "ぇ"),
        ("lo", "ぉ"),
        ("ka", "か"),
        ("ki", "き"),
        ("ku", "く"),
        ("ke", "け"),
        ("ko", "こ"),
        ("ga", "が"),
        ("gi", "ぎ"),
        ("gu", "ぐ"),
        ("ge", "げ"),
        ("go", "ご"),
        ("sa", "さ"),
        ("si", "し"),
        ("su", "す"),
        ("se", "せ"),
        ("so", "そ"),
        ("za", "ざ"),
        ("zi", "じ"),
        ("zu", "ず"),
        ("ze", "ぜ"),
        ("zo", "ぞ"),
        ("ta", "た"),
        ("ti", "ち"),
        ("tu", "つ"),
        ("te", "て"),
        ("to", "と"),
        ("da", "だ"),
        ("di", "ぢ"),
        ("du", "づ"),
        ("de", "で"),
        ("do", "ど"),
        ("na", "な"),
        ("ni", "に"),
        ("nu", "ぬ"),
        ("ne", "ね"),
        ("no", "の"),
        ("ha", "は"),
        ("hi", "ひ"),
        ("hu", "ふ"),
        ("he", "へ"),
        ("ho", "ほ"),
        ("ba", "ば"),
        ("bi", "び"),
        ("bu", "ぶ"),
        ("be", "べ"),
        ("bo", "ぼ"),
        ("pa", "ぱ"),
        ("pi", "ぴ"),
        ("pu", "ぷ"),
        ("pe", "ぺ"),
        ("po", "ぽ"),
        ("fa", "ふぁ"),
        ("fi", "ふぃ"),
        ("fu", "ふ"),
        ("fe", "ふぇ"),
        ("fo", "ふぉ"),
        ("ma", "ま"),
        ("mi", "み"),
        ("mu", "む"),
        ("me", "め"),
        ("mo", "も"),
        ("ya", "や"),
        ("yu", "ゆ"),
        ("yo", "よ"),
        ("ra", "ら"),
        ("ri", "り"),
        ("ru", "る"),
        ("re", "れ"),
        ("ro", "ろ"),
        ("wa", "わ"),
        ("wi", "うぃ"),
        ("we", "うぇ"),
        ("wo", "を"),
        ("ja", "じゃ"),
        ("ju", "じゅ"),
        ("je", "じぇ"),
        ("jo", "じょ"),
        ("ji", "じ"),
        ("vu", "ヴ"),
        ("nn", "ん"),
        // Google IME arrow shortcuts (zh/zj/zk/zl)
        ("zh", "←"),
        ("zj", "↓"),
        ("zk", "↑"),
        ("zl", "→"),
        // single letters
        ("a", "あ"),
        ("i", "い"),
        ("u", "う"),
        ("e", "え"),
        ("o", "お"),
    ];

    let mut result = String::new();
    let mut rest = romaji;
    'outer: while !rest.is_empty() {
        for len in (1..=3).rev() {
            if rest.len() >= len
                && let Some((_, kana)) = TABLE.iter().find(|(r, _)| *r == &rest[..len])
            {
                result.push_str(kana);
                rest = &rest[len..];
                continue 'outer;
            }
        }
        return None;
    }
    Some(result)
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn in_grid(key_code: &KeyCode) -> bool {
    let id = key_id(key_code);
    keyboard_rows().iter().flatten().any(|k| key_id(k) == id)
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

const HEADER: &str = r#"<!doctype html>
<html lang="ja">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>karaconf cheatsheet</title>
<style>
:root {
  color-scheme: light dark;
  --bg: #fafafa; --fg: #1a1a1a; --muted: #777;
  --key-bg: #fff; --key-border: #d5d5d5; --free-fg: #bbb;
  --vk1: #2563eb; --vk2: #059669; --vk3: #d97706; --vk4: #db2777; --shingeta: #7c3aed;
  --badge-bg: #eef;
}
@media (prefers-color-scheme: dark) {
  :root {
    --bg: #16181d; --fg: #e6e6e6; --muted: #999;
    --key-bg: #22252c; --key-border: #3a3e48; --free-fg: #555;
    --badge-bg: #333a55;
  }
}
* { box-sizing: border-box; }
body { background: var(--bg); color: var(--fg); font-family: -apple-system, "Hiragino Sans", sans-serif; margin: 0; }
main { max-width: 1180px; margin: 0 auto; padding: 24px 20px 80px; }
h1 { font-size: 26px; margin-bottom: 4px; }
.meta { color: var(--muted); margin-top: 0; }
section { margin-top: 36px; }
h2 { border-bottom: 2px solid var(--key-border); padding-bottom: 6px; font-size: 20px; }
h3 { font-size: 15px; margin: 18px 0 8px; }
.count { color: var(--muted); font-size: 12px; font-weight: normal; }
.chip { color: #fff; border-radius: 6px; padding: 2px 10px; font-size: 15px; }
.chip.vk1 { background: var(--vk1); } .chip.vk2 { background: var(--vk2); }
.chip.vk3 { background: var(--vk3); } .chip.vk4 { background: var(--vk4); }
.chip.shingeta { background: var(--shingeta); }
.kb { display: inline-block; overflow-x: auto; max-width: 100%; }
.krow { display: flex; gap: 4px; margin-bottom: 4px; }
.key { width: 82px; min-height: 58px; background: var(--key-bg); border: 1px solid var(--key-border);
  border-radius: 7px; padding: 3px 5px; font-size: 10.5px; display: flex; flex-direction: column; gap: 1px; }
.kb.kana .key { width: 58px; min-height: 46px; }
.key .legend { color: var(--muted); font-size: 9px; }
.key .out { font-weight: 600; line-height: 1.25; word-break: break-all; }
.kb.kana .key .out { font-size: 16px; font-weight: 500; }
.kb.kana .key .out.fallback { font-size: 9.5px; }
.key.free { opacity: 0.45; }
.badge { background: var(--badge-bg); border-radius: 4px; padding: 0 3px; margin-right: 3px; font-size: 9px; }
.free-list { color: var(--muted); font-size: 12px; }
table { border-collapse: collapse; margin-top: 8px; font-size: 13px; }
th, td { border: 1px solid var(--key-border); padding: 4px 10px; text-align: left; }
th { background: var(--key-bg); }
td.from { font-weight: 600; white-space: nowrap; }
td.cond, td.src { color: var(--muted); white-space: nowrap; }
ul.lint li { margin-bottom: 4px; }
a { color: var(--vk1); }
</style>
</head>
<body><main>
"#;
