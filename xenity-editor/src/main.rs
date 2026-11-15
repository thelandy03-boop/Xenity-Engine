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
mod build_export_dialog;
pub mod roblox_icon_bytes;
pub mod xenity_ffi;

use eframe::egui::{self, Color32, CursorIcon, FontId, Pos2, Rect, Rounding, Sense, Stroke, Vec2};
use studio_style::*;
use studio_textures::StudioIcons;
use studio_ribbon::*;
use studio_viewport::*;
use studio_toolbox::*;
use studio_output::*;
use hierarchy_window::{ExplorerItem, RobloxClass, SceneHierarchyState};
use properties_window::PropertiesWindowState;
use build_export_dialog::{BuildExportState, BuildDialogAction};
use xenity_ffi::XenityEngine;

struct RobloxStudioApp {
    // Studio Icons & Assets
    icons: Option<StudioIcons>,

    // Embedded C++ Engine (Xenity_Editor.dll)
    engine: Option<XenityEngine>,

    // Studio Windows & Components
    ribbon: StudioRibbonState,
    viewport: StudioViewportState,
    toolbox: StudioToolboxState,
    output: StudioOutputState,
    hierarchy: SceneHierarchyState,
    properties: PropertiesWindowState,
    build_dialog: BuildExportState,

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
    right_width: f32,       // Ancho del panel derecho (Explorer / Properties)

    // Simulation Runtime
    is_playing: bool,
    is_paused: bool,
}

impl Default for RobloxStudioApp {
    fn default() -> Self {
        let engine = XenityEngine::try_load();
        Self {
            icons: None,
            engine,
            ribbon: StudioRibbonState::default(),
            viewport: StudioViewportState::default(),
            toolbox: StudioToolboxState::default(),
            output: StudioOutputState::default(),
            hierarchy: SceneHierarchyState::default(),
            properties: PropertiesWindowState::default(),
            build_dialog: BuildExportState::default(),

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
            right_width: 330.0,

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

        // 0. Sincronizar jerarquía de GameObjects reales desde el motor C++ Xenity
        if let Some(ref mut eng) = self.engine {
            self.hierarchy.sync_from_engine(eng);
        }

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
                        if let Some(e) = &mut self.engine {
                            e.set_sim_state(1);
                        }
                        self.output.add_log(LogType::Info, "Server", "Simulation started (Player joined, Xenity physics active).");
                    }
                    RibbonAction::PauseSimulation => {
                        self.is_paused = !self.is_paused;
                        let sim_state = if self.is_paused { 2 } else { 1 };
                        if let Some(e) = &mut self.engine {
                            e.set_sim_state(sim_state);
                        }
                        let msg = if self.is_paused { "Simulation paused." } else { "Simulation resumed." };
                        self.output.add_log(LogType::Info, "Server", msg);
                    }
                    RibbonAction::StopSimulation => {
                        self.is_playing = false;
                        self.is_paused = false;
                        if let Some(e) = &mut self.engine {
                            e.set_sim_state(0);
                        }
                        self.output.add_log(LogType::Info, "Server", "Simulation stopped.");
                    }
                    RibbonAction::InsertPart(shape) => {
                        let (name, prim_id) = match shape {
                            PartShape::Block => ("Part", 0),
                            PartShape::Sphere => ("SpherePart", 1),
                            PartShape::Cylinder => ("CylinderPart", 2),
                            PartShape::Wedge => ("WedgePart", 4),
                        };
                        if let Some(e) = &mut self.engine {
                            e.spawn_primitive(prim_id);
                        }
                        let id = self.hierarchy.insert_child_into_selected_or_workspace(RobloxClass::Part, name.to_string());
                        self.output.add_log(LogType::Info, "Workspace", format!("Created {} in Xenity Engine (id: {})", name, id));
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
                    RibbonAction::OpenBuildDialog => {
                        self.build_dialog.open();
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
            let out_resp = egui::TopBottomPanel::bottom("studio_output_panel")
                .resizable(true)
                .default_height(self.bottom_height)
                .min_height(90.0)
                .max_height(650.0)
                .frame(egui::Frame::none().fill(STUDIO_BG).inner_margin(egui::Margin::same(0.0)))
                .show(ctx, |ui| {
                    ui.set_min_width(ui.available_width());
                    ui.set_min_height(ui.available_height());
                    let (cmd_result, out_close) = self.output.show(ui, icons, self.show_command_bar);
                    if out_close {
                        self.show_output = false;
                    }
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
            self.bottom_height = out_resp.response.rect.height();
        }

        // =====================================================================
        // 3. PANEL IZQUIERDO: TOOLBOX (MARKETPLACE DE ROBLOX STUDIO)
        // =====================================================================
        if self.show_toolbox {
            let tb_resp = egui::SidePanel::left("studio_toolbox_panel")
                .resizable(true)
                .default_width(self.left_width)
                .min_width(180.0)
                .max_width(550.0)
                .frame(egui::Frame::none().fill(STUDIO_BG).inner_margin(egui::Margin::same(0.0)))
                .show(ctx, |ui| {
                    ui.set_min_width(ui.available_width());
                    ui.set_min_height(ui.available_height());
                    let (model_inserted, tb_close) = self.toolbox.show(ui, icons);
                    if tb_close {
                        self.show_toolbox = false;
                    }
                    if let Some(model_name) = model_inserted {
                        let id = self.hierarchy.insert_child_into_selected_or_workspace(RobloxClass::Model, model_name.clone());
                        self.output.add_log(LogType::Info, "Toolbox", format!("Inserted '{}' into Workspace (id: {}).", model_name, id));
                    }
                });
            self.left_width = tb_resp.response.rect.width();
        }

        // =====================================================================
        // 4. PANEL DERECHO: DOCK DE EXPLORER Y PROPERTIES
        // =====================================================================
        if self.show_explorer || self.show_properties {
            let panel_resp = egui::SidePanel::right("studio_right_dock")
                .resizable(true)
                .default_width(self.right_width)
                .min_width(200.0)
                .max_width(950.0)
                .frame(egui::Frame::none().fill(STUDIO_BG).inner_margin(egui::Margin::same(0.0)))
                .show(ctx, |ui| {
                    ui.set_min_width(ui.available_width());
                    ui.set_min_height(ui.available_height());
                    let full_rect = ui.available_rect_before_wrap();
                    let total_h = full_rect.height();

                    if self.show_explorer && self.show_properties {
                        // Ambas ventanas visibles: divididas por splitter interactivo que adapta ambas ventanas
                        let mut current_split_h = (total_h * self.right_split_ratio).clamp(70.0, (total_h - 70.0).max(70.0));
                        let split_y = full_rect.min.y + current_split_h;

                        // Splitter Barra Horizontal interactiva (zona de agarre de 10px centrada)
                        let grab_h = 10.0;
                        let grab_rect = Rect::from_center_size(
                            Pos2::new(full_rect.center().x, split_y),
                            Vec2::new(full_rect.width(), grab_h),
                        );
                        let splitter_resp = ui.interact(grab_rect, ui.id().with("dock_splitter"), Sense::click_and_drag());
                        if splitter_resp.hovered() || splitter_resp.dragged() {
                            ui.ctx().set_cursor_icon(CursorIcon::ResizeVertical);
                        }

                        // Actualizar proporción instantáneamente según la posición real del ratón
                        if splitter_resp.dragged() {
                            if let Some(pos) = ui.ctx().pointer_latest_pos() {
                                let rel_y = pos.y - full_rect.min.y;
                                self.right_split_ratio = (rel_y / total_h).clamp(0.08, 0.92);
                                current_split_h = (total_h * self.right_split_ratio).clamp(70.0, (total_h - 70.0).max(70.0));
                            }
                        }
                        let current_split_y = full_rect.min.y + current_split_h;

                        // Explorer (Mitad Superior - se adapta y recorta con precisión)
                        let explorer_rect = Rect::from_min_max(full_rect.min, Pos2::new(full_rect.max.x, current_split_y - 2.0));
                        let mut explorer_ui = ui.child_ui(explorer_rect, egui::Layout::top_down(egui::Align::Min), None);
                        explorer_ui.set_clip_rect(explorer_rect);
                        let (clicked_name, exp_close) = self.hierarchy.show(&mut explorer_ui, icons);
                        if let Some(clicked_name) = clicked_name {
                            self.output.add_log(LogType::Info, "Explorer", format!("Selected: {}", clicked_name));
                        }
                        if exp_close {
                            self.show_explorer = false;
                        }

                        // Línea divisoria del splitter (azul cian en hover/drag, borde sutil en reposo)
                        let is_splitter_active = splitter_resp.dragged() || splitter_resp.hovered();
                        let splitter_color = if is_splitter_active {
                            Color32::from_rgb(0, 162, 255)
                        } else {
                            STUDIO_BORDER
                        };
                        ui.painter().rect_filled(
                            Rect::from_center_size(Pos2::new(full_rect.center().x, current_split_y), Vec2::new(full_rect.width(), 3.0)),
                            0.0,
                            splitter_color,
                        );

                        // Properties (Mitad Inferior - al agrandar, el explorador se adapta suavemente y sin solaparse)
                        let properties_rect = Rect::from_min_max(Pos2::new(full_rect.min.x, current_split_y + 2.0), full_rect.max);
                        let mut prop_ui = ui.child_ui(properties_rect, egui::Layout::top_down(egui::Align::Min), None);
                        prop_ui.set_clip_rect(properties_rect);
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
            self.right_width = panel_resp.response.rect.width();
        }

        // =====================================================================
        // 5. PANEL CENTRAL: 3D VIEWPORT DE ROBLOX STUDIO
        // =====================================================================
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(STUDIO_VIEWPORT_BG).inner_margin(egui::Margin::same(0.0)))
            .show(ctx, |ui| {
                let hit_opt = self.viewport.show(
                    ui,
                    icons,
                    self.is_playing,
                    self.show_grid,
                    self.show_view_selector,
                    self.ribbon.current_tool,
                    &mut self.engine,
                );
                if let Some(hit_idx) = hit_opt {
                    if let Some(id) = self.hierarchy.get_id_for_xenity_index(hit_idx) {
                        self.hierarchy.selected_id = Some(id);
                    }
                }
            });

        // Sincronizar selección y propiedades con el motor C++
        if let Some(ref mut eng) = self.engine {
            if let Some(sel_id) = self.hierarchy.selected_id {
                if let Some(x_idx) = self.hierarchy.get_xenity_index_for_id(sel_id) {
                    eng.select_gameobject(x_idx);

                    if let Some(item) = self.hierarchy.find_item_mut(sel_id) {
                        eng.set_gameobject_transform(
                            x_idx,
                            item.properties.position,
                            item.properties.orientation,
                            item.properties.size,
                        );
                        let c = item.properties.color;
                        eng.set_gameobject_color(
                            x_idx,
                            [c.r() as f32 / 255.0, c.g() as f32 / 255.0, c.b() as f32 / 255.0, c.a() as f32 / 255.0],
                        );
                        eng.set_gameobject_name(x_idx, &item.name);
                    }
                }
            }
        }

        // 6. DIÁLOGO MODAL: BUILD & EXPORT (PS3, PSP, VITA, PC)
        let build_action = self.build_dialog.show(ctx, &mut self.engine);
        match build_action {
            BuildDialogAction::OpenOutputLog(msg) => {
                self.output.add_log(LogType::Info, "Export", msg);
                self.show_output = true;
            }
            BuildDialogAction::None => {}
        }

        // Solicitar refresco continuo si la simulación está activa o el motor está inicializado
        if self.is_playing || self.engine.as_ref().map(|e| e.is_initialized).unwrap_or(false) {
            ctx.request_repaint();
        }
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
            .with_title("Place2 - Roblox Studio")
            .with_inner_size([1440.0, 900.0])
            .with_min_inner_size([1024.0, 680.0]),
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };

    eframe::run_native(
        "Place2 - Roblox Studio",
        native_options,
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);
            Ok(Box::new(RobloxStudioApp::default()))
        }),
    )
}
