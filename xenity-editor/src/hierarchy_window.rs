use eframe::egui::{self, Color32, CursorIcon, FontId, Pos2, Rect, Rounding, Sense, Stroke, Vec2};
use crate::icons;
use crate::unity_style::*;

#[derive(Clone)]
pub struct GameObjectNode {
    pub id: usize,
    pub name: String,
    pub is_active: bool,
    pub is_prefab: bool,
    pub is_visible: bool,
    pub is_pickable: bool,
    pub is_expanded: bool,
    pub children: Vec<GameObjectNode>,
}

pub struct SceneHierarchyState {
    pub scene_name: String,
    pub search_query: String,
    pub selected_id: Option<usize>,
    pub is_locked: bool,
    pub items: Vec<GameObjectNode>,
}

impl Default for SceneHierarchyState {
    fn default() -> Self {
        Self {
            scene_name: "MainScene".into(),
            search_query: String::new(),
            selected_id: Some(1), // Main Camera seleccionado
            is_locked: false,
            items: vec![
                GameObjectNode {
                    id: 1,
                    name: "Main Camera".into(),
                    is_active: true,
                    is_prefab: false,
                    is_visible: true,
                    is_pickable: true,
                    is_expanded: false,
                    children: vec![],
                },
                GameObjectNode {
                    id: 2,
                    name: "Directional Light".into(),
                    is_active: true,
                    is_prefab: false,
                    is_visible: true,
                    is_pickable: true,
                    is_expanded: false,
                    children: vec![],
                },
                GameObjectNode {
                    id: 3,
                    name: "Player".into(),
                    is_active: true,
                    is_prefab: true, // Prefab oficial de Unity
                    is_visible: true,
                    is_pickable: true,
                    is_expanded: true,
                    children: vec![
                        GameObjectNode {
                            id: 31,
                            name: "Weapon_Holder".into(),
                            is_active: true,
                            is_prefab: false,
                            is_visible: true,
                            is_pickable: true,
                            is_expanded: true,
                            children: vec![
                                GameObjectNode {
                                    id: 311,
                                    name: "Pistol_Mesh".into(),
                                    is_active: true,
                                    is_prefab: false,
                                    is_visible: true,
                                    is_pickable: true,
                                    is_expanded: false,
                                    children: vec![],
                                },
                            ],
                        },
                        GameObjectNode {
                            id: 32,
                            name: "Body_Collider".into(),
                            is_active: false, // Inactivo (texto atenuado #707070)
                            is_prefab: false,
                            is_visible: true,
                            is_pickable: true,
                            is_expanded: false,
                            children: vec![],
                        },
                    ],
                },
                GameObjectNode {
                    id: 4,
                    name: "Environment".into(),
                    is_active: true,
                    is_prefab: false,
                    is_visible: true,
                    is_pickable: true,
                    is_expanded: true,
                    children: vec![
                        GameObjectNode {
                            id: 41,
                            name: "Ground_Plane".into(),
                            is_active: true,
                            is_prefab: false,
                            is_visible: true,
                            is_pickable: true,
                            is_expanded: false,
                            children: vec![],
                        },
                        GameObjectNode {
                            id: 42,
                            name: "Tree_01".into(),
                            is_active: true,
                            is_prefab: true,
                            is_visible: true,
                            is_pickable: true,
                            is_expanded: false,
                            children: vec![],
                        },
                        GameObjectNode {
                            id: 43,
                            name: "Tree_02".into(),
                            is_active: true,
                            is_prefab: true,
                            is_visible: false, // Oculto en escena
                            is_pickable: true,
                            is_expanded: false,
                            children: vec![],
                        },
                    ],
                },
                GameObjectNode {
                    id: 5,
                    name: "Skybox".into(),
                    is_active: true,
                    is_prefab: false,
                    is_visible: true,
                    is_pickable: false, // Bloqueado contra picking
                    is_expanded: false,
                    children: vec![],
                },
            ],
        }
    }
}

pub const PREFAB_BLUE: Color32 = Color32::from_rgb(77, 158, 224);  // #4D9EE0 (Color oficial Prefab de Unity)
pub const DISABLED_GRAY: Color32 = Color32::from_rgb(112, 112, 112); // #707070 (Color objeto inactivo)

impl SceneHierarchyState {
    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<String> {
        let mut clicked_object_name = None;

        // 1. PESTAÑA SUPERIOR DE UNITY [ Hierarchy ]
        let (tab_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 24.0), Sense::hover());
        ui.painter().rect_filled(tab_rect, 0.0, HEADER_BG);

        let active_tab_rect = Rect::from_min_size(tab_rect.min, Vec2::new(90.0, 24.0));
        ui.painter().rect_filled(active_tab_rect, Rounding { nw: 2.0, ne: 2.0, sw: 0.0, se: 0.0 }, PANEL_BG);
        ui.painter().line_segment(
            [Pos2::new(active_tab_rect.min.x, active_tab_rect.min.y), Pos2::new(active_tab_rect.max.x, active_tab_rect.min.y)],
            Stroke::new(2.0, SELECTION_BLUE),
        );
        ui.painter().text(
            active_tab_rect.center(),
            egui::Align2::CENTER_CENTER,
            "Hierarchy",
            FontId::proportional(11.5),
            TEXT_WHITE,
        );

        // Candado y menú contextual en el extremo derecho de la pestaña
        let lock_center = Pos2::new(tab_rect.max.x - 26.0, tab_rect.center().y);
        let lock_rect = Rect::from_center_size(lock_center, Vec2::new(14.0, 14.0));
        let lock_resp = ui.interact(lock_rect, ui.id().with("hier_tab_lock"), Sense::click());
        if lock_resp.clicked() {
            self.is_locked = !self.is_locked;
        }
        let lock_color = if self.is_locked { TEXT_WHITE } else if lock_resp.hovered() { TEXT_NORMAL } else { TEXT_MUTED };
        icons::padlock(ui.painter(), lock_center, self.is_locked, lock_color);

        let menu_center = Pos2::new(tab_rect.max.x - 10.0, tab_rect.center().y);
        icons::kebab_menu(ui.painter(), menu_center, TEXT_MUTED);

        ui.painter().line_segment(
            [Pos2::new(tab_rect.min.x, tab_rect.max.y), Pos2::new(tab_rect.max.x, tab_rect.max.y)],
            Stroke::new(1.0, BORDER_COLOR),
        );

        // 2. SUB-BARRA DE HERRAMIENTAS: Botón [+ ▾] y Barra de Búsqueda redondeada de Unity
        let (toolbar_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 26.0), Sense::hover());
        ui.painter().rect_filled(toolbar_rect, 0.0, PANEL_BG);

        let mut tb_ui = ui.child_ui(toolbar_rect, egui::Layout::left_to_right(egui::Align::Center), None);
        tb_ui.add_space(6.0);

        // Botón vector [+] para crear GameObject (sin caracteres unicode rotos)
        let (create_btn, create_resp) = tb_ui.allocate_exact_size(Vec2::new(22.0, 18.0), Sense::click());
        let create_bg = if create_resp.hovered() { BTN_HOVER } else { BTN_BG };
        tb_ui.painter().rect_filled(create_btn, 3.0, create_bg);
        tb_ui.painter().rect_stroke(create_btn, 3.0, Stroke::new(1.0, Color32::from_rgb(60, 60, 60)));
        let c = create_btn.center();
        let stroke = Stroke::new(1.4, if create_resp.hovered() { TEXT_WHITE } else { TEXT_NORMAL });
        tb_ui.painter().line_segment([Pos2::new(c.x - 3.5, c.y), Pos2::new(c.x + 3.5, c.y)], stroke);
        tb_ui.painter().line_segment([Pos2::new(c.x, c.y - 3.5), Pos2::new(c.x, c.y + 3.5)], stroke);

        tb_ui.add_space(4.0);

        // Search Bar redondeada con icono de lupa (exacto estilo Unity)
        let search_avail_w = tb_ui.available_width() - 8.0;
        let search_rect = Rect::from_min_size(tb_ui.cursor().min, Vec2::new(search_avail_w, 18.0));
        tb_ui.advance_cursor_after_rect(search_rect);

        tb_ui.painter().rect_filled(search_rect, Rounding::same(9.0), INPUT_BG);
        tb_ui.painter().rect_stroke(search_rect, Rounding::same(9.0), Stroke::new(1.0, Color32::from_rgb(50, 50, 50)));

        let mag_center = Pos2::new(search_rect.min.x + 10.0, search_rect.center().y);
        icons::magnifying_glass(tb_ui.painter(), mag_center, 4.0, TEXT_MUTED);

        let edit_rect = Rect::from_min_max(
            Pos2::new(search_rect.min.x + 22.0, search_rect.min.y),
            Pos2::new(search_rect.max.x - 16.0, search_rect.max.y),
        );
        let mut edit_ui = tb_ui.child_ui(edit_rect, egui::Layout::left_to_right(egui::Align::Center), None);
        edit_ui.add(
            egui::TextEdit::singleline(&mut self.search_query)
                .hint_text("Search...")
                .frame(false)
                .desired_width(edit_rect.width()),
        );

        if !self.search_query.is_empty() {
            let clear_center = Pos2::new(search_rect.max.x - 10.0, search_rect.center().y);
            let clear_rect = Rect::from_center_size(clear_center, Vec2::new(12.0, 12.0));
            let clear_resp = tb_ui.interact(clear_rect, tb_ui.id().with("clear_hier_search"), Sense::click());
            let clear_col = if clear_resp.hovered() { TEXT_WHITE } else { TEXT_MUTED };
            tb_ui.painter().text(clear_center, egui::Align2::CENTER_CENTER, "✕", FontId::proportional(10.0), clear_col);
            if clear_resp.clicked() {
                self.search_query.clear();
            }
        }

        ui.painter().line_segment(
            [Pos2::new(toolbar_rect.min.x, toolbar_rect.max.y), Pos2::new(toolbar_rect.max.x, toolbar_rect.max.y)],
            Stroke::new(1.0, BORDER_COLOR),
        );

        // 3. ÁREA DEL ÁRBOL (FONDO OSCURO UNITY #282828)
        let tree_rect = Rect::from_min_size(ui.cursor().min, ui.available_size());
        ui.painter().rect_filled(tree_rect, 0.0, DARK_BG);

        egui::ScrollArea::vertical()
            .id_salt("scene_hierarchy_scroll")
            .show(ui, |ui| {
                // A. CABECERA DE LA ESCENA (Scene Header Row oficial de Unity)
                let (scene_rect, scene_resp) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 22.0), Sense::click());
                let scene_bg = if scene_resp.hovered() { Color32::from_rgb(52, 52, 52) } else { Color32::from_rgb(46, 46, 46) };
                ui.painter().rect_filled(scene_rect, 0.0, scene_bg);

                let foldout_pos = Pos2::new(scene_rect.min.x + 8.0, scene_rect.center().y);
                icons::foldout_arrow(ui.painter(), foldout_pos, true, Color32::from_gray(190));

                let clapper_pos = Pos2::new(scene_rect.min.x + 20.0, scene_rect.center().y);
                icons::scene_clapper(ui.painter(), clapper_pos, 10.0, Color32::from_rgb(85, 170, 245));

                ui.painter().text(
                    Pos2::new(scene_rect.min.x + 32.0, scene_rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    &self.scene_name,
                    FontId::proportional(12.0),
                    TEXT_WHITE,
                );

                let menu_pos = Pos2::new(scene_rect.max.x - 12.0, scene_rect.center().y);
                icons::kebab_menu(ui.painter(), menu_pos, TEXT_MUTED);

                ui.painter().line_segment(
                    [Pos2::new(scene_rect.min.x, scene_rect.max.y), Pos2::new(scene_rect.max.x, scene_rect.max.y)],
                    Stroke::new(1.0, BORDER_COLOR),
                );

                // B. FILAS DE GAMEOBJECTS (Limpias, elegantes, sin cubos en objetos normales)
                let mut items = self.items.clone();
                for item in &mut items {
                    Self::draw_node(
                        ui,
                        item,
                        0,
                        &mut self.selected_id,
                        &self.search_query,
                        &mut clicked_object_name,
                    );
                }
                self.items = items;
            });

        clicked_object_name
    }

    fn draw_node(
        ui: &mut egui::Ui,
        node: &mut GameObjectNode,
        depth: usize,
        selected_id: &mut Option<usize>,
        search_filter: &str,
        clicked_name: &mut Option<String>,
    ) {
        let is_selected = *selected_id == Some(node.id);
        let has_children = !node.children.is_empty();
        let avail_w = ui.available_width();

        let matches_filter = search_filter.is_empty()
            || node.name.to_lowercase().contains(&search_filter.to_lowercase());

        let (row_rect, row_resp) = ui.allocate_exact_size(Vec2::new(avail_w, 19.0), Sense::click());

        // Highlight de selección o hover en toda la fila
        if is_selected {
            ui.painter().rect_filled(row_rect, 0.0, SELECTION_BLUE);
        } else if row_resp.hovered() {
            ui.painter().rect_filled(row_rect, 0.0, HOVER_GRAY);
        }

        let indent_x = row_rect.min.x + (depth as f32 * 14.0) + 8.0;

        // Flecha Foldout ▶ / ▼ (solo si tiene hijos)
        if has_children {
            let arrow_pos = Pos2::new(indent_x + 4.0, row_rect.center().y);
            let arrow_rect = Rect::from_center_size(arrow_pos, Vec2::new(14.0, 14.0));
            let arrow_resp = ui.interact(arrow_rect, ui.id().with(node.id).with("arrow"), Sense::click());
            if arrow_resp.clicked() {
                node.is_expanded = !node.is_expanded;
            }
            icons::foldout_arrow(ui.painter(), arrow_pos, node.is_expanded, Color32::from_gray(180));
        }

        // Si es un PREFAB, mostramos el glifo 3D cian oficial de Unity
        let text_start_x = if node.is_prefab {
            let glyph_pos = Pos2::new(indent_x + 18.0, row_rect.center().y);
            icons::prefab_glyph(ui.painter(), glyph_pos, 9.0, PREFAB_BLUE);
            indent_x + 28.0
        } else {
            // Objeto normal de Unity: ¡SIN ICONO! Tipografía limpia y despejada
            indent_x + 18.0
        };

        // Color del texto oficial de Unity
        let text_color = if !node.is_active {
            DISABLED_GRAY
        } else if is_selected {
            TEXT_WHITE
        } else if node.is_prefab {
            PREFAB_BLUE
        } else {
            TEXT_NORMAL
        };

        ui.painter().text(
            Pos2::new(text_start_x, row_rect.center().y),
            egui::Align2::LEFT_CENTER,
            &node.name,
            FontId::proportional(12.0),
            text_color,
        );

        // Botón derecho para abrir el Prefab en Isolation Mode (si es prefab)
        if node.is_prefab && (row_resp.hovered() || is_selected) {
            let arrow_pos = Pos2::new(row_rect.max.x - 42.0, row_rect.center().y);
            ui.painter().text(arrow_pos, egui::Align2::CENTER_CENTER, ">", FontId::proportional(11.0), TEXT_MUTED);
        }

        // ICONOS DERECHOS: Visibilidad 👁 y Candado 🔒 (Vectoriales limpios)
        let eye_pos = Pos2::new(row_rect.max.x - 26.0, row_rect.center().y);
        let eye_rect = Rect::from_center_size(eye_pos, Vec2::new(14.0, 14.0));
        let eye_resp = ui.interact(eye_rect, ui.id().with(node.id).with("eye"), Sense::click());
        if eye_resp.clicked() {
            node.is_visible = !node.is_visible;
        }

        if row_resp.hovered() || !node.is_visible {
            let eye_color = if node.is_visible {
                if eye_resp.hovered() { TEXT_WHITE } else { TEXT_MUTED }
            } else {
                Color32::from_rgb(225, 90, 90)
            };
            icons::eye_icon(ui.painter(), eye_pos, node.is_visible, eye_color);
        }

        let pick_pos = Pos2::new(row_rect.max.x - 10.0, row_rect.center().y);
        let pick_rect = Rect::from_center_size(pick_pos, Vec2::new(14.0, 14.0));
        let pick_resp = ui.interact(pick_rect, ui.id().with(node.id).with("pick"), Sense::click());
        if pick_resp.clicked() {
            node.is_pickable = !node.is_pickable;
        }

        if row_resp.hovered() || !node.is_pickable {
            let pick_color = if node.is_pickable {
                if pick_resp.hovered() { TEXT_WHITE } else { TEXT_MUTED }
            } else {
                Color32::from_rgb(240, 175, 45)
            };
            icons::padlock(ui.painter(), pick_pos, !node.is_pickable, pick_color);
        }

        if row_resp.clicked() {
            *selected_id = Some(node.id);
            *clicked_name = Some(node.name.clone());
        }

        // Hijos
        if has_children && node.is_expanded {
            for child in &mut node.children {
                Self::draw_node(ui, child, depth + 1, selected_id, search_filter, clicked_name);
            }
        }
    }
}
