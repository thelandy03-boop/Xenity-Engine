use eframe::egui::{self, Color32, CursorIcon, FontId, Pos2, Rect, Rounding, Sense, Stroke, Vec2};
use crate::studio_style::*;
use crate::studio_textures::StudioIcons;
use crate::icons;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum RibbonTab {
    Home,
    Model,
    Avatar,
    Test,
    View,
    Plugins,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum StudioTool {
    Select,
    Move,
    Rotate,
    Scale,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum PartShape {
    Block,
    Sphere,
    Wedge,
    Cylinder,
}

pub struct StudioRibbonState {
    pub current_tab: RibbonTab,
    pub current_tool: StudioTool,
    pub part_dropdown_open: bool,
    pub play_dropdown_open: bool,

    // Snapping & Physical Settings
    pub move_snap_enabled: bool,
    pub move_snap_studs: f32,
    pub rotate_snap_enabled: bool,
    pub rotate_snap_deg: f32,
    pub collisions_enabled: bool,
    pub join_surfaces: bool,

    // Fast quick-access
    pub can_undo: bool,
    pub can_redo: bool,
}

impl Default for StudioRibbonState {
    fn default() -> Self {
        Self {
            current_tab: RibbonTab::Home,
            current_tool: StudioTool::Select,
            part_dropdown_open: false,
            play_dropdown_open: false,

            move_snap_enabled: true,
            move_snap_studs: 1.0,
            rotate_snap_enabled: true,
            rotate_snap_deg: 45.0,
            collisions_enabled: true,
            join_surfaces: true,

            can_undo: true,
            can_redo: false,
        }
    }
}

pub enum RibbonAction {
    None,
    ChangeTool(StudioTool),
    InsertPart(PartShape),
    ToggleToolbox,
    ToggleExplorer,
    ToggleProperties,
    ToggleOutput,
    ToggleCommandBar,
    ToggleViewSelector,
    ToggleGrid,
    ToggleCollisions,
    ToggleAnchor,
    PlaySimulation,
    PauseSimulation,
    StopSimulation,
    SavePlace,
    Undo,
    Redo,
}

impl StudioRibbonState {
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        icons: &StudioIcons,
        is_playing: bool,
        is_paused: bool,
        toolbox_visible: bool,
        explorer_visible: bool,
        properties_visible: bool,
        output_visible: bool,
        command_bar_visible: bool,
        view_selector_visible: bool,
        grid_visible: bool,
    ) -> RibbonAction {
        let mut action = RibbonAction::None;

        // =====================================================================
        // 1. BARRA SUPERIOR DE TÍTULO Y QUICK ACCESS (Acceso Rápido)
        // =====================================================================
        let title_bar_h = 30.0;
        let (title_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), title_bar_h), Sense::hover());
        ui.painter().rect_filled(title_rect, 0.0, STUDIO_TOP_BG);

        // Logo Oficial Roblox Studio (Extraído de QtResources)
        let logo_center = Pos2::new(title_rect.min.x + 16.0, title_rect.center().y);
        let logo_rect = Rect::from_center_size(logo_center, Vec2::new(18.0, 18.0));
        ui.painter().image(icons.studio_logo.id(), logo_rect, Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), Color32::WHITE);

        // Quick Access Icons: Save, Undo, Redo (PNGs REALES)
        let mut qa_x = title_rect.min.x + 36.0;

        // Save
        let save_rect = Rect::from_center_size(Pos2::new(qa_x, title_rect.center().y), Vec2::new(20.0, 20.0));
        let save_resp = ui.interact(save_rect, ui.id().with("qa_save"), Sense::click());
        if save_resp.hovered() {
            ui.painter().rect_filled(save_rect, Rounding::same(2.0), STUDIO_BTN_HOVER);
        }
        ui.painter().text(save_rect.center(), egui::Align2::CENTER_CENTER, "💾", FontId::proportional(11.0), Color32::from_rgb(210, 210, 210));
        if save_resp.clicked() { action = RibbonAction::SavePlace; }
        save_resp.on_hover_text("Save to Roblox (Ctrl+S)");
        qa_x += 24.0;

        // Undo (PNG REAL DE STUDIO)
        let undo_rect = Rect::from_center_size(Pos2::new(qa_x, title_rect.center().y), Vec2::new(20.0, 20.0));
        let undo_resp = ui.interact(undo_rect, ui.id().with("qa_undo"), Sense::click());
        if undo_resp.hovered() {
            ui.painter().rect_filled(undo_rect, Rounding::same(2.0), STUDIO_BTN_HOVER);
        }
        let undo_img_rect = Rect::from_center_size(undo_rect.center(), Vec2::new(16.0, 16.0));
        ui.painter().image(icons.qa_undo.id(), undo_img_rect, Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), Color32::WHITE);
        if undo_resp.clicked() { action = RibbonAction::Undo; }
        undo_resp.on_hover_text("Undo (Ctrl+Z)");
        qa_x += 24.0;

        // Redo (PNG REAL DE STUDIO)
        let redo_rect = Rect::from_center_size(Pos2::new(qa_x, title_rect.center().y), Vec2::new(20.0, 20.0));
        let redo_resp = ui.interact(redo_rect, ui.id().with("qa_redo"), Sense::click());
        if redo_resp.hovered() {
            ui.painter().rect_filled(redo_rect, Rounding::same(2.0), STUDIO_BTN_HOVER);
        }
        let redo_img_rect = Rect::from_center_size(redo_rect.center(), Vec2::new(16.0, 16.0));
        ui.painter().image(icons.qa_redo.id(), redo_img_rect, Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), Color32::WHITE);
        if redo_resp.clicked() { action = RibbonAction::Redo; }
        redo_resp.on_hover_text("Redo (Ctrl+Y)");

        // Título de la experiencia centrado
        ui.painter().text(
            title_rect.center(),
            egui::Align2::CENTER_CENTER,
            "Place1* - Roblox Studio [Official Icons Embedded]",
            FontId::proportional(11.5),
            Color32::from_rgb(180, 180, 180),
        );

        // Lado derecho: Botón Collaborate con icono real + Avatar
        let right_x = title_rect.max.x - 16.0;

        // Avatar de usuario
        let avatar_center = Pos2::new(right_x - 10.0, title_rect.center().y);
        ui.painter().circle_filled(avatar_center, 9.0, Color32::from_rgb(50, 120, 220));
        ui.painter().circle_stroke(avatar_center, 9.0, Stroke::new(1.0, Color32::from_rgb(80, 150, 240)));
        ui.painter().text(avatar_center, egui::Align2::CENTER_CENTER, "👤", FontId::proportional(10.0), Color32::WHITE);

        // Botón Collaborate azul con icono PNG real
        let collab_rect = Rect::from_min_max(Pos2::new(right_x - 130.0, title_rect.center().y - 11.0), Pos2::new(right_x - 30.0, title_rect.center().y + 11.0));
        let collab_resp = ui.interact(collab_rect, ui.id().with("btn_collaborate"), Sense::click());
        let collab_bg = if collab_resp.hovered() { Color32::from_rgb(28, 142, 230) } else { STUDIO_SELECTION };
        ui.painter().rect_filled(collab_rect, Rounding::same(4.0), collab_bg);
        let collab_icon_rect = Rect::from_center_size(Pos2::new(collab_rect.min.x + 14.0, collab_rect.center().y), Vec2::new(14.0, 14.0));
        ui.painter().image(icons.action_collab.id(), collab_icon_rect, Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), Color32::WHITE);
        ui.painter().text(Pos2::new(collab_rect.min.x + 28.0, collab_rect.center().y), egui::Align2::LEFT_CENTER, "Collaborate", FontId::proportional(11.0), Color32::WHITE);

        ui.painter().line_segment(
            [Pos2::new(title_rect.min.x, title_rect.max.y), Pos2::new(title_rect.max.x, title_rect.max.y)],
            Stroke::new(1.0, STUDIO_DARK_BORDER),
        );

        // =====================================================================
        // 2. PESTAÑAS DEL RIBBON
        // =====================================================================
        let tabs_bar_h = 28.0;
        let (tabs_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), tabs_bar_h), Sense::hover());
        ui.painter().rect_filled(tabs_rect, 0.0, STUDIO_BG);

        let mut tab_x = tabs_rect.min.x + 8.0;

        let file_rect = Rect::from_min_size(Pos2::new(tab_x, tabs_rect.min.y + 3.0), Vec2::new(48.0, 22.0));
        let file_resp = ui.interact(file_rect, ui.id().with("ribbon_tab_file"), Sense::click());
        let file_bg = if file_resp.hovered() { Color32::from_rgb(60, 60, 65) } else { Color32::from_rgb(48, 48, 52) };
        ui.painter().rect_filled(file_rect, Rounding::same(3.0), file_bg);
        ui.painter().text(file_rect.center(), egui::Align2::CENTER_CENTER, "FILE", FontId::proportional(11.5), Color32::WHITE);
        tab_x += 54.0;

        let tabs = [
            (RibbonTab::Home, "HOME"),
            (RibbonTab::Model, "MODEL"),
            (RibbonTab::Avatar, "AVATAR"),
            (RibbonTab::Test, "TEST"),
            (RibbonTab::View, "VIEW"),
            (RibbonTab::Plugins, "PLUGINS"),
        ];

        for (tab, label) in tabs {
            let is_active = self.current_tab == tab;
            let tab_w = 64.0;
            let rect = Rect::from_min_size(Pos2::new(tab_x, tabs_rect.min.y + 2.0), Vec2::new(tab_w, tabs_bar_h - 2.0));
            let resp = ui.interact(rect, ui.id().with(label), Sense::click());

            let bg = if is_active {
                STUDIO_TAB_ACTIVE
            } else if resp.hovered() {
                STUDIO_TAB_HOVER
            } else {
                STUDIO_BG
            };

            ui.painter().rect_filled(rect, Rounding { nw: 3.0, ne: 3.0, sw: 0.0, se: 0.0 }, bg);

            if is_active {
                ui.painter().line_segment(
                    [Pos2::new(rect.min.x + 2.0, rect.min.y), Pos2::new(rect.max.x - 2.0, rect.min.y)],
                    Stroke::new(2.5, STUDIO_SELECTION),
                );
            }

            let text_color = if is_active { Color32::WHITE } else { STUDIO_TEXT };
            ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, label, FontId::proportional(11.5), text_color);

            if resp.clicked() {
                self.current_tab = tab;
            }

            tab_x += tab_w + 2.0;
        }

        ui.painter().line_segment(
            [Pos2::new(tabs_rect.min.x, tabs_rect.max.y), Pos2::new(tabs_rect.max.x, tabs_rect.max.y)],
            Stroke::new(1.0, STUDIO_BORDER),
        );

        // =====================================================================
        // 3. CUERPO DEL RIBBON (HERRAMIENTAS CON ICONOS REALES)
        // =====================================================================
        let ribbon_body_h = 84.0;
        let (body_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), ribbon_body_h), Sense::hover());
        ui.painter().rect_filled(body_rect, 0.0, STUDIO_TAB_ACTIVE);

        let mut child_ui = ui.child_ui(body_rect, egui::Layout::left_to_right(egui::Align::Min), None);

        match self.current_tab {
            RibbonTab::Home => {
                self.draw_home_ribbon(&mut child_ui, icons, is_playing, is_paused, toolbox_visible, &mut action);
            }
            RibbonTab::Model => {
                self.draw_model_ribbon(&mut child_ui, icons, &mut action);
            }
            RibbonTab::Avatar => {
                self.draw_avatar_ribbon(&mut child_ui, icons);
            }
            RibbonTab::Test => {
                self.draw_test_ribbon(&mut child_ui, icons, is_playing, is_paused, &mut action);
            }
            RibbonTab::View => {
                self.draw_view_ribbon(
                    &mut child_ui,
                    icons,
                    explorer_visible,
                    properties_visible,
                    toolbox_visible,
                    output_visible,
                    command_bar_visible,
                    view_selector_visible,
                    grid_visible,
                    &mut action,
                );
            }
            RibbonTab::Plugins => {
                self.draw_plugins_ribbon(&mut child_ui, icons);
            }
        }

        ui.painter().line_segment(
            [Pos2::new(body_rect.min.x, body_rect.max.y), Pos2::new(body_rect.max.x, body_rect.max.y)],
            Stroke::new(1.0, STUDIO_DARK_BORDER),
        );

        action
    }

    // --- HOME RIBBON CON ICONOS REALES ---
    fn draw_home_ribbon(
        &mut self,
        ui: &mut egui::Ui,
        icons: &StudioIcons,
        is_playing: bool,
        is_paused: bool,
        toolbox_visible: bool,
        action: &mut RibbonAction,
    ) {
        ui.add_space(8.0);

        // 1. Tools Group (Select, Move, Rotate, Scale) - ICONOS REALES
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if draw_real_icon_button(ui, "Select", self.current_tool == StudioTool::Select, &icons.tool_select) {
                    self.current_tool = StudioTool::Select;
                    *action = RibbonAction::ChangeTool(StudioTool::Select);
                }
                if draw_real_icon_button(ui, "Move", self.current_tool == StudioTool::Move, &icons.tool_move) {
                    self.current_tool = StudioTool::Move;
                    *action = RibbonAction::ChangeTool(StudioTool::Move);
                }
                if draw_real_icon_button(ui, "Rotate", self.current_tool == StudioTool::Rotate, &icons.tool_rotate) {
                    self.current_tool = StudioTool::Rotate;
                    *action = RibbonAction::ChangeTool(StudioTool::Rotate);
                }
                if draw_real_icon_button(ui, "Scale", self.current_tool == StudioTool::Scale, &icons.tool_scale) {
                    self.current_tool = StudioTool::Scale;
                    *action = RibbonAction::ChangeTool(StudioTool::Scale);
                }
            });
            draw_group_footer(ui, "Tools");
        });

        draw_ribbon_divider(ui);

        // 2. Terrain Group - ICONO REAL
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if draw_real_icon_button(ui, "Editor", false, &icons.win_terrain) {
                    *action = RibbonAction::InsertPart(PartShape::Block);
                }
            });
            draw_group_footer(ui, "Terrain");
        });

        draw_ribbon_divider(ui);

        // 3. Insert Group (Part, Toolbox) - ICONOS REALES
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if draw_real_icon_button(ui, "Part", false, &icons.insert_part) {
                    *action = RibbonAction::InsertPart(PartShape::Block);
                }
                if draw_real_icon_button(ui, "Toolbox", toolbox_visible, &icons.win_toolbox) {
                    *action = RibbonAction::ToggleToolbox;
                }
            });
            draw_group_footer(ui, "Insert");
        });

        draw_ribbon_divider(ui);

        // 4. Edit Group (Color, Anchor, Collisions) - ICONOS REALES
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.add_space(4.0);
                    let (btn_rect, btn_resp) = ui.allocate_exact_size(Vec2::new(36.0, 24.0), Sense::click());
                    ui.painter().rect_filled(btn_rect, Rounding::same(3.0), Color32::from_rgb(163, 162, 165));
                    ui.painter().rect_stroke(btn_rect, Rounding::same(3.0), Stroke::new(1.0, STUDIO_BORDER));
                    if btn_resp.hovered() {
                        ui.painter().rect_stroke(btn_rect, Rounding::same(3.0), Stroke::new(1.5, STUDIO_SELECTION));
                    }
                    ui.label(egui::RichText::new("Color").size(10.0).color(STUDIO_TEXT_MUTED));
                });

                if draw_real_icon_button(ui, "Anchor", false, &icons.action_anchor) {
                    *action = RibbonAction::ToggleAnchor;
                }

                ui.vertical(|ui| {
                    ui.add_space(8.0);
                    if ui.checkbox(&mut self.collisions_enabled, "Collisions").changed() {
                        *action = RibbonAction::ToggleCollisions;
                    }
                    ui.checkbox(&mut self.join_surfaces, "Join Surfaces");
                });
            });
            draw_group_footer(ui, "Edit");
        });

        draw_ribbon_divider(ui);

        // 5. Simulation / Test Group - ICONOS REALES
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if draw_real_icon_button(ui, if is_playing { "Playing" } else { "Play" }, is_playing, &icons.sim_play) {
                    *action = RibbonAction::PlaySimulation;
                }

                if is_playing {
                    if draw_real_icon_button(ui, if is_paused { "Resume" } else { "Pause" }, is_paused, &icons.sim_pause) {
                        *action = RibbonAction::PauseSimulation;
                    }

                    if draw_real_icon_button(ui, "Stop", false, &icons.sim_stop) {
                        *action = RibbonAction::StopSimulation;
                    }
                }
            });
            draw_group_footer(ui, "Test");
        });

        draw_ribbon_divider(ui);

        // 6. Settings Group - ICONO REAL
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                draw_real_icon_button(ui, "Game Settings", false, &icons.action_gamesettings);
            });
            draw_group_footer(ui, "Settings");
        });
    }

    // --- MODEL RIBBON CON ICONOS REALES ---
    fn draw_model_ribbon(&mut self, ui: &mut egui::Ui, icons: &StudioIcons, action: &mut RibbonAction) {
        ui.add_space(8.0);
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if draw_real_icon_button(ui, "Select", self.current_tool == StudioTool::Select, &icons.tool_select) {
                    self.current_tool = StudioTool::Select;
                }
                if draw_real_icon_button(ui, "Move", self.current_tool == StudioTool::Move, &icons.tool_move) {
                    self.current_tool = StudioTool::Move;
                }
                if draw_real_icon_button(ui, "Rotate", self.current_tool == StudioTool::Rotate, &icons.tool_rotate) {
                    self.current_tool = StudioTool::Rotate;
                }
                if draw_real_icon_button(ui, "Scale", self.current_tool == StudioTool::Scale, &icons.tool_scale) {
                    self.current_tool = StudioTool::Scale;
                }
                if draw_real_icon_button(ui, "Transform", false, &icons.tool_transform) {}
            });
            draw_group_footer(ui, "Tools");
        });

        draw_ribbon_divider(ui);

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut self.rotate_snap_enabled, "Rotate");
                        ui.add(egui::DragValue::new(&mut self.rotate_snap_deg).suffix("°").speed(5.0));
                    });
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut self.move_snap_enabled, "Move");
                        ui.add(egui::DragValue::new(&mut self.move_snap_studs).suffix(" studs").speed(0.5));
                    });
                });
            });
            draw_group_footer(ui, "Snap to Grid");
        });

        draw_ribbon_divider(ui);

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if draw_real_icon_button(ui, "Spawn", false, &icons.insert_spawn) {
                    *action = RibbonAction::InsertPart(PartShape::Block);
                }
                draw_real_icon_button(ui, "PointLight", false, &icons.insert_light);
            });
            draw_group_footer(ui, "Gameplay");
        });
    }

    // --- AVATAR RIBBON CON ICONOS REALES ---
    fn draw_avatar_ribbon(&mut self, ui: &mut egui::Ui, icons: &StudioIcons) {
        ui.add_space(8.0);
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                draw_real_icon_button(ui, "Rig Builder", false, &icons.win_buildrig);
                draw_real_icon_button(ui, "Animation Editor", false, &icons.win_animeditor);
            });
            draw_group_footer(ui, "Rig & Animation");
        });
    }

    // --- TEST RIBBON CON ICONOS REALES ---
    fn draw_test_ribbon(
        &mut self,
        ui: &mut egui::Ui,
        icons: &StudioIcons,
        is_playing: bool,
        is_paused: bool,
        action: &mut RibbonAction,
    ) {
        ui.add_space(8.0);
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if draw_real_icon_button(ui, if is_playing { "Playing" } else { "Play" }, is_playing, &icons.sim_play) {
                    *action = RibbonAction::PlaySimulation;
                }
                if draw_real_icon_button(ui, "Play Here", false, &icons.sim_playhere) {
                    *action = RibbonAction::PlaySimulation;
                }
                if draw_real_icon_button(ui, "Run", false, &icons.sim_run) {
                    *action = RibbonAction::PlaySimulation;
                }

                if is_playing {
                    if draw_real_icon_button(ui, if is_paused { "Resume" } else { "Pause" }, is_paused, &icons.sim_pause) {
                        *action = RibbonAction::PauseSimulation;
                    }

                    if draw_real_icon_button(ui, "Stop", false, &icons.sim_stop) {
                        *action = RibbonAction::StopSimulation;
                    }
                }
            });
            draw_group_footer(ui, "Simulation");
        });

        draw_ribbon_divider(ui);

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                draw_real_icon_button(ui, "Team Create", false, &icons.win_teamcreate);
            });
            draw_group_footer(ui, "Collaboration");
        });
    }

    // --- VIEW RIBBON CON ICONOS REALES ---
    fn draw_view_ribbon(
        &mut self,
        ui: &mut egui::Ui,
        icons: &StudioIcons,
        explorer_vis: bool,
        properties_vis: bool,
        toolbox_vis: bool,
        output_vis: bool,
        command_bar_vis: bool,
        view_selector_vis: bool,
        grid_vis: bool,
        action: &mut RibbonAction,
    ) {
        ui.add_space(8.0);

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if draw_real_icon_button(ui, "Explorer", explorer_vis, &icons.win_explorer) {
                    *action = RibbonAction::ToggleExplorer;
                }
                if draw_real_icon_button(ui, "Properties", properties_vis, &icons.win_properties) {
                    *action = RibbonAction::ToggleProperties;
                }
                if draw_real_icon_button(ui, "Toolbox", toolbox_vis, &icons.win_toolbox) {
                    *action = RibbonAction::ToggleToolbox;
                }
                if draw_real_icon_button(ui, "Output", output_vis, &icons.win_output) {
                    *action = RibbonAction::ToggleOutput;
                }
                if draw_real_icon_button(ui, "Command Bar", command_bar_vis, &icons.win_cmdbar) {
                    *action = RibbonAction::ToggleCommandBar;
                }
                if draw_real_icon_button(ui, "Asset Mgr", false, &icons.win_assetmgr) {}
            });
            draw_group_footer(ui, "Studio Windows");
        });

        draw_ribbon_divider(ui);

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if draw_real_icon_button(ui, "View Selector", view_selector_vis, &icons.view_front) {
                    *action = RibbonAction::ToggleViewSelector;
                }
                if draw_large_tool_button(ui, "Grid", grid_vis, |p, r| {
                    icons::grid_icon(p, r, Color32::WHITE);
                }) {
                    *action = RibbonAction::ToggleGrid;
                }
            });
            draw_group_footer(ui, "Display");
        });
    }

    // --- PLUGINS RIBBON CON ICONOS REALES ---
    fn draw_plugins_ribbon(&mut self, ui: &mut egui::Ui, icons: &StudioIcons) {
        ui.add_space(8.0);
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                draw_real_icon_button(ui, "Manage Plugins", false, &icons.win_plugins);
            });
            draw_group_footer(ui, "Plugin Tools");
        });
    }
}

// =========================================================================
// RENDERIZADO DE BOTONES CON ICONOS REALES DE ROBLOX STUDIO
// =========================================================================

fn draw_real_icon_button(ui: &mut egui::Ui, label: &str, is_active: bool, tex: &egui::TextureHandle) -> bool {
    let btn_w = 48.0;
    let btn_h = 56.0;
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(btn_w, btn_h), Sense::click());

    let bg = if is_active {
        Color32::from_rgb(38, 65, 95) // Active blue tint
    } else if resp.hovered() {
        STUDIO_BTN_HOVER
    } else {
        Color32::TRANSPARENT
    };

    ui.painter().rect_filled(rect, Rounding::same(3.0), bg);

    if is_active {
        ui.painter().rect_stroke(rect, Rounding::same(3.0), Stroke::new(1.0, STUDIO_SELECTION));
    }

    // Renderizar imagen PNG real centrada (22x22 px)
    let icon_rect = Rect::from_center_size(Pos2::new(rect.center().x, rect.min.y + 18.0), Vec2::new(22.0, 22.0));
    ui.painter().image(
        tex.id(),
        icon_rect,
        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
        Color32::WHITE,
    );

    // Label de la herramienta
    let label_rect = Rect::from_min_size(Pos2::new(rect.min.x, rect.min.y + 36.0), Vec2::new(btn_w, 18.0));
    ui.painter().text(
        label_rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        FontId::proportional(10.0),
        if is_active { Color32::WHITE } else { STUDIO_TEXT },
    );

    resp.clicked()
}

fn draw_large_tool_button<F>(ui: &mut egui::Ui, label: &str, is_active: bool, draw_icon: F) -> bool
where
    F: FnOnce(&egui::Painter, Rect),
{
    let btn_w = 46.0;
    let btn_h = 56.0;
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(btn_w, btn_h), Sense::click());

    let bg = if is_active {
        Color32::from_rgb(40, 65, 95)
    } else if resp.hovered() {
        STUDIO_BTN_HOVER
    } else {
        Color32::TRANSPARENT
    };

    ui.painter().rect_filled(rect, Rounding::same(3.0), bg);
    if is_active {
        ui.painter().rect_stroke(rect, Rounding::same(3.0), Stroke::new(1.0, STUDIO_SELECTION));
    }

    let icon_rect = Rect::from_min_size(rect.min, Vec2::new(btn_w, 34.0));
    draw_icon(ui.painter(), icon_rect);

    let label_rect = Rect::from_min_size(Pos2::new(rect.min.x, rect.min.y + 36.0), Vec2::new(btn_w, 18.0));
    ui.painter().text(
        label_rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        FontId::proportional(10.0),
        if is_active { Color32::WHITE } else { STUDIO_TEXT },
    );

    resp.clicked()
}

fn draw_group_footer(ui: &mut egui::Ui, label: &str) {
    ui.add_space(2.0);
    ui.vertical_centered(|ui| {
        ui.label(egui::RichText::new(label).size(10.0).color(STUDIO_TEXT_MUTED));
    });
}

fn draw_ribbon_divider(ui: &mut egui::Ui) {
    ui.add_space(6.0);
    let (rect, _) = ui.allocate_exact_size(Vec2::new(1.0, 68.0), Sense::hover());
    ui.painter().line_segment(
        [Pos2::new(rect.min.x, rect.min.y + 4.0), Pos2::new(rect.min.x, rect.max.y - 4.0)],
        Stroke::new(1.0, STUDIO_BORDER),
    );
    ui.add_space(6.0);
}

fn draw_roblox_studio_logo(painter: &egui::Painter, center: Pos2, size: f32) {
    let s = size * 0.5;
    let angle: f32 = -0.26;
    let cos_a = angle.cos();
    let sin_a = angle.sin();

    let rotate_pt = |dx: f32, dy: f32| -> Pos2 {
        Pos2::new(center.x + (dx * cos_a - dy * sin_a), center.y + (dx * sin_a + dy * cos_a))
    };

    let p1 = rotate_pt(-s, -s);
    let p2 = rotate_pt(s, -s);
    let p3 = rotate_pt(s, s);
    let p4 = rotate_pt(-s, s);

    painter.add(egui::Shape::convex_polygon(
        vec![p1, p2, p3, p4],
        Color32::from_rgb(0, 162, 255),
        Stroke::NONE,
    ));

    let h = s * 0.45;
    let h1 = rotate_pt(-h, -h);
    let h2 = rotate_pt(h, -h);
    let h3 = rotate_pt(h, h);
    let h4 = rotate_pt(-h, h);

    painter.add(egui::Shape::convex_polygon(
        vec![h1, h2, h3, h4],
        STUDIO_TOP_BG,
        Stroke::NONE,
    ));
}
