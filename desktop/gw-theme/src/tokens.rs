//! Design tokens. Hand-written mirror of `docs/design/tokens.json` (ADR 0003);
//! `tests/tokens.rs` fails if the two drift apart. Sizes are in egui points.

use egui::{Color32, FontFamily, FontId};

/// Declares one `pub const` per token plus a table of all of them, which the
/// drift test compares with the JSON.
macro_rules! tokens {
    ($all:ident: $ty:ty { $($name:ident = $val:expr,)* }) => {
        $(pub const $name: $ty = $val;)*
        pub const $all: &[(&str, $ty)] = &[$((stringify!($name), $name),)*];
    };
}

const fn rgb(hex: u32) -> Color32 {
    Color32::from_rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

tokens!(COLORS: Color32 {
    // Palette named on the design system board.
    BG = rgb(0x0E1116),
    SURFACE = rgb(0x161A22),
    SURFACE_2 = rgb(0x1D222C),
    BORDER = rgb(0x343B49),
    TEXT = rgb(0xE8EBF1),
    TEXT_2 = rgb(0x9AA3B5),
    ACCENT = rgb(0x8B9CFF),
    OK = rgb(0x3DD68C),
    WARN = rgb(0xF2B640),
    ERR = rgb(0xFF6B6B),
    NEUTRAL = rgb(0xAEB6C6),
    // Used by the screens but not named on the board; named here by role.
    TITLE_BAR = rgb(0x0B0D12),
    SIDEBAR = rgb(0x12151C),
    DIVIDER = rgb(0x222834),
    CARD_BORDER = rgb(0x2A303C),
    SELECTED = rgb(0x2A3142),
    TEXT_LABEL = rgb(0xC9CFDB),
    ACCENT_TEXT = rgb(0xA9B5FF),
    ACCENT_TEXT_HOVER = rgb(0xC9D0FF),
    ERR_TEXT = rgb(0xFF7B7B),
    ERR_HELP = rgb(0xFF8A8A),
    ON_ACCENT = rgb(0x0E1116),
    RADIO_OFF = rgb(0x5A6274),
    DARK_SCREEN = rgb(0x000000),
    DARK_SCREEN_TEXT = rgb(0x8A93A6),
});

tokens!(ALPHAS: f32 {
    ALPHA_BADGE_BG = 0.14,
    ALPHA_BADGE_BG_INFO = 0.16,
    ALPHA_ALERT_BG = 0.08,
    ALPHA_ALERT_BORDER = 0.45,
    ALPHA_FOCUS_RING = 0.25,
    ALPHA_OPTION_SELECTED_BG = 0.07,
    ALPHA_DISABLED = 0.45,
});

/// A text style: one of the embedded fonts (family + weight) at a size.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Text {
    pub family: &'static str,
    pub weight: u16,
    pub size: f32,
}

impl Text {
    const fn new(family: &'static str, weight: u16, size: f32) -> Self {
        Self {
            family,
            weight,
            size,
        }
    }

    /// The same face at another size (button and badge size variants).
    pub const fn sized(self, size: f32) -> Self {
        Self { size, ..self }
    }

    /// egui has no font weights, so every weight is its own named family.
    pub fn font_name(&self) -> String {
        format!("{}-{}", self.family, self.weight)
    }

    pub fn font_id(&self) -> FontId {
        FontId::new(self.size, FontFamily::Name(self.font_name().into()))
    }
}

const GROTESK: &str = "Space Grotesk";
const INTER: &str = "Inter";
const MONO: &str = "JetBrains Mono";

tokens!(TEXTS: Text {
    TEXT_BRAND = Text::new(GROTESK, 700, 18.0),
    TEXT_SCREEN_TITLE = Text::new(GROTESK, 600, 28.0),
    TEXT_CARD_TITLE = Text::new(GROTESK, 600, 18.0),
    TEXT_BODY = Text::new(INTER, 400, 14.0),
    TEXT_LABEL_TEXT = Text::new(INTER, 600, 13.0),
    TEXT_HELP = Text::new(INTER, 400, 12.0),
    TEXT_ERROR_HELP = Text::new(INTER, 500, 12.0),
    // The canvas uses Inter 700 here; 600 is the heaviest embedded Inter.
    TEXT_ALERT_TITLE = Text::new(INTER, 600, 14.0),
    TEXT_CAPTION = Text::new(INTER, 600, 12.0),
    TEXT_BUTTON = Text::new(INTER, 600, 14.0),
    TEXT_BADGE = Text::new(INTER, 600, 12.0),
    TEXT_MONO = Text::new(MONO, 400, 12.0),
    TEXT_NUMBER = Text::new(MONO, 600, 22.0),
    TEXT_NUMBER_LARGE = Text::new(MONO, 600, 32.0),
});

tokens!(TEXT_SIZES: f32 {
    TEXT_SIZE_BUTTON_SMALL = 13.0,
    TEXT_SIZE_BUTTON_LARGE = 15.0,
    TEXT_SIZE_MOBILE_BUTTON = 16.0,
    TEXT_SIZE_MOBILE_BADGE = 13.0,
    TEXT_SIZE_MOBILE_TITLE = 26.0,
});

tokens!(RADII: u8 {
    RADIUS_CONTROL = 8,
    RADIUS_SEGMENT = 6,
    RADIUS_ALERT = 10,
    RADIUS_CARD = 12,
    RADIUS_MODAL = 14,
    RADIUS_MOBILE_BUTTON = 12,
    RADIUS_MOBILE_TILE = 14,
});

tokens!(SIZES: f32 {
    SIZE_TOUCH_TARGET = 40.0,
    SIZE_MOBILE_TOUCH_TARGET = 56.0,
    SIZE_BUTTON = 40.0,
    SIZE_BUTTON_SMALL = 32.0,
    SIZE_BUTTON_LARGE = 48.0,
    SIZE_FIELD = 40.0,
    SIZE_BADGE = 24.0,
    SIZE_BADGE_DOT = 6.0,
    SIZE_MOBILE_BADGE = 26.0,
    SIZE_MOBILE_BADGE_DOT = 7.0,
    SIZE_ALERT_ICON = 20.0,
    SIZE_TOGGLE_WIDTH = 40.0,
    SIZE_TOGGLE_HEIGHT = 22.0,
    SIZE_SLIDER_TRACK = 6.0,
    SIZE_SLIDER_THUMB = 18.0,
    SIZE_FOCUS_RING = 3.0,
});

tokens!(SPACES: f32 {
    SPACE_XS = 4.0,
    SPACE_SM = 6.0,
    SPACE_MD = 8.0,
    SPACE_LG = 12.0,
    SPACE_XL = 16.0,
    SPACE_XXL = 24.0,
    SPACE_CARD_PADDING = 20.0,
    SPACE_BUTTON_PADDING_X = 16.0,
    SPACE_BUTTON_SMALL_PADDING_X = 12.0,
    SPACE_BUTTON_LARGE_PADDING_X = 20.0,
    SPACE_MOBILE_BUTTON_PADDING_X = 24.0,
    SPACE_FIELD_PADDING_X = 12.0,
    SPACE_BADGE_PADDING_X = 10.0,
    SPACE_MOBILE_BADGE_PADDING_X = 12.0,
    SPACE_ALERT_PADDING_X = 16.0,
    SPACE_ALERT_PADDING_Y = 14.0,
});
