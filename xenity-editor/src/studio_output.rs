use eframe::egui::{self, Color32, FontId, Pos2, Rect, Rounding, Sense, Stroke, Vec2};
use crate::studio_style::*;
use crate::studio_textures::StudioIcons;
use crate::icons;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LogType {
    Info,
    Warning,
    Error,
    Command,
}

#[derive(Clone)]
pub struct OutputLog {
    pub timestamp: String,
    pub log_type: LogType,
    pub source: &'static str,
    pub message: String,
}

pub struct StudioOutputState {
    pub logs: Vec<OutputLog>,
    pub search_query: String,
    pub filter_message_idx: usize, // 0: "Todos los mensajes", 1: "Información", 2: "Advertencias", 3: "Errores"
    pub filter_context_idx: usize, // 0: "Todos los contextos", 1: "Cliente", 2: "Servidor", 3: "Studio", 4: "Editar"
    pub command_input: String,
}

impl Default for StudioOutputState {
    fn default() -> Self {
        let initial_logs = vec![
            OutputLog {
                timestamp: "23:46:15.028".into(),
                log_type: LogType::Info,
                source: "Studio",
                message: "Darkiusresurfaced joined live editing session.".into(),
            },
            OutputLog {
                timestamp: "23:46:19.360".into(),
                log_type: LogType::Warning,
                source: "Studio",
                message: "ContentId: rbxassetid://203785492 failed due to: Asset type does not match requested type".into(),
            },
            OutputLog {
                timestamp: "23:46:23.532".into(),
                log_type: LogType::Error,
                source: "Editar",
                message: "[robloxstudio-mcp] /ready failed for instance:3kp-8kg/edit: RequestAsync threw for http://localhost:58741/ready: HttpError: ConnectFail".into(),
            },
        ];

        Self {
            logs: initial_logs,
            search_query: String::new(),
            filter_message_idx: 0,
            filter_context_idx: 0,
            command_input: String::new(),
        }
    }
}

pub enum CommandBarResult {
    None,
    SpawnPart,
    SpawnScript,
    ClearOutput,
    ExecuteScript(String),
}

impl StudioOutputState {
    pub fn add_log(&mut self, log_type: LogType, source: &'static str, message: impl Into<String>) {
        let now = chrono_or_fallback_time();
        self.logs.push(OutputLog {
            timestamp: now,
            log_type,
            source,
            message: message.into(),
        });
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        icons: &StudioIcons,
        show_command_bar: bool,
    ) -> (CommandBarResult, bool) {
        let mut result = CommandBarResult::None;
        let mut close_requested = false;

        // =====================================================================
        // 1. HEADER DE PANEL: Salida + Botones ⤢ y ✕
        // =====================================================================
        let header_h = 24.0;
        let (header_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), header_h), Sense::hover());
        ui.painter().rect_filled(header_rect, 0.0, STUDIO_BG);

        ui.painter().text(
            header_rect.center(),
            egui::Align2::CENTER_CENTER,
            "Salida",
            FontId::proportional(12.0),
            Color32::from_rgb(220, 220, 225),
        );

        // Botones derecha: ⤢ y ✕
        let undock_center = Pos2::new(header_rect.max.x - 26.0, header_rect.center().y);
        let undock_rect = Rect::from_center_size(undock_center, Vec2::new(16.0, 16.0));
        let undock_resp = ui.interact(undock_rect, ui.id().with("out_undock_btn"), Sense::click());
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
        let close_resp = ui.interact(close_rect, ui.id().with("out_close_btn"), Sense::click());
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
        // 2. SUB-BARRA DE HERRAMIENTAS: Dropdowns + Filtrar + Limpiar (🧹) + ⋮
        // =====================================================================
        let toolbar_h = 28.0;
        let (tbar_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), toolbar_h), Sense::hover());
        ui.painter().rect_filled(tbar_rect, 0.0, Color32::from_rgb(32, 32, 35));

        let msg_options = ["Todos los mensajes", "Información", "Advertencias", "Errores"];
        let ctx_options = ["Todos los contextos", "Cliente", "Servidor", "Studio", "Editar"];

        // Lado izquierdo: Desplegables "Todos los mensajes" y "Todos los contextos"
        let mut left_ui = ui.child_ui(
            Rect::from_min_max(Pos2::new(tbar_rect.min.x + 6.0, tbar_rect.min.y), Pos2::new(tbar_rect.center().x, tbar_rect.max.y)),
            egui::Layout::left_to_right(egui::Align::Center),
            None,
        );
        left_ui.spacing_mut().item_spacing = Vec2::new(6.0, 0.0);

        egui::ComboBox::from_id_salt("out_msg_filter")
            .selected_text(msg_options[self.filter_message_idx])
            .show_ui(&mut left_ui, |ui| {
                for (i, opt) in msg_options.iter().enumerate() {
                    ui.selectable_value(&mut self.filter_message_idx, i, *opt);
                }
            });

        egui::ComboBox::from_id_salt("out_ctx_filter")
            .selected_text(ctx_options[self.filter_context_idx])
            .show_ui(&mut left_ui, |ui| {
                for (i, opt) in ctx_options.iter().enumerate() {
                    ui.selectable_value(&mut self.filter_context_idx, i, *opt);
                }
            });

        // Lado derecho: Filtrar... + Broom (🧹) + Options (⋮)
        let right_w = 200.0;
        let right_rect = Rect::from_min_max(Pos2::new(tbar_rect.max.x - right_w, tbar_rect.min.y), tbar_rect.max);
        let mut right_ui = ui.child_ui(right_rect, egui::Layout::right_to_left(egui::Align::Center), None);
        right_ui.spacing_mut().item_spacing = Vec2::new(4.0, 0.0);
        right_ui.add_space(4.0);

        // Botón ⋮ (Opciones)
        let (opt_rect, opt_resp) = right_ui.allocate_exact_size(Vec2::new(20.0, 20.0), Sense::click());
        if opt_resp.hovered() {
            right_ui.painter().rect_filled(opt_rect, Rounding::same(2.0), STUDIO_HOVER);
        }
        icons::three_dots_vertical(
            right_ui.painter(),
            opt_rect.center(),
            if opt_resp.hovered() { Color32::WHITE } else { Color32::from_rgb(160, 160, 165) },
        );
        opt_resp.on_hover_text("Opciones de salida");

        // Botón Broom 🧹 (Limpiar)
        let (broom_rect, broom_resp) = right_ui.allocate_exact_size(Vec2::new(20.0, 20.0), Sense::click());
        if broom_resp.clicked() {
            self.logs.clear();
            result = CommandBarResult::ClearOutput;
        }
        if broom_resp.hovered() {
            right_ui.painter().rect_filled(broom_rect, Rounding::same(2.0), STUDIO_HOVER);
        }
        icons::broom_icon(
            right_ui.painter(),
            broom_rect.center(),
            if broom_resp.hovered() { Color32::WHITE } else { Color32::from_rgb(175, 175, 180) },
        );
        broom_resp.on_hover_text("Limpiar salida");

        // Input de búsqueda: Filtrar...
        let search_w = 115.0;
        let (filter_box_rect, _) = right_ui.allocate_exact_size(Vec2::new(search_w, 20.0), Sense::hover());
        right_ui.painter().rect_filled(filter_box_rect, Rounding::same(3.0), STUDIO_INPUT_BG);
        right_ui.painter().rect_stroke(filter_box_rect, Rounding::same(3.0), Stroke::new(1.0, STUDIO_BORDER));

        let text_rect = Rect::from_min_max(
            Pos2::new(filter_box_rect.min.x + 6.0, filter_box_rect.min.y + 1.0),
            Pos2::new(filter_box_rect.max.x - 18.0, filter_box_rect.max.y - 1.0),
        );
        let mut edit_ui = right_ui.child_ui(text_rect, egui::Layout::left_to_right(egui::Align::Center), None);
        edit_ui.spacing_mut().item_spacing = Vec2::ZERO;
        edit_ui.add(
            egui::TextEdit::singleline(&mut self.search_query)
                .hint_text("Filtrar...")
                .frame(false)
                .margin(egui::Margin::symmetric(0.0, 1.0))
                .font(FontId::proportional(11.0))
                .desired_width(text_rect.width()),
        );

        // Flecha desplegable ▾ pequeña dentro del cuadro Filtrar
        let drop_arrow_pos = Pos2::new(filter_box_rect.max.x - 9.0, filter_box_rect.center().y);
        right_ui.painter().text(
            drop_arrow_pos,
            egui::Align2::CENTER_CENTER,
            "▾",
            FontId::proportional(9.0),
            Color32::from_rgb(140, 140, 145),
        );

        ui.painter().line_segment(
            [Pos2::new(tbar_rect.min.x, tbar_rect.max.y), Pos2::new(tbar_rect.max.x, tbar_rect.max.y)],
            Stroke::new(1.0, STUDIO_BORDER),
        );

        // =====================================================================
        // 3. LOG STREAM CONSOLA (Monospace, Formato idéntico a la imagen 1)
        // =====================================================================
        let cmd_bar_h = if show_command_bar { 26.0 } else { 0.0 };
        let stream_h = ui.available_height() - cmd_bar_h;
        let query = self.search_query.to_lowercase();

        let stream_rect = Rect::from_min_size(ui.cursor().min, Vec2::new(ui.available_width(), stream_h));
        ui.painter().rect_filled(stream_rect, 0.0, Color32::from_rgb(20, 20, 22));

        egui::ScrollArea::vertical()
            .stick_to_bottom(true)
            .id_salt("roblox_studio_output_scroll")
            .max_height(stream_h)
            .show(ui, |ui| {
                ui.add_space(4.0);
                for log in &self.logs {
                    // Filtro por tipo de mensaje
                    let allow_type = match self.filter_message_idx {
                        1 => matches!(log.log_type, LogType::Info),
                        2 => matches!(log.log_type, LogType::Warning),
                        3 => matches!(log.log_type, LogType::Error),
                        _ => true,
                    };
                    if !allow_type {
                        continue;
                    }

                    // Filtro por contexto
                    let allow_ctx = match self.filter_context_idx {
                        1 => log.source.contains("Cliente") || log.source.contains("Client"),
                        2 => log.source.contains("Servidor") || log.source.contains("Server"),
                        3 => log.source.contains("Studio"),
                        4 => log.source.contains("Editar") || log.source.contains("Edit"),
                        _ => true,
                    };
                    if !allow_ctx {
                        continue;
                    }

                    // Filtro por texto
                    if !query.is_empty()
                        && !log.message.to_lowercase().contains(&query)
                        && !log.source.to_lowercase().contains(&query)
                    {
                        continue;
                    }

                    // Colores idénticos a la consola oficial de Roblox Studio (Imagen 1)
                    let text_color = match log.log_type {
                        LogType::Info => Color32::from_rgb(215, 215, 220),
                        LogType::Warning => Color32::from_rgb(255, 153, 0),  // Naranja oficial warning (#FF9900)
                        LogType::Error => Color32::from_rgb(255, 75, 40),    // Rojo/naranja oficial error (#FF4B28)
                        LogType::Command => Color32::from_rgb(0, 162, 255),  // Azul cian comando
                    };

                    let line_text = format!("{} {}  -  {}", log.timestamp, log.message, log.source);

                    ui.horizontal(|ui| {
                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new(line_text)
                                .monospace()
                                .size(11.5)
                                .color(text_color),
                        );
                    });
                }
            });

        // =====================================================================
        // 4. BARRA DE COMANDOS INFERIOR (Formato idéntico a la imagen 1)
        // =====================================================================
        if show_command_bar {
            let (cmd_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 26.0), Sense::hover());
            ui.painter().rect_filled(cmd_rect, 0.0, Color32::from_rgb(24, 24, 27));
            ui.painter().line_segment(
                [Pos2::new(cmd_rect.min.x, cmd_rect.min.y), Pos2::new(cmd_rect.max.x, cmd_rect.min.y)],
                Stroke::new(1.0, STUDIO_BORDER),
            );

            // Handle de arrastre a la izquierda: ⋮
            ui.painter().text(
                Pos2::new(cmd_rect.min.x + 8.0, cmd_rect.center().y),
                egui::Align2::CENTER_CENTER,
                "⋮",
                FontId::proportional(13.0),
                Color32::from_rgb(110, 110, 115),
            );

            // Botones a la derecha: Bookmark, Clock, ▶ Ejecutar
            let mut right_cmd = ui.child_ui(
                Rect::from_min_max(Pos2::new(cmd_rect.max.x - 170.0, cmd_rect.min.y), cmd_rect.max),
                egui::Layout::right_to_left(egui::Align::Center),
                None,
            );
            right_cmd.spacing_mut().item_spacing = Vec2::new(4.0, 0.0);
            right_cmd.add_space(6.0);

            // Botón ▶ Ejecutar
            let run_btn = right_cmd.add(
                egui::Button::new(
                    egui::RichText::new("▶ Ejecutar")
                        .size(11.0)
                        .color(Color32::from_rgb(220, 220, 225)),
                )
                .fill(Color32::from_rgb(38, 38, 44))
                .stroke(Stroke::new(1.0, Color32::from_rgb(55, 55, 62)))
                .rounding(Rounding::same(2.5)),
            );

            // Botón Reloj de Historial (🕒)
            let (hist_rect, hist_resp) = right_cmd.allocate_exact_size(Vec2::new(18.0, 18.0), Sense::click());
            if hist_resp.hovered() {
                right_cmd.painter().rect_filled(hist_rect, Rounding::same(2.0), STUDIO_HOVER);
            }
            icons::clock_icon(
                right_cmd.painter(),
                hist_rect.center(),
                if hist_resp.hovered() { Color32::WHITE } else { Color32::from_rgb(150, 150, 155) },
            );
            hist_resp.on_hover_text("Historial de comandos");

            // Botón Marcador (Bookmark)
            let (bm_rect, bm_resp) = right_cmd.allocate_exact_size(Vec2::new(18.0, 18.0), Sense::click());
            if bm_resp.hovered() {
                right_cmd.painter().rect_filled(bm_rect, Rounding::same(2.0), STUDIO_HOVER);
            }
            icons::bookmark_icon(
                right_cmd.painter(),
                bm_rect.center(),
                if bm_resp.hovered() { Color32::WHITE } else { Color32::from_rgb(150, 150, 155) },
            );
            bm_resp.on_hover_text("Marcador");

            // Campo de texto de comando central
            let edit_rect = Rect::from_min_max(
                Pos2::new(cmd_rect.min.x + 18.0, cmd_rect.min.y + 2.0),
                Pos2::new(cmd_rect.max.x - 175.0, cmd_rect.max.y - 2.0),
            );

            let mut cmd_child = ui.child_ui(edit_rect, egui::Layout::left_to_right(egui::Align::Center), None);
            let edit_resp = cmd_child.add(
                egui::TextEdit::singleline(&mut self.command_input)
                    .hint_text("Ejecutar un comando con Ctrl+Enter o usar Ctrl+Arriba/Abajo para el historial")
                    .frame(false)
                    .font(FontId::monospace(11.0))
                    .text_color(Color32::WHITE)
                    .desired_width(edit_rect.width()),
            );

            let enter_pressed = edit_resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
            let run_clicked = run_btn.clicked();

            if (enter_pressed || run_clicked) && !self.command_input.trim().is_empty() {
                let cmd = self.command_input.trim().to_string();
                self.command_input.clear();

                self.add_log(LogType::Command, "CmdBar", cmd.clone());

                if cmd.contains("Part") || cmd.contains("part") {
                    self.add_log(LogType::Info, "Servidor", "Created new Part in Workspace.");
                    result = CommandBarResult::SpawnPart;
                } else if cmd.contains("clear") {
                    self.logs.clear();
                    result = CommandBarResult::ClearOutput;
                } else if cmd.starts_with("print") {
                    let inside = cmd.trim_start_matches("print(").trim_end_matches(')').trim_matches('"').trim_matches('\'');
                    self.add_log(LogType::Info, "Servidor", inside);
                } else {
                    self.add_log(LogType::Info, "Luau", format!("Executed: {}", cmd));
                    result = CommandBarResult::ExecuteScript(cmd);
                }
            }
        }

        (result, close_requested)
    }
}

fn chrono_or_fallback_time() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = now.as_secs();
    let millis = now.subsec_millis();
    let h = (secs / 3600) % 24;
    let m = (secs / 60) % 60;
    let s = secs % 60;
    format!("{:02}:{:02}:{:02}.{:03}", h, m, s, millis)
}
