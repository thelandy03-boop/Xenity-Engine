// SPDX-License-Identifier: MIT
//
// Copyright (c) 2022-2026 Gregory Machefer (Fewnity)
//
// This file is part of Xenity Engine

#pragma once

#include <memory>

#ifdef _WIN32
#define XENITY_API __declspec(dllexport)
#else
#define XENITY_API
#endif

class Camera;

class XenityFFI
{
public:
	static int Init(int width, int height);
	static void Tick();
	static void Resize(int width, int height);
	static int ReadFrame(unsigned char* out_pixels, int width, int height);
	static void CameraOrbit(float delta_yaw, float delta_pitch, float move_fwd, float move_right, float move_up);
	static void SpawnPrimitive(int primitive_type);
	static void SetSimState(int state);
	static int GetGameObjectCount();
	static void Shutdown();

private:
	static std::shared_ptr<Camera> s_camera;
	static int s_width;
	static int s_height;
	static bool s_initialized;
};

extern "C" {
	XENITY_API int xenity_init_editor(int width, int height);
	XENITY_API void xenity_tick();
	XENITY_API void xenity_resize(int width, int height);
	XENITY_API int xenity_read_frame(unsigned char* out_pixels, int width, int height);
	XENITY_API void xenity_camera_orbit(float delta_yaw, float delta_pitch, float move_fwd, float move_right, float move_up);
	XENITY_API void xenity_spawn_primitive(int primitive_type);
	XENITY_API void xenity_set_sim_state(int state);
	XENITY_API int xenity_get_gameobject_count();
	XENITY_API void xenity_shutdown();
}
