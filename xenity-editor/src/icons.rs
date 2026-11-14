use eframe::egui::{self, Color32, FontId, Painter, Pos2, Rect, Stroke, Vec2};

pub fn play(painter: &Painter, rect: Rect, color: Color32) {
    let center = rect.center();
    let size = rect.height() * 0.38;
    let p1 = Pos2::new(center.x - size * 0.5, center.y - size);
    let p2 = Pos2::new(center.x + size * 0.7, center.y);
    let p3 = Pos2::new(center.x - size * 0.5, center.y + size);
    painter.add(egui::Shape::convex_polygon(vec![p1, p2, p3], color, Stroke::NONE));
}

pub fn pause(painter: &Painter, rect: Rect, color: Color32) {
    let center = rect.center();
    let w = 2.5;
    let h = rect.height() * 0.65;
    let r1 = Rect::from_center_size(Pos2::new(center.x - 3.0, center.y), Vec2::new(w, h));
    let r2 = Rect::from_center_size(Pos2::new(center.x + 3.0, center.y), Vec2::new(w, h));
    painter.rect_filled(r1, 1.0, color);
    painter.rect_filled(r2, 1.0, color);
}

pub fn stop(painter: &Painter, rect: Rect, color: Color32) {
    let center = rect.center();
    let s = rect.height() * 0.58;
    painter.rect_filled(Rect::from_center_size(center, Vec2::new(s, s)), 1.5, color);
}

pub fn step(painter: &Painter, rect: Rect, color: Color32) {
    let center = rect.center();
    let s = rect.height() * 0.35;
    let p1 = Pos2::new(center.x - s * 0.4, center.y - s);
    let p2 = Pos2::new(center.x + s * 0.5, center.y);
    let p3 = Pos2::new(center.x - s * 0.4, center.y + s);
    painter.add(egui::Shape::convex_polygon(vec![p1, p2, p3], color, Stroke::NONE));
    painter.line_segment([Pos2::new(center.x + s * 0.8, center.y - s), Pos2::new(center.x + s * 0.8, center.y + s)], Stroke::new(1.5, color));
}

pub fn foldout_arrow(painter: &Painter, center: Pos2, is_expanded: bool, color: Color32) {
    let s = 3.5;
    if is_expanded {
        // Flecha hacia abajo ▼ sólida y nítida
        let p1 = Pos2::new(center.x - s, center.y - s * 0.4);
        let p2 = Pos2::new(center.x + s, center.y - s * 0.4);
        let p3 = Pos2::new(center.x, center.y + s * 0.6);
        painter.add(egui::Shape::convex_polygon(vec![p1, p2, p3], color, Stroke::NONE));
    } else {
        // Flecha hacia la derecha ▶ sólida y nítida
        let p1 = Pos2::new(center.x - s * 0.4, center.y - s);
        let p2 = Pos2::new(center.x + s * 0.6, center.y);
        let p3 = Pos2::new(center.x - s * 0.4, center.y + s);
        painter.add(egui::Shape::convex_polygon(vec![p1, p2, p3], color, Stroke::NONE));
    }
}

/// Icono oficial de Prefab de Unity: cubo sólido cian con perspectiva 3D
pub fn prefab_glyph(painter: &Painter, center: Pos2, size: f32, base_color: Color32) {
    let s = size * 0.48;
    let top = Pos2::new(center.x, center.y - s);
    let bottom = Pos2::new(center.x, center.y + s);
    let left = Pos2::new(center.x - s * 0.86, center.y - s * 0.5);
    let right = Pos2::new(center.x + s * 0.86, center.y - s * 0.5);
    let bl = Pos2::new(center.x - s * 0.86, center.y + s * 0.5);
    let br = Pos2::new(center.x + s * 0.86, center.y + s * 0.5);

    let col_top = Color32::from_rgb(
        (base_color.r() as f32 * 1.25).min(255.0) as u8,
        (base_color.g() as f32 * 1.25).min(255.0) as u8,
        (base_color.b() as f32 * 1.25).min(255.0) as u8,
    );
    let col_left = base_color;
    let col_right = Color32::from_rgb(
        (base_color.r() as f32 * 0.75) as u8,
        (base_color.g() as f32 * 0.75) as u8,
        (base_color.b() as f32 * 0.75) as u8,
    );

    let stroke = Stroke::new(0.8, Color32::from_black_alpha(120));
    painter.add(egui::Shape::convex_polygon(vec![center, top, right, br], col_top, stroke));
    painter.add(egui::Shape::convex_polygon(vec![center, top, left, bl], col_left, stroke));
    painter.add(egui::Shape::convex_polygon(vec![center, bl, bottom, br], col_right, stroke));
}

/// Icono de Escena de Unity: Clapperboard / Claqueta de cine nítida
pub fn scene_clapper(painter: &Painter, center: Pos2, size: f32, color: Color32) {
    let w = size;
    let h = size * 0.82;
    let slate_rect = Rect::from_center_size(Pos2::new(center.x, center.y + h * 0.15), Vec2::new(w, h * 0.65));
    painter.rect_filled(slate_rect, 1.0, color);

    // Barra superior de la claqueta
    let bar_rect = Rect::from_center_size(Pos2::new(center.x, center.y - h * 0.3), Vec2::new(w, h * 0.35));
    painter.rect_filled(bar_rect, 1.0, Color32::from_rgb(45, 45, 45));
    painter.rect_stroke(bar_rect, 1.0, Stroke::new(1.0, color));

    // Rayas diagonales en la barra superior
    for dx in [-3.0, 0.0, 3.0] {
        painter.line_segment(
            [Pos2::new(bar_rect.center().x + dx - 1.5, bar_rect.max.y), Pos2::new(bar_rect.center().x + dx + 1.5, bar_rect.min.y)],
            Stroke::new(1.0, color),
        );
    }
}

/// Icono vector de Ojo (Visibilidad) con modo tachado
pub fn eye_icon(painter: &Painter, center: Pos2, is_visible: bool, color: Color32) {
    let stroke = Stroke::new(1.1, color);
    let w = 5.0;
    let h = 2.8;

    // Arco superior
    let p_left = Pos2::new(center.x - w, center.y);
    let p_right = Pos2::new(center.x + w, center.y);
    let p_top = Pos2::new(center.x, center.y - h);
    let p_bot = Pos2::new(center.x, center.y + h);

    painter.line_segment([p_left, p_top], stroke);
    painter.line_segment([p_top, p_right], stroke);
    painter.line_segment([p_left, p_bot], stroke);
    painter.line_segment([p_bot, p_right], stroke);

    // Pupila
    if is_visible {
        painter.circle_filled(center, 1.4, color);
    } else {
        // Línea diagonal que tacha el ojo si está oculto
        painter.line_segment(
            [Pos2::new(center.x - w * 0.9, center.y - h * 1.2), Pos2::new(center.x + w * 0.9, center.y + h * 1.2)],
            Stroke::new(1.3, color),
        );
    }
}

/// Icono vector de Candado (Lock / Picking)
pub fn padlock(painter: &Painter, center: Pos2, is_locked: bool, color: Color32) {
    let stroke = Stroke::new(1.1, color);
    // Cuerpo del candado (rectángulo)
    let body_rect = Rect::from_center_size(Pos2::new(center.x, center.y + 1.5), Vec2::new(7.0, 5.5));
    painter.rect_filled(body_rect, 1.0, color);

    // Arco superior del candado
    let arc_top = if is_locked { center.y - 3.5 } else { center.y - 4.5 };
    let shackle_left = Pos2::new(center.x - 2.0, body_rect.min.y);
    let shackle_right = if is_locked {
        Pos2::new(center.x + 2.0, body_rect.min.y)
    } else {
        Pos2::new(center.x + 2.8, body_rect.min.y - 1.5)
    };
    painter.line_segment([shackle_left, Pos2::new(shackle_left.x, arc_top)], stroke);
    painter.line_segment([Pos2::new(shackle_left.x, arc_top), Pos2::new(shackle_right.x, arc_top)], stroke);
    painter.line_segment([Pos2::new(shackle_right.x, arc_top), shackle_right], stroke);
}

/// Menú de 3 puntos verticales `⋮` limpio
pub fn kebab_menu(painter: &Painter, center: Pos2, color: Color32) {
    for dy in [-3.2, 0.0, 3.2] {
        painter.circle_filled(Pos2::new(center.x, center.y + dy), 1.1, color);
    }
}

/// Carpeta sólida clásica de Unity
pub fn solid_folder(painter: &Painter, rect: Rect, color: Color32) {
    let tab_w = rect.width() * 0.45;
    let tab_h = rect.height() * 0.28;

    let tab_p1 = rect.min;
    let tab_p2 = Pos2::new(rect.min.x + tab_w, rect.min.y);
    let tab_p3 = Pos2::new(rect.min.x + tab_w + 3.0, rect.min.y + tab_h);
    let tab_p4 = Pos2::new(rect.min.x, rect.min.y + tab_h);
    let darker = Color32::from_rgb(
        (color.r() as f32 * 0.82) as u8,
        (color.g() as f32 * 0.82) as u8,
        (color.b() as f32 * 0.82) as u8,
    );
    painter.add(egui::Shape::convex_polygon(vec![tab_p1, tab_p2, tab_p3, tab_p4], darker, Stroke::NONE));

    let body_rect = Rect::from_min_max(
        Pos2::new(rect.min.x, rect.min.y + tab_h * 0.8),
        rect.max,
    );
    painter.rect_filled(body_rect, 1.5, color);
}

/// Esfera de material 3D con sombreado difuso y brillo especular
pub fn material_ball(painter: &Painter, center: Pos2, radius: f32, base_color: Color32) {
    painter.circle_filled(center, radius, base_color);
    painter.circle_stroke(center, radius, Stroke::new(1.0, Color32::from_black_alpha(80)));

    let spec_pos = Pos2::new(center.x - radius * 0.32, center.y - radius * 0.32);
    let spec_r = radius * 0.35;
    painter.circle_filled(spec_pos, spec_r, Color32::from_white_alpha(150));
    painter.circle_filled(spec_pos, spec_r * 0.45, Color32::from_white_alpha(220));
}

/// Documento de script de Unity / C++ con esquina doblada
pub fn script_file(painter: &Painter, rect: Rect) {
    let fold = (rect.width() * 0.32).min(8.0);
    let p_tl = rect.min;
    let p_tr_cut = Pos2::new(rect.max.x - fold, rect.min.y);
    let p_tr_fold = Pos2::new(rect.max.x, rect.min.y + fold);
    let p_br = rect.max;
    let p_bl = Pos2::new(rect.min.x, rect.max.y);

    let paper_color = Color32::from_rgb(238, 238, 238);
    painter.add(egui::Shape::convex_polygon(
        vec![p_tl, p_tr_cut, p_tr_fold, p_br, p_bl],
        paper_color,
        Stroke::new(1.0, Color32::from_rgb(180, 180, 180)),
    ));

    let p_inner = Pos2::new(rect.max.x - fold, rect.min.y + fold);
    painter.add(egui::Shape::convex_polygon(
        vec![p_tr_cut, p_inner, p_tr_fold],
        Color32::from_rgb(200, 200, 200),
        Stroke::NONE,
    ));

    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        "C++",
        FontId::monospace(rect.height() * 0.28),
        Color32::from_rgb(45, 110, 185),
    );
}

/// Modelo 3D isométrico con caras iluminadas
pub fn shaded_model(painter: &Painter, rect: Rect, base_color: Color32) {
    let c = rect.center();
    let s = rect.width() * 0.46;

    let top = Pos2::new(c.x, c.y - s);
    let bottom = Pos2::new(c.x, c.y + s);
    let left = Pos2::new(c.x - s * 0.86, c.y - s * 0.5);
    let right = Pos2::new(c.x + s * 0.86, c.y - s * 0.5);
    let bl = Pos2::new(c.x - s * 0.86, c.y + s * 0.5);
    let br = Pos2::new(c.x + s * 0.86, c.y + s * 0.5);

    let col_top = Color32::from_rgb(
        (base_color.r() as f32 * 1.15).min(255.0) as u8,
        (base_color.g() as f32 * 1.15).min(255.0) as u8,
        (base_color.b() as f32 * 1.15).min(255.0) as u8,
    );
    let col_left = base_color;
    let col_right = Color32::from_rgb(
        (base_color.r() as f32 * 0.72) as u8,
        (base_color.g() as f32 * 0.72) as u8,
        (base_color.b() as f32 * 0.72) as u8,
    );

    let edge_stroke = Stroke::new(1.0, Color32::from_black_alpha(100));

    painter.add(egui::Shape::convex_polygon(vec![c, top, right, br], col_top, edge_stroke));
    painter.add(egui::Shape::convex_polygon(vec![c, top, left, bl], col_left, edge_stroke));
    painter.add(egui::Shape::convex_polygon(vec![c, bl, bottom, br], col_right, edge_stroke));
}

pub fn cube(painter: &Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let s = rect.width() * 0.38;
    let stroke = Stroke::new(1.3, color);

    let top = Pos2::new(c.x, c.y - s);
    let bottom = Pos2::new(c.x, c.y + s);
    let left = Pos2::new(c.x - s * 0.86, c.y - s * 0.5);
    let right = Pos2::new(c.x + s * 0.86, c.y - s * 0.5);
    let bottom_left = Pos2::new(c.x - s * 0.86, c.y + s * 0.5);
    let bottom_right = Pos2::new(c.x + s * 0.86, c.y + s * 0.5);

    painter.line_segment([top, right], stroke);
    painter.line_segment([right, bottom_right], stroke);
    painter.line_segment([bottom_right, bottom], stroke);
    painter.line_segment([bottom, bottom_left], stroke);
    painter.line_segment([bottom_left, left], stroke);
    painter.line_segment([left, top], stroke);
    painter.line_segment([c, top], stroke);
    painter.line_segment([c, bottom_left], stroke);
    painter.line_segment([c, bottom_right], stroke);
}

pub fn magnifying_glass(painter: &Painter, center: Pos2, radius: f32, color: Color32) {
    let stroke = Stroke::new(1.3, color);
    let c = Pos2::new(center.x - 1.2, center.y - 1.2);
    painter.circle_stroke(c, radius, stroke);
    let p1 = Pos2::new(c.x + radius * 0.7, c.y + radius * 0.7);
    let p2 = Pos2::new(center.x + radius * 1.5, center.y + radius * 1.5);
    painter.line_segment([p1, p2], stroke);
}

pub fn star(painter: &Painter, center: Pos2, radius: f32, color: Color32) {
    let mut points = Vec::with_capacity(10);
    for i in 0..10 {
        let r = if i % 2 == 0 { radius } else { radius * 0.42 };
        let angle = (i as f32 * std::f32::consts::PI / 5.0) - (std::f32::consts::PI / 2.0);
        points.push(Pos2::new(center.x + r * angle.cos(), center.y + r * angle.sin()));
    }
    painter.add(egui::Shape::convex_polygon(points, color, Stroke::NONE));
}

pub fn list_icon(painter: &Painter, rect: Rect, color: Color32) {
    let stroke = Stroke::new(1.2, color);
    let c = rect.center();
    let w = 4.5;
    for y_off in [-3.5, 0.0, 3.5] {
        painter.line_segment([Pos2::new(c.x - w, c.y + y_off), Pos2::new(c.x + w, c.y + y_off)], stroke);
    }
}

pub fn grid_icon(painter: &Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let s = 3.0;
    let col = color;
    for (dx, dy) in [(-s, -s), (s, -s), (-s, s), (s, s)] {
        painter.rect_filled(Rect::from_center_size(Pos2::new(c.x + dx * 1.1, c.y + dy * 1.1), Vec2::new(s, s)), 0.5, col);
    }
}

pub fn target_picker(painter: &Painter, center: Pos2, radius: f32, color: Color32) {
    painter.circle_stroke(center, radius, Stroke::new(1.3, color));
    painter.circle_filled(center, radius * 0.4, color);
}

pub fn scene_icon(painter: &Painter, rect: Rect) {
    scene_clapper(painter, rect.center(), rect.width() * 0.85, Color32::from_rgb(85, 170, 245));
}

pub fn audio_icon(painter: &Painter, center: Pos2, radius: f32, color: Color32) {
    let stroke = Stroke::new(1.5, color);
    let s = radius * 0.7;
    painter.line_segment([Pos2::new(center.x - s, center.y - s * 0.5), Pos2::new(center.x - s * 0.2, center.y - s * 0.5)], stroke);
    painter.line_segment([Pos2::new(center.x - s * 0.2, center.y - s * 0.5), Pos2::new(center.x + s * 0.4, center.y - s)], stroke);
    painter.line_segment([Pos2::new(center.x + s * 0.4, center.y - s), Pos2::new(center.x + s * 0.4, center.y + s)], stroke);
    painter.line_segment([Pos2::new(center.x + s * 0.4, center.y + s), Pos2::new(center.x - s * 0.2, center.y + s * 0.5)], stroke);
    painter.line_segment([Pos2::new(center.x - s * 0.2, center.y + s * 0.5), Pos2::new(center.x - s, center.y + s * 0.5)], stroke);
    painter.line_segment([Pos2::new(center.x - s, center.y + s * 0.5), Pos2::new(center.x - s, center.y - s * 0.5)], stroke);
}
