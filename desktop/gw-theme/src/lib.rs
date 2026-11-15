//! The egui theme for gamepad-wheel: design tokens, embedded fonts and the
//! styles of the base components (button, badge, field, alert).
//!
//! Call [`apply`] once at startup. Design rules kept by this API:
//! - one accent (lavender), for actions only; green, amber and red are status;
//! - status never relies on colour alone: a [`badge`] always has a dot and a
//!   text, an [`alert`] always has an icon.

mod fonts;
pub mod tokens;

pub use fonts::FONTS;

use egui::{
    Color32, CornerRadius, FontFamily, Frame, Margin, Response, RichText, Sense, Stroke,
    StrokeKind, TextEdit, TextStyle, Ui, Vec2, pos2, vec2,
};
use tokens::*;

/// Product name shown in the UI. Code, repo and executable stay `gamepad-wheel`.
pub const APP_NAME: &str = "Gamepad Volante";

/// Installs the fonts and the dark style. The design has no light theme.
pub fn apply(ctx: &egui::Context) {
    ctx.set_fonts(fonts::definitions());
    ctx.set_theme(egui::Theme::Dark);
    ctx.all_styles_mut(|style| {
        style.text_styles = [
            (TextStyle::Heading, TEXT_SCREEN_TITLE.font_id()),
            (TextStyle::Body, TEXT_BODY.font_id()),
            (TextStyle::Button, TEXT_BUTTON.font_id()),
            (TextStyle::Small, TEXT_HELP.font_id()),
            (
                TextStyle::Monospace,
                egui::FontId::new(TEXT_MONO.size, FontFamily::Monospace),
            ),
        ]
        .into();

        let s = &mut style.spacing;
        s.item_spacing = Vec2::splat(SPACE_MD);
        s.interact_size = Vec2::splat(SIZE_TOUCH_TARGET);
        s.button_padding = vec2(SPACE_BUTTON_PADDING_X, SPACE_MD);
        s.slider_rail_height = SIZE_SLIDER_TRACK;

        let v = &mut style.visuals;
        v.panel_fill = BG;
        v.window_fill = SURFACE_2;
        v.window_stroke = Stroke::new(1.0, BORDER);
        v.window_corner_radius = CornerRadius::same(RADIUS_MODAL);
        v.menu_corner_radius = CornerRadius::same(RADIUS_ALERT);
        v.extreme_bg_color = BG;
        v.text_edit_bg_color = Some(BG);
        v.faint_bg_color = SURFACE;
        v.code_bg_color = SURFACE_2;
        v.hyperlink_color = ACCENT_TEXT;
        v.warn_fg_color = WARN;
        v.error_fg_color = ERR_TEXT;
        v.disabled_alpha = ALPHA_DISABLED;
        v.slider_trailing_fill = true;
        v.selection.bg_fill = SELECTED;
        v.selection.stroke = Stroke::new(1.0, TEXT);

        let radius = CornerRadius::same(RADIUS_CONTROL);
        let w = &mut v.widgets;
        w.noninteractive.bg_fill = SURFACE;
        w.noninteractive.weak_bg_fill = SURFACE;
        w.noninteractive.bg_stroke = Stroke::new(1.0, CARD_BORDER);
        w.noninteractive.fg_stroke = Stroke::new(1.0, TEXT);
        w.noninteractive.corner_radius = radius;
        // The canvas draws no hover or pressed state except the menu row
        // (`SELECTED`), so that is the only feedback used here.
        for (state, fill) in [
            (&mut w.inactive, SURFACE_2),
            (&mut w.hovered, SELECTED),
            (&mut w.active, SELECTED),
            (&mut w.open, SELECTED),
        ] {
            state.bg_fill = BORDER;
            state.weak_bg_fill = fill;
            state.bg_stroke = Stroke::new(1.0, BORDER);
            state.fg_stroke = Stroke::new(1.0, TEXT);
            state.corner_radius = radius;
            state.expansion = 0.0;
        }
    });
}

/// `color` at CSS-style alpha, for the tinted badge and alert backgrounds.
fn tint(color: Color32, alpha: f32) -> Color32 {
    color.gamma_multiply(alpha)
}

fn text(s: &str, style: Text, color: Color32) -> RichText {
    RichText::new(s).font(style.font_id()).color(color)
}

/// Surface for grouped content.
pub fn card() -> Frame {
    Frame::new()
        .fill(SURFACE)
        .stroke(Stroke::new(1.0, CARD_BORDER))
        .corner_radius(RADIUS_CARD)
        .inner_margin(SPACE_CARD_PADDING)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonKind {
    /// The one accent-coloured action of a view.
    Primary,
    Secondary,
    Ghost,
    /// Destructive action.
    Danger,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

/// A button in one of the four design variants. Wrap in `ui.add_enabled_ui`
/// for the disabled look.
pub fn button(ui: &mut Ui, label: &str, kind: ButtonKind, size: ButtonSize) -> Response {
    let (fill, fg, border) = match kind {
        ButtonKind::Primary => (ACCENT, ON_ACCENT, Color32::TRANSPARENT),
        ButtonKind::Secondary => (SURFACE_2, TEXT, BORDER),
        ButtonKind::Ghost => (Color32::TRANSPARENT, TEXT_LABEL, Color32::TRANSPARENT),
        ButtonKind::Danger => (ERR, ON_ACCENT, Color32::TRANSPARENT),
    };
    let (height, padding_x, font) = match size {
        ButtonSize::Small => (
            SIZE_BUTTON_SMALL,
            SPACE_BUTTON_SMALL_PADDING_X,
            TEXT_BUTTON.sized(TEXT_SIZE_BUTTON_SMALL),
        ),
        ButtonSize::Medium => (SIZE_BUTTON, SPACE_BUTTON_PADDING_X, TEXT_BUTTON),
        ButtonSize::Large => (
            SIZE_BUTTON_LARGE,
            SPACE_BUTTON_LARGE_PADDING_X,
            TEXT_BUTTON.sized(TEXT_SIZE_BUTTON_LARGE),
        ),
    };
    ui.scope(|ui| {
        ui.spacing_mut().button_padding.x = padding_x;
        ui.spacing_mut().interact_size.y = height;
        ui.add(
            egui::Button::new(text(label, font, fg))
                .fill(fill)
                .stroke(Stroke::new(1.0, border))
                .corner_radius(RADIUS_CONTROL)
                .min_size(vec2(0.0, height)),
        )
    })
    .inner
}

/// The meaning a badge or an alert carries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tone {
    Ok,
    Warn,
    Err,
    /// Neutral information; the only non-action use of the accent hue.
    Info,
    /// Inactive / not applicable.
    Off,
}

impl Tone {
    /// `(base colour for tints, foreground colour for text and icons)`.
    fn colors(self) -> (Color32, Color32) {
        match self {
            Self::Ok => (OK, OK),
            Self::Warn => (WARN, WARN),
            Self::Err => (ERR, ERR_TEXT),
            Self::Info => (ACCENT, ACCENT_TEXT),
            Self::Off => (TEXT_2, NEUTRAL),
        }
    }
}

/// Status badge: tinted pill with a dot and a text, never colour alone.
pub fn badge(ui: &mut Ui, tone: Tone, label: &str) -> Response {
    let (base, fg) = tone.colors();
    let alpha = if tone == Tone::Info {
        ALPHA_BADGE_BG_INFO
    } else {
        ALPHA_BADGE_BG
    };
    let galley = ui
        .painter()
        .layout_no_wrap(label.to_owned(), TEXT_BADGE.font_id(), fg);
    let width = 2.0 * SPACE_BADGE_PADDING_X + SIZE_BADGE_DOT + SPACE_SM + galley.size().x;
    let (rect, response) = ui.allocate_exact_size(vec2(width, SIZE_BADGE), Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(rect, SIZE_BADGE / 2.0, tint(base, alpha));
    let dot = pos2(
        rect.left() + SPACE_BADGE_PADDING_X + SIZE_BADGE_DOT / 2.0,
        rect.center().y,
    );
    painter.circle_filled(dot, SIZE_BADGE_DOT / 2.0, fg);
    let text_pos = pos2(
        dot.x + SIZE_BADGE_DOT / 2.0 + SPACE_SM,
        rect.center().y - galley.size().y / 2.0,
    );
    painter.galley(text_pos, galley, fg);
    response
}

/// Alert box: tinted frame with an icon, a title and a body.
pub fn alert(ui: &mut Ui, tone: Tone, title: &str, body: &str) -> Response {
    let (base, fg) = tone.colors();
    Frame::new()
        .fill(tint(base, ALPHA_ALERT_BG))
        .stroke(Stroke::new(1.0, tint(base, ALPHA_ALERT_BORDER)))
        .corner_radius(RADIUS_ALERT)
        .inner_margin(Margin::symmetric(
            SPACE_ALERT_PADDING_X as i8,
            SPACE_ALERT_PADDING_Y as i8,
        ))
        .show(ui, |ui| {
            ui.horizontal_top(|ui| {
                ui.spacing_mut().item_spacing = vec2(SPACE_LG, 2.0);
                alert_icon(ui, tone, fg);
                ui.vertical(|ui| {
                    ui.label(text(title, TEXT_ALERT_TITLE, TEXT));
                    ui.label(text(body, TEXT_BODY, TEXT));
                });
            });
        })
        .response
}

/// The 20 px stroke icons of the canvas, drawn with the painter so no icon
/// font is needed: cross, warning triangle, check, "i".
fn alert_icon(ui: &mut Ui, tone: Tone, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(SIZE_ALERT_ICON), Sense::hover());
    let painter = ui.painter();
    // The SVGs are 24 units wide with a 2-unit stroke.
    let unit = SIZE_ALERT_ICON / 24.0;
    let stroke = Stroke::new(2.0 * unit, color);
    let at = |x: f32, y: f32| rect.min + vec2(x, y) * unit;
    let line =
        |a: (f32, f32), b: (f32, f32)| painter.line_segment([at(a.0, a.1), at(b.0, b.1)], stroke);
    if tone == Tone::Warn {
        let triangle = vec![at(12.0, 3.0), at(22.0, 20.5), at(2.0, 20.5)];
        painter.add(egui::Shape::closed_line(triangle, stroke));
    } else {
        painter.circle_stroke(at(12.0, 12.0), 10.0 * unit, stroke);
    }
    match tone {
        Tone::Err => {
            line((15.0, 9.0), (9.0, 15.0));
            line((9.0, 9.0), (15.0, 15.0));
        }
        Tone::Warn => {
            line((12.0, 9.0), (12.0, 13.0));
            line((12.0, 16.8), (12.0, 17.2));
        }
        Tone::Ok => {
            line((8.0, 12.0), (11.0, 15.0));
            line((11.0, 15.0), (16.0, 9.0));
        }
        Tone::Info | Tone::Off => {
            line((12.0, 11.0), (12.0, 16.0));
            line((12.0, 7.8), (12.0, 8.2));
        }
    }
}

/// The line under a field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Note<'a> {
    None,
    Help(&'a str),
    /// Marks the field invalid: red border plus this message.
    Error(&'a str),
}

/// Labelled single-line text field with a help or error line.
pub fn field(ui: &mut Ui, label: &str, value: &mut String, note: Note<'_>) -> Response {
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = SPACE_SM;
        ui.label(text(label, TEXT_LABEL_TEXT, TEXT_LABEL));

        let id = ui.make_persistent_id(label);
        let focused = ui.memory(|m| m.has_focus(id));
        let border = match note {
            _ if focused => ACCENT,
            Note::Error(_) => ERR,
            _ => BORDER,
        };
        let frame = Frame::new()
            .fill(BG)
            .stroke(Stroke::new(1.0, border))
            .corner_radius(RADIUS_CONTROL)
            .inner_margin(Margin::symmetric(SPACE_FIELD_PADDING_X as i8, 0));
        let response = ui.add(
            TextEdit::singleline(value)
                .id(id)
                .frame(frame)
                .font(TEXT_BODY.font_id())
                .text_color(TEXT)
                .align(egui::Align2::LEFT_CENTER)
                .min_size(vec2(0.0, SIZE_FIELD))
                .desired_width(f32::INFINITY),
        );
        if focused {
            ui.painter().rect_stroke(
                response.rect,
                RADIUS_CONTROL,
                Stroke::new(SIZE_FOCUS_RING, tint(ACCENT, ALPHA_FOCUS_RING)),
                StrokeKind::Outside,
            );
        }
        match note {
            Note::None => {}
            Note::Help(help) => drop(ui.label(text(help, TEXT_HELP, TEXT_2))),
            Note::Error(error) => drop(ui.label(text(error, TEXT_ERROR_HELP, ERR_HELP))),
        }
        response
    })
    .inner
}
