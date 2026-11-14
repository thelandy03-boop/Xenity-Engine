use eframe::egui::{self, ColorImage, TextureHandle, TextureOptions};

pub struct StudioIcons {
    // Dragger Tools
    pub tool_select: TextureHandle,
    pub tool_move: TextureHandle,
    pub tool_rotate: TextureHandle,
    pub tool_scale: TextureHandle,
    pub tool_transform: TextureHandle,

    // Insertable Objects
    pub insert_part: TextureHandle,
    pub insert_model: TextureHandle,
    pub insert_script: TextureHandle,
    pub insert_sound: TextureHandle,
    pub insert_light: TextureHandle,
    pub insert_spawn: TextureHandle,

    // Studio Windows
    pub win_terrain: TextureHandle,
    pub win_toolbox: TextureHandle,
    pub win_explorer: TextureHandle,
    pub win_properties: TextureHandle,
    pub win_output: TextureHandle,
    pub win_cmdbar: TextureHandle,
    pub win_assetmgr: TextureHandle,
    pub win_animeditor: TextureHandle,
    pub win_buildrig: TextureHandle,
    pub win_teamcreate: TextureHandle,
    pub win_plugins: TextureHandle,

    // Simulation
    pub sim_play: TextureHandle,
    pub sim_playhere: TextureHandle,
    pub sim_run: TextureHandle,
    pub sim_pause: TextureHandle,
    pub sim_stop: TextureHandle,

    // Actions & Quick Access
    pub action_anchor: TextureHandle,
    pub action_gamesettings: TextureHandle,
    pub action_collab: TextureHandle,
    pub qa_undo: TextureHandle,
    pub qa_redo: TextureHandle,
    pub icon_search: TextureHandle,
    pub studio_logo: TextureHandle,
    pub out_clear: TextureHandle,
    pub out_more: TextureHandle,
    pub doc_place_cloud: TextureHandle,

    // View Selector Cube
    pub view_front: TextureHandle,
    pub view_top: TextureHandle,
    pub view_right: TextureHandle,
}

fn load_png(ctx: &egui::Context, name: &str, bytes: &[u8]) -> TextureHandle {
    let img = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)
        .expect("Valid Roblox Studio PNG")
        .to_rgba8();
    let (w, h) = img.dimensions();
    ctx.load_texture(
        name,
        ColorImage::from_rgba_unmultiplied([w as usize, h as usize], &img.into_raw()),
        TextureOptions::LINEAR,
    )
}

impl StudioIcons {
    pub fn new(ctx: &egui::Context) -> Self {
        Self {
            tool_select: load_png(ctx, "rbx_tool_select", include_bytes!("../assets/icons/tool_select.png")),
            tool_move: load_png(ctx, "rbx_tool_move", include_bytes!("../assets/icons/tool_move.png")),
            tool_rotate: load_png(ctx, "rbx_tool_rotate", include_bytes!("../assets/icons/tool_rotate.png")),
            tool_scale: load_png(ctx, "rbx_tool_scale", include_bytes!("../assets/icons/tool_scale.png")),
            tool_transform: load_png(ctx, "rbx_tool_transform", include_bytes!("../assets/icons/tool_transform.png")),

            insert_part: load_png(ctx, "rbx_insert_part", include_bytes!("../assets/icons/insert_part.png")),
            insert_model: load_png(ctx, "rbx_insert_model", include_bytes!("../assets/icons/insert_model.png")),
            insert_script: load_png(ctx, "rbx_insert_script", include_bytes!("../assets/icons/insert_script.png")),
            insert_sound: load_png(ctx, "rbx_insert_sound", include_bytes!("../assets/icons/insert_sound.png")),
            insert_light: load_png(ctx, "rbx_insert_light", include_bytes!("../assets/icons/insert_light.png")),
            insert_spawn: load_png(ctx, "rbx_insert_spawn", include_bytes!("../assets/icons/insert_spawn.png")),

            win_terrain: load_png(ctx, "rbx_win_terrain", include_bytes!("../assets/icons/win_terrain.png")),
            win_toolbox: load_png(ctx, "rbx_win_toolbox", include_bytes!("../assets/icons/win_toolbox.png")),
            win_explorer: load_png(ctx, "rbx_win_explorer", include_bytes!("../assets/icons/win_explorer.png")),
            win_properties: load_png(ctx, "rbx_win_properties", include_bytes!("../assets/icons/win_properties.png")),
            win_output: load_png(ctx, "rbx_win_output", include_bytes!("../assets/icons/win_output.png")),
            win_cmdbar: load_png(ctx, "rbx_win_cmdbar", include_bytes!("../assets/icons/win_cmdbar.png")),
            win_assetmgr: load_png(ctx, "rbx_win_assetmgr", include_bytes!("../assets/icons/win_assetmgr.png")),
            win_animeditor: load_png(ctx, "rbx_win_animeditor", include_bytes!("../assets/icons/win_animeditor.png")),
            win_buildrig: load_png(ctx, "rbx_win_buildrig", include_bytes!("../assets/icons/win_buildrig.png")),
            win_teamcreate: load_png(ctx, "rbx_win_teamcreate", include_bytes!("../assets/icons/win_teamcreate.png")),
            win_plugins: load_png(ctx, "rbx_win_plugins", include_bytes!("../assets/icons/win_plugins.png")),

            sim_play: load_png(ctx, "rbx_sim_play", include_bytes!("../assets/icons/sim_play.png")),
            sim_playhere: load_png(ctx, "rbx_sim_playhere", include_bytes!("../assets/icons/sim_playhere.png")),
            sim_run: load_png(ctx, "rbx_sim_run", include_bytes!("../assets/icons/sim_run.png")),
            sim_pause: load_png(ctx, "rbx_sim_pause", include_bytes!("../assets/icons/sim_pause.png")),
            sim_stop: load_png(ctx, "rbx_sim_stop", include_bytes!("../assets/icons/sim_stop.png")),

            action_anchor: load_png(ctx, "rbx_action_anchor", include_bytes!("../assets/icons/action_anchor.png")),
            action_gamesettings: load_png(ctx, "rbx_action_gamesettings", include_bytes!("../assets/icons/action_gamesettings.png")),
            action_collab: load_png(ctx, "rbx_action_collab", include_bytes!("../assets/icons/action_collab.png")),
            qa_undo: load_png(ctx, "rbx_qa_undo", include_bytes!("../assets/icons/qa_undo.png")),
            qa_redo: load_png(ctx, "rbx_qa_redo", include_bytes!("../assets/icons/qa_redo.png")),
            icon_search: load_png(ctx, "rbx_icon_search", include_bytes!("../assets/icons/icon_search.png")),
            studio_logo: load_png(ctx, "rbx_studio_logo", include_bytes!("../assets/icons/roblox_studio_logo.png")),
            out_clear: load_png(ctx, "rbx_out_clear", include_bytes!("../assets/icons/output_clear.png")),
            out_more: load_png(ctx, "rbx_out_more", include_bytes!("../assets/icons/output_more.png")),
            doc_place_cloud: load_png(ctx, "rbx_doc_place_cloud", include_bytes!("../assets/icons/place_cloud.png")),

            view_front: load_png(ctx, "rbx_view_front", include_bytes!("../assets/icons/view_front.png")),
            view_top: load_png(ctx, "rbx_view_top", include_bytes!("../assets/icons/view_top.png")),
            view_right: load_png(ctx, "rbx_view_right", include_bytes!("../assets/icons/view_right.png")),
        }
    }
}
