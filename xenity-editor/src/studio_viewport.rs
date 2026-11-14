use eframe::egui::{self, Color32, CursorIcon, FontId, Pos2, Rect, Rounding, Sense, Stroke, Vec2};
use crate::studio_style::*;
use crate::icons;

pub struct StudioViewportState {
    pub camera_pitch: f32,
    pub camera_yaw: f32,
    pub camera_distance: f32,
    pub camera_target: [f32; 3],
    pub is_dragging: bool,
}

impl Default for StudioViewportState {
    fn default() -> Self {
        Self {
            camera_pitch: 25.0,
            camera_yaw: 45.0,
            camera_distance: 30.0,
            camera_target: [0.0, 1.0, 0.0],
            is_dragging: false,
        }
    }
}

impl StudioViewportState {
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        is_playing: bool,
        show_grid: bool,
        show_view_selector: bool,
    ) {
        // 1. PESTAÑA SUPERIOR DE DOCUMENTO VIEWPORT [ Place1* ✕ ]
        let doc_bar_h = 26.0;
        let (doc_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), doc_bar_h), Sense::hover());
        ui.painter().rect_filled(doc_rect, 0.0, STUDIO_BG);

        let active_tab_rect = Rect::from_min_size(doc_rect.min, Vec2::new(100.0, doc_bar_h));
        ui.painter().rect_filled(active_tab_rect, Rounding { nw: 3.0, ne: 3.0, sw: 0.0, se: 0.0 }, STUDIO_VIEWPORT_BG);
        ui.painter().line_segment(
            [Pos2::new(active_tab_rect.min.x, active_tab_rect.min.y), Pos2::new(active_tab_rect.max.x, active_tab_rect.min.y)],
            Stroke::new(2.0, STUDIO_SELECTION),
        );

        // Icono de documento o globo terráqueo
        icons::roblox_workspace(ui.painter(), Pos2::new(active_tab_rect.min.x + 14.0, active_tab_rect.center().y), 14.0);

        ui.painter().text(
            Pos2::new(active_tab_rect.min.x + 28.0, active_tab_rect.center().y),
            egui::Align2::LEFT_CENTER,
            "Place1*",
            FontId::proportional(11.5),
            Color32::WHITE,
        );

        // Botón cerrar ✕
        let close_center = Pos2::new(active_tab_rect.max.x - 12.0, active_tab_rect.center().y);
        ui.painter().text(close_center, egui::Align2::CENTER_CENTER, "✕", FontId::proportional(9.0), STUDIO_TEXT_MUTED);

        ui.painter().line_segment(
            [Pos2::new(doc_rect.min.x, doc_rect.max.y), Pos2::new(doc_rect.max.x, doc_rect.max.y)],
            Stroke::new(1.0, STUDIO_DARK_BORDER),
        );

        // 2. VIEWPORT 3D ÁREA CENTRAL
        let (vp_rect, vp_resp) = ui.allocate_exact_size(ui.available_size(), Sense::click_and_drag());

        // Manejo de órbita de cámara con clic derecho o arrastre
        if vp_resp.dragged() {
            let delta = vp_resp.drag_delta();
            self.camera_yaw += delta.x * 0.4;
            self.camera_pitch = (self.camera_pitch + delta.y * 0.4).clamp(-85.0, 85.0);
        }

        let vp_bg = if is_playing {
            Color32::from_rgb(20, 20, 24)
        } else {
            STUDIO_VIEWPORT_BG
        };
        ui.painter().rect_filled(vp_rect, 0.0, vp_bg);

        // 3. SUELO BASEPLATE Y CUADRÍCULA DE STUDS (Roblox Studio Studs Grid)
        if show_grid {
            draw_roblox_studs_grid(ui.painter(), vp_rect, self.camera_yaw, self.camera_pitch);
        }

        // 4. MODELO DE JUGADOR / ENTIDADES 3D EN EL VIEWPORT
        draw_viewport_scene_objects(ui.painter(), vp_rect, is_playing);

        // 5. VIEW SELECTOR (3D Orientation Cube en la esquina superior derecha)
        if show_view_selector {
            let cube_size = 64.0;
            let cube_rect = Rect::from_min_size(
                Pos2::new(vp_rect.max.x - cube_size - 18.0, vp_rect.min.y + 14.0),
                Vec2::new(cube_size, cube_size),
            );
            draw_view_selector_cube(ui, cube_rect, &mut self.camera_pitch, &mut self.camera_yaw);
        }

        // 6. OVERLAYS DE ESTADO Y ESTADÍSTICAS EN EL VIEWPORT (FPS, Modo)
        let stats_rect = Rect::from_min_size(Pos2::new(vp_rect.min.x + 12.0, vp_rect.min.y + 12.0), Vec2::new(180.0, 22.0));
        ui.painter().rect_filled(stats_rect, Rounding::same(3.0), Color32::from_rgba_unmultiplied(20, 20, 24, 180));
        let stats_text = if is_playing {
            "🔴 Running (60.0 FPS | Luau)"
        } else {
            "Edit Mode | 60.0 FPS | Baseplate"
        };
        ui.painter().text(
            Pos2::new(stats_rect.min.x + 8.0, stats_rect.center().y),
            egui::Align2::LEFT_CENTER,
            stats_text,
            FontId::proportional(11.0),
            if is_playing { Color32::from_rgb(255, 120, 120) } else { Color32::from_rgb(180, 220, 255) },
        );
    }
}

// =========================================================================
// RENDERIZADO DE LA CUADRÍCULA DE STUDS DE ROBLOX STUDIO
// =========================================================================

fn draw_roblox_studs_grid(painter: &egui::Painter, vp: Rect, yaw: f32, pitch: f32) {
    let center_y = vp.center().y + 50.0;
    let horizon_y = center_y - 120.0;

    // Líneas menores (1 stud)
    let minor_stroke = Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 8));
    // Líneas mayores (4 studs)
    let major_stroke = Stroke::new(1.2, Color32::from_rgba_unmultiplied(255, 255, 255, 22));

    // Cuadrícula en perspectiva simulada
    let num_lines = 16;
    for i in -num_lines..=num_lines {
        let is_major = i % 4 == 0;
        let stroke = if is_major { major_stroke } else { minor_stroke };

        // Líneas horizontales del suelo
        let t = (i as f32) / (num_lines as f32);
        let y = center_y + t * 140.0;

        if y > horizon_y && y < vp.max.y {
            painter.line_segment([Pos2::new(vp.min.x, y), Pos2::new(vp.max.x, y)], stroke);
        }

        // Líneas de fuga longitudinales
        let spread_x = (i as f32) * 36.0;
        let p_bottom = Pos2::new(vp.center().x + spread_x * 1.8, vp.max.y);
        let p_top = Pos2::new(vp.center().x + spread_x * 0.3, horizon_y);
        painter.line_segment([p_bottom, p_top], stroke);
    }

    // Ejes cartesianos en el origen
    let origin = Pos2::new(vp.center().x, center_y);
    // Eje X rojo (+X)
    painter.line_segment([origin, Pos2::new(origin.x + 35.0, origin.y + 6.0)], Stroke::new(2.0, Color32::from_rgb(230, 50, 50)));
    // Eje Y verde (+Y vertical)
    painter.line_segment([origin, Pos2::new(origin.x, origin.y - 35.0)], Stroke::new(2.0, Color32::from_rgb(50, 210, 50)));
    // Eje Z azul (+Z)
    painter.line_segment([origin, Pos2::new(origin.x - 25.0, origin.y + 16.0)], Stroke::new(2.0, Color32::from_rgb(50, 130, 230)));
}

// =========================================================================
// OBJETOS DEL MUNDO 3D (AVATAR R15 / BLOQUES)
// =========================================================================

fn draw_viewport_scene_objects(painter: &egui::Painter, vp: Rect, is_playing: bool) {
    let center = Pos2::new(vp.center().x, vp.center().y + 30.0);

    // Cubo de selección azul en el origen
    let box_s = 28.0;
    let b_rect = Rect::from_center_size(center, Vec2::new(box_s, box_s * 1.4));

    // Bounding Box punteada de selección de Studio
    painter.rect_stroke(
        b_rect,
        Rounding::same(1.0),
        Stroke::new(1.5, STUDIO_ACCENT_BLUE),
    );

    // Esquinas de selección
    for c in [b_rect.left_top(), b_rect.right_top(), b_rect.left_bottom(), b_rect.right_bottom()] {
        painter.rect_filled(Rect::from_center_size(c, Vec2::new(4.0, 4.0)), 0.5, STUDIO_ACCENT_BLUE);
    }

    // Representación isométrica del Player
    let player_color = if is_playing { Color32::from_rgb(0, 162, 255) } else { Color32::from_rgb(163, 162, 165) };
    painter.rect_filled(
        Rect::from_center_size(Pos2::new(center.x, center.y - 4.0), Vec2::new(18.0, 22.0)),
        2.0,
        player_color,
    );
    // Cabeza
    painter.rect_filled(
        Rect::from_center_size(Pos2::new(center.x, center.y - 20.0), Vec2::new(10.0, 10.0)),
        1.5,
        Color32::from_rgb(245, 205, 47), // Classic Roblox Yellow Head
    );

    // Texto de la etiqueta de nombre flotante
    painter.text(
        Pos2::new(center.x, center.y - 32.0),
        egui::Align2::CENTER_CENTER,
        "Player",
        FontId::proportional(11.0),
        Color32::WHITE,
    );
}

// =========================================================================
// VIEW SELECTOR (CUBO DE ORIENTACIÓN 3D DE ROBLOX STUDIO)
// =========================================================================

fn draw_view_selector_cube(ui: &mut egui::Ui, rect: Rect, pitch: &mut f32, yaw: &mut f32) {
    let center = rect.center();
    let r = rect.width() * 0.44;

    let cube_resp = ui.interact(rect, ui.id().with("view_selector_cube"), Sense::click());
    if cube_resp.clicked() {
        // Clic en el cubo resetea a vista frontal isométrica estándar
        *pitch = 25.0;
        *yaw = 45.0;
    }

    // Fondo circular sutil del View Selector
    ui.painter().circle_filled(center, r * 1.15, Color32::from_rgba_unmultiplied(35, 35, 40, 210));
    ui.painter().circle_stroke(center, r * 1.15, Stroke::new(1.0, Color32::from_rgb(60, 60, 65)));

    // Caras del cubo isométrico
    let top_y = center.y - r * 0.7;
    let bot_y = center.y + r * 0.7;
    let left_x = center.x - r * 0.75;
    let right_x = center.x + r * 0.75;

    let p_center = Pos2::new(center.x, center.y);
    let p_top = Pos2::new(center.x, top_y);
    let p_bottom = Pos2::new(center.x, bot_y);
    let p_left = Pos2::new(left_x, center.y - r * 0.35);
    let p_right = Pos2::new(right_x, center.y - r * 0.35);
    let p_bl = Pos2::new(left_x, center.y + r * 0.35);
    let p_br = Pos2::new(right_x, center.y + r * 0.35);

    let edge = Stroke::new(1.0, Color32::from_rgb(80, 80, 85));

    // Cara TOP
    ui.painter().add(egui::Shape::convex_polygon(
        vec![p_center, p_top, p_right, p_center],
        Color32::from_rgb(65, 65, 72),
        edge,
    ));
    ui.painter().text(
        Pos2::new(center.x + r * 0.25, center.y - r * 0.35),
        egui::Align2::CENTER_CENTER,
        "TOP",
        FontId::proportional(8.5),
        Color32::WHITE,
    );

    // Cara FRONT
    ui.painter().add(egui::Shape::convex_polygon(
        vec![p_center, p_top, p_left, p_bl],
        Color32::from_rgb(50, 50, 56),
        edge,
    ));
    ui.painter().text(
        Pos2::new(center.x - r * 0.28, center.y),
        egui::Align2::CENTER_CENTER,
        "FRONT",
        FontId::proportional(7.5),
        Color32::WHITE,
    );

    // Cara RIGHT
    ui.painter().add(egui::Shape::convex_polygon(
        vec![p_center, p_bl, p_bottom, p_br],
        Color32::from_rgb(40, 40, 45),
        edge,
    ));
    ui.painter().text(
        Pos2::new(center.x + r * 0.22, center.y + r * 0.22),
        egui::Align2::CENTER_CENTER,
        "RIGHT",
        FontId::proportional(8.0),
        Color32::WHITE,
    );
}
