use eframe::egui::{self, Color32, CursorIcon, FontId, Pos2, Rect, Rounding, Sense, Stroke, Vec2};
use crate::studio_style::*;
use crate::studio_textures::StudioIcons;
use crate::icons;
use crate::xenity_ffi::XenityEngine;

pub struct StudioViewportState {
    pub camera_pitch: f32,
    pub camera_yaw: f32,
    pub camera_distance: f32,
    pub camera_target: [f32; 3],
    pub is_dragging: bool,
    pub viewport_texture: Option<egui::TextureHandle>,
    pub pixel_buffer: Vec<u8>,
    pub engine_rendered: bool,

    pub last_width: u32,
    pub last_height: u32,
    pub last_tool: Option<crate::studio_ribbon::StudioTool>,
    pub is_dirty: bool,
    pub is_transforming_object: bool,
}

impl Default for StudioViewportState {
    fn default() -> Self {
        Self {
            camera_pitch: 25.0,
            camera_yaw: 45.0,
            camera_distance: 30.0,
            camera_target: [0.0, 1.0, 0.0],
            is_dragging: false,
            viewport_texture: None,
            pixel_buffer: Vec::new(),
            engine_rendered: false,
            last_width: 0,
            last_height: 0,
            last_tool: None,
            is_dirty: true,
            is_transforming_object: false,
        }
    }
}

impl StudioViewportState {
    pub fn request_render(&mut self) {
        self.is_dirty = true;
    }
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        icons: &StudioIcons,
        is_playing: bool,
        show_grid: bool,
        show_view_selector: bool,
        current_tool: crate::studio_ribbon::StudioTool,
        engine: &mut Option<XenityEngine>,
    ) -> Option<i32> {
        // 1. PESTAÑA SUPERIOR DE DOCUMENTO VIEWPORT [ Experiencia sin título ✕ ]
        let doc_bar_h = 28.0;
        let (doc_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), doc_bar_h), Sense::hover());
        ui.painter().rect_filled(doc_rect, 0.0, Color32::from_rgb(25, 26, 31)); // Fondo de barra de documentos

        let title = "Experiencia sin título";
        let font_id = FontId::proportional(12.0);
        let title_galley = ui.painter().layout_no_wrap(title.to_string(), font_id, Color32::WHITE);
        let title_w = title_galley.rect.width();

        let tab_w = 14.0 + 16.0 + 8.0 + title_w + 12.0 + 14.0 + 10.0;
        let active_tab_rect = Rect::from_min_size(doc_rect.min, Vec2::new(tab_w, doc_bar_h));
        ui.painter().rect_filled(active_tab_rect, Rounding { nw: 3.0, ne: 3.0, sw: 0.0, se: 0.0 }, Color32::from_rgb(32, 34, 39));

        // Separador sutil derecho de la pestaña
        ui.painter().line_segment(
            [Pos2::new(active_tab_rect.max.x, active_tab_rect.min.y + 4.0), Pos2::new(active_tab_rect.max.x, active_tab_rect.max.y - 2.0)],
            Stroke::new(1.0, STUDIO_BORDER),
        );

        // Icono oficial PlaceCloud.png (Documento de Place con nube de Roblox Studio)
        let doc_icon_rect = Rect::from_center_size(Pos2::new(active_tab_rect.min.x + 16.0, active_tab_rect.center().y), Vec2::new(16.0, 16.0));
        ui.painter().image(icons.doc_place_cloud.id(), doc_icon_rect, Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), Color32::WHITE);

        // Texto del título "Experiencia sin título"
        let text_pos = Pos2::new(active_tab_rect.min.x + 30.0, active_tab_rect.center().y);
        ui.painter().galley(Pos2::new(text_pos.x, text_pos.y - title_galley.rect.height() * 0.5), title_galley, Color32::WHITE);

        // Botón cerrar ✕ interactivo y dibujado con líneas nítidas
        let close_center = Pos2::new(active_tab_rect.max.x - 14.0, active_tab_rect.center().y);
        let close_rect = Rect::from_center_size(close_center, Vec2::new(16.0, 16.0));
        let close_resp = ui.interact(close_rect, ui.id().with("vp_doc_close"), Sense::click());
        if close_resp.hovered() {
            ui.painter().rect_filled(close_rect, Rounding::same(3.0), STUDIO_HOVER);
        }
        let close_col = if close_resp.hovered() { Color32::WHITE } else { STUDIO_TEXT_MUTED };
        let cs = 3.5;
        ui.painter().line_segment([Pos2::new(close_center.x - cs, close_center.y - cs), Pos2::new(close_center.x + cs, close_center.y + cs)], Stroke::new(1.2, close_col));
        ui.painter().line_segment([Pos2::new(close_center.x - cs, close_center.y + cs), Pos2::new(close_center.x + cs, close_center.y - cs)], Stroke::new(1.2, close_col));

        // Franja inferior azul de la pestaña / barra de documento (referencia #0067FF)
        ui.painter().line_segment(
            [Pos2::new(doc_rect.min.x, doc_rect.max.y), Pos2::new(doc_rect.max.x, doc_rect.max.y)],
            Stroke::new(2.0, Color32::from_rgb(0, 103, 255)),
        );

        // 2. VIEWPORT 3D ÁREA CENTRAL
        let (vp_rect, vp_resp) = ui.allocate_exact_size(ui.available_size(), Sense::click_and_drag());
        let vp_w = (vp_rect.width().round() as u32).max(64);
        let vp_h = (vp_rect.height().round() as u32).max(64);

        self.engine_rendered = false;
        let mut picked_object_index = None;

        // Renderizado con el motor C++ Xenity
        if let Some(eng) = engine {
            if !eng.is_initialized {
                eng.init(vp_w, vp_h);
            } else {
                eng.resize(vp_w, vp_h);
            }

            // Actualizar modo de herramienta en el motor C++
            let tool_mode = match current_tool {
                crate::studio_ribbon::StudioTool::Select => 0,
                crate::studio_ribbon::StudioTool::Move => 1,
                crate::studio_ribbon::StudioTool::Rotate => 2,
                crate::studio_ribbon::StudioTool::Scale => 3,
                crate::studio_ribbon::StudioTool::Transform => 1,
            };
            eng.set_tool_mode(tool_mode);

            let mut scene_interacted = false;

            // Clic izquierdo / Inicio de arrastre en el Viewport 3D: Raycast & Selección precisa
            if vp_resp.drag_started_by(egui::PointerButton::Primary) && !ui.input(|i| i.modifiers.alt) {
                if let Some(pos) = ui.ctx().pointer_latest_pos() {
                    if vp_rect.contains(pos) {
                        let norm_x = (pos.x - vp_rect.min.x) / vp_rect.width();
                        let norm_y = (pos.y - vp_rect.min.y) / vp_rect.height();
                        let hit = eng.pick_gameobject(norm_x, norm_y);
                        if hit >= 0 {
                            picked_object_index = Some(hit);
                            self.is_transforming_object = true;
                        } else {
                            picked_object_index = Some(-1);
                            eng.select_gameobject(-1);
                            self.is_transforming_object = false;
                        }
                        scene_interacted = true;
                    }
                }
            } else if vp_resp.clicked_by(egui::PointerButton::Primary) && !ui.input(|i| i.modifiers.alt) {
                if let Some(pos) = ui.ctx().pointer_latest_pos() {
                    if vp_rect.contains(pos) {
                        let norm_x = (pos.x - vp_rect.min.x) / vp_rect.width();
                        let norm_y = (pos.y - vp_rect.min.y) / vp_rect.height();
                        let hit = eng.pick_gameobject(norm_x, norm_y);
                        picked_object_index = Some(hit);
                        scene_interacted = true;
                    }
                }
            }

            // Al soltar el clic principal, terminar la transformación
            if !ui.input(|i| i.pointer.primary_down()) {
                self.is_transforming_object = false;
            }

            // Arrastre con clic izquierdo: Transformar ÚNICAMENTE si se inició sobre un objeto válido
            if self.is_transforming_object && vp_resp.dragged_by(egui::PointerButton::Primary) && !ui.input(|i| i.modifiers.alt) {
                let delta = vp_resp.drag_delta();
                if tool_mode != 0 && (delta.x != 0.0 || delta.y != 0.0) {
                    eng.transform_selected(delta.x, delta.y, tool_mode, 0);
                    scene_interacted = true;
                }
            }

            // Controles de cámara de Studio: Arrastre con clic derecho o Alt + Clic izquierdo
            if vp_resp.dragged_by(egui::PointerButton::Secondary) || (vp_resp.dragged_by(egui::PointerButton::Primary) && ui.input(|i| i.modifiers.alt)) {
                let delta = vp_resp.drag_delta();
                if delta.x != 0.0 || delta.y != 0.0 {
                    let rot_sens = 0.28;
                    // delta.x > 0 (derecha) -> delta_yaw negativo (girar a la derecha)
                    // delta.y < 0 (arriba) -> delta_pitch positivo (mirar hacia arriba)
                    eng.camera_orbit(-delta.x * rot_sens, -delta.y * rot_sens, 0.0, 0.0, 0.0);
                    scene_interacted = true;
                }
            }

            // Movimiento WASD + Zoom rueda si el ratón está sobre el viewport
            if vp_resp.hovered() {
                let mut fwd = 0.0f32;
                let mut right = 0.0f32;
                let mut up = 0.0f32;
                
                let dt = ui.input(|i| i.unstable_dt).clamp(0.001, 0.05);
                let base_speed = 45.0 * dt;
                let speed = if ui.input(|i| i.modifiers.shift) { base_speed * 3.5 } else { base_speed };

                if ui.input(|i| i.key_down(egui::Key::W)) { fwd += speed; }
                if ui.input(|i| i.key_down(egui::Key::S)) { fwd -= speed; }
                if ui.input(|i| i.key_down(egui::Key::D)) { right += speed; }
                if ui.input(|i| i.key_down(egui::Key::A)) { right -= speed; }
                if ui.input(|i| i.key_down(egui::Key::E)) { up += speed; }
                if ui.input(|i| i.key_down(egui::Key::Q)) { up -= speed; }

                // Zoom con la rueda del ratón
                let scroll_y = ui.input(|i| i.raw_scroll_delta.y);
                if scroll_y != 0.0 {
                    let zoom_factor = if ui.input(|i| i.modifiers.shift) { 4.5 } else { 1.8 };
                    fwd += (scroll_y / 30.0).clamp(-5.0, 5.0) * zoom_factor;
                }

                if fwd != 0.0 || right != 0.0 || up != 0.0 {
                    eng.camera_orbit(0.0, 0.0, fwd, right, up);
                    scene_interacted = true;
                }
            }

            if scene_interacted {
                ui.ctx().request_repaint();
            }

            let needs_render = is_playing
                || self.is_dirty
                || scene_interacted
                || self.viewport_texture.is_none()
                || vp_w != self.last_width
                || vp_h != self.last_height
                || Some(current_tool) != self.last_tool;

            if needs_render {
                self.is_dirty = false;
                self.last_width = vp_w;
                self.last_height = vp_h;
                self.last_tool = Some(current_tool);

                // Frame Tick en el motor C++
                eng.tick();

                // Leer framebuffer RGBA del motor
                if eng.read_frame(vp_w, vp_h, &mut self.pixel_buffer) {
                    let color_image = egui::ColorImage::from_rgba_unmultiplied(
                        [vp_w as usize, vp_h as usize],
                        &self.pixel_buffer,
                    );
                    match &mut self.viewport_texture {
                        Some(tex) => tex.set(color_image, egui::TextureOptions::LINEAR),
                        None => {
                            self.viewport_texture = Some(ui.ctx().load_texture(
                                "xenity_scene_viewport",
                                color_image,
                                egui::TextureOptions::LINEAR,
                            ));
                        }
                    }
                    self.engine_rendered = true;
                }
            }
        }

        // Si el motor renderizó o ya tiene textura en caché VRAM, dibujamos la textura OpenGL (invertida en V por GPU para máxima velocidad)
        if (self.engine_rendered || self.viewport_texture.is_some()) && self.viewport_texture.is_some() {
            let tex = self.viewport_texture.as_ref().unwrap();
            ui.painter().image(
                tex.id(),
                vp_rect,
                Rect::from_min_max(Pos2::new(0.0, 1.0), Pos2::new(1.0, 0.0)),
                Color32::WHITE,
            );
        } else {
            // Fondo de reserva y cuadrícula procedimental si el motor no está activo
            let vp_bg = if is_playing {
                Color32::from_rgb(20, 20, 24)
            } else {
                STUDIO_VIEWPORT_BG
            };
            ui.painter().rect_filled(vp_rect, 0.0, vp_bg);

            if show_grid {
                draw_roblox_studs_grid(ui.painter(), vp_rect, self.camera_yaw, self.camera_pitch);
            }
            draw_viewport_scene_objects(ui.painter(), vp_rect, is_playing);
        }

        // 3. VIEW SELECTOR (3D Orientation Cube con texturas PNG oficiales de Studio)
        if show_view_selector {
            let cube_size = 64.0;
            let cube_rect = Rect::from_min_size(
                Pos2::new(vp_rect.max.x - cube_size - 18.0, vp_rect.min.y + 14.0),
                Vec2::new(cube_size, cube_size),
            );
            draw_view_selector_cube(ui, cube_rect, icons, &mut self.camera_pitch, &mut self.camera_yaw);
        }

        // 4. OVERLAYS DE ESTADO Y ESTADÍSTICAS EN EL VIEWPORT (FPS, Motor, Objetos)
        let stats_rect = Rect::from_min_size(Pos2::new(vp_rect.min.x + 12.0, vp_rect.min.y + 12.0), Vec2::new(260.0, 24.0));
        ui.painter().rect_filled(stats_rect, Rounding::same(4.0), Color32::from_rgba_unmultiplied(20, 20, 24, 210));
        ui.painter().rect_stroke(stats_rect, Rounding::same(4.0), Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 20)));

        let obj_count = engine.as_mut().map(|e| e.get_gameobject_count()).unwrap_or(0);
        let stats_text = if is_playing {
            format!("🔴 Running | Xenity C++ | {} GameObjects", obj_count)
        } else if self.engine_rendered {
            format!("Edit Mode | Xenity OpenGL 3D | {} Objects", obj_count)
        } else {
            "Edit Mode | 60.0 FPS | Baseplate".to_string()
        };

        ui.painter().text(
            Pos2::new(stats_rect.min.x + 10.0, stats_rect.center().y),
            egui::Align2::LEFT_CENTER,
            stats_text,
            FontId::proportional(11.5),
            if is_playing { Color32::from_rgb(255, 120, 120) } else { Color32::from_rgb(180, 225, 255) },
        );

        picked_object_index
    }
}

// =========================================================================
// RENDERIZADO DE LA CUADRÍCULA DE STUDS DE ROBLOX STUDIO (FALLBACK)
// =========================================================================

fn draw_roblox_studs_grid(painter: &egui::Painter, vp: Rect, _yaw: f32, _pitch: f32) {
    let center_y = vp.center().y + 50.0;
    let horizon_y = center_y - 120.0;

    let minor_stroke = Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 8));
    let major_stroke = Stroke::new(1.2, Color32::from_rgba_unmultiplied(255, 255, 255, 22));

    let num_lines = 16;
    for i in -num_lines..=num_lines {
        let is_major = i % 4 == 0;
        let stroke = if is_major { major_stroke } else { minor_stroke };

        let t = (i as f32) / (num_lines as f32);
        let y = center_y + t * 140.0;

        if y > horizon_y && y < vp.max.y {
            painter.line_segment([Pos2::new(vp.min.x, y), Pos2::new(vp.max.x, y)], stroke);
        }

        let spread_x = (i as f32) * 36.0;
        let p_bottom = Pos2::new(vp.center().x + spread_x * 1.8, vp.max.y);
        let p_top = Pos2::new(vp.center().x + spread_x * 0.3, horizon_y);
        painter.line_segment([p_bottom, p_top], stroke);
    }

    let origin = Pos2::new(vp.center().x, center_y);
    painter.line_segment([origin, Pos2::new(origin.x + 35.0, origin.y + 6.0)], Stroke::new(2.0, Color32::from_rgb(230, 50, 50)));
    painter.line_segment([origin, Pos2::new(origin.x, origin.y - 35.0)], Stroke::new(2.0, Color32::from_rgb(50, 210, 50)));
    painter.line_segment([origin, Pos2::new(origin.x - 25.0, origin.y + 16.0)], Stroke::new(2.0, Color32::from_rgb(50, 130, 230)));
}

// =========================================================================
// OBJETOS DEL MUNDO 3D (FALLBACK)
// =========================================================================

fn draw_viewport_scene_objects(painter: &egui::Painter, vp: Rect, is_playing: bool) {
    let center = Pos2::new(vp.center().x, vp.center().y + 30.0);

    let box_s = 28.0;
    let b_rect = Rect::from_center_size(center, Vec2::new(box_s, box_s * 1.4));

    painter.rect_stroke(
        b_rect,
        Rounding::same(1.0),
        Stroke::new(1.5, STUDIO_ACCENT_BLUE),
    );

    for c in [b_rect.left_top(), b_rect.right_top(), b_rect.left_bottom(), b_rect.right_bottom()] {
        painter.rect_filled(Rect::from_center_size(c, Vec2::new(4.0, 4.0)), 0.5, STUDIO_ACCENT_BLUE);
    }

    let player_color = if is_playing { Color32::from_rgb(0, 162, 255) } else { Color32::from_rgb(163, 162, 165) };
    painter.rect_filled(
        Rect::from_center_size(Pos2::new(center.x, center.y - 4.0), Vec2::new(18.0, 22.0)),
        2.0,
        player_color,
    );
    painter.rect_filled(
        Rect::from_center_size(Pos2::new(center.x, center.y - 20.0), Vec2::new(10.0, 10.0)),
        1.5,
        Color32::from_rgb(245, 205, 47),
    );

    painter.text(
        Pos2::new(center.x, center.y - 32.0),
        egui::Align2::CENTER_CENTER,
        "Player",
        FontId::proportional(11.0),
        Color32::WHITE,
    );
}

// =========================================================================
// VIEW SELECTOR CON TEXTURAS REALES DE ROBLOX STUDIO
// =========================================================================

fn draw_view_selector_cube(
    ui: &mut egui::Ui,
    rect: Rect,
    icons: &StudioIcons,
    pitch: &mut f32,
    yaw: &mut f32,
) {
    let center = rect.center();
    let r = rect.width() * 0.44;

    let cube_resp = ui.interact(rect, ui.id().with("view_selector_cube"), Sense::click());
    if cube_resp.clicked() {
        *pitch = 25.0;
        *yaw = 45.0;
    }

    ui.painter().circle_filled(center, r * 1.15, Color32::from_rgba_unmultiplied(35, 35, 40, 210));
    ui.painter().circle_stroke(center, r * 1.15, Stroke::new(1.0, Color32::from_rgb(60, 60, 65)));

    let cube_inner_rect = Rect::from_center_size(center, Vec2::new(r * 1.8, r * 1.8));
    ui.painter().image(
        icons.view_front.id(),
        cube_inner_rect,
        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
        Color32::WHITE,
    );
}
