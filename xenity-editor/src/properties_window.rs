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
            tags_expanded: false,
            attributes_expanded: false,
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
    ) -> (bool, bool) {
        let mut close_requested = false;
        let mut changed = false;

        // =====================================================================
        // 1. DOCK TAB BAR OFICIAL (Properties Tab con acento cian #00A2FF y botón ⋮)
        // =====================================================================
        let header_h = 26.0;
        let (header_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), header_h), Sense::hover());
        ui.painter().rect_filled(header_rect, 0.0, STUDIO_BG);

        // Pestaña activa Properties
        let tab_w = 100.0;
        let tab_rect = Rect::from_min_size(header_rect.min, Vec2::new(tab_w, header_h));
        ui.painter().rect_filled(tab_rect, 0.0, Color32::from_rgb(40, 40, 44));

        // Franja superior cian activo (Cyan Roblox Studio #00A2FF)
        ui.painter().line_segment(
            [tab_rect.min, Pos2::new(tab_rect.max.x, tab_rect.min.y)],
            Stroke::new(2.0, Color32::from_rgb(0, 162, 255)),
        );

        // Borde derecho de la pestaña
        ui.painter().line_segment(
            [Pos2::new(tab_rect.max.x, tab_rect.min.y), tab_rect.max],
            Stroke::new(1.0, STUDIO_BORDER),
        );

        // Ícono de la pestaña Properties
        let icon_center = Pos2::new(tab_rect.min.x + 13.0, tab_rect.center().y);
        icons::tab_properties_icon(ui.painter(), icon_center, Color32::from_rgb(215, 215, 220));

        // Texto de la pestaña "Properties"
        ui.painter().text(
            Pos2::new(tab_rect.min.x + 25.0, tab_rect.center().y),
            egui::Align2::LEFT_CENTER,
            "Properties",
            FontId::proportional(12.0),
            Color32::from_rgb(235, 235, 240),
        );

        // Botón derecho: ⋮ (Options / Menú de ventana)
        let opt_center = Pos2::new(header_rect.max.x - 12.0, header_rect.center().y);
        let opt_rect = Rect::from_center_size(opt_center, Vec2::new(18.0, 18.0));
        let opt_resp = ui.interact(opt_rect, ui.id().with("prop_dock_menu"), Sense::click());
        if opt_resp.hovered() {
            ui.painter().rect_filled(opt_rect, Rounding::same(2.0), STUDIO_HOVER);
        }
        icons::three_dots_vertical(
            ui.painter(),
            opt_center,
            if opt_resp.hovered() { Color32::WHITE } else { Color32::from_rgb(150, 150, 155) },
        );
        opt_resp.on_hover_text("Options");

        // Separador inferior de cabecera
        ui.painter().line_segment(
            [Pos2::new(header_rect.min.x, header_rect.max.y), Pos2::new(header_rect.max.x, header_rect.max.y)],
            Stroke::new(1.0, STUDIO_BORDER),
        );

        // =====================================================================
        // 2. SEARCH BAR: Filter Properties (Ctrl+Shift+P)
        // =====================================================================
        let toolbar_h = 30.0;
        let (toolbar_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), toolbar_h), Sense::hover());
        ui.painter().rect_filled(toolbar_rect, 0.0, STUDIO_BG);

        let search_h = 22.0;
        let search_y = toolbar_rect.min.y + (toolbar_h - search_h) * 0.5;
        let search_rect = Rect::from_min_size(
            Pos2::new(toolbar_rect.min.x + 6.0, search_y),
            Vec2::new(toolbar_rect.width() - 12.0, search_h),
        );

        ui.painter().rect_filled(search_rect, Rounding::same(3.5), STUDIO_INPUT_BG);
        ui.painter().rect_stroke(search_rect, Rounding::same(3.5), Stroke::new(1.0, STUDIO_BORDER));

        // Ícono de lupa alineado
        let mag_center = Pos2::new(search_rect.min.x + 13.0, search_rect.center().y);
        let mag_rect = Rect::from_center_size(mag_center, Vec2::new(13.0, 13.0));
        ui.painter().image(
            icons.icon_search.id(),
            mag_rect,
            Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
            Color32::from_rgb(160, 160, 160),
        );

        let right_reserve = if self.search_query.is_empty() { 6.0 } else { 22.0 };
        let text_rect = Rect::from_min_max(
            Pos2::new(search_rect.min.x + 25.0, search_rect.min.y + 2.0),
            Pos2::new(search_rect.max.x - right_reserve, search_rect.max.y - 2.0),
        );

        let mut edit_ui = ui.child_ui(text_rect, egui::Layout::left_to_right(egui::Align::Center), None);
        edit_ui.spacing_mut().item_spacing = Vec2::ZERO;
        edit_ui.add(
            egui::TextEdit::singleline(&mut self.search_query)
                .hint_text("Filter Properties (Ctrl+Shift+P)")
                .frame(false)
                .margin(egui::Margin::symmetric(0.0, 1.0))
                .font(FontId::proportional(11.5))
                .desired_width(text_rect.width()),
        );

        if !self.search_query.is_empty() {
            let clear_center = Pos2::new(search_rect.max.x - 11.0, search_rect.center().y);
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
            return (close_requested, false);
        };

        let filter = self.search_query.to_lowercase();
        let col_name_w = (ui.available_width() * 0.42).clamp(110.0, 500.0);

        egui::ScrollArea::vertical()
            .id_salt("roblox_properties_scroll")
            .show(ui, |ui| {
                // --- CATEGORÍA 1: DATA (Exactamente como en la imagen 1 de Roblox Studio) ---
                let show_data = filter.is_empty()
                    || "data".contains(&filter)
                    || "datos".contains(&filter)
                    || "classname".contains(&filter)
                    || "name".contains(&filter)
                    || "parent".contains(&filter)
                    || "archivable".contains(&filter);

                if show_data {
                    draw_category_header(ui, "Data", &mut self.data_expanded, false);
                    if self.data_expanded {
                        if filter.is_empty() || "classname".contains(&filter) {
                            draw_read_only_dimmed_row(ui, col_name_w, "ClassName", &item.properties.class_name);
                        }
                        if filter.is_empty() || "name".contains(&filter) {
                            if draw_text_edit_row(ui, col_name_w, "Name", &mut item.name) {
                                changed = true;
                            }
                        }
                        if filter.is_empty() || "parent".contains(&filter) {
                            draw_read_only_dimmed_row(ui, col_name_w, "Parent", &item.properties.parent_name);
                        }
                        if filter.is_empty() || "archivable".contains(&filter) {
                            if draw_bool_row(ui, col_name_w, "Archivable", &mut item.properties.archivable) {
                                changed = true;
                            }
                        }
                    }
                }

                // --- CATEGORÍA 2: TRANSFORM (Inmediatamente después de Data como en la imagen 1) ---
                let has_transform = matches!(item.class, RobloxClass::Part | RobloxClass::MeshPart | RobloxClass::Model | RobloxClass::Camera);
                if has_transform {
                    let show_trans = filter.is_empty()
                        || "transform".contains(&filter)
                        || "position".contains(&filter)
                        || "orientation".contains(&filter)
                        || "size".contains(&filter)
                        || "cframe".contains(&filter);

                    if show_trans {
                        draw_category_header(ui, "Transform", &mut self.transform_expanded, false);
                        if self.transform_expanded {
                            if filter.is_empty() || "position".contains(&filter) {
                                if draw_vector3_expandable_row(
                                    ui,
                                    col_name_w,
                                    "Position",
                                    &mut item.properties.position,
                                    &mut self.pos_sub_expanded,
                                    0.1,
                                ) {
                                    changed = true;
                                }
                            }
                            if filter.is_empty() || "orientation".contains(&filter) {
                                if draw_vector3_expandable_row(
                                    ui,
                                    col_name_w,
                                    "Orientation",
                                    &mut item.properties.orientation,
                                    &mut self.rot_sub_expanded,
                                    0.5,
                                ) {
                                    changed = true;
                                }
                            }
                            if filter.is_empty() || "size".contains(&filter) {
                                if draw_vector3_expandable_row(
                                    ui,
                                    col_name_w,
                                    "Size",
                                    &mut item.properties.size,
                                    &mut self.size_sub_expanded,
                                    0.1,
                                ) {
                                    changed = true;
                                }
                            }
                        }
                    }
                }

                // --- CATEGORÍA 3: APPEARANCE (Para Part, MeshPart, Model, Terrain) ---
                let is_part_like = matches!(item.class, RobloxClass::Part | RobloxClass::MeshPart | RobloxClass::Terrain);
                if is_part_like {
                    let show_app = filter.is_empty()
                        || "appearance".contains(&filter)
                        || "apariencia".contains(&filter)
                        || "color".contains(&filter)
                        || "material".contains(&filter)
                        || "transparency".contains(&filter)
                        || "reflectance".contains(&filter)
                        || "castshadow".contains(&filter);

                    if show_app {
                        draw_category_header(ui, "Appearance", &mut self.appearance_expanded, false);
                        if self.appearance_expanded {
                            if filter.is_empty() || "color".contains(&filter) {
                                if draw_color_row(ui, col_name_w, "Color", &mut item.properties.color) {
                                    changed = true;
                                }
                            }
                            if filter.is_empty() || "material".contains(&filter) {
                                if draw_material_dropdown_row(ui, col_name_w, "Material", &mut item.properties.material) {
                                    changed = true;
                                }
                            }
                            if filter.is_empty() || "reflectance".contains(&filter) {
                                if draw_float_row(ui, col_name_w, "Reflectance", &mut item.properties.reflectance, 0.0..=1.0, 0.02) {
                                    changed = true;
                                }
                            }
                            if filter.is_empty() || "transparency".contains(&filter) {
                                if draw_float_row(ui, col_name_w, "Transparency", &mut item.properties.transparency, 0.0..=1.0, 0.02) {
                                    changed = true;
                                }
                            }
                            if filter.is_empty() || "castshadow".contains(&filter) {
                                if draw_bool_row(ui, col_name_w, "CastShadow", &mut item.properties.cast_shadow) {
                                    changed = true;
                                }
                            }
                        }
                    }
                }

                // --- CATEGORÍA 4: COLLISION (Part, MeshPart) ---
                if is_part_like {
                    let show_col = filter.is_empty()
                        || "collision".contains(&filter)
                        || "cancollide".contains(&filter)
                        || "cantouch".contains(&filter)
                        || "canquery".contains(&filter)
                        || "collisiongroup".contains(&filter);

                    if show_col {
                        draw_category_header(ui, "Collision", &mut self.collision_expanded, false);
                        if self.collision_expanded {
                            if filter.is_empty() || "cancollide".contains(&filter) {
                                if draw_bool_row(ui, col_name_w, "CanCollide", &mut item.properties.can_collide) {
                                    changed = true;
                                }
                            }
                            if filter.is_empty() || "cantouch".contains(&filter) {
                                if draw_bool_row(ui, col_name_w, "CanTouch", &mut item.properties.can_touch) {
                                    changed = true;
                                }
                            }
                            if filter.is_empty() || "canquery".contains(&filter) {
                                if draw_bool_row(ui, col_name_w, "CanQuery", &mut item.properties.can_query) {
                                    changed = true;
                                }
                            }
                            if filter.is_empty() || "collisiongroup".contains(&filter) {
                                draw_read_only_dimmed_row(ui, col_name_w, "CollisionGroup", &item.properties.collision_group);
                            }
                        }
                    }
                }

                // --- CATEGORÍA 5: ASSEMBLY (Part, MeshPart) ---
                if is_part_like {
                    let show_asm = filter.is_empty()
                        || "assembly".contains(&filter)
                        || "anchored".contains(&filter)
                        || "massless".contains(&filter)
                        || "mass".contains(&filter);

                    if show_asm {
                        draw_category_header(ui, "Assembly", &mut self.assembly_expanded, false);
                        if self.assembly_expanded {
                            if filter.is_empty() || "anchored".contains(&filter) {
                                if draw_bool_row(ui, col_name_w, "Anchored", &mut item.properties.anchored) {
                                    changed = true;
                                }
                            }
                            if filter.is_empty() || "massless".contains(&filter) {
                                if draw_bool_row(ui, col_name_w, "Massless", &mut item.properties.massless) {
                                    changed = true;
                                }
                            }
                            if filter.is_empty() || "mass".contains(&filter) {
                                draw_read_only_dimmed_row(ui, col_name_w, "AssemblyMass", &format!("{:.2}", item.properties.mass));
                            }
                        }
                    }
                }

                // --- CATEGORÍA 6: TAGS & ATTRIBUTES (Al final, colapsadas por defecto) ---
                let show_tags = filter.is_empty() || "tags".contains(&filter) || "etiquetas".contains(&filter);
                if show_tags {
                    let plus_clicked = draw_category_header(ui, "Tags", &mut self.tags_expanded, true);
                    if plus_clicked {
                        item.properties.tags.push(format!("Tag_{}", item.properties.tags.len() + 1));
                    }
                    if self.tags_expanded {
                        if item.properties.tags.is_empty() {
                            draw_empty_placeholder_row(ui, "No tags added yet");
                        } else {
                            for tag in &item.properties.tags {
                                draw_read_only_dimmed_row(ui, col_name_w, "Tag", tag);
                            }
                        }
                    }
                }

                let show_attr = filter.is_empty() || "attributes".contains(&filter) || "atributos".contains(&filter);
                if show_attr {
                    let plus_clicked = draw_category_header(ui, "Attributes", &mut self.attributes_expanded, true);
                    if plus_clicked {
                        item.properties.attributes.push((format!("Attr_{}", item.properties.attributes.len() + 1), "Value".into()));
                    }
                    if self.attributes_expanded {
                        if item.properties.attributes.is_empty() {
                            draw_empty_placeholder_row(ui, "No attributes added yet");
                        } else {
                            for (attr_name, attr_val) in &item.properties.attributes {
                                draw_read_only_dimmed_row(ui, col_name_w, attr_name, attr_val);
                            }
                        }
                    }
                }

                // --- CATEGORÍA ESPECÍFICA SEGÚN CLASE ---
                match item.class {
                    RobloxClass::Light => {
                        draw_category_header(ui, "Luz", &mut self.specific_expanded, false);
                        if self.specific_expanded {
                            if draw_bool_row(ui, col_name_w, "Enabled", &mut item.properties.light_enabled) { changed = true; }
                            if draw_float_row(ui, col_name_w, "Brightness", &mut item.properties.brightness, 0.0..=10.0, 0.1) { changed = true; }
                            if draw_float_row(ui, col_name_w, "Range", &mut item.properties.range, 0.0..=120.0, 1.0) { changed = true; }
                            if draw_bool_row(ui, col_name_w, "Shadows", &mut item.properties.shadows) { changed = true; }
                        }
                    }
                    RobloxClass::Sound | RobloxClass::SoundService => {
                        draw_category_header(ui, "Sonido", &mut self.specific_expanded, false);
                        if self.specific_expanded {
                            if draw_text_edit_row(ui, col_name_w, "SoundId", &mut item.properties.sound_id) { changed = true; }
                            if draw_float_row(ui, col_name_w, "Volume", &mut item.properties.volume, 0.0..=2.0, 0.05) { changed = true; }
                            if draw_float_row(ui, col_name_w, "PlaybackSpeed", &mut item.properties.playback_speed, 0.1..=5.0, 0.1) { changed = true; }
                            if draw_bool_row(ui, col_name_w, "Playing", &mut item.properties.playing) { changed = true; }
                            if draw_bool_row(ui, col_name_w, "Looped", &mut item.properties.looped) { changed = true; }
                        }
                    }
                    RobloxClass::Script | RobloxClass::ServerScriptService | RobloxClass::StarterCharacterScripts | RobloxClass::StarterPlayerScripts => {
                        draw_category_header(ui, "Script", &mut self.specific_expanded, false);
                        if self.specific_expanded {
                            if draw_bool_row(ui, col_name_w, "Enabled", &mut item.properties.script_enabled) { changed = true; }
                            draw_read_only_dimmed_row(ui, col_name_w, "RunContext", &item.properties.run_context);
                        }
                    }
                    RobloxClass::Humanoid => {
                        draw_category_header(ui, "Humanoid", &mut self.specific_expanded, false);
                        if self.specific_expanded {
                            if draw_float_row(ui, col_name_w, "Health", &mut item.properties.health, 0.0..=item.properties.max_health, 1.0) { changed = true; }
                            if draw_float_row(ui, col_name_w, "MaxHealth", &mut item.properties.max_health, 1.0..=1000.0, 1.0) { changed = true; }
                            if draw_float_row(ui, col_name_w, "WalkSpeed", &mut item.properties.walk_speed, 0.0..=100.0, 1.0) { changed = true; }
                            if draw_float_row(ui, col_name_w, "JumpPower", &mut item.properties.jump_power, 0.0..=200.0, 1.0) { changed = true; }
                            if draw_bool_row(ui, col_name_w, "AutoRotate", &mut item.properties.auto_rotate) { changed = true; }
                        }
                    }
                    RobloxClass::Camera => {
                        draw_category_header(ui, "Camera", &mut self.specific_expanded, false);
                        if self.specific_expanded {
                            if draw_float_row(ui, col_name_w, "FieldOfView", &mut item.properties.fov, 10.0..=120.0, 1.0) { changed = true; }
                            draw_read_only_dimmed_row(ui, col_name_w, "CameraType", &item.properties.camera_type);
                        }
                    }
                    _ => {}
                }
            });

        (close_requested, changed)
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

    // Row background (sutil alternado / hover)
    let bg_col = if row_resp.hovered() { Color32::from_rgb(38, 38, 42) } else { Color32::from_rgb(26, 26, 28) };
    ui.painter().rect_filled(row_rect, 0.0, bg_col);

    // Property name text
    let label_col = if is_dimmed { Color32::from_rgb(175, 175, 180) } else { Color32::from_rgb(215, 215, 220) };
    ui.painter().text(
        Pos2::new(name_rect.min.x + 14.0, name_rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        FontId::proportional(11.5),
        label_col,
    );

    // Línea vertical entre columnas (rejilla Roblox Studio)
    ui.painter().line_segment(
        [Pos2::new(name_rect.max.x, row_rect.min.y), Pos2::new(name_rect.max.x, row_rect.max.y)],
        Stroke::new(1.0, Color32::from_rgb(38, 38, 42)),
    );

    // Línea horizontal inferior entre filas
    ui.painter().line_segment(
        [Pos2::new(row_rect.min.x, row_rect.max.y), Pos2::new(row_rect.max.x, row_rect.max.y)],
        Stroke::new(1.0, Color32::from_rgb(32, 32, 35)),
    );

    (row_rect, name_rect, val_rect, row_resp)
}

fn draw_read_only_dimmed_row(ui: &mut egui::Ui, name_w: f32, label: &str, value: &str) {
    let (_, _, val_rect, _) = allocate_property_row(ui, name_w, label, false);
    ui.painter().text(
        Pos2::new(val_rect.min.x + 8.0, val_rect.center().y),
        egui::Align2::LEFT_CENTER,
        value,
        FontId::proportional(11.5),
        Color32::from_rgb(215, 215, 220),
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

fn draw_text_edit_row(ui: &mut egui::Ui, name_w: f32, label: &str, value: &mut String) -> bool {
    let (_, _, val_rect, _) = allocate_property_row(ui, name_w, label, false);
    let edit_rect = Rect::from_min_max(
        Pos2::new(val_rect.min.x + 3.0, val_rect.min.y + 2.0),
        Pos2::new(val_rect.max.x - 4.0, val_rect.max.y - 2.0),
    );
    ui.painter().rect_filled(edit_rect, Rounding::same(2.5), STUDIO_FIELD_BG);
    ui.painter().rect_stroke(edit_rect, Rounding::same(2.5), Stroke::new(1.0, Color32::from_rgb(45, 45, 50)));

    let text_inner = Rect::from_min_max(
        Pos2::new(edit_rect.min.x + 4.0, edit_rect.min.y),
        Pos2::new(edit_rect.max.x - 4.0, edit_rect.max.y),
    );
    let mut child = ui.child_ui(text_inner, egui::Layout::left_to_right(egui::Align::Center), None);
    child.spacing_mut().item_spacing = Vec2::ZERO;
    let resp = child.add(
        egui::TextEdit::singleline(value)
            .frame(false)
            .margin(egui::Margin::symmetric(0.0, 1.0))
            .font(FontId::proportional(11.5))
            .text_color(Color32::WHITE)
            .desired_width(text_inner.width()),
    );
    resp.changed()
}

fn draw_bool_row(ui: &mut egui::Ui, name_w: f32, label: &str, value: &mut bool) -> bool {
    let (_, _, val_rect, _) = allocate_property_row(ui, name_w, label, false);

    // Roblox Studio Checkbox
    let box_center = Pos2::new(val_rect.min.x + 14.0, val_rect.center().y);
    let box_rect = Rect::from_center_size(box_center, Vec2::new(14.0, 14.0));
    let resp = ui.interact(box_rect, ui.id().with(label).with("bool_box"), Sense::click());

    let changed = resp.clicked();
    if changed {
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

    changed
}

fn draw_float_row(
    ui: &mut egui::Ui,
    name_w: f32,
    label: &str,
    val: &mut f32,
    range: std::ops::RangeInclusive<f32>,
    speed: f64,
) -> bool {
    let (_, _, val_rect, _) = allocate_property_row(ui, name_w, label, false);
    let edit_rect = Rect::from_min_max(
        Pos2::new(val_rect.min.x + 4.0, val_rect.min.y + 2.0),
        Pos2::new(val_rect.max.x - 4.0, val_rect.max.y - 2.0),
    );
    let mut child = ui.child_ui(edit_rect, egui::Layout::left_to_right(egui::Align::Center), None);
    let resp = child.add(
        egui::DragValue::new(val)
            .range(range)
            .speed(speed)
            .max_decimals(3),
    );
    resp.changed()
}

fn draw_color_row(ui: &mut egui::Ui, name_w: f32, label: &str, color: &mut Color32) -> bool {
    let (_, _, val_rect, _) = allocate_property_row(ui, name_w, label, false);

    // Color Swatch Rect
    let swatch_center = Pos2::new(val_rect.min.x + 14.0, val_rect.center().y);
    let swatch_rect = Rect::from_center_size(swatch_center, Vec2::new(14.0, 14.0));

    let resp = ui.interact(swatch_rect, ui.id().with(label).with("color_swatch"), Sense::click());
    let mut changed = false;
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
        changed = true;
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

    changed
}

fn draw_material_dropdown_row(ui: &mut egui::Ui, name_w: f32, label: &str, current: &mut String) -> bool {
    let (_, _, val_rect, _) = allocate_property_row(ui, name_w, label, false);
    let edit_rect = Rect::from_min_max(
        Pos2::new(val_rect.min.x + 4.0, val_rect.min.y + 2.0),
        Pos2::new(val_rect.max.x - 4.0, val_rect.max.y - 2.0),
    );

    let materials = [
        "Plastic", "SmoothPlastic", "Neon", "Wood", "WoodPlanks", "Slate", "Concrete", "CorrodedMetal", "DiamondPlate", "Foil", "Grass", "Ice", "Brick", "Sand", "Fabric", "Marble", "Granite", "Pebble", "Glass", "ForceField"
    ];

    let mut changed = false;
    let mut child = ui.child_ui(edit_rect, egui::Layout::left_to_right(egui::Align::Center), None);
    egui::ComboBox::from_id_salt("material_select_combo")
        .selected_text(current.as_str())
        .width(edit_rect.width() - 8.0)
        .show_ui(&mut child, |ui| {
            for mat in materials {
                if ui.selectable_value(current, mat.to_string(), mat).changed() {
                    changed = true;
                }
            }
        });
    changed
}

fn draw_vector3_expandable_row(
    ui: &mut egui::Ui,
    name_w: f32,
    label: &str,
    vals: &mut [f32; 3],
    is_expanded: &mut bool,
    speed: f64,
) -> bool {
    let mut changed = false;
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
            let resp = child.add(
                egui::DragValue::new(&mut vals[i])
                    .speed(speed)
                    .max_decimals(3),
            );
            if resp.changed() {
                changed = true;
            }
        }
    }
    changed
}
