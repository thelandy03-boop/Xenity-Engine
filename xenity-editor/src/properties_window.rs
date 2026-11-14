use eframe::egui::{self, Color32, CursorIcon, FontId, Pos2, Rect, Rounding, Sense, Stroke, Vec2};
use crate::hierarchy_window::{ExplorerItem, RobloxClass, STUDIO_BG, STUDIO_BORDER, STUDIO_HOVER, STUDIO_INPUT_BG, STUDIO_SELECTION, STUDIO_TEXT, STUDIO_TREE_BG};
use crate::icons;

pub const STUDIO_HEADER_BG: Color32 = Color32::from_rgb(42, 42, 45);     // #2A2A2D
pub const STUDIO_ROW_BORDER: Color32 = Color32::from_rgb(38, 38, 40);     // #262628
pub const STUDIO_FIELD_BG: Color32 = Color32::from_rgb(26, 26, 26);       // #1A1A1A
pub const STUDIO_TEXT_MUTED: Color32 = Color32::from_rgb(150, 150, 150);  // #969696

pub struct PropertiesWindowState {
    pub search_query: String,
    pub data_expanded: bool,
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
    ) {
        // 1. PESTAÑA SUPERIOR [ Properties - ClassName "ItemName" ]
        let tab_h = 26.0;
        let (tab_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), tab_h), Sense::hover());
        ui.painter().rect_filled(tab_rect, 0.0, STUDIO_BG);

        let active_tab_rect = Rect::from_min_size(tab_rect.min, Vec2::new(96.0, tab_h));
        ui.painter().rect_filled(active_tab_rect, Rounding { nw: 3.0, ne: 3.0, sw: 0.0, se: 0.0 }, STUDIO_TREE_BG);
        ui.painter().line_segment(
            [Pos2::new(active_tab_rect.min.x, active_tab_rect.min.y), Pos2::new(active_tab_rect.max.x, active_tab_rect.min.y)],
            Stroke::new(2.0, STUDIO_SELECTION),
        );

        ui.painter().text(
            active_tab_rect.center(),
            egui::Align2::CENTER_CENTER,
            "Properties",
            FontId::proportional(11.5),
            Color32::WHITE,
        );

        let menu_pos = Pos2::new(tab_rect.max.x - 12.0, tab_rect.center().y);
        icons::kebab_menu(ui.painter(), menu_pos, Color32::from_rgb(140, 140, 140));

        ui.painter().line_segment(
            [Pos2::new(tab_rect.min.x, tab_rect.max.y), Pos2::new(tab_rect.max.x, tab_rect.max.y)],
            Stroke::new(1.0, STUDIO_BORDER),
        );

        // 2. SUB-BARRA DE BÚSQUEDA (Filter Properties)
        let toolbar_h = 32.0;
        let (toolbar_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), toolbar_h), Sense::hover());
        ui.painter().rect_filled(toolbar_rect, 0.0, STUDIO_BG);

        let search_h = 22.0;
        let search_y = toolbar_rect.min.y + (toolbar_h - search_h) * 0.5;
        let search_rect = Rect::from_min_size(
            Pos2::new(toolbar_rect.min.x + 8.0, search_y),
            Vec2::new(toolbar_rect.width() - 16.0, search_h),
        );

        ui.painter().rect_filled(search_rect, Rounding::same(4.0), STUDIO_INPUT_BG);
        ui.painter().rect_stroke(search_rect, Rounding::same(4.0), Stroke::new(1.0, STUDIO_BORDER));

        let mag_center = Pos2::new(search_rect.min.x + 12.0, search_rect.center().y);
        icons::magnifying_glass(ui.painter(), mag_center, 3.8, Color32::from_rgb(140, 140, 140));

        let clear_w = if self.search_query.is_empty() { 0.0 } else { 18.0 };
        let text_rect = Rect::from_min_max(
            Pos2::new(search_rect.min.x + 24.0, search_rect.min.y),
            Pos2::new(search_rect.max.x - 6.0 - clear_w, search_rect.max.y),
        );

        let mut edit_ui = ui.child_ui(text_rect, egui::Layout::left_to_right(egui::Align::Center), None);
        edit_ui.add(
            egui::TextEdit::singleline(&mut self.search_query)
                .hint_text("Filter Properties (Ctrl+Shift+P)")
                .frame(false)
                .font(FontId::proportional(11.5))
                .desired_width(text_rect.width()),
        );

        if !self.search_query.is_empty() {
            let clear_center = Pos2::new(search_rect.max.x - 10.0, search_rect.center().y);
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

        // 3. CONTENIDO DE PROPIEDADES (Roblox Studio Grid)
        let body_rect = Rect::from_min_size(ui.cursor().min, ui.available_size());
        ui.painter().rect_filled(body_rect, 0.0, STUDIO_TREE_BG);

        let Some(item) = selected_item else {
            // No selection placeholder
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.label(
                    egui::RichText::new("Select an instance to view its properties.")
                        .color(STUDIO_TEXT_MUTED)
                        .size(12.0),
                );
            });
            return;
        };

        let filter = self.search_query.to_lowercase();
        let col_name_w = 120.0;

        egui::ScrollArea::vertical()
            .id_salt("roblox_properties_scroll")
            .show(ui, |ui| {
                // --- CATEGORÍA: DATA ---
                let show_data = filter.is_empty()
                    || "name".contains(&filter)
                    || "classname".contains(&filter)
                    || "parent".contains(&filter)
                    || "archivable".contains(&filter);

                if show_data {
                    draw_category_header(ui, "Data", &mut self.data_expanded);
                    if self.data_expanded {
                        if filter.is_empty() || "classname".contains(&filter) {
                            draw_read_only_string_row(ui, col_name_w, "ClassName", &item.properties.class_name);
                        }
                        if filter.is_empty() || "name".contains(&filter) {
                            draw_text_edit_row(ui, col_name_w, "Name", &mut item.name);
                        }
                        if filter.is_empty() || "parent".contains(&filter) {
                            draw_read_only_string_row(ui, col_name_w, "Parent", &item.properties.parent_name);
                        }
                        if filter.is_empty() || "archivable".contains(&filter) {
                            draw_bool_row(ui, col_name_w, "Archivable", &mut item.properties.archivable);
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
                        draw_category_header(ui, "Appearance", &mut self.appearance_expanded);
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
                        draw_category_header(ui, "Transform", &mut self.transform_expanded);
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
                        draw_category_header(ui, "Collision", &mut self.collision_expanded);
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
                                draw_read_only_string_row(ui, col_name_w, "CollisionGroup", &item.properties.collision_group);
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
                        draw_category_header(ui, "Assembly", &mut self.assembly_expanded);
                        if self.assembly_expanded {
                            if filter.is_empty() || "anchored".contains(&filter) {
                                draw_bool_row(ui, col_name_w, "Anchored", &mut item.properties.anchored);
                            }
                            if filter.is_empty() || "massless".contains(&filter) {
                                draw_bool_row(ui, col_name_w, "Massless", &mut item.properties.massless);
                            }
                            if filter.is_empty() || "mass".contains(&filter) {
                                draw_float_row(ui, col_name_w, "AssemblyMass", &mut item.properties.mass, 0.0..=1000.0, 0.1);
                            }
                        }
                    }
                }

                // --- CATEGORÍA ESPECÍFICA SEGÚN LA CLASE ---
                match item.class {
                    RobloxClass::Light => {
                        let show_light = filter.is_empty()
                            || "brightness".contains(&filter)
                            || "range".contains(&filter)
                            || "shadows".contains(&filter)
                            || "enabled".contains(&filter)
                            || "color".contains(&filter);

                        if show_light {
                            draw_category_header(ui, "Light", &mut self.specific_expanded);
                            if self.specific_expanded {
                                draw_bool_row(ui, col_name_w, "Enabled", &mut item.properties.light_enabled);
                                draw_color_row(ui, col_name_w, "Color", &mut item.properties.color);
                                draw_float_row(ui, col_name_w, "Brightness", &mut item.properties.brightness, 0.0..=50.0, 0.1);
                                draw_float_row(ui, col_name_w, "Range", &mut item.properties.range, 0.0..=120.0, 1.0);
                                draw_bool_row(ui, col_name_w, "Shadows", &mut item.properties.shadows);
                            }
                        }
                    }
                    RobloxClass::Sound => {
                        let show_sound = filter.is_empty()
                            || "soundid".contains(&filter)
                            || "volume".contains(&filter)
                            || "playbackspeed".contains(&filter)
                            || "playing".contains(&filter)
                            || "looped".contains(&filter);

                        if show_sound {
                            draw_category_header(ui, "Playback", &mut self.specific_expanded);
                            if self.specific_expanded {
                                draw_text_edit_row(ui, col_name_w, "SoundId", &mut item.properties.sound_id);
                                draw_float_row(ui, col_name_w, "Volume", &mut item.properties.volume, 0.0..=5.0, 0.05);
                                draw_float_row(ui, col_name_w, "PlaybackSpeed", &mut item.properties.playback_speed, 0.1..=5.0, 0.05);
                                draw_bool_row(ui, col_name_w, "Playing", &mut item.properties.playing);
                                draw_bool_row(ui, col_name_w, "Looped", &mut item.properties.looped);
                            }
                        }
                    }
                    RobloxClass::Script => {
                        let show_script = filter.is_empty()
                            || "enabled".contains(&filter)
                            || "runcontext".contains(&filter);

                        if show_script {
                            draw_category_header(ui, "Script", &mut self.specific_expanded);
                            if self.specific_expanded {
                                draw_bool_row(ui, col_name_w, "Enabled", &mut item.properties.script_enabled);
                                draw_run_context_dropdown_row(ui, col_name_w, "RunContext", &mut item.properties.run_context);
                            }
                        }
                    }
                    RobloxClass::Humanoid => {
                        let show_hum = filter.is_empty()
                            || "health".contains(&filter)
                            || "maxhealth".contains(&filter)
                            || "walkspeed".contains(&filter)
                            || "jumppower".contains(&filter)
                            || "autorotate".contains(&filter);

                        if show_hum {
                            draw_category_header(ui, "Game", &mut self.specific_expanded);
                            if self.specific_expanded {
                                draw_float_row(ui, col_name_w, "Health", &mut item.properties.health, 0.0..=1000.0, 1.0);
                                draw_float_row(ui, col_name_w, "MaxHealth", &mut item.properties.max_health, 1.0..=1000.0, 1.0);
                                draw_float_row(ui, col_name_w, "WalkSpeed", &mut item.properties.walk_speed, 0.0..=100.0, 0.5);
                                draw_float_row(ui, col_name_w, "JumpPower", &mut item.properties.jump_power, 0.0..=200.0, 1.0);
                                draw_bool_row(ui, col_name_w, "AutoRotate", &mut item.properties.auto_rotate);
                            }
                        }
                    }
                    RobloxClass::Camera => {
                        let show_cam = filter.is_empty()
                            || "fieldofview".contains(&filter)
                            || "cameratype".contains(&filter);

                        if show_cam {
                            draw_category_header(ui, "Camera", &mut self.specific_expanded);
                            if self.specific_expanded {
                                draw_float_row(ui, col_name_w, "FieldOfView", &mut item.properties.fov, 1.0..=120.0, 1.0);
                                draw_camera_type_dropdown_row(ui, col_name_w, "CameraType", &mut item.properties.camera_type);
                            }
                        }
                    }
                    _ => {}
                }
            });
    }
}

// --- WIDGETS Y FILAS DE ROBLOX STUDIO ---

fn draw_category_header(ui: &mut egui::Ui, title: &str, expanded: &mut bool) {
    let row_h = 24.0;
    let avail_w = ui.available_width();
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(avail_w, row_h), Sense::click());

    if resp.clicked() {
        *expanded = !*expanded;
    }

    let bg = if resp.hovered() {
        Color32::from_rgb(52, 52, 56)
    } else {
        STUDIO_HEADER_BG
    };
    ui.painter().rect_filled(rect, 0.0, bg);

    // Foldout arrow
    let arrow_pos = Pos2::new(rect.min.x + 10.0, rect.center().y);
    icons::foldout_arrow(ui.painter(), arrow_pos, *expanded, Color32::from_rgb(190, 190, 190));

    // Category Label
    ui.painter().text(
        Pos2::new(rect.min.x + 22.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        title,
        FontId::proportional(12.0),
        Color32::from_rgb(220, 220, 220),
    );

    // Separator line at bottom
    ui.painter().line_segment(
        [Pos2::new(rect.min.x, rect.max.y), Pos2::new(rect.max.x, rect.max.y)],
        Stroke::new(1.0, STUDIO_BORDER),
    );
}

fn allocate_property_row(ui: &mut egui::Ui, name_w: f32, label: &str) -> (Rect, Rect, Rect, egui::Response) {
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

    // Property name text
    ui.painter().text(
        Pos2::new(name_rect.min.x + 14.0, name_rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        FontId::proportional(11.5),
        STUDIO_TEXT,
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

fn draw_read_only_string_row(ui: &mut egui::Ui, name_w: f32, label: &str, value: &str) {
    let (_, _, val_rect, _) = allocate_property_row(ui, name_w, label);
    ui.painter().text(
        Pos2::new(val_rect.min.x + 8.0, val_rect.center().y),
        egui::Align2::LEFT_CENTER,
        value,
        FontId::proportional(11.5),
        Color32::from_rgb(160, 160, 165),
    );
}

fn draw_text_edit_row(ui: &mut egui::Ui, name_w: f32, label: &str, value: &mut String) {
    let (_, _, val_rect, _) = allocate_property_row(ui, name_w, label);
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
    let (_, _, val_rect, _) = allocate_property_row(ui, name_w, label);

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
    let (_, _, val_rect, _) = allocate_property_row(ui, name_w, label);
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
    let (_, _, val_rect, _) = allocate_property_row(ui, name_w, label);

    // Color Swatch Rect
    let swatch_center = Pos2::new(val_rect.min.x + 14.0, val_rect.center().y);
    let swatch_rect = Rect::from_center_size(swatch_center, Vec2::new(14.0, 14.0));

    let resp = ui.interact(swatch_rect, ui.id().with(label).with("color_swatch"), Sense::click());
    if resp.clicked() {
        // Ciclar o alternar rápidamente colores de ejemplo de Roblox Studio
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

    // RGB label "[R, G, B]"
    let rgb_str = format!("[{}, {}, {}]", color.r(), color.g(), color.b());
    ui.painter().text(
        Pos2::new(swatch_rect.max.x + 8.0, val_rect.center().y),
        egui::Align2::LEFT_CENTER,
        rgb_str,
        FontId::proportional(11.5),
        Color32::WHITE,
    );
}

fn draw_material_dropdown_row(ui: &mut egui::Ui, name_w: f32, label: &str, material: &mut String) {
    let (_, _, val_rect, _) = allocate_property_row(ui, name_w, label);
    let btn_rect = Rect::from_min_max(
        Pos2::new(val_rect.min.x + 4.0, val_rect.min.y + 2.0),
        Pos2::new(val_rect.max.x - 4.0, val_rect.max.y - 2.0),
    );

    let materials = ["Plastic", "SmoothPlastic", "Neon", "Wood", "Metal", "Concrete", "Glass", "CorrodedMetal"];
    let mut child = ui.child_ui(btn_rect, egui::Layout::left_to_right(egui::Align::Center), None);

    egui::ComboBox::from_id_salt(ui.id().with(label).with("mat_combo"))
        .selected_text(material.as_str())
        .show_ui(&mut child, |ui| {
            for m in materials {
                if ui.selectable_label(*material == m, m).clicked() {
                    *material = m.to_string();
                }
            }
        });
}

fn draw_run_context_dropdown_row(ui: &mut egui::Ui, name_w: f32, label: &str, run_ctx: &mut String) {
    let (_, _, val_rect, _) = allocate_property_row(ui, name_w, label);
    let btn_rect = Rect::from_min_max(
        Pos2::new(val_rect.min.x + 4.0, val_rect.min.y + 2.0),
        Pos2::new(val_rect.max.x - 4.0, val_rect.max.y - 2.0),
    );

    let options = ["Legacy", "Server", "Client"];
    let mut child = ui.child_ui(btn_rect, egui::Layout::left_to_right(egui::Align::Center), None);

    egui::ComboBox::from_id_salt(ui.id().with(label).with("run_ctx_combo"))
        .selected_text(run_ctx.as_str())
        .show_ui(&mut child, |ui| {
            for opt in options {
                if ui.selectable_label(*run_ctx == opt, opt).clicked() {
                    *run_ctx = opt.to_string();
                }
            }
        });
}

fn draw_camera_type_dropdown_row(ui: &mut egui::Ui, name_w: f32, label: &str, cam_type: &mut String) {
    let (_, _, val_rect, _) = allocate_property_row(ui, name_w, label);
    let btn_rect = Rect::from_min_max(
        Pos2::new(val_rect.min.x + 4.0, val_rect.min.y + 2.0),
        Pos2::new(val_rect.max.x - 4.0, val_rect.max.y - 2.0),
    );

    let options = ["Custom", "Scriptable", "Fixed", "Attach", "Watch"];
    let mut child = ui.child_ui(btn_rect, egui::Layout::left_to_right(egui::Align::Center), None);

    egui::ComboBox::from_id_salt(ui.id().with(label).with("cam_type_combo"))
        .selected_text(cam_type.as_str())
        .show_ui(&mut child, |ui| {
            for opt in options {
                if ui.selectable_label(*cam_type == opt, opt).clicked() {
                    *cam_type = opt.to_string();
                }
            }
        });
}

fn draw_vector3_expandable_row(
    ui: &mut egui::Ui,
    name_w: f32,
    label: &str,
    vec3: &mut [f32; 3],
    is_expanded: &mut bool,
    speed: f64,
) {
    let row_h = 22.0;
    let avail_w = ui.available_width();
    let (row_rect, row_resp) = ui.allocate_exact_size(Vec2::new(avail_w, row_h), Sense::hover());

    let name_rect = Rect::from_min_max(row_rect.min, Pos2::new(row_rect.min.x + name_w, row_rect.max.y));
    let val_rect = Rect::from_min_max(Pos2::new(row_rect.min.x + name_w, row_rect.min.y), row_rect.max);

    if row_resp.hovered() {
        ui.painter().rect_filled(row_rect, 0.0, STUDIO_HOVER);
    }

    // Small arrow button for expanding
    let arrow_pos = Pos2::new(name_rect.min.x + 6.0, name_rect.center().y);
    let arrow_rect = Rect::from_center_size(arrow_pos, Vec2::new(12.0, 12.0));
    let arrow_resp = ui.interact(arrow_rect, ui.id().with(label).with("arrow"), Sense::click());
    if arrow_resp.clicked() {
        *is_expanded = !*is_expanded;
    }
    icons::foldout_arrow(ui.painter(), arrow_pos, *is_expanded, Color32::from_rgb(160, 160, 160));

    // Property name
    ui.painter().text(
        Pos2::new(name_rect.min.x + 18.0, name_rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        FontId::proportional(11.5),
        STUDIO_TEXT,
    );

    // Combined summary in value column: "X, Y, Z"
    let summary = format!("{:.2}, {:.2}, {:.2}", vec3[0], vec3[1], vec3[2]);
    ui.painter().text(
        Pos2::new(val_rect.min.x + 8.0, val_rect.center().y),
        egui::Align2::LEFT_CENTER,
        summary,
        FontId::proportional(11.5),
        Color32::from_rgb(210, 210, 210),
    );

    // Vertical line between columns
    ui.painter().line_segment(
        [Pos2::new(name_rect.max.x, row_rect.min.y), Pos2::new(name_rect.max.x, row_rect.max.y)],
        Stroke::new(1.0, STUDIO_BORDER),
    );

    // Horizontal bottom line
    ui.painter().line_segment(
        [Pos2::new(row_rect.min.x, row_rect.max.y), Pos2::new(row_rect.max.x, row_rect.max.y)],
        Stroke::new(1.0, STUDIO_ROW_BORDER),
    );

    // Sub-properties if expanded: X, Y, Z
    if *is_expanded {
        draw_sub_axis_row(ui, name_w, "X", &mut vec3[0], speed);
        draw_sub_axis_row(ui, name_w, "Y", &mut vec3[1], speed);
        draw_sub_axis_row(ui, name_w, "Z", &mut vec3[2], speed);
    }
}

fn draw_sub_axis_row(ui: &mut egui::Ui, name_w: f32, axis: &str, val: &mut f32, speed: f64) {
    let row_h = 20.0;
    let avail_w = ui.available_width();
    let (row_rect, row_resp) = ui.allocate_exact_size(Vec2::new(avail_w, row_h), Sense::hover());

    let name_rect = Rect::from_min_max(row_rect.min, Pos2::new(row_rect.min.x + name_w, row_rect.max.y));
    let val_rect = Rect::from_min_max(Pos2::new(row_rect.min.x + name_w, row_rect.min.y), row_rect.max);

    if row_resp.hovered() {
        ui.painter().rect_filled(row_rect, 0.0, STUDIO_HOVER);
    }

    // Indented sub-axis label
    ui.painter().text(
        Pos2::new(name_rect.min.x + 28.0, name_rect.center().y),
        egui::Align2::LEFT_CENTER,
        axis,
        FontId::proportional(11.0),
        Color32::from_rgb(180, 180, 180),
    );

    // Vertical line
    ui.painter().line_segment(
        [Pos2::new(name_rect.max.x, row_rect.min.y), Pos2::new(name_rect.max.x, row_rect.max.y)],
        Stroke::new(1.0, STUDIO_BORDER),
    );

    // DragValue for axis
    let edit_rect = Rect::from_min_max(
        Pos2::new(val_rect.min.x + 4.0, val_rect.min.y + 1.0),
        Pos2::new(val_rect.max.x - 4.0, val_rect.max.y - 1.0),
    );
    let mut child = ui.child_ui(edit_rect, egui::Layout::left_to_right(egui::Align::Center), None);
    child.add(
        egui::DragValue::new(val)
            .speed(speed)
            .max_decimals(3),
    );

    // Horizontal bottom line
    ui.painter().line_segment(
        [Pos2::new(row_rect.min.x, row_rect.max.y), Pos2::new(row_rect.max.x, row_rect.max.y)],
        Stroke::new(1.0, STUDIO_ROW_BORDER),
    );
}
