// SPDX-License-Identifier: MIT
//
// Integration bridge to C++ Xenity Engine (Xenity_Editor.dll)

use libloading::{Library, Symbol};
use std::path::{Path, PathBuf};
use log::{info, warn, error};

type FnInit = unsafe extern "C" fn(i32, i32) -> i32;
type FnTick = unsafe extern "C" fn();
type FnResize = unsafe extern "C" fn(i32, i32);
type FnReadFrame = unsafe extern "C" fn(*mut u8, i32, i32) -> i32;
type FnCameraOrbit = unsafe extern "C" fn(f32, f32, f32, f32, f32);
type FnSpawnPrimitive = unsafe extern "C" fn(i32);
type FnSetSimState = unsafe extern "C" fn(i32);
type FnGetGameObjectCount = unsafe extern "C" fn() -> i32;
type FnShutdown = unsafe extern "C" fn();

type FnSetToolMode = unsafe extern "C" fn(i32);
type FnSelectGameObject = unsafe extern "C" fn(i32);
type FnGetSelectedGameObject = unsafe extern "C" fn() -> i32;
type FnPickGameObject = unsafe extern "C" fn(f32, f32) -> i32;
type FnTransformSelected = unsafe extern "C" fn(f32, f32, i32, i32);

type FnGetGameObjectInfo = unsafe extern "C" fn(i32, *mut XenityGameObjectInfo) -> i32;
type FnSetGameObjectTransform = unsafe extern "C" fn(i32, f32, f32, f32, f32, f32, f32, f32, f32, f32);
type FnSetGameObjectColor = unsafe extern "C" fn(i32, f32, f32, f32, f32);
type FnSetGameObjectName = unsafe extern "C" fn(i32, *const std::os::raw::c_char);
type FnDeleteGameObject = unsafe extern "C" fn(i32);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XenityGameObjectInfo {
    pub index: i32,
    pub name: [u8; 64],
    pub pos: [f32; 3],
    pub rot: [f32; 3],
    pub scale: [f32; 3],
    pub color: [f32; 4],
    pub has_mesh: i32,
    pub has_collider: i32,
    pub has_light: i32,
    pub is_active: i32,
}

impl Default for XenityGameObjectInfo {
    fn default() -> Self {
        Self {
            index: -1,
            name: [0; 64],
            pos: [0.0; 3],
            rot: [0.0; 3],
            scale: [1.0; 3],
            color: [1.0; 4],
            has_mesh: 0,
            has_collider: 0,
            has_light: 0,
            is_active: 0,
        }
    }
}

impl XenityGameObjectInfo {
    pub fn name_as_str(&self) -> &str {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(self.name.len());
        std::str::from_utf8(&self.name[..len]).unwrap_or("GameObject")
    }
}

pub struct XenityEngine {
    _lib: Library,
    fn_init: FnInit,
    fn_tick: FnTick,
    fn_resize: FnResize,
    fn_read_frame: FnReadFrame,
    fn_camera_orbit: FnCameraOrbit,
    fn_spawn_primitive: FnSpawnPrimitive,
    fn_set_sim_state: FnSetSimState,
    fn_get_gameobject_count: FnGetGameObjectCount,
    fn_shutdown: FnShutdown,

    fn_set_tool_mode: FnSetToolMode,
    fn_select_gameobject: FnSelectGameObject,
    fn_get_selected_gameobject: FnGetSelectedGameObject,
    fn_pick_gameobject: FnPickGameObject,
    fn_transform_selected: FnTransformSelected,

    fn_get_gameobject_info: FnGetGameObjectInfo,
    fn_set_gameobject_transform: FnSetGameObjectTransform,
    fn_set_gameobject_color: FnSetGameObjectColor,
    fn_set_gameobject_name: FnSetGameObjectName,
    fn_delete_gameobject: FnDeleteGameObject,

    pub is_initialized: bool,
    pub width: u32,
    pub height: u32,
}

impl XenityEngine {
    pub fn try_load() -> Option<Self> {
        let candidates = [
            PathBuf::from("Xenity_Editor.dll"),
            PathBuf::from("xenity-editor/Xenity_Editor.dll"),
            PathBuf::from("../XenityBuild/x64/Release Editor/Xenity_Editor.dll"),
            PathBuf::from("XenityBuild/x64/Release Editor/Xenity_Editor.dll"),
            PathBuf::from("C:/Landy_proyectos/Xenity-Engine/XenityBuild/x64/Release Editor/Xenity_Editor.dll"),
        ];

        let mut found_path: Option<PathBuf> = None;
        for path in &candidates {
            if path.exists() {
                found_path = Some(path.clone());
                break;
            }
        }

        let dll_path = match found_path {
            Some(p) => p,
            None => {
                warn!("[Xenity FFI] Could not locate Xenity_Editor.dll in candidate paths");
                return None;
            }
        };

        info!("[Xenity FFI] Loading dynamic library from {:?}", dll_path);
        let lib = unsafe {
            match Library::new(&dll_path) {
                Ok(l) => l,
                Err(e) => {
                    error!("[Xenity FFI] Failed to load {:?}: {}", dll_path, e);
                    return None;
                }
            }
        };

        unsafe {
            let fn_init: Symbol<FnInit> = match lib.get(b"xenity_init_editor") {
                Ok(s) => s,
                Err(e) => { error!("Symbol xenity_init_editor: {}", e); return None; }
            };
            let fn_tick: Symbol<FnTick> = match lib.get(b"xenity_tick") {
                Ok(s) => s,
                Err(e) => { error!("Symbol xenity_tick: {}", e); return None; }
            };
            let fn_resize: Symbol<FnResize> = match lib.get(b"xenity_resize") {
                Ok(s) => s,
                Err(e) => { error!("Symbol xenity_resize: {}", e); return None; }
            };
            let fn_read_frame: Symbol<FnReadFrame> = match lib.get(b"xenity_read_frame") {
                Ok(s) => s,
                Err(e) => { error!("Symbol xenity_read_frame: {}", e); return None; }
            };
            let fn_camera_orbit: Symbol<FnCameraOrbit> = match lib.get(b"xenity_camera_orbit") {
                Ok(s) => s,
                Err(e) => { error!("Symbol xenity_camera_orbit: {}", e); return None; }
            };
            let fn_spawn_primitive: Symbol<FnSpawnPrimitive> = match lib.get(b"xenity_spawn_primitive") {
                Ok(s) => s,
                Err(e) => { error!("Symbol xenity_spawn_primitive: {}", e); return None; }
            };
            let fn_set_sim_state: Symbol<FnSetSimState> = match lib.get(b"xenity_set_sim_state") {
                Ok(s) => s,
                Err(e) => { error!("Symbol xenity_set_sim_state: {}", e); return None; }
            };
            let fn_get_gameobject_count: Symbol<FnGetGameObjectCount> = match lib.get(b"xenity_get_gameobject_count") {
                Ok(s) => s,
                Err(e) => { error!("Symbol xenity_get_gameobject_count: {}", e); return None; }
            };
            let fn_shutdown: Symbol<FnShutdown> = match lib.get(b"xenity_shutdown") {
                Ok(s) => s,
                Err(e) => { error!("Symbol xenity_shutdown: {}", e); return None; }
            };

            let fn_set_tool_mode: Symbol<FnSetToolMode> = match lib.get(b"xenity_set_tool_mode") {
                Ok(s) => s,
                Err(e) => { error!("Symbol xenity_set_tool_mode: {}", e); return None; }
            };
            let fn_select_gameobject: Symbol<FnSelectGameObject> = match lib.get(b"xenity_select_gameobject") {
                Ok(s) => s,
                Err(e) => { error!("Symbol xenity_select_gameobject: {}", e); return None; }
            };
            let fn_get_selected_gameobject: Symbol<FnGetSelectedGameObject> = match lib.get(b"xenity_get_selected_gameobject") {
                Ok(s) => s,
                Err(e) => { error!("Symbol xenity_get_selected_gameobject: {}", e); return None; }
            };
            let fn_pick_gameobject: Symbol<FnPickGameObject> = match lib.get(b"xenity_pick_gameobject") {
                Ok(s) => s,
                Err(e) => { error!("Symbol xenity_pick_gameobject: {}", e); return None; }
            };
            let fn_transform_selected: Symbol<FnTransformSelected> = match lib.get(b"xenity_transform_selected") {
                Ok(s) => s,
                Err(e) => { error!("Symbol xenity_transform_selected: {}", e); return None; }
            };

            let fn_get_gameobject_info: Symbol<FnGetGameObjectInfo> = match lib.get(b"xenity_get_gameobject_info") {
                Ok(s) => s,
                Err(e) => { error!("Symbol xenity_get_gameobject_info: {}", e); return None; }
            };
            let fn_set_gameobject_transform: Symbol<FnSetGameObjectTransform> = match lib.get(b"xenity_set_gameobject_transform") {
                Ok(s) => s,
                Err(e) => { error!("Symbol xenity_set_gameobject_transform: {}", e); return None; }
            };
            let fn_set_gameobject_color: Symbol<FnSetGameObjectColor> = match lib.get(b"xenity_set_gameobject_color") {
                Ok(s) => s,
                Err(e) => { error!("Symbol xenity_set_gameobject_color: {}", e); return None; }
            };
            let fn_set_gameobject_name: Symbol<FnSetGameObjectName> = match lib.get(b"xenity_set_gameobject_name") {
                Ok(s) => s,
                Err(e) => { error!("Symbol xenity_set_gameobject_name: {}", e); return None; }
            };
            let fn_delete_gameobject: Symbol<FnDeleteGameObject> = match lib.get(b"xenity_delete_gameobject") {
                Ok(s) => s,
                Err(e) => { error!("Symbol xenity_delete_gameobject: {}", e); return None; }
            };

            info!("[Xenity FFI] Successfully bound all exported engine symbols including Gizmos & Sync!");
            Some(Self {
                fn_init: *fn_init,
                fn_tick: *fn_tick,
                fn_resize: *fn_resize,
                fn_read_frame: *fn_read_frame,
                fn_camera_orbit: *fn_camera_orbit,
                fn_spawn_primitive: *fn_spawn_primitive,
                fn_set_sim_state: *fn_set_sim_state,
                fn_get_gameobject_count: *fn_get_gameobject_count,
                fn_shutdown: *fn_shutdown,

                fn_set_tool_mode: *fn_set_tool_mode,
                fn_select_gameobject: *fn_select_gameobject,
                fn_get_selected_gameobject: *fn_get_selected_gameobject,
                fn_pick_gameobject: *fn_pick_gameobject,
                fn_transform_selected: *fn_transform_selected,

                fn_get_gameobject_info: *fn_get_gameobject_info,
                fn_set_gameobject_transform: *fn_set_gameobject_transform,
                fn_set_gameobject_color: *fn_set_gameobject_color,
                fn_set_gameobject_name: *fn_set_gameobject_name,
                fn_delete_gameobject: *fn_delete_gameobject,

                _lib: lib,
                is_initialized: false,
                width: 0,
                height: 0,
            })
        }
    }

    pub fn init(&mut self, width: u32, height: u32) -> bool {
        let w = width.max(320) as i32;
        let h = height.max(240) as i32;
        let res = unsafe { (self.fn_init)(w, h) };
        if res == 0 {
            self.is_initialized = true;
            self.width = width;
            self.height = height;
            info!("[Xenity FFI] Engine initialized ({}x{})", width, height);
            true
        } else {
            error!("[Xenity FFI] Engine init failed with code {}", res);
            false
        }
    }

    pub fn tick(&mut self) {
        if self.is_initialized {
            unsafe { (self.fn_tick)() };
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if self.is_initialized && (width != self.width || height != self.height) {
            self.width = width.max(1);
            self.height = height.max(1);
            unsafe { (self.fn_resize)(self.width as i32, self.height as i32) };
        }
    }

    pub fn read_frame(&mut self, width: u32, height: u32, buffer: &mut Vec<u8>) -> bool {
        if !self.is_initialized || width == 0 || height == 0 {
            return false;
        }

        let total_bytes = (width as usize) * (height as usize) * 4;
        if buffer.len() != total_bytes {
            buffer.resize(total_bytes, 0);
        }

        let ret = unsafe { (self.fn_read_frame)(buffer.as_mut_ptr(), width as i32, height as i32) };
        ret != 0
    }

    pub fn camera_orbit(&mut self, delta_yaw: f32, delta_pitch: f32, move_fwd: f32, move_right: f32, move_up: f32) {
        if self.is_initialized {
            unsafe { (self.fn_camera_orbit)(delta_yaw, delta_pitch, move_fwd, move_right, move_up) };
        }
    }

    pub fn spawn_primitive(&mut self, prim_type: i32) {
        if self.is_initialized {
            unsafe { (self.fn_spawn_primitive)(prim_type) };
        }
    }

    pub fn set_sim_state(&mut self, state: i32) {
        if self.is_initialized {
            unsafe { (self.fn_set_sim_state)(state) };
        }
    }

    pub fn get_gameobject_count(&mut self) -> i32 {
        if self.is_initialized {
            unsafe { (self.fn_get_gameobject_count)() }
        } else {
            0
        }
    }

    pub fn set_tool_mode(&mut self, mode: i32) {
        if self.is_initialized {
            unsafe { (self.fn_set_tool_mode)(mode) };
        }
    }

    pub fn select_gameobject(&mut self, index: i32) {
        if self.is_initialized {
            unsafe { (self.fn_select_gameobject)(index) };
        }
    }

    pub fn get_selected_gameobject(&mut self) -> i32 {
        if self.is_initialized {
            unsafe { (self.fn_get_selected_gameobject)() }
        } else {
            -1
        }
    }

    pub fn pick_gameobject(&mut self, norm_x: f32, norm_y: f32) -> i32 {
        if self.is_initialized {
            unsafe { (self.fn_pick_gameobject)(norm_x, norm_y) }
        } else {
            -1
        }
    }

    pub fn transform_selected(&mut self, delta_x: f32, delta_y: f32, tool_mode: i32, axis: i32) {
        if self.is_initialized {
            unsafe { (self.fn_transform_selected)(delta_x, delta_y, tool_mode, axis) };
        }
    }

    pub fn get_gameobject_info(&mut self, index: i32) -> Option<XenityGameObjectInfo> {
        if !self.is_initialized {
            return None;
        }
        let mut info = XenityGameObjectInfo::default();
        let ret = unsafe { (self.fn_get_gameobject_info)(index, &mut info) };
        if ret != 0 {
            Some(info)
        } else {
            None
        }
    }

    pub fn set_gameobject_transform(
        &mut self,
        index: i32,
        pos: [f32; 3],
        rot: [f32; 3],
        scale: [f32; 3],
    ) {
        if self.is_initialized {
            unsafe {
                (self.fn_set_gameobject_transform)(
                    index,
                    pos[0], pos[1], pos[2],
                    rot[0], rot[1], rot[2],
                    scale[0], scale[1], scale[2],
                )
            };
        }
    }

    pub fn set_gameobject_color(&mut self, index: i32, color: [f32; 4]) {
        if self.is_initialized {
            unsafe {
                (self.fn_set_gameobject_color)(index, color[0], color[1], color[2], color[3])
            };
        }
    }

    pub fn set_gameobject_name(&mut self, index: i32, name: &str) {
        if self.is_initialized {
            if let Ok(c_str) = std::ffi::CString::new(name) {
                unsafe { (self.fn_set_gameobject_name)(index, c_str.as_ptr()) };
            }
        }
    }

    pub fn delete_gameobject(&mut self, index: i32) {
        if self.is_initialized {
            unsafe { (self.fn_delete_gameobject)(index) };
        }
    }

    pub fn shutdown(&mut self) {
        if self.is_initialized {
            unsafe { (self.fn_shutdown)() };
            self.is_initialized = false;
        }
    }
}

impl Drop for XenityEngine {
    fn drop(&mut self) {
        self.shutdown();
    }
}
