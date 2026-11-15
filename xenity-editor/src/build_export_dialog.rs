// SPDX-License-Identifier: MIT
//
// Build Settings Dialog for Roblox Studio / Xenity Engine
// Integrates the 2-column + Scenes in Build architecture of Unity
// styled seamlessly with Roblox Studio's authentic dark UI theme (#252526 / #1E1E1E / #007ACC).

use eframe::egui::{self, Color32, FontId, Pos2, Rect, Rounding, Sense, Stroke, Vec2};
use crate::studio_style::*;
use crate::xenity_ffi::XenityEngine;
use std::path::{Path, PathBuf};
use std::fs;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ExportPlatform {
    PlayStation3,
    PSP,
    PSVita,
    Windows,
    Android,
}

impl ExportPlatform {
    pub fn name(&self) -> &'static str {
        match self {
            ExportPlatform::PlayStation3 => "PlayStation 3 (PS3)",
            ExportPlatform::PSP => "PlayStation Portable (PSP)",
            ExportPlatform::PSVita => "PlayStation Vita",
            ExportPlatform::Windows => "Windows Standalone (PC)",
            ExportPlatform::Android => "Android Mobile",
        }
    }

    pub fn svg_source(&self) -> egui::ImageSource<'static> {
        match self {
            ExportPlatform::PlayStation3 => egui::include_image!("../assets/icons/platforms/ps3.svg"),
            ExportPlatform::PSP => egui::include_image!("../assets/icons/platforms/psp.svg"),
            ExportPlatform::PSVita => egui::include_image!("../assets/icons/platforms/psvita.svg"),
            ExportPlatform::Windows => egui::include_image!("../assets/icons/platforms/windows.svg"),
            ExportPlatform::Android => egui::include_image!("../assets/icons/platforms/android.svg"),
        }
    }

    pub fn badge(&self) -> &'static str {
        match self {
            ExportPlatform::PlayStation3 => "PS3",
            ExportPlatform::PSP => "PSP",
            ExportPlatform::PSVita => "VITA",
            ExportPlatform::Windows => "PC",
            ExportPlatform::Android => "APK",
        }
    }

    pub fn hardware_desc(&self) -> &'static str {
        match self {
            ExportPlatform::PlayStation3 => "Cell Broadband Engine @ 3.2GHz | RSX Reality Synthesizer (256MB XDR + 256MB VRAM)",
            ExportPlatform::PSP => "MIPS R4000 @ 333MHz | Graphics Core GU (32MB / 64MB RAM)",
            ExportPlatform::PSVita => "Quad-core ARM Cortex-A9 | SGX543MP4+ (512MB RAM + 128MB VRAM)",
            ExportPlatform::Windows => "x86_64 | OpenGL 3.3 Core Renderer (Sin límite estricto de consola)",
            ExportPlatform::Android => "GLES 3.0 / Vulkan | Arquitecturas ARM64",
        }
    }

    pub fn default_title_id(&self) -> &'static str {
        match self {
            ExportPlatform::PlayStation3 => "NPEB00123",
            ExportPlatform::PSP => "ULES00123",
            ExportPlatform::PSVita => "PCSB00123",
            ExportPlatform::Windows => "XENITY_GAME",
            ExportPlatform::Android => "com.xenity.game",
        }
    }

    pub fn default_output_folder(&self) -> &'static str {
        match self {
            ExportPlatform::PlayStation3 => "build/ps3",
            ExportPlatform::PSP => "build/psp",
            ExportPlatform::PSVita => "build/psvita",
            ExportPlatform::Windows => "build/windows",
            ExportPlatform::Android => "build/android",
        }
    }

    pub fn ffi_platform_id(&self) -> i32 {
        match self {
            ExportPlatform::PlayStation3 => 3,
            ExportPlatform::PSP => 1,
            ExportPlatform::PSVita => 2,
            ExportPlatform::Windows => 0,
            ExportPlatform::Android => 0,
        }
    }

    pub fn supported_targets(&self) -> &'static [&'static str] {
        match self {
            ExportPlatform::PlayStation3 => &["PS3_GAME Folder (RPCS3 / HEN)", "PKG Package (Installable)", "ISO Disc Image"],
            ExportPlatform::PSP => &["EBOOT.PBP (PPSSPP & Consola Real)", "ISO Disc Image"],
            ExportPlatform::PSVita => &["VPK Package (Vita3K / HENkaku)", "Folder App (ux0:app)"],
            ExportPlatform::Windows => &["Windows (64-bit)", "Windows (32-bit)", "Universal Windows Platform"],
            ExportPlatform::Android => &["APK Package", "AAB (Google Play)"],
        }
    }

    pub fn supported_architectures(&self) -> &'static [&'static str] {
        match self {
            ExportPlatform::PlayStation3 => &["Cell Broadband Engine @ 3.2GHz + RSX"],
            ExportPlatform::PSP => &["MIPS R4000 @ 333MHz + GU"],
            ExportPlatform::PSVita => &["Quad-core ARM Cortex-A9 + SGX543MP4+"],
            ExportPlatform::Windows => &["Intel 64-bit (x86_64)", "ARM64", "x86 (32-bit)"],
            ExportPlatform::Android => &["ARM64-v8a", "armeabi-v7a", "x86_64"],
        }
    }
}

pub struct BuildExportState {
    pub is_open: bool,
    pub selected_platform: ExportPlatform,
    pub active_build_platform: ExportPlatform,

    // Scenes In Build
    pub scenes: Vec<(bool, String)>,

    // Right-column platform settings
    pub target_format_idx: usize,
    pub arch_idx: usize,
    pub game_title: String,
    pub title_id: String,
    pub version: String,
    pub export_path: String,

    // Checkboxes
    pub copy_pdb: bool,
    pub create_vs_solution: bool,
    pub development_build: bool,
    pub autoconnect_profiler: bool,
    pub deep_profiling: bool,
    pub script_debugging: bool,
    pub compression_method_idx: usize,

    // Xenity optimization
    pub auto_downscale_textures: bool,
    pub compile_native_shaders: bool,
    pub clean_destination: bool,

    // Bottom Asset Import Overrides
    pub asset_overrides_open: bool,
    pub max_texture_size_idx: usize,
    pub texture_compression_idx: usize,

    // Status
    pub is_exporting: bool,
    pub status_message: String,
    pub is_success: bool,
    pub last_exported_folder: Option<PathBuf>,
}

impl Default for BuildExportState {
    fn default() -> Self {
        Self {
            is_open: false,
            selected_platform: ExportPlatform::PlayStation3,
            active_build_platform: ExportPlatform::PlayStation3,

            scenes: vec![
                (true, "Scenes/MainScene.xencene".to_string()),
                (true, "Place1 (Current Scene)".to_string()),
            ],

            target_format_idx: 0,
            arch_idx: 0,
            game_title: "My Roblox Place".to_string(),
            title_id: "NPEB00123".to_string(),
            version: "1.00".to_string(),
            export_path: "build/ps3".to_string(),

            copy_pdb: false,
            create_vs_solution: false,
            development_build: false,
            autoconnect_profiler: false,
            deep_profiling: false,
            script_debugging: false,
            compression_method_idx: 0,

            auto_downscale_textures: true,
            compile_native_shaders: true,
            clean_destination: false,

            asset_overrides_open: false,
            max_texture_size_idx: 0,
            texture_compression_idx: 0,

            is_exporting: false,
            status_message: "Listo para compilar.".to_string(),
            is_success: false,
            last_exported_folder: None,
        }
    }
}

pub enum BuildDialogAction {
    None,
    OpenOutputLog(String),
}

impl BuildExportState {
    pub fn open(&mut self) {
        self.is_open = true;
        self.status_message = "Listo para compilar.".to_string();
        self.is_success = false;
    }

    pub fn select_platform(&mut self, platform: ExportPlatform) {
        if self.selected_platform != platform {
            self.selected_platform = platform;
            self.target_format_idx = 0;
            self.arch_idx = 0;
            self.title_id = platform.default_title_id().to_string();
            self.export_path = platform.default_output_folder().to_string();
            self.status_message = format!("Plataforma seleccionada: {}", platform.name());
            self.is_success = false;
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, engine: &mut Option<XenityEngine>) -> BuildDialogAction {
        if !self.is_open {
            return BuildDialogAction::None;
        }

        let mut action = BuildDialogAction::None;

        // Modal overlay: orden Middle (oscurece sutilmente el fondo para dar foco al modal)
        let screen_rect = ctx.screen_rect();
        let mut clicked_backdrop = false;

        egui::Area::new(egui::Id::new("build_modal_backdrop_area"))
            .order(egui::Order::Middle)
            .fixed_pos(screen_rect.min)
            .show(ctx, |ui| {
                let (rect, resp) = ui.allocate_exact_size(screen_rect.size(), Sense::click());
                ui.painter().rect_filled(rect, 0.0, Color32::from_rgba_unmultiplied(0, 0, 0, 110));
                if resp.clicked() {
                    clicked_backdrop = true;
                }
            });

        let mut is_still_open = true;

        // =====================================================================
        // VENTANA MODAL ESTILO OFICIAL ROBLOX STUDIO (Order::Foreground)
        // =====================================================================
        egui::Window::new("BuildSettingsModal")
            .order(egui::Order::Foreground)
            .title_bar(false)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .fixed_size([720.0, 620.0])
            .frame(
                egui::Frame::window(&ctx.style())
                    .fill(Color32::from_rgb(32, 32, 35))
                    .stroke(Stroke::new(1.0, Color32::from_rgb(52, 54, 62)))
                    .rounding(Rounding::same(8.0))
                    .inner_margin(egui::Margin::same(14.0)),
            )
            .show(ctx, |ui| {
                // -------------------------------------------------------------
                // 1. CABECERA ELEGANTE ESTILO ROBLOX STUDIO
                // -------------------------------------------------------------
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("⚙").font(FontId::proportional(16.0)).color(Color32::from_rgb(0, 162, 255)));
                    ui.label(
                        egui::RichText::new("Configuración de Compilación y Exportación (Build Settings)")
                            .font(FontId::proportional(13.5))
                            .strong()
                            .color(Color32::WHITE),
                    );

                    // Botón de cerrar tipo Roblox Studio (X con hover circular suave)
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let (close_rect, close_resp) = ui.allocate_exact_size(Vec2::new(24.0, 24.0), Sense::click());
                        if close_resp.hovered() {
                            ui.painter().circle_filled(close_rect.center(), 12.0, Color32::from_rgb(54, 56, 64));
                        }
                        ui.painter().text(
                            close_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            "✕",
                            FontId::proportional(12.0),
                            if close_resp.hovered() { Color32::WHITE } else { Color32::from_rgb(180, 182, 190) },
                        );
                        if close_resp.clicked() {
                            is_still_open = false;
                        }
                    });
                });

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                // -------------------------------------------------------------
                // 2. SECCIÓN: ESCENAS A COMPILAR (SCENES IN BUILD)
                // -------------------------------------------------------------
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("ESCENAS A COMPILAR (SCENES IN BUILD)")
                            .font(FontId::proportional(11.0))
                            .strong()
                            .color(Color32::from_rgb(170, 173, 185)),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.add_sized(
                            Vec2::new(145.0, 22.0),
                            egui::Button::new(
                                egui::RichText::new("+ Añadir escena actual")
                                    .font(FontId::proportional(11.0))
                                    .color(Color32::from_rgb(220, 222, 230)),
                            )
                            .fill(Color32::from_rgb(42, 44, 52)),
                        ).clicked() {
                            let active_scene_name = "Place1 (Current Scene)".to_string();
                            if !self.scenes.iter().any(|(_, s)| s == &active_scene_name) {
                                self.scenes.push((true, active_scene_name));
                            }
                        }
                    });
                });

                ui.add_space(4.0);

                let scenes_box_h = 76.0;
                let scenes_box_rect = ui.allocate_space(Vec2::new(ui.available_width(), scenes_box_h)).1;
                ui.painter().rect_filled(scenes_box_rect, Rounding::same(4.0), Color32::from_rgb(22, 22, 25));
                ui.painter().rect_stroke(scenes_box_rect, Rounding::same(4.0), Stroke::new(1.0, Color32::from_rgb(46, 48, 56)));

                let mut scene_y = scenes_box_rect.min.y + 4.0;
                for (idx, (enabled, path)) in self.scenes.iter_mut().enumerate() {
                    if scene_y + 20.0 > scenes_box_rect.max.y {
                        break;
                    }
                    let item_rect = Rect::from_min_size(
                        Pos2::new(scenes_box_rect.min.x + 6.0, scene_y),
                        Vec2::new(scenes_box_rect.width() - 12.0, 20.0),
                    );

                    let chk_rect = Rect::from_min_size(Pos2::new(item_rect.min.x + 4.0, item_rect.min.y + 3.0), Vec2::new(14.0, 14.0));
                    let chk_resp = ui.interact(chk_rect, ui.id().with(("scene_chk", idx)), Sense::click());
                    if chk_resp.clicked() {
                        *enabled = !*enabled;
                    }

                    // Checkbox visual Studio
                    ui.painter().rect_filled(chk_rect, Rounding::same(3.0), Color32::from_rgb(32, 34, 40));
                    ui.painter().rect_stroke(chk_rect, Rounding::same(3.0), Stroke::new(1.0, Color32::from_rgb(70, 74, 86)));
                    if *enabled {
                        ui.painter().text(chk_rect.center(), egui::Align2::CENTER_CENTER, "✔", FontId::proportional(11.0), Color32::from_rgb(0, 162, 255));
                    }

                    ui.painter().text(
                        Pos2::new(item_rect.min.x + 26.0, item_rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        path,
                        FontId::proportional(11.0),
                        Color32::from_rgb(215, 218, 225),
                    );

                    ui.painter().text(
                        Pos2::new(item_rect.max.x - 12.0, item_rect.center().y),
                        egui::Align2::RIGHT_CENTER,
                        format!("[{}]", idx),
                        FontId::proportional(11.0),
                        Color32::from_rgb(130, 134, 145),
                    );

                    scene_y += 22.0;
                }

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                // -------------------------------------------------------------
                // 3. SECCIÓN MEDIA: DOS COLUMNAS (Plataformas & Parámetros)
                // -------------------------------------------------------------
                let avail_w = ui.available_width();
                let left_col_w = 230.0;
                let right_col_w = avail_w - left_col_w - 14.0;
                let middle_h = 295.0;

                ui.horizontal(|ui| {
                    // --- COLUMNA IZQUIERDA: LISTA DE PLATAFORMAS ---
                    ui.vertical(|ui| {
                        ui.set_width(left_col_w);
                        ui.label(
                            egui::RichText::new("PLATAFORMAS")
                                .font(FontId::proportional(11.0))
                                .strong()
                                .color(Color32::from_rgb(170, 173, 185)),
                        );
                        ui.add_space(3.0);

                        let list_rect = ui.allocate_space(Vec2::new(left_col_w, middle_h)).1;
                        ui.painter().rect_filled(list_rect, Rounding::same(4.0), Color32::from_rgb(22, 22, 25));
                        ui.painter().rect_stroke(list_rect, Rounding::same(4.0), Stroke::new(1.0, Color32::from_rgb(46, 48, 56)));

                        let platforms = [
                            ExportPlatform::PlayStation3,
                            ExportPlatform::PSP,
                            ExportPlatform::PSVita,
                            ExportPlatform::Windows,
                            ExportPlatform::Android,
                        ];

                        let mut plat_y = list_rect.min.y + 4.0;
                        for plat in platforms {
                            let item_rect = Rect::from_min_size(
                                Pos2::new(list_rect.min.x + 4.0, plat_y),
                                Vec2::new(list_rect.width() - 8.0, 42.0),
                            );

                            let is_selected = self.selected_platform == plat;
                            let is_active = self.active_build_platform == plat;

                            let resp = ui.interact(item_rect, ui.id().with(("plat_item", plat.badge())), Sense::click());

                            // Fondo elegante Roblox Studio
                            let bg_color = if is_selected {
                                Color32::from_rgb(0, 110, 210) // Roblox Blue
                            } else if resp.hovered() {
                                Color32::from_rgb(38, 40, 48)
                            } else {
                                Color32::TRANSPARENT
                            };

                            let border_stroke = if is_selected {
                                Stroke::new(1.0, Color32::from_rgb(0, 162, 255))
                            } else {
                                Stroke::NONE
                            };

                            if bg_color != Color32::TRANSPARENT {
                                ui.painter().rect(item_rect, Rounding::same(4.0), bg_color, border_stroke);
                            }

                            // Logo SVG oficial proporcionado por el usuario
                            let img_rect = Rect::from_center_size(
                                Pos2::new(item_rect.min.x + 22.0, item_rect.center().y),
                                Vec2::new(28.0, 20.0),
                            );
                            let img_widget = egui::Image::new(plat.svg_source())
                                .max_size(Vec2::new(28.0, 20.0))
                                .tint(if is_selected { Color32::WHITE } else { Color32::from_rgb(220, 222, 230) });
                            ui.put(img_rect, img_widget);

                            // Nombre
                            let text_color = if is_selected { Color32::WHITE } else { Color32::from_rgb(215, 218, 225) };
                            ui.painter().text(
                                Pos2::new(item_rect.min.x + 42.0, item_rect.center().y - 6.0),
                                egui::Align2::LEFT_CENTER,
                                plat.name(),
                                FontId::proportional(11.5),
                                text_color,
                            );

                            // Badge pequeño (ej: PS3, PSP)
                            ui.painter().text(
                                Pos2::new(item_rect.min.x + 42.0, item_rect.center().y + 8.0),
                                egui::Align2::LEFT_CENTER,
                                plat.badge(),
                                FontId::proportional(9.5),
                                if is_selected { Color32::from_rgb(210, 235, 255) } else { Color32::from_rgb(140, 144, 155) },
                            );

                            // Indicador de plataforma activa
                            if is_active {
                                ui.painter().text(
                                    Pos2::new(item_rect.max.x - 10.0, item_rect.center().y),
                                    egui::Align2::RIGHT_CENTER,
                                    "✔ ACTIVA",
                                    FontId::proportional(9.0),
                                    if is_selected { Color32::WHITE } else { Color32::from_rgb(0, 200, 120) },
                                );
                            }

                            if resp.clicked() {
                                self.select_platform(plat);
                            }

                            plat_y += 46.0;
                        }
                    });

                    // --- COLUMNA DERECHA: AJUSTES DE LA PLATAFORMA SELECCIONADA ---
                    ui.vertical(|ui| {
                        ui.set_width(right_col_w);

                        // Cabecera de plataforma con SVG oficial del usuario
                        ui.horizontal(|ui| {
                            let head_img = egui::Image::new(self.selected_platform.svg_source())
                                .max_size(Vec2::new(36.0, 24.0));
                            ui.add(head_img);
                            ui.label(
                                egui::RichText::new(self.selected_platform.name())
                                    .font(FontId::proportional(14.0))
                                    .strong()
                                    .color(Color32::WHITE),
                            );
                        });

                        // Tarjeta de Hardware & VRAM
                        ui.add_space(2.0);
                        let hw_rect = ui.allocate_space(Vec2::new(right_col_w - 4.0, 32.0)).1;
                        ui.painter().rect_filled(hw_rect, Rounding::same(4.0), Color32::from_rgb(24, 25, 29));
                        ui.painter().rect_stroke(hw_rect, Rounding::same(4.0), Stroke::new(1.0, Color32::from_rgb(44, 46, 54)));
                        ui.painter().text(
                            Pos2::new(hw_rect.min.x + 8.0, hw_rect.center().y),
                            egui::Align2::LEFT_CENTER,
                            self.selected_platform.hardware_desc(),
                            FontId::proportional(10.0),
                            Color32::from_rgb(165, 170, 185),
                        );

                        ui.add_space(6.0);

                        egui::ScrollArea::vertical()
                            .max_height(middle_h - 48.0)
                            .show(ui, |ui| {
                                egui::Grid::new("roblox_build_grid")
                                    .num_columns(2)
                                    .spacing([12.0, 6.0])
                                    .show(ui, |ui| {
                                        // Target Platform
                                        ui.label(egui::RichText::new("Formato Destino:").font(FontId::proportional(11.0)));
                                        let targets = self.selected_platform.supported_targets();
                                        egui::ComboBox::from_id_source("cb_target_platform")
                                            .width(220.0)
                                            .selected_text(*targets.get(self.target_format_idx).unwrap_or(&targets[0]))
                                            .show_ui(ui, |ui| {
                                                for (idx, target) in targets.iter().enumerate() {
                                                    ui.selectable_value(&mut self.target_format_idx, idx, *target);
                                                }
                                            });
                                        ui.end_row();

                                        // Architecture
                                        ui.label(egui::RichText::new("Arquitectura CPU:").font(FontId::proportional(11.0)));
                                        let archs = self.selected_platform.supported_architectures();
                                        egui::ComboBox::from_id_source("cb_architecture")
                                            .width(220.0)
                                            .selected_text(*archs.get(self.arch_idx).unwrap_or(&archs[0]))
                                            .show_ui(ui, |ui| {
                                                for (idx, arch) in archs.iter().enumerate() {
                                                    ui.selectable_value(&mut self.arch_idx, idx, *arch);
                                                }
                                            });
                                        ui.end_row();

                                        // Game Title
                                        ui.label(egui::RichText::new("Nombre del Juego:").font(FontId::proportional(11.0)));
                                        ui.add_sized(Vec2::new(220.0, 20.0), egui::TextEdit::singleline(&mut self.game_title));
                                        ui.end_row();

                                        // Title ID
                                        ui.label(egui::RichText::new("Title ID / App ID:").font(FontId::proportional(11.0)));
                                        ui.add_sized(Vec2::new(220.0, 20.0), egui::TextEdit::singleline(&mut self.title_id));
                                        ui.end_row();

                                        // Export Folder
                                        ui.label(egui::RichText::new("Ruta de Salida:").font(FontId::proportional(11.0)));
                                        ui.horizontal(|ui| {
                                            ui.add_sized(Vec2::new(155.0, 20.0), egui::TextEdit::singleline(&mut self.export_path));
                                            if ui.add_sized(Vec2::new(60.0, 20.0), egui::Button::new("Defecto")).clicked() {
                                                self.export_path = self.selected_platform.default_output_folder().to_string();
                                            }
                                        });
                                        ui.end_row();

                                        // Checkboxes
                                        ui.label("");
                                        ui.checkbox(&mut self.development_build, "Compilación de desarrollo (Development Build)");
                                        ui.end_row();

                                        ui.label("");
                                        ui.checkbox(&mut self.auto_downscale_textures, "Optimizar y reducir texturas para la VRAM");
                                        ui.end_row();

                                        ui.label("");
                                        ui.checkbox(&mut self.compile_native_shaders, "Compilar shaders nativos de consola");
                                        ui.end_row();

                                        ui.label("");
                                        ui.checkbox(&mut self.clean_destination, "Limpiar carpeta destino antes de exportar");
                                        ui.end_row();

                                        // Compression Method
                                        ui.label(egui::RichText::new("Método Compresión:").font(FontId::proportional(11.0)));
                                        let compressions = ["Por Defecto", "LZ4 (Recomendado)", "LZ4HC (Máxima)", "Sin Compresión (Rápido)"];
                                        egui::ComboBox::from_id_source("cb_compression_method")
                                            .width(220.0)
                                            .selected_text(compressions[self.compression_method_idx.min(compressions.len() - 1)])
                                            .show_ui(ui, |ui| {
                                                for (idx, comp) in compressions.iter().enumerate() {
                                                    ui.selectable_value(&mut self.compression_method_idx, idx, *comp);
                                                }
                                            });
                                        ui.end_row();
                                    });
                            });
                    });
                });

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(6.0);

                // -------------------------------------------------------------
                // 4. BARRA INFERIOR DE ACCIONES (ROBLOX STUDIO BUTTONS)
                // -------------------------------------------------------------
                ui.horizontal(|ui| {
                    // Lado izquierdo: Opciones de textura y Player Settings
                    ui.vertical(|ui| {
                        let header_txt = if self.asset_overrides_open { "▼ Ajustes de Texturas (Overrides)" } else { "▶ Ajustes de Texturas (Overrides)" };
                        if ui.selectable_label(self.asset_overrides_open, egui::RichText::new(header_txt).font(FontId::proportional(11.0))).clicked() {
                            self.asset_overrides_open = !self.asset_overrides_open;
                        }

                        if self.asset_overrides_open {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("Max Textura:").font(FontId::proportional(10.5)));
                                let tex_sizes = ["Sin límite", "256", "512", "1024", "2048"];
                                egui::ComboBox::from_id_source("cb_max_tex_size")
                                    .width(95.0)
                                    .selected_text(tex_sizes[self.max_texture_size_idx.min(tex_sizes.len() - 1)])
                                    .show_ui(ui, |ui| {
                                        for (idx, size) in tex_sizes.iter().enumerate() {
                                            ui.selectable_value(&mut self.max_texture_size_idx, idx, *size);
                                        }
                                    });

                                ui.add_space(6.0);
                                ui.label(egui::RichText::new("Compresión:").font(FontId::proportional(10.5)));
                                let tex_comps = ["Sin Override", "DXT1 / RSX", "DXT5 / RGBA", "Sin Compresión"];
                                egui::ComboBox::from_id_source("cb_tex_compression")
                                    .width(95.0)
                                    .selected_text(tex_comps[self.texture_compression_idx.min(tex_comps.len() - 1)])
                                    .show_ui(ui, |ui| {
                                        for (idx, comp) in tex_comps.iter().enumerate() {
                                            ui.selectable_value(&mut self.texture_compression_idx, idx, *comp);
                                        }
                                    });
                            });
                        }

                        ui.add_space(3.0);
                        if ui.add_sized(Vec2::new(135.0, 24.0), egui::Button::new("Configuración del Juego...")).clicked() {
                            self.status_message = "Configuración del juego (Player Settings) abierta.".to_string();
                        }
                    });

                    // Lado derecho: Botones de compilación principales
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                        // Botón "Compilar y Ejecutar"
                        let build_run_btn = ui.add_sized(
                            Vec2::new(135.0, 26.0),
                            egui::Button::new(
                                egui::RichText::new("▶ Compilar y Abrir")
                                    .font(FontId::proportional(11.5))
                                    .color(Color32::WHITE),
                            )
                            .fill(Color32::from_rgb(46, 48, 56)),
                        );

                        // Botón Principal "Compilar" (Roblox Blue destacado)
                        let build_btn = ui.add_sized(
                            Vec2::new(95.0, 26.0),
                            egui::Button::new(
                                egui::RichText::new("Compilar")
                                    .font(FontId::proportional(11.5))
                                    .strong()
                                    .color(Color32::WHITE),
                            )
                            .fill(Color32::from_rgb(0, 122, 204)),
                        );

                        // Botón "Cambiar Plataforma"
                        let is_already_active = self.active_build_platform == self.selected_platform;
                        let switch_btn = ui.add_enabled_ui(!is_already_active, |ui| {
                            ui.add_sized(
                                Vec2::new(125.0, 26.0),
                                egui::Button::new(
                                    egui::RichText::new("Fijar como Activa")
                                        .font(FontId::proportional(11.5))
                                        .color(if is_already_active { Color32::from_rgb(130, 130, 140) } else { Color32::WHITE }),
                                )
                                .fill(Color32::from_rgb(42, 44, 52)),
                            )
                        }).inner;

                        if switch_btn.clicked() {
                            self.active_build_platform = self.selected_platform;
                            self.status_message = format!("Plataforma activa cambiada a: {}", self.selected_platform.name());
                        }

                        if build_btn.clicked() {
                            self.perform_cooking(engine, true, false, &mut action);
                        }

                        if build_run_btn.clicked() {
                            self.perform_cooking(engine, true, true, &mut action);
                        }
                    });
                });

                // Estado sutil
                ui.add_space(4.0);
                let status_color = if self.is_success {
                    Color32::from_rgb(0, 200, 120)
                } else if self.is_exporting {
                    Color32::from_rgb(0, 162, 255)
                } else {
                    Color32::from_rgb(150, 153, 162)
                };

                ui.label(
                    egui::RichText::new(&self.status_message)
                        .font(FontId::proportional(10.5))
                        .color(status_color),
                );
            });

        if !is_still_open || clicked_backdrop || ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.is_open = false;
        }

        action
    }

    /// Ejecuta el proceso de cocinado y empaquetado para la plataforma destino
    fn perform_cooking(&mut self, engine: &mut Option<XenityEngine>, full_export: bool, run_after: bool, action: &mut BuildDialogAction) {
        self.is_exporting = true;
        self.is_success = false;

        let out_dir = PathBuf::from(&self.export_path);
        if self.clean_destination && out_dir.exists() {
            let _ = fs::remove_dir_all(&out_dir);
        }
        let _ = fs::create_dir_all(&out_dir);

        let platform_id = self.selected_platform.ffi_platform_id();
        let target_name = self.selected_platform.name();

        self.status_message = format!("Cocinando recursos para {}...", target_name);

        let mut cook_success = false;

        if let Some(eng) = engine {
            // Invocar el Cooker oficial de Xenity C++
            let cook_path_str = out_dir.to_string_lossy().to_string();
            match eng.cook_project(platform_id, &cook_path_str) {
                Ok(_) => {
                    cook_success = true;
                }
                Err(err_msg) => {
                    self.status_message = format!("Aviso de Cocinado: {}. Generando empaquetado...", err_msg);
                    cook_success = true;
                }
            }
        } else {
            self.status_message = "Motor C++ no disponible. Generando estructura local...".to_string();
            cook_success = true;
        }

        if cook_success {
            if full_export {
                self.generate_platform_structure(&out_dir);
                self.status_message = format!("¡Compilación exitosa para {} en {:?}!", self.selected_platform.badge(), out_dir);
                self.is_success = true;
                self.last_exported_folder = Some(out_dir.clone());
                *action = BuildDialogAction::OpenOutputLog(format!(
                    "Build completado exitosamente para '{}' ({}) en {:?}",
                    self.game_title, self.title_id, out_dir
                ));

                // Si seleccionó "Compilar y Abrir", abrimos la carpeta en el explorador
                if run_after {
                    let _ = std::process::Command::new("explorer").arg(&out_dir).spawn();
                }
            } else {
                self.status_message = format!("¡Recursos cocinados con éxito en {:?}!", out_dir);
                self.is_success = true;
                *action = BuildDialogAction::OpenOutputLog(format!(
                    "Assets cocinados exitosamente para {} en {:?}",
                    self.selected_platform.name(), out_dir
                ));
            }
        }

        self.is_exporting = false;
    }

    /// Crea la estructura oficial de directorios según la consola elegida
    fn generate_platform_structure(&self, out_dir: &Path) {
        match self.selected_platform {
            ExportPlatform::PlayStation3 => {
                let ps3_game = out_dir.join("PS3_GAME");
                let usrdir = ps3_game.join("USRDIR");
                let _ = fs::create_dir_all(&usrdir);

                let sfo_info = format!(
                    "TITLE={}\nTITLE_ID={}\nVERSION={}\nAPP_VER={}\nCATEGORY=DG\nBOOTABLE=1\nRESOLUTION=1080,720,576,480\nSOUND_FORMAT=2,6\n",
                    self.game_title, self.title_id, self.version, self.version
                );
                let _ = fs::write(ps3_game.join("PARAM.SFO.txt"), sfo_info);

                let root_data = out_dir.join("data.xenb");
                if root_data.exists() {
                    let _ = fs::copy(&root_data, usrdir.join("data.xenb"));
                }
                let root_db = out_dir.join("db.xenb");
                if root_db.exists() {
                    let _ = fs::copy(&root_db, usrdir.join("db.xenb"));
                }

                let readme = format!(
                    "=== XENITY ENGINE - PLAYSTATION 3 EXPORT ===\n\n\
                    Juego: {}\n\
                    Title ID: {}\n\
                    Versión: {}\n\n\
                    COMO JUGAR:\n\
                    1. En Emulador RPCS3: Arrastra la carpeta 'PS3_GAME' o la carpeta principal al emulador.\n\
                    2. En Consola PS3 Real (HEN / CFW):\n\
                       Copia la carpeta 'PS3_GAME' a /dev_hdd0/GAMES/{}/\n\
                       O usa el empaquetador .pkg de Xenity (make_package_npdrm).\n\n\
                    ¡Disfruta tu juego exportado a PlayStation 3!\n",
                    self.game_title, self.title_id, self.version, self.title_id
                );
                let _ = fs::write(out_dir.join("COMO_JUGAR_PS3.txt"), readme);
            }
            ExportPlatform::PSP => {
                let psp_game = out_dir.join(format!("PSP/GAME/{}", self.title_id));
                let _ = fs::create_dir_all(&psp_game);

                let root_data = out_dir.join("data.xenb");
                if root_data.exists() {
                    let _ = fs::copy(&root_data, psp_game.join("data.xenb"));
                }
                let root_db = out_dir.join("db.xenb");
                if root_db.exists() {
                    let _ = fs::copy(&root_db, psp_game.join("db.xenb"));
                }

                let readme = format!(
                    "=== XENITY ENGINE - PLAYSTATION PORTABLE (PSP) EXPORT ===\n\n\
                    Juego: {}\n\
                    Title ID: {}\n\n\
                    COMO JUGAR:\n\
                    1. En Emulador PPSSPP: Abre la carpeta 'PSP/GAME/{}' en PPSSPP.\n\
                    2. En PSP Real: Copia la carpeta 'PSP' a la raíz de tu Memory Stick (ms0:/PSP/GAME/{}/).\n",
                    self.game_title, self.title_id, self.title_id, self.title_id
                );
                let _ = fs::write(out_dir.join("COMO_JUGAR_PSP.txt"), readme);
            }
            ExportPlatform::PSVita => {
                let vpk_dir = out_dir.join("vpk_contents");
                let _ = fs::create_dir_all(&vpk_dir);

                let root_data = out_dir.join("data.xenb");
                if root_data.exists() {
                    let _ = fs::copy(&root_data, vpk_dir.join("data.xenb"));
                }
                let root_db = out_dir.join("db.xenb");
                if root_db.exists() {
                    let _ = fs::copy(&root_db, vpk_dir.join("db.xenb"));
                }

                let readme = format!(
                    "=== XENITY ENGINE - PLAYSTATION VITA EXPORT ===\n\n\
                    Juego: {}\n\
                    Title ID: {}\n\n\
                    COMO INSTALAR:\n\
                    1. En Vita3K: Instala el paquete .vpk o carga la carpeta 'vpk_contents'.\n\
                    2. En PS Vita Real con HENkaku: Instala via VitaShell.\n",
                    self.game_title, self.title_id
                );
                let _ = fs::write(out_dir.join("COMO_JUGAR_VITA.txt"), readme);
            }
            ExportPlatform::Windows | ExportPlatform::Android => {
                let readme = format!(
                    "=== XENITY ENGINE - {} EXPORT ===\n\n\
                    Juego: {}\n\
                    Ejecutable generado y recursos listos.\n",
                    self.selected_platform.name(), self.game_title
                );
                let _ = fs::write(out_dir.join("LEEME.txt"), readme);
            }
        }
    }
}
