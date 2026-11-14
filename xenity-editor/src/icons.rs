use eframe::egui::{self, Color32, FontId, Painter, Pos2, Rect, Rounding, Stroke, Vec2};

// =========================================================================
// RUNTIME CONTROLS
// =========================================================================

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

// =========================================================================
// NAVEGACIÓN Y FLECHAS DE ÁRBOL
// =========================================================================

pub fn foldout_arrow(painter: &Painter, center: Pos2, is_expanded: bool, color: Color32) {
    let s = 3.2;
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

// =========================================================================
// ICONOS DE CLASE OFICIALES DE ROBLOX STUDIO (Explorer ClassIcons)
// =========================================================================

/// 🌍 Workspace: Globo terráqueo azul con continentes verdes y meridianos
pub fn roblox_workspace(painter: &Painter, center: Pos2, size: f32) {
    let r = size * 0.46;
    // Océano azul profundo
    painter.circle_filled(center, r, Color32::from_rgb(38, 128, 235));
    // Mancha de continente verde
    painter.circle_filled(Pos2::new(center.x - r * 0.25, center.y - r * 0.2), r * 0.5, Color32::from_rgb(72, 190, 105));
    painter.circle_filled(Pos2::new(center.x + r * 0.35, center.y + r * 0.25), r * 0.4, Color32::from_rgb(72, 190, 105));
    // Anillo exterior y ecuador sutil
    painter.circle_stroke(center, r, Stroke::new(1.0, Color32::from_rgb(20, 80, 160)));
    painter.line_segment(
        [Pos2::new(center.x - r * 0.85, center.y), Pos2::new(center.x + r * 0.85, center.y)],
        Stroke::new(0.8, Color32::from_rgba_unmultiplied(255, 255, 255, 120)),
    );
}

/// 📦 Model: Cubo isométrico 3D azul cian icónico de Roblox Studio
pub fn roblox_model(painter: &Painter, center: Pos2, size: f32) {
    let s = size * 0.46;
    let top = Pos2::new(center.x, center.y - s);
    let bottom = Pos2::new(center.x, center.y + s);
    let left = Pos2::new(center.x - s * 0.86, center.y - s * 0.5);
    let right = Pos2::new(center.x + s * 0.86, center.y - s * 0.5);
    let bl = Pos2::new(center.x - s * 0.86, center.y + s * 0.5);
    let br = Pos2::new(center.x + s * 0.86, center.y + s * 0.5);

    let col_top = Color32::from_rgb(70, 195, 255);    // Cara superior cian brillante
    let col_left = Color32::from_rgb(32, 145, 235);   // Cara izquierda azul medio
    let col_right = Color32::from_rgb(18, 105, 185);  // Cara derecha azul oscuro

    let edge_stroke = Stroke::new(0.9, Color32::from_rgb(12, 70, 130));
    painter.add(egui::Shape::convex_polygon(vec![center, top, right, br], col_top, edge_stroke));
    painter.add(egui::Shape::convex_polygon(vec![center, top, left, bl], col_left, edge_stroke));
    painter.add(egui::Shape::convex_polygon(vec![center, bl, bottom, br], col_right, edge_stroke));
}

/// 🧱 Part: Ladrillo gris clásico de Roblox con studs en la cara superior
pub fn roblox_part(painter: &Painter, center: Pos2, size: f32) {
    let s = size * 0.44;
    let top = Pos2::new(center.x, center.y - s);
    let bottom = Pos2::new(center.x, center.y + s);
    let left = Pos2::new(center.x - s * 0.86, center.y - s * 0.5);
    let right = Pos2::new(center.x + s * 0.86, center.y - s * 0.5);
    let bl = Pos2::new(center.x - s * 0.86, center.y + s * 0.5);
    let br = Pos2::new(center.x + s * 0.86, center.y + s * 0.5);

    let col_top = Color32::from_rgb(205, 210, 218);
    let col_left = Color32::from_rgb(155, 160, 172);
    let col_right = Color32::from_rgb(115, 120, 132);

    let edge_stroke = Stroke::new(0.9, Color32::from_rgb(80, 85, 95));
    painter.add(egui::Shape::convex_polygon(vec![center, top, right, br], col_top, edge_stroke));
    painter.add(egui::Shape::convex_polygon(vec![center, top, left, bl], col_left, edge_stroke));
    painter.add(egui::Shape::convex_polygon(vec![center, bl, bottom, br], col_right, edge_stroke));

    // Studs circulares en la cara superior
    let stud_pos1 = Pos2::new(center.x - s * 0.25, center.y - s * 0.5);
    let stud_pos2 = Pos2::new(center.x + s * 0.25, center.y - s * 0.2);
    painter.circle_filled(stud_pos1, s * 0.18, Color32::from_rgb(230, 235, 245));
    painter.circle_filled(stud_pos2, s * 0.18, Color32::from_rgb(230, 235, 245));
}

/// 💎 MeshPart: Gema/malla 3D púrpura geométrica oficial de Roblox Studio
pub fn roblox_mesh_part(painter: &Painter, center: Pos2, size: f32) {
    let s = size * 0.46;
    let top = Pos2::new(center.x, center.y - s);
    let bottom = Pos2::new(center.x, center.y + s);
    let left = Pos2::new(center.x - s * 0.86, center.y - s * 0.5);
    let right = Pos2::new(center.x + s * 0.86, center.y - s * 0.5);
    let bl = Pos2::new(center.x - s * 0.86, center.y + s * 0.5);
    let br = Pos2::new(center.x + s * 0.86, center.y + s * 0.5);

    let col_top = Color32::from_rgb(195, 125, 255);   // Púrpura claro
    let col_left = Color32::from_rgb(148, 65, 235);   // Púrpura medio
    let col_right = Color32::from_rgb(95, 28, 175);   // Púrpura oscuro

    let edge_stroke = Stroke::new(0.9, Color32::from_rgb(60, 15, 115));
    painter.add(egui::Shape::convex_polygon(vec![center, top, right, br], col_top, edge_stroke));
    painter.add(egui::Shape::convex_polygon(vec![center, top, left, bl], col_left, edge_stroke));
    painter.add(egui::Shape::convex_polygon(vec![center, bl, bottom, br], col_right, edge_stroke));
}

/// 📷 Camera: Cámara de estudio oscura con lente cian
pub fn roblox_camera(painter: &Painter, center: Pos2, size: f32) {
    let body_w = size * 0.72;
    let body_h = size * 0.52;
    let body = Rect::from_center_size(Pos2::new(center.x - 1.0, center.y + 0.8), Vec2::new(body_w, body_h));
    painter.rect_filled(body, 1.5, Color32::from_rgb(60, 65, 75));
    painter.rect_stroke(body, 1.5, Stroke::new(0.8, Color32::from_rgb(35, 40, 48)));

    // Visor superior
    let top_notch = Rect::from_center_size(Pos2::new(center.x - 2.0, center.y - body_h * 0.5), Vec2::new(size * 0.3, size * 0.18));
    painter.rect_filled(top_notch, 0.5, Color32::from_rgb(85, 90, 102));

    // Lente circular cian brillante
    let lens_pos = Pos2::new(center.x - 1.0, center.y + 0.8);
    painter.circle_filled(lens_pos, size * 0.18, Color32::from_rgb(45, 195, 245));
    painter.circle_filled(Pos2::new(lens_pos.x - 0.8, lens_pos.y - 0.8), size * 0.07, Color32::WHITE);
}

/// 💡 Lighting / Sun / Lightbulb: Bombilla/sol amarillo brillante con resplandor
pub fn roblox_light(painter: &Painter, center: Pos2, size: f32) {
    let r = size * 0.28;
    // Resplandor exterior cálido
    painter.circle_filled(center, r * 1.55, Color32::from_rgba_unmultiplied(255, 215, 60, 45));
    // Bombilla central dorada
    painter.circle_filled(center, r, Color32::from_rgb(255, 210, 45));
    painter.circle_stroke(center, r, Stroke::new(1.0, Color32::from_rgb(220, 160, 20)));

    // Destellos cardinales
    let ray_len = size * 0.44;
    let stroke = Stroke::new(1.2, Color32::from_rgb(255, 215, 65));
    for (dx, dy) in [(0.0, -ray_len), (0.0, ray_len), (-ray_len, 0.0), (ray_len, 0.0)] {
        painter.line_segment(
            [Pos2::new(center.x + dx * 0.65, center.y + dy * 0.65), Pos2::new(center.x + dx, center.y + dy)],
            stroke,
        );
    }
}

/// ⛰️ Terrain: Bloque voxel con hierba verde brillante arriba y roca marrón abajo
pub fn roblox_terrain(painter: &Painter, center: Pos2, size: f32) {
    let s = size * 0.44;
    let top = Pos2::new(center.x, center.y - s);
    let bottom = Pos2::new(center.x, center.y + s);
    let left = Pos2::new(center.x - s * 0.86, center.y - s * 0.5);
    let right = Pos2::new(center.x + s * 0.86, center.y - s * 0.5);
    let bl = Pos2::new(center.x - s * 0.86, center.y + s * 0.5);
    let br = Pos2::new(center.x + s * 0.86, center.y + s * 0.5);

    // Cara superior: Hierba verde brillante
    let col_grass = Color32::from_rgb(75, 200, 75);
    // Caras laterales: Tierra/roca marrón
    let col_soil_l = Color32::from_rgb(145, 95, 55);
    let col_soil_r = Color32::from_rgb(105, 65, 35);

    let edge_stroke = Stroke::new(0.9, Color32::from_black_alpha(120));
    painter.add(egui::Shape::convex_polygon(vec![center, top, right, br], col_grass, edge_stroke));
    painter.add(egui::Shape::convex_polygon(vec![center, top, left, bl], col_soil_l, edge_stroke));
    painter.add(egui::Shape::convex_polygon(vec![center, bl, bottom, br], col_soil_r, edge_stroke));
}

/// 📜 Script: Documento de código blanco con líneas de código azules
pub fn roblox_script(painter: &Painter, center: Pos2, size: f32) {
    let w = size * 0.64;
    let h = size * 0.82;
    let rect = Rect::from_center_size(center, Vec2::new(w, h));
    painter.rect_filled(rect, 1.5, Color32::from_rgb(245, 245, 248));
    painter.rect_stroke(rect, 1.5, Stroke::new(0.9, Color32::from_rgb(180, 185, 195)));

    // Líneas de código coloreadas
    let line_col1 = Color32::from_rgb(45, 125, 225);
    let line_col2 = Color32::from_rgb(105, 110, 125);
    let start_x = rect.min.x + 2.5;
    for (dy, col, len) in [(3.0, line_col1, w * 0.6), (6.0, line_col2, w * 0.72), (9.0, line_col1, w * 0.45)] {
        painter.line_segment(
            [Pos2::new(start_x, rect.min.y + dy), Pos2::new(start_x + len, rect.min.y + dy)],
            Stroke::new(1.1, col),
        );
    }
}

/// 🔊 Sound: Altavoz azul con ondas acústicas
pub fn roblox_sound(painter: &Painter, center: Pos2, size: f32) {
    let stroke = Stroke::new(1.3, Color32::from_rgb(45, 160, 245));
    let s = size * 0.32;
    // Cono de altavoz relleno
    let p_back_top = Pos2::new(center.x - s * 0.9, center.y - s * 0.5);
    let p_back_bot = Pos2::new(center.x - s * 0.9, center.y + s * 0.5);
    let p_mid_top = Pos2::new(center.x - s * 0.2, center.y - s * 0.5);
    let p_mid_bot = Pos2::new(center.x - s * 0.2, center.y + s * 0.5);
    let p_front_top = Pos2::new(center.x + s * 0.4, center.y - s);
    let p_front_bot = Pos2::new(center.x + s * 0.4, center.y + s);

    painter.add(egui::Shape::convex_polygon(
        vec![p_back_top, p_mid_top, p_front_top, p_front_bot, p_mid_bot, p_back_bot],
        Color32::from_rgb(45, 160, 245),
        Stroke::NONE,
    ));

    // Ondas sonoras
    painter.line_segment([Pos2::new(center.x + s * 0.8, center.y - s * 0.6), Pos2::new(center.x + s * 0.8, center.y + s * 0.6)], stroke);
    painter.line_segment([Pos2::new(center.x + s * 1.25, center.y - s * 0.9), Pos2::new(center.x + s * 1.25, center.y + s * 0.9)], stroke);
}

/// 👤 Humanoid: Avatar humanoide con cabeza y torso
pub fn roblox_humanoid(painter: &Painter, center: Pos2, size: f32) {
    // Cabeza
    painter.circle_filled(Pos2::new(center.x, center.y - size * 0.24), size * 0.18, Color32::from_rgb(115, 205, 255));
    // Torso y hombros
    let torso_rect = Rect::from_center_size(Pos2::new(center.x, center.y + size * 0.2), Vec2::new(size * 0.68, size * 0.42));
    painter.rect_filled(torso_rect, 1.5, Color32::from_rgb(70, 165, 240));
}

/// 📁 Folder: Carpeta dorada icónica de Roblox Studio
pub fn roblox_folder(painter: &Painter, center: Pos2, size: f32) {
    let w = size * 0.82;
    let h = size * 0.62;
    let rect = Rect::from_center_size(center, Vec2::new(w, h));
    let tab_w = w * 0.45;
    let tab_h = h * 0.32;

    let col_tab = Color32::from_rgb(220, 175, 45);
    let col_body = Color32::from_rgb(245, 195, 60);

    // Pestaña
    let tab_rect = Rect::from_min_max(rect.min, Pos2::new(rect.min.x + tab_w, rect.min.y + tab_h));
    painter.rect_filled(tab_rect, 1.0, col_tab);
    // Cuerpo
    let body_rect = Rect::from_min_max(Pos2::new(rect.min.x, rect.min.y + tab_h * 0.6), rect.max);
    painter.rect_filled(body_rect, 1.5, col_body);
    painter.rect_stroke(body_rect, 1.5, Stroke::new(0.8, Color32::from_rgb(190, 145, 25)));
}

// =========================================================================
// UTILIDADES VECTORIALES COMPARTIDAS
// =========================================================================

pub fn magnifying_glass(painter: &Painter, center: Pos2, radius: f32, color: Color32) {
    let stroke = Stroke::new(1.3, color);
    let c = Pos2::new(center.x - 1.0, center.y - 1.0);
    painter.circle_stroke(c, radius, stroke);
    let p1 = Pos2::new(c.x + radius * 0.707, c.y + radius * 0.707);
    let p2 = Pos2::new(center.x + radius * 1.5, center.y + radius * 1.5);
    painter.line_segment([p1, p2], stroke);
}

pub fn eye_icon(painter: &Painter, center: Pos2, is_visible: bool, color: Color32) {
    let stroke = Stroke::new(1.1, color);
    let w = 5.0;
    let h = 2.8;

    let p_left = Pos2::new(center.x - w, center.y);
    let p_right = Pos2::new(center.x + w, center.y);
    let p_top = Pos2::new(center.x, center.y - h);
    let p_bot = Pos2::new(center.x, center.y + h);

    painter.line_segment([p_left, p_top], stroke);
    painter.line_segment([p_top, p_right], stroke);
    painter.line_segment([p_left, p_bot], stroke);
    painter.line_segment([p_bot, p_right], stroke);

    if is_visible {
        painter.circle_filled(center, 1.4, color);
    } else {
        painter.line_segment(
            [Pos2::new(center.x - w * 0.9, center.y - h * 1.2), Pos2::new(center.x + w * 0.9, center.y + h * 1.2)],
            Stroke::new(1.3, color),
        );
    }
}

pub fn padlock(painter: &Painter, center: Pos2, is_locked: bool, color: Color32) {
    let stroke = Stroke::new(1.1, color);
    let body_rect = Rect::from_center_size(Pos2::new(center.x, center.y + 1.5), Vec2::new(7.0, 5.5));
    painter.rect_filled(body_rect, 1.0, color);

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

pub fn kebab_menu(painter: &Painter, center: Pos2, color: Color32) {
    for dy in [-3.2, 0.0, 3.2] {
        painter.circle_filled(Pos2::new(center.x, center.y + dy), 1.1, color);
    }
}

pub fn solid_folder(painter: &Painter, rect: Rect, color: Color32) {
    roblox_folder(painter, rect.center(), rect.width());
}

pub fn material_ball(painter: &Painter, center: Pos2, radius: f32, base_color: Color32) {
    painter.circle_filled(center, radius, base_color);
    painter.circle_stroke(center, radius, Stroke::new(1.0, Color32::from_black_alpha(80)));
    let spec_pos = Pos2::new(center.x - radius * 0.32, center.y - radius * 0.32);
    let spec_r = radius * 0.35;
    painter.circle_filled(spec_pos, spec_r, Color32::from_white_alpha(150));
    painter.circle_filled(spec_pos, spec_r * 0.45, Color32::from_white_alpha(220));
}

pub fn script_file(painter: &Painter, rect: Rect) {
    roblox_script(painter, rect.center(), rect.width());
}

pub fn shaded_model(painter: &Painter, rect: Rect, _base_color: Color32) {
    roblox_model(painter, rect.center(), rect.width());
}

pub fn cube(painter: &Painter, rect: Rect, _color: Color32) {
    roblox_part(painter, rect.center(), rect.width());
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

pub fn scene_clapper(painter: &Painter, center: Pos2, size: f32, color: Color32) {
    roblox_workspace(painter, center, size);
}

pub fn scene_icon(painter: &Painter, rect: Rect) {
    roblox_workspace(painter, rect.center(), rect.width());
}

pub fn audio_icon(painter: &Painter, center: Pos2, size: f32, _color: Color32) {
    roblox_sound(painter, center, size);
}

// =========================================================================
// ICONOS DE LA BARRA DE HERRAMIENTAS RIBBON DE ROBLOX STUDIO
// =========================================================================

/// Herramienta SELECT (Cursor flecha clásica)
pub fn ribbon_select(painter: &Painter, center: Pos2, size: f32, color: Color32) {
    let s = size * 0.45;
    let p1 = Pos2::new(center.x - s * 0.7, center.y - s);
    let p2 = Pos2::new(center.x - s * 0.7, center.y + s * 0.8);
    let p3 = Pos2::new(center.x - s * 0.1, center.y + s * 0.3);
    let p4 = Pos2::new(center.x + s * 0.6, center.y + s * 0.8);
    let p5 = Pos2::new(center.x + s * 0.8, center.y + s * 0.5);
    let p6 = Pos2::new(center.x + s * 0.1, center.y);
    let p7 = Pos2::new(center.x + s * 0.6, center.y - s * 0.1);
    painter.add(egui::Shape::convex_polygon(vec![p1, p2, p3, p4, p5, p6, p7], color, Stroke::new(1.0, Color32::from_rgb(20, 20, 20))));
}

/// Herramienta MOVE (4 flechas cardinales)
pub fn ribbon_move(painter: &Painter, center: Pos2, size: f32, color: Color32) {
    let s = size * 0.42;
    let stroke = Stroke::new(1.8, color);
    // Cruz central
    painter.line_segment([Pos2::new(center.x - s, center.y), Pos2::new(center.x + s, center.y)], stroke);
    painter.line_segment([Pos2::new(center.x, center.y - s), Pos2::new(center.x, center.y + s)], stroke);
    // Puntas de flecha
    let head = s * 0.4;
    // Izquierda
    painter.line_segment([Pos2::new(center.x - s + head, center.y - head), Pos2::new(center.x - s, center.y)], stroke);
    painter.line_segment([Pos2::new(center.x - s + head, center.y + head), Pos2::new(center.x - s, center.y)], stroke);
    // Derecha
    painter.line_segment([Pos2::new(center.x + s - head, center.y - head), Pos2::new(center.x + s, center.y)], stroke);
    painter.line_segment([Pos2::new(center.x + s - head, center.y + head), Pos2::new(center.x + s, center.y)], stroke);
    // Arriba
    painter.line_segment([Pos2::new(center.x - head, center.y - s + head), Pos2::new(center.x, center.y - s)], stroke);
    painter.line_segment([Pos2::new(center.x + head, center.y - s + head), Pos2::new(center.x, center.y - s)], stroke);
    // Abajo
    painter.line_segment([Pos2::new(center.x - head, center.y + s - head), Pos2::new(center.x, center.y + s)], stroke);
    painter.line_segment([Pos2::new(center.x + head, center.y + s - head), Pos2::new(center.x, center.y + s)], stroke);
}

/// Herramienta ROTATE (Círculo de rotación con flecha)
pub fn ribbon_rotate(painter: &Painter, center: Pos2, size: f32, color: Color32) {
    let r = size * 0.38;
    painter.circle_stroke(center, r, Stroke::new(1.8, color));
    // Flecha indicadora de giro
    let head_pos = Pos2::new(center.x + r * 0.8, center.y - r * 0.5);
    let p1 = Pos2::new(head_pos.x - 3.0, head_pos.y - 4.0);
    let p2 = Pos2::new(head_pos.x + 3.0, head_pos.y - 1.0);
    let p3 = Pos2::new(head_pos.x - 1.0, head_pos.y + 4.0);
    painter.add(egui::Shape::convex_polygon(vec![p1, p2, p3], color, Stroke::NONE));
}

/// Herramienta SCALE (Cubo con caja delimitadora de escalado)
pub fn ribbon_scale(painter: &Painter, center: Pos2, size: f32, color: Color32) {
    let s = size * 0.36;
    let rect = Rect::from_center_size(center, Vec2::new(s * 1.5, s * 1.5));
    painter.rect_stroke(rect, Rounding::same(1.0), Stroke::new(1.4, color));
    // Puntos de escala en las esquinas
    let handle_s = 3.5;
    for corner in [rect.left_top(), rect.right_top(), rect.left_bottom(), rect.right_bottom()] {
        painter.rect_filled(Rect::from_center_size(corner, Vec2::new(handle_s, handle_s)), 0.5, Color32::from_rgb(0, 162, 255));
    }
}

/// Herramienta PART (Bloque isométrico con stud circular)
pub fn ribbon_part(painter: &Painter, center: Pos2, size: f32) {
    roblox_part(painter, center, size);
}

/// Herramienta TERRAIN (Montañas y colinas)
pub fn ribbon_terrain(painter: &Painter, center: Pos2, size: f32) {
    roblox_terrain(painter, center, size);
}

/// Herramienta TOOLBOX (Caja de herramientas naranja de Studio)
pub fn ribbon_toolbox(painter: &Painter, center: Pos2, size: f32) {
    let s = size * 0.42;
    // Caja naranja
    let box_rect = Rect::from_min_max(Pos2::new(center.x - s, center.y - s * 0.4), Pos2::new(center.x + s, center.y + s * 0.8));
    painter.rect_filled(box_rect, Rounding::same(2.0), Color32::from_rgb(235, 120, 30));
    painter.rect_stroke(box_rect, Rounding::same(2.0), Stroke::new(1.0, Color32::from_rgb(180, 80, 10)));
    // Asa superior
    let handle_rect = Rect::from_min_max(Pos2::new(center.x - s * 0.45, center.y - s * 0.8), Pos2::new(center.x + s * 0.45, center.y - s * 0.3));
    painter.rect_stroke(handle_rect, Rounding::same(2.0), Stroke::new(1.5, Color32::from_rgb(220, 220, 220)));
    // Broche metálico central
    painter.rect_filled(Rect::from_center_size(Pos2::new(center.x, center.y + s * 0.1), Vec2::new(4.0, 5.0)), 1.0, Color32::from_rgb(240, 240, 240));
}

/// Herramienta ANCHOR (Ancla náutica de anclaje de física)
pub fn ribbon_anchor(painter: &Painter, center: Pos2, size: f32, is_anchored: bool) {
    let color = if is_anchored { Color32::from_rgb(0, 162, 255) } else { Color32::from_rgb(190, 190, 190) };
    let s = size * 0.38;
    // Anillo superior
    painter.circle_stroke(Pos2::new(center.x, center.y - s * 0.7), s * 0.28, Stroke::new(1.4, color));
    // Eje vertical
    painter.line_segment([Pos2::new(center.x, center.y - s * 0.4), Pos2::new(center.x, center.y + s * 0.6)], Stroke::new(1.8, color));
    // Barra transversal
    painter.line_segment([Pos2::new(center.x - s * 0.5, center.y - s * 0.1), Pos2::new(center.x + s * 0.5, center.y - s * 0.1)], Stroke::new(1.5, color));
    // Arco inferior
    let p_left = Pos2::new(center.x - s * 0.75, center.y + s * 0.3);
    let p_mid = Pos2::new(center.x, center.y + s * 0.75);
    let p_right = Pos2::new(center.x + s * 0.75, center.y + s * 0.3);
    painter.line_segment([p_left, p_mid], Stroke::new(1.8, color));
    painter.line_segment([p_mid, p_right], Stroke::new(1.8, color));
}

/// Herramienta GAME SETTINGS (Engranaje)
pub fn ribbon_settings(painter: &Painter, center: Pos2, size: f32, color: Color32) {
    let r = size * 0.38;
    painter.circle_stroke(center, r, Stroke::new(2.2, color));
    painter.circle_filled(center, r * 0.35, Color32::from_rgb(37, 37, 38));
    for i in 0..6 {
        let angle = (i as f32) * std::f32::consts::PI / 3.0;
        let d = Pos2::new(center.x + angle.cos() * (r + 2.0), center.y + angle.sin() * (r + 2.0));
        painter.circle_filled(d, 1.8, color);
    }
}

/// Botón PLAY de simulación (Triángulo verde con fondo redondo/cuadrado)
pub fn ribbon_play(painter: &Painter, center: Pos2, size: f32) {
    let r = size * 0.45;
    painter.circle_filled(center, r, Color32::from_rgb(16, 165, 85));
    // Triángulo play blanco
    let s = r * 0.55;
    let p1 = Pos2::new(center.x - s * 0.6, center.y - s);
    let p2 = Pos2::new(center.x + s * 0.8, center.y);
    let p3 = Pos2::new(center.x - s * 0.6, center.y + s);
    painter.add(egui::Shape::convex_polygon(vec![p1, p2, p3], Color32::WHITE, Stroke::NONE));
}

/// Ícono de pestaña Explorer (dock tab)
pub fn tab_explorer_icon(painter: &Painter, center: Pos2, color: Color32) {
    let r = Rect::from_center_size(center, Vec2::new(13.0, 13.0));
    painter.rect_stroke(r, Rounding::same(1.5), Stroke::new(1.1, color));
    // Header bar
    painter.line_segment([Pos2::new(r.min.x, r.min.y + 3.5), Pos2::new(r.max.x, r.min.y + 3.5)], Stroke::new(0.9, color));
    // Line 1
    painter.circle_filled(Pos2::new(r.min.x + 3.0, r.min.y + 6.5), 0.9, color);
    painter.line_segment([Pos2::new(r.min.x + 5.5, r.min.y + 6.5), Pos2::new(r.max.x - 2.5, r.min.y + 6.5)], Stroke::new(0.9, color));
    // Line 2
    painter.circle_filled(Pos2::new(r.min.x + 3.0, r.min.y + 9.5), 0.9, color);
    painter.line_segment([Pos2::new(r.min.x + 5.5, r.min.y + 9.5), Pos2::new(r.max.x - 2.5, r.min.y + 9.5)], Stroke::new(0.9, color));
}

/// Ícono de pestaña Properties (dock tab)
pub fn tab_properties_icon(painter: &Painter, center: Pos2, color: Color32) {
    let r = Rect::from_center_size(center, Vec2::new(13.0, 13.0));
    painter.rect_stroke(r, Rounding::same(1.5), Stroke::new(1.1, color));
    // 3 bullet lines
    for i in 0..3 {
        let y = r.min.y + 3.5 + (i as f32 * 3.2);
        painter.circle_filled(Pos2::new(r.min.x + 3.0, y), 0.8, color);
        painter.line_segment([Pos2::new(r.min.x + 5.5, y), Pos2::new(r.max.x - 2.5, y)], Stroke::new(0.9, color));
    }
}

/// Botón de tres puntos verticales (Options ⋮)
pub fn three_dots_vertical(painter: &Painter, center: Pos2, color: Color32) {
    let r = 1.3;
    painter.circle_filled(Pos2::new(center.x, center.y - 4.0), r, color);
    painter.circle_filled(center, r, color);
    painter.circle_filled(Pos2::new(center.x, center.y + 4.0), r, color);
}

/// Ícono de escoba para limpiar salida (Clear output)
pub fn broom_icon(painter: &Painter, center: Pos2, color: Color32) {
    let p_top = Pos2::new(center.x + 4.0, center.y - 5.0);
    let p_mid = Pos2::new(center.x - 0.5, center.y + 0.5);
    painter.line_segment([p_top, p_mid], Stroke::new(1.5, color));

    let b1 = Pos2::new(center.x - 4.5, center.y + 1.5);
    let b2 = Pos2::new(center.x - 2.0, center.y + 5.0);
    let b3 = Pos2::new(center.x + 2.5, center.y + 2.5);
    painter.add(egui::Shape::convex_polygon(vec![p_mid, b1, b2, b3], color, Stroke::NONE));
}

/// Ícono de marcador (Bookmark ribbon)
pub fn bookmark_icon(painter: &Painter, center: Pos2, color: Color32) {
    let w = 4.0;
    let h = 5.5;
    let top_left = Pos2::new(center.x - w, center.y - h);
    let top_right = Pos2::new(center.x + w, center.y - h);
    let bot_right = Pos2::new(center.x + w, center.y + h);
    let notch = Pos2::new(center.x, center.y + h - 2.5);
    let bot_left = Pos2::new(center.x - w, center.y + h);
    painter.add(egui::Shape::convex_polygon(vec![top_left, top_right, bot_right, notch, bot_left], color, Stroke::NONE));
}

/// Ícono de reloj de historial (History clock)
pub fn clock_icon(painter: &Painter, center: Pos2, color: Color32) {
    let r = 5.5;
    painter.circle_stroke(center, r, Stroke::new(1.2, color));
    painter.line_segment([center, Pos2::new(center.x, center.y - 3.0)], Stroke::new(1.2, color));
    painter.line_segment([center, Pos2::new(center.x + 2.5, center.y)], Stroke::new(1.2, color));
}
