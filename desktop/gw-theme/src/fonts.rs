//! The embedded fonts (all SIL OFL 1.1, see `docs/design/fonts/`). One copy of
//! the files serves both sides: they live in the Android `res/font` directory.

use egui::{FontData, FontDefinitions, FontFamily};
use std::sync::Arc;

macro_rules! font {
    ($family:literal, $weight:literal, $file:literal) => {
        (
            $family,
            $weight,
            include_bytes!(concat!("../../../mobile/app/src/main/res/font/", $file)).as_slice(),
        )
    };
}

/// `(family, weight, ttf bytes)` for every embedded face.
pub const FONTS: &[(&str, u16, &[u8])] = &[
    font!("Inter", 400, "inter_regular.ttf"),
    font!("Inter", 500, "inter_medium.ttf"),
    font!("Inter", 600, "inter_semibold.ttf"),
    font!("Space Grotesk", 600, "space_grotesk_semibold.ttf"),
    font!("Space Grotesk", 700, "space_grotesk_bold.ttf"),
    font!("JetBrains Mono", 400, "jetbrains_mono_regular.ttf"),
    font!("JetBrains Mono", 600, "jetbrains_mono_semibold.ttf"),
];

/// Every face as its own named family (`"Inter-600"`, see `Text::font_name`),
/// with Inter 400 and JetBrains Mono 400 as egui's default proportional and
/// monospace fonts.
pub fn definitions() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    for &(family, weight, bytes) in FONTS {
        let name = format!("{family}-{weight}");
        fonts
            .font_data
            .insert(name.clone(), Arc::new(FontData::from_static(bytes)));
        fonts
            .families
            .insert(FontFamily::Name(name.clone().into()), vec![name.clone()]);
        let default_for = match (family, weight) {
            ("Inter", 400) => FontFamily::Proportional,
            ("JetBrains Mono", 400) => FontFamily::Monospace,
            _ => continue,
        };
        fonts
            .families
            .entry(default_for)
            .or_default()
            .insert(0, name);
    }
    fonts
}
