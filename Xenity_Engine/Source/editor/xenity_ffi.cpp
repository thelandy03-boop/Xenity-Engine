// SPDX-License-Identifier: MIT
//
// Copyright (c) 2022-2026 Gregory Machefer (Fewnity)
//
// This file is part of Xenity Engine

#include "xenity_ffi.h"

#include <filesystem>
#include <memory>
#include <vector>
#include <cstring>
#include <algorithm>

#include <glad/gl.h>
#include <SDL3/SDL.h>

#include <engine/engine.h>
#include <engine/ui/window.h>
#include <engine/graphics/graphics.h>
#include <engine/graphics/camera.h>
#include <engine/game_elements/gameobject.h>
#include <engine/game_elements/transform.h>
#include <engine/time/time.h>
#include <engine/inputs/input_system.h>
#include <engine/physics/physics_manager.h>
#include <engine/game_elements/gameplay_manager.h>
#include <engine/asset_management/project_manager.h>
#include <engine/scene_management/scene_manager.h>
#include <engine/tools/shape_spawner.h>
#include <engine/lighting/lighting.h>

#include <editor/editor.h>
#include <editor/ui/menus/basic/scene_menu.h>

std::shared_ptr<Camera> XenityFFI::s_camera = nullptr;
int XenityFFI::s_width = 1280;
int XenityFFI::s_height = 720;
bool XenityFFI::s_initialized = false;

int XenityFFI::Init(int width, int height)
{
	if (s_initialized)
	{
		Resize(width, height);
		return 0;
	}

	if (width <= 0) width = 1280;
	if (height <= 0) height = 720;
	s_width = width;
	s_height = height;

	// 1. Initialize Engine
	const int initRes = Engine::Init(0, nullptr);
	if (initRes != 0)
	{
		return initRes;
	}

	// 2. Hide SDL host window so rendering is embedded in Rust viewport
	if (Window::s_window)
	{
		SDL_HideWindow(Window::s_window);
	}

	// 3. Ensure a project and scene are loaded
	if (ProjectManager::GetProjectState() != ProjectState::Loaded)
	{
		if (!std::filesystem::exists("./DefaultProject/assets/"))
		{
			(void)ProjectManager::CreateProject("DefaultProject", "./");
		}
		(void)ProjectManager::LoadProject("./DefaultProject/");
	}

	// 4. Retrieve or create Editor Camera
	auto sceneMenu = Editor::GetMenu<SceneMenu>();
	if (sceneMenu && !sceneMenu->weakCamera.expired())
	{
		s_camera = sceneMenu->weakCamera.lock();
	}

	if (!s_camera)
	{
		auto camGO = CreateGameObjectEditor("EditorCamera");
		s_camera = camGO->AddComponent<Camera>();
		s_camera->SetNearClippingPlane(0.05f);
		s_camera->SetFarClippingPlane(2000.0f);
		s_camera->SetFov(70.0f);
		s_camera->SetIsEditor(true);
	}

	if (s_camera)
	{
		s_camera->GetTransform()->SetPosition(Vector3(0.0f, 6.0f, -14.0f));
		s_camera->GetTransform()->SetEulerAngles(Vector3(20.0f, 0.0f, 0.0f));
		s_camera->ChangeFrameBufferSize(Vector2Int(s_width, s_height));
	}

	// 5. Ensure Baseplate and Sunlight exist
	bool hasSun = false;
	bool hasBaseplate = false;
	const int goCount = GameplayManager::gameObjectCount;
	for (int i = 0; i < goCount; i++)
	{
		const auto& go = GameplayManager::GetGameObjects()[i];
		if (go)
		{
			if (go->GetName() == "Sun" || go->GetComponent<Light>())
				hasSun = true;
			if (go->GetName() == "Baseplate")
				hasBaseplate = true;
		}
	}

	if (!hasSun)
	{
		auto sunGO = CreateGameObject("Sun");
		auto sunLight = sunGO->AddComponent<Light>();
		sunLight->SetupDirectionalLight(Color::CreateFromRGBAFloat(1.0f, 0.98f, 0.92f, 1.0f), 0.8f);
		sunGO->GetTransform()->SetEulerAngles(Vector3(45.0f, -35.0f, 0.0f));
	}

	if (!hasBaseplate)
	{
		auto bp = ShapeSpawner::SpawnCube();
		if (bp)
		{
			bp->SetName("Baseplate");
			bp->GetTransform()->SetPosition(Vector3(0.0f, -0.5f, 0.0f));
			bp->GetTransform()->SetLocalScale(Vector3(50.0f, 1.0f, 50.0f));
		}
	}

	s_initialized = true;
	return 0;
}

void XenityFFI::Tick()
{
	if (!s_initialized)
		return;

	Time::UpdateTime();
	InputSystem::ClearInputs();

	// Process SDL events
	InputSystem::UpdateControllers();
	SDL_Event event;
	while (SDL_PollEvent(&event))
	{
		InputSystem::Read(event);
	}

	// Update game simulation if playing
	if (GameplayManager::GetGameState() == GameState::Playing)
	{
		PhysicsManager::Update();
		GameplayManager::UpdateComponents();
	}

	GameplayManager::RemoveDestroyedGameObjects();
	GameplayManager::RemoveDestroyedComponents();

	// Render frame to camera framebuffers
	Graphics::Draw();
}

void XenityFFI::Resize(int width, int height)
{
	if (width <= 0 || height <= 0)
		return;

	s_width = width;
	s_height = height;

	if (s_camera)
	{
		s_camera->ChangeFrameBufferSize(Vector2Int(width, height));
	}
}

int XenityFFI::ReadFrame(unsigned char* out_pixels, int width, int height)
{
	if (!s_initialized || !s_camera || !out_pixels || width <= 0 || height <= 0)
	{
		return 0;
	}

	// Make sure camera framebuffer size matches
	if (s_camera->GetWidth() != width || s_camera->GetHeight() != height)
	{
		s_camera->ChangeFrameBufferSize(Vector2Int(width, height));
		Graphics::Draw();
	}

	// Read RGBA framebuffer texture
	glBindTexture(GL_TEXTURE_2D, s_camera->m_secondFramebufferTexture);
	glPixelStorei(GL_PACK_ALIGNMENT, 1);
	glGetTexImage(GL_TEXTURE_2D, 0, GL_RGBA, GL_UNSIGNED_BYTE, out_pixels);
	glBindTexture(GL_TEXTURE_2D, 0);

	// Flip vertically for top-left UI origin
	const int row_size = width * 4;
	std::vector<unsigned char> temp_row(row_size);
	for (int y = 0; y < height / 2; ++y)
	{
		unsigned char* row_top = out_pixels + y * row_size;
		unsigned char* row_bottom = out_pixels + (height - 1 - y) * row_size;
		std::memcpy(temp_row.data(), row_top, row_size);
		std::memcpy(row_top, row_bottom, row_size);
		std::memcpy(row_bottom, temp_row.data(), row_size);
	}

	return 1;
}

void XenityFFI::CameraOrbit(float delta_yaw, float delta_pitch, float move_fwd, float move_right, float move_up)
{
	if (!s_camera)
		return;

	auto camTrans = s_camera->GetTransform();
	if (!camTrans)
		return;

	// Rotation
	Vector3 rot = camTrans->GetEulerAngles();
	rot.x += delta_pitch;
	rot.y += delta_yaw;
	if (rot.x > 89.0f) rot.x = 89.0f;
	if (rot.x < -89.0f) rot.x = -89.0f;
	camTrans->SetEulerAngles(rot);

	// Translation
	Vector3 pos = camTrans->GetPosition();
	pos -= camTrans->GetForward() * move_fwd;
	pos -= camTrans->GetLeft() * move_right;
	pos += Vector3(0.0f, 1.0f, 0.0f) * move_up;
	camTrans->SetPosition(pos);
}

void XenityFFI::SpawnPrimitive(int primitive_type)
{
	std::shared_ptr<GameObject> spawned = nullptr;
	switch (primitive_type)
	{
	case 0: spawned = ShapeSpawner::SpawnCube(); break;
	case 1: spawned = ShapeSpawner::SpawnSphere(); break;
	case 2: spawned = ShapeSpawner::SpawnCylinder(); break;
	case 3: spawned = ShapeSpawner::SpawnPlane(); break;
	case 4: spawned = ShapeSpawner::SpawnCone(); break;
	case 5: spawned = ShapeSpawner::SpawnDonut(); break;
	default: spawned = ShapeSpawner::SpawnCube(); break;
	}

	if (spawned && s_camera)
	{
		Vector3 camPos = s_camera->GetTransform()->GetPosition();
		Vector3 camFwd = -s_camera->GetTransform()->GetForward();
		Vector3 spawnPos = camPos + camFwd * 5.0f;
		if (spawnPos.y < 0.5f) spawnPos.y = 0.5f;
		spawned->GetTransform()->SetPosition(spawnPos);
	}
}

void XenityFFI::SetSimState(int state)
{
	GameState gs = GameState::Stopped;
	if (state == 1) gs = GameState::Playing;
	else if (state == 2) gs = GameState::Paused;
	GameplayManager::SetGameState(gs, true);
}

int XenityFFI::GetGameObjectCount()
{
	return GameplayManager::gameObjectCount;
}

void XenityFFI::Shutdown()
{
	if (!s_initialized)
		return;

	s_camera.reset();
	Engine::Stop();
	s_initialized = false;
}

extern "C" {

XENITY_API int xenity_init_editor(int width, int height)
{
	return XenityFFI::Init(width, height);
}

XENITY_API void xenity_tick()
{
	XenityFFI::Tick();
}

XENITY_API void xenity_resize(int width, int height)
{
	XenityFFI::Resize(width, height);
}

XENITY_API int xenity_read_frame(unsigned char* out_pixels, int width, int height)
{
	return XenityFFI::ReadFrame(out_pixels, width, height);
}

XENITY_API void xenity_camera_orbit(float delta_yaw, float delta_pitch, float move_fwd, float move_right, float move_up)
{
	XenityFFI::CameraOrbit(delta_yaw, delta_pitch, move_fwd, move_right, move_up);
}

XENITY_API void xenity_spawn_primitive(int primitive_type)
{
	XenityFFI::SpawnPrimitive(primitive_type);
}

XENITY_API void xenity_set_sim_state(int state)
{
	XenityFFI::SetSimState(state);
}

XENITY_API int xenity_get_gameobject_count()
{
	return XenityFFI::GetGameObjectCount();
}

XENITY_API void xenity_shutdown()
{
	XenityFFI::Shutdown();
}

}
