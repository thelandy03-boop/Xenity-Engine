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

impl RobloxClass {
    pub fn is_service(&self) -> bool {
        matches!(
            self,
            RobloxClass::Workspace
                | RobloxClass::Players
                | RobloxClass::Lighting
                | RobloxClass::MaterialService
                | RobloxClass::NetworkClient
                | RobloxClass::ReplicatedFirst
                | RobloxClass::ReplicatedStorage
                | RobloxClass::ServerScriptService
                | RobloxClass::ServerStorage
                | RobloxClass::StarterGui
                | RobloxClass::StarterPack
                | RobloxClass::StarterPlayer
                | RobloxClass::StarterCharacterScripts
                | RobloxClass::StarterPlayerScripts
                | RobloxClass::Teams
                | RobloxClass::SoundService
                | RobloxClass::TextChatService
        )
    }

    /// Índice exacto de icono en el spritesheet oficial ClassImages.PNG (según ReflectionMetadata.xml)
    pub fn explorer_image_index(&self) -> usize {
        match self {
            RobloxClass::Part => 1,
            RobloxClass::Model => 2,
            RobloxClass::Camera => 5,
            RobloxClass::Script => 6,
            RobloxClass::Humanoid => 9,
            RobloxClass::Sound => 11,
            RobloxClass::Lighting | RobloxClass::Light => 13,
            RobloxClass::NetworkClient => 16,
            RobloxClass::Workspace => 19,
            RobloxClass::StarterPack => 20,
            RobloxClass::Players => 21,
            RobloxClass::Teams => 23,
            RobloxClass::SoundService => 31,
            RobloxClass::StarterGui => 46,
            RobloxClass::Terrain => 65,
            RobloxClass::ServerStorage => 69,
            RobloxClass::ReplicatedFirst | RobloxClass::ReplicatedStorage => 70,
            RobloxClass::ServerScriptService => 71,
            RobloxClass::MeshPart => 73,
            RobloxClass::Folder => 77,
            RobloxClass::StarterCharacterScripts | RobloxClass::StarterPlayerScripts => 78,
            RobloxClass::StarterPlayer => 79,
            RobloxClass::MaterialService => 131,
            RobloxClass::TextChatService => 143,
        }
    }
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
    pub class_images: egui::TextureHandle,
    pub icon_add: egui::TextureHandle,
}

impl RobloxIconTextures {
    pub fn new(ctx: &egui::Context) -> Self {
        let opt = egui::TextureOptions::NEAREST;
        let img = image::load_from_memory_with_format(
            include_bytes!("../assets/icons/ClassImages.PNG"),
            image::ImageFormat::Png,
        )
        .expect("Valid Roblox Studio ClassImages.PNG")
        .to_rgba8();
        let (w, h) = img.dimensions();

        let add_img = image::load_from_memory_with_format(
            include_bytes!("../assets/icons/explorer_add.png"),
            image::ImageFormat::Png,
        )
        .expect("Valid Roblox Studio explorer_add.png")
        .to_rgba8();
        let (add_w, add_h) = add_img.dimensions();

        Self {
            class_images: ctx.load_texture(
                "rbx_class_images",
                egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], &img.into_raw()),
                opt,
            ),
            icon_add: ctx.load_texture(
                "rbx_explorer_add",
                egui::ColorImage::from_rgba_unmultiplied([add_w as usize, add_h as usize], &add_img.into_raw()),
                egui::TextureOptions::LINEAR,
            ),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ExplorerContextMenu {
    pub is_service: bool,
    pub pos: Pos2,
}

pub struct SceneHierarchyState {
    pub search_query: String,
    pub selected_id: Option<usize>,
    pub items: Vec<ExplorerItem>,
    pub next_id: usize,
    pub textures: Option<RobloxIconTextures>,
    pub context_menu: Option<ExplorerContextMenu>,
    pub xenity_mappings: std::collections::HashMap<usize, i32>,
    pub last_sync_instant: std::time::Instant,
    pub cached_gameobject_count: i32,
    pub is_dirty: bool,
}

impl Default for SceneHierarchyState {
    fn default() -> Self {
        Self {
            search_query: String::new(),
            selected_id: None,
            next_id: 200,
            textures: None,
            context_menu: None,
            xenity_mappings: std::collections::HashMap::new(),
            last_sync_instant: std::time::Instant::now(),
            cached_gameobject_count: -1,
            is_dirty: true,
            items: vec![
                // 1. Workspace (Servicio raíz de Roblox)
                ExplorerItem {
                    id: 10,
                    name: "Workspace".into(),
                    class: RobloxClass::Workspace,
                    is_active: true,
                    is_expanded: true,
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
pub const STUDIO_BG: Color32 = Color32::from_rgb(30, 30, 32);          // #1E1E20
pub const STUDIO_TREE_BG: Color32 = Color32::from_rgb(26, 26, 28);     // #1A1A1C
pub const STUDIO_SELECTION: Color32 = Color32::from_rgb(0, 122, 218);   // #007ACC (Azul oficial Roblox Studio)
pub const STUDIO_HOVER: Color32 = Color32::from_rgb(46, 46, 50);       // #2E2E32
pub const STUDIO_BORDER: Color32 = Color32::from_rgb(45, 45, 48);      // #2D2D30
pub const STUDIO_TEXT: Color32 = Color32::from_rgb(215, 215, 220);     // #D7D7DC
pub const STUDIO_TEXT_MUTED: Color32 = Color32::from_rgb(130, 130, 130);
pub const STUDIO_INPUT_BG: Color32 = Color32::from_rgb(22, 22, 24);    // #161618

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

    pub fn get_xenity_index_for_id(&self, id: usize) -> Option<i32> {
        self.xenity_mappings.get(&id).copied()
    }

    pub fn get_id_for_xenity_index(&self, x_idx: i32) -> Option<usize> {
        self.xenity_mappings.iter().find_map(|(&k, &v)| if v == x_idx { Some(k) } else { None })
    }

    pub fn sync_from_engine(&mut self, engine: &mut crate::xenity_ffi::XenityEngine) {
        let count = engine.get_gameobject_count();
        if count <= 0 {
            return;
        }

        // Evitar escanear el motor C++ y reservar memoria en cada frame si no hay cambios
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(self.last_sync_instant).as_millis();
        if count == self.cached_gameobject_count && !self.is_dirty && elapsed < 400 {
            return;
        }

        self.last_sync_instant = now;
        self.cached_gameobject_count = count;
        self.is_dirty = false;

        let ws_pos = self.items.iter().position(|it| it.class == RobloxClass::Workspace);
        let ws_idx = match ws_pos {
            Some(i) => i,
            None => return,
        };

        let mut active_indices = std::collections::HashSet::new();

        for i in 0..count {
            if let Some(info) = engine.get_gameobject_info(i) {
                let name = info.name_as_str();
                if name.is_empty() || name == "EditorCamera" {
                    continue;
                }
                active_indices.insert(i);

                let color = Color32::from_rgb(
                    (info.color[0] * 255.0).clamp(0.0, 255.0) as u8,
                    (info.color[1] * 255.0).clamp(0.0, 255.0) as u8,
                    (info.color[2] * 255.0).clamp(0.0, 255.0) as u8,
                );

                // Buscar si ya está mapeado al índice del motor i
                let mapped_id = self.xenity_mappings.iter()
                    .find_map(|(&id, &x_i)| if x_i == i { Some(id) } else { None });

                if let Some(id) = mapped_id {
                    if let Some(item) = self.items[ws_idx].children.iter_mut().find(|c| c.id == id) {
                        item.name = name.to_string();
                        item.properties.position = info.pos;
                        item.properties.orientation = info.rot;
                        item.properties.size = info.scale;
                        item.properties.color = color;
                        item.is_active = info.is_active != 0;
                    }
                } else {
                    // Si no está mapeado, verificar si hay un hijo en Workspace con el mismo nombre
                    let unmapped_item = self.items[ws_idx].children.iter_mut()
                        .find(|c| c.name == name && !self.xenity_mappings.contains_key(&c.id));

                    if let Some(item) = unmapped_item {
                        item.properties.position = info.pos;
                        item.properties.orientation = info.rot;
                        item.properties.size = info.scale;
                        item.properties.color = color;
                        item.is_active = info.is_active != 0;
                        self.xenity_mappings.insert(item.id, i);
                    } else {
                        let roblox_class = if info.has_light != 0 {
                            RobloxClass::Light
                        } else if name == "Camera" {
                            RobloxClass::Camera
                        } else {
                            RobloxClass::Part
                        };

                        let new_id = self.next_id;
                        self.next_id += 1;
                        let mut props = InstanceProperties::default_for(&roblox_class, "Workspace");
                        props.position = info.pos;
                        props.orientation = info.rot;
                        props.size = info.scale;
                        props.color = color;

                        self.items[ws_idx].children.push(ExplorerItem {
                            id: new_id,
                            name: name.to_string(),
                            class: roblox_class,
                            is_active: info.is_active != 0,
                            is_expanded: false,
                            properties: props,
                            children: vec![],
                        });
                        self.xenity_mappings.insert(new_id, i);
                    }
                }
            }
        }

        // Limpiar objetos y mapeos que ya no existen en C++
        let ids_to_remove: Vec<usize> = self.xenity_mappings.iter()
            .filter(|(_, x_i)| !active_indices.contains(x_i))
            .map(|(&id, _)| id)
            .collect();

        for id in ids_to_remove {
            self.xenity_mappings.remove(&id);
            self.items[ws_idx].children.retain(|c| c.id != id);
            if self.selected_id == Some(id) {
                self.selected_id = None;
            }
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, icons: &StudioIcons) -> (Option<String>, bool) {
        let mut clicked_object_name = None;
        let mut close_requested = false;

        // Cargar texturas oficiales de Roblox Studio si no están inicializadas
        if self.textures.is_none() {
            self.textures = Some(RobloxIconTextures::new(ui.ctx()));
        }

        // =====================================================================
        // 1. DOCK TAB BAR OFICIAL (Explorer Tab con acento cian #00A2FF y botón ⋮)
        // =====================================================================
        let header_h = 26.0;
        let (header_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), header_h), Sense::hover());
        ui.painter().rect_filled(header_rect, 0.0, STUDIO_BG);

        // Pestaña activa Explorer
        let tab_w = 95.0;
        let tab_rect = Rect::from_min_size(header_rect.min, Vec2::new(tab_w, header_h));
        ui.painter().rect_filled(tab_rect, 0.0, Color32::from_rgb(40, 40, 44));

        // Franja superior cian activo (Cyan Roblox Studio #00A2FF)
        ui.painter().line_segment(
            [tab_rect.min, Pos2::new(tab_rect.max.x, tab_rect.min.y)],
            Stroke::new(2.0, Color32::from_rgb(0, 162, 255)),
        );

        // Borde derecho de la pestaña
        ui.painter().line_segment(
            [Pos2::new(tab_rect.max.x, tab_rect.min.y), tab_rect.max],
            Stroke::new(1.0, STUDIO_BORDER),
        );

        // Ícono de la pestaña Explorer
        let icon_center = Pos2::new(tab_rect.min.x + 13.0, tab_rect.center().y);
        icons::tab_explorer_icon(ui.painter(), icon_center, Color32::from_rgb(215, 215, 220));

        // Texto de la pestaña "Explorer"
        ui.painter().text(
            Pos2::new(tab_rect.min.x + 25.0, tab_rect.center().y),
            egui::Align2::LEFT_CENTER,
            "Explorer",
            FontId::proportional(12.0),
            Color32::from_rgb(235, 235, 240),
        );

        // Botón derecho: ⋮ (Options / Menú de ventana)
        let opt_center = Pos2::new(header_rect.max.x - 12.0, header_rect.center().y);
        let opt_rect = Rect::from_center_size(opt_center, Vec2::new(18.0, 18.0));
        let opt_resp = ui.interact(opt_rect, ui.id().with("explorer_dock_menu"), Sense::click());
        if opt_resp.hovered() {
            ui.painter().rect_filled(opt_rect, Rounding::same(2.0), STUDIO_HOVER);
        }
        icons::three_dots_vertical(
            ui.painter(),
            opt_center,
            if opt_resp.hovered() { Color32::WHITE } else { Color32::from_rgb(150, 150, 155) },
        );
        opt_resp.on_hover_text("Options");

        // Separador inferior de cabecera
        ui.painter().line_segment(
            [Pos2::new(header_rect.min.x, header_rect.max.y), Pos2::new(header_rect.max.x, header_rect.max.y)],
            Stroke::new(1.0, STUDIO_BORDER),
        );

        // =====================================================================
        // 2. SEARCH BAR: Filter Workspace (Ctrl+Shift+X)
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

        ui.painter().rect_filled(search_rect, Rounding::same(3.5), STUDIO_INPUT_BG);
        ui.painter().rect_stroke(search_rect, Rounding::same(3.5), Stroke::new(1.0, STUDIO_BORDER));

        // Ícono de lupa alineado
        let mag_center = Pos2::new(search_rect.min.x + 13.0, search_rect.center().y);
        let mag_rect = Rect::from_center_size(mag_center, Vec2::new(13.0, 13.0));
        ui.painter().image(
            icons.icon_search.id(),
            mag_rect,
            Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
            Color32::from_rgb(160, 160, 160),
        );

        let right_reserve = if self.search_query.is_empty() { 6.0 } else { 22.0 };
        let text_rect = Rect::from_min_max(
            Pos2::new(search_rect.min.x + 25.0, search_rect.min.y + 2.0),
            Pos2::new(search_rect.max.x - right_reserve, search_rect.max.y - 2.0),
        );

        let mut edit_ui = ui.child_ui(text_rect, egui::Layout::left_to_right(egui::Align::Center), None);
        edit_ui.spacing_mut().item_spacing = Vec2::ZERO;
        edit_ui.add(
            egui::TextEdit::singleline(&mut self.search_query)
                .hint_text("Filter Workspace (Ctrl+Shift+X)")
                .frame(false)
                .margin(egui::Margin::symmetric(0.0, 1.0))
                .font(FontId::proportional(11.5))
                .desired_width(text_rect.width()),
        );

        if !self.search_query.is_empty() {
            let clear_center = Pos2::new(search_rect.max.x - 11.0, search_rect.center().y);
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
        let mut pending_context_menu = None;

        let selected_id = &mut self.selected_id;
        let search_query = &self.search_query;
        let items = &mut self.items;

        egui::ScrollArea::vertical()
            .id_salt("roblox_explorer_scroll")
            .show(ui, |ui| {
                ui.add_space(2.0);
                for item in items.iter_mut() {
                    Self::draw_explorer_node(
                        ui,
                        item,
                        0,
                        selected_id,
                        search_query,
                        &mut clicked_object_name,
                        textures_ref,
                        &mut pending_context_menu,
                    );
                }
            });

        if pending_context_menu.is_some() {
            self.context_menu = pending_context_menu;
        }

        // Renderizar Menú Contextual flotante en la posición exacta del cursor (idéntico a Roblox Studio)
        if let Some(menu) = self.context_menu.clone() {
            let mut close_menu = false;
            let menu_w = 175.0;

            let screen_rect = ui.ctx().screen_rect();
            let x = menu.pos.x.clamp(screen_rect.min.x + 4.0, screen_rect.max.x - menu_w - 4.0);
            let y = menu.pos.y.clamp(screen_rect.min.y + 4.0, screen_rect.max.y - 260.0);
            let menu_pos = Pos2::new(x, y);

            let area_resp = egui::Area::new(ui.id().with("explorer_context_menu_area"))
                .order(egui::Order::Foreground)
                .fixed_pos(menu_pos)
                .show(ui.ctx(), |ui| {
                    let frame = egui::Frame::popup(ui.style())
                        .fill(Color32::from_rgb(26, 27, 32))
                        .stroke(Stroke::new(1.0, Color32::from_rgb(48, 50, 58)))
                        .rounding(Rounding::same(4.0))
                        .inner_margin(egui::Margin::symmetric(2.0, 3.0))
                        .shadow(egui::epaint::Shadow {
                            offset: Vec2::new(0.0, 2.0),
                            blur: 8.0,
                            spread: 0.0,
                            color: Color32::from_black_alpha(150),
                        });

                    frame.show(ui, |ui| {
                        ui.set_width(menu_w);

                        if menu.is_service {
                            if draw_context_menu_item(ui, "Copiar como", true).clicked() { close_menu = true; }
                            draw_context_menu_divider(ui);
                            if draw_context_menu_item(ui, "Renombrar", false).clicked() { close_menu = true; }
                            if draw_context_menu_item(ui, "Insertar", true).clicked() { close_menu = true; }
                            if draw_context_menu_item(ui, "Jerarquía", true).clicked() { close_menu = true; }
                            draw_context_menu_divider(ui);
                            if draw_context_menu_item(ui, "Guardar / Exportar", true).clicked() { close_menu = true; }
                            draw_context_menu_divider(ui);
                            if draw_context_menu_item(ui, "Mostrar Servicios...", false).clicked() { close_menu = true; }
                            if draw_context_menu_item(ui, "Ocultar Service en Explorador", false).clicked() { close_menu = true; }
                            draw_context_menu_divider(ui);
                            if draw_context_menu_item(ui, "Ayuda", false).clicked() { close_menu = true; }
                        } else {
                            if draw_context_menu_item(ui, "Cortar", false).clicked() { close_menu = true; }
                            if draw_context_menu_item(ui, "Copiar", false).clicked() { close_menu = true; }
                            if draw_context_menu_item(ui, "Pegar en", false).clicked() { close_menu = true; }
                            if draw_context_menu_item(ui, "Duplicar", false).clicked() { close_menu = true; }
                            if draw_context_menu_item(ui, "Eliminar", false).clicked() { close_menu = true; }
                            draw_context_menu_divider(ui);
                            if draw_context_menu_item(ui, "Renombrar", false).clicked() { close_menu = true; }
                            if draw_context_menu_item(ui, "Insertar objeto...", true).clicked() { close_menu = true; }
                            draw_context_menu_divider(ui);
                            if draw_context_menu_item(ui, "Zoom a selección", false).clicked() { close_menu = true; }
                        }
                    });
                });

            // Cerrar si se hace clic fuera del menú
            if ui.input(|i| i.pointer.any_pressed()) {
                if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
                    if !area_resp.response.rect.contains(pos) && pos.distance(menu.pos) > 5.0 {
                        close_menu = true;
                    }
                }
            }

            if close_menu {
                self.context_menu = None;
            }
        }

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
        pending_context_menu: &mut Option<ExplorerContextMenu>,
    ) {
        let is_selected = *selected_id == Some(node.id);
        let has_children = !node.children.is_empty();
        let avail_w = ui.available_width();
        let row_h = 22.0;

        let (row_rect, row_resp) = ui.allocate_exact_size(Vec2::new(avail_w, row_h), Sense::click());

        // Highlight de selección oficial Roblox Studio (azul #0074D9 / #007ACC en toda la fila)
        if is_selected {
            ui.painter().rect_filled(row_rect, 0.0, Color32::from_rgb(0, 116, 217));
        } else if row_resp.hovered() {
            ui.painter().rect_filled(row_rect, 0.0, Color32::from_rgb(38, 40, 44));
        }

        let indent_x = row_rect.min.x + (depth as f32 * 14.0) + 4.0;

        // Flecha Foldout ▶ / ▼
        if has_children {
            let arrow_pos = Pos2::new(indent_x + 3.0, row_rect.center().y);
            let arrow_rect = Rect::from_center_size(arrow_pos, Vec2::new(14.0, 14.0));
            let arrow_resp = ui.interact(arrow_rect, ui.id().with(node.id).with("arrow"), Sense::click());
            if arrow_resp.clicked() {
                node.is_expanded = !node.is_expanded;
            }
            let arrow_col = if is_selected {
                Color32::WHITE
            } else if arrow_resp.hovered() {
                Color32::WHITE
            } else {
                Color32::from_rgb(180, 180, 185)
            };
            icons::foldout_arrow(ui.painter(), arrow_pos, node.is_expanded, arrow_col);
        }

        // ICONOS CLÁSICOS OFICIALES (Pixel-art retro 100% auténticos de ClassImages.PNG de Roblox Studio)
        let icon_rect = Rect::from_center_size(Pos2::new(indent_x + 18.0, row_rect.center().y), Vec2::new(16.0, 16.0));
        if let Some(texs) = textures {
            let idx = node.class.explorer_image_index();
            let total_icons = 147.0;
            let u_min = (idx as f32) / total_icons;
            let u_max = (idx as f32 + 1.0) / total_icons;
            let uv = Rect::from_min_max(Pos2::new(u_min, 0.0), Pos2::new(u_max, 1.0));
            ui.painter().image(
                texs.class_images.id(),
                icon_rect,
                uv,
                Color32::WHITE,
            );
        }

        // Texto del nombre de la instancia (cero asignación en heap con text directo)
        let text_color = if is_selected { Color32::WHITE } else { STUDIO_TEXT };
        let font_id = FontId::proportional(12.0);
        let text_pos = Pos2::new(indent_x + 30.0, row_rect.center().y);
        ui.painter().text(text_pos, egui::Align2::LEFT_CENTER, &node.name, font_id, text_color);

        // Botón circular [+] (Insert Object) oficial de Roblox Studio (Add.png)
        // Se ubica alineado a la derecha de la fila, idéntico a Roblox Studio oficial
        if row_resp.hovered() || is_selected {
            let plus_pos = Pos2::new(row_rect.max.x - 14.0, row_rect.center().y);
            let plus_rect = Rect::from_center_size(plus_pos, Vec2::new(14.0, 14.0));
            let plus_resp = ui.interact(plus_rect, ui.id().with(node.id).with("plus_insert"), Sense::click());

            if let Some(texs) = textures {
                let tint = if plus_resp.hovered() {
                    Color32::WHITE
                } else if is_selected {
                    Color32::from_rgb(235, 235, 240)
                } else {
                    Color32::from_rgb(200, 200, 205)
                };
                ui.painter().image(
                    texs.icon_add.id(),
                    plus_rect,
                    Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                    tint,
                );
            }

            plus_resp.on_hover_text("Insert Object");
        }

        if row_resp.clicked() {
            *selected_id = Some(node.id);
            *clicked_name = Some(node.name.clone());
        }

        if row_resp.secondary_clicked() {
            *selected_id = Some(node.id);
            *clicked_name = Some(node.name.clone());
            let click_pos = ui.input(|i| i.pointer.interact_pos()).unwrap_or_else(|| Pos2::new(row_rect.min.x + 60.0, row_rect.max.y));
            *pending_context_menu = Some(ExplorerContextMenu {
                is_service: node.class.is_service(),
                pos: click_pos,
            });
        }

        // Hijos
        if has_children && node.is_expanded {
            for child in &mut node.children {
                Self::draw_explorer_node(
                    ui,
                    child,
                    depth + 1,
                    selected_id,
                    search_filter,
                    clicked_name,
                    textures,
                    pending_context_menu,
                );
            }
        }
    }
}

// =========================================================================
// ELEMENTOS DEL MENÚ CONTEXTUAL DE EXPLORER (ESTILO OFICIAL ROBLOX STUDIO)
// =========================================================================

fn draw_context_menu_item(ui: &mut egui::Ui, label: &str, has_submenu: bool) -> egui::Response {
    let item_h = 22.0;
    let item_w = ui.available_width();
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(item_w, item_h), Sense::click());

    if resp.hovered() {
        ui.painter().rect_filled(rect, Rounding::same(2.0), Color32::from_rgb(0, 116, 217));
    }

    let text_col = if resp.hovered() { Color32::WHITE } else { Color32::from_rgb(220, 222, 230) };
    ui.painter().text(
        Pos2::new(rect.min.x + 10.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        FontId::proportional(11.5),
        text_col,
    );

    if has_submenu {
        let arrow_x = rect.max.x - 12.0;
        let p1 = Pos2::new(arrow_x - 3.0, rect.center().y - 3.5);
        let p2 = Pos2::new(arrow_x + 1.5, rect.center().y);
        let p3 = Pos2::new(arrow_x - 3.0, rect.center().y + 3.5);
        ui.painter().add(egui::Shape::convex_polygon(vec![p1, p2, p3], text_col, Stroke::NONE));
    }

    resp
}

fn draw_context_menu_divider(ui: &mut egui::Ui) {
    ui.add_space(2.0);
    let w = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(Vec2::new(w, 1.0), Sense::hover());
    ui.painter().line_segment(
        [Pos2::new(rect.min.x + 4.0, rect.center().y), Pos2::new(rect.max.x - 4.0, rect.center().y)],
        Stroke::new(1.0, Color32::from_rgb(45, 48, 54)),
    );
    ui.add_space(2.0);
}
