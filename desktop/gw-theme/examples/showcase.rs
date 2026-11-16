//! Component catalogue, to check the theme against the "00 · Design system"
//! board of the canvas. Not a product screen.
//!
//! `cargo run -p gw-theme --example showcase`

use eframe::egui::{self, RichText, Ui, vec2};
use gw_theme::tokens::*;
use gw_theme::{ButtonKind, ButtonSize, Note, Tone};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 960.0]),
        ..Default::default()
    };
    let mut fields = Fields {
        profile: "Euro Truck Simulator 2".to_owned(),
        executable: "eurotrucks2".to_owned(),
        toggle: true,
        dead_zone: 3.0,
    };
    let mut themed = false;
    eframe::run_ui_native(gw_theme::APP_NAME, options, move |ui, _frame| {
        if !themed {
            gw_theme::apply(ui.ctx());
            themed = true;
        }
        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| catalogue(ui, &mut fields));
        });
    })
}

struct Fields {
    profile: String,
    executable: String,
    toggle: bool,
    dead_zone: f32,
}

fn styled(text: &str, style: Text, color: egui::Color32) -> RichText {
    RichText::new(text).font(style.font_id()).color(color)
}

fn section(ui: &mut Ui, title: &str) {
    ui.add_space(SPACE_XXL);
    ui.label(styled(title, TEXT_CARD_TITLE, TEXT));
}

fn catalogue(ui: &mut Ui, fields: &mut Fields) {
    ui.label(styled(gw_theme::APP_NAME, TEXT_SCREEN_TITLE, TEXT));
    ui.label(styled(
        "Catálogo do tema. Um acento (lavanda) para ação; verde, âmbar e vermelho só para estado.",
        TEXT_BODY,
        TEXT_2,
    ));
    ui.columns(2, |columns| {
        tokens(&mut columns[0]);
        components(&mut columns[1], fields);
    });
}

fn tokens(ui: &mut Ui) {
    section(ui, "Cores");
    egui::Grid::new("colors")
        .spacing(vec2(SPACE_LG, SPACE_LG))
        .show(ui, |ui| {
            for (i, (name, color)) in COLORS.iter().enumerate() {
                ui.vertical(|ui| {
                    let (rect, _) = ui.allocate_exact_size(vec2(104.0, 32.0), egui::Sense::hover());
                    ui.painter().rect_filled(rect, RADIUS_CONTROL, *color);
                    ui.painter().rect_stroke(
                        rect,
                        RADIUS_CONTROL,
                        egui::Stroke::new(1.0, CARD_BORDER),
                        egui::StrokeKind::Inside,
                    );
                    ui.label(styled(&name.to_lowercase(), TEXT_MONO.sized(10.0), TEXT_2));
                });
                if i % 5 == 4 {
                    ui.end_row();
                }
            }
        });

    section(ui, "Tipografia");
    gw_theme::card().show(ui, |ui| {
        ui.spacing_mut().interact_size.y = 0.0;
        for (name, style) in TEXTS {
            ui.horizontal(|ui| {
                ui.label(styled("+45,0° · 12 ms", *style, TEXT));
                let spec = format!(
                    "{name} · {} {} · {}",
                    style.family, style.weight, style.size
                );
                ui.label(styled(&spec, TEXT_MONO, TEXT_2));
            });
        }
    });
}

fn components(ui: &mut Ui, fields: &mut Fields) {
    section(ui, "Botões");
    gw_theme::card().show(ui, |ui| {
        ui.horizontal(|ui| {
            gw_theme::button(ui, "Salvar perfil", ButtonKind::Primary, ButtonSize::Medium);
            gw_theme::button(ui, "Cancelar", ButtonKind::Secondary, ButtonSize::Medium);
            gw_theme::button(
                ui,
                "Restaurar padrão",
                ButtonKind::Ghost,
                ButtonSize::Medium,
            );
            gw_theme::button(ui, "Excluir", ButtonKind::Danger, ButtonSize::Medium);
        });
        ui.horizontal(|ui| {
            gw_theme::button(ui, "Recentralizar", ButtonKind::Primary, ButtonSize::Large);
            gw_theme::button(ui, "Pequeno", ButtonKind::Secondary, ButtonSize::Small);
            ui.add_enabled_ui(false, |ui| {
                gw_theme::button(ui, "Desativado", ButtonKind::Primary, ButtonSize::Medium);
            });
        });
    });

    section(ui, "Selos de status");
    gw_theme::card().show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            gw_theme::badge(ui, Tone::Off, "Não pareado");
            gw_theme::badge(ui, Tone::Info, "Aguardando pareamento");
            gw_theme::badge(ui, Tone::Ok, "Conectado");
            gw_theme::badge(ui, Tone::Warn, "Latência alta");
            gw_theme::badge(ui, Tone::Err, "Conexão perdida");
        });
    });

    section(ui, "Campos de formulário");
    gw_theme::card().show(ui, |ui| {
        ui.columns(2, |columns| {
            gw_theme::field(
                &mut columns[0],
                "Nome do perfil",
                &mut fields.profile,
                Note::Help("Aparece no seletor do topo."),
            );
            gw_theme::field(
                &mut columns[1],
                "Executável",
                &mut fields.executable,
                Note::Error("Informe o nome com .exe."),
            );
        });
        // Stock egui widgets, to check what the global style gives them.
        ui.checkbox(&mut fields.toggle, "Vibrar ao perder conexão");
        ui.add(
            egui::Slider::new(&mut fields.dead_zone, 0.0..=15.0)
                .suffix("°")
                .text("Zona morta"),
        );
    });

    section(ui, "Avisos");
    gw_theme::alert(
        ui,
        Tone::Err,
        "ViGEmBus não está instalado",
        "Sem ele o jogo não vê o controle virtual.",
    );
    gw_theme::alert(
        ui,
        Tone::Warn,
        "Latência alta (48 ms)",
        "Prefira Wi-Fi 5 GHz ou aproxime o roteador.",
    );
    gw_theme::alert(
        ui,
        Tone::Ok,
        "Perfil salvo",
        "As mudanças já valem no jogo aberto.",
    );
    gw_theme::alert(
        ui,
        Tone::Info,
        "Firewall do Windows",
        "Na primeira vez, o Windows pede permissão para a porta UDP.",
    );
}
