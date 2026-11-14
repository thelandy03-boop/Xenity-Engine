use eframe::egui::{self, Color32, CursorIcon, FontId, Pos2, Rect, Rounding, Sense, Stroke, Vec2};
use crate::studio_style::*;
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

        // Logo Roblox Studio (Cuadrado rotado con agujero)
        let logo_center = Pos2::new(title_rect.min.x + 16.0, title_rect.center().y);
        draw_roblox_studio_logo(ui.painter(), logo_center, 14.0);

        // Quick Access Icons: Save, Undo, Redo
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

        // Undo
        let undo_rect = Rect::from_center_size(Pos2::new(qa_x, title_rect.center().y), Vec2::new(20.0, 20.0));
        let undo_resp = ui.interact(undo_rect, ui.id().with("qa_undo"), Sense::click());
        if undo_resp.hovered() {
            ui.painter().rect_filled(undo_rect, Rounding::same(2.0), STUDIO_BTN_HOVER);
        }
        ui.painter().text(undo_rect.center(), egui::Align2::CENTER_CENTER, "↶", FontId::proportional(14.0), Color32::from_rgb(210, 210, 210));
        if undo_resp.clicked() { action = RibbonAction::Undo; }
        undo_resp.on_hover_text("Undo (Ctrl+Z)");
        qa_x += 24.0;

        // Redo
        let redo_rect = Rect::from_center_size(Pos2::new(qa_x, title_rect.center().y), Vec2::new(20.0, 20.0));
        let redo_resp = ui.interact(redo_rect, ui.id().with("qa_redo"), Sense::click());
        if redo_resp.hovered() {
            ui.painter().rect_filled(redo_rect, Rounding::same(2.0), STUDIO_BTN_HOVER);
        }
        ui.painter().text(redo_rect.center(), egui::Align2::CENTER_CENTER, "↷", FontId::proportional(14.0), Color32::from_rgb(150, 150, 150));
        if redo_resp.clicked() { action = RibbonAction::Redo; }
        redo_resp.on_hover_text("Redo (Ctrl+Y)");

        // Título de la experiencia centrado
        ui.painter().text(
            title_rect.center(),
            egui::Align2::CENTER_CENTER,
            "Place1* - Roblox Studio [Xenity Engine 60 FPS]",
            FontId::proportional(11.5),
            Color32::from_rgb(180, 180, 180),
        );

        // Lado derecho de la barra de título: Botón Collaborate + Avatar de Usuario
        let right_x = title_rect.max.x - 16.0;

        // Avatar de usuario circular
        let avatar_center = Pos2::new(right_x - 10.0, title_rect.center().y);
        ui.painter().circle_filled(avatar_center, 9.0, Color32::from_rgb(50, 120, 220));
        ui.painter().circle_stroke(avatar_center, 9.0, Stroke::new(1.0, Color32::from_rgb(80, 150, 240)));
        ui.painter().text(avatar_center, egui::Align2::CENTER_CENTER, "👤", FontId::proportional(10.0), Color32::WHITE);

        // Botón Collaborate azul de Roblox Studio
        let collab_rect = Rect::from_min_max(Pos2::new(right_x - 130.0, title_rect.center().y - 11.0), Pos2::new(right_x - 30.0, title_rect.center().y + 11.0));
        let collab_resp = ui.interact(collab_rect, ui.id().with("btn_collaborate"), Sense::click());
        let collab_bg = if collab_resp.hovered() { Color32::from_rgb(28, 142, 230) } else { STUDIO_SELECTION };
        ui.painter().rect_filled(collab_rect, Rounding::same(4.0), collab_bg);
        ui.painter().text(collab_rect.center(), egui::Align2::CENTER_CENTER, "👥 Collaborate", FontId::proportional(11.0), Color32::WHITE);

        // Línea divisoria bajo el title bar
        ui.painter().line_segment(
            [Pos2::new(title_rect.min.x, title_rect.max.y), Pos2::new(title_rect.max.x, title_rect.max.y)],
            Stroke::new(1.0, STUDIO_DARK_BORDER),
        );

        // =====================================================================
        // 2. PESTAÑAS DEL RIBBON (FILE, HOME, MODEL, AVATAR, TEST, VIEW, PLUGINS)
        // =====================================================================
        let tabs_bar_h = 28.0;
        let (tabs_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), tabs_bar_h), Sense::hover());
        ui.painter().rect_filled(tabs_rect, 0.0, STUDIO_BG);

        let mut tab_x = tabs_rect.min.x + 8.0;

        // Botón FILE especial de Roblox Studio
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
                // Línea de acento azul superior
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
        // 3. CUERPO DEL RIBBON (ACCIONES Y HERRAMIENTAS CATEGORIZADAS)
        // =====================================================================
        let ribbon_body_h = 84.0;
        let (body_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), ribbon_body_h), Sense::hover());
        ui.painter().rect_filled(body_rect, 0.0, STUDIO_TAB_ACTIVE);

        let mut child_ui = ui.child_ui(body_rect, egui::Layout::left_to_right(egui::Align::Min), None);

        match self.current_tab {
            RibbonTab::Home => {
                self.draw_home_ribbon(&mut child_ui, is_playing, is_paused, toolbox_visible, &mut action);
            }
            RibbonTab::Model => {
                self.draw_model_ribbon(&mut child_ui, &mut action);
            }
            RibbonTab::Avatar => {
                self.draw_avatar_ribbon(&mut child_ui);
            }
            RibbonTab::Test => {
                self.draw_test_ribbon(&mut child_ui, is_playing, is_paused, &mut action);
            }
            RibbonTab::View => {
                self.draw_view_ribbon(
                    &mut child_ui,
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
                self.draw_plugins_ribbon(&mut child_ui);
            }
        }

        ui.painter().line_segment(
            [Pos2::new(body_rect.min.x, body_rect.max.y), Pos2::new(body_rect.max.x, body_rect.max.y)],
            Stroke::new(1.0, STUDIO_DARK_BORDER),
        );

        action
    }

    // --- SUB-RIBBON: HOME ---
    fn draw_home_ribbon(
        &mut self,
        ui: &mut egui::Ui,
        is_playing: bool,
        is_paused: bool,
        toolbox_visible: bool,
        action: &mut RibbonAction,
    ) {
        ui.add_space(8.0);

        // 1. Tools Group (Select, Move, Rotate, Scale)
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if draw_large_tool_button(ui, "Select", self.current_tool == StudioTool::Select, |p, r| {
                    icons::ribbon_select(p, r.center(), 24.0, Color32::WHITE);
                }) {
                    self.current_tool = StudioTool::Select;
                    *action = RibbonAction::ChangeTool(StudioTool::Select);
                }

                if draw_large_tool_button(ui, "Move", self.current_tool == StudioTool::Move, |p, r| {
                    icons::ribbon_move(p, r.center(), 22.0, Color32::WHITE);
                }) {
                    self.current_tool = StudioTool::Move;
                    *action = RibbonAction::ChangeTool(StudioTool::Move);
                }

                if draw_large_tool_button(ui, "Rotate", self.current_tool == StudioTool::Rotate, |p, r| {
                    icons::ribbon_rotate(p, r.center(), 22.0, Color32::WHITE);
                }) {
                    self.current_tool = StudioTool::Rotate;
                    *action = RibbonAction::ChangeTool(StudioTool::Rotate);
                }

                if draw_large_tool_button(ui, "Scale", self.current_tool == StudioTool::Scale, |p, r| {
                    icons::ribbon_scale(p, r.center(), 22.0, Color32::WHITE);
                }) {
                    self.current_tool = StudioTool::Scale;
                    *action = RibbonAction::ChangeTool(StudioTool::Scale);
                }
            });
            draw_group_footer(ui, "Tools");
        });

        draw_ribbon_divider(ui);

        // 2. Terrain Group
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if draw_large_tool_button(ui, "Editor", false, |p, r| {
                    icons::ribbon_terrain(p, r.center(), 24.0);
                }) {
                    *action = RibbonAction::InsertPart(PartShape::Block);
                }
            });
            draw_group_footer(ui, "Terrain");
        });

        draw_ribbon_divider(ui);

        // 3. Insert Group (Part dropdown, UI, Toolbox)
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if draw_large_tool_button(ui, "Part", false, |p, r| {
                    icons::ribbon_part(p, r.center(), 24.0);
                }) {
                    *action = RibbonAction::InsertPart(PartShape::Block);
                }

                if draw_large_tool_button(ui, "Toolbox", toolbox_visible, |p, r| {
                    icons::ribbon_toolbox(p, r.center(), 24.0);
                }) {
                    *action = RibbonAction::ToggleToolbox;
                }
            });
            draw_group_footer(ui, "Insert");
        });

        draw_ribbon_divider(ui);

        // 4. Edit Group (Material, Color, Group, Anchor, Collisions)
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                // Color swatch
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

                // Anchor button
                if draw_large_tool_button(ui, "Anchor", false, |p, r| {
                    icons::ribbon_anchor(p, r.center(), 24.0, false);
                }) {
                    *action = RibbonAction::ToggleAnchor;
                }

                // Collisions checkbox
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

        // 5. Simulation / Test Group (Play, Pause, Stop)
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                // Big Play Button
                if draw_play_button(ui, is_playing) {
                    *action = RibbonAction::PlaySimulation;
                }

                if is_playing {
                    let pause_col = if is_paused { STUDIO_SELECTION } else { Color32::from_rgb(220, 220, 220) };
                    if draw_large_tool_button(ui, if is_paused { "Resume" } else { "Pause" }, is_paused, |p, r| {
                        icons::pause(p, r, pause_col);
                    }) {
                        *action = RibbonAction::PauseSimulation;
                    }

                    if draw_large_tool_button(ui, "Stop", false, |p, r| {
                        icons::stop(p, r, STUDIO_RED);
                    }) {
                        *action = RibbonAction::StopSimulation;
                    }
                }
            });
            draw_group_footer(ui, "Test");
        });

        draw_ribbon_divider(ui);

        // 6. Settings Group
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                draw_large_tool_button(ui, "Game Settings", false, |p, r| {
                    icons::ribbon_settings(p, r.center(), 22.0, STUDIO_TEXT);
                });
            });
            draw_group_footer(ui, "Settings");
        });
    }

    // --- SUB-RIBBON: MODEL ---
    fn draw_model_ribbon(&mut self, ui: &mut egui::Ui, action: &mut RibbonAction) {
        ui.add_space(8.0);

        // Tools
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if draw_large_tool_button(ui, "Select", self.current_tool == StudioTool::Select, |p, r| {
                    icons::ribbon_select(p, r.center(), 22.0, Color32::WHITE);
                }) {
                    self.current_tool = StudioTool::Select;
                }
                if draw_large_tool_button(ui, "Move", self.current_tool == StudioTool::Move, |p, r| {
                    icons::ribbon_move(p, r.center(), 22.0, Color32::WHITE);
                }) {
                    self.current_tool = StudioTool::Move;
                }
                if draw_large_tool_button(ui, "Rotate", self.current_tool == StudioTool::Rotate, |p, r| {
                    icons::ribbon_rotate(p, r.center(), 22.0, Color32::WHITE);
                }) {
                    self.current_tool = StudioTool::Rotate;
                }
                if draw_large_tool_button(ui, "Scale", self.current_tool == StudioTool::Scale, |p, r| {
                    icons::ribbon_scale(p, r.center(), 22.0, Color32::WHITE);
                }) {
                    self.current_tool = StudioTool::Scale;
                }
            });
            draw_group_footer(ui, "Tools");
        });

        draw_ribbon_divider(ui);

        // Snap to Grid settings
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

        // Solid Modeling (Union, Negate, Separate)
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                draw_large_tool_button(ui, "Union", false, |p, r| {
                    p.rect_filled(Rect::from_center_size(r.center(), Vec2::new(16.0, 16.0)), 2.0, Color32::from_rgb(0, 162, 255));
                });
                draw_large_tool_button(ui, "Negate", false, |p, r| {
                    p.rect_filled(Rect::from_center_size(r.center(), Vec2::new(16.0, 16.0)), 2.0, Color32::from_rgb(220, 60, 60));
                });
                draw_large_tool_button(ui, "Separate", false, |p, r| {
                    p.rect_stroke(Rect::from_center_size(r.center(), Vec2::new(16.0, 16.0)), Rounding::same(2.0), Stroke::new(1.5, Color32::WHITE));
                });
            });
            draw_group_footer(ui, "Solid Modeling");
        });

        draw_ribbon_divider(ui);

        // Gameplay
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if draw_large_tool_button(ui, "Spawn", false, |p, r| {
                    icons::roblox_part(p, r.center(), 22.0);
                }) {
                    *action = RibbonAction::InsertPart(PartShape::Block);
                }
                draw_large_tool_button(ui, "Effects", false, |p, r| {
                    p.circle_filled(r.center(), 8.0, Color32::from_rgb(255, 200, 50));
                });
            });
            draw_group_footer(ui, "Gameplay");
        });
    }

    // --- SUB-RIBBON: AVATAR ---
    fn draw_avatar_ribbon(&mut self, ui: &mut egui::Ui) {
        ui.add_space(8.0);
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                draw_large_tool_button(ui, "Rig Builder", false, |p, r| {
                    icons::roblox_humanoid(p, r.center(), 22.0);
                });
                draw_large_tool_button(ui, "Animation Editor", false, |p, r| {
                    p.rect_filled(Rect::from_center_size(r.center(), Vec2::new(18.0, 12.0)), 2.0, Color32::from_rgb(100, 180, 240));
                });
                draw_large_tool_button(ui, "Avatar Setup", false, |p, r| {
                    p.circle_stroke(r.center(), 9.0, Stroke::new(1.5, STUDIO_TEXT));
                });
            });
            draw_group_footer(ui, "Rig & Animation");
        });
    }

    // --- SUB-RIBBON: TEST ---
    fn draw_test_ribbon(
        &mut self,
        ui: &mut egui::Ui,
        is_playing: bool,
        is_paused: bool,
        action: &mut RibbonAction,
    ) {
        ui.add_space(8.0);
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if draw_play_button(ui, is_playing) {
                    *action = RibbonAction::PlaySimulation;
                }
                if draw_large_tool_button(ui, "Play Here", false, |p, r| {
                    p.circle_filled(r.center(), 9.0, Color32::from_rgb(30, 180, 100));
                }) {
                    *action = RibbonAction::PlaySimulation;
                }
                if draw_large_tool_button(ui, "Run", false, |p, r| {
                    p.rect_filled(Rect::from_center_size(r.center(), Vec2::new(16.0, 16.0)), 2.0, Color32::from_rgb(40, 160, 220));
                }) {
                    *action = RibbonAction::PlaySimulation;
                }

                if is_playing {
                    if draw_large_tool_button(ui, if is_paused { "Resume" } else { "Pause" }, is_paused, |p, r| {
                        icons::pause(p, r, Color32::WHITE);
                    }) {
                        *action = RibbonAction::PauseSimulation;
                    }

                    if draw_large_tool_button(ui, "Stop", false, |p, r| {
                        icons::stop(p, r, STUDIO_RED);
                    }) {
                        *action = RibbonAction::StopSimulation;
                    }
                }
            });
            draw_group_footer(ui, "Simulation");
        });

        draw_ribbon_divider(ui);

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                draw_large_tool_button(ui, "Device", false, |p, r| {
                    p.rect_stroke(Rect::from_center_size(r.center(), Vec2::new(14.0, 20.0)), Rounding::same(2.0), Stroke::new(1.4, STUDIO_TEXT));
                });
                draw_large_tool_button(ui, "Clients and Servers", false, |p, r| {
                    p.text(r.center(), egui::Align2::CENTER_CENTER, "🌐", FontId::proportional(14.0), Color32::WHITE);
                });
            });
            draw_group_footer(ui, "Emulation & Network");
        });
    }

    // --- SUB-RIBBON: VIEW (CONMUTADORES DE VENTANAS ACTIVAS DE ROBLOX STUDIO) ---
    fn draw_view_ribbon(
        &mut self,
        ui: &mut egui::Ui,
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
                // Explorer toggle
                if draw_large_tool_button(ui, "Explorer", explorer_vis, |p, r| {
                    icons::roblox_workspace(p, r.center(), 22.0);
                }) {
                    *action = RibbonAction::ToggleExplorer;
                }

                // Properties toggle
                if draw_large_tool_button(ui, "Properties", properties_vis, |p, r| {
                    p.rect_stroke(Rect::from_center_size(r.center(), Vec2::new(16.0, 16.0)), Rounding::same(2.0), Stroke::new(1.4, Color32::WHITE));
                    p.line_segment([Pos2::new(r.center().x - 4.0, r.center().y), Pos2::new(r.center().x + 4.0, r.center().y)], Stroke::new(1.2, Color32::WHITE));
                }) {
                    *action = RibbonAction::ToggleProperties;
                }

                // Toolbox toggle
                if draw_large_tool_button(ui, "Toolbox", toolbox_vis, |p, r| {
                    icons::ribbon_toolbox(p, r.center(), 22.0);
                }) {
                    *action = RibbonAction::ToggleToolbox;
                }

                // Output toggle
                if draw_large_tool_button(ui, "Output", output_vis, |p, r| {
                    p.text(r.center(), egui::Align2::CENTER_CENTER, "🖵", FontId::proportional(16.0), Color32::WHITE);
                }) {
                    *action = RibbonAction::ToggleOutput;
                }

                // Command Bar toggle
                if draw_large_tool_button(ui, "Command Bar", command_bar_vis, |p, r| {
                    p.text(r.center(), egui::Align2::CENTER_CENTER, ">_", FontId::proportional(13.0), Color32::WHITE);
                }) {
                    *action = RibbonAction::ToggleCommandBar;
                }
            });
            draw_group_footer(ui, "Studio Windows");
        });

        draw_ribbon_divider(ui);

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                // View Selector Cube toggle
                if draw_large_tool_button(ui, "View Selector", view_selector_vis, |p, r| {
                    p.rect_filled(Rect::from_center_size(r.center(), Vec2::new(14.0, 14.0)), 1.0, Color32::from_rgb(0, 162, 255));
                }) {
                    *action = RibbonAction::ToggleViewSelector;
                }

                // Grid toggle
                if draw_large_tool_button(ui, "Grid", grid_vis, |p, r| {
                    icons::grid_icon(p, r, Color32::WHITE);
                }) {
                    *action = RibbonAction::ToggleGrid;
                }
            });
            draw_group_footer(ui, "Display");
        });
    }

    // --- SUB-RIBBON: PLUGINS ---
    fn draw_plugins_ribbon(&mut self, ui: &mut egui::Ui) {
        ui.add_space(8.0);
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                draw_large_tool_button(ui, "Manage Plugins", false, |p, r| {
                    p.text(r.center(), egui::Align2::CENTER_CENTER, "🔌", FontId::proportional(16.0), Color32::WHITE);
                });
                draw_large_tool_button(ui, "Plugins Folder", false, |p, r| {
                    p.text(r.center(), egui::Align2::CENTER_CENTER, "📁", FontId::proportional(16.0), Color32::WHITE);
                });
            });
            draw_group_footer(ui, "Plugin Tools");
        });
    }
}

// =========================================================================
// WIDGETS AUXILIARES DEL RIBBON DE ROBLOX STUDIO
// =========================================================================

fn draw_large_tool_button<F>(ui: &mut egui::Ui, label: &str, is_active: bool, draw_icon: F) -> bool
where
    F: FnOnce(&egui::Painter, Rect),
{
    let btn_w = 46.0;
    let btn_h = 56.0;
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(btn_w, btn_h), Sense::click());

    let bg = if is_active {
        Color32::from_rgb(40, 65, 95) // Active blue tint
    } else if resp.hovered() {
        STUDIO_BTN_HOVER
    } else {
        Color32::TRANSPARENT
    };

    ui.painter().rect_filled(rect, Rounding::same(3.0), bg);

    if is_active {
        ui.painter().rect_stroke(rect, Rounding::same(3.0), Stroke::new(1.0, STUDIO_SELECTION));
    }

    // Icon area
    let icon_rect = Rect::from_min_size(rect.min, Vec2::new(btn_w, 34.0));
    draw_icon(ui.painter(), icon_rect);

    // Label area
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

fn draw_play_button(ui: &mut egui::Ui, is_playing: bool) -> bool {
    let btn_w = 46.0;
    let btn_h = 56.0;
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(btn_w, btn_h), Sense::click());

    let bg = if resp.hovered() {
        Color32::from_rgb(25, 185, 95)
    } else {
        STUDIO_GREEN
    };

    ui.painter().rect_filled(rect, Rounding::same(3.0), bg);

    let icon_center = Pos2::new(rect.center().x, rect.min.y + 18.0);
    // Triángulo play blanco
    let s = 8.0;
    let p1 = Pos2::new(icon_center.x - s * 0.5, icon_center.y - s);
    let p2 = Pos2::new(icon_center.x + s * 0.7, icon_center.y);
    let p3 = Pos2::new(icon_center.x - s * 0.5, icon_center.y + s);
    ui.painter().add(egui::Shape::convex_polygon(vec![p1, p2, p3], Color32::WHITE, Stroke::NONE));

    let label_y = rect.min.y + 42.0;
    ui.painter().text(
        Pos2::new(rect.center().x, label_y),
        egui::Align2::CENTER_CENTER,
        if is_playing { "Playing" } else { "Play" },
        FontId::proportional(11.0),
        Color32::WHITE,
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
    // Cuadrado inclinado azul icónico de Roblox Studio
    let angle: f32 = -0.26; // ~15 grados
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

    // Agujero cuadrado central
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
