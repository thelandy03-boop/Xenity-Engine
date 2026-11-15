// SPDX-License-Identifier: MIT
//
// Copyright (c) 2022-2026 Gregory Machefer (Fewnity)
//
// This file is part of Xenity Engine

#pragma once

#include <memory>
#include <string>

#if defined(_WIN32) || defined(_WIN64)
#define XENITY_API __declspec(dllexport)
#else
#define XENITY_API __attribute__((visibility("default")))
#endif

class Camera;
class GameObject;

#pragma pack(push, 1)
struct XenityGameObjectInfo
{
	int index;
	char name[64];
	float pos[3];
	float rot[3];
	float scale[3];
	float color[4];
	int has_mesh;
	int has_collider;
	int has_light;
	int is_active;
};
#pragma pack(pop)

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

	static std::shared_ptr<class Texture> LoadTextureDirect(const std::string& relativePath);

	// Gizmos & Selection
	static void SetToolMode(int mode);
	static void SelectGameObject(int index);
	static int GetSelectedGameObject();
	static int PickGameObject(float norm_x, float norm_y);
	static void TransformSelected(float delta_x, float delta_y, int tool_mode, int axis);

	// Hierarchy & Properties Sync
	static int GetGameObjectInfo(int index, XenityGameObjectInfo* out_info);
	static void SetGameObjectTransform(int index, float px, float py, float pz, float rx, float ry, float rz, float sx, float sy, float sz);
	static void SetGameObjectColor(int index, float r, float g, float b, float a);
	static void SetGameObjectName(int index, const char* name);
	static void DeleteGameObject(int index);

	// Project Export & Cooking
	static int CookProject(int platform_id, const char* export_path);

private:
	static std::shared_ptr<Camera> s_camera;
	static int s_width;
	static int s_height;
	static bool s_initialized;
	static int s_currentToolMode;
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

	XENITY_API void xenity_set_tool_mode(int mode);
	XENITY_API void xenity_select_gameobject(int index);
	XENITY_API int xenity_get_selected_gameobject();
	XENITY_API int xenity_pick_gameobject(float norm_x, float norm_y);
	XENITY_API void xenity_transform_selected(float delta_x, float delta_y, int tool_mode, int axis);

	XENITY_API int xenity_get_gameobject_info(int index, XenityGameObjectInfo* out_info);
	XENITY_API void xenity_set_gameobject_transform(int index, float px, float py, float pz, float rx, float ry, float rz, float sx, float sy, float sz);
	XENITY_API void xenity_set_gameobject_color(int index, float r, float g, float b, float a);
	XENITY_API void xenity_set_gameobject_name(int index, const char* name);
	XENITY_API void xenity_delete_gameobject(int index);

	XENITY_API int xenity_cook_project(int platform_id, const char* export_path);
}
