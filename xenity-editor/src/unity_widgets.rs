use eframe::egui::{self, Color32, CursorIcon, DragValue, FontId, Pos2, Rect, Rounding, Sense, Stroke, Vec2};
use crate::icons;
use crate::unity_style::*;

/// Dibuja la etiqueta izquierda estándar de Unity con soporte de cursor ↔ al arrastrar
pub fn unity_prefix_label(ui: &mut egui::Ui, text: &str) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(LABEL_WIDTH, SINGLE_LINE_HEIGHT), Sense::hover());
    
    // Si el usuario pasa el ratón sobre la etiqueta, muestra el cursor de arrastre horizontal ↔ de Unity
    if resp.hovered() {
        ui.output_mut(|o| o.cursor_icon = CursorIcon::ResizeHorizontal);
    }

    ui.painter().text(
        Pos2::new(rect.min.x, rect.center().y),
        egui::Align2::LEFT_CENTER,
        text,
        FontId::proportional(12.0),
        TEXT_NORMAL,
    );

    resp
}

/// Implementación 1:1 de `EditorGUILayout.Vector3Field("Label", pos)` de Unity
pub fn unity_vector3(ui: &mut egui::Ui, label: &str, values: &mut [f32; 3], speed: f32) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 2.0;

        // 1. Etiqueta izquierda estándar de Unity
        unity_prefix_label(ui, label);

        let total_avail = ui.available_width();
        let axis_w = (total_avail / 3.0).max(45.0);

        let [vx, vy, vz] = values;

        let axes: [(&str, &mut f32, Color32, Color32); 3] = [
            ("X", vx, Color32::from_rgb(168, 52, 52), Color32::from_rgb(235, 120, 120)),
            ("Y", vy, Color32::from_rgb(52, 138, 58), Color32::from_rgb(110, 215, 120)),
            ("Z", vz, Color32::from_rgb(45, 100, 175), Color32::from_rgb(100, 170, 245)),
        ];

        for (name, val, tag_bg, tag_fg) in axes {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;

                // Pastilla de letra X / Y / Z compacta adosada al input
                let (tag_rect, tag_resp) = ui.allocate_exact_size(egui::vec2(16.0, 18.0), Sense::click_and_drag());
                if tag_resp.hovered() {
                    ui.output_mut(|o| o.cursor_icon = CursorIcon::ResizeHorizontal);
                }
                if tag_resp.dragged() {
                    let delta = tag_resp.drag_delta().x * speed;
                    *val += delta;
                }

                ui.painter().rect_filled(tag_rect, Rounding { nw: 2.0, sw: 2.0, se: 0.0, ne: 0.0 }, tag_bg);
                ui.painter().text(
                    tag_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    name,
                    FontId::proportional(11.0),
                    tag_fg,
                );

                // Campo numérico directo con fuente monoespaciada para evitar que baile
                let input_w = (axis_w - 16.0).max(28.0);
                ui.add_sized(
                    [input_w, 18.0],
                    DragValue::new(val)
                        .speed(speed)
                        .custom_formatter(|n, _| format!("{:.2}", n)),
                );
            });
            ui.add_space(2.0);
        }
    });
}

/// Implementación 1:1 de `EditorGUILayout.FloatField("Label", val)` de Unity
pub fn unity_float(ui: &mut egui::Ui, label: &str, value: &mut f32, speed: f32) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 2.0;
        unity_prefix_label(ui, label);

        let input_w = ui.available_width().max(60.0);
        ui.add_sized(
            [input_w, 18.0],
            DragValue::new(value)
                .speed(speed)
                .custom_formatter(|n, _| format!("{:.2}", n)),
        );
    });
}

/// Implementación 1:1 de `EditorGUILayout.Slider("Label", val, min, max)` de Unity
pub fn unity_slider(ui: &mut egui::Ui, label: &str, value: &mut f32, range: std::ops::RangeInclusive<f32>) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 2.0;
        unity_prefix_label(ui, label);

        let input_w = ui.available_width().max(60.0);
        ui.add_sized([input_w, 18.0], egui::Slider::new(value, range));
    });
}

/// Implementación 1:1 de `EditorGUILayout.Toggle("Label", bool)` de Unity
pub fn unity_checkbox(ui: &mut egui::Ui, label: &str, value: &mut bool) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 2.0;
        unity_prefix_label(ui, label);

        let (rect, resp) = ui.allocate_exact_size(egui::vec2(16.0, 16.0), Sense::click());
        if resp.clicked() {
            *value = !*value;
        }

        ui.painter().rect_filled(rect, 2.0, INPUT_BG);
        ui.painter().rect_stroke(rect, 2.0, Stroke::new(1.0, Color32::from_rgb(60, 60, 60)));
        if *value {
            // Checkmark azul Unity
            ui.painter().rect_filled(rect.shrink(3.0), 1.0, SELECTION_BLUE);
        }
    });
}

/// Implementación 1:1 de `EditorGUILayout.ObjectField("Label", obj, typeof(T))` de Unity
pub fn unity_object_field(ui: &mut egui::Ui, label: &str, asset_name: &str, type_name: &str) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 2.0;
        unity_prefix_label(ui, label);

        let field_w = ui.available_width();
        let (rect, resp) = ui.allocate_exact_size(egui::vec2(field_w, 18.0), Sense::click());

        let bg = if resp.hovered() { Color32::from_rgb(46, 46, 46) } else { INPUT_BG };
        ui.painter().rect_filled(rect, 2.0, bg);
        ui.painter().rect_stroke(rect, 2.0, Stroke::new(1.0, Color32::from_rgb(52, 52, 52)));

        // Nombre del asset seleccionado
        let text_rect = Rect::from_min_max(Pos2::new(rect.min.x + 6.0, rect.min.y), Pos2::new(rect.max.x - 20.0, rect.max.y));
        ui.painter().text(
            Pos2::new(text_rect.min.x, text_rect.center().y),
            egui::Align2::LEFT_CENTER,
            format!("{} ({})", asset_name, type_name),
            FontId::proportional(11.5),
            TEXT_NORMAL,
        );

        // Botón circular Object Picker `⊙` a la derecha
        let picker_center = Pos2::new(rect.max.x - 10.0, rect.center().y);
        let picker_rect = Rect::from_center_size(picker_center, Vec2::new(16.0, 16.0));
        let picker_resp = ui.interact(picker_rect, ui.id().with(label), Sense::click());
        let picker_color = if picker_resp.hovered() { TEXT_WHITE } else { TEXT_MUTED };
        icons::target_picker(ui.painter(), picker_center, 5.0, picker_color);
    });
}

/// Cabecera de Componente Unity idéntica a `EditorGUILayout.InspectorTitlebar`
pub fn unity_component_header(ui: &mut egui::Ui, title: &str, is_expanded: &mut bool, enabled: Option<&mut bool>) -> bool {
    let avail_w = ui.available_width();
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(avail_w, 22.0), Sense::click());

    let header_bg = if resp.hovered() { Color32::from_rgb(66, 66, 66) } else { Color32::from_rgb(52, 52, 52) };
    ui.painter().rect_filled(rect, 1.0, header_bg);
    ui.painter().line_segment(
        [Pos2::new(rect.min.x, rect.max.y), Pos2::new(rect.max.x, rect.max.y)],
        Stroke::new(1.0, BORDER_COLOR),
    );

    // Flecha foldout de Unity ▶ / ▼
    let arrow_pos = Pos2::new(rect.min.x + 10.0, rect.center().y);
    icons::foldout_arrow(ui.painter(), arrow_pos, *is_expanded, Color32::from_gray(185));

    let mut title_x = rect.min.x + 22.0;

    // Checkbox opcional de activar/desactivar componente
    if let Some(en) = enabled {
        let chk_rect = Rect::from_center_size(Pos2::new(rect.min.x + 26.0, rect.center().y), Vec2::new(12.0, 12.0));
        let chk_resp = ui.interact(chk_rect, ui.id().with(title), Sense::click());
        if chk_resp.clicked() {
            *en = !*en;
        }
        ui.painter().rect_filled(chk_rect, 1.5, INPUT_BG);
        ui.painter().rect_stroke(chk_rect, 1.5, Stroke::new(1.0, Color32::from_gray(120)));
        if *en {
            ui.painter().rect_filled(chk_rect.shrink(2.0), 1.0, SELECTION_BLUE);
        }
        title_x += 18.0;
    }

    // Título del Componente
    ui.painter().text(
        Pos2::new(title_x, rect.center().y),
        egui::Align2::LEFT_CENTER,
        title,
        FontId::proportional(12.5),
        TEXT_WHITE,
    );

    // Botón de menú contextual `⋮` a la derecha
    ui.painter().text(
        Pos2::new(rect.max.x - 10.0, rect.center().y),
        egui::Align2::CENTER_CENTER,
        "⋮",
        FontId::proportional(13.0),
        TEXT_MUTED,
    );

    if resp.clicked() {
        *is_expanded = !*is_expanded;
    }

    *is_expanded
}
