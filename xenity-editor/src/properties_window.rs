use eframe::egui::{self, Color32, CursorIcon, FontId, Pos2, Rect, Rounding, Sense, Stroke, Vec2};
use crate::hierarchy_window::{ExplorerItem, RobloxClass, STUDIO_BG, STUDIO_BORDER, STUDIO_HOVER, STUDIO_INPUT_BG, STUDIO_SELECTION, STUDIO_TEXT, STUDIO_TREE_BG};
use crate::studio_textures::StudioIcons;
use crate::icons;

pub const STUDIO_HEADER_BG: Color32 = Color32::from_rgb(42, 42, 45);     // #2A2A2D
pub const STUDIO_ROW_BORDER: Color32 = Color32::from_rgb(38, 38, 40);     // #262628
pub const STUDIO_FIELD_BG: Color32 = Color32::from_rgb(26, 26, 26);       // #1A1A1A
pub const STUDIO_TEXT_MUTED: Color32 = Color32::from_rgb(130, 130, 130);  // #828282
pub const STUDIO_TEXT_DIMMED: Color32 = Color32::from_rgb(105, 105, 110); // #69696E (Propiedades Read-Only de Roblox Studio)

pub struct PropertiesWindowState {
    pub search_query: String,
    pub data_expanded: bool,
    pub tags_expanded: bool,
    pub attributes_expanded: bool,
    pub appearance_expanded: bool,
    pub transform_expanded: bool,
    pub collision_expanded: bool,
    pub assembly_expanded: bool,
    pub specific_expanded: bool,
    pub pos_sub_expanded: bool,
    pub rot_sub_expanded: bool,
    pub size_sub_expanded: bool,
}

impl Default for PropertiesWindowState {
    fn default() -> Self {
        Self {
            search_query: String::new(),
            data_expanded: true,
            tags_expanded: true,
            attributes_expanded: true,
            appearance_expanded: true,
            transform_expanded: true,
            collision_expanded: true,
            assembly_expanded: true,
            specific_expanded: true,
            pos_sub_expanded: false,
            rot_sub_expanded: false,
            size_sub_expanded: false,
        }
    }
}

impl PropertiesWindowState {
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        selected_item: Option<&mut ExplorerItem>,
        icons: &StudioIcons,
    ) -> bool {
        let mut close_requested = false;

        // =====================================================================
        // 1. HEADER DE PANEL DESACOPLABLE (Roblox Studio Header)
        // Ejemplo imagen 2: Propiedades - StarterCharacterScripts "StarterCharacterScripts"
        // =====================================================================
        let header_h = 26.0;
        let (header_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), header_h), Sense::hover());
        ui.painter().rect_filled(header_rect, 0.0, STUDIO_BG);

        let title_text = match &selected_item {
            Some(item) => format!("Propiedades - {} \"{}\"", item.properties.class_name, item.name),
            None => "Propiedades".to_string(),
        };

        let max_text_w = (header_rect.width() - 56.0).max(40.0);
        let font = FontId::proportional(11.5);
        ui.painter().text(
            Pos2::new(header_rect.min.x + 8.0, header_rect.center().y),
            egui::Align2::LEFT_CENTER,
            title_text,
            font,
            Color32::from_rgb(220, 220, 225),
        );

        // Botones derecha: ⤢ y ✕
        let undock_center = Pos2::new(header_rect.max.x - 26.0, header_rect.center().y);
        let undock_rect = Rect::from_center_size(undock_center, Vec2::new(16.0, 16.0));
        let undock_resp = ui.interact(undock_rect, ui.id().with("prop_undock_btn"), Sense::click());
        if undock_resp.hovered() {
            ui.painter().rect_filled(undock_rect, Rounding::same(2.0), STUDIO_HOVER);
        }
        ui.painter().text(
            undock_center,
            egui::Align2::CENTER_CENTER,
            "⤢",
            FontId::proportional(11.0),
            if undock_resp.hovered() { Color32::WHITE } else { STUDIO_TEXT_MUTED },
        );
        undock_resp.on_hover_text("Desacoplar / Flotar ventana");

        let close_center = Pos2::new(header_rect.max.x - 10.0, header_rect.center().y);
        let close_rect = Rect::from_center_size(close_center, Vec2::new(16.0, 16.0));
        let close_resp = ui.interact(close_rect, ui.id().with("prop_close_btn"), Sense::click());
        if close_resp.clicked() {
            close_requested = true;
        }
        if close_resp.hovered() {
            ui.painter().rect_filled(close_rect, Rounding::same(2.0), Color32::from_rgb(180, 40, 40));
        }
        ui.painter().text(
            close_center,
            egui::Align2::CENTER_CENTER,
            "✕",
            FontId::proportional(10.0),
            if close_resp.hovered() { Color32::WHITE } else { STUDIO_TEXT_MUTED },
        );
        close_resp.on_hover_text("Cerrar ventana");

        ui.painter().line_segment(
            [Pos2::new(header_rect.min.x, header_rect.max.y), Pos2::new(header_rect.max.x, header_rect.max.y)],
            Stroke::new(1.0, STUDIO_BORDER),
        );

        // =====================================================================
        // 2. SUB-BARRA DE BÚSQUEDA (Propiedades del filtro: (Control+Mayúsculas+P)  ▾)
        // =====================================================================
        let toolbar_h = 28.0;
        let (toolbar_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), toolbar_h), Sense::hover());
        ui.painter().rect_filled(toolbar_rect, 0.0, STUDIO_BG);

        let search_h = 22.0;
        let search_y = toolbar_rect.min.y + (toolbar_h - search_h) * 0.5;
        let search_rect = Rect::from_min_size(
            Pos2::new(toolbar_rect.min.x + 6.0, search_y),
            Vec2::new(toolbar_rect.width() - 12.0, search_h),
        );

        ui.painter().rect_filled(search_rect, Rounding::same(3.0), STUDIO_INPUT_BG);
        ui.painter().rect_stroke(search_rect, Rounding::same(3.0), Stroke::new(1.0, STUDIO_BORDER));

        // Botón flecha hacia abajo ▾ a la derecha (como en la imagen 2)
        let dropdown_center = Pos2::new(search_rect.max.x - 12.0, search_rect.center().y);
        let dropdown_rect = Rect::from_center_size(dropdown_center, Vec2::new(14.0, 14.0));
        let dropdown_resp = ui.interact(dropdown_rect, ui.id().with("prop_filter_dropdown"), Sense::click());
        if dropdown_resp.hovered() {
            ui.painter().rect_filled(dropdown_rect, Rounding::same(2.0), STUDIO_HOVER);
        }
        ui.painter().text(
            dropdown_center,
            egui::Align2::CENTER_CENTER,
            "▾",
            FontId::proportional(11.0),
            if dropdown_resp.hovered() { Color32::WHITE } else { Color32::from_rgb(160, 160, 165) },
        );
        dropdown_resp.on_hover_text("Opciones de filtro de propiedades");

        let mut right_w = 24.0;
        if !self.search_query.is_empty() {
            right_w += 18.0;
        }

        let text_rect = Rect::from_min_max(
            Pos2::new(search_rect.min.x + 8.0, search_rect.min.y + 2.0),
            Pos2::new(search_rect.max.x - right_w, search_rect.max.y - 2.0),
        );

        let mut edit_ui = ui.child_ui(text_rect, egui::Layout::left_to_right(egui::Align::Center), None);
        edit_ui.spacing_mut().item_spacing = Vec2::ZERO;
        edit_ui.add(
            egui::TextEdit::singleline(&mut self.search_query)
                .hint_text("Propiedades del filtro: (Control+Mayúsculas+P)")
                .frame(false)
                .margin(egui::Margin::symmetric(0.0, 1.0))
                .font(FontId::proportional(11.5))
                .desired_width(text_rect.width()),
        );

        if !self.search_query.is_empty() {
            let clear_center = Pos2::new(search_rect.max.x - 28.0, search_rect.center().y);
            let clear_rect = Rect::from_center_size(clear_center, Vec2::new(14.0, 14.0));
            let clear_resp = ui.interact(clear_rect, ui.id().with("clear_prop_search"), Sense::click());
            let clear_col = if clear_resp.hovered() { Color32::WHITE } else { Color32::from_rgb(150, 150, 150) };
            ui.painter().text(clear_center, egui::Align2::CENTER_CENTER, "✕", FontId::proportional(10.0), clear_col);
            if clear_resp.clicked() {
                self.search_query.clear();
            }
        }

        ui.painter().line_segment(
            [Pos2::new(toolbar_rect.min.x, toolbar_rect.max.y), Pos2::new(toolbar_rect.max.x, toolbar_rect.max.y)],
            Stroke::new(1.0, STUDIO_BORDER),
        );

        // =====================================================================
        // 3. CONTENIDO DE PROPIEDADES (Roblox Studio Grid)
        // =====================================================================
        let body_rect = Rect::from_min_size(ui.cursor().min, ui.available_size());
        ui.painter().rect_filled(body_rect, 0.0, STUDIO_TREE_BG);

        let Some(item) = selected_item else {
            // No selection placeholder
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.label(
                    egui::RichText::new("Selecciona una instancia para ver sus propiedades.")
                        .color(STUDIO_TEXT_MUTED)
                        .size(12.0),
                );
            });
            return close_requested;
        };

        let filter = self.search_query.to_lowercase();
        let col_name_w = 120.0;

        egui::ScrollArea::vertical()
            .id_salt("roblox_properties_scroll")
            .show(ui, |ui| {
                // --- CATEGORÍA: DATOS (Data) ---
                let show_data = filter.is_empty()
                    || "datos".contains(&filter)
                    || "data".contains(&filter)
                    || "archivable".contains(&filter)
                    || "classname".contains(&filter)
                    || "name".contains(&filter)
                    || "parent".contains(&filter)
                    || "uniqueid".contains(&filter);

                if show_data {
                    draw_category_header(ui, "Datos", &mut self.data_expanded, false);
                    if self.data_expanded {
                        if filter.is_empty() || "archivable".contains(&filter) {
                            draw_bool_row(ui, col_name_w, "Archivable", &mut item.properties.archivable);
                        }
                        if filter.is_empty() || "classname".contains(&filter) {
                            draw_read_only_dimmed_row(ui, col_name_w, "ClassName", &item.properties.class_name);
                        }
                        if filter.is_empty() || "name".contains(&filter) {
                            draw_text_edit_row(ui, col_name_w, "Name", &mut item.name);
                        }
                        if filter.is_empty() || "parent".contains(&filter) {
                            draw_read_only_dimmed_row(ui, col_name_w, "Parent", &item.properties.parent_name);
                        }
                        if filter.is_empty() || "uniqueid".contains(&filter) {
                            draw_read_only_dimmed_row(ui, col_name_w, "UniqueId", &item.properties.unique_id);
                        }
                    }
                }

                // --- CATEGORÍA: ETIQUETAS (Tags) con botón [+] ---
                let show_tags = filter.is_empty() || "etiquetas".contains(&filter) || "tags".contains(&filter);
                if show_tags {
                    let plus_clicked = draw_category_header(ui, "Etiquetas", &mut self.tags_expanded, true);
                    if plus_clicked {
                        item.properties.tags.push(format!("Tag_{}", item.properties.tags.len() + 1));
                    }
                    if self.tags_expanded {
                        if item.properties.tags.is_empty() {
                            draw_empty_placeholder_row(ui, "Todavía no se han agregado etiquetas");
                        } else {
                            for tag in &item.properties.tags {
                                draw_read_only_dimmed_row(ui, col_name_w, "Tag", tag);
                            }
                        }
                    }
                }

                // --- CATEGORÍA: ATRIBUTOS (Attributes) con botón [+] ---
                let show_attr = filter.is_empty() || "atributos".contains(&filter) || "attributes".contains(&filter);
                if show_attr {
                    let plus_clicked = draw_category_header(ui, "Atributos", &mut self.attributes_expanded, true);
                    if plus_clicked {
                        item.properties.attributes.push((format!("Attr_{}", item.properties.attributes.len() + 1), "Value".into()));
                    }
                    if self.attributes_expanded {
                        if item.properties.attributes.is_empty() {
                            draw_empty_placeholder_row(ui, "No se ha añadido ningún atributo todavía");
                        } else {
                            for (attr_name, attr_val) in &item.properties.attributes {
                                draw_read_only_dimmed_row(ui, col_name_w, attr_name, attr_val);
                            }
                        }
                    }
                }

                // --- CATEGORÍA: APPEARANCE (Para Part, MeshPart, Model, Terrain) ---
                let is_part_like = matches!(item.class, RobloxClass::Part | RobloxClass::MeshPart | RobloxClass::Terrain);
                if is_part_like {
                    let show_app = filter.is_empty()
                        || "color".contains(&filter)
                        || "material".contains(&filter)
                        || "transparency".contains(&filter)
                        || "reflectance".contains(&filter)
                        || "castshadow".contains(&filter);

                    if show_app {
                        draw_category_header(ui, "Apariencia", &mut self.appearance_expanded, false);
                        if self.appearance_expanded {
                            if filter.is_empty() || "color".contains(&filter) {
                                draw_color_row(ui, col_name_w, "Color", &mut item.properties.color);
                            }
                            if filter.is_empty() || "material".contains(&filter) {
                                draw_material_dropdown_row(ui, col_name_w, "Material", &mut item.properties.material);
                            }
                            if filter.is_empty() || "reflectance".contains(&filter) {
                                draw_float_row(ui, col_name_w, "Reflectance", &mut item.properties.reflectance, 0.0..=1.0, 0.02);
                            }
                            if filter.is_empty() || "transparency".contains(&filter) {
                                draw_float_row(ui, col_name_w, "Transparency", &mut item.properties.transparency, 0.0..=1.0, 0.02);
                            }
                            if filter.is_empty() || "castshadow".contains(&filter) {
                                draw_bool_row(ui, col_name_w, "CastShadow", &mut item.properties.cast_shadow);
                            }
                        }
                    }
                }

                // --- CATEGORÍA: TRANSFORM (Part, MeshPart, Model, Camera) ---
                let has_transform = matches!(item.class, RobloxClass::Part | RobloxClass::MeshPart | RobloxClass::Model | RobloxClass::Camera);
                if has_transform {
                    let show_trans = filter.is_empty()
                        || "position".contains(&filter)
                        || "orientation".contains(&filter)
                        || "size".contains(&filter)
                        || "cframe".contains(&filter);

                    if show_trans {
                        draw_category_header(ui, "Transformación", &mut self.transform_expanded, false);
                        if self.transform_expanded {
                            if filter.is_empty() || "position".contains(&filter) {
                                draw_vector3_expandable_row(
                                    ui,
                                    col_name_w,
                                    "Position",
                                    &mut item.properties.position,
                                    &mut self.pos_sub_expanded,
                                    0.1,
                                );
                            }
                            if filter.is_empty() || "orientation".contains(&filter) {
                                draw_vector3_expandable_row(
                                    ui,
                                    col_name_w,
                                    "Orientation",
                                    &mut item.properties.orientation,
                                    &mut self.rot_sub_expanded,
                                    0.5,
                                );
                            }
                            if filter.is_empty() || "size".contains(&filter) {
                                draw_vector3_expandable_row(
                                    ui,
                                    col_name_w,
                                    "Size",
                                    &mut item.properties.size,
                                    &mut self.size_sub_expanded,
                                    0.1,
                                );
                            }
                        }
                    }
                }

                // --- CATEGORÍA: COLLISION (Part, MeshPart) ---
                if is_part_like {
                    let show_col = filter.is_empty()
                        || "cancollide".contains(&filter)
                        || "cantouch".contains(&filter)
                        || "canquery".contains(&filter)
                        || "collisiongroup".contains(&filter);

                    if show_col {
                        draw_category_header(ui, "Colisión", &mut self.collision_expanded, false);
                        if self.collision_expanded {
                            if filter.is_empty() || "cancollide".contains(&filter) {
                                draw_bool_row(ui, col_name_w, "CanCollide", &mut item.properties.can_collide);
                            }
                            if filter.is_empty() || "cantouch".contains(&filter) {
                                draw_bool_row(ui, col_name_w, "CanTouch", &mut item.properties.can_touch);
                            }
                            if filter.is_empty() || "canquery".contains(&filter) {
                                draw_bool_row(ui, col_name_w, "CanQuery", &mut item.properties.can_query);
                            }
                            if filter.is_empty() || "collisiongroup".contains(&filter) {
                                draw_read_only_dimmed_row(ui, col_name_w, "CollisionGroup", &item.properties.collision_group);
                            }
                        }
                    }
                }

                // --- CATEGORÍA: ASSEMBLY (Part, MeshPart) ---
                if is_part_like {
                    let show_asm = filter.is_empty()
                        || "anchored".contains(&filter)
                        || "massless".contains(&filter)
                        || "mass".contains(&filter);

                    if show_asm {
                        draw_category_header(ui, "Ensamblaje", &mut self.assembly_expanded, false);
                        if self.assembly_expanded {
                            if filter.is_empty() || "anchored".contains(&filter) {
                                draw_bool_row(ui, col_name_w, "Anchored", &mut item.properties.anchored);
                            }
                            if filter.is_empty() || "massless".contains(&filter) {
                                draw_bool_row(ui, col_name_w, "Massless", &mut item.properties.massless);
                            }
                            if filter.is_empty() || "mass".contains(&filter) {
                                draw_read_only_dimmed_row(ui, col_name_w, "AssemblyMass", &format!("{:.2}", item.properties.mass));
                            }
                        }
                    }
                }

                // --- CATEGORÍA ESPECÍFICA SEGÚN CLASE ---
                match item.class {
                    RobloxClass::Light => {
                        draw_category_header(ui, "Luz", &mut self.specific_expanded, false);
                        if self.specific_expanded {
                            draw_bool_row(ui, col_name_w, "Enabled", &mut item.properties.light_enabled);
                            draw_float_row(ui, col_name_w, "Brightness", &mut item.properties.brightness, 0.0..=10.0, 0.1);
                            draw_float_row(ui, col_name_w, "Range", &mut item.properties.range, 0.0..=120.0, 1.0);
                            draw_bool_row(ui, col_name_w, "Shadows", &mut item.properties.shadows);
                        }
                    }
                    RobloxClass::Sound | RobloxClass::SoundService => {
                        draw_category_header(ui, "Sonido", &mut self.specific_expanded, false);
                        if self.specific_expanded {
                            draw_text_edit_row(ui, col_name_w, "SoundId", &mut item.properties.sound_id);
                            draw_float_row(ui, col_name_w, "Volume", &mut item.properties.volume, 0.0..=2.0, 0.05);
                            draw_float_row(ui, col_name_w, "PlaybackSpeed", &mut item.properties.playback_speed, 0.1..=5.0, 0.1);
                            draw_bool_row(ui, col_name_w, "Playing", &mut item.properties.playing);
                            draw_bool_row(ui, col_name_w, "Looped", &mut item.properties.looped);
                        }
                    }
                    RobloxClass::Script | RobloxClass::ServerScriptService | RobloxClass::StarterCharacterScripts | RobloxClass::StarterPlayerScripts => {
                        draw_category_header(ui, "Script", &mut self.specific_expanded, false);
                        if self.specific_expanded {
                            draw_bool_row(ui, col_name_w, "Enabled", &mut item.properties.script_enabled);
                            draw_read_only_dimmed_row(ui, col_name_w, "RunContext", &item.properties.run_context);
                        }
                    }
                    RobloxClass::Humanoid => {
                        draw_category_header(ui, "Humanoid", &mut self.specific_expanded, false);
                        if self.specific_expanded {
                            draw_float_row(ui, col_name_w, "Health", &mut item.properties.health, 0.0..=item.properties.max_health, 1.0);
                            draw_float_row(ui, col_name_w, "MaxHealth", &mut item.properties.max_health, 1.0..=1000.0, 1.0);
                            draw_float_row(ui, col_name_w, "WalkSpeed", &mut item.properties.walk_speed, 0.0..=100.0, 1.0);
                            draw_float_row(ui, col_name_w, "JumpPower", &mut item.properties.jump_power, 0.0..=200.0, 1.0);
                            draw_bool_row(ui, col_name_w, "AutoRotate", &mut item.properties.auto_rotate);
                        }
                    }
                    RobloxClass::Camera => {
                        draw_category_header(ui, "Camera", &mut self.specific_expanded, false);
                        if self.specific_expanded {
                            draw_float_row(ui, col_name_w, "FieldOfView", &mut item.properties.fov, 10.0..=120.0, 1.0);
                            draw_read_only_dimmed_row(ui, col_name_w, "CameraType", &item.properties.camera_type);
                        }
                    }
                    _ => {}
                }
            });

        close_requested
    }
}

// =========================================================================
// WIDGETS AUXILIARES PARA PROPIEDADES ESTILO ROBLOX STUDIO
// =========================================================================

fn draw_category_header(ui: &mut egui::Ui, title: &str, expanded: &mut bool, has_plus: bool) -> bool {
    let header_h = 22.0;
    let avail_w = ui.available_width();
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(avail_w, header_h), Sense::click());

    let mut plus_clicked = false;

    // Fondo de categoría
    let bg_color = if resp.hovered() { Color32::from_rgb(48, 48, 52) } else { STUDIO_HEADER_BG };
    ui.painter().rect_filled(rect, 0.0, bg_color);

    // Flecha Foldout ▶ / ▼
    let arrow_pos = Pos2::new(rect.min.x + 10.0, rect.center().y);
    icons::foldout_arrow(ui.painter(), arrow_pos, *expanded, Color32::from_rgb(180, 180, 185));

    // Category Label
    ui.painter().text(
        Pos2::new(rect.min.x + 22.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        title,
        FontId::proportional(12.0),
        Color32::from_rgb(220, 220, 220),
    );

    // Botón [+] en el extremo derecho si aplica
    if has_plus {
        let plus_pos = Pos2::new(rect.max.x - 14.0, rect.center().y);
        let plus_rect = Rect::from_center_size(plus_pos, Vec2::new(16.0, 16.0));
        let plus_resp = ui.interact(plus_rect, ui.id().with(title).with("cat_plus"), Sense::click());
        if plus_resp.clicked() {
            plus_clicked = true;
        }
        if plus_resp.hovered() {
            ui.painter().rect_filled(plus_rect, Rounding::same(2.0), STUDIO_HOVER);
        }
        ui.painter().text(
            plus_pos,
            egui::Align2::CENTER_CENTER,
            "+",
            FontId::proportional(13.0),
            if plus_resp.hovered() { Color32::WHITE } else { Color32::from_rgb(180, 180, 185) },
        );
        plus_resp.on_hover_text(format!("Agregar {}", title));
    }

    if resp.clicked() && !plus_clicked {
        *expanded = !*expanded;
    }

    // Separator line at bottom
    ui.painter().line_segment(
        [Pos2::new(rect.min.x, rect.max.y), Pos2::new(rect.max.x, rect.max.y)],
        Stroke::new(1.0, STUDIO_BORDER),
    );

    plus_clicked
}

fn allocate_property_row(ui: &mut egui::Ui, name_w: f32, label: &str, is_dimmed: bool) -> (Rect, Rect, Rect, egui::Response) {
    let row_h = 22.0;
    let avail_w = ui.available_width();
    let (row_rect, row_resp) = ui.allocate_exact_size(Vec2::new(avail_w, row_h), Sense::hover());

    let name_rect = Rect::from_min_max(
        row_rect.min,
        Pos2::new(row_rect.min.x + name_w, row_rect.max.y),
    );
    let val_rect = Rect::from_min_max(
        Pos2::new(row_rect.min.x + name_w, row_rect.min.y),
        row_rect.max,
    );

    // Hover background
    if row_resp.hovered() {
        ui.painter().rect_filled(row_rect, 0.0, STUDIO_HOVER);
    }

    // Property name text (dimmed si es read-only)
    let label_col = if is_dimmed { STUDIO_TEXT_DIMMED } else { STUDIO_TEXT };
    ui.painter().text(
        Pos2::new(name_rect.min.x + 14.0, name_rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        FontId::proportional(11.5),
        label_col,
    );

    // Vertical line between columns
    ui.painter().line_segment(
        [Pos2::new(name_rect.max.x, row_rect.min.y), Pos2::new(name_rect.max.x, row_rect.max.y)],
        Stroke::new(1.0, STUDIO_BORDER),
    );

    // Horizontal line beneath row
    ui.painter().line_segment(
        [Pos2::new(row_rect.min.x, row_rect.max.y), Pos2::new(row_rect.max.x, row_rect.max.y)],
        Stroke::new(1.0, STUDIO_ROW_BORDER),
    );

    (row_rect, name_rect, val_rect, row_resp)
}

fn draw_read_only_dimmed_row(ui: &mut egui::Ui, name_w: f32, label: &str, value: &str) {
    let (_, _, val_rect, _) = allocate_property_row(ui, name_w, label, true);
    ui.painter().text(
        Pos2::new(val_rect.min.x + 8.0, val_rect.center().y),
        egui::Align2::LEFT_CENTER,
        value,
        FontId::proportional(11.5),
        STUDIO_TEXT_DIMMED,
    );
}

fn draw_empty_placeholder_row(ui: &mut egui::Ui, text: &str) {
    let row_h = 24.0;
    let (row_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), row_h), Sense::hover());
    ui.painter().text(
        Pos2::new(row_rect.min.x + 14.0, row_rect.center().y),
        egui::Align2::LEFT_CENTER,
        text,
        FontId::proportional(11.5),
        Color32::from_rgb(115, 115, 120),
    );
    ui.painter().line_segment(
        [Pos2::new(row_rect.min.x, row_rect.max.y), Pos2::new(row_rect.max.x, row_rect.max.y)],
        Stroke::new(1.0, STUDIO_ROW_BORDER),
    );
}

fn draw_text_edit_row(ui: &mut egui::Ui, name_w: f32, label: &str, value: &mut String) {
    let (_, _, val_rect, _) = allocate_property_row(ui, name_w, label, false);
    let edit_rect = Rect::from_min_max(
        Pos2::new(val_rect.min.x + 4.0, val_rect.min.y + 2.0),
        Pos2::new(val_rect.max.x - 4.0, val_rect.max.y - 2.0),
    );
    let mut child = ui.child_ui(edit_rect, egui::Layout::left_to_right(egui::Align::Center), None);
    child.add(
        egui::TextEdit::singleline(value)
            .font(FontId::proportional(11.5))
            .desired_width(edit_rect.width()),
    );
}

fn draw_bool_row(ui: &mut egui::Ui, name_w: f32, label: &str, value: &mut bool) {
    let (_, _, val_rect, _) = allocate_property_row(ui, name_w, label, false);

    // Roblox Studio Checkbox
    let box_center = Pos2::new(val_rect.min.x + 14.0, val_rect.center().y);
    let box_rect = Rect::from_center_size(box_center, Vec2::new(14.0, 14.0));
    let resp = ui.interact(box_rect, ui.id().with(label).with("bool_box"), Sense::click());

    if resp.clicked() {
        *value = !*value;
    }

    let bg_color = if *value {
        STUDIO_SELECTION
    } else {
        STUDIO_FIELD_BG
    };
    let border_color = if *value {
        STUDIO_SELECTION
    } else if resp.hovered() {
        Color32::from_rgb(100, 100, 110)
    } else {
        Color32::from_rgb(60, 60, 65)
    };

    ui.painter().rect_filled(box_rect, Rounding::same(2.0), bg_color);
    ui.painter().rect_stroke(box_rect, Rounding::same(2.0), Stroke::new(1.0, border_color));

    if *value {
        // Draw crisp checkmark
        let p1 = Pos2::new(box_center.x - 3.5, box_center.y);
        let p2 = Pos2::new(box_center.x - 1.0, box_center.y + 3.0);
        let p3 = Pos2::new(box_center.x + 4.0, box_center.y - 3.0);
        ui.painter().line_segment([p1, p2], Stroke::new(1.6, Color32::WHITE));
        ui.painter().line_segment([p2, p3], Stroke::new(1.6, Color32::WHITE));
    }
}

fn draw_float_row(
    ui: &mut egui::Ui,
    name_w: f32,
    label: &str,
    val: &mut f32,
    range: std::ops::RangeInclusive<f32>,
    speed: f64,
) {
    let (_, _, val_rect, _) = allocate_property_row(ui, name_w, label, false);
    let edit_rect = Rect::from_min_max(
        Pos2::new(val_rect.min.x + 4.0, val_rect.min.y + 2.0),
        Pos2::new(val_rect.max.x - 4.0, val_rect.max.y - 2.0),
    );
    let mut child = ui.child_ui(edit_rect, egui::Layout::left_to_right(egui::Align::Center), None);
    child.add(
        egui::DragValue::new(val)
            .range(range)
            .speed(speed)
            .max_decimals(3),
    );
}

fn draw_color_row(ui: &mut egui::Ui, name_w: f32, label: &str, color: &mut Color32) {
    let (_, _, val_rect, _) = allocate_property_row(ui, name_w, label, false);

    // Color Swatch Rect
    let swatch_center = Pos2::new(val_rect.min.x + 14.0, val_rect.center().y);
    let swatch_rect = Rect::from_center_size(swatch_center, Vec2::new(14.0, 14.0));

    let resp = ui.interact(swatch_rect, ui.id().with(label).with("color_swatch"), Sense::click());
    if resp.clicked() {
        let presets = [
            Color32::from_rgb(163, 162, 165), // Medium stone grey
            Color32::from_rgb(196, 40, 28),   // Bright red
            Color32::from_rgb(13, 105, 172),  // Bright blue
            Color32::from_rgb(39, 70, 45),    // Earth green
            Color32::from_rgb(245, 205, 47),  // Bright yellow
            Color32::from_rgb(27, 42, 53),    // Black
            Color32::from_rgb(242, 243, 243), // White
        ];
        let cur = presets.iter().position(|c| *c == *color).unwrap_or(0);
        *color = presets[(cur + 1) % presets.len()];
    }

    ui.painter().rect_filled(swatch_rect, Rounding::same(2.0), *color);
    ui.painter().rect_stroke(swatch_rect, Rounding::same(2.0), Stroke::new(1.0, Color32::from_rgb(80, 80, 85)));

    // RGB text
    let rgb_str = format!("{}, {}, {}", color.r(), color.g(), color.b());
    ui.painter().text(
        Pos2::new(val_rect.min.x + 28.0, val_rect.center().y),
        egui::Align2::LEFT_CENTER,
        rgb_str,
        FontId::proportional(11.0),
        Color32::from_rgb(180, 180, 185),
    );
}

fn draw_material_dropdown_row(ui: &mut egui::Ui, name_w: f32, label: &str, current: &mut String) {
    let (_, _, val_rect, _) = allocate_property_row(ui, name_w, label, false);
    let edit_rect = Rect::from_min_max(
        Pos2::new(val_rect.min.x + 4.0, val_rect.min.y + 2.0),
        Pos2::new(val_rect.max.x - 4.0, val_rect.max.y - 2.0),
    );

    let materials = [
        "Plastic", "SmoothPlastic", "Neon", "Wood", "WoodPlanks", "Slate", "Concrete", "CorrodedMetal", "DiamondPlate", "Foil", "Grass", "Ice", "Brick", "Sand", "Fabric", "Marble", "Granite", "Pebble", "Glass", "ForceField"
    ];

    let mut child = ui.child_ui(edit_rect, egui::Layout::left_to_right(egui::Align::Center), None);
    egui::ComboBox::from_id_salt("material_select_combo")
        .selected_text(current.as_str())
        .width(edit_rect.width() - 8.0)
        .show_ui(&mut child, |ui| {
            for mat in materials {
                ui.selectable_value(current, mat.to_string(), mat);
            }
        });
}

fn draw_vector3_expandable_row(
    ui: &mut egui::Ui,
    name_w: f32,
    label: &str,
    vals: &mut [f32; 3],
    is_expanded: &mut bool,
    speed: f64,
) {
    let (_, name_rect, val_rect, _) = allocate_property_row(ui, name_w, label, false);

    // Flecha foldout en name_rect
    let arrow_pos = Pos2::new(name_rect.min.x + 7.0, name_rect.center().y);
    let arrow_rect = Rect::from_center_size(arrow_pos, Vec2::new(12.0, 12.0));
    let arrow_resp = ui.interact(arrow_rect, ui.id().with(label).with("arrow"), Sense::click());
    if arrow_resp.clicked() {
        *is_expanded = !*is_expanded;
    }
    icons::foldout_arrow(ui.painter(), arrow_pos, *is_expanded, Color32::from_rgb(180, 180, 185));

    // Valor preview compacto: "X, Y, Z"
    let prev_str = format!("{:.2}, {:.2}, {:.2}", vals[0], vals[1], vals[2]);
    ui.painter().text(
        Pos2::new(val_rect.min.x + 8.0, val_rect.center().y),
        egui::Align2::LEFT_CENTER,
        prev_str,
        FontId::proportional(11.5),
        Color32::from_rgb(200, 200, 205),
    );

    // Si está expandido, renderizar filas X, Y, Z indentadas
    if *is_expanded {
        let axes = ["X", "Y", "Z"];
        let colors = [
            Color32::from_rgb(230, 80, 80),  // Red X
            Color32::from_rgb(80, 220, 80),  // Green Y
            Color32::from_rgb(80, 140, 240), // Blue Z
        ];

        for i in 0..3 {
            let (_, sub_name, sub_val, _) = allocate_property_row(ui, name_w, "", false);

            // Sub etiqueta "X" / "Y" / "Z"
            ui.painter().text(
                Pos2::new(sub_name.min.x + 24.0, sub_name.center().y),
                egui::Align2::LEFT_CENTER,
                axes[i],
                FontId::proportional(11.0),
                colors[i],
            );

            // DragValue del eje
            let edit_rect = Rect::from_min_max(
                Pos2::new(sub_val.min.x + 4.0, sub_val.min.y + 2.0),
                Pos2::new(sub_val.max.x - 4.0, sub_val.max.y - 2.0),
            );
            let mut child = ui.child_ui(edit_rect, egui::Layout::left_to_right(egui::Align::Center), None);
            child.add(
                egui::DragValue::new(&mut vals[i])
                    .speed(speed)
                    .max_decimals(3),
            );
        }
    }
}
