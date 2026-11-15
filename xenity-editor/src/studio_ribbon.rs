use eframe::egui::{self, Color32, CursorIcon, FontId, Pos2, Rect, Rounding, Sense, Stroke, TextureHandle, Vec2};
use crate::studio_style::*;
use crate::studio_textures::StudioIcons;
use crate::icons;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum RibbonTab {
    Home,
    Avatar,
    UI,
    Script,
    Model,
    Plugins,
    Untitled,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum StudioTool {
    Select,
    Move,
    Scale,
    Rotate,
    Transform,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum PartShape {
    Block,
    Sphere,
    Wedge,
    Cylinder,
}

#[derive(Clone, Debug)]
pub struct PluginRibbonItem {
    pub name: String,
    pub section: usize,
    pub icon_color: Color32,
    pub icon_text: String,
}

pub struct StudioRibbonState {
    pub current_tab: RibbonTab,
    pub current_tool: StudioTool,
    pub part_dropdown_open: bool,
    pub play_dropdown_open: bool,
    pub probar_dropdown_open: bool,
    pub probar_mode: usize,

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

    // Plugins
    pub plugins: Vec<PluginRibbonItem>,
}

impl Default for StudioRibbonState {
    fn default() -> Self {
        Self {
            current_tab: RibbonTab::Home,
            current_tool: StudioTool::Select,
            part_dropdown_open: false,
            play_dropdown_open: false,
            probar_dropdown_open: false,
            probar_mode: 0,

            move_snap_enabled: true,
            move_snap_studs: 21.0,
            rotate_snap_enabled: true,
            rotate_snap_deg: 45.0,
            collisions_enabled: true,
            join_surfaces: true,

            can_undo: true,
            can_redo: false,

            plugins: Vec::new(),
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
        // 1. BARRA SUPERIOR DE TÍTULO
        // =====================================================================
        let title_bar_h = 30.0;
        let (title_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), title_bar_h), Sense::hover());
        ui.painter().rect_filled(title_rect, 0.0, STUDIO_TOP_BG);

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
        // 2. BARRA SUPERIOR MODERNA (PROBAR + CONTROLES + PESTAÑAS PÍLDORA)
        // =====================================================================
        let tabs_bar_h = 32.0;
        let (tabs_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), tabs_bar_h), Sense::hover());
        ui.painter().rect_filled(tabs_rect, 0.0, Color32::from_rgb(22, 23, 27));

        let mut cur_x = tabs_rect.min.x + 8.0;
        let bar_center_y = tabs_rect.center().y;

        // --- LADO IZQUIERDO: Botón "Probar ▾" ---
        let probar_w = 68.0;
        let probar_rect = Rect::from_center_size(Pos2::new(cur_x + probar_w * 0.5, bar_center_y), Vec2::new(probar_w, 24.0));
        let probar_resp = ui.interact(probar_rect, ui.id().with("btn_probar_dropdown"), Sense::click());
        let probar_bg = if self.probar_dropdown_open {
            Color32::from_rgb(34, 35, 40)
        } else if probar_resp.hovered() {
            Color32::from_rgb(46, 48, 54)
        } else {
            Color32::from_rgb(38, 40, 46)
        };
        let probar_stroke = if self.probar_dropdown_open {
            Stroke::new(1.5, Color32::from_rgb(0, 136, 255))
        } else {
            Stroke::new(1.0, Color32::from_rgb(56, 58, 66))
        };
        ui.painter().rect(probar_rect, Rounding::same(4.0), probar_bg, probar_stroke);

        let cur_label = match self.probar_mode {
            1 => "Probar",
            2 => "Ejecutar",
            _ => "Probar",
        };
        ui.painter().text(
            Pos2::new(probar_rect.min.x + 10.0, bar_center_y),
            egui::Align2::LEFT_CENTER,
            cur_label,
            FontId::proportional(11.5),
            Color32::from_rgb(225, 226, 232),
        );
        let p_arrow_x = probar_rect.max.x - 12.0;
        if self.probar_dropdown_open {
            // Flecha ▴ hacia arriba
            let p1 = Pos2::new(p_arrow_x - 3.0, bar_center_y + 1.5);
            let p2 = Pos2::new(p_arrow_x + 3.0, bar_center_y + 1.5);
            let p3 = Pos2::new(p_arrow_x, bar_center_y - 2.5);
            ui.painter().add(egui::Shape::convex_polygon(vec![p1, p2, p3], Color32::WHITE, Stroke::NONE));
        } else {
            // Flecha ▾ hacia abajo
            let p1 = Pos2::new(p_arrow_x - 3.0, bar_center_y - 1.5);
            let p2 = Pos2::new(p_arrow_x + 3.0, bar_center_y - 1.5);
            let p3 = Pos2::new(p_arrow_x, bar_center_y + 2.5);
            ui.painter().add(egui::Shape::convex_polygon(vec![p1, p2, p3], Color32::from_rgb(165, 167, 175), Stroke::NONE));
        }
        if probar_resp.clicked() {
            // ÚNICAMENTE abre o cierra el menú desplegable, NUNCA ejecuta la simulación
            self.probar_dropdown_open = !self.probar_dropdown_open;
        }
        probar_resp.on_hover_text("Opciones de prueba y simulación");

        // Menú Desplegable Oficial de "Probar" (Imagen 1 y 2 de referencia)
        if self.probar_dropdown_open {
            let menu_pos = Pos2::new(probar_rect.min.x, probar_rect.max.y + 3.0);
            let menu_w = 145.0;
            let menu_items = [
                (0, "Probar"),
                (1, "Probar aquí"),
                (2, "Ejecutar"),
                (3, "Prueba en equipo"),
                (4, "Servidor y clientes"),
            ];
            let item_h = 24.0;
            let menu_h = (menu_items.len() as f32) * item_h + 8.0;

            egui::Area::new(ui.id().with("area_probar_menu"))
                .fixed_pos(menu_pos)
                .order(egui::Order::Foreground)
                .show(ui.ctx(), |ui| {
                    let (popup_rect, _) = ui.allocate_exact_size(Vec2::new(menu_w, menu_h), Sense::hover());
                    ui.painter().rect_filled(popup_rect, Rounding::same(4.0), Color32::from_rgb(26, 27, 32));
                    ui.painter().rect_stroke(popup_rect, Rounding::same(4.0), Stroke::new(1.0, Color32::from_rgb(48, 50, 58)));

                    let mut item_y = popup_rect.min.y + 4.0;
                    for (idx, text) in menu_items {
                        let it_rect = Rect::from_min_size(Pos2::new(popup_rect.min.x + 4.0, item_y), Vec2::new(menu_w - 8.0, item_h));
                        let it_resp = ui.interact(it_rect, ui.id().with(("probar_item", idx)), Sense::click());
                        let is_sel = self.probar_mode == idx;

                        if it_resp.hovered() || is_sel {
                            ui.painter().rect_filled(it_rect, Rounding::same(3.0), Color32::from_rgb(46, 48, 55));
                        }

                        let text_color = if it_resp.hovered() || is_sel { Color32::WHITE } else { Color32::from_rgb(215, 218, 225) };
                        ui.painter().text(
                            Pos2::new(it_rect.min.x + 8.0, it_rect.center().y),
                            egui::Align2::LEFT_CENTER,
                            text,
                            FontId::proportional(11.5),
                            text_color,
                        );

                        if it_resp.clicked() {
                            // Selecciona la opción y cierra el menú SIN ejecutar la simulación
                            self.probar_mode = idx;
                            self.probar_dropdown_open = false;
                        }

                        item_y += item_h;
                    }
                });
        }
        cur_x += probar_w + 14.0;

        // Botón Play ▶ (Azul vibrante)
        let play_rect = Rect::from_center_size(Pos2::new(cur_x + 7.0, bar_center_y), Vec2::new(18.0, 22.0));
        let play_resp = ui.interact(play_rect, ui.id().with("quick_sim_play"), Sense::click());
        if play_resp.hovered() {
            ui.painter().rect_filled(play_rect, Rounding::same(3.0), Color32::from_rgba_unmultiplied(255, 255, 255, 18));
        }
        let tri_color = if is_playing { Color32::from_rgb(100, 140, 180) } else { Color32::from_rgb(0, 136, 255) };
        let tri_p1 = Pos2::new(play_rect.center().x - 4.5, bar_center_y - 5.5);
        let tri_p2 = Pos2::new(play_rect.center().x + 5.5, bar_center_y);
        let tri_p3 = Pos2::new(play_rect.center().x - 4.5, bar_center_y + 5.5);
        ui.painter().add(egui::Shape::convex_polygon(vec![tri_p1, tri_p2, tri_p3], tri_color, Stroke::NONE));
        if play_resp.clicked() { action = RibbonAction::PlaySimulation; }
        play_resp.on_hover_text("Jugar (F5)");
        cur_x += 24.0;

        // Botón Pause ⏸ ▾
        let pause_rect = Rect::from_center_size(Pos2::new(cur_x + 9.0, bar_center_y), Vec2::new(22.0, 22.0));
        let pause_resp = ui.interact(pause_rect, ui.id().with("quick_sim_pause"), Sense::click());
        if pause_resp.hovered() {
            ui.painter().rect_filled(pause_rect, Rounding::same(3.0), Color32::from_rgba_unmultiplied(255, 255, 255, 18));
        }
        let pause_col = if is_paused { Color32::WHITE } else { Color32::from_rgb(150, 153, 162) };
        let bar1_rect = Rect::from_center_size(Pos2::new(pause_rect.center().x - 5.0, bar_center_y), Vec2::new(2.5, 10.0));
        let bar2_rect = Rect::from_center_size(Pos2::new(pause_rect.center().x - 0.5, bar_center_y), Vec2::new(2.5, 10.0));
        ui.painter().rect_filled(bar1_rect, 0.5, pause_col);
        ui.painter().rect_filled(bar2_rect, 0.5, pause_col);
        let pause_arrow_x = pause_rect.center().x + 6.0;
        let pau_p1 = Pos2::new(pause_arrow_x - 2.5, bar_center_y - 1.5);
        let pau_p2 = Pos2::new(pause_arrow_x + 2.5, bar_center_y - 1.5);
        let pau_p3 = Pos2::new(pause_arrow_x, bar_center_y + 2.0);
        ui.painter().add(egui::Shape::convex_polygon(vec![pau_p1, pau_p2, pau_p3], pause_col, Stroke::NONE));
        if pause_resp.clicked() { action = RibbonAction::PauseSimulation; }
        pause_resp.on_hover_text("Pausar");
        cur_x += 26.0;

        // Botón Stop ⏹ (Rojo apagado cuando no está en juego, idéntico a Imagen 2)
        let stop_rect = Rect::from_center_size(Pos2::new(cur_x + 7.0, bar_center_y), Vec2::new(18.0, 22.0));
        let stop_resp = ui.interact(stop_rect, ui.id().with("quick_sim_stop"), Sense::click());
        if stop_resp.hovered() {
            ui.painter().rect_filled(stop_rect, Rounding::same(3.0), Color32::from_rgba_unmultiplied(255, 255, 255, 18));
        }
        let stop_col = if is_playing { Color32::from_rgb(230, 50, 50) } else { Color32::from_rgb(115, 42, 42) };
        let stop_sq_rect = Rect::from_center_size(stop_rect.center(), Vec2::new(9.5, 9.5));
        ui.painter().rect_filled(stop_sq_rect, Rounding::same(1.5), stop_col);
        if stop_resp.clicked() { action = RibbonAction::StopSimulation; }
        stop_resp.on_hover_text("Detener (Shift+F5)");
        cur_x += 24.0;

        // Botón Emulador de Dispositivo 🖥
        let dev_rect = Rect::from_center_size(Pos2::new(cur_x + 7.0, bar_center_y), Vec2::new(18.0, 22.0));
        let dev_resp = ui.interact(dev_rect, ui.id().with("quick_device_emu"), Sense::click());
        if dev_resp.hovered() {
            ui.painter().rect_filled(dev_rect, Rounding::same(3.0), Color32::from_rgba_unmultiplied(255, 255, 255, 18));
        }
        let dev_col = if dev_resp.hovered() { Color32::WHITE } else { Color32::from_rgb(150, 153, 162) };
        let mon_rect = Rect::from_center_size(Pos2::new(dev_rect.center().x, bar_center_y - 1.5), Vec2::new(12.0, 9.0));
        ui.painter().rect_stroke(mon_rect, Rounding::same(1.5), Stroke::new(1.0, dev_col));
        ui.painter().line_segment(
            [Pos2::new(dev_rect.center().x, mon_rect.max.y), Pos2::new(dev_rect.center().x, mon_rect.max.y + 2.5)],
            Stroke::new(1.0, dev_col),
        );
        ui.painter().line_segment(
            [Pos2::new(dev_rect.center().x - 3.5, mon_rect.max.y + 2.5), Pos2::new(dev_rect.center().x + 3.5, mon_rect.max.y + 2.5)],
            Stroke::new(1.0, dev_col),
        );
        dev_resp.on_hover_text("Emulación de dispositivo");

        // --- PESTAÑAS MODERNAS TIPO PÍLDORA (PERFECTAMENTE CENTRADAS Y SIN ERRORES) ---
        let all_tabs = [
            ("Inicio", RibbonTab::Home),
            ("Avatar", RibbonTab::Avatar),
            ("UI", RibbonTab::UI),
            ("Script", RibbonTab::Script),
            ("Modelo", RibbonTab::Model),
            ("Plugins", RibbonTab::Plugins),
            ("Sin título", RibbonTab::Untitled),
        ];

        let tab_gap = 14.0;
        let font_id = FontId::proportional(11.8);

        // Precalcular anchos uniformes (cada pestaña tiene su ancho de píldora completo y constante)
        let mut tab_widths: Vec<f32> = Vec::with_capacity(all_tabs.len());
        let mut total_tabs_w = 0.0;

        for (label, _) in &all_tabs {
            let galley = ui.painter().layout_no_wrap(label.to_string(), font_id.clone(), Color32::WHITE);
            let text_w = galley.size().x;
            // Ancho constante idéntico activa o inactiva: evita compactación de texto y saltos al hacer clic
            let w = text_w + 18.0;
            tab_widths.push(w);
            total_tabs_w += w + tab_gap;
        }
        total_tabs_w += 18.0; // Botón '+'

        let center_x = tabs_rect.center().x;
        let mut tab_x = (center_x - total_tabs_w * 0.5).max(cur_x + 20.0);

        for (i, (label, tab)) in all_tabs.iter().enumerate() {
            let w = tab_widths[i];
            let is_active = self.current_tab == *tab;

            let rect = Rect::from_center_size(Pos2::new(tab_x + w * 0.5, bar_center_y), Vec2::new(w, 24.0));
            let resp = ui.interact(rect, ui.id().with(("mod_tab", label)), Sense::click());

            if is_active {
                // Píldora redondeada activa oficial
                ui.painter().rect_filled(rect, Rounding::same(4.0), Color32::from_rgb(52, 54, 60));
                ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, *label, font_id.clone(), Color32::WHITE);
            } else {
                if resp.hovered() {
                    ui.painter().rect_filled(rect, Rounding::same(4.0), Color32::from_rgba_unmultiplied(255, 255, 255, 12));
                }
                let col = if resp.hovered() { Color32::WHITE } else { Color32::from_rgb(160, 163, 172) };
                ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, *label, font_id.clone(), col);
            }

            if resp.clicked() {
                self.current_tab = *tab;
            }

            tab_x += w + tab_gap;
        }

        // Botón '+' para nueva pestaña
        let add_tab_rect = Rect::from_center_size(Pos2::new(tab_x + 8.0, bar_center_y), Vec2::new(18.0, 20.0));
        let add_tab_resp = ui.interact(add_tab_rect, ui.id().with("add_tab_btn"), Sense::click());
        if add_tab_resp.hovered() {
            ui.painter().rect_filled(add_tab_rect, Rounding::same(3.0), Color32::from_rgba_unmultiplied(255, 255, 255, 18));
        }
        ui.painter().text(
            add_tab_rect.center(),
            egui::Align2::CENTER_CENTER,
            "+",
            FontId::proportional(13.0),
            if add_tab_resp.hovered() { Color32::WHITE } else { Color32::from_rgb(150, 153, 162) },
        );
        add_tab_resp.on_hover_text("Nueva pestaña");

        ui.painter().line_segment(
            [Pos2::new(tabs_rect.min.x, tabs_rect.max.y), Pos2::new(tabs_rect.max.x, tabs_rect.max.y)],
            Stroke::new(1.0, Color32::from_rgb(38, 40, 45)),
        );

        // =====================================================================
        // 3. CUERPO DEL RIBBON (HERRAMIENTAS CON ICONOS REALES)
        // =====================================================================
        let ribbon_body_h = 60.0;
        let (body_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), ribbon_body_h), Sense::hover());
        ui.painter().rect_filled(body_rect, 0.0, STUDIO_TAB_ACTIVE);

        let mut child_ui = ui.child_ui(body_rect, egui::Layout::left_to_right(egui::Align::Center), None);

        match self.current_tab {
            RibbonTab::Home => {
                self.draw_home_ribbon(
                    &mut child_ui,
                    icons,
                    is_playing,
                    is_paused,
                    toolbox_visible,
                    explorer_visible,
                    properties_visible,
                    &mut action,
                );
            }
            RibbonTab::Avatar => {
                self.draw_avatar_ribbon(&mut child_ui, icons, &mut action);
            }
            RibbonTab::UI => {
                self.draw_ui_ribbon(&mut child_ui, icons, &mut action);
            }
            RibbonTab::Script => {
                self.draw_script_ribbon(&mut child_ui, icons, &mut action);
            }
            RibbonTab::Model => {
                self.draw_model_ribbon(&mut child_ui, icons, &mut action);
            }
            RibbonTab::Plugins => {
                self.draw_plugins_ribbon(&mut child_ui, icons);
            }
            RibbonTab::Untitled => {
                self.draw_home_ribbon(
                    &mut child_ui,
                    icons,
                    is_playing,
                    is_paused,
                    toolbox_visible,
                    explorer_visible,
                    properties_visible,
                    &mut action,
                );
            }
        }

        ui.painter().line_segment(
            [Pos2::new(body_rect.min.x, body_rect.max.y), Pos2::new(body_rect.max.x, body_rect.max.y)],
            Stroke::new(1.0, STUDIO_DARK_BORDER),
        );

        action
    }

    // --- HOME RIBBON CON ICONOS OFICIALES Y DISEÑO IDÉNTICO A ROBLOX STUDIO ---
    fn draw_home_ribbon(
        &mut self,
        ui: &mut egui::Ui,
        icons: &StudioIcons,
        is_playing: bool,
        _is_paused: bool,
        toolbox_visible: bool,
        explorer_visible: bool,
        properties_visible: bool,
        action: &mut RibbonAction,
    ) {
        ui.add_space(4.0);

        // 1. Herramientas de Transformación (Seleccionar, Mover, Escalar, Girar, Transformar)
        if draw_modern_ribbon_button(ui, "Seleccionar", self.current_tool == StudioTool::Select, &icons.rib_select, false) {
            self.current_tool = StudioTool::Select;
            *action = RibbonAction::ChangeTool(StudioTool::Select);
        }
        if draw_modern_ribbon_button(ui, "Mover", self.current_tool == StudioTool::Move, &icons.rib_move, false) {
            self.current_tool = StudioTool::Move;
            *action = RibbonAction::ChangeTool(StudioTool::Move);
        }
        if draw_modern_ribbon_button(ui, "Escalar", self.current_tool == StudioTool::Scale, &icons.rib_scale, false) {
            self.current_tool = StudioTool::Scale;
            *action = RibbonAction::ChangeTool(StudioTool::Scale);
        }
        if draw_modern_ribbon_button(ui, "Girar", self.current_tool == StudioTool::Rotate, &icons.rib_rotate, false) {
            self.current_tool = StudioTool::Rotate;
            *action = RibbonAction::ChangeTool(StudioTool::Rotate);
        }
        if draw_modern_ribbon_button(ui, "Transformar", self.current_tool == StudioTool::Transform, &icons.rib_transform, false) {
            self.current_tool = StudioTool::Transform;
            *action = RibbonAction::ChangeTool(StudioTool::Transform);
        }

        // 2. Geométrico + Snap Steppers
        if draw_modern_ribbon_button(ui, "Geométrico", false, &icons.rib_geometric, true) {
            // Dropdown de tipo de pivote geométrico
        }

        ui.vertical(|ui| {
            ui.add_space(8.0);
            draw_snap_row(ui, &mut self.move_snap_enabled, &mut self.move_snap_studs, &icons.rib_move_spin, true);
            ui.add_space(2.0);
            draw_snap_row(ui, &mut self.rotate_snap_enabled, &mut self.rotate_snap_deg, &icons.rib_rotate_spin, false);
        });

        draw_modern_divider(ui);

        // 3. Insertar Objetos (Pieza ▾, Terreno, Personaje, GUI ▾, Script ▾, Importar)
        if draw_modern_ribbon_button(ui, "Pieza", false, &icons.rib_part, true) {
            *action = RibbonAction::InsertPart(PartShape::Block);
        }
        if draw_modern_ribbon_button(ui, "Terreno", false, &icons.rib_terrain, false) {
            *action = RibbonAction::InsertPart(PartShape::Block);
        }
        if draw_modern_ribbon_button(ui, "Personaje", false, &icons.rib_character, false) {
            // Rig builder
        }
        if draw_modern_ribbon_button(ui, "GUI", false, &icons.rib_gui, true) {
            // GUI element insertion
        }
        if draw_modern_ribbon_button(ui, "Script", false, &icons.rib_script, true) {
            // Script insertion
        }
        if draw_modern_ribbon_button(ui, "Importar", false, &icons.rib_import, false) {
            // 3D Mesh / Model Import
        }

        draw_modern_divider(ui);

        // 4. Edición y Estilo (Material ▾, Color ▾, Grupo ▾, Bloquear ▾, Anclar ▾)
        if draw_modern_ribbon_button(ui, "Material", false, &icons.rib_material, true) {
            // Material Picker
        }
        if draw_modern_ribbon_button(ui, "Color", false, &icons.rib_color, true) {
            // Color Picker
        }
        if draw_modern_ribbon_button(ui, "Grupo", false, &icons.rib_group, true) {
            // Group Objects
        }
        if draw_modern_ribbon_button(ui, "Bloquear", false, &icons.rib_lock, true) {
            // Lock Objects
        }
        if draw_modern_ribbon_button(ui, "Anclar", false, &icons.rib_anchor, true) {
            *action = RibbonAction::ToggleAnchor;
        }

        draw_modern_divider(ui);

        // 5. Ventanas y Paneles (Explorador, Propiedades, Caja de..., Recursos)
        if draw_modern_ribbon_button(ui, "Explorador", false, &icons.rib_explorer, false) {
            *action = RibbonAction::ToggleExplorer;
        }
        if draw_modern_ribbon_button(ui, "Propiedades", false, &icons.rib_properties, false) {
            *action = RibbonAction::ToggleProperties;
        }
        if draw_modern_ribbon_button(ui, "Caja de...", false, &icons.rib_toolbox, false) {
            *action = RibbonAction::ToggleToolbox;
        }
        if draw_modern_ribbon_button(ui, "Recursos", false, &icons.rib_assetmgr, false) {
            // Asset Manager
        }
    }

    // --- MODEL RIBBON CON ICONOS OFICIALES REALES (Idéntico a Imagen de Referencia de Modelo) ---
    fn draw_model_ribbon(&mut self, ui: &mut egui::Ui, icons: &StudioIcons, action: &mut RibbonAction) {
        ui.add_space(4.0);

        // 1. Herramientas de Transformación (Seleccionar, Mover, Escalar, Girar, Transformar)
        if draw_modern_ribbon_button(ui, "Seleccionar", self.current_tool == StudioTool::Select, &icons.rib_select, false) {
            self.current_tool = StudioTool::Select;
            *action = RibbonAction::ChangeTool(StudioTool::Select);
        }
        if draw_modern_ribbon_button(ui, "Mover", self.current_tool == StudioTool::Move, &icons.rib_move, false) {
            self.current_tool = StudioTool::Move;
            *action = RibbonAction::ChangeTool(StudioTool::Move);
        }
        if draw_modern_ribbon_button(ui, "Escalar", self.current_tool == StudioTool::Scale, &icons.rib_scale, false) {
            self.current_tool = StudioTool::Scale;
            *action = RibbonAction::ChangeTool(StudioTool::Scale);
        }
        if draw_modern_ribbon_button(ui, "Girar", self.current_tool == StudioTool::Rotate, &icons.rib_rotate, false) {
            self.current_tool = StudioTool::Rotate;
            *action = RibbonAction::ChangeTool(StudioTool::Rotate);
        }
        if draw_modern_ribbon_button(ui, "Transformar", self.current_tool == StudioTool::Transform, &icons.rib_transform, false) {
            self.current_tool = StudioTool::Transform;
            *action = RibbonAction::ChangeTool(StudioTool::Transform);
        }

        // 2. Geométrico + Snap Steppers
        if draw_modern_ribbon_button(ui, "Geométrico", false, &icons.rib_geometric, true) {
            // Dropdown de tipo de pivote geométrico
        }

        ui.vertical(|ui| {
            ui.add_space(8.0);
            draw_snap_row(ui, &mut self.move_snap_enabled, &mut self.move_snap_studs, &icons.rib_move_spin, true);
            ui.add_space(2.0);
            draw_snap_row(ui, &mut self.rotate_snap_enabled, &mut self.rotate_snap_deg, &icons.rib_rotate_spin, false);
        });

        draw_modern_divider(ui);

        // 3. Pivote (Pivote ▾, Restablecer)
        if draw_modern_ribbon_button(ui, "Pivote", false, &icons.rib_edit_pivot, true) {
            // Dropdown de Pivote
        }
        if draw_modern_ribbon_button(ui, "Restablecer", false, &icons.rib_reset_pivot, false) {
            // Restablecer pivote
        }

        draw_modern_divider(ui);

        // 4. Objetos y Efectos (Pieza ▾, Efecto ▾)
        if draw_modern_ribbon_button(ui, "Pieza", false, &icons.rib_part, true) {
            *action = RibbonAction::InsertPart(PartShape::Block);
        }
        if draw_modern_ribbon_button(ui, "Efecto", false, &icons.rib_effect, true) {
            // Dropdown de Efectos (Partículas, Luces, Humo, Fuego)
        }

        draw_modern_divider(ui);

        // 5. Restricciones y Conexiones (Conexión ▾, Restricciones ▾, Soldadura ▾)
        if draw_modern_ribbon_button(ui, "Conexión", false, &icons.rib_attachment, true) {
            // Conexión / Attachment
        }
        if draw_modern_ribbon_button(ui, "Restricciones", false, &icons.rib_constraint, true) {
            // Restricciones / Constraints
        }
        if draw_modern_ribbon_button(ui, "Soldadura", false, &icons.rib_weld, true) {
            // Soldadura / Weld
        }

        draw_modern_divider(ui);

        // 6. Material, Color, Texturas, Agrupación (Material ▾, Color ▾, Texture, Grupo ▾, Bloquear ▾, Anclar ▾)
        if draw_modern_ribbon_button(ui, "Material", false, &icons.rib_material, true) {
            // Material Picker
        }
        if draw_modern_ribbon_button(ui, "Color", false, &icons.rib_color, true) {
            // Color Picker
        }
        if draw_modern_ribbon_button(ui, "Texture", false, &icons.rib_texture, false) {
            // Texture Generator / Tool
        }
        if draw_modern_ribbon_button(ui, "Grupo", false, &icons.rib_group, true) {
            // Group Objects
        }
        if draw_modern_ribbon_button(ui, "Bloquear", false, &icons.rib_lock, true) {
            // Bloquear / Lock Tool
        }
        if draw_modern_ribbon_button(ui, "Anclar", false, &icons.rib_anchor, true) {
            *action = RibbonAction::ToggleAnchor;
        }

        draw_modern_divider(ui);

        // 7. Alineación (Alinear)
        if draw_modern_ribbon_button(ui, "Alinear", false, &icons.rib_align, false) {
            // Align Tool
        }

        draw_modern_divider(ui);

        // 8. Operaciones CSG / Sólidos (Unir, Intersecar, Separar, Sustraer)
        if draw_modern_ribbon_button(ui, "Unir", false, &icons.rib_union, false) {
            // CSG Union
        }
        if draw_modern_ribbon_button(ui, "Intersecar", false, &icons.rib_intersect, false) {
            // CSG Intersect
        }
        if draw_modern_ribbon_button(ui, "Separar", false, &icons.rib_separate, false) {
            // CSG Separate
        }
        if draw_modern_ribbon_button(ui, "Sustraer", false, &icons.rib_negate, false) {
            // CSG Negate
        }
    }

    // --- AVATAR RIBBON CON ICONOS OFICIALES REALES (Idéntico a Imagen 1 de Referencia) ---
    fn draw_avatar_ribbon(&mut self, ui: &mut egui::Ui, icons: &StudioIcons, action: &mut RibbonAction) {
        ui.add_space(4.0);

        // 1. Herramientas de Transformación (Seleccionar, Mover, Escalar, Girar, Transformar)
        if draw_modern_ribbon_button(ui, "Seleccionar", self.current_tool == StudioTool::Select, &icons.rib_select, false) {
            self.current_tool = StudioTool::Select;
            *action = RibbonAction::ChangeTool(StudioTool::Select);
        }
        if draw_modern_ribbon_button(ui, "Mover", self.current_tool == StudioTool::Move, &icons.rib_move, false) {
            self.current_tool = StudioTool::Move;
            *action = RibbonAction::ChangeTool(StudioTool::Move);
        }
        if draw_modern_ribbon_button(ui, "Escalar", self.current_tool == StudioTool::Scale, &icons.rib_scale, false) {
            self.current_tool = StudioTool::Scale;
            *action = RibbonAction::ChangeTool(StudioTool::Scale);
        }
        if draw_modern_ribbon_button(ui, "Girar", self.current_tool == StudioTool::Rotate, &icons.rib_rotate, false) {
            self.current_tool = StudioTool::Rotate;
            *action = RibbonAction::ChangeTool(StudioTool::Rotate);
        }
        if draw_modern_ribbon_button(ui, "Transformar", self.current_tool == StudioTool::Transform, &icons.rib_transform, false) {
            self.current_tool = StudioTool::Transform;
            *action = RibbonAction::ChangeTool(StudioTool::Transform);
        }

        // 2. Geométrico + Snap Steppers
        if draw_modern_ribbon_button(ui, "Geométrico", false, &icons.rib_geometric, true) {
            // Dropdown de tipo de pivote geométrico
        }

        ui.vertical(|ui| {
            ui.add_space(8.0);
            draw_snap_row(ui, &mut self.move_snap_enabled, &mut self.move_snap_studs, &icons.rib_move_spin, true);
            ui.add_space(2.0);
            draw_snap_row(ui, &mut self.rotate_snap_enabled, &mut self.rotate_snap_deg, &icons.rib_rotate_spin, false);
        });

        draw_modern_divider(ui);

        // 3. Creación y Configuración de Avatar
        if draw_modern_ribbon_button(ui, "Configuración", false, &icons.rib_avatar_settings, false) {
            // Configuración de Avatar
        }
        if draw_modern_ribbon_button(ui, "Personaje", false, &icons.rib_character, false) {
            // Rig Builder / Personaje
        }
        if draw_modern_ribbon_button(ui, "Configurar", false, &icons.rib_avatar_setup, false) {
            // Setup de Avatar
        }
        if draw_modern_ribbon_button(ui, "Accesorio", false, &icons.rib_accessory, false) {
            // Ajuste de accesorios
        }
        if draw_modern_ribbon_button(ui, "Animación...", false, &icons.rib_anim_constraint, false) {
            // Animación constraint
        }
        if draw_modern_ribbon_button(ui, "Validación...", false, &icons.rib_ugc_validation, false) {
            // Validación UGC
        }

        draw_modern_divider(ui);

        // 4. Editores de Animación y Clips
        if draw_modern_ribbon_button(ui, "Editor de clips", false, &icons.rib_layered, false) {
            // Editor de clips
        }
        if draw_modern_ribbon_button(ui, "Editor de...", false, &icons.rib_remap_rig, false) {
            // Editor de animaciones
        }
    }

    // --- UI RIBBON CON ICONOS OFICIALES REALES (Idéntico a Imagen 2 de Referencia) ---
    fn draw_ui_ribbon(&mut self, ui: &mut egui::Ui, icons: &StudioIcons, _action: &mut RibbonAction) {
        ui.add_space(4.0);

        // 1. Elementos y Contenedores UI (Todos con flecha desplegable ▾)
        if draw_modern_ribbon_button(ui, "GUI", false, &icons.rib_gui, true) {
            // Dropdown ScreenGui / SurfaceGui / BillboardGui
        }
        if draw_modern_ribbon_button(ui, "Cuadro", false, &icons.rib_ui_frame, true) {
            // Dropdown Frame / ScrollingFrame / CanvasGroup
        }
        if draw_modern_ribbon_button(ui, "Etiqueta", false, &icons.rib_ui_label, true) {
            // Dropdown TextLabel / ImageLabel
        }
        if draw_modern_ribbon_button(ui, "Entrada", false, &icons.rib_ui_entry, true) {
            // Dropdown TextBox / TextButton / ImageButton
        }
        if draw_modern_ribbon_button(ui, "Apariencia", false, &icons.rib_ui_appearance, true) {
            // Dropdown UICorner / UIGradient / UIStroke
        }
        if draw_modern_ribbon_button(ui, "Diseño", false, &icons.rib_ui_layout, true) {
            // Dropdown UIListLayout / UIGridLayout / UITableLayout
        }
        if draw_modern_ribbon_button(ui, "Restricción", false, &icons.rib_ui_constraint, true) {
            // Dropdown UIAspectRatioConstraint / UISizeConstraint
        }

        draw_modern_divider(ui);

        // 2. Editor de Estilos
        if draw_modern_ribbon_button(ui, "Editor de...", false, &icons.rib_ui_style_editor, false) {
            // Style Editor
        }
    }

    // --- SCRIPT RIBBON CON ICONOS OFICIALES REALES (Idéntico a Referencia de Script) ---
    fn draw_script_ribbon(&mut self, ui: &mut egui::Ui, icons: &StudioIcons, action: &mut RibbonAction) {
        ui.add_space(4.0);

        // 1. Navegación (Volver, Avanzar)
        draw_modern_ribbon_button(ui, "Volver", false, &icons.rib_back, false);
        draw_modern_ribbon_button(ui, "Avanzar", false, &icons.rib_fwd, false);

        draw_modern_divider(ui);

        // 2. Herramientas de Script y Código (Script ▾, Formato ▾, Buscar ▾, Ir a la línea)
        draw_modern_ribbon_button(ui, "Script", false, &icons.rib_script, true);
        draw_modern_ribbon_button(ui, "Formato", false, &icons.rib_format_doc, true);
        draw_modern_ribbon_button(ui, "Buscar", false, &icons.rib_find, true);
        draw_modern_ribbon_button(ui, "Ir a la línea", false, &icons.rib_goto_line, false);

        draw_modern_divider(ui);

        // 3. Ventanas / Consola de Código (Comando, Salida)
        if draw_modern_ribbon_button(ui, "Comando", false, &icons.rib_command, false) {
            *action = RibbonAction::ToggleCommandBar;
        }
        if draw_modern_ribbon_button(ui, "Salida", false, &icons.rib_output, false) {
            *action = RibbonAction::ToggleOutput;
        }

        draw_modern_divider(ui);

        // 4. Depuración (Puntos de..., Pila de..., Inspeccionar)
        draw_modern_ribbon_button(ui, "Puntos de...", false, &icons.rib_breakpoints, false);
        draw_modern_ribbon_button(ui, "Pila de...", false, &icons.rib_callstack, false);
        draw_modern_ribbon_button(ui, "Inspeccionar", false, &icons.rib_watch, false);

        draw_modern_divider(ui);

        // 5. Diagnóstico y Rendimiento (Análisis, Actividad de...)
        draw_modern_ribbon_button(ui, "Análisis", false, &icons.rib_analysis, false);
        draw_modern_ribbon_button(ui, "Actividad de...", false, &icons.rib_performance, false);
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

    // --- PLUGINS RIBBON (Vacía por defecto idéntico a Imagen 2) ---
    fn draw_plugins_ribbon(&mut self, ui: &mut egui::Ui, _icons: &StudioIcons) {
        if self.plugins.is_empty() {
            // Vacía por defecto cuando no hay plugins añadidos (Imagen 2 de referencia)
            return;
        }

        ui.add_space(4.0);
        let mut last_sec = 0;
        for (idx, p) in self.plugins.iter().enumerate() {
            if idx > 0 && p.section != last_sec {
                draw_modern_divider(ui);
            }
            last_sec = p.section;
            draw_modern_plugin_button(ui, &p.name, p.icon_color, &p.icon_text);
        }
    }
}

// =========================================================================
// RENDERIZADO DE BOTONES DE PLUGINS MODERNOS DE ROBLOX STUDIO
// =========================================================================

fn draw_modern_plugin_button(
    ui: &mut egui::Ui,
    label: &str,
    icon_bg: Color32,
    icon_text: &str,
) -> bool {
    let font_id = FontId::proportional(10.5);
    let galley = ui.painter().layout_no_wrap(label.to_string(), font_id.clone(), Color32::WHITE);
    let text_w = galley.size().x;

    let btn_w = (text_w + 14.0).max(42.0);
    let btn_h = 52.0;
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(btn_w, btn_h), Sense::click());

    let bg = if resp.is_pointer_button_down_on() {
        Color32::from_rgba_unmultiplied(255, 255, 255, 25)
    } else if resp.hovered() {
        Color32::from_rgba_unmultiplied(255, 255, 255, 12)
    } else {
        Color32::TRANSPARENT
    };
    ui.painter().rect_filled(rect, Rounding::same(4.0), bg);

    // Icono centrado (22x22 px) con fondo distintivo del plugin
    let icon_center = Pos2::new(rect.center().x, rect.min.y + 16.0);
    let icon_rect = Rect::from_center_size(icon_center, Vec2::new(22.0, 22.0));
    ui.painter().rect_filled(icon_rect, Rounding::same(4.0), icon_bg);
    ui.painter().text(
        icon_center,
        egui::Align2::CENTER_CENTER,
        icon_text,
        FontId::proportional(12.0),
        Color32::WHITE,
    );

    // Texto de la herramienta debajo
    let text_pos = Pos2::new(rect.center().x, rect.min.y + 36.0);
    let text_col = if resp.hovered() {
        Color32::WHITE
    } else {
        Color32::from_rgb(204, 207, 216)
    };
    ui.painter().text(text_pos, egui::Align2::CENTER_CENTER, label, font_id, text_col);

    resp.clicked()
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

fn draw_modern_divider(ui: &mut egui::Ui) {
    ui.add_space(2.0);
    let (rect, _) = ui.allocate_exact_size(Vec2::new(1.0, 50.0), Sense::hover());
    let y_center = rect.center().y;
    ui.painter().line_segment(
        [Pos2::new(rect.min.x, y_center - 18.0), Pos2::new(rect.min.x, y_center + 18.0)],
        Stroke::new(1.0, Color32::from_rgb(52, 54, 58)),
    );
    ui.add_space(2.0);
}

fn draw_modern_ribbon_button(
    ui: &mut egui::Ui,
    label: &str,
    is_active: bool,
    icon: &TextureHandle,
    has_dropdown: bool,
) -> bool {
    let font_size = 9.5;
    let galley = ui.painter().layout_no_wrap(label.to_string(), FontId::proportional(font_size), Color32::WHITE);
    let text_w = galley.size().x;
    let btn_w = (text_w + 14.0).max(44.0);
    let btn_h = 52.0;

    let (rect, resp) = ui.allocate_exact_size(Vec2::new(btn_w, btn_h), Sense::click());

    // Background & Border idéntico a Roblox Studio oficial
    if is_active {
        ui.painter().rect_filled(rect, Rounding::same(3.0), Color32::from_rgba_unmultiplied(255, 255, 255, 12));
        ui.painter().rect_stroke(rect, Rounding::same(3.0), Stroke::new(1.0, Color32::from_rgb(65, 68, 76)));
    } else if resp.hovered() {
        ui.painter().rect_filled(rect, Rounding::same(3.0), Color32::from_rgba_unmultiplied(255, 255, 255, 18));
        ui.painter().rect_stroke(rect, Rounding::same(3.0), Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 25)));
    }

    let icon_size = Vec2::new(24.0, 24.0);
    let center_x = rect.center().x;
    let top_y = rect.min.y;

    if has_dropdown {
        // Icon shifted slightly left to make room for arrow on the right
        let icon_center = Pos2::new(center_x - 3.0, top_y + 16.0);
        let icon_rect = Rect::from_center_size(icon_center, icon_size);
        ui.painter().image(icon.id(), icon_rect, Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), Color32::WHITE);

        // Small dropdown arrow next to the icon
        let arrow_x = center_x + 13.0;
        let arrow_y = top_y + 16.0;
        let p1 = Pos2::new(arrow_x - 2.5, arrow_y - 1.5);
        let p2 = Pos2::new(arrow_x + 2.5, arrow_y - 1.5);
        let p3 = Pos2::new(arrow_x, arrow_y + 2.0);
        let arrow_color = if resp.hovered() || is_active { Color32::WHITE } else { Color32::from_rgb(180, 180, 185) };
        ui.painter().add(egui::Shape::convex_polygon(vec![p1, p2, p3], arrow_color, Stroke::NONE));
    } else {
        let icon_center = Pos2::new(center_x, top_y + 16.0);
        let icon_rect = Rect::from_center_size(icon_center, icon_size);
        ui.painter().image(icon.id(), icon_rect, Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), Color32::WHITE);
    }

    // Text label centered below icon
    let text_pos = Pos2::new(center_x, top_y + 39.0);
    let text_color = if is_active || resp.hovered() {
        Color32::WHITE
    } else {
        Color32::from_rgb(205, 205, 210)
    };
    ui.painter().text(text_pos, egui::Align2::CENTER_CENTER, label, FontId::proportional(font_size), text_color);

    resp.on_hover_text(label).clicked()
}

fn draw_snap_row(
    ui: &mut egui::Ui,
    enabled: &mut bool,
    value: &mut f32,
    icon: &TextureHandle,
    is_studs: bool,
) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(4.0, 0.0);

        // 1. Checkbox
        let (cb_rect, cb_resp) = ui.allocate_exact_size(Vec2::new(14.0, 14.0), Sense::click());
        if cb_resp.clicked() {
            *enabled = !*enabled;
        }
        let cb_bg = if *enabled { Color32::from_rgb(38, 40, 44) } else { Color32::from_rgb(28, 28, 32) };
        let cb_stroke = if cb_resp.hovered() {
            Stroke::new(1.0, Color32::from_rgb(0, 162, 255))
        } else {
            Stroke::new(1.0, Color32::from_rgb(70, 72, 78))
        };
        ui.painter().rect(cb_rect, Rounding::same(2.0), cb_bg, cb_stroke);
        if *enabled {
            let c = cb_rect.center();
            let col = Color32::from_rgb(220, 220, 225);
            ui.painter().line_segment(
                [Pos2::new(c.x - 3.5, c.y + 0.2), Pos2::new(c.x - 1.0, c.y + 2.8)],
                Stroke::new(1.6, col),
            );
            ui.painter().line_segment(
                [Pos2::new(c.x - 1.0, c.y + 2.8), Pos2::new(c.x + 3.8, c.y - 2.8)],
                Stroke::new(1.6, col),
            );
        }

        // 2. Spinbox Field
        let spin_size = Vec2::new(64.0, 18.0);
        let (spin_rect, _spin_resp) = ui.allocate_exact_size(spin_size, Sense::hover());
        let spin_bg = Color32::from_rgb(24, 25, 28);
        let spin_stroke = Stroke::new(1.0, Color32::from_rgb(48, 50, 56));
        ui.painter().rect(spin_rect, Rounding::same(2.0), spin_bg, spin_stroke);

        // Icon inside spinbox (left side)
        let icon_rect = Rect::from_min_size(
            Pos2::new(spin_rect.min.x + 3.0, spin_rect.center().y - 6.0),
            Vec2::new(12.0, 12.0),
        );
        ui.painter().image(icon.id(), icon_rect, Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), Color32::WHITE);

        // Value text
        let text_str = if is_studs {
            format!("{:.0} st...", *value)
        } else {
            format!("{:.0}°", *value)
        };
        let text_pos = Pos2::new(spin_rect.min.x + 18.0, spin_rect.center().y);
        ui.painter().text(
            text_pos,
            egui::Align2::LEFT_CENTER,
            text_str,
            FontId::proportional(9.5),
            if *enabled { Color32::from_rgb(220, 220, 220) } else { Color32::from_rgb(120, 120, 125) },
        );

        // Up/down stepper buttons on the right side of spinbox
        let steppers_rect = Rect::from_min_max(
            Pos2::new(spin_rect.max.x - 12.0, spin_rect.min.y),
            Pos2::new(spin_rect.max.x, spin_rect.max.y),
        );
        let up_rect = Rect::from_min_max(steppers_rect.min, Pos2::new(steppers_rect.max.x, steppers_rect.center().y));
        let down_rect = Rect::from_min_max(Pos2::new(steppers_rect.min.x, steppers_rect.center().y), steppers_rect.max);

        let up_resp = ui.interact(up_rect, ui.id().with((is_studs, "up")), Sense::click());
        let down_resp = ui.interact(down_rect, ui.id().with((is_studs, "down")), Sense::click());

        if up_resp.clicked() {
            if is_studs {
                *value = (*value + 1.0).clamp(0.2, 50.0);
            } else {
                *value = (*value + 15.0).clamp(5.0, 180.0);
            }
        }
        if down_resp.clicked() {
            if is_studs {
                *value = (*value - 1.0).max(1.0);
            } else {
                *value = (*value - 15.0).max(5.0);
            }
        }

        let up_c = up_rect.center();
        let up_col = if up_resp.hovered() { Color32::WHITE } else { Color32::from_rgb(170, 172, 178) };
        let p1 = Pos2::new(up_c.x, up_c.y - 2.2);
        let p2 = Pos2::new(up_c.x - 2.8, up_c.y + 1.2);
        let p3 = Pos2::new(up_c.x + 2.8, up_c.y + 1.2);
        ui.painter().add(egui::Shape::convex_polygon(vec![p1, p2, p3], up_col, Stroke::NONE));

        let down_c = down_rect.center();
        let down_col = if down_resp.hovered() { Color32::WHITE } else { Color32::from_rgb(170, 172, 178) };
        let d1 = Pos2::new(down_c.x - 2.8, down_c.y - 1.2);
        let d2 = Pos2::new(down_c.x + 2.8, down_c.y - 1.2);
        let d3 = Pos2::new(down_c.x, down_c.y + 2.2);
        ui.painter().add(egui::Shape::convex_polygon(vec![d1, d2, d3], down_col, Stroke::NONE));
    });
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
