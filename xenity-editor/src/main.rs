#![allow(warnings)]

mod icons;
mod unity_style;
mod unity_widgets;
mod hierarchy_window;
mod properties_window;
mod project_browser;
pub mod roblox_icon_bytes;

use eframe::egui::{self, Color32, CursorIcon, FontId, Pos2, Rect, Rounding, Sense, Stroke, Vec2};
use unity_style::*;
use unity_widgets::*;
use hierarchy_window::{SceneHierarchyState, STUDIO_BG, STUDIO_BORDER, STUDIO_SELECTION};
use properties_window::PropertiesWindowState;
use project_browser::{AssetItem, AssetType, ProjectBrowserState};

#[derive(PartialEq, Eq, Clone, Copy)]
enum CenterTab {
    Scene,
    Game,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum BottomTab {
    Project,
    Console,
    Animation,
    Animator,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum TransformTool {
    View,
    Translate,
    Rotate,
    Scale,
    Rect,
}

struct XenityEditorApp {
    // Runtime
    is_playing: bool,
    is_paused: bool,

    // Tabs & Tools
    center_tab: CenterTab,
    bottom_tab: BottomTab,
    current_tool: TransformTool,
    pivot_mode: bool,
    local_space: bool,

    // Roblox Studio Dock
    hierarchy: SceneHierarchyState,
    properties_window: PropertiesWindowState,
    right_split_ratio: f32,

    // Project Browser oficial de Unity 2019
    project_browser: ProjectBrowserState,

    // GameObject Header
    entity_is_active: bool,
    entity_name: String,
    entity_tag: String,
    entity_layer: String,
    entity_is_static: bool,

    // Transform
    pos: [f32; 3],
    rot: [f32; 3],
    scale: [f32; 3],

    // Components State
    transform_expanded: bool,
    mesh_expanded: bool,
    rigidbody_expanded: bool,
    collider_expanded: bool,
    audio_expanded: bool,

    // Component Values
    cast_shadows: bool,
    receive_shadows: bool,

    rigidbody_enabled: bool,
    mass: f32,
    drag: f32,
    angular_drag: f32,
    use_gravity: bool,
    is_kinematic: bool,

    collider_enabled: bool,
    is_trigger: bool,
    collider_center: [f32; 3],
    collider_size: [f32; 3],

    audio_enabled: bool,
    volume: f32,
    pitch: f32,
    loop_audio: bool,

    // Logs
    logs: Vec<(Color32, String)>,
    last_log: String,
    last_log_color: Color32,

}

impl Default for XenityEditorApp {
    fn default() -> Self {
        let initial_logs = vec![
            (Color32::from_rgb(180, 215, 255), "[Xenity Engine] Core initialized (wgpu/D3D12)".into()),
            (Color32::from_rgb(180, 215, 255), "[Platform] Crossplatform targets: Windows, PSP, PS Vita, PS3".into()),
            (Color32::from_rgb(180, 215, 255), "[AssetManager] 28 assets loaded in 0.04s".into()),
            (Color32::from_rgb(255, 210, 60), "[Warning] Audio sample rate on PS3 target must be 48000Hz".into()),
            (Color32::from_rgb(120, 240, 140), "[Ready] Editor is ready. C++ hot-reloading listener active.".into()),
        ];

        let last = initial_logs.last().unwrap().clone();

        Self {
            is_playing: false,
            is_paused: false,
            center_tab: CenterTab::Scene,
            bottom_tab: BottomTab::Project,
            current_tool: TransformTool::Translate,
            pivot_mode: true,
            local_space: false,

            hierarchy: SceneHierarchyState::default(),
            properties_window: PropertiesWindowState::default(),
            right_split_ratio: 0.5,
            project_browser: ProjectBrowserState::default(),

            entity_is_active: true,
            entity_name: "Player".into(),
            entity_tag: "Player".into(),
            entity_layer: "Default".into(),
            entity_is_static: false,

            pos: [0.0, 1.25, 0.0],
            rot: [0.0, 45.0, 0.0],
            scale: [1.0, 1.0, 1.0],

            transform_expanded: true,
            mesh_expanded: true,
            rigidbody_expanded: true,
            collider_expanded: true,
            audio_expanded: true,

            cast_shadows: true,
            receive_shadows: true,

            rigidbody_enabled: true,
            mass: 1.0,
            drag: 0.0,
            angular_drag: 0.05,
            use_gravity: true,
            is_kinematic: false,

            collider_enabled: true,
            is_trigger: false,
            collider_center: [0.0, 0.0, 0.0],
            collider_size: [1.0, 2.0, 1.0],

            audio_enabled: true,
            volume: 0.85,
            pitch: 1.0,
            loop_audio: true,

            logs: initial_logs,
            last_log: last.1,
            last_log_color: last.0,
        }
    }
}

fn draw_unity_window_tab(ui: &mut egui::Ui, title: &str, is_active: bool, has_lock: bool) {
    let (tab_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 24.0), Sense::hover());
    ui.painter().rect_filled(tab_rect, 0.0, HEADER_BG);

    let tab_w = 90.0;
    let active_tab_rect = Rect::from_min_size(tab_rect.min, Vec2::new(tab_w, 24.0));
    let bg = if is_active { PANEL_BG } else { HEADER_BG };
    ui.painter().rect_filled(active_tab_rect, Rounding { nw: 2.0, ne: 2.0, sw: 0.0, se: 0.0 }, bg);

    if is_active {
        ui.painter().line_segment(
            [Pos2::new(active_tab_rect.min.x, active_tab_rect.min.y), Pos2::new(active_tab_rect.max.x, active_tab_rect.min.y)],
            Stroke::new(2.0, SELECTION_BLUE),
        );
    }

    ui.painter().text(
        active_tab_rect.center(),
        egui::Align2::CENTER_CENTER,
        title,
        FontId::proportional(11.5),
        if is_active { TEXT_WHITE } else { TEXT_MUTED },
    );

    let right_x = tab_rect.max.x - 12.0;
    ui.painter().text(
        Pos2::new(right_x, tab_rect.center().y),
        egui::Align2::CENTER_CENTER,
        "⋮",
        FontId::proportional(13.0),
        TEXT_MUTED,
    );

    if has_lock {
        ui.painter().text(
            Pos2::new(right_x - 16.0, tab_rect.center().y),
            egui::Align2::CENTER_CENTER,
            "🔒",
            FontId::proportional(10.0),
            TEXT_MUTED,
        );
    }

    ui.painter().line_segment(
        [Pos2::new(tab_rect.min.x, tab_rect.max.y), Pos2::new(tab_rect.max.x, tab_rect.max.y)],
        Stroke::new(1.0, BORDER_COLOR),
    );
}

impl XenityEditorApp {
    fn add_log(&mut self, color: Color32, msg: impl Into<String>) {
        let m = msg.into();
        self.last_log = m.clone();
        self.last_log_color = color;
        self.logs.push((color, m));
    }
}

impl eframe::App for XenityEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        apply_unity_theme(ctx);

        // --- 1. MENÚ PRINCIPAL ---
        egui::TopBottomPanel::top("top_menu")
            .frame(egui::Frame::none().fill(HEADER_BG).inner_margin(egui::Margin::symmetric(8.0, 3.0)))
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("New Scene").clicked() { ui.close_menu(); }
                        if ui.button("Open Scene...").clicked() { ui.close_menu(); }
                        if ui.button("Save Scene").clicked() { ui.close_menu(); }
                        ui.separator();
                        ui.label("Build Platforms:");
                        if ui.button("Sony PlayStation Portable (PSP)").clicked() { ui.close_menu(); }
                        if ui.button("Sony PlayStation Vita").clicked() { ui.close_menu(); }
                        if ui.button("Sony PlayStation 3").clicked() { ui.close_menu(); }
                        if ui.button("PC Windows (MSVC x64)").clicked() { ui.close_menu(); }
                    });

                    ui.menu_button("Edit", |ui| {
                        if ui.button("Undo").clicked() { ui.close_menu(); }
                        if ui.button("Redo").clicked() { ui.close_menu(); }
                        ui.separator();
                        if ui.button("Project Settings...").clicked() { ui.close_menu(); }
                    });

                    ui.menu_button("Assets", |ui| {
                        if ui.button("Create > C++ Script").clicked() { ui.close_menu(); }
                        if ui.button("Create > Material").clicked() { ui.close_menu(); }
                        if ui.button("Import New Asset...").clicked() { ui.close_menu(); }
                    });

                    ui.menu_button("GameObject", |ui| {
                        if ui.button("Create Empty").clicked() { ui.close_menu(); }
                        ui.separator();
                        if ui.button("3D Object > Cube").clicked() { ui.close_menu(); }
                        if ui.button("Light > Directional Light").clicked() { ui.close_menu(); }
                    });

                    ui.menu_button("Window", |ui| {
                        if ui.button("Layouts > Default").clicked() { ui.close_menu(); }
                    });

                    ui.menu_button("Help", |ui| {
                        if ui.button("About Xenity").clicked() { ui.close_menu(); }
                    });
                });
            });

        // --- 2. BARRA DE HERRAMIENTAS UNITY (TRANSFORM TOOLS + PLAY/PAUSE/STEP) ---
        egui::TopBottomPanel::top("toolbar")
            .frame(egui::Frame::none().fill(PANEL_BG).inner_margin(egui::Margin::symmetric(8.0, 4.0)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.current_tool, TransformTool::View, "✋");
                    ui.selectable_value(&mut self.current_tool, TransformTool::Translate, "✛");
                    ui.selectable_value(&mut self.current_tool, TransformTool::Rotate, "↻");
                    ui.selectable_value(&mut self.current_tool, TransformTool::Scale, "⤢");
                    ui.selectable_value(&mut self.current_tool, TransformTool::Rect, "⊡");

                    ui.separator();

                    let pivot_text = if self.pivot_mode { "Pivot" } else { "Center" };
                    if ui.button(pivot_text).clicked() {
                        self.pivot_mode = !self.pivot_mode;
                    }
                    let space_text = if self.local_space { "Local" } else { "Global" };
                    if ui.button(space_text).clicked() {
                        self.local_space = !self.local_space;
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(egui::RichText::new("60.0 FPS | wgpu").color(TEXT_MUTED).size(11.0));
                        ui.separator();

                        let (btn_rect, play_resp) = ui.allocate_exact_size(Vec2::new(28.0, 22.0), Sense::click());
                        let play_color = if self.is_playing { Color32::from_rgb(215, 65, 65) } else { SELECTION_BLUE };
                        let play_bg = if play_resp.hovered() { Color32::from_rgb(60, 120, 190) } else { play_color };
                        ui.painter().rect_filled(btn_rect, 2.0, play_bg);
                        if self.is_playing {
                            icons::stop(ui.painter(), btn_rect, TEXT_WHITE);
                        } else {
                            icons::play(ui.painter(), btn_rect, TEXT_WHITE);
                        }
                        if play_resp.clicked() {
                            self.is_playing = !self.is_playing;
                            let msg = if self.is_playing { "[Runtime] Play mode started." } else { "[Runtime] Play mode stopped." };
                            self.add_log(Color32::from_rgb(120, 240, 140), msg);
                        }

                        let (pause_rect, pause_resp) = ui.allocate_exact_size(Vec2::new(26.0, 22.0), Sense::click());
                        let pause_bg = if pause_resp.hovered() { BTN_HOVER } else { BTN_BG };
                        ui.painter().rect_filled(pause_rect, 2.0, pause_bg);
                        icons::pause(ui.painter(), pause_rect, TEXT_NORMAL);
                        if pause_resp.clicked() {
                            self.is_paused = !self.is_paused;
                        }

                        let (step_rect, step_resp) = ui.allocate_exact_size(Vec2::new(26.0, 22.0), Sense::click());
                        let step_bg = if step_resp.hovered() { BTN_HOVER } else { BTN_BG };
                        ui.painter().rect_filled(step_rect, 2.0, step_bg);
                        icons::step(ui.painter(), step_rect, TEXT_NORMAL);
                        if step_resp.clicked() {
                            self.add_log(Color32::from_rgb(180, 215, 255), "[Runtime] Step 1 frame.");
                        }
                    });
                });
            });

        // --- 3. BARRA DE ESTADO INFERIOR ---
        egui::TopBottomPanel::bottom("status_bar")
            .frame(egui::Frame::none().fill(HEADER_BG).inner_margin(egui::Margin::symmetric(8.0, 3.0)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.colored_label(self.last_log_color, &self.last_log);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(egui::RichText::new("Target: Windows x64 | Unity 2019 IMGUI Specs").color(TEXT_MUTED).size(11.0));
                    });
                });
            });

        // --- 4. DOCK INFERIOR (PROJECT BROWSER / CONSOLE DE UNITY 2019) ---
        egui::TopBottomPanel::bottom("unity_bottom_dock")
            .resizable(true)
            .default_height(280.0)
            .min_height(140.0)
            .max_height(800.0)
            .frame(egui::Frame::none().fill(PANEL_BG).inner_margin(egui::Margin::same(0.0)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(4.0);
                    let tabs = [
                        (BottomTab::Project, "Project"),
                        (BottomTab::Console, "Console"),
                        (BottomTab::Animation, "Animation"),
                        (BottomTab::Animator, "Animator"),
                    ];

                    for (tab, label) in tabs {
                        let is_sel = self.bottom_tab == tab;
                        let bg = if is_sel { PANEL_BG } else { HEADER_BG };
                        let txt_color = if is_sel { TEXT_WHITE } else { TEXT_MUTED };

                        let (rect, resp) = ui.allocate_exact_size(Vec2::new(75.0, 24.0), Sense::click());
                        ui.painter().rect_filled(rect, Rounding { nw: 2.0, ne: 2.0, sw: 0.0, se: 0.0 }, bg);
                        if is_sel {
                            ui.painter().line_segment(
                                [Pos2::new(rect.min.x, rect.min.y), Pos2::new(rect.max.x, rect.min.y)],
                                Stroke::new(2.0, SELECTION_BLUE),
                            );
                        }
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            label,
                            FontId::proportional(11.5),
                            txt_color,
                        );

                        if resp.clicked() {
                            self.bottom_tab = tab;
                        }
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(8.0);
                        ui.label(egui::RichText::new("⋮").color(TEXT_MUTED));
                        ui.label(egui::RichText::new("🔒").size(10.0));
                    });
                });

                ui.painter().line_segment(
                    [Pos2::new(ui.available_rect_before_wrap().min.x, ui.cursor().top()), Pos2::new(ui.available_rect_before_wrap().max.x, ui.cursor().top())],
                    Stroke::new(1.0, BORDER_COLOR),
                );

                match self.bottom_tab {
                    BottomTab::Project => {
                        if let Some(log) = self.project_browser.show(ui) {
                            self.add_log(Color32::from_rgb(180, 215, 255), log);
                        }
                    }
                    BottomTab::Console => {
                        ui.horizontal(|ui| {
                            if ui.button("Clear").clicked() {
                                self.logs.clear();
                            }
                        });
                        ui.separator();

                        egui::ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                            for (color, msg) in &self.logs {
                                ui.colored_label(*color, msg);
                            }
                        });
                    }
                    BottomTab::Animation => {
                        ui.label("Timeline Sequencer");
                    }
                    BottomTab::Animator => {
                        ui.label("Animation State Machine");
                    }
                }
            });

        // --- 5. PANEL DERECHO: ROBLOX STUDIO DOCK (EXPLORER ARRIBA + PROPERTIES ABAJO) ---
        egui::SidePanel::right("studio_right_dock")
            .resizable(true)
            .default_width(330.0)
            .min_width(240.0)
            .max_width(650.0)
            .frame(egui::Frame::none().fill(STUDIO_BG).inner_margin(egui::Margin::same(0.0)))
            .show(ctx, |ui| {
                let full_rect = ui.available_rect_before_wrap();
                let total_h = full_rect.height();

                let split_h = (total_h * self.right_split_ratio).clamp(100.0, (total_h - 100.0).max(100.0));
                let split_y = full_rect.min.y + split_h;

                // --- 1. EXPLORER (Mitad Superior) ---
                let explorer_rect = Rect::from_min_max(
                    full_rect.min,
                    Pos2::new(full_rect.max.x, split_y - 2.0),
                );
                let mut explorer_ui = ui.child_ui(explorer_rect, egui::Layout::top_down(egui::Align::Min), None);
                if let Some(clicked_name) = self.hierarchy.show(&mut explorer_ui) {
                    self.entity_name = clicked_name.clone();
                    self.add_log(Color32::from_rgb(180, 215, 255), format!("Selected: {}", clicked_name));
                }

                // --- 2. SEPARADOR HORIZONTAL REDIMENSIONABLE (SPLITTER) ---
                let splitter_rect = Rect::from_min_max(
                    Pos2::new(full_rect.min.x, split_y - 2.0),
                    Pos2::new(full_rect.max.x, split_y + 2.0),
                );
                let splitter_resp = ui.interact(splitter_rect, ui.id().with("dock_vertical_splitter"), Sense::click_and_drag());
                if splitter_resp.hovered() || splitter_resp.dragged() {
                    ui.ctx().set_cursor_icon(CursorIcon::ResizeVertical);
                }
                let splitter_color = if splitter_resp.dragged() || splitter_resp.hovered() {
                    STUDIO_SELECTION
                } else {
                    STUDIO_BORDER
                };
                ui.painter().rect_filled(splitter_rect, 0.0, splitter_color);

                if splitter_resp.dragged() {
                    let delta_y = splitter_resp.drag_delta().y;
                    let new_h = split_h + delta_y;
                    self.right_split_ratio = (new_h / total_h).clamp(0.15, 0.85);
                }

                // --- 3. PROPERTIES (Mitad Inferior) ---
                let properties_rect = Rect::from_min_max(
                    Pos2::new(full_rect.min.x, split_y + 2.0),
                    full_rect.max,
                );
                let mut prop_ui = ui.child_ui(properties_rect, egui::Layout::top_down(egui::Align::Min), None);
                let selected_item = self.hierarchy.get_selected_item_mut();
                self.properties_window.show(&mut prop_ui, selected_item);
            });

        // --- 7. PANEL CENTRAL: VIEWPORT (WGPU RENDER TARGET) ---
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(DARK_BG).inner_margin(egui::Margin::same(0.0)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let (s_rect, _) = ui.allocate_exact_size(Vec2::new(75.0, 24.0), Sense::click());
                    let is_scene = self.center_tab == CenterTab::Scene;
                    ui.painter().rect_filled(s_rect, Rounding { nw: 2.0, ne: 2.0, sw: 0.0, se: 0.0 }, if is_scene { PANEL_BG } else { HEADER_BG });
                    if is_scene {
                        ui.painter().line_segment([Pos2::new(s_rect.min.x, s_rect.min.y), Pos2::new(s_rect.max.x, s_rect.min.y)], Stroke::new(2.0, SELECTION_BLUE));
                    }
                    ui.painter().text(s_rect.center(), egui::Align2::CENTER_CENTER, "Scene", FontId::proportional(11.5), if is_scene { TEXT_WHITE } else { TEXT_MUTED });
                    if ui.interact(s_rect, ui.id().with("scene_tab"), Sense::click()).clicked() {
                        self.center_tab = CenterTab::Scene;
                    }

                    let (g_rect, _) = ui.allocate_exact_size(Vec2::new(75.0, 24.0), Sense::click());
                    let is_game = self.center_tab == CenterTab::Game;
                    ui.painter().rect_filled(g_rect, Rounding { nw: 2.0, ne: 2.0, sw: 0.0, se: 0.0 }, if is_game { PANEL_BG } else { HEADER_BG });
                    if is_game {
                        ui.painter().line_segment([Pos2::new(g_rect.min.x, g_rect.min.y), Pos2::new(g_rect.max.x, g_rect.min.y)], Stroke::new(2.0, SELECTION_BLUE));
                    }
                    ui.painter().text(g_rect.center(), egui::Align2::CENTER_CENTER, "Game", FontId::proportional(11.5), if is_game { TEXT_WHITE } else { TEXT_MUTED });
                    if ui.interact(g_rect, ui.id().with("game_tab"), Sense::click()).clicked() {
                        self.center_tab = CenterTab::Game;
                    }
                });

                ui.horizontal(|ui| {
                    ui.add_space(8.0);
                    if ui.small_button("Shaded ▼").clicked() {}
                    if ui.small_button("2D").clicked() {}
                    if ui.small_button("☀").clicked() {}
                    if ui.small_button("🔊").clicked() {}
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(8.0);
                        if ui.small_button("Gizmos ▼").clicked() {}
                    });
                });
                ui.separator();

                let (rect, _response) = ui.allocate_exact_size(
                    ui.available_size(),
                    Sense::hover(),
                );

                ui.painter().rect_filled(
                    rect,
                    0.0,
                    if self.is_playing {
                        Color32::from_rgb(32, 24, 26)
                    } else {
                        Color32::from_rgb(26, 26, 28)
                    },
                );

                let stroke = Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(255, 255, 255, 12));
                let center_y = rect.center().y + 80.0;
                for i in -10..=10 {
                    let offset = i as f32 * 35.0;
                    ui.painter().line_segment(
                        [Pos2::new(rect.min.x, center_y + offset), Pos2::new(rect.max.x, center_y + offset)],
                        stroke,
                    );
                    ui.painter().line_segment(
                        [Pos2::new(rect.center().x + offset, center_y - 200.0), Pos2::new(rect.center().x + offset, rect.max.y)],
                        stroke,
                    );
                }

                let gizmo_origin = Pos2::new(rect.min.x + 36.0, rect.min.y + 36.0);
                ui.painter().line_segment([gizmo_origin, Pos2::new(gizmo_origin.x + 22.0, gizmo_origin.y)], Stroke::new(2.0_f32, Color32::from_rgb(220, 60, 60)));
                ui.painter().line_segment([gizmo_origin, Pos2::new(gizmo_origin.x, gizmo_origin.y - 22.0)], Stroke::new(2.0_f32, Color32::from_rgb(60, 200, 60)));
                ui.painter().line_segment([gizmo_origin, Pos2::new(gizmo_origin.x - 14.0, gizmo_origin.y + 14.0)], Stroke::new(2.0_f32, Color32::from_rgb(60, 130, 220)));

                let text = match self.center_tab {
                    CenterTab::Scene => "Xenity 3D Viewport [wgpu Hardware Accelerated]",
                    CenterTab::Game => "Game Runtime View (60 FPS)",
                };
                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    text,
                    FontId::proportional(16.0),
                    Color32::from_gray(140),
                );
            });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Xenity Engine - Unity 2019 Hierarchy & IMGUI Architecture")
            .with_inner_size([1440.0, 880.0])
            .with_min_inner_size([960.0, 640.0]),
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };

    eframe::run_native(
        "Xenity Engine Editor",
        native_options,
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);
            Ok(Box::new(XenityEditorApp::default()))
        }),
    )
}
