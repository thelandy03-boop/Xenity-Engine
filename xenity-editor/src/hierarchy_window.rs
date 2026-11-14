use eframe::egui::{self, Color32, CursorIcon, FontId, Pos2, Rect, Rounding, Sense, Stroke, Vec2};
use crate::icons;
use crate::unity_style::*;
use crate::roblox_icon_bytes;

#[derive(Clone, PartialEq, Eq)]
pub enum RobloxClass {
    Workspace,
    Model,
    Part,
    MeshPart,
    Camera,
    Light,
    Terrain,
    Script,
    Sound,
    Humanoid,
    Folder,
}

#[derive(Clone)]
pub struct ExplorerItem {
    pub id: usize,
    pub name: String,
    pub class: RobloxClass,
    pub is_active: bool,
    pub is_expanded: bool,
    pub children: Vec<ExplorerItem>,
}

pub struct RobloxIconTextures {
    pub workspace: egui::TextureHandle,
    pub model: egui::TextureHandle,
    pub part: egui::TextureHandle,
    pub meshpart: egui::TextureHandle,
    pub camera: egui::TextureHandle,
    pub lighting: egui::TextureHandle,
    pub terrain: egui::TextureHandle,
    pub script: egui::TextureHandle,
    pub sound: egui::TextureHandle,
    pub humanoid: egui::TextureHandle,
    pub folder: egui::TextureHandle,
}

impl RobloxIconTextures {
    pub fn new(ctx: &egui::Context) -> Self {
        let opt = egui::TextureOptions::NEAREST;
        Self {
            workspace: ctx.load_texture("rbx_workspace", egui::ColorImage::from_rgba_unmultiplied([16, 16], &roblox_icon_bytes::WORKSPACE_RGBA), opt),
            model: ctx.load_texture("rbx_model", egui::ColorImage::from_rgba_unmultiplied([16, 16], &roblox_icon_bytes::MODEL_RGBA), opt),
            part: ctx.load_texture("rbx_part", egui::ColorImage::from_rgba_unmultiplied([16, 16], &roblox_icon_bytes::PART_RGBA), opt),
            meshpart: ctx.load_texture("rbx_meshpart", egui::ColorImage::from_rgba_unmultiplied([16, 16], &roblox_icon_bytes::MESHPART_RGBA), opt),
            camera: ctx.load_texture("rbx_camera", egui::ColorImage::from_rgba_unmultiplied([16, 16], &roblox_icon_bytes::CAMERA_RGBA), opt),
            lighting: ctx.load_texture("rbx_lighting", egui::ColorImage::from_rgba_unmultiplied([16, 16], &roblox_icon_bytes::LIGHTING_RGBA), opt),
            terrain: ctx.load_texture("rbx_terrain", egui::ColorImage::from_rgba_unmultiplied([16, 16], &roblox_icon_bytes::TERRAIN_RGBA), opt),
            script: ctx.load_texture("rbx_script", egui::ColorImage::from_rgba_unmultiplied([16, 16], &roblox_icon_bytes::SCRIPT_RGBA), opt),
            sound: ctx.load_texture("rbx_sound", egui::ColorImage::from_rgba_unmultiplied([16, 16], &roblox_icon_bytes::SOUND_RGBA), opt),
            humanoid: ctx.load_texture("rbx_humanoid", egui::ColorImage::from_rgba_unmultiplied([16, 16], &roblox_icon_bytes::HUMANOID_RGBA), opt),
            folder: ctx.load_texture("rbx_folder", egui::ColorImage::from_rgba_unmultiplied([16, 16], &roblox_icon_bytes::FOLDER_RGBA), opt),
        }
    }
}

pub struct SceneHierarchyState {
    pub search_query: String,
    pub selected_id: Option<usize>,
    pub items: Vec<ExplorerItem>,
    pub next_id: usize,
    pub textures: Option<RobloxIconTextures>,
}

impl Default for SceneHierarchyState {
    fn default() -> Self {
        Self {
            search_query: String::new(),
            selected_id: Some(3), // Player seleccionado por defecto
            next_id: 100,
            textures: None,
            items: vec![
                ExplorerItem {
                    id: 10,
                    name: "Workspace".into(),
                    class: RobloxClass::Workspace,
                    is_active: true,
                    is_expanded: true,
                    children: vec![
                        ExplorerItem {
                            id: 1,
                            name: "Camera".into(),
                            class: RobloxClass::Camera,
                            is_active: true,
                            is_expanded: false,
                            children: vec![],
                        },
                        ExplorerItem {
                            id: 2,
                            name: "Terrain".into(),
                            class: RobloxClass::Terrain,
                            is_active: true,
                            is_expanded: false,
                            children: vec![],
                        },
                        ExplorerItem {
                            id: 3,
                            name: "Player".into(),
                            class: RobloxClass::Model,
                            is_active: true,
                            is_expanded: true,
                            children: vec![
                                ExplorerItem {
                                    id: 31,
                                    name: "Humanoid".into(),
                                    class: RobloxClass::Humanoid,
                                    is_active: true,
                                    is_expanded: false,
                                    children: vec![],
                                },
                                ExplorerItem {
                                    id: 32,
                                    name: "Weapon_Holder".into(),
                                    class: RobloxClass::Model,
                                    is_active: true,
                                    is_expanded: true,
                                    children: vec![
                                        ExplorerItem {
                                            id: 321,
                                            name: "Pistol_Mesh".into(),
                                            class: RobloxClass::MeshPart,
                                            is_active: true,
                                            is_expanded: false,
                                            children: vec![],
                                        },
                                    ],
                                },
                            ],
                        },
                        ExplorerItem {
                            id: 4,
                            name: "Environment".into(),
                            class: RobloxClass::Model,
                            is_active: true,
                            is_expanded: true,
                            children: vec![
                                ExplorerItem {
                                    id: 41,
                                    name: "Ground_Plane".into(),
                                    class: RobloxClass::Part,
                                    is_active: true,
                                    is_expanded: false,
                                    children: vec![],
                                },
                                ExplorerItem {
                                    id: 42,
                                    name: "Tree_01".into(),
                                    class: RobloxClass::Model,
                                    is_active: true,
                                    is_expanded: true,
                                    children: vec![
                                        ExplorerItem {
                                            id: 421,
                                            name: "Trunk".into(),
                                            class: RobloxClass::Part,
                                            is_active: true,
                                            is_expanded: false,
                                            children: vec![],
                                        },
                                        ExplorerItem {
                                            id: 422,
                                            name: "Leaves".into(),
                                            class: RobloxClass::Part,
                                            is_active: true,
                                            is_expanded: false,
                                            children: vec![],
                                        },
                                    ],
                                },
                            ],
                        },
                    ],
                },
                ExplorerItem {
                    id: 20,
                    name: "Lighting".into(),
                    class: RobloxClass::Light,
                    is_active: true,
                    is_expanded: true,
                    children: vec![
                        ExplorerItem {
                            id: 21,
                            name: "DirectionalLight".into(),
                            class: RobloxClass::Light,
                            is_active: true,
                            is_expanded: false,
                            children: vec![],
                        },
                        ExplorerItem {
                            id: 22,
                            name: "Sky".into(),
                            class: RobloxClass::Part,
                            is_active: true,
                            is_expanded: false,
                            children: vec![],
                        },
                    ],
                },
                ExplorerItem {
                    id: 30,
                    name: "StarterPack".into(),
                    class: RobloxClass::Folder,
                    is_active: true,
                    is_expanded: true,
                    children: vec![
                        ExplorerItem {
                            id: 31,
                            name: "PlayerController".into(),
                            class: RobloxClass::Script,
                            is_active: true,
                            is_expanded: false,
                            children: vec![],
                        },
                    ],
                },
                ExplorerItem {
                    id: 40,
                    name: "ReplicatedStorage".into(),
                    class: RobloxClass::Folder,
                    is_active: true,
                    is_expanded: true,
                    children: vec![
                        ExplorerItem {
                            id: 41,
                            name: "BulletVFX".into(),
                            class: RobloxClass::MeshPart,
                            is_active: true,
                            is_expanded: false,
                            children: vec![],
                        },
                        ExplorerItem {
                            id: 42,
                            name: "HitSound".into(),
                            class: RobloxClass::Sound,
                            is_active: true,
                            is_expanded: false,
                            children: vec![],
                        },
                    ],
                },
            ],
        }
    }
}

// Colores exactos de Roblox Studio Dark Theme
pub const STUDIO_BG: Color32 = Color32::from_rgb(37, 37, 38);          // #252526
pub const STUDIO_TREE_BG: Color32 = Color32::from_rgb(30, 30, 30);     // #1E1E1E
pub const STUDIO_SELECTION: Color32 = Color32::from_rgb(0, 122, 204);   // #007ACC (Azul oficial Studio)
pub const STUDIO_HOVER: Color32 = Color32::from_rgb(46, 46, 50);       // #2E2E32
pub const STUDIO_BORDER: Color32 = Color32::from_rgb(45, 45, 48);      // #2D2D30
pub const STUDIO_TEXT: Color32 = Color32::from_rgb(204, 204, 204);     // #CCCCCC
pub const STUDIO_INPUT_BG: Color32 = Color32::from_rgb(26, 26, 26);    // #1A1A1A

impl SceneHierarchyState {
    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<String> {
        let mut clicked_object_name = None;

        // Cargar texturas oficiales de Roblox Studio si no están inicializadas
        if self.textures.is_none() {
            self.textures = Some(RobloxIconTextures::new(ui.ctx()));
        }

        // 1. PESTAÑA SUPERIOR [ Explorer ]
        let tab_h = 26.0;
        let (tab_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), tab_h), Sense::hover());
        ui.painter().rect_filled(tab_rect, 0.0, STUDIO_BG);

        let active_tab_rect = Rect::from_min_size(tab_rect.min, Vec2::new(96.0, tab_h));
        ui.painter().rect_filled(active_tab_rect, Rounding { nw: 3.0, ne: 3.0, sw: 0.0, se: 0.0 }, STUDIO_TREE_BG);
        ui.painter().line_segment(
            [Pos2::new(active_tab_rect.min.x, active_tab_rect.min.y), Pos2::new(active_tab_rect.max.x, active_tab_rect.min.y)],
            Stroke::new(2.0, STUDIO_SELECTION),
        );
        ui.painter().text(
            active_tab_rect.center(),
            egui::Align2::CENTER_CENTER,
            "Explorer",
            FontId::proportional(11.5),
            Color32::WHITE,
        );

        let menu_pos = Pos2::new(tab_rect.max.x - 12.0, tab_rect.center().y);
        icons::kebab_menu(ui.painter(), menu_pos, Color32::from_rgb(140, 140, 140));

        ui.painter().line_segment(
            [Pos2::new(tab_rect.min.x, tab_rect.max.y), Pos2::new(tab_rect.max.x, tab_rect.max.y)],
            Stroke::new(1.0, STUDIO_BORDER),
        );

        // 2. SUB-BARRA DE BÚSQUEDA (Filter Workspace)
        let toolbar_h = 32.0;
        let (toolbar_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), toolbar_h), Sense::hover());
        ui.painter().rect_filled(toolbar_rect, 0.0, STUDIO_BG);

        let search_h = 22.0;
        let search_y = toolbar_rect.min.y + (toolbar_h - search_h) * 0.5;
        let search_rect = Rect::from_min_size(
            Pos2::new(toolbar_rect.min.x + 8.0, search_y),
            Vec2::new(toolbar_rect.width() - 16.0, search_h),
        );

        ui.painter().rect_filled(search_rect, Rounding::same(4.0), STUDIO_INPUT_BG);
        ui.painter().rect_stroke(search_rect, Rounding::same(4.0), Stroke::new(1.0, STUDIO_BORDER));

        let mag_center = Pos2::new(search_rect.min.x + 12.0, search_rect.center().y);
        icons::magnifying_glass(ui.painter(), mag_center, 3.8, Color32::from_rgb(140, 140, 140));

        let clear_w = if self.search_query.is_empty() { 0.0 } else { 18.0 };
        let text_rect = Rect::from_min_max(
            Pos2::new(search_rect.min.x + 24.0, search_rect.min.y),
            Pos2::new(search_rect.max.x - 6.0 - clear_w, search_rect.max.y),
        );

        let mut edit_ui = ui.child_ui(text_rect, egui::Layout::left_to_right(egui::Align::Center), None);
        edit_ui.add(
            egui::TextEdit::singleline(&mut self.search_query)
                .hint_text("Filter Workspace (Ctrl+Shift+X)")
                .frame(false)
                .font(FontId::proportional(11.5))
                .desired_width(text_rect.width()),
        );

        if !self.search_query.is_empty() {
            let clear_center = Pos2::new(search_rect.max.x - 10.0, search_rect.center().y);
            let clear_rect = Rect::from_center_size(clear_center, Vec2::new(14.0, 14.0));
            let clear_resp = ui.interact(clear_rect, ui.id().with("clear_explorer_search"), Sense::click());
            let clear_col = if clear_resp.hovered() { Color32::WHITE } else { Color32::from_rgb(150, 150, 150) };
            ui.painter().text(clear_center, egui::Align2::CENTER_CENTER, "✕", FontId::proportional(10.0), clear_col);
            if clear_resp.clicked() {
                self.search_query.clear();
            }
        }

        ui.painter().line_segment(
            [Pos2::new(toolbar_rect.min.x, toolbar_rect.max.y), Pos2::new(toolbar_rect.max.x, toolbar_rect.max.y)],
            Stroke::new(1.0, STUDIO_BORDER),
        );

        // 3. ÁREA DEL ÁRBOL
        let tree_rect = Rect::from_min_size(ui.cursor().min, ui.available_size());
        ui.painter().rect_filled(tree_rect, 0.0, STUDIO_TREE_BG);

        let textures_ref = self.textures.as_ref();

        egui::ScrollArea::vertical()
            .id_salt("roblox_explorer_scroll")
            .show(ui, |ui| {
                ui.add_space(2.0);
                let mut items = self.items.clone();
                for item in &mut items {
                    Self::draw_explorer_node(
                        ui,
                        item,
                        0,
                        &mut self.selected_id,
                        &self.search_query,
                        &mut clicked_object_name,
                        textures_ref,
                    );
                }
                self.items = items;
            });

        clicked_object_name
    }

    fn draw_explorer_node(
        ui: &mut egui::Ui,
        node: &mut ExplorerItem,
        depth: usize,
        selected_id: &mut Option<usize>,
        search_filter: &str,
        clicked_name: &mut Option<String>,
        textures: Option<&RobloxIconTextures>,
    ) {
        let is_selected = *selected_id == Some(node.id);
        let has_children = !node.children.is_empty();
        let avail_w = ui.available_width();
        let row_h = 22.0;

        let (row_rect, row_resp) = ui.allocate_exact_size(Vec2::new(avail_w, row_h), Sense::click());

        // Highlight oficial de selección de Studio: #007ACC
        if is_selected {
            ui.painter().rect_filled(row_rect, 0.0, STUDIO_SELECTION);
        } else if row_resp.hovered() {
            ui.painter().rect_filled(row_rect, 0.0, STUDIO_HOVER);
        }

        let indent_x = row_rect.min.x + (depth as f32 * 16.0) + 6.0;

        // Flecha Foldout › / ⌄
        if has_children {
            let arrow_pos = Pos2::new(indent_x + 3.0, row_rect.center().y);
            let arrow_rect = Rect::from_center_size(arrow_pos, Vec2::new(14.0, 14.0));
            let arrow_resp = ui.interact(arrow_rect, ui.id().with(node.id).with("arrow"), Sense::click());
            if arrow_resp.clicked() {
                node.is_expanded = !node.is_expanded;
            }
            let arrow_col = if arrow_resp.hovered() { Color32::WHITE } else { Color32::from_rgb(160, 160, 160) };
            icons::foldout_arrow(ui.painter(), arrow_pos, node.is_expanded, arrow_col);
        }

        // ICONO OFICIAL DE ROBLOX STUDIO EXTRAÍDO DE ClassImages.PNG (16x16px perfecto)
        let icon_rect = Rect::from_center_size(Pos2::new(indent_x + 19.0, row_rect.center().y), Vec2::new(16.0, 16.0));
        if let Some(texs) = textures {
            let tex = match node.class {
                RobloxClass::Workspace => &texs.workspace,
                RobloxClass::Model => &texs.model,
                RobloxClass::Part => &texs.part,
                RobloxClass::MeshPart => &texs.meshpart,
                RobloxClass::Camera => &texs.camera,
                RobloxClass::Light => &texs.lighting,
                RobloxClass::Terrain => &texs.terrain,
                RobloxClass::Script => &texs.script,
                RobloxClass::Sound => &texs.sound,
                RobloxClass::Humanoid => &texs.humanoid,
                RobloxClass::Folder => &texs.folder,
            };
            ui.painter().image(
                tex.id(),
                icon_rect,
                Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                Color32::WHITE,
            );
        }

        // Texto del nombre de la instancia
        let text_color = if is_selected {
            Color32::WHITE
        } else {
            STUDIO_TEXT
        };

        ui.painter().text(
            Pos2::new(indent_x + 32.0, row_rect.center().y),
            egui::Align2::LEFT_CENTER,
            &node.name,
            FontId::proportional(12.0),
            text_color,
        );

        // Botón circular [+] (Insert Object) al hacer hover
        if row_resp.hovered() || is_selected {
            let plus_pos = Pos2::new(row_rect.max.x - 14.0, row_rect.center().y);
            let plus_rect = Rect::from_center_size(plus_pos, Vec2::new(15.0, 15.0));
            let plus_resp = ui.interact(plus_rect, ui.id().with(node.id).with("plus_insert"), Sense::click());

            let plus_bg = if plus_resp.hovered() {
                Color32::from_rgb(65, 65, 75)
            } else {
                Color32::from_rgb(48, 48, 56)
            };
            ui.painter().circle_filled(plus_pos, 7.0, plus_bg);
            ui.painter().circle_stroke(plus_pos, 7.0, Stroke::new(1.0, Color32::from_rgb(75, 75, 85)));

            let stroke = Stroke::new(1.3, if plus_resp.hovered() { Color32::WHITE } else { Color32::from_rgb(210, 210, 210) });
            ui.painter().line_segment([Pos2::new(plus_pos.x - 3.0, plus_pos.y), Pos2::new(plus_pos.x + 3.0, plus_pos.y)], stroke);
            ui.painter().line_segment([Pos2::new(plus_pos.x, plus_pos.y - 3.0), Pos2::new(plus_pos.x, plus_pos.y + 3.0)], stroke);

            plus_resp.on_hover_text("Insert Object");
        }

        if row_resp.clicked() {
            *selected_id = Some(node.id);
            *clicked_name = Some(node.name.clone());
        }

        // Hijos
        if has_children && node.is_expanded {
            for child in &mut node.children {
                Self::draw_explorer_node(ui, child, depth + 1, selected_id, search_filter, clicked_name, textures);
            }
        }
    }
}
