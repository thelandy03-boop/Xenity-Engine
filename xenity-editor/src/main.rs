#![allow(warnings)]

mod icons;
mod studio_style;
mod studio_textures;
mod studio_ribbon;
mod studio_viewport;
mod studio_toolbox;
mod studio_output;
mod hierarchy_window;
mod properties_window;
pub mod roblox_icon_bytes;

use eframe::egui::{self, Color32, CursorIcon, FontId, Pos2, Rect, Rounding, Sense, Stroke, Vec2};
use studio_style::*;
use studio_textures::StudioIcons;
use studio_ribbon::*;
use studio_viewport::*;
use studio_toolbox::*;
use studio_output::*;
use hierarchy_window::{ExplorerItem, RobloxClass, SceneHierarchyState};
use properties_window::PropertiesWindowState;

struct RobloxStudioApp {
    // Studio Icons & Assets
    icons: Option<StudioIcons>,

    // Studio Windows & Components
    ribbon: StudioRibbonState,
    viewport: StudioViewportState,
    toolbox: StudioToolboxState,
    output: StudioOutputState,
    hierarchy: SceneHierarchyState,
    properties: PropertiesWindowState,

    // Window Visibility Toggles (controlados desde la pestaña VIEW y accesos directos)
    show_toolbox: bool,
    show_explorer: bool,
    show_properties: bool,
    show_output: bool,
    show_command_bar: bool,
    show_view_selector: bool,
    show_grid: bool,

    // Docking Split Ratios
    right_split_ratio: f32, // Explorer vs Properties (mitad y mitad: 0.5)
    bottom_height: f32,     // Altura de Output
    left_width: f32,        // Ancho de Toolbox

    // Simulation Runtime
    is_playing: bool,
    is_paused: bool,
}

impl Default for RobloxStudioApp {
    fn default() -> Self {
        Self {
            icons: None,
            ribbon: StudioRibbonState::default(),
            viewport: StudioViewportState::default(),
            toolbox: StudioToolboxState::default(),
            output: StudioOutputState::default(),
            hierarchy: SceneHierarchyState::default(),
            properties: PropertiesWindowState::default(),

            // Por defecto, Explorer, Properties, View Selector y Grid están abiertos (layout clásico de Studio)
            show_toolbox: false,
            show_explorer: true,
            show_properties: true,
            show_output: true,
            show_command_bar: true,
            show_view_selector: true,
            show_grid: true,

            right_split_ratio: 0.48,
            bottom_height: 200.0,
            left_width: 250.0,

            is_playing: false,
            is_paused: false,
        }
    }
}

impl eframe::App for RobloxStudioApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        apply_roblox_studio_theme(ctx);

        if self.icons.is_none() {
            self.icons = Some(StudioIcons::new(ctx));
        }
        let icons = self.icons.as_ref().unwrap();

        // =====================================================================
        // 1. RIBBON BAR SUPERIOR COMPLETO DE ROBLOX STUDIO
        // =====================================================================
        egui::TopBottomPanel::top("studio_ribbon_panel")
            .frame(egui::Frame::none().fill(STUDIO_BG).inner_margin(egui::Margin::same(0.0)))
            .show(ctx, |ui| {
                let action = self.ribbon.show(
                    ui,
                    icons,
                    self.is_playing,
                    self.is_paused,
                    self.show_toolbox,
                    self.show_explorer,
                    self.show_properties,
                    self.show_output,
                    self.show_command_bar,
                    self.show_view_selector,
                    self.show_grid,
                );

                match action {
                    RibbonAction::PlaySimulation => {
                        self.is_playing = true;
                        self.is_paused = false;
                        self.output.add_log(LogType::Info, "Server", "Simulation started (Player joined).");
                    }
                    RibbonAction::PauseSimulation => {
                        self.is_paused = !self.is_paused;
                        let msg = if self.is_paused { "Simulation paused." } else { "Simulation resumed." };
                        self.output.add_log(LogType::Info, "Server", msg);
                    }
                    RibbonAction::StopSimulation => {
                        self.is_playing = false;
                        self.is_paused = false;
                        self.output.add_log(LogType::Info, "Server", "Simulation stopped.");
                    }
                    RibbonAction::InsertPart(shape) => {
                        let name = match shape {
                            PartShape::Block => "Part",
                            PartShape::Sphere => "SpherePart",
                            PartShape::Wedge => "WedgePart",
                            PartShape::Cylinder => "CylinderPart",
                        };
                        let id = self.hierarchy.insert_child_into_selected_or_workspace(RobloxClass::Part, name.to_string());
                        self.output.add_log(LogType::Info, "Workspace", format!("Created {} (id: {})", name, id));
                    }
                    RibbonAction::ToggleToolbox => {
                        self.show_toolbox = !self.show_toolbox;
                    }
                    RibbonAction::ToggleExplorer => {
                        self.show_explorer = !self.show_explorer;
                    }
                    RibbonAction::ToggleProperties => {
                        self.show_properties = !self.show_properties;
                    }
                    RibbonAction::ToggleOutput => {
                        self.show_output = !self.show_output;
                    }
                    RibbonAction::ToggleCommandBar => {
                        self.show_command_bar = !self.show_command_bar;
                    }
                    RibbonAction::ToggleViewSelector => {
                        self.show_view_selector = !self.show_view_selector;
                    }
                    RibbonAction::ToggleGrid => {
                        self.show_grid = !self.show_grid;
                    }
                    RibbonAction::ToggleAnchor => {
                        if let Some(item) = self.hierarchy.get_selected_item_mut() {
                            item.properties.anchored = !item.properties.anchored;
                            let status = if item.properties.anchored { "Anchored" } else { "Unanchored" };
                            self.output.add_log(LogType::Info, "Physics", format!("{} is now {}.", item.name, status));
                        }
                    }
                    RibbonAction::ToggleCollisions => {
                        self.output.add_log(LogType::Info, "Physics", format!("Collisions: {}", self.ribbon.collisions_enabled));
                    }
                    RibbonAction::SavePlace => {
                        self.output.add_log(LogType::Info, "Studio", "Place saved to local disk cache successfully.");
                    }
                    RibbonAction::Undo => {
                        self.output.add_log(LogType::Info, "History", "Undo action performed.");
                    }
                    RibbonAction::Redo => {
                        self.output.add_log(LogType::Info, "History", "Redo action performed.");
                    }
                    _ => {}
                }
            });

        // =====================================================================
        // 2. PANEL INFERIOR: OUTPUT WINDOW Y COMMAND BAR (ROBLOX STUDIO)
        // =====================================================================
        if self.show_output {
            egui::TopBottomPanel::bottom("studio_output_panel")
                .resizable(true)
                .default_height(self.bottom_height)
                .min_height(90.0)
                .max_height(600.0)
                .frame(egui::Frame::none().fill(STUDIO_BG).inner_margin(egui::Margin::same(0.0)))
                .show(ctx, |ui| {
                    let cmd_result = self.output.show(ui, icons, self.show_command_bar);
                    match cmd_result {
                        CommandBarResult::SpawnPart => {
                            self.hierarchy.insert_child_into_selected_or_workspace(RobloxClass::Part, "Part".into());
                        }
                        CommandBarResult::SpawnScript => {
                            self.hierarchy.insert_child_into_selected_or_workspace(RobloxClass::Script, "Script".into());
                        }
                        _ => {}
                    }
                });
        }

        // =====================================================================
        // 3. PANEL IZQUIERDO: TOOLBOX (MARKETPLACE DE ROBLOX STUDIO)
        // =====================================================================
        if self.show_toolbox {
            egui::SidePanel::left("studio_toolbox_panel")
                .resizable(true)
                .default_width(self.left_width)
                .min_width(180.0)
                .max_width(450.0)
                .frame(egui::Frame::none().fill(STUDIO_BG).inner_margin(egui::Margin::same(0.0)))
                .show(ctx, |ui| {
                    if let Some(model_name) = self.toolbox.show(ui, icons) {
                        let id = self.hierarchy.insert_child_into_selected_or_workspace(RobloxClass::Model, model_name.clone());
                        self.output.add_log(LogType::Info, "Toolbox", format!("Inserted '{}' into Workspace (id: {}).", model_name, id));
                    }
                });
        }

        // =====================================================================
        // 4. PANEL DERECHO: DOCK DE EXPLORER Y PROPERTIES
        // =====================================================================
        if self.show_explorer || self.show_properties {
            egui::SidePanel::right("studio_right_dock")
                .resizable(true)
                .default_width(330.0)
                .min_width(240.0)
                .max_width(650.0)
                .frame(egui::Frame::none().fill(STUDIO_BG).inner_margin(egui::Margin::same(0.0)))
                .show(ctx, |ui| {
                    let full_rect = ui.available_rect_before_wrap();
                    let total_h = full_rect.height();

                    if self.show_explorer && self.show_properties {
                        // Ambas ventanas visibles: divididas por splitter interactivo
                        let split_h = (total_h * self.right_split_ratio).clamp(80.0, (total_h - 80.0).max(80.0));
                        let split_y = full_rect.min.y + split_h;

                        // Explorer (Mitad Superior)
                        let explorer_rect = Rect::from_min_max(full_rect.min, Pos2::new(full_rect.max.x, split_y - 2.0));
                        let mut explorer_ui = ui.child_ui(explorer_rect, egui::Layout::top_down(egui::Align::Min), None);
                        let (clicked_name, exp_close) = self.hierarchy.show(&mut explorer_ui, icons);
                        if let Some(clicked_name) = clicked_name {
                            self.output.add_log(LogType::Info, "Explorer", format!("Selected: {}", clicked_name));
                        }
                        if exp_close {
                            self.show_explorer = false;
                        }

                        // Splitter Barra Horizontal
                        let splitter_rect = Rect::from_min_max(Pos2::new(full_rect.min.x, split_y - 2.0), Pos2::new(full_rect.max.x, split_y + 2.0));
                        let splitter_resp = ui.interact(splitter_rect, ui.id().with("dock_splitter"), Sense::click_and_drag());
                        if splitter_resp.hovered() || splitter_resp.dragged() {
                            ui.ctx().set_cursor_icon(CursorIcon::ResizeVertical);
                        }
                        let splitter_color = if splitter_resp.dragged() || splitter_resp.hovered() { STUDIO_SELECTION } else { STUDIO_BORDER };
                        ui.painter().rect_filled(splitter_rect, 0.0, splitter_color);

                        if splitter_resp.dragged() {
                            let delta_y = splitter_resp.drag_delta().y;
                            let new_h = split_h + delta_y;
                            self.right_split_ratio = (new_h / total_h).clamp(0.15, 0.85);
                        }

                        // Properties (Mitad Inferior)
                        let properties_rect = Rect::from_min_max(Pos2::new(full_rect.min.x, split_y + 2.0), full_rect.max);
                        let mut prop_ui = ui.child_ui(properties_rect, egui::Layout::top_down(egui::Align::Min), None);
                        let selected_item = self.hierarchy.get_selected_item_mut();
                        let prop_close = self.properties.show(&mut prop_ui, selected_item, icons);
                        if prop_close {
                            self.show_properties = false;
                        }
                    } else if self.show_explorer {
                        // Solo Explorer ocupa todo el panel
                        let (clicked_name, exp_close) = self.hierarchy.show(ui, icons);
                        if let Some(clicked_name) = clicked_name {
                            self.output.add_log(LogType::Info, "Explorer", format!("Selected: {}", clicked_name));
                        }
                        if exp_close {
                            self.show_explorer = false;
                        }
                    } else if self.show_properties {
                        // Solo Properties ocupa todo el panel
                        let selected_item = self.hierarchy.get_selected_item_mut();
                        let prop_close = self.properties.show(ui, selected_item, icons);
                        if prop_close {
                            self.show_properties = false;
                        }
                    }
                });
        }

        // =====================================================================
        // 5. PANEL CENTRAL: 3D VIEWPORT DE ROBLOX STUDIO
        // =====================================================================
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(STUDIO_VIEWPORT_BG).inner_margin(egui::Margin::same(0.0)))
            .show(ctx, |ui| {
                self.viewport.show(
                    ui,
                    icons,
                    self.is_playing,
                    self.show_grid,
                    self.show_view_selector,
                );
            });
    }
}

fn setup_fonts(ctx: &egui::Context) {
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

fn main() -> eframe::Result<()> {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Roblox Studio - [Xenity Engine Edition]")
            .with_inner_size([1440.0, 900.0])
            .with_min_inner_size([1024.0, 680.0]),
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };

    eframe::run_native(
        "Roblox Studio",
        native_options,
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);
            Ok(Box::new(RobloxStudioApp::default()))
        }),
    )
}
