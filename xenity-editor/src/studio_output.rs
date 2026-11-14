use eframe::egui::{self, Color32, FontId, Pos2, Rect, Rounding, Sense, Stroke, Vec2};
use crate::studio_style::*;
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
        show_command_bar: bool,
    ) -> CommandBarResult {
        let mut result = CommandBarResult::None;

        // 1. PESTAÑA SUPERIOR [ Output ]
        let tab_h = 26.0;
        let (tab_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), tab_h), Sense::hover());
        ui.painter().rect_filled(tab_rect, 0.0, STUDIO_BG);

        let active_tab_rect = Rect::from_min_size(tab_rect.min, Vec2::new(80.0, tab_h));
        ui.painter().rect_filled(active_tab_rect, Rounding { nw: 3.0, ne: 3.0, sw: 0.0, se: 0.0 }, STUDIO_TREE_BG);
        ui.painter().line_segment(
            [Pos2::new(active_tab_rect.min.x, active_tab_rect.min.y), Pos2::new(active_tab_rect.max.x, active_tab_rect.min.y)],
            Stroke::new(2.0, STUDIO_SELECTION),
        );
        ui.painter().text(active_tab_rect.center(), egui::Align2::CENTER_CENTER, "Output", FontId::proportional(11.5), Color32::WHITE);

        // Kebab menu
        icons::kebab_menu(ui.painter(), Pos2::new(tab_rect.max.x - 12.0, tab_rect.center().y), STUDIO_TEXT_MUTED);

        ui.painter().line_segment(
            [Pos2::new(tab_rect.min.x, tab_rect.max.y), Pos2::new(tab_rect.max.x, tab_rect.max.y)],
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
                    // Filtrado por nivel
                    let allow_level = match log.log_type {
                        LogType::Info => self.show_info,
                        LogType::Warning => self.show_warnings,
                        LogType::Error => self.show_errors,
                        LogType::Command => true,
                    };
                    if !allow_level {
                        continue;
                    }

                    // Filtrado por texto
                    if !query.is_empty() && !log.message.to_lowercase().contains(&query) {
                        continue;
                    }

                    ui.horizontal(|ui| {
                        ui.add_space(8.0);
                        // Timestamp
                        ui.label(egui::RichText::new(&log.timestamp).size(11.0).color(STUDIO_TEXT_MUTED));
                        // Source
                        let src_str = format!("-- [{}]", log.source);
                        ui.label(egui::RichText::new(src_str).size(11.0).color(Color32::from_rgb(130, 160, 200)));

                        // Message Text with authentic colors
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

        // 4. ROBLOX STUDIO COMMAND BAR (Barra de comandos Luau)
        if show_command_bar {
            let (cmd_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 28.0), Sense::hover());
            ui.painter().rect_filled(cmd_rect, 0.0, STUDIO_INPUT_BG);
            ui.painter().line_segment(
                [Pos2::new(cmd_rect.min.x, cmd_rect.min.y), Pos2::new(cmd_rect.max.x, cmd_rect.min.y)],
                Stroke::new(1.0, STUDIO_BORDER),
            );

            // Prompt symbol "> "
            ui.painter().text(
                Pos2::new(cmd_rect.min.x + 12.0, cmd_rect.center().y),
                egui::Align2::LEFT_CENTER,
                ">",
                FontId::proportional(14.0),
                STUDIO_ACCENT_BLUE,
            );

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

        result
    }
}

fn chrono_or_fallback_time() -> String {
    // Generar timestamp limpio estilo Roblox Studio
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
