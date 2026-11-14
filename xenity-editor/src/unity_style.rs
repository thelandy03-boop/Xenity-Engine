use eframe::egui::{self, Color32, Margin, Rounding, Stroke};

// =========================================================================
// CONSTANTES Y MÉTRICAS EXACTAS DE UNITY 2019 (UnityCsReference EditorStyles)
// =========================================================================

pub const SINGLE_LINE_HEIGHT: f32 = 18.0;
pub const CONTROL_SPACING: f32 = 2.0;
pub const INDENT_PER_LEVEL: f32 = 14.0;
pub const LABEL_WIDTH: f32 = 110.0;

// Paleta Oficial Unity 2019 Dark Skin
pub const PANEL_BG: Color32 = Color32::from_rgb(56, 56, 56);        // #383838 (Gris base paneles)
pub const DARK_BG: Color32 = Color32::from_rgb(40, 40, 40);         // #282828 (Listas y árboles)
pub const HEADER_BG: Color32 = Color32::from_rgb(46, 46, 46);       // #2E2E2E (Barras de menú y tabs)
pub const SUB_HEADER_BG: Color32 = Color32::from_rgb(50, 50, 50);   // #323232
pub const INPUT_BG: Color32 = Color32::from_rgb(34, 34, 34);        // #222222 (Campos de texto y drags)
pub const BORDER_COLOR: Color32 = Color32::from_rgb(30, 30, 30);    // #1E1E1E (Líneas divisorias)

pub const SELECTION_BLUE: Color32 = Color32::from_rgb(44, 93, 148); // #2C5D94 (Azul selección Unity)
pub const SELECTION_UNFOCUSED: Color32 = Color32::from_rgb(70, 70, 70);
pub const HOVER_GRAY: Color32 = Color32::from_rgb(68, 68, 68);

pub const BTN_BG: Color32 = Color32::from_rgb(64, 64, 64);          // #404040
pub const BTN_HOVER: Color32 = Color32::from_rgb(76, 76, 76);       // #4C4C4C
pub const BTN_ACTIVE: Color32 = Color32::from_rgb(50, 50, 50);

pub const TEXT_NORMAL: Color32 = Color32::from_rgb(215, 215, 215);  // #D7D7D7
pub const TEXT_MUTED: Color32 = Color32::from_rgb(145, 145, 145);   // #919191
pub const TEXT_WHITE: Color32 = Color32::WHITE;

pub fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    if let Ok(data) = std::fs::read("C:\\Windows\\Fonts\\segoeui.ttf") {
        fonts.font_data.insert(
            "segoe_ui".to_owned(),
            std::sync::Arc::new(egui::FontData::from_owned(data)),
        );
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "segoe_ui".to_owned());
    }
    if let Ok(data) = std::fs::read("C:\\Windows\\Fonts\\segoeuib.ttf") {
        fonts.font_data.insert(
            "segoe_ui_bold".to_owned(),
            std::sync::Arc::new(egui::FontData::from_owned(data)),
        );
    }
    ctx.set_fonts(fonts);
}

pub fn apply_unity_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();

    visuals.panel_fill = PANEL_BG;
    visuals.window_fill = PANEL_BG;
    visuals.extreme_bg_color = INPUT_BG;

    // Bordes y separadores sutiles de 1px
    visuals.widgets.noninteractive.bg_fill = PANEL_BG;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, BORDER_COLOR);

    // Botones inactivos
    visuals.widgets.inactive.bg_fill = BTN_BG;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(46, 46, 46));
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, TEXT_NORMAL);

    // Hover
    visuals.widgets.hovered.bg_fill = BTN_HOVER;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, Color32::from_rgb(90, 90, 90));
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, TEXT_WHITE);

    // Activo / Clic
    visuals.widgets.active.bg_fill = SELECTION_BLUE;
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, TEXT_WHITE);

    // Selección
    visuals.selection.bg_fill = SELECTION_BLUE;
    visuals.selection.stroke = Stroke::new(1.0, TEXT_WHITE);

    // Redondeo clásico Unity (2.0px muy sutil)
    let round = Rounding::same(2.0);
    visuals.widgets.inactive.rounding = round;
    visuals.widgets.hovered.rounding = round;
    visuals.widgets.active.rounding = round;
    visuals.window_rounding = round;

    ctx.set_visuals(visuals);
}
