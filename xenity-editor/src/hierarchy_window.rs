use eframe::egui::{self, Color32, CursorIcon, FontId, Pos2, Rect, Rounding, Sense, Stroke, Vec2};
use crate::icons;
use crate::unity_style::*;

#[derive(Clone, PartialEq, Eq)]
pub enum GameObjectKind {
    Camera,
    Light,
    Prefab,
    Mesh,
    Group,
    Default,
}

#[derive(Clone)]
pub struct GameObjectNode {
    pub id: usize,
    pub name: String,
    pub kind: GameObjectKind,
    pub is_active: bool,
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
                    kind: GameObjectKind::Camera,
                    is_active: true,
                    is_visible: true,
                    is_pickable: true,
                    is_expanded: false,
                    children: vec![],
                },
                GameObjectNode {
                    id: 2,
                    name: "Directional Light".into(),
                    kind: GameObjectKind::Light,
                    is_active: true,
                    is_visible: true,
                    is_pickable: true,
                    is_expanded: false,
                    children: vec![],
                },
                GameObjectNode {
                    id: 3,
                    name: "Player".into(),
                    kind: GameObjectKind::Prefab,
                    is_active: true,
                    is_visible: true,
                    is_pickable: true,
                    is_expanded: true,
                    children: vec![
                        GameObjectNode {
                            id: 31,
                            name: "Weapon_Holder".into(),
                            kind: GameObjectKind::Group,
                            is_active: true,
                            is_visible: true,
                            is_pickable: true,
                            is_expanded: true,
                            children: vec![
                                GameObjectNode {
                                    id: 311,
                                    name: "Pistol_Mesh".into(),
                                    kind: GameObjectKind::Mesh,
                                    is_active: true,
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
                            kind: GameObjectKind::Mesh,
                            is_active: false, // Inactivo (#707070)
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
                    kind: GameObjectKind::Group,
                    is_active: true,
                    is_visible: true,
                    is_pickable: true,
                    is_expanded: true,
                    children: vec![
                        GameObjectNode {
                            id: 41,
                            name: "Ground_Plane".into(),
                            kind: GameObjectKind::Mesh,
                            is_active: true,
                            is_visible: true,
                            is_pickable: true,
                            is_expanded: false,
                            children: vec![],
                        },
                        GameObjectNode {
                            id: 42,
                            name: "Tree_01".into(),
                            kind: GameObjectKind::Prefab,
                            is_active: true,
                            is_visible: true,
                            is_pickable: true,
                            is_expanded: false,
                            children: vec![],
                        },
                        GameObjectNode {
                            id: 43,
                            name: "Tree_02".into(),
                            kind: GameObjectKind::Prefab,
                            is_active: true,
                            is_visible: false, // Oculto
                            is_pickable: true,
                            is_expanded: false,
                            children: vec![],
                        },
                    ],
                },
                GameObjectNode {
                    id: 5,
                    name: "Skybox".into(),
                    kind: GameObjectKind::Default,
                    is_active: true,
                    is_visible: true,
                    is_pickable: false, // Bloqueado
                    is_expanded: false,
                    children: vec![],
                },
            ],
        }
    }
}

pub const PREFAB_BLUE: Color32 = Color32::from_rgb(75, 175, 255);   // #4BAFFF (Azul moderno Prefab)
pub const DISABLED_GRAY: Color32 = Color32::from_rgb(115, 118, 128); // #737680 (Inactivo)

impl SceneHierarchyState {
    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<String> {
        let mut clicked_object_name = None;

        // 1. PESTAÑA SUPERIOR [ Hierarchy ]
        let (tab_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 26.0), Sense::hover());
        ui.painter().rect_filled(tab_rect, 0.0, HEADER_BG);

        let active_tab_rect = Rect::from_min_size(tab_rect.min, Vec2::new(92.0, 26.0));
        ui.painter().rect_filled(active_tab_rect, Rounding { nw: 3.0, ne: 3.0, sw: 0.0, se: 0.0 }, PANEL_BG);
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

        // Candado y menú de ventana a la derecha
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

        // 2. SUB-BARRA DE HERRAMIENTAS: Botón [+] y Search Bar moderna perfectamente centrados
        let toolbar_h = 30.0;
        let (toolbar_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), toolbar_h), Sense::hover());
        ui.painter().rect_filled(toolbar_rect, 0.0, PANEL_BG);

        let elem_h = 20.0;
        let elem_y = toolbar_rect.min.y + (toolbar_h - elem_h) * 0.5;

        // Botón [+] con bordes suaves
        let btn_w = 22.0;
        let create_rect = Rect::from_min_size(Pos2::new(toolbar_rect.min.x + 6.0, elem_y), Vec2::new(btn_w, elem_h));
        let create_resp = ui.interact(create_rect, ui.id().with("hier_create_btn"), Sense::click());
        let create_bg = if create_resp.hovered() { BTN_HOVER } else { Color32::from_rgb(52, 53, 58) };
        ui.painter().rect_filled(create_rect, Rounding::same(3.0), create_bg);
        ui.painter().rect_stroke(create_rect, Rounding::same(3.0), Stroke::new(1.0, Color32::from_rgb(65, 66, 72)));

        let c = create_rect.center();
        let stroke = Stroke::new(1.4, if create_resp.hovered() { TEXT_WHITE } else { TEXT_NORMAL });
        ui.painter().line_segment([Pos2::new(c.x - 3.5, c.y), Pos2::new(c.x + 3.5, c.y)], stroke);
        ui.painter().line_segment([Pos2::new(c.x, c.y - 3.5), Pos2::new(c.x, c.y + 3.5)], stroke);

        // Search Bar redondeada perfectamente alineada ópticamente con la lupa
        let search_x = create_rect.max.x + 6.0;
        let search_w = (toolbar_rect.max.x - search_x - 8.0).max(60.0);
        let search_rect = Rect::from_min_size(Pos2::new(search_x, elem_y), Vec2::new(search_w, elem_h));

        ui.painter().rect_filled(search_rect, Rounding::same(5.0), Color32::from_rgb(32, 33, 36));
        ui.painter().rect_stroke(search_rect, Rounding::same(5.0), Stroke::new(1.0, Color32::from_rgb(52, 54, 60)));

        // Lupa centrada exactamente en el eje vertical
        let mag_center = Pos2::new(search_rect.min.x + 12.0, search_rect.center().y);
        icons::magnifying_glass(ui.painter(), mag_center, 3.8, TEXT_MUTED);

        // Cuadro de texto
        let clear_w = if self.search_query.is_empty() { 0.0 } else { 16.0 };
        let text_rect = Rect::from_min_max(
            Pos2::new(search_rect.min.x + 24.0, search_rect.min.y),
            Pos2::new(search_rect.max.x - 6.0 - clear_w, search_rect.max.y),
        );

        let mut edit_ui = ui.child_ui(text_rect, egui::Layout::left_to_right(egui::Align::Center), None);
        edit_ui.add(
            egui::TextEdit::singleline(&mut self.search_query)
                .hint_text("Search...")
                .frame(false)
                .font(FontId::proportional(11.5))
                .desired_width(text_rect.width()),
        );

        // Botón ✕ para borrar búsqueda
        if !self.search_query.is_empty() {
            let clear_center = Pos2::new(search_rect.max.x - 10.0, search_rect.center().y);
            let clear_rect = Rect::from_center_size(clear_center, Vec2::new(14.0, 14.0));
            let clear_resp = ui.interact(clear_rect, ui.id().with("clear_hier_search"), Sense::click());
            let clear_col = if clear_resp.hovered() { TEXT_WHITE } else { TEXT_MUTED };
            ui.painter().text(clear_center, egui::Align2::CENTER_CENTER, "✕", FontId::proportional(10.0), clear_col);
            if clear_resp.clicked() {
                self.search_query.clear();
            }
        }

        ui.painter().line_segment(
            [Pos2::new(toolbar_rect.min.x, toolbar_rect.max.y), Pos2::new(toolbar_rect.max.x, toolbar_rect.max.y)],
            Stroke::new(1.0, BORDER_COLOR),
        );

        // 3. ÁREA DEL ÁRBOL
        let tree_rect = Rect::from_min_size(ui.cursor().min, ui.available_size());
        ui.painter().rect_filled(tree_rect, 0.0, DARK_BG);

        egui::ScrollArea::vertical()
            .id_salt("scene_hierarchy_scroll")
            .show(ui, |ui| {
                // A. CABECERA DE LA ESCENA (Moderna, elegante)
                let (scene_rect, scene_resp) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 24.0), Sense::click());
                let scene_bg = if scene_resp.hovered() { Color32::from_rgb(50, 52, 58) } else { Color32::from_rgb(42, 43, 48) };
                ui.painter().rect_filled(scene_rect, 0.0, scene_bg);

                let foldout_pos = Pos2::new(scene_rect.min.x + 8.0, scene_rect.center().y);
                icons::foldout_arrow(ui.painter(), foldout_pos, true, Color32::from_gray(190));

                let clapper_pos = Pos2::new(scene_rect.min.x + 20.0, scene_rect.center().y);
                icons::scene_clapper(ui.painter(), clapper_pos, 10.0, Color32::from_rgb(85, 175, 255));

                ui.painter().text(
                    Pos2::new(scene_rect.min.x + 32.0, scene_rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    &self.scene_name,
                    FontId::proportional(12.0),
                    TEXT_WHITE,
                );

                // Contador de objetos
                let count_str = format!("({})", self.items.len());
                ui.painter().text(
                    Pos2::new(scene_rect.min.x + 105.0, scene_rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    count_str,
                    FontId::proportional(10.5),
                    TEXT_MUTED,
                );

                let menu_pos = Pos2::new(scene_rect.max.x - 12.0, scene_rect.center().y);
                icons::kebab_menu(ui.painter(), menu_pos, TEXT_MUTED);

                ui.painter().line_segment(
                    [Pos2::new(scene_rect.min.x, scene_rect.max.y), Pos2::new(scene_rect.max.x, scene_rect.max.y)],
                    Stroke::new(1.0, BORDER_COLOR),
                );

                ui.add_space(2.0);

                // B. FILAS DE GAMEOBJECTS (21px de altura, selección moderna con barra izquierda)
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
        let row_h = 21.0;

        let (row_rect, row_resp) = ui.allocate_exact_size(Vec2::new(avail_w, row_h), Sense::click());

        // Highlight moderno de selección y hover
        if is_selected {
            // Fondo azul elegante con esquinas suaves
            ui.painter().rect_filled(row_rect, Rounding::same(3.0), Color32::from_rgb(36, 75, 122));
            // Tira indicadora brillante en el borde izquierdo
            let indicator_rect = Rect::from_min_size(row_rect.min, Vec2::new(2.5, row_h));
            ui.painter().rect_filled(indicator_rect, Rounding { nw: 3.0, ne: 0.0, sw: 3.0, se: 0.0 }, Color32::from_rgb(75, 165, 255));
        } else if row_resp.hovered() {
            ui.painter().rect_filled(row_rect, Rounding::same(3.0), Color32::from_rgb(50, 52, 58));
        }

        let indent_x = row_rect.min.x + (depth as f32 * 15.0) + 8.0;

        // Línea sutil en forma de L para guiar la jerarquía padre-hijo
        if depth > 0 {
            let guide_x = indent_x - 7.0;
            let guide_y_mid = row_rect.center().y;
            let guide_stroke = Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 30));
            // Línea vertical que baja
            ui.painter().line_segment([Pos2::new(guide_x, row_rect.min.y), Pos2::new(guide_x, guide_y_mid)], guide_stroke);
            // Ramal horizontal hacia el objeto
            ui.painter().line_segment([Pos2::new(guide_x, guide_y_mid), Pos2::new(guide_x + 5.0, guide_y_mid)], guide_stroke);
        }

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

        // Glifos modernos y minimalistas según tipo de GameObject
        let glyph_pos = Pos2::new(indent_x + 16.0, row_rect.center().y);
        match node.kind {
            GameObjectKind::Camera => {
                icons::camera_glyph(ui.painter(), glyph_pos, Color32::from_rgb(130, 185, 235));
            }
            GameObjectKind::Light => {
                icons::light_glyph(ui.painter(), glyph_pos, Color32::from_rgb(245, 205, 105));
            }
            GameObjectKind::Prefab => {
                icons::prefab_glyph(ui.painter(), glyph_pos, 9.0, PREFAB_BLUE);
            }
            GameObjectKind::Mesh => {
                icons::mesh_glyph(ui.painter(), glyph_pos, Color32::from_rgb(160, 168, 180));
            }
            GameObjectKind::Group => {
                icons::group_glyph(ui.painter(), glyph_pos, Color32::from_rgb(150, 155, 168));
            }
            GameObjectKind::Default => {}
        }

        let text_start_x = if node.kind == GameObjectKind::Default {
            indent_x + 14.0
        } else {
            indent_x + 26.0
        };

        // Color del texto
        let text_color = if !node.is_active {
            DISABLED_GRAY
        } else if is_selected {
            TEXT_WHITE
        } else if node.kind == GameObjectKind::Prefab {
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

        // Icono `>` para Prefabs al hacer hover
        if node.kind == GameObjectKind::Prefab && (row_resp.hovered() || is_selected) {
            let arrow_pos = Pos2::new(row_rect.max.x - 44.0, row_rect.center().y);
            ui.painter().text(arrow_pos, egui::Align2::CENTER_CENTER, ">", FontId::proportional(11.0), TEXT_MUTED);
        }

        // ICONOS DERECHOS: Visibilidad 👁 y Candado 🔒
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
                Color32::from_rgb(230, 95, 95)
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
                Color32::from_rgb(245, 185, 55)
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
