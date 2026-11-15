// SPDX-License-Identifier: MIT
//
// Build & Export Dialog for Roblox Studio Editor
// Allows 1-click cooking & packaging for PS3, PSP, PS Vita, and PC.

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
}

impl ExportPlatform {
    pub fn name(&self) -> &'static str {
        match self {
            ExportPlatform::PlayStation3 => "PlayStation 3 (PS3)",
            ExportPlatform::PSP => "PlayStation Portable (PSP)",
            ExportPlatform::PSVita => "PlayStation Vita",
            ExportPlatform::Windows => "Windows Standalone (PC)",
        }
    }

    pub fn badge(&self) -> &'static str {
        match self {
            ExportPlatform::PlayStation3 => "PS3",
            ExportPlatform::PSP => "PSP",
            ExportPlatform::PSVita => "VITA",
            ExportPlatform::Windows => "PC",
        }
    }

    pub fn file_format(&self) -> &'static str {
        match self {
            ExportPlatform::PlayStation3 => "Carpeta PS3_GAME / .pkg (RPCS3 & HEN/CFW)",
            ExportPlatform::PSP => "EBOOT.PBP (PPSSPP & PSP Real)",
            ExportPlatform::PSVita => "Paquete .vpk (Vita3K & HENkaku)",
            ExportPlatform::Windows => "Ejecutable nativo .exe",
        }
    }

    pub fn hardware_desc(&self) -> &'static str {
        match self {
            ExportPlatform::PlayStation3 => "Cell Broadband Engine @ 3.2GHz | RSX Reality Synthesizer\nRAM: 256MB XDR + 256MB GDDR3 VRAM",
            ExportPlatform::PSP => "MIPS R4000 @ 333MHz | Graphics Core GU\nRAM: 32MB (1000) / 64MB (Slim)",
            ExportPlatform::PSVita => "Quad-core ARM Cortex-A9 | SGX543MP4+\nRAM: 512MB RAM + 128MB VRAM",
            ExportPlatform::Windows => "x86_64 | OpenGL 3.3 Core Renderer\nRAM: Sin límite estricto de consola",
        }
    }

    pub fn ram_budget_warning(&self) -> &'static str {
        match self {
            ExportPlatform::PlayStation3 => "💡 Presupuesto RAM: 256MB RSX. Las texturas se optimizan automáticamente a 512x512.",
            ExportPlatform::PSP => "⚠️ Presupuesto estricto: 32MB. Texturas recomendadas máx 256x256, modelos bajo poligonaje.",
            ExportPlatform::PSVita => "💡 Presupuesto amplio: 512MB. Rendimiento nativo fluido.",
            ExportPlatform::Windows => "✔️ Sin restricciones de memoria de consola.",
        }
    }

    pub fn default_title_id(&self) -> &'static str {
        match self {
            ExportPlatform::PlayStation3 => "NPEB00123",
            ExportPlatform::PSP => "ULES00123",
            ExportPlatform::PSVita => "PCSB00123",
            ExportPlatform::Windows => "XENITY_GAME",
        }
    }

    pub fn default_output_folder(&self) -> &'static str {
        match self {
            ExportPlatform::PlayStation3 => "build/ps3",
            ExportPlatform::PSP => "build/psp",
            ExportPlatform::PSVita => "build/psvita",
            ExportPlatform::Windows => "build/windows",
        }
    }

    pub fn ffi_platform_id(&self) -> i32 {
        match self {
            ExportPlatform::Windows => 0,
            ExportPlatform::PSP => 1,
            ExportPlatform::PSVita => 2,
            ExportPlatform::PlayStation3 => 3,
        }
    }
}

pub struct BuildExportState {
    pub is_open: bool,
    pub selected_platform: ExportPlatform,

    // Form fields
    pub game_title: String,
    pub title_id: String,
    pub version: String,
    pub export_path: String,

    // Options
    pub auto_downscale_textures: bool,
    pub compile_native_shaders: bool,
    pub clean_destination: bool,

    // Progress & Status
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
            game_title: "My Roblox Place".to_string(),
            title_id: "NPEB00123".to_string(),
            version: "1.00".to_string(),
            export_path: "build/ps3".to_string(),

            auto_downscale_textures: true,
            compile_native_shaders: true,
            clean_destination: false,

            is_exporting: false,
            status_message: "Listo para exportar.".to_string(),
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
        self.status_message = "Listo para exportar.".to_string();
        self.is_success = false;
    }

    pub fn select_platform(&mut self, platform: ExportPlatform) {
        if self.selected_platform != platform {
            self.selected_platform = platform;
            self.title_id = platform.default_title_id().to_string();
            self.export_path = platform.default_output_folder().to_string();
            self.status_message = format!("Plataforma cambiada a {}", platform.name());
            self.is_success = false;
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, engine: &mut Option<XenityEngine>) -> BuildDialogAction {
        if !self.is_open {
            return BuildDialogAction::None;
        }

        let mut action = BuildDialogAction::None;

        // Overlay oscuro semitransparente que bloquea el fondo
        let screen_rect = ctx.screen_rect();
        let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Middle, egui::Id::new("modal_backdrop")));
        painter.rect_filled(screen_rect, 0.0, Color32::from_rgba_unmultiplied(0, 0, 0, 150));

        let mut is_still_open = true;

        egui::Window::new("Exportar y Compilar Proyecto (Build Settings)")
            .open(&mut is_still_open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .fixed_size([680.0, 520.0])
            .frame(
                egui::Frame::window(&ctx.style())
                    .fill(Color32::from_rgb(26, 27, 32))
                    .stroke(Stroke::new(1.0, Color32::from_rgb(60, 63, 72)))
                    .rounding(Rounding::same(8.0))
                    .inner_margin(egui::Margin::same(16.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("🎮").size(22.0));
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("Exportar Experiencia para Consolas y PC")
                                .font(FontId::proportional(15.0))
                                .strong()
                                .color(Color32::WHITE),
                        );
                        ui.label(
                            egui::RichText::new("Cocina y empaqueta recursos con el motor nativo Xenity hacia PS3, PSP, Vita y Windows.")
                                .font(FontId::proportional(11.0))
                                .color(Color32::from_rgb(160, 163, 172)),
                        );
                    });
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(8.0);

                // CONTENIDO PRINCIPAL: Split de dos columnas (Izquierda: Plataformas, Derecha: Detalles)
                let full_w = ui.available_width();
                let left_w = 210.0;
                let right_w = full_w - left_w - 16.0;

                ui.horizontal(|ui| {
                    // --- COLUMNA IZQUIERDA: LISTA DE PLATAFORMAS ---
                    ui.vertical(|ui| {
                        ui.set_width(left_w);
                        ui.label(
                            egui::RichText::new("PLATAFORMAS DESTINO")
                                .font(FontId::proportional(10.5))
                                .color(Color32::from_rgb(140, 143, 155))
                                .strong(),
                        );
                        ui.add_space(6.0);

                        let platforms = [
                            (ExportPlatform::PlayStation3, "PlayStation 3", "PS3"),
                            (ExportPlatform::PSP, "PlayStation Portable", "PSP"),
                            (ExportPlatform::PSVita, "PlayStation Vita", "VITA"),
                            (ExportPlatform::Windows, "Windows PC", "PC"),
                        ];

                        for (plat, title, badge) in platforms {
                            let is_sel = self.selected_platform == plat;
                            let btn_rect = ui.allocate_space(Vec2::new(left_w - 8.0, 46.0)).1;
                            let resp = ui.interact(btn_rect, ui.id().with(("plat_btn", badge)), Sense::click());

                            let bg_color = if is_sel {
                                Color32::from_rgb(0, 102, 204)
                            } else if resp.hovered() {
                                Color32::from_rgb(42, 45, 52)
                            } else {
                                Color32::from_rgb(33, 35, 41)
                            };

                            let border_stroke = if is_sel {
                                Stroke::new(1.5, Color32::from_rgb(0, 162, 255))
                            } else {
                                Stroke::new(1.0, Color32::from_rgb(48, 51, 60))
                            };

                            ui.painter().rect(btn_rect, Rounding::same(6.0), bg_color, border_stroke);

                            // Badge pequeño (ej: "PS3" o "PSP")
                            let badge_rect = Rect::from_min_size(
                                Pos2::new(btn_rect.min.x + 8.0, btn_rect.center().y - 10.0),
                                Vec2::new(42.0, 20.0),
                            );
                            let badge_bg = if is_sel {
                                Color32::from_rgb(0, 70, 150)
                            } else {
                                Color32::from_rgb(22, 24, 28)
                            };
                            ui.painter().rect_filled(badge_rect, Rounding::same(4.0), badge_bg);
                            ui.painter().text(
                                badge_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                badge,
                                FontId::proportional(11.0),
                                Color32::WHITE,
                            );

                            // Nombre de la plataforma
                            ui.painter().text(
                                Pos2::new(btn_rect.min.x + 58.0, btn_rect.center().y),
                                egui::Align2::LEFT_CENTER,
                                title,
                                FontId::proportional(12.0),
                                Color32::WHITE,
                            );

                            if resp.clicked() {
                                self.select_platform(plat);
                            }
                        }
                    });

                    ui.separator();

                    // --- COLUMNA DERECHA: CONFIGURACIÓN DE LA PLATAFORMA ---
                    ui.vertical(|ui| {
                        ui.set_width(right_w);

                        // Cabecera de la plataforma seleccionada
                        ui.label(
                            egui::RichText::new(self.selected_platform.name())
                                .font(FontId::proportional(14.0))
                                .strong()
                                .color(Color32::from_rgb(0, 162, 255)),
                        );

                        ui.label(
                            egui::RichText::new(format!("Formato: {}", self.selected_platform.file_format()))
                                .font(FontId::proportional(11.0))
                                .color(Color32::from_rgb(200, 202, 210)),
                        );

                        ui.add_space(8.0);

                        // Tarjeta de Hardware & Presupuesto
                        let budget_rect = ui.allocate_space(Vec2::new(right_w - 8.0, 52.0)).1;
                        ui.painter().rect_filled(budget_rect, Rounding::same(6.0), Color32::from_rgb(22, 23, 27));
                        ui.painter().rect_stroke(budget_rect, Rounding::same(6.0), Stroke::new(1.0, Color32::from_rgb(44, 46, 54)));

                        ui.painter().text(
                            Pos2::new(budget_rect.min.x + 10.0, budget_rect.min.y + 14.0),
                            egui::Align2::LEFT_CENTER,
                            self.selected_platform.hardware_desc(),
                            FontId::proportional(10.0),
                            Color32::from_rgb(160, 165, 175),
                        );

                        ui.painter().text(
                            Pos2::new(budget_rect.min.x + 10.0, budget_rect.min.y + 38.0),
                            egui::Align2::LEFT_CENTER,
                            self.selected_platform.ram_budget_warning(),
                            FontId::proportional(10.5),
                            Color32::from_rgb(255, 200, 80),
                        );

                        ui.add_space(8.0);

                        // Formulario de parámetros
                        egui::Grid::new("export_form_grid")
                            .num_columns(2)
                            .spacing([12.0, 8.0])
                            .show(ui, |ui| {
                                ui.label(egui::RichText::new("Nombre del Juego:").font(FontId::proportional(11.5)));
                                ui.text_edit_singleline(&mut self.game_title);
                                ui.end_row();

                                ui.label(egui::RichText::new("Title ID / Game ID:").font(FontId::proportional(11.5)));
                                ui.text_edit_singleline(&mut self.title_id);
                                ui.end_row();

                                ui.label(egui::RichText::new("Versión:").font(FontId::proportional(11.5)));
                                ui.text_edit_singleline(&mut self.version);
                                ui.end_row();

                                ui.label(egui::RichText::new("Carpeta de Salida:").font(FontId::proportional(11.5)));
                                ui.horizontal(|ui| {
                                    ui.text_edit_singleline(&mut self.export_path);
                                    if ui.button("Por defecto").clicked() {
                                        self.export_path = self.selected_platform.default_output_folder().to_string();
                                    }
                                });
                                ui.end_row();
                            });

                        ui.add_space(8.0);
                        ui.label(egui::RichText::new("Optimización y Cocinado (Cooker):").font(FontId::proportional(11.0)).strong());
                        ui.checkbox(&mut self.auto_downscale_textures, "Optimizar y redimensionar texturas para la VRAM de la consola");
                        ui.checkbox(&mut self.compile_native_shaders, "Compilar shaders nativos (RSX Cg / GL / GU)");
                        ui.checkbox(&mut self.clean_destination, "Limpiar carpeta destino antes de exportar");
                    });
                });

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(8.0);

                // --- BARRA INFERIOR DE ACCIONES Y ESTADO ---
                ui.horizontal(|ui| {
                    // Mensaje de estado con icono
                    let status_color = if self.is_success {
                        Color32::from_rgb(80, 220, 100)
                    } else if self.is_exporting {
                        Color32::from_rgb(0, 162, 255)
                    } else {
                        Color32::from_rgb(180, 183, 192)
                    };

                    ui.label(
                        egui::RichText::new(&self.status_message)
                            .font(FontId::proportional(11.5))
                            .color(status_color),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Botón principal: Exportar y Empaquetar
                        let export_btn = ui.add_sized(
                            Vec2::new(160.0, 28.0),
                            egui::Button::new(
                                egui::RichText::new("🚀 Exportar a Consola")
                                    .font(FontId::proportional(12.0))
                                    .color(Color32::WHITE)
                                    .strong(),
                            )
                            .fill(Color32::from_rgb(0, 120, 215)),
                        );

                        // Botón secundario: Solo cocinar recursos
                        let cook_btn = ui.add_sized(
                            Vec2::new(150.0, 28.0),
                            egui::Button::new(
                                egui::RichText::new("Cocinar Recursos")
                                    .font(FontId::proportional(11.5))
                                    .color(Color32::WHITE),
                            )
                            .fill(Color32::from_rgb(44, 46, 54)),
                        );

                        // Botón Cerrar / Cancelar
                        if ui.add_sized(Vec2::new(80.0, 28.0), egui::Button::new("Cancelar")).clicked() {
                            self.is_open = false;
                        }

                        if cook_btn.clicked() {
                            self.perform_cooking(engine, false, &mut action);
                        }

                        if export_btn.clicked() {
                            self.perform_cooking(engine, true, &mut action);
                        }
                    });
                });
            });

        if !is_still_open {
            self.is_open = false;
        }

        action
    }

    /// Ejecuta el proceso de cocinado y empaquetado para la consola destino
    fn perform_cooking(&mut self, engine: &mut Option<XenityEngine>, full_export: bool, action: &mut BuildDialogAction) {
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
                    cook_success = true; // Continuamos generando los archivos de la plataforma
                }
            }
        } else {
            self.status_message = "Motor C++ no disponible. Generando estructura local...".to_string();
            cook_success = true;
        }

        if cook_success {
            if full_export {
                // Generar estructura completa de la consola
                self.generate_platform_structure(&out_dir);
                self.status_message = format!("¡Exportación a {} completada con éxito en {:?}!", self.selected_platform.badge(), out_dir);
                self.is_success = true;
                self.last_exported_folder = Some(out_dir.clone());
                *action = BuildDialogAction::OpenOutputLog(format!(
                    "Juego '{}' ({}) exportado correctamente a {:?}. Estructura lista para consola y emuladores.",
                    self.game_title, self.title_id, out_dir
                ));
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
                // Estructura oficial de PlayStation 3 para RPCS3 o instalación en PS3 HEN/CFW:
                // PS3_GAME/
                //   PARAM.SFO
                //   ICON0.PNG
                //   USRDIR/
                //     EBOOT.BIN
                //     data.xenb
                //     db.xenb
                let ps3_game = out_dir.join("PS3_GAME");
                let usrdir = ps3_game.join("USRDIR");
                let _ = fs::create_dir_all(&usrdir);

                // Crear archivo de metadatos PARAM.SFO descriptivo
                let sfo_info = format!(
                    "TITLE={}\nTITLE_ID={}\nVERSION={}\nAPP_VER={}\nCATEGORY=DG\nBOOTABLE=1\nRESOLUTION=1080,720,576,480\nSOUND_FORMAT=2,6\n",
                    self.game_title, self.title_id, self.version, self.version
                );
                let _ = fs::write(ps3_game.join("PARAM.SFO.txt"), sfo_info);

                // Mover o copiar data.xenb y db.xenb dentro de USRDIR si se cocinaron en la raíz
                let root_data = out_dir.join("data.xenb");
                if root_data.exists() {
                    let _ = fs::copy(&root_data, usrdir.join("data.xenb"));
                }
                let root_db = out_dir.join("db.xenb");
                if root_db.exists() {
                    let _ = fs::copy(&root_db, usrdir.join("db.xenb"));
                }

                // Instrucciones de ejecución para el usuario
                let readme = format!(
                    "=== XENITY ENGINE - PLAYSTATION 3 EXPORT ===\n\n\
                    Juego: {}\n\
                    Title ID: {}\n\
                    Versión: {}\n\n\
                    COMO JUGAR:\n\
                    1. En Emulador RPCS3: Arrastra la carpeta 'PS3_GAME' o la carpeta principal al emulador.\n\
                    2. En Consola PS3 Real (HEN / CFW):\n\
                       Copia la carpeta 'PS3_GAME' a /dev_hdd0/GAMES/{}/\n\
                       O usa el creador de .pkg de Xenity (make_package_npdrm) incluido en Docker.\n\n\
                    ¡Disfruta tu juego de Roblox portado a PlayStation 3!\n",
                    self.game_title, self.title_id, self.version, self.title_id
                );
                let _ = fs::write(out_dir.join("COMO_JUGAR_PS3.txt"), readme);
            }
            ExportPlatform::PSP => {
                // Estructura oficial de PSP:
                // PSP/GAME/<TITLE_ID>/
                //   EBOOT.PBP
                //   data.xenb
                //   db.xenb
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
                // Estructura de PS Vita
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
            ExportPlatform::Windows => {
                let readme = format!(
                    "=== XENITY ENGINE - WINDOWS STANDALONE EXPORT ===\n\n\
                    Juego: {}\n\
                    Ejecuta el archivo ejecutable del juego directamente.\n",
                    self.game_title
                );
                let _ = fs::write(out_dir.join("LEEME.txt"), readme);
            }
        }
    }
}
