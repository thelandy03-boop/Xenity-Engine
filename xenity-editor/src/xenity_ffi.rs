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

            info!("[Xenity FFI] Successfully bound all exported engine symbols!");
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
