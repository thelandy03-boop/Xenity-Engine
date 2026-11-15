use eframe::egui::{self, Color32, Rounding, Stroke};

// =========================================================================
// PALETA DE COLORES OFICIAL DE ROBLOX STUDIO (Dark Theme Tokens)
// =========================================================================

pub const STUDIO_BG: Color32 = Color32::from_rgb(37, 37, 38);             // #252526
pub const STUDIO_TOP_BG: Color32 = Color32::from_rgb(30, 30, 30);         // #1E1E1E
pub const STUDIO_RIBBON_BG: Color32 = Color32::from_rgb(37, 37, 38);      // #252526
pub const STUDIO_TAB_ACTIVE: Color32 = Color32::from_rgb(45, 45, 48);     // #2D2D30
pub const STUDIO_TAB_HOVER: Color32 = Color32::from_rgb(53, 53, 56);      // #353538
pub const STUDIO_SECTION_BORDER: Color32 = Color32::from_rgb(45, 45, 48); // #2D2D30

pub const STUDIO_TREE_BG: Color32 = Color32::from_rgb(30, 30, 30);        // #1E1E1E
pub const STUDIO_VIEWPORT_BG: Color32 = Color32::from_rgb(25, 25, 26);    // #19191A
pub const STUDIO_INPUT_BG: Color32 = Color32::from_rgb(26, 26, 26);       // #1A1A1A

pub const STUDIO_SELECTION: Color32 = Color32::from_rgb(0, 122, 204);     // #007ACC (Azul oficial)
pub const STUDIO_ACCENT_BLUE: Color32 = Color32::from_rgb(0, 162, 255);   // #00A2FF
pub const STUDIO_HOVER: Color32 = Color32::from_rgb(46, 46, 50);          // #2E2E32
pub const STUDIO_BTN_HOVER: Color32 = Color32::from_rgb(60, 60, 65);      // #3C3C41
pub const STUDIO_BORDER: Color32 = Color32::from_rgb(45, 45, 48);         // #2D2D30
pub const STUDIO_DARK_BORDER: Color32 = Color32::from_rgb(28, 28, 30);    // #1C1C1E

pub const STUDIO_GREEN: Color32 = Color32::from_rgb(16, 165, 85);         // Play button green
pub const STUDIO_RED: Color32 = Color32::from_rgb(220, 50, 50);           // Stop button red
pub const STUDIO_ORANGE: Color32 = Color32::from_rgb(245, 140, 25);       // Warning yellow/orange

pub const STUDIO_TEXT: Color32 = Color32::from_rgb(204, 204, 204);        // #CCCCCC
pub const STUDIO_TEXT_WHITE: Color32 = Color32::WHITE;
pub const STUDIO_TEXT_MUTED: Color32 = Color32::from_rgb(140, 140, 140);   // #8C8C8C
pub const STUDIO_TEXT_DARK: Color32 = Color32::from_rgb(100, 100, 100);

pub fn apply_roblox_studio_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();

    visuals.panel_fill = STUDIO_BG;
    visuals.window_fill = STUDIO_BG;
    visuals.extreme_bg_color = STUDIO_INPUT_BG;

    // Bordes y separadores sutiles de 1px
    visuals.widgets.noninteractive.bg_fill = STUDIO_BG;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, STUDIO_BORDER);

    // Botones inactivos
    visuals.widgets.inactive.bg_fill = Color32::from_rgb(45, 45, 48);
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, STUDIO_BORDER);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, STUDIO_TEXT);

    // Hover
    visuals.widgets.hovered.bg_fill = STUDIO_BTN_HOVER;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, Color32::from_rgb(80, 80, 85));
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, STUDIO_TEXT_WHITE);

    // Activo / Clic
    visuals.widgets.active.bg_fill = STUDIO_SELECTION;
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, STUDIO_TEXT_WHITE);

    // Selección
    visuals.selection.bg_fill = STUDIO_SELECTION;
    visuals.selection.stroke = Stroke::new(1.0, STUDIO_TEXT_WHITE);

    // Redondeo Studio (3.0px sutil en botones e inputs)
    let round = Rounding::same(3.0);
    visuals.widgets.inactive.rounding = round;
    visuals.widgets.hovered.rounding = round;
    visuals.widgets.active.rounding = round;
    visuals.window_rounding = round;

    ctx.set_visuals(visuals);

    ctx.style_mut(|s| {
        s.animation_time = 0.0; // Desactivar completamente todas las animaciones (transiciones 100% instantáneas)
        s.interaction.resize_grab_radius_side = 6.0;
    });
}
