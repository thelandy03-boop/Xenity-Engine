use eframe::egui::{self, Color32, CursorIcon, FontId, Id, Painter, Pos2, Rect, Rounding, Sense, Stroke, Vec2};
use crate::icons;
use crate::unity_style::*;

#[derive(Clone, PartialEq, Eq)]
pub enum AssetType {
    Folder,
    Model,
    Material,
    Texture,
    Script,
    Scene,
    Audio,
    Prefab,
}

#[derive(Clone)]
pub struct AssetItem {
    pub name: String,
    pub asset_type: AssetType,
    pub preview_color: Color32,
    pub folder: String,
}

pub struct ProjectBrowserState {
    pub left_column_width: f32,
    pub is_dragging_splitter: bool,
    pub selected_folder: String,
    pub selected_asset: Option<String>,
    pub search_query: String,
    pub zoom: f32,
    pub assets: Vec<AssetItem>,
    pub expanded_folders: std::collections::HashSet<String>,
}

impl Default for ProjectBrowserState {
    fn default() -> Self {
        let mut expanded = std::collections::HashSet::new();
        expanded.insert("Assets".into());
        expanded.insert("MegaCity".into());
        expanded.insert("Decals".into());
        expanded.insert("Textures".into());

        let assets = vec![
            AssetItem { name: "Materials".into(), asset_type: AssetType::Folder, preview_color: Color32::from_rgb(235, 235, 235), folder: "Quick Access".into() },
            AssetItem { name: "Presets".into(), asset_type: AssetType::Folder, preview_color: Color32::from_rgb(235, 235, 235), folder: "Quick Access".into() },
            AssetItem { name: "TestNinja".into(), asset_type: AssetType::Folder, preview_color: Color32::from_rgb(235, 235, 235), folder: "Quick Access".into() },
            AssetItem { name: "Building_01".into(), asset_type: AssetType::Model, preview_color: Color32::from_rgb(160, 170, 185), folder: "Quick Access".into() },
            AssetItem { name: "Building_Roof".into(), asset_type: AssetType::Model, preview_color: Color32::from_rgb(170, 180, 195), folder: "Quick Access".into() },
            AssetItem { name: "Floor_Light".into(), asset_type: AssetType::Prefab, preview_color: Color32::from_rgb(80, 200, 245), folder: "Quick Access".into() },
            AssetItem { name: "Ground_Concrete".into(), asset_type: AssetType::Material, preview_color: Color32::from_rgb(120, 125, 130), folder: "Quick Access".into() },
            AssetItem { name: "Sky_Cyan".into(), asset_type: AssetType::Texture, preview_color: Color32::from_rgb(0, 215, 255), folder: "Quick Access".into() },
            AssetItem { name: "Normal_PBR".into(), asset_type: AssetType::Texture, preview_color: Color32::from_rgb(125, 125, 255), folder: "Quick Access".into() },
            AssetItem { name: "Lighting_Settings".into(), asset_type: AssetType::Prefab, preview_color: Color32::from_rgb(245, 185, 40), folder: "Quick Access".into() },
            AssetItem { name: "Xenity_Pipeline".into(), asset_type: AssetType::Prefab, preview_color: Color32::from_rgb(70, 165, 245), folder: "Quick Access".into() },
            AssetItem { name: "Ninja_Die_Clip".into(), asset_type: AssetType::Model, preview_color: Color32::from_rgb(215, 80, 80), folder: "Quick Access".into() },
            AssetItem { name: "Ninja_Idle_Anim".into(), asset_type: AssetType::Prefab, preview_color: Color32::from_rgb(80, 215, 225), folder: "Quick Access".into() },
            AssetItem { name: "Ninja_Torch".into(), asset_type: AssetType::Prefab, preview_color: Color32::from_rgb(75, 220, 180), folder: "Quick Access".into() },
            AssetItem { name: "PostProcessing".into(), asset_type: AssetType::Prefab, preview_color: Color32::from_rgb(240, 75, 75), folder: "Quick Access".into() },
            AssetItem { name: "MainScene".into(), asset_type: AssetType::Scene, preview_color: Color32::from_rgb(65, 135, 215), folder: "Quick Access".into() },
            AssetItem { name: "PlayerController.cpp".into(), asset_type: AssetType::Script, preview_color: Color32::from_rgb(85, 175, 240), folder: "Quick Access".into() },
            AssetItem { name: "CameraController.cpp".into(), asset_type: AssetType::Script, preview_color: Color32::from_rgb(85, 175, 240), folder: "Quick Access".into() },
            AssetItem { name: "BGM_Theme.wav".into(), asset_type: AssetType::Audio, preview_color: Color32::from_rgb(255, 120, 120), folder: "Quick Access".into() },
        ];

        Self {
            left_column_width: 220.0,
            is_dragging_splitter: false,
            selected_folder: "Quick Access".into(),
            selected_asset: Some("PlayerController.cpp".into()),
            search_query: String::new(),
            zoom: 68.0,
            assets,
            expanded_folders: expanded,
        }
    }
}

impl ProjectBrowserState {
    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<String> {
        let mut clicked_log = None;

        // 1. SUB-TOOLBAR DE UNITY (Height: 24px, #383838)
        let (toolbar_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 24.0), Sense::hover());
        ui.painter().rect_filled(toolbar_rect, 0.0, HEADER_BG);
        ui.painter().line_segment(
            [Pos2::new(toolbar_rect.min.x, toolbar_rect.max.y), Pos2::new(toolbar_rect.max.x, toolbar_rect.max.y)],
            Stroke::new(1.0, BORDER_COLOR),
        );

        let mut tb_ui = ui.child_ui(toolbar_rect, egui::Layout::left_to_right(egui::Align::Center), None);
        tb_ui.add_space(6.0);

        // Botón vector [+] para Create Asset
        let (create_btn, create_resp) = tb_ui.allocate_exact_size(Vec2::new(22.0, 18.0), Sense::click());
        let create_bg = if create_resp.hovered() { BTN_HOVER } else { BTN_BG };
        tb_ui.painter().rect_filled(create_btn, 3.0, create_bg);
        tb_ui.painter().rect_stroke(create_btn, 3.0, Stroke::new(1.0, Color32::from_rgb(60, 60, 60)));
        let c = create_btn.center();
        let stroke = Stroke::new(1.4, if create_resp.hovered() { TEXT_WHITE } else { TEXT_NORMAL });
        tb_ui.painter().line_segment([Pos2::new(c.x - 3.5, c.y), Pos2::new(c.x + 3.5, c.y)], stroke);
        tb_ui.painter().line_segment([Pos2::new(c.x, c.y - 3.5), Pos2::new(c.x, c.y + 3.5)], stroke);
        if create_resp.clicked() {
            clicked_log = Some("[Project] Open Create Asset Menu".into());
        }

        tb_ui.add_space(4.0);
        tb_ui.separator();
        tb_ui.add_space(4.0);

        // Breadcrumbs: Assets > Quick Access
        let breadcrumb_text = format!("Assets  >  {}", self.selected_folder);
        let bread_resp = tb_ui.label(egui::RichText::new(&breadcrumb_text).color(TEXT_NORMAL).size(12.0));
        if bread_resp.hovered() {
            tb_ui.output_mut(|o| o.cursor_icon = CursorIcon::PointingHand);
        }

        // Búsqueda en extremo derecho
        tb_ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |tb_ui| {
            tb_ui.add_space(8.0);
            let (star_rect, star_resp) = tb_ui.allocate_exact_size(Vec2::new(18.0, 18.0), Sense::click());
            icons::star(tb_ui.painter(), star_rect.center(), 5.0, if star_resp.hovered() { TEXT_WHITE } else { TEXT_MUTED });

            tb_ui.add_space(4.0);
            let search_edit = egui::TextEdit::singleline(&mut self.search_query)
                .hint_text("Search...")
                .desired_width(150.0);
            tb_ui.add(search_edit);

            tb_ui.add_space(2.0);
            let (mag_rect, _) = tb_ui.allocate_exact_size(Vec2::new(14.0, 14.0), Sense::hover());
            icons::magnifying_glass(tb_ui.painter(), mag_rect.center(), 4.5, TEXT_MUTED);
        });

        // 2. CALCULAR ESPACIOS PARA LAS DOS COLUMNAS Y EL FOOTER
        let total_avail_h = ui.available_height();
        let footer_h = 24.0;
        let content_h = (total_avail_h - footer_h).max(60.0);
        let total_w = ui.available_width();

        // Manejo del Splitter interactivo (Drag)
        let splitter_w = 4.0;
        let left_w = self.left_column_width.clamp(120.0, (total_w - 180.0).max(120.0));
        self.left_column_width = left_w;

        let content_rect = Rect::from_min_size(ui.cursor().min, Vec2::new(total_w, content_h));
        ui.advance_cursor_after_rect(content_rect);

        let left_rect = Rect::from_min_size(content_rect.min, Vec2::new(left_w, content_h));
        let splitter_rect = Rect::from_min_size(Pos2::new(left_rect.max.x, content_rect.min.y), Vec2::new(splitter_w, content_h));
        let right_rect = Rect::from_min_size(Pos2::new(splitter_rect.max.x, content_rect.min.y), Vec2::new(total_w - left_w - splitter_w, content_h));

        // Interaction del Splitter
        let splitter_id = ui.make_persistent_id("project_splitter");
        let splitter_resp = ui.interact(splitter_rect, splitter_id, Sense::click_and_drag());
        if splitter_resp.hovered() || splitter_resp.dragged() {
            ui.output_mut(|o| o.cursor_icon = CursorIcon::ResizeHorizontal);
        }
        if splitter_resp.dragged() {
            let delta_x = ui.input(|i| i.pointer.delta().x);
            self.left_column_width = (self.left_column_width + delta_x).clamp(120.0, total_w - 180.0);
        }

        // Fondo del splitter
        let splitter_bg = if splitter_resp.dragged() {
            SELECTION_BLUE
        } else if splitter_resp.hovered() {
            HOVER_GRAY
        } else {
            BORDER_COLOR
        };
        ui.painter().rect_filled(splitter_rect, 0.0, splitter_bg);

        // --- COLUMNA 1: ÁRBOL DE CARPETAS Y FAVORITOS (Fondo #282828) ---
        let mut left_ui = ui.child_ui(left_rect, egui::Layout::top_down(egui::Align::Min), None);
        left_ui.painter().rect_filled(left_rect, 0.0, DARK_BG);

        egui::ScrollArea::vertical()
            .id_salt("project_tree_scroll")
            .show(&mut left_ui, |tree_ui| {
                tree_ui.add_space(4.0);

                // Sección FAVORITOS
                self.draw_tree_section_header(tree_ui, "Favorites", true);
                let favs = [
                    ("Quick Access", true),
                    ("All Materials", false),
                    ("All Models", false),
                    ("All Prefabs", false),
                ];
                for (fav, is_folder) in favs {
                    let is_sel = self.selected_folder == fav;
                    if self.draw_tree_row(tree_ui, fav, 1, is_sel, is_folder) {
                        self.selected_folder = fav.into();
                        clicked_log = Some(format!("[Project] Selected Favorite: {}", fav));
                    }
                }

                tree_ui.add_space(8.0);
                self.draw_tree_section_header(tree_ui, "Assets", false);

                // Carpetas anidadas reales
                let folder_nodes = [
                    ("AnimationRiggingWorkshop", 1, false, false),
                    ("MegaCity", 1, true, true),
                    ("Building", 2, false, false),
                    ("Decals", 2, true, true),
                    ("Materials", 3, false, false),
                    ("Textures", 3, true, true),
                    ("Decal_Dirt_A", 4, false, false),
                    ("Decal_Dirt_B", 4, false, false),
                    ("Decal_Dirt_Leak_A", 4, false, false),
                    ("Decal_Rust_A", 4, false, false),
                ];

                for (name, indent, has_children, is_open) in folder_nodes {
                    let is_sel = self.selected_folder == name;
                    if self.draw_tree_folder_row(tree_ui, name, indent, is_sel, has_children, is_open) {
                        self.selected_folder = name.into();
                        clicked_log = Some(format!("[Project] Selected Folder: {}", name));
                    }
                }
            });

        // --- COLUMNA 2: GRID DE ASSETS (Fondo #383838) ---
        let mut right_ui = ui.child_ui(right_rect, egui::Layout::top_down(egui::Align::Min), None);
        right_ui.painter().rect_filled(right_rect, 0.0, PANEL_BG);

        egui::ScrollArea::vertical()
            .id_salt("project_grid_scroll")
            .auto_shrink([false, false])
            .show(&mut right_ui, |grid_ui| {
                grid_ui.add_space(8.0);

                let query = self.search_query.to_lowercase();
                let filtered: Vec<AssetItem> = self.assets
                    .iter()
                    .filter(|a| query.is_empty() || a.name.to_lowercase().contains(&query))
                    .cloned()
                    .collect();

                if self.zoom < 48.0 {
                    // MODO LISTA COMPACTA DE UNITY (cuando el slider está al mínimo)
                    for asset in &filtered {
                        let is_sel = self.selected_asset.as_deref() == Some(&asset.name);
                        let (row_rect, resp) = grid_ui.allocate_exact_size(Vec2::new(grid_ui.available_width(), 20.0), Sense::click());
                        if is_sel {
                            grid_ui.painter().rect_filled(row_rect, 2.0, SELECTION_BLUE);
                        } else if resp.hovered() {
                            grid_ui.painter().rect_filled(row_rect, 2.0, HOVER_GRAY);
                        }

                        let icon_rect = Rect::from_min_size(Pos2::new(row_rect.min.x + 8.0, row_rect.min.y + 2.0), Vec2::new(16.0, 16.0));
                        draw_asset_icon(grid_ui.painter(), icon_rect, &asset.asset_type, asset.preview_color);

                        grid_ui.painter().text(
                            Pos2::new(row_rect.min.x + 30.0, row_rect.center().y),
                            egui::Align2::LEFT_CENTER,
                            &asset.name,
                            FontId::proportional(11.5),
                            if is_sel { TEXT_WHITE } else { TEXT_NORMAL },
                        );

                        if resp.clicked() {
                            self.selected_asset = Some(asset.name.clone());
                            clicked_log = Some(format!("[Project] Asset: {}", asset.name));
                        }
                    }
                } else {
                    // MODO GRID 2D DE CARDS DE UNITY (con preview y thumbnail)
                    let card_size = self.zoom;
                    let card_w = card_size + 16.0;
                    let card_h = card_size + 28.0;

                    grid_ui.horizontal_wrapped(|wrap_ui| {
                        wrap_ui.spacing_mut().item_spacing = Vec2::new(6.0, 8.0);

                        for asset in &filtered {
                            let is_sel = self.selected_asset.as_deref() == Some(&asset.name);
                            let (card_rect, resp) = wrap_ui.allocate_exact_size(Vec2::new(card_w, card_h), Sense::click());

                            let bg_color = if is_sel {
                                SELECTION_BLUE
                            } else if resp.hovered() {
                                HOVER_GRAY
                            } else {
                                Color32::TRANSPARENT
                            };

                            wrap_ui.painter().rect_filled(card_rect, 3.0, bg_color);

                            // Recuadro interior del Thumbnail
                            let thumb_rect = Rect::from_center_size(
                                Pos2::new(card_rect.center().x, card_rect.min.y + card_size * 0.5 + 4.0),
                                Vec2::new(card_size, card_size),
                            );

                            // Fondo del thumbnail (oscuro como Unity)
                            let thumb_fill = match asset.asset_type {
                                AssetType::Folder => Color32::from_rgb(46, 46, 46),
                                AssetType::Material => Color32::from_rgb(26, 26, 26),
                                AssetType::Texture => asset.preview_color,
                                AssetType::Script => Color32::from_rgb(38, 38, 38),
                                _ => Color32::from_rgb(34, 34, 34),
                            };

                            wrap_ui.painter().rect_filled(thumb_rect, 2.0, thumb_fill);
                            wrap_ui.painter().rect_stroke(thumb_rect, 2.0, Stroke::new(1.0, Color32::from_rgb(50, 50, 50)));

                            // Renderizar icono detallado dentro del thumbnail
                            draw_asset_icon(wrap_ui.painter(), thumb_rect, &asset.asset_type, asset.preview_color);

                            // Texto de nombre truncado debajo
                            let display_name = if asset.name.len() > 11 {
                                format!("{}...", &asset.name[..9])
                            } else {
                                asset.name.clone()
                            };

                            wrap_ui.painter().text(
                                Pos2::new(card_rect.center().x, thumb_rect.max.y + 11.0),
                                egui::Align2::CENTER_CENTER,
                                display_name,
                                FontId::proportional(11.0),
                                if is_sel { TEXT_WHITE } else { TEXT_NORMAL },
                            );

                            let resp = resp.on_hover_text(&asset.name);

                            if resp.clicked() {
                                self.selected_asset = Some(asset.name.clone());
                                clicked_log = Some(format!("[Project] Asset: {}", asset.name));
                            }
                            if resp.double_clicked() && asset.asset_type == AssetType::Folder {
                                self.selected_folder = asset.name.clone();
                                clicked_log = Some(format!("[Project] Entered Folder: {}", asset.name));
                            }
                        }
                    });
                }
            });

        // --- 3. BARRA INFERIOR / FOOTER DE UNITY (Height: 24px, #2A2A2A) ---
        let footer_rect = Rect::from_min_size(Pos2::new(content_rect.min.x, content_rect.max.y), Vec2::new(total_w, footer_h));
        ui.painter().rect_filled(footer_rect, 0.0, HEADER_BG);
        ui.painter().line_segment(
            [Pos2::new(footer_rect.min.x, footer_rect.min.y), Pos2::new(footer_rect.max.x, footer_rect.min.y)],
            Stroke::new(1.0, BORDER_COLOR),
        );

        let mut foot_ui = ui.child_ui(footer_rect, egui::Layout::left_to_right(egui::Align::Center), None);
        foot_ui.add_space(8.0);

        let count_str = format!("{} items", self.assets.len());
        foot_ui.label(egui::RichText::new(count_str).color(TEXT_MUTED).size(11.0));

        // Slider de Zoom Unity en la esquina inferior derecha (Fijo y limpio, nunca choca)
        foot_ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |foot_ui| {
            foot_ui.add_space(12.0);

            // Icono Grid (derecha)
            let (grid_icon_rect, _) = foot_ui.allocate_exact_size(Vec2::new(14.0, 14.0), Sense::hover());
            icons::grid_icon(foot_ui.painter(), grid_icon_rect, TEXT_MUTED);

            foot_ui.add_space(4.0);

            // Slider numérico de zoom Unity
            let slider = egui::Slider::new(&mut self.zoom, 36.0..=110.0)
                .show_value(false);
            foot_ui.add_sized(Vec2::new(65.0, 14.0), slider);

            foot_ui.add_space(4.0);

            // Icono Lista (izquierda)
            let (list_icon_rect, _) = foot_ui.allocate_exact_size(Vec2::new(14.0, 14.0), Sense::hover());
            icons::list_icon(foot_ui.painter(), list_icon_rect, TEXT_MUTED);
        });

        clicked_log
    }

    fn draw_tree_section_header(&self, ui: &mut egui::Ui, title: &str, is_fav: bool) {
        let (rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 18.0), Sense::hover());
        let arrow_pos = Pos2::new(rect.min.x + 8.0, rect.center().y);
        icons::foldout_arrow(ui.painter(), arrow_pos, true, TEXT_MUTED);

        if is_fav {
            icons::star(ui.painter(), Pos2::new(rect.min.x + 19.0, rect.center().y), 4.5, Color32::from_rgb(240, 195, 60));
        }

        let text_x = if is_fav { rect.min.x + 28.0 } else { rect.min.x + 19.0 };
        ui.painter().text(
            Pos2::new(text_x, rect.center().y),
            egui::Align2::LEFT_CENTER,
            title,
            FontId::proportional(11.0),
            TEXT_MUTED,
        );
    }

    fn draw_tree_row(&self, ui: &mut egui::Ui, name: &str, indent: usize, is_sel: bool, is_folder: bool) -> bool {
        let h = 18.0;
        let (rect, resp) = ui.allocate_exact_size(Vec2::new(ui.available_width(), h), Sense::click());

        if is_sel {
            ui.painter().rect_filled(rect, 0.0, SELECTION_BLUE);
        } else if resp.hovered() {
            ui.painter().rect_filled(rect, 0.0, HOVER_GRAY);
        }

        let x_start = rect.min.x + (indent as f32 * 14.0) + 6.0;

        if is_folder {
            let f_rect = Rect::from_min_size(Pos2::new(x_start, rect.min.y + 2.0), Vec2::new(14.0, 13.0));
            icons::solid_folder(ui.painter(), f_rect, Color32::from_rgb(220, 220, 220));
        } else {
            let m_pos = Pos2::new(x_start + 6.0, rect.center().y);
            icons::magnifying_glass(ui.painter(), m_pos, 4.0, TEXT_MUTED);
        }

        ui.painter().text(
            Pos2::new(x_start + 20.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            name,
            FontId::proportional(11.5),
            if is_sel { TEXT_WHITE } else { TEXT_NORMAL },
        );

        resp.clicked()
    }

    fn draw_tree_folder_row(&self, ui: &mut egui::Ui, name: &str, indent: usize, is_sel: bool, has_children: bool, is_open: bool) -> bool {
        let h = 18.0;
        let (rect, resp) = ui.allocate_exact_size(Vec2::new(ui.available_width(), h), Sense::click());

        if is_sel {
            ui.painter().rect_filled(rect, 0.0, SELECTION_BLUE);
        } else if resp.hovered() {
            ui.painter().rect_filled(rect, 0.0, HOVER_GRAY);
        }

        let x_start = rect.min.x + (indent as f32 * 14.0);

        if has_children {
            let arrow_pos = Pos2::new(x_start + 6.0, rect.center().y);
            icons::foldout_arrow(ui.painter(), arrow_pos, is_open, TEXT_MUTED);
        }

        let f_rect = Rect::from_min_size(Pos2::new(x_start + 16.0, rect.min.y + 2.5), Vec2::new(14.0, 13.0));
        icons::solid_folder(ui.painter(), f_rect, Color32::from_rgb(220, 220, 220));

        ui.painter().text(
            Pos2::new(x_start + 34.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            name,
            FontId::proportional(11.5),
            if is_sel { TEXT_WHITE } else { TEXT_NORMAL },
        );

        resp.clicked()
    }
}

fn draw_asset_icon(painter: &Painter, rect: Rect, asset_type: &AssetType, color: Color32) {
    let size = rect.width();
    let center = rect.center();

    match asset_type {
        AssetType::Folder => {
            let f_w = size * 0.58;
            let f_h = size * 0.46;
            let f_rect = Rect::from_center_size(center, Vec2::new(f_w, f_h));
            icons::solid_folder(painter, f_rect, Color32::from_rgb(240, 240, 240));
        }
        AssetType::Material => {
            let r = size * 0.34;
            icons::material_ball(painter, center, r, color);
        }
        AssetType::Texture => {
            // Textura con borde nítido
        }
        AssetType::Script => {
            let doc_w = size * 0.42;
            let doc_h = size * 0.54;
            let doc_rect = Rect::from_center_size(center, Vec2::new(doc_w, doc_h));
            icons::script_file(painter, doc_rect);
        }
        AssetType::Model => {
            let s = size * 0.42;
            let model_rect = Rect::from_center_size(center, Vec2::new(s, s));
            icons::shaded_model(painter, model_rect, color);
        }
        AssetType::Prefab => {
            let c_rect = Rect::from_center_size(center, Vec2::new(size * 0.46, size * 0.46));
            icons::cube(painter, c_rect, Color32::from_rgb(85, 185, 255));
        }
        AssetType::Scene => {
            let s_rect = Rect::from_center_size(center, Vec2::new(size * 0.44, size * 0.44));
            icons::scene_icon(painter, s_rect);
        }
        AssetType::Audio => {
            icons::audio_icon(painter, center, size * 0.28, color);
        }
    }
}
