use eframe::egui::{self, Color32, CursorIcon, FontId, Pos2, Rect, Rounding, Sense, Stroke, TextureHandle, Vec2};
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
            move_snap_studs: 21.0,
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
        // 2. BARRA SUPERIOR MODERNA (PROBAR + CONTROLES + PESTAÑAS PÍLDORA)
        // =====================================================================
        let tabs_bar_h = 32.0;
        let (tabs_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), tabs_bar_h), Sense::hover());
        ui.painter().rect_filled(tabs_rect, 0.0, Color32::from_rgb(22, 23, 27));

        let mut cur_x = tabs_rect.min.x + 8.0;
        let bar_center_y = tabs_rect.center().y;

        // --- LADO IZQUIERDO: Botón "Probar ▾" ---
        let probar_w = 64.0;
        let probar_rect = Rect::from_center_size(Pos2::new(cur_x + probar_w * 0.5, bar_center_y), Vec2::new(probar_w, 22.0));
        let probar_resp = ui.interact(probar_rect, ui.id().with("btn_probar_dropdown"), Sense::click());
        let probar_bg = if probar_resp.hovered() { Color32::from_rgb(48, 50, 56) } else { Color32::from_rgb(34, 35, 40) };
        ui.painter().rect(probar_rect, Rounding::same(4.0), probar_bg, Stroke::new(1.0, Color32::from_rgb(52, 54, 60)));
        ui.painter().text(
            Pos2::new(probar_rect.min.x + 8.0, bar_center_y),
            egui::Align2::LEFT_CENTER,
            "Probar",
            FontId::proportional(11.0),
            Color32::from_rgb(220, 220, 225),
        );
        let p_arrow_x = probar_rect.max.x - 10.0;
        let p1 = Pos2::new(p_arrow_x - 2.5, bar_center_y - 1.5);
        let p2 = Pos2::new(p_arrow_x + 2.5, bar_center_y - 1.5);
        let p3 = Pos2::new(p_arrow_x, bar_center_y + 2.0);
        ui.painter().add(egui::Shape::convex_polygon(vec![p1, p2, p3], Color32::from_rgb(180, 180, 185), Stroke::NONE));
        if probar_resp.clicked() {
            if !is_playing { action = RibbonAction::PlaySimulation; }
        }
        probar_resp.on_hover_text("Opciones de simulación");
        cur_x += probar_w + 10.0;

        // Botón Play ▶ (Azul)
        let play_rect = Rect::from_center_size(Pos2::new(cur_x + 8.0, bar_center_y), Vec2::new(18.0, 20.0));
        let play_resp = ui.interact(play_rect, ui.id().with("quick_sim_play"), Sense::click());
        if play_resp.hovered() {
            ui.painter().rect_filled(play_rect, Rounding::same(3.0), STUDIO_HOVER);
        }
        let tri_color = if is_playing { Color32::from_rgb(100, 140, 180) } else { Color32::from_rgb(0, 136, 255) };
        let tri_p1 = Pos2::new(play_rect.center().x - 4.0, bar_center_y - 5.0);
        let tri_p2 = Pos2::new(play_rect.center().x + 5.0, bar_center_y);
        let tri_p3 = Pos2::new(play_rect.center().x - 4.0, bar_center_y + 5.0);
        ui.painter().add(egui::Shape::convex_polygon(vec![tri_p1, tri_p2, tri_p3], tri_color, Stroke::NONE));
        if play_resp.clicked() { action = RibbonAction::PlaySimulation; }
        play_resp.on_hover_text("Jugar (F5)");
        cur_x += 20.0;

        // Botón Pause ⏸ ▾
        let pause_rect = Rect::from_center_size(Pos2::new(cur_x + 12.0, bar_center_y), Vec2::new(24.0, 20.0));
        let pause_resp = ui.interact(pause_rect, ui.id().with("quick_sim_pause"), Sense::click());
        if pause_resp.hovered() {
            ui.painter().rect_filled(pause_rect, Rounding::same(3.0), STUDIO_HOVER);
        }
        let pause_col = if is_paused { Color32::WHITE } else { Color32::from_rgb(150, 153, 160) };
        let bar1_rect = Rect::from_center_size(Pos2::new(pause_rect.center().x - 5.0, bar_center_y), Vec2::new(2.5, 9.0));
        let bar2_rect = Rect::from_center_size(Pos2::new(pause_rect.center().x - 0.5, bar_center_y), Vec2::new(2.5, 9.0));
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

        // Botón Stop ⏹ (Rojo)
        let stop_rect = Rect::from_center_size(Pos2::new(cur_x + 8.0, bar_center_y), Vec2::new(18.0, 20.0));
        let stop_resp = ui.interact(stop_rect, ui.id().with("quick_sim_stop"), Sense::click());
        if stop_resp.hovered() {
            ui.painter().rect_filled(stop_rect, Rounding::same(3.0), STUDIO_HOVER);
        }
        let stop_col = if is_playing { Color32::from_rgb(230, 60, 60) } else { Color32::from_rgb(110, 50, 50) };
        let stop_sq_rect = Rect::from_center_size(stop_rect.center(), Vec2::new(9.0, 9.0));
        ui.painter().rect_filled(stop_sq_rect, Rounding::same(1.5), stop_col);
        if stop_resp.clicked() { action = RibbonAction::StopSimulation; }
        stop_resp.on_hover_text("Detener (Shift+F5)");
        cur_x += 20.0;

        // Botón Emulador de Dispositivo 🖥
        let dev_rect = Rect::from_center_size(Pos2::new(cur_x + 8.0, bar_center_y), Vec2::new(18.0, 20.0));
        let dev_resp = ui.interact(dev_rect, ui.id().with("quick_device_emu"), Sense::click());
        if dev_resp.hovered() {
            ui.painter().rect_filled(dev_rect, Rounding::same(3.0), STUDIO_HOVER);
        }
        let dev_col = if dev_resp.hovered() { Color32::WHITE } else { Color32::from_rgb(140, 143, 150) };
        let mon_rect = Rect::from_center_size(Pos2::new(dev_rect.center().x, bar_center_y - 1.5), Vec2::new(11.0, 8.0));
        ui.painter().rect_stroke(mon_rect, Rounding::same(1.0), Stroke::new(1.0, dev_col));
        ui.painter().line_segment(
            [Pos2::new(dev_rect.center().x, mon_rect.max.y), Pos2::new(dev_rect.center().x, mon_rect.max.y + 2.5)],
            Stroke::new(1.0, dev_col),
        );
        ui.painter().line_segment(
            [Pos2::new(dev_rect.center().x - 3.0, mon_rect.max.y + 2.5), Pos2::new(dev_rect.center().x + 3.0, mon_rect.max.y + 2.5)],
            Stroke::new(1.0, dev_col),
        );
        dev_resp.on_hover_text("Emulación de dispositivo");

        // --- PESTAÑAS MODERNAS TIPO PÍLDORA (CENTRADAS / DERECHA) ---
        let modern_tabs = [
            ("Inicio", RibbonTab::Home),
            ("Avatar", RibbonTab::Avatar),
            ("UI", RibbonTab::View),
            ("Script", RibbonTab::Plugins),
            ("Modelo", RibbonTab::Model),
            ("Plugins", RibbonTab::Plugins),
            ("Sin título", RibbonTab::Home),
        ];

        let mut tab_x = (tabs_rect.min.x + 360.0).max(cur_x + 30.0);

        for (label, tab) in modern_tabs {
            let is_active = self.current_tab == tab && label == "Inicio";
            let font_id = FontId::proportional(11.5);
            let galley = ui.painter().layout_no_wrap(label.to_string(), font_id.clone(), Color32::WHITE);
            let text_w = galley.size().x;
            let tab_w = text_w + 18.0;
            let tab_h = 22.0;

            let rect = Rect::from_center_size(Pos2::new(tab_x + tab_w * 0.5, bar_center_y), Vec2::new(tab_w, tab_h));
            let resp = ui.interact(rect, ui.id().with(("mod_tab", label)), Sense::click());

            if is_active {
                ui.painter().rect_filled(rect, Rounding::same(5.0), Color32::from_rgb(52, 54, 60));
                ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, label, font_id, Color32::WHITE);
            } else {
                if resp.hovered() {
                    ui.painter().rect_filled(rect, Rounding::same(5.0), Color32::from_rgba_unmultiplied(255, 255, 255, 12));
                }
                let col = if resp.hovered() { Color32::WHITE } else { Color32::from_rgb(160, 163, 172) };
                ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, label, font_id, col);
            }

            if resp.clicked() {
                self.current_tab = tab;
            }

            tab_x += tab_w + 6.0;
        }

        // Botón '+' para nueva pestaña
        let add_tab_rect = Rect::from_center_size(Pos2::new(tab_x + 10.0, bar_center_y), Vec2::new(18.0, 20.0));
        let add_tab_resp = ui.interact(add_tab_rect, ui.id().with("add_tab_btn"), Sense::click());
        if add_tab_resp.hovered() {
            ui.painter().rect_filled(add_tab_rect, Rounding::same(3.0), STUDIO_HOVER);
        }
        ui.painter().text(
            add_tab_rect.center(),
            egui::Align2::CENTER_CENTER,
            "+",
            FontId::proportional(13.0),
            if add_tab_resp.hovered() { Color32::WHITE } else { Color32::from_rgb(160, 163, 172) },
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
        if draw_modern_ribbon_button(ui, "Explorador", explorer_visible, &icons.rib_explorer, false) {
            *action = RibbonAction::ToggleExplorer;
        }
        if draw_modern_ribbon_button(ui, "Propiedades", properties_visible, &icons.rib_properties, false) {
            *action = RibbonAction::ToggleProperties;
        }
        if draw_modern_ribbon_button(ui, "Caja de...", toolbox_visible, &icons.rib_toolbox, false) {
            *action = RibbonAction::ToggleToolbox;
        }
        if draw_modern_ribbon_button(ui, "Recursos", false, &icons.rib_assetmgr, false) {
            // Asset Manager
        }
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

    // Background & Border
    if is_active {
        ui.painter().rect_filled(rect, Rounding::same(3.0), Color32::from_rgba_unmultiplied(0, 162, 255, 35));
        ui.painter().rect_stroke(rect, Rounding::same(3.0), Stroke::new(1.0, Color32::from_rgb(0, 162, 255)));
    } else if resp.hovered() {
        ui.painter().rect_filled(rect, Rounding::same(3.0), Color32::from_rgba_unmultiplied(255, 255, 255, 20));
        ui.painter().rect_stroke(rect, Rounding::same(3.0), Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 35)));
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
            ui.painter().text(
                cb_rect.center(),
                egui::Align2::CENTER_CENTER,
                "✓",
                FontId::proportional(10.0),
                Color32::from_rgb(220, 220, 225),
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

        ui.painter().text(up_rect.center(), egui::Align2::CENTER_CENTER, "▴", FontId::proportional(7.0), Color32::from_rgb(160, 160, 165));
        ui.painter().text(down_rect.center(), egui::Align2::CENTER_CENTER, "▾", FontId::proportional(7.0), Color32::from_rgb(160, 160, 165));
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
