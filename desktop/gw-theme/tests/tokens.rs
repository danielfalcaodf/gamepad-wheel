//! Guards the design rules that live in data: the tokens match
//! `docs/design/tokens.json`, every text style has an embedded font, and text
//! colours keep a 4.5:1 contrast on the surfaces they are drawn on.

use egui::Color32;
use gw_theme::tokens::*;
use std::collections::BTreeMap;

fn json_tokens() -> BTreeMap<String, String> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../docs/design/tokens.json");
    let json: BTreeMap<String, serde_json::Value> =
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    json.into_iter()
        .filter(|(key, _)| !key.starts_with('$'))
        .map(|(key, value)| match value {
            serde_json::Value::String(s) => (key, s),
            other => (key, other.to_string()),
        })
        .collect()
}

/// `("color", "", "TEXT_2")` -> `"color.text_2"`: drop the const prefix, lower-case.
fn key(group: &str, prefix: &str, name: &str) -> String {
    format!(
        "{group}.{}",
        name.strip_prefix(prefix).unwrap().to_lowercase()
    )
}

fn code_tokens() -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    for (name, c) in COLORS {
        let hex = format!("#{:02X}{:02X}{:02X}", c.r(), c.g(), c.b());
        map.insert(key("color", "", name), hex);
    }
    type Table = &'static [(&'static str, f32)];
    let numbers: [(&str, &str, Table); 4] = [
        ("alpha", "ALPHA_", ALPHAS),
        ("text_size", "TEXT_SIZE_", TEXT_SIZES),
        ("size", "SIZE_", SIZES),
        ("space", "SPACE_", SPACES),
    ];
    for (group, prefix, table) in numbers {
        for (name, value) in table {
            map.insert(key(group, prefix, name), value.to_string());
        }
    }
    for (name, value) in RADII {
        map.insert(key("radius", "RADIUS_", name), value.to_string());
    }
    for (name, t) in TEXTS {
        // TEXT_LABEL_TEXT is `text.label`: the bare TEXT_LABEL name is a colour.
        let base = key("text", "TEXT_", name.strip_suffix("_TEXT").unwrap_or(name));
        map.insert(format!("{base}.family"), t.family.to_owned());
        map.insert(format!("{base}.weight"), t.weight.to_string());
        map.insert(format!("{base}.size"), t.size.to_string());
    }
    map
}

#[test]
fn tokens_match_the_json_source_of_truth() {
    assert_eq!(code_tokens(), json_tokens());
}

#[test]
fn every_text_style_has_an_embedded_font() {
    for (name, t) in TEXTS {
        assert!(
            gw_theme::FONTS
                .iter()
                .any(|(family, weight, _)| *family == t.family && *weight == t.weight),
            "{name}: no embedded font for {}",
            t.font_name()
        );
    }
}

/// WCAG 2 relative luminance.
fn luminance(c: Color32) -> f64 {
    let channel = |v: u8| {
        let v = f64::from(v) / 255.0;
        if v <= 0.04045 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * channel(c.r()) + 0.7152 * channel(c.g()) + 0.0722 * channel(c.b())
}

fn contrast(a: Color32, b: Color32) -> f64 {
    let (hi, lo) = (
        luminance(a).max(luminance(b)),
        luminance(a).min(luminance(b)),
    );
    (hi + 0.05) / (lo + 0.05)
}

/// `tint` at `alpha` over an opaque `base`, as the badges and alerts are drawn.
fn over(tint: Color32, alpha: f32, base: Color32) -> Color32 {
    let mix = |t: u8, b: u8| (f32::from(t) * alpha + f32::from(b) * (1.0 - alpha)).round() as u8;
    Color32::from_rgb(
        mix(tint.r(), base.r()),
        mix(tint.g(), base.g()),
        mix(tint.b(), base.b()),
    )
}

#[test]
fn text_colours_pass_wcag_aa_on_their_surfaces() {
    let mut pairs = Vec::new();
    for surface in [BG, SURFACE, SURFACE_2] {
        for (name, fg) in [
            ("TEXT", TEXT),
            ("TEXT_2", TEXT_2),
            ("TEXT_LABEL", TEXT_LABEL),
            ("ACCENT_TEXT", ACCENT_TEXT),
            ("ERR_HELP", ERR_HELP),
        ] {
            pairs.push((name.to_owned(), fg, surface));
        }
    }
    pairs.push(("ON_ACCENT on ACCENT".to_owned(), ON_ACCENT, ACCENT));
    pairs.push(("ON_ACCENT on ERR".to_owned(), ON_ACCENT, ERR));
    pairs.push(("DARK_SCREEN_TEXT".to_owned(), DARK_SCREEN_TEXT, DARK_SCREEN));
    // Badges: the label on its own tint, with the badge on a card.
    for (name, base, fg, alpha) in [
        ("badge ok", OK, OK, ALPHA_BADGE_BG),
        ("badge warn", WARN, WARN, ALPHA_BADGE_BG),
        ("badge err", ERR, ERR_TEXT, ALPHA_BADGE_BG),
        ("badge info", ACCENT, ACCENT_TEXT, ALPHA_BADGE_BG_INFO),
        ("badge off", TEXT_2, NEUTRAL, ALPHA_BADGE_BG),
    ] {
        pairs.push((name.to_owned(), fg, over(base, alpha, SURFACE)));
    }
    // Alerts: body text on the tinted box, with the alert on the window.
    for (name, base) in [("ok", OK), ("warn", WARN), ("err", ERR), ("info", ACCENT)] {
        pairs.push((
            format!("alert {name}"),
            TEXT,
            over(base, ALPHA_ALERT_BG, BG),
        ));
    }

    let failures: Vec<String> = pairs
        .iter()
        .filter(|(_, fg, bg)| contrast(*fg, *bg) < 4.5)
        .map(|(name, fg, bg)| format!("{name}: {:.2}:1 on {bg:?}", contrast(*fg, *bg)))
        .collect();
    assert!(failures.is_empty(), "below 4.5:1: {failures:#?}");
}
