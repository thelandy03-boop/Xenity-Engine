use eframe::egui::{self, Color32, CursorIcon, FontId, Pos2, Rect, Rounding, Sense, Stroke, Vec2};
use crate::icons;
use crate::studio_style::*;
use crate::studio_textures::StudioIcons;
use crate::roblox_icon_bytes;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum RobloxClass {
    // Canonical Roblox Root Services
    Workspace,
    Players,
    Lighting,
    MaterialService,
    NetworkClient,
    ReplicatedFirst,
    ReplicatedStorage,
    ServerScriptService,
    ServerStorage,
    StarterGui,
    StarterPack,
    StarterPlayer,
    StarterCharacterScripts,
    StarterPlayerScripts,
    Teams,
    SoundService,
    TextChatService,

    // Core Instances
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

#[derive(Clone, Debug)]
pub struct InstanceProperties {
    pub class_name: String,
    pub parent_name: String,
    pub archivable: bool,
    pub unique_id: String,
    pub tags: Vec<String>,
    pub attributes: Vec<(String, String)>,

    // Appearance
    pub color: Color32,
    pub material: String,
    pub transparency: f32,
    pub reflectance: f32,
    pub cast_shadow: bool,

    // Transform
    pub position: [f32; 3],
    pub orientation: [f32; 3],
    pub size: [f32; 3],

    // Collision
    pub can_collide: bool,
    pub can_touch: bool,
    pub can_query: bool,
    pub collision_group: String,

    // Assembly
    pub anchored: bool,
    pub massless: bool,
    pub mass: f32,

    // Light
    pub brightness: f32,
    pub range: f32,
    pub shadows: bool,
    pub light_enabled: bool,

    // Sound
    pub sound_id: String,
    pub volume: f32,
    pub playback_speed: f32,
    pub playing: bool,
    pub looped: bool,

    // Script
    pub script_enabled: bool,
    pub run_context: String,

    // Humanoid
    pub health: f32,
    pub max_health: f32,
    pub walk_speed: f32,
    pub jump_power: f32,
    pub auto_rotate: bool,

    // Camera
    pub fov: f32,
    pub camera_type: String,
}

impl InstanceProperties {
    pub fn default_for(class: &RobloxClass, parent: &str) -> Self {
        let (class_name, color, material, pos, rot, size, anchored, can_collide, mass) = match class {
            RobloxClass::Workspace => ("Workspace", Color32::WHITE, "Air", [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], true, false, 0.0),
            RobloxClass::Players => ("Players", Color32::WHITE, "Air", [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], true, false, 0.0),
            RobloxClass::Lighting => ("Lighting", Color32::WHITE, "Air", [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], true, false, 0.0),
            RobloxClass::MaterialService => ("MaterialService", Color32::WHITE, "Air", [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], true, false, 0.0),
            RobloxClass::NetworkClient => ("NetworkClient", Color32::WHITE, "Air", [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], true, false, 0.0),
            RobloxClass::ReplicatedFirst => ("ReplicatedFirst", Color32::WHITE, "Air", [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], true, false, 0.0),
            RobloxClass::ReplicatedStorage => ("ReplicatedStorage", Color32::WHITE, "Air", [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], true, false, 0.0),
            RobloxClass::ServerScriptService => ("ServerScriptService", Color32::WHITE, "Air", [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], true, false, 0.0),
            RobloxClass::ServerStorage => ("ServerStorage", Color32::WHITE, "Air", [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], true, false, 0.0),
            RobloxClass::StarterGui => ("StarterGui", Color32::WHITE, "Air", [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], true, false, 0.0),
            RobloxClass::StarterPack => ("StarterPack", Color32::WHITE, "Air", [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], true, false, 0.0),
            RobloxClass::StarterPlayer => ("StarterPlayer", Color32::WHITE, "Air", [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], true, false, 0.0),
            RobloxClass::StarterCharacterScripts => ("StarterCharacterScripts", Color32::WHITE, "Air", [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], true, false, 0.0),
            RobloxClass::StarterPlayerScripts => ("StarterPlayerScripts", Color32::WHITE, "Air", [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], true, false, 0.0),
            RobloxClass::Teams => ("Teams", Color32::WHITE, "Air", [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], true, false, 0.0),
            RobloxClass::SoundService => ("SoundService", Color32::WHITE, "Air", [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], true, false, 0.0),
            RobloxClass::TextChatService => ("TextChatService", Color32::WHITE, "Air", [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], true, false, 0.0),
            RobloxClass::Model => ("Model", Color32::WHITE, "SmoothPlastic", [0.0, 1.25, 0.0], [0.0, 0.0, 0.0], [4.0, 5.0, 4.0], false, true, 15.0),
            RobloxClass::Part => ("Part", Color32::from_rgb(163, 162, 165), "Plastic", [0.0, 1.0, 0.0], [0.0, 0.0, 0.0], [4.0, 1.2, 2.0], true, true, 5.0),
            RobloxClass::MeshPart => ("MeshPart", Color32::from_rgb(85, 85, 90), "SmoothPlastic", [0.0, 1.0, 0.0], [0.0, 0.0, 0.0], [2.0, 2.0, 2.0], false, true, 3.2),
            RobloxClass::Camera => ("Camera", Color32::WHITE, "Air", [0.0, 15.0, 25.0], [-15.0, 0.0, 0.0], [1.0, 1.0, 1.0], true, false, 0.0),
            RobloxClass::Light => ("DirectionalLight", Color32::from_rgb(255, 245, 210), "Air", [0.0, 50.0, 0.0], [-45.0, 30.0, 0.0], [1.0, 1.0, 1.0], true, false, 0.0),
            RobloxClass::Terrain => ("Terrain", Color32::from_rgb(106, 127, 63), "Grass", [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [2048.0, 512.0, 2048.0], true, true, 9999.0),
            RobloxClass::Script => ("Script", Color32::WHITE, "Air", [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], true, false, 0.0),
            RobloxClass::Sound => ("Sound", Color32::WHITE, "Air", [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], true, false, 0.0),
            RobloxClass::Humanoid => ("Humanoid", Color32::WHITE, "Air", [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], false, false, 1.0),
            RobloxClass::Folder => ("Folder", Color32::WHITE, "Air", [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], true, false, 0.0),
        };

        Self {
            class_name: class_name.to_string(),
            parent_name: parent.to_string(),
            archivable: true,
            unique_id: "3bbadd56473ef31f0b07d9fa000003f2".to_string(),
            tags: Vec::new(),
            attributes: Vec::new(),

            color,
            material: material.to_string(),
            transparency: 0.0,
            reflectance: 0.0,
            cast_shadow: true,

            position: pos,
            orientation: rot,
            size,

            can_collide,
            can_touch: true,
            can_query: true,
            collision_group: "Default".to_string(),

            anchored,
            massless: false,
            mass,

            brightness: 2.0,
            range: 60.0,
            shadows: true,
            light_enabled: true,

            sound_id: "rbxassetid://9043254123".to_string(),
            volume: 0.85,
            playback_speed: 1.0,
            playing: false,
            looped: true,

            script_enabled: true,
            run_context: "Legacy".to_string(),

            health: 100.0,
            max_health: 100.0,
            walk_speed: 16.0,
            jump_power: 50.0,
            auto_rotate: true,

            fov: 70.0,
            camera_type: "Custom".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExplorerItem {
    pub id: usize,
    pub name: String,
    pub class: RobloxClass,
    pub is_active: bool,
    pub is_expanded: bool,
    pub properties: InstanceProperties,
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
            selected_id: Some(121), // StarterCharacterScripts seleccionado por defecto (como en la imagen 2)
            next_id: 200,
            textures: None,
            items: vec![
                // 1. Workspace
                ExplorerItem {
                    id: 10,
                    name: "Workspace".into(),
                    class: RobloxClass::Workspace,
                    is_active: true,
                    is_expanded: false,
                    properties: InstanceProperties::default_for(&RobloxClass::Workspace, "Game"),
                    children: vec![
                        ExplorerItem {
                            id: 11,
                            name: "Camera".into(),
                            class: RobloxClass::Camera,
                            is_active: true,
                            is_expanded: false,
                            properties: InstanceProperties::default_for(&RobloxClass::Camera, "Workspace"),
                            children: vec![],
                        },
                        ExplorerItem {
                            id: 12,
                            name: "Terrain".into(),
                            class: RobloxClass::Terrain,
                            is_active: true,
                            is_expanded: false,
                            properties: InstanceProperties::default_for(&RobloxClass::Terrain, "Workspace"),
                            children: vec![],
                        },
                        ExplorerItem {
                            id: 13,
                            name: "Baseplate".into(),
                            class: RobloxClass::Part,
                            is_active: true,
                            is_expanded: false,
                            properties: InstanceProperties::default_for(&RobloxClass::Part, "Workspace"),
                            children: vec![],
                        },
                    ],
                },
                // 2. Players
                ExplorerItem {
                    id: 20,
                    name: "Players".into(),
                    class: RobloxClass::Players,
                    is_active: true,
                    is_expanded: false,
                    properties: InstanceProperties::default_for(&RobloxClass::Players, "Game"),
                    children: vec![],
                },
                // 3. Lighting
                ExplorerItem {
                    id: 30,
                    name: "Lighting".into(),
                    class: RobloxClass::Lighting,
                    is_active: true,
                    is_expanded: false,
                    properties: InstanceProperties::default_for(&RobloxClass::Lighting, "Game"),
                    children: vec![
                        ExplorerItem {
                            id: 31,
                            name: "DirectionalLight".into(),
                            class: RobloxClass::Light,
                            is_active: true,
                            is_expanded: false,
                            properties: InstanceProperties::default_for(&RobloxClass::Light, "Lighting"),
                            children: vec![],
                        },
                        ExplorerItem {
                            id: 32,
                            name: "Sky".into(),
                            class: RobloxClass::Part,
                            is_active: true,
                            is_expanded: false,
                            properties: InstanceProperties::default_for(&RobloxClass::Part, "Lighting"),
                            children: vec![],
                        },
                    ],
                },
                // 4. MaterialService
                ExplorerItem {
                    id: 40,
                    name: "MaterialService".into(),
                    class: RobloxClass::MaterialService,
                    is_active: true,
                    is_expanded: false,
                    properties: InstanceProperties::default_for(&RobloxClass::MaterialService, "Game"),
                    children: vec![],
                },
                // 5. NetworkClient
                ExplorerItem {
                    id: 50,
                    name: "NetworkClient".into(),
                    class: RobloxClass::NetworkClient,
                    is_active: true,
                    is_expanded: false,
                    properties: InstanceProperties::default_for(&RobloxClass::NetworkClient, "Game"),
                    children: vec![],
                },
                // 6. ReplicatedFirst
                ExplorerItem {
                    id: 60,
                    name: "ReplicatedFirst".into(),
                    class: RobloxClass::ReplicatedFirst,
                    is_active: true,
                    is_expanded: false,
                    properties: InstanceProperties::default_for(&RobloxClass::ReplicatedFirst, "Game"),
                    children: vec![],
                },
                // 7. ReplicatedStorage
                ExplorerItem {
                    id: 70,
                    name: "ReplicatedStorage".into(),
                    class: RobloxClass::ReplicatedStorage,
                    is_active: true,
                    is_expanded: false,
                    properties: InstanceProperties::default_for(&RobloxClass::ReplicatedStorage, "Game"),
                    children: vec![
                        ExplorerItem {
                            id: 71,
                            name: "BulletVFX".into(),
                            class: RobloxClass::MeshPart,
                            is_active: true,
                            is_expanded: false,
                            properties: InstanceProperties::default_for(&RobloxClass::MeshPart, "ReplicatedStorage"),
                            children: vec![],
                        },
                        ExplorerItem {
                            id: 72,
                            name: "HitSound".into(),
                            class: RobloxClass::Sound,
                            is_active: true,
                            is_expanded: false,
                            properties: InstanceProperties::default_for(&RobloxClass::Sound, "ReplicatedStorage"),
                            children: vec![],
                        },
                    ],
                },
                // 8. ServerScriptService
                ExplorerItem {
                    id: 80,
                    name: "ServerScriptService".into(),
                    class: RobloxClass::ServerScriptService,
                    is_active: true,
                    is_expanded: false,
                    properties: InstanceProperties::default_for(&RobloxClass::ServerScriptService, "Game"),
                    children: vec![
                        ExplorerItem {
                            id: 81,
                            name: "MainGameServer".into(),
                            class: RobloxClass::Script,
                            is_active: true,
                            is_expanded: false,
                            properties: InstanceProperties::default_for(&RobloxClass::Script, "ServerScriptService"),
                            children: vec![],
                        },
                    ],
                },
                // 9. ServerStorage
                ExplorerItem {
                    id: 90,
                    name: "ServerStorage".into(),
                    class: RobloxClass::ServerStorage,
                    is_active: true,
                    is_expanded: false,
                    properties: InstanceProperties::default_for(&RobloxClass::ServerStorage, "Game"),
                    children: vec![],
                },
                // 10. StarterGui
                ExplorerItem {
                    id: 100,
                    name: "StarterGui".into(),
                    class: RobloxClass::StarterGui,
                    is_active: true,
                    is_expanded: false,
                    properties: InstanceProperties::default_for(&RobloxClass::StarterGui, "Game"),
                    children: vec![],
                },
                // 11. StarterPack
                ExplorerItem {
                    id: 110,
                    name: "StarterPack".into(),
                    class: RobloxClass::StarterPack,
                    is_active: true,
                    is_expanded: false,
                    properties: InstanceProperties::default_for(&RobloxClass::StarterPack, "Game"),
                    children: vec![],
                },
                // 12. StarterPlayer
                ExplorerItem {
                    id: 120,
                    name: "StarterPlayer".into(),
                    class: RobloxClass::StarterPlayer,
                    is_active: true,
                    is_expanded: true,
                    properties: InstanceProperties::default_for(&RobloxClass::StarterPlayer, "Game"),
                    children: vec![
                        ExplorerItem {
                            id: 121,
                            name: "StarterCharacterScripts".into(),
                            class: RobloxClass::StarterCharacterScripts,
                            is_active: true,
                            is_expanded: false,
                            properties: InstanceProperties::default_for(&RobloxClass::StarterCharacterScripts, "StarterPlayer"),
                            children: vec![],
                        },
                        ExplorerItem {
                            id: 122,
                            name: "StarterPlayerScripts".into(),
                            class: RobloxClass::StarterPlayerScripts,
                            is_active: true,
                            is_expanded: false,
                            properties: InstanceProperties::default_for(&RobloxClass::StarterPlayerScripts, "StarterPlayer"),
                            children: vec![],
                        },
                    ],
                },
                // 13. Teams
                ExplorerItem {
                    id: 130,
                    name: "Teams".into(),
                    class: RobloxClass::Teams,
                    is_active: true,
                    is_expanded: false,
                    properties: InstanceProperties::default_for(&RobloxClass::Teams, "Game"),
                    children: vec![],
                },
                // 14. SoundService
                ExplorerItem {
                    id: 140,
                    name: "SoundService".into(),
                    class: RobloxClass::SoundService,
                    is_active: true,
                    is_expanded: false,
                    properties: InstanceProperties::default_for(&RobloxClass::SoundService, "Game"),
                    children: vec![],
                },
                // 15. TextChatService
                ExplorerItem {
                    id: 150,
                    name: "TextChatService".into(),
                    class: RobloxClass::TextChatService,
                    is_active: true,
                    is_expanded: false,
                    properties: InstanceProperties::default_for(&RobloxClass::TextChatService, "Game"),
                    children: vec![],
                },
            ],
        }
    }
}

// Colores exactos de Roblox Studio Dark Theme
pub const STUDIO_BG: Color32 = Color32::from_rgb(37, 37, 38);          // #252526
pub const STUDIO_TREE_BG: Color32 = Color32::from_rgb(30, 30, 30);     // #1E1E1E
pub const STUDIO_SELECTION: Color32 = Color32::from_rgb(0, 95, 184);   // #005FB8 (Azul moderno Studio)
pub const STUDIO_HOVER: Color32 = Color32::from_rgb(46, 46, 50);       // #2E2E32
pub const STUDIO_BORDER: Color32 = Color32::from_rgb(45, 45, 48);      // #2D2D30
pub const STUDIO_TEXT: Color32 = Color32::from_rgb(204, 204, 204);     // #CCCCCC
pub const STUDIO_TEXT_MUTED: Color32 = Color32::from_rgb(130, 130, 130);
pub const STUDIO_INPUT_BG: Color32 = Color32::from_rgb(26, 26, 26);    // #1A1A1A

impl SceneHierarchyState {
    pub fn find_item_mut(&mut self, target_id: usize) -> Option<&mut ExplorerItem> {
        fn find_recursive<'a>(items: &'a mut [ExplorerItem], target_id: usize) -> Option<&'a mut ExplorerItem> {
            for item in items.iter_mut() {
                if item.id == target_id {
                    return Some(item);
                }
                if let Some(found) = find_recursive(&mut item.children, target_id) {
                    return Some(found);
                }
            }
            None
        }
        find_recursive(&mut self.items, target_id)
    }

    pub fn get_selected_item_mut(&mut self) -> Option<&mut ExplorerItem> {
        let id = self.selected_id?;
        self.find_item_mut(id)
    }

    pub fn insert_child_into_selected_or_workspace(&mut self, class: RobloxClass, name: String) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        let new_item = ExplorerItem {
            id,
            name: name.clone(),
            class: class.clone(),
            is_active: true,
            is_expanded: false,
            properties: InstanceProperties::default_for(&class, "Workspace"),
            children: vec![],
        };

        if let Some(target_id) = self.selected_id {
            if let Some(parent_item) = self.find_item_mut(target_id) {
                if matches!(parent_item.class, RobloxClass::Workspace | RobloxClass::Model | RobloxClass::Folder | RobloxClass::StarterPlayer | RobloxClass::ServerStorage | RobloxClass::ReplicatedStorage) {
                    parent_item.children.push(new_item);
                    parent_item.is_expanded = true;
                    self.selected_id = Some(id);
                    return id;
                }
            }
        }

        if let Some(ws) = self.find_item_mut(10) {
            ws.children.push(new_item);
            ws.is_expanded = true;
        } else if let Some(first) = self.items.first_mut() {
            first.children.push(new_item);
        }
        self.selected_id = Some(id);
        id
    }

    pub fn show(&mut self, ui: &mut egui::Ui, icons: &StudioIcons) -> (Option<String>, bool) {
        let mut clicked_object_name = None;
        let mut close_requested = false;

        // Cargar texturas oficiales de Roblox Studio si no están inicializadas
        if self.textures.is_none() {
            self.textures = Some(RobloxIconTextures::new(ui.ctx()));
        }

        // =====================================================================
        // 1. HEADER DE PANEL DESACOPLABLE (Roblox Studio Header: Centrado + ⤢ + ✕)
        // =====================================================================
        let header_h = 26.0;
        let (header_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), header_h), Sense::hover());
        ui.painter().rect_filled(header_rect, 0.0, STUDIO_BG);

        // Título de panel centrado
        ui.painter().text(
            header_rect.center(),
            egui::Align2::CENTER_CENTER,
            "Explorador",
            FontId::proportional(12.0),
            Color32::from_rgb(220, 220, 225),
        );

        // Botones derecha: ⤢ (Flotar / Desacoplar) y ✕ (Cerrar)
        let undock_center = Pos2::new(header_rect.max.x - 26.0, header_rect.center().y);
        let undock_rect = Rect::from_center_size(undock_center, Vec2::new(16.0, 16.0));
        let undock_resp = ui.interact(undock_rect, ui.id().with("explorer_undock_btn"), Sense::click());
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
        let close_resp = ui.interact(close_rect, ui.id().with("explorer_close_btn"), Sense::click());
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
        // 2. SUB-BARRA DE BÚSQUEDA MODERNA (Buscar + 🕒 Historial + ⋯ Filtros)
        // =====================================================================
        let toolbar_h = 30.0;
        let (toolbar_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), toolbar_h), Sense::hover());
        ui.painter().rect_filled(toolbar_rect, 0.0, STUDIO_BG);

        let search_h = 22.0;
        let search_y = toolbar_rect.min.y + (toolbar_h - search_h) * 0.5;
        let search_rect = Rect::from_min_size(
            Pos2::new(toolbar_rect.min.x + 6.0, search_y),
            Vec2::new(toolbar_rect.width() - 12.0, search_h),
        );

        ui.painter().rect_filled(search_rect, Rounding::same(3.0), STUDIO_INPUT_BG);
        ui.painter().rect_stroke(search_rect, Rounding::same(3.0), Stroke::new(1.0, STUDIO_BORDER));

        // Ícono de lupa alineado
        let mag_center = Pos2::new(search_rect.min.x + 13.0, search_rect.center().y);
        let mag_rect = Rect::from_center_size(mag_center, Vec2::new(13.0, 13.0));
        ui.painter().image(
            icons.icon_search.id(),
            mag_rect,
            Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
            Color32::from_rgb(160, 160, 160),
        );

        // Botones en el extremo derecho: ⋯ (Filtros) y 🕒 (Historial)
        let mut right_reserve = 44.0;
        if !self.search_query.is_empty() {
            right_reserve += 18.0;
        }

        // Historial (🕒)
        let hist_center = Pos2::new(search_rect.max.x - 30.0, search_rect.center().y);
        let hist_rect = Rect::from_center_size(hist_center, Vec2::new(16.0, 16.0));
        let hist_resp = ui.interact(hist_rect, ui.id().with("search_hist_btn"), Sense::click());
        if hist_resp.hovered() {
            ui.painter().rect_filled(hist_rect, Rounding::same(2.0), STUDIO_HOVER);
        }
        ui.painter().text(
            hist_center,
            egui::Align2::CENTER_CENTER,
            "🕒",
            FontId::proportional(10.5),
            if hist_resp.hovered() { Color32::WHITE } else { Color32::from_rgb(160, 160, 165) },
        );
        hist_resp.on_hover_text("Historial de búsqueda");

        // Opciones de filtro (⋯)
        let opt_center = Pos2::new(search_rect.max.x - 12.0, search_rect.center().y);
        let opt_rect = Rect::from_center_size(opt_center, Vec2::new(16.0, 16.0));
        let opt_resp = ui.interact(opt_rect, ui.id().with("search_opt_btn"), Sense::click());
        if opt_resp.hovered() {
            ui.painter().rect_filled(opt_rect, Rounding::same(2.0), STUDIO_HOVER);
        }
        ui.painter().text(
            opt_center,
            egui::Align2::CENTER_CENTER,
            "⋯",
            FontId::proportional(11.0),
            if opt_resp.hovered() { Color32::WHITE } else { Color32::from_rgb(160, 160, 165) },
        );
        opt_resp.on_hover_text("Opciones de filtro");

        let text_rect = Rect::from_min_max(
            Pos2::new(search_rect.min.x + 25.0, search_rect.min.y + 2.0),
            Pos2::new(search_rect.max.x - right_reserve, search_rect.max.y - 2.0),
        );

        let mut edit_ui = ui.child_ui(text_rect, egui::Layout::left_to_right(egui::Align::Center), None);
        edit_ui.spacing_mut().item_spacing = Vec2::ZERO;
        edit_ui.add(
            egui::TextEdit::singleline(&mut self.search_query)
                .hint_text("Buscar")
                .frame(false)
                .margin(egui::Margin::symmetric(0.0, 1.0))
                .font(FontId::proportional(11.5))
                .desired_width(text_rect.width()),
        );

        if !self.search_query.is_empty() {
            let clear_center = Pos2::new(search_rect.max.x - 48.0, search_rect.center().y);
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

        // =====================================================================
        // 3. ÁREA DEL ÁRBOL
        // =====================================================================
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

        (clicked_object_name, close_requested)
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

        // Highlight moderno de selección estilo Roblox Studio con redondeo
        if is_selected {
            let sel_rect = Rect::from_min_max(Pos2::new(row_rect.min.x + 2.0, row_rect.min.y), Pos2::new(row_rect.max.x - 2.0, row_rect.max.y));
            ui.painter().rect_filled(sel_rect, Rounding::same(3.0), STUDIO_SELECTION);
        } else if row_resp.hovered() {
            ui.painter().rect_filled(row_rect, 0.0, STUDIO_HOVER);
        }

        let indent_x = row_rect.min.x + (depth as f32 * 14.0) + 6.0;

        // Flecha Foldout ▶ / ▼
        if has_children {
            let arrow_pos = Pos2::new(indent_x + 3.0, row_rect.center().y);
            let arrow_rect = Rect::from_center_size(arrow_pos, Vec2::new(14.0, 14.0));
            let arrow_resp = ui.interact(arrow_rect, ui.id().with(node.id).with("arrow"), Sense::click());
            if arrow_resp.clicked() {
                node.is_expanded = !node.is_expanded;
            }
            let arrow_col = if arrow_resp.hovered() { Color32::WHITE } else { Color32::from_rgb(180, 180, 185) };
            icons::foldout_arrow(ui.painter(), arrow_pos, node.is_expanded, arrow_col);
        }

        // ICONOS CLÁSICOS OFICIALES (Pixel-art retro que le encanta al usuario)
        let icon_rect = Rect::from_center_size(Pos2::new(indent_x + 18.0, row_rect.center().y), Vec2::new(16.0, 16.0));
        if let Some(texs) = textures {
            let tex = match node.class {
                RobloxClass::Workspace => &texs.workspace,
                RobloxClass::Players | RobloxClass::Teams => &texs.humanoid,
                RobloxClass::Lighting | RobloxClass::Light => &texs.lighting,
                RobloxClass::MaterialService | RobloxClass::Terrain => &texs.terrain,
                RobloxClass::NetworkClient => &texs.model,
                RobloxClass::ServerScriptService | RobloxClass::StarterCharacterScripts | RobloxClass::StarterPlayerScripts | RobloxClass::Script => &texs.script,
                RobloxClass::SoundService | RobloxClass::Sound => &texs.sound,
                RobloxClass::TextChatService => &texs.script,
                RobloxClass::ReplicatedFirst | RobloxClass::ReplicatedStorage | RobloxClass::ServerStorage | RobloxClass::StarterGui | RobloxClass::StarterPack | RobloxClass::StarterPlayer | RobloxClass::Folder => &texs.folder,
                RobloxClass::Model => &texs.model,
                RobloxClass::Part => &texs.part,
                RobloxClass::MeshPart => &texs.meshpart,
                RobloxClass::Camera => &texs.camera,
                RobloxClass::Humanoid => &texs.humanoid,
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
            Pos2::new(indent_x + 30.0, row_rect.center().y),
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
