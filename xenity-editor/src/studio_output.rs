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
    pub show_info: bool,
    pub show_warnings: bool,
    pub show_errors: bool,
    pub command_input: String,
}

impl Default for StudioOutputState {
    fn default() -> Self {
        let initial_logs = vec![
            OutputLog {
                timestamp: "00:01:22.412".into(),
                log_type: LogType::Info,
                source: "Server",
                message: "[Xenity Engine] Luau VM Core 5.1 initialized.".into(),
            },
            OutputLog {
                timestamp: "00:01:22.450".into(),
                log_type: LogType::Info,
                source: "Studio",
                message: "Roblox Studio layout docked successfully (60.0 FPS).".into(),
            },
            OutputLog {
                timestamp: "00:01:22.502".into(),
                log_type: LogType::Info,
                source: "Workspace",
                message: "Baseplate loaded: 512x512 studs (Material: Plastic).".into(),
            },
            OutputLog {
                timestamp: "00:01:22.810".into(),
                log_type: LogType::Warning,
                source: "SoundService",
                message: "Sound 'HitSound' has empty SoundId; set valid rbxassetid:// to play.".into(),
            },
            OutputLog {
                timestamp: "00:01:23.015".into(),
                log_type: LogType::Info,
                source: "Server",
                message: "Team Create active: 1 editor connected.".into(),
            },
        ];

        Self {
            logs: initial_logs,
            search_query: String::new(),
            show_info: true,
            show_warnings: true,
            show_errors: true,
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

        // 1. HEADER DE PANEL DESACOPLABLE (Roblox Studio Header)
        let header_h = 26.0;
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

        // 2. SUB-BARRA DE HERRAMIENTAS (Filtros, Búsqueda, Clear)
        let toolbar_h = 28.0;
        let (tbar_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), toolbar_h), Sense::hover());
        ui.painter().rect_filled(tbar_rect, 0.0, STUDIO_BG);

        let mut child_tbar = ui.child_ui(tbar_rect, egui::Layout::left_to_right(egui::Align::Center), None);
        child_tbar.add_space(8.0);

        // Botón Clear 🗑
        if child_tbar.button("🗑 Clear").clicked() {
            self.logs.clear();
            result = CommandBarResult::ClearOutput;
        }

        child_tbar.separator();

        // Filtros: Info, Warnings, Errors
        child_tbar.checkbox(&mut self.show_info, "Info");
        child_tbar.checkbox(&mut self.show_warnings, "Warnings");
        child_tbar.checkbox(&mut self.show_errors, "Errors");

        child_tbar.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(8.0);
            let s_w = 140.0;
            ui.add(
                egui::TextEdit::singleline(&mut self.search_query)
                    .hint_text("Search Output...")
                    .desired_width(s_w),
            );
        });

        ui.painter().line_segment(
            [Pos2::new(tbar_rect.min.x, tbar_rect.max.y), Pos2::new(tbar_rect.max.x, tbar_rect.max.y)],
            Stroke::new(1.0, STUDIO_BORDER),
        );

        // 3. LOG STREAM ÁREA DE TEXTO
        let cmd_bar_h = if show_command_bar { 28.0 } else { 0.0 };
        let stream_h = ui.available_height() - cmd_bar_h;

        let query = self.search_query.to_lowercase();

        egui::ScrollArea::vertical()
            .stick_to_bottom(true)
            .id_salt("roblox_studio_output_scroll")
            .max_height(stream_h)
            .show(ui, |ui| {
                ui.add_space(4.0);
                for log in &self.logs {
                    let allow_level = match log.log_type {
                        LogType::Info => self.show_info,
                        LogType::Warning => self.show_warnings,
                        LogType::Error => self.show_errors,
                        LogType::Command => true,
                    };
                    if !allow_level {
                        continue;
                    }

                    if !query.is_empty() && !log.message.to_lowercase().contains(&query) {
                        continue;
                    }

                    ui.horizontal(|ui| {
                        ui.add_space(8.0);
                        ui.label(egui::RichText::new(&log.timestamp).size(11.0).color(STUDIO_TEXT_MUTED));
                        let src_str = format!("-- [{}]", log.source);
                        ui.label(egui::RichText::new(src_str).size(11.0).color(Color32::from_rgb(130, 160, 200)));

                        let (txt_col, prefix) = match log.log_type {
                            LogType::Info => (Color32::from_rgb(220, 220, 225), ""),
                            LogType::Warning => (Color32::from_rgb(255, 210, 60), "⚠️ "),
                            LogType::Error => (Color32::from_rgb(255, 75, 75), "🛑 "),
                            LogType::Command => (Color32::from_rgb(0, 162, 255), "> "),
                        };

                        let formatted = format!("{}{}", prefix, log.message);
                        ui.label(egui::RichText::new(formatted).size(11.5).color(txt_col));
                    });
                }
            });

        // 4. ROBLOX STUDIO COMMAND BAR
        if show_command_bar {
            let (cmd_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 28.0), Sense::hover());
            ui.painter().rect_filled(cmd_rect, 0.0, STUDIO_INPUT_BG);
            ui.painter().line_segment(
                [Pos2::new(cmd_rect.min.x, cmd_rect.min.y), Pos2::new(cmd_rect.max.x, cmd_rect.min.y)],
                Stroke::new(1.0, STUDIO_BORDER),
            );

            // Icono CommandBar real
            let cmd_icon_rect = Rect::from_center_size(Pos2::new(cmd_rect.min.x + 14.0, cmd_rect.center().y), Vec2::new(14.0, 14.0));
            ui.painter().image(icons.win_cmdbar.id(), cmd_icon_rect, Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), Color32::WHITE);

            let edit_rect = Rect::from_min_max(
                Pos2::new(cmd_rect.min.x + 28.0, cmd_rect.min.y + 2.0),
                Pos2::new(cmd_rect.max.x - 40.0, cmd_rect.max.y - 2.0),
            );

            let mut cmd_child = ui.child_ui(edit_rect, egui::Layout::left_to_right(egui::Align::Center), None);
            let edit_resp = cmd_child.add(
                egui::TextEdit::singleline(&mut self.command_input)
                    .hint_text("Run a command or Luau script (e.g. Instance.new('Part', workspace), print('Hello'))")
                    .frame(false)
                    .font(FontId::monospace(11.5))
                    .desired_width(edit_rect.width()),
            );

            let submit = edit_resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

            if submit && !self.command_input.trim().is_empty() {
                let cmd = self.command_input.trim().to_string();
                self.command_input.clear();

                self.add_log(LogType::Command, "CmdBar", cmd.clone());

                if cmd.contains("Part") || cmd.contains("part") {
                    self.add_log(LogType::Info, "Server", "Created new Part in Workspace.");
                    result = CommandBarResult::SpawnPart;
                } else if cmd.contains("clear") {
                    self.logs.clear();
                    result = CommandBarResult::ClearOutput;
                } else if cmd.starts_with("print") {
                    let inside = cmd.trim_start_matches("print(").trim_end_matches(')').trim_matches('"').trim_matches('\'');
                    self.add_log(LogType::Info, "Server", inside);
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
