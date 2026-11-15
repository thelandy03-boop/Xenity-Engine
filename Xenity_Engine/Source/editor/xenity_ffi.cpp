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
#include <engine/graphics/material.h>
#include <engine/graphics/3d_graphics/mesh_renderer.h>
#include <engine/graphics/3d_graphics/mesh_data.h>
#include <engine/game_elements/gameobject.h>
#include <engine/game_elements/transform.h>
#include <engine/time/time.h>
#include <engine/inputs/input_system.h>
#include <engine/physics/physics_manager.h>
#include <engine/physics/box_collider.h>
#include <engine/physics/sphere_collider.h>
#include <engine/game_elements/gameplay_manager.h>
#include <engine/asset_management/project_manager.h>
#include <engine/asset_management/asset_manager.h>
#include <engine/scene_management/scene_manager.h>
#include <engine/tools/shape_spawner.h>
#include <engine/lighting/lighting.h>
#include <engine/tools/gameplay_utility.h>
#include <engine/file_system/file_system.h>
#include <engine/file_system/file.h>
#include <engine/graphics/skybox.h>
#include <engine/graphics/texture/texture.h>
#include <engine/graphics/texture/texture_default.h>
#include <engine/math/quaternion.h>

#include <editor/editor.h>
#include <editor/ui/menus/basic/scene_menu.h>
#include <editor/tool_mode.h>
#include <editor/cooker/cooker.h>

std::shared_ptr<Camera> XenityFFI::s_camera = nullptr;
int XenityFFI::s_width = 1280;
int XenityFFI::s_height = 720;
bool XenityFFI::s_initialized = false;
int XenityFFI::s_currentToolMode = 1; // 1 = ToolMode::Tool_Move

static bool GetAABBDistance(const Vector3& corner1, const Vector3& corner2, const Vector3& dirfrac, const Vector3& startPosition, float* t)
{
	float t1 = (corner1.x - startPosition.x) * dirfrac.x;
	float t2 = (corner2.x - startPosition.x) * dirfrac.x;
	float t3 = (corner1.y - startPosition.y) * dirfrac.y;
	float t4 = (corner2.y - startPosition.y) * dirfrac.y;
	float t5 = (corner1.z - startPosition.z) * dirfrac.z;
	float t6 = (corner2.z - startPosition.z) * dirfrac.z;

	float tmin = std::max(std::max(std::min(t1, t2), std::min(t3, t4)), std::min(t5, t6));
	float tmax = std::min(std::min(std::max(t1, t2), std::max(t3, t4)), std::max(t5, t6));

	if (tmax < 0.0f || tmin > tmax)
	{
		return false;
	}
	else
	{
		if (*t > tmin)
		{
			*t = tmin;
			return true;
		}
	}
	return false;
}

std::shared_ptr<Texture> XenityFFI::LoadTextureDirect(const std::string& relativePath)
{
	std::string finalPath = relativePath;
	if (!std::filesystem::exists(finalPath))
	{
		finalPath = "../" + relativePath;
	}
	if (!std::filesystem::exists(finalPath))
	{
		return nullptr;
	}

	auto tex = Texture::MakeTexture(true);
	tex->m_file = FileSystem::MakeFile(finalPath);
	tex->SetFilter(Filter::Bilinear);
	tex->SetUseMipmap(true);
	tex->LoadTexture();
	tex->OnLoadFileReferenceFinished();
	return tex;
}

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
		sceneMenu->toolMode = ToolMode::Tool_Move;
	}

	if (!s_camera)
	{
		auto camGO = CreateGameObjectEditor("EditorCamera");
		s_camera = camGO->AddComponent<Camera>();
		s_camera->SetNearClippingPlane(0.5f);
		s_camera->SetFarClippingPlane(4000.0f);
		s_camera->SetFov(70.0f);
		s_camera->SetIsEditor(true);
	}

	if (s_camera)
	{
		s_camera->SetNearClippingPlane(0.5f);
		s_camera->SetFarClippingPlane(4000.0f);
		s_camera->GetTransform()->SetPosition(Vector3(0.0f, 16.0f, -40.0f));
		s_camera->GetTransform()->SetEulerAngles(Vector3(20.0f, 0.0f, 0.0f));
		s_camera->ChangeFrameBufferSize(Vector2Int(s_width, s_height));
	}

	// 5. Disable noisy debug wireframe grid (eliminates moiré/aliasing artifacts)
	Graphics::SetIsGridRenderingEnabled(false);

	// 5b. Ensure default white texture is loaded synchronously
	auto defTex = LoadTextureDirect("public_engine_assets/textures/default_texture.png");
	if (defTex)
	{
		AssetManager::defaultTexture = defTex;
	}

	// 6. Setup Authentic Roblox Skybox
	auto texSides = LoadTextureDirect("public_engine_assets/textures/roblox/sky_sides.png");
	auto texUp = LoadTextureDirect("public_engine_assets/textures/roblox/sky_up.png");
	auto texDown = LoadTextureDirect("public_engine_assets/textures/roblox/sky_down.png");
	if (texSides && texUp && texDown)
	{
		auto sky = std::make_shared<SkyBox>();
		sky->front = texSides;
		sky->back = texSides;
		sky->left = texSides;
		sky->right = texSides;
		sky->up = texUp;
		sky->down = texDown;
		Graphics::SetSkybox(sky);
	}
	else
	{
		// Clean atmospheric fallback sky color
		Graphics::s_settings.skyColor = Color::CreateFromRGBAFloat(0.53f, 0.70f, 0.90f, 1.0f);
	}

	// 7. Ensure Sunlight, Ambient Lighting and Baseplate exist with authentic Roblox visuals
	bool hasSun = false;
	bool hasAmbient = false;
	bool hasBaseplate = false;
	const int goCount = GameplayManager::gameObjectCount;
	for (int i = 0; i < goCount; i++)
	{
		const auto& go = GameplayManager::GetGameObjects()[i];
		if (go)
		{
			if (go->GetName() == "Sun")
			{
				hasSun = true;
				auto light = go->GetComponent<Light>();
				if (light)
				{
					light->SetupDirectionalLight(Color::CreateFromRGBAFloat(1.0f, 0.98f, 0.95f, 1.0f), 0.65f);
				}
			}
			if (go->GetName() == "Ambient")
			{
				hasAmbient = true;
				auto light = go->GetComponent<Light>();
				if (light)
				{
					light->SetupAmbientLight(Color::CreateFromRGBAFloat(0.65f, 0.70f, 0.78f, 1.0f), 0.50f);
				}
			}
			if (go->GetName() == "Baseplate")
			{
				hasBaseplate = true;
				// Update existing Baseplate to Roblox material & dimensions
				auto mesh = go->GetComponent<MeshRenderer>();
				if (mesh)
				{
					auto bpMat = Material::MakeMaterial();
					bpMat->SetShader(AssetManager::standardShader);
					auto bpTex = LoadTextureDirect("public_engine_assets/textures/roblox/baseplate_diffuse.png");
					if (bpTex)
					{
						bpTex->SetWrapMode(WrapMode::Repeat);
						bpMat->SetTexture(bpTex);
						bpMat->SetTiling(Vector2(64.0f, 64.0f));
					}
					bpMat->SetColor(Color::CreateFromRGBAFloat(1.0f, 1.0f, 1.0f, 1.0f));
					bpMat->SetUseLighting(true);
					mesh->SetMaterial(bpMat, 0);
					go->GetTransform()->SetPosition(Vector3(0.0f, -4.0f, 0.0f));
					go->GetTransform()->SetLocalScale(Vector3(512.0f, 8.0f, 512.0f));
				}
			}
		}
	}

	if (!hasSun)
	{
		auto sunGO = CreateGameObject("Sun");
		auto sunLight = sunGO->AddComponent<Light>();
		sunLight->SetupDirectionalLight(Color::CreateFromRGBAFloat(1.0f, 0.98f, 0.95f, 1.0f), 0.65f);
		sunGO->GetTransform()->SetEulerAngles(Vector3(50.0f, -30.0f, 0.0f));
	}

	if (!hasAmbient)
	{
		auto ambGO = CreateGameObject("Ambient");
		auto ambLight = ambGO->AddComponent<Light>();
		ambLight->SetupAmbientLight(Color::CreateFromRGBAFloat(0.65f, 0.70f, 0.78f, 1.0f), 0.50f);
	}

	if (!hasBaseplate)
	{
		auto bp = ShapeSpawner::SpawnCube();
		if (bp)
		{
			bp->SetName("Baseplate");
			bp->GetTransform()->SetPosition(Vector3(0.0f, -4.0f, 0.0f));
			bp->GetTransform()->SetLocalScale(Vector3(512.0f, 8.0f, 512.0f));

			auto mesh = bp->GetComponent<MeshRenderer>();
			if (mesh)
			{
				auto bpMat = Material::MakeMaterial();
				bpMat->SetShader(AssetManager::standardShader);
				auto bpTex = LoadTextureDirect("public_engine_assets/textures/roblox/baseplate_diffuse.png");
				if (bpTex)
				{
					bpTex->SetWrapMode(WrapMode::Repeat);
					bpMat->SetTexture(bpTex);
					bpMat->SetTiling(Vector2(64.0f, 64.0f));
				}
				bpMat->SetColor(Color::CreateFromRGBAFloat(1.0f, 1.0f, 1.0f, 1.0f));
				bpMat->SetUseLighting(true);
				mesh->SetMaterial(bpMat, 0);
			}
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

	// Rotation using quaternions for stable, gimbal-free orbit
	if (delta_yaw != 0.0f || delta_pitch != 0.0f)
	{
		const Quaternion& rotQ = camTrans->GetRotation();
		Quaternion rotX = Quaternion::AngleAxis(delta_pitch, Vector3(1, 0, 0));
		Quaternion rotY = Quaternion::AngleAxis(delta_yaw, Vector3(0, 1, 0));
		Quaternion newRot = rotY * rotQ * rotX;
		camTrans->SetRotation(newRot);
	}

	// Translation
	if (move_fwd != 0.0f || move_right != 0.0f || move_up != 0.0f)
	{
		Vector3 pos = camTrans->GetPosition();
		pos += camTrans->GetForward() * move_fwd;
		pos += camTrans->GetRight() * move_right;
		pos += Vector3(0.0f, 1.0f, 0.0f) * move_up;
		camTrans->SetPosition(pos);
	}
}

void XenityFFI::SpawnPrimitive(int primitive_type)
{
	std::shared_ptr<GameObject> spawned = nullptr;
	Vector3 defaultScale(4.0f, 1.2f, 2.0f);
	float halfHeight = 0.6f;

	switch (primitive_type)
	{
	case 0:
		spawned = ShapeSpawner::SpawnCube();
		if (spawned) spawned->SetName("Part");
		defaultScale = Vector3(4.0f, 1.2f, 2.0f);
		halfHeight = 0.6f;
		break;
	case 1:
		spawned = ShapeSpawner::SpawnSphere();
		if (spawned) spawned->SetName("SpherePart");
		defaultScale = Vector3(2.0f, 2.0f, 2.0f);
		halfHeight = 1.0f;
		break;
	case 2:
		spawned = ShapeSpawner::SpawnCylinder();
		if (spawned) spawned->SetName("CylinderPart");
		defaultScale = Vector3(2.0f, 2.0f, 2.0f);
		halfHeight = 1.0f;
		break;
	case 3:
		spawned = ShapeSpawner::SpawnPlane();
		if (spawned) spawned->SetName("PlanePart");
		defaultScale = Vector3(4.0f, 1.0f, 4.0f);
		halfHeight = 0.5f;
		break;
	case 4:
		spawned = ShapeSpawner::SpawnCone();
		if (spawned) spawned->SetName("WedgePart");
		defaultScale = Vector3(4.0f, 1.2f, 2.0f);
		halfHeight = 0.6f;
		break;
	case 5:
		spawned = ShapeSpawner::SpawnDonut();
		if (spawned) spawned->SetName("DonutPart");
		defaultScale = Vector3(2.0f, 2.0f, 2.0f);
		halfHeight = 1.0f;
		break;
	default:
		spawned = ShapeSpawner::SpawnCube();
		if (spawned) spawned->SetName("Part");
		defaultScale = Vector3(4.0f, 1.2f, 2.0f);
		halfHeight = 0.6f;
		break;
	}

	if (spawned)
	{
		spawned->GetTransform()->SetLocalScale(defaultScale);

		// Material Roblox clásico (Medium Stone Grey) con iluminación estándar
		if (auto mr = spawned->GetComponent<MeshRenderer>())
		{
			auto mat = Material::MakeMaterial();
			if (AssetManager::standardShader)
			{
				mat->SetShader(AssetManager::standardShader);
			}
			else if (AssetManager::standardMaterial)
			{
				mat->m_shader = AssetManager::standardMaterial->m_shader;
			}
			auto whiteTex = LoadTextureDirect("public_engine_assets/textures/default_texture.png");
			if (whiteTex)
			{
				mat->SetTexture(whiteTex);
			}
			else if (AssetManager::defaultTexture)
			{
				mat->SetTexture(AssetManager::defaultTexture);
			}
			mat->SetColor(Color::CreateFromRGBAFloat(0.639f, 0.635f, 0.647f, 1.0f));
			mat->SetUseLighting(true);
			mr->SetMaterial(mat, 0);
		}

		if (s_camera)
		{
			Vector3 camPos = s_camera->GetTransform()->GetPosition();
			Vector3 camFwd = s_camera->GetTransform()->GetForward(); // Positivo hacia adelante

			Vector3 spawnPos;
			// Si la cámara mira hacia abajo hacia la Baseplate, calcular la intersección sobre la superficie
			if (camFwd.y < -0.05f)
			{
				float t = (halfHeight - camPos.y) / camFwd.y;
				if (t > 1.0f && t < 120.0f)
				{
					spawnPos = camPos + camFwd * t;
				}
				else
				{
					spawnPos = camPos + camFwd * 12.0f;
				}
			}
			else
			{
				spawnPos = camPos + camFwd * 12.0f;
			}

			if (spawnPos.y < halfHeight)
			{
				spawnPos.y = halfHeight;
			}

			spawned->GetTransform()->SetPosition(spawnPos);
		}

		Editor::SetSelectedGameObject(spawned);
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

// -------------------------------------------------------------
// Gizmos & Selection Implementation
// -------------------------------------------------------------

void XenityFFI::SetToolMode(int mode)
{
	s_currentToolMode = mode;
	auto sceneMenu = Editor::GetMenu<SceneMenu>();
	if (sceneMenu)
	{
		// mode: 0: Tool_MoveCamera (Select), 1: Tool_Move, 2: Tool_Rotate, 3: Tool_Scale
		sceneMenu->toolMode = static_cast<ToolMode>(mode);
	}
}

void XenityFFI::SelectGameObject(int index)
{
	if (index >= 0 && index < GameplayManager::gameObjectCount)
	{
		auto go = GameplayManager::GetGameObjects()[index];
		Editor::SetSelectedGameObject(go);
	}
	else
	{
		Editor::SetSelectedGameObject(nullptr);
	}
}

int XenityFFI::GetSelectedGameObject()
{
	if (!Editor::GetSelectedGameObjects().empty())
	{
		auto selected = Editor::GetSelectedGameObjects()[0].lock();
		if (selected)
		{
			const int count = GameplayManager::gameObjectCount;
			for (int i = 0; i < count; i++)
			{
				if (GameplayManager::GetGameObjects()[i] == selected)
					return i;
			}
		}
	}
	return -1;
}

int XenityFFI::PickGameObject(float norm_x, float norm_y)
{
	if (!s_camera || norm_x < 0.0f || norm_y < 0.0f || norm_x > 1.0f || norm_y > 1.0f)
	{
		Editor::SetSelectedGameObject(nullptr);
		return -1;
	}

	InputSystem::mousePosition = Vector2(norm_x * s_camera->GetWidth(), norm_y * s_camera->GetHeight());
	const Vector3 dir = s_camera->GetMouseRay();
	if (dir.x == 0.0f && dir.y == 0.0f && dir.z == 0.0f)
	{
		return -1;
	}

	Vector3 dirfrac;
	dirfrac.x = 1.0f / (dir.x != 0.0f ? dir.x : 0.00001f);
	dirfrac.y = 1.0f / (dir.y != 0.0f ? dir.y : 0.00001f);
	dirfrac.z = 1.0f / (dir.z != 0.0f ? dir.z : 0.00001f);

	const Vector3 camPos = s_camera->GetTransform()->GetPosition();
	float minDis = 999999.0f;
	int hitIndex = -1;
	std::shared_ptr<GameObject> hitGo = nullptr;

	const int count = GameplayManager::gameObjectCount;
	for (int i = 0; i < count; i++)
	{
		auto go = GameplayManager::GetGameObjects()[i];
		if (!go || !go->IsActive() || go->GetName() == "EditorCamera" || go->GetName() == "Camera")
			continue;

		Vector3 transformedMin;
		Vector3 transformedMax;
		auto meshRenderer = go->GetComponent<MeshRenderer>();
		if (meshRenderer && meshRenderer->GetMeshData() && meshRenderer->IsEnabled())
		{
			const Vector3& minB = meshRenderer->GetMeshData()->GetMinBoundingBox();
			const Vector3& maxB = meshRenderer->GetMeshData()->GetMaxBoundingBox();
			glm::mat4 mat = go->GetTransform()->GetTransformationMatrix();
			transformedMin = Vector3(mat * glm::vec4(minB.x, minB.y, minB.z, 1.0f));
			transformedMax = Vector3(mat * glm::vec4(maxB.x, maxB.y, maxB.z, 1.0f));
			transformedMin.x = -transformedMin.x;
			transformedMax.x = -transformedMax.x;
		}
		else
		{
			Vector3 p = go->GetTransform()->GetPosition();
			Vector3 s = go->GetTransform()->GetLocalScale() * 0.5f;
			if (s.x < 0.5f) s = Vector3(0.5f);
			transformedMin = p - s;
			transformedMax = p + s;
		}

		float dis = 999999.0f;
		if (GetAABBDistance(transformedMin, transformedMax, dirfrac, camPos, &dis))
		{
			if (dis > 0.0f && dis < minDis)
			{
				minDis = dis;
				hitIndex = i;
				hitGo = go;
			}
		}
	}

	if (hitGo)
	{
		Editor::SetSelectedGameObject(hitGo);
	}
	return hitIndex;
}

void XenityFFI::TransformSelected(float delta_x, float delta_y, int tool_mode, int axis)
{
	if (Editor::GetSelectedGameObjects().empty() || !s_camera)
		return;

	auto go = Editor::GetSelectedGameObjects()[0].lock();
	if (!go)
		return;

	auto trans = go->GetTransform();
	if (!trans)
		return;

	if (tool_mode == 1) // Tool_Move
	{
		Vector3 camRight = -s_camera->GetTransform()->GetLeft();
		Vector3 camUp = s_camera->GetTransform()->GetUp();
		float camDist = Vector3::Distance(trans->GetPosition(), s_camera->GetTransform()->GetPosition());
		float factor = std::max(0.001f, camDist * 0.0018f);

		Vector3 move(0.0f);
		if (axis == 1) // X axis
			move = Vector3(1.0f, 0.0f, 0.0f) * (delta_x * factor);
		else if (axis == 2) // Y axis
			move = Vector3(0.0f, 1.0f, 0.0f) * (-delta_y * factor);
		else if (axis == 3) // Z axis
			move = Vector3(0.0f, 0.0f, 1.0f) * (delta_x * factor);
		else // View aligned
			move = (camRight * delta_x - camUp * delta_y) * factor;

		trans->SetPosition(trans->GetPosition() + move);
	}
	else if (tool_mode == 2) // Tool_Rotate
	{
		Vector3 rot = trans->GetEulerAngles();
		if (axis == 1)
			rot.x += delta_y * 0.5f;
		else if (axis == 2)
			rot.y += delta_x * 0.5f;
		else if (axis == 3)
			rot.z += delta_x * 0.5f;
		else
		{
			rot.y += delta_x * 0.5f;
			rot.x += delta_y * 0.5f;
		}
		trans->SetEulerAngles(rot);
	}
	else if (tool_mode == 3) // Tool_Scale
	{
		Vector3 scale = trans->GetLocalScale();
		float factor = (delta_x - delta_y) * 0.015f;
		if (axis == 1)
			scale.x = std::max(0.05f, scale.x + delta_x * 0.02f);
		else if (axis == 2)
			scale.y = std::max(0.05f, scale.y - delta_y * 0.02f);
		else if (axis == 3)
			scale.z = std::max(0.05f, scale.z + delta_x * 0.02f);
		else
		{
			scale.x = std::max(0.05f, scale.x + factor);
			scale.y = std::max(0.05f, scale.y + factor);
			scale.z = std::max(0.05f, scale.z + factor);
		}
		trans->SetLocalScale(scale);
	}
}

// -------------------------------------------------------------
// Hierarchy & Properties Sync Implementation
// -------------------------------------------------------------

int XenityFFI::GetGameObjectInfo(int index, XenityGameObjectInfo* out_info)
{
	if (index < 0 || index >= GameplayManager::gameObjectCount || !out_info)
		return 0;

	auto go = GameplayManager::GetGameObjects()[index];
	if (!go)
		return 0;

	out_info->index = index;
	std::string name = go->GetName();
	strncpy(out_info->name, name.c_str(), sizeof(out_info->name) - 1);
	out_info->name[sizeof(out_info->name) - 1] = '\0';

	Vector3 pos = go->GetTransform()->GetPosition();
	Vector3 rot = go->GetTransform()->GetEulerAngles();
	Vector3 scale = go->GetTransform()->GetLocalScale();

	out_info->pos[0] = pos.x;
	out_info->pos[1] = pos.y;
	out_info->pos[2] = pos.z;

	out_info->rot[0] = rot.x;
	out_info->rot[1] = rot.y;
	out_info->rot[2] = rot.z;

	out_info->scale[0] = scale.x;
	out_info->scale[1] = scale.y;
	out_info->scale[2] = scale.z;

	out_info->has_mesh = go->GetComponent<MeshRenderer>() ? 1 : 0;
	out_info->has_collider = (go->GetComponent<BoxCollider>() || go->GetComponent<SphereCollider>()) ? 1 : 0;
	out_info->has_light = go->GetComponent<Light>() ? 1 : 0;
	out_info->is_active = go->IsActive() ? 1 : 0;

	out_info->color[0] = 0.64f;
	out_info->color[1] = 0.63f;
	out_info->color[2] = 0.65f;
	out_info->color[3] = 1.0f;

	if (auto mr = go->GetComponent<MeshRenderer>())
	{
		if (auto mat = mr->GetMaterial(0))
		{
			const RGBA& c = mat->GetColor().GetRGBA();
			out_info->color[0] = c.r;
			out_info->color[1] = c.g;
			out_info->color[2] = c.b;
			out_info->color[3] = c.a;
		}
	}
	return 1;
}

void XenityFFI::SetGameObjectTransform(int index, float px, float py, float pz, float rx, float ry, float rz, float sx, float sy, float sz)
{
	if (index >= 0 && index < GameplayManager::gameObjectCount)
	{
		auto go = GameplayManager::GetGameObjects()[index];
		if (go)
		{
			go->GetTransform()->SetPosition(Vector3(px, py, pz));
			go->GetTransform()->SetEulerAngles(Vector3(rx, ry, rz));
			go->GetTransform()->SetLocalScale(Vector3(sx, sy, sz));
		}
	}
}

void XenityFFI::SetGameObjectColor(int index, float r, float g, float b, float a)
{
	if (index >= 0 && index < GameplayManager::gameObjectCount)
	{
		auto go = GameplayManager::GetGameObjects()[index];
		if (go)
		{
			if (auto mr = go->GetComponent<MeshRenderer>())
			{
				auto mat = mr->GetMaterial(0);
				if (!mat || mat == AssetManager::standardMaterial)
				{
					auto newMat = Material::MakeMaterial();
					if (AssetManager::standardShader)
					{
						newMat->SetShader(AssetManager::standardShader);
					}
					else if (AssetManager::standardMaterial)
					{
						newMat->m_shader = AssetManager::standardMaterial->m_shader;
					}
					auto whiteTex = LoadTextureDirect("public_engine_assets/textures/default_texture.png");
					if (whiteTex)
					{
						newMat->SetTexture(whiteTex);
					}
					else if (AssetManager::defaultTexture)
					{
						newMat->SetTexture(AssetManager::defaultTexture);
					}
					newMat->SetUseLighting(true);
					newMat->SetColor(Color::CreateFromRGBAFloat(r, g, b, a));
					mr->SetMaterial(newMat, 0);
				}
				else
				{
					if (!mat->GetTexture())
					{
						auto whiteTex = LoadTextureDirect("public_engine_assets/textures/default_texture.png");
						if (whiteTex) mat->SetTexture(whiteTex);
						else if (AssetManager::defaultTexture) mat->SetTexture(AssetManager::defaultTexture);
					}
					mat->SetUseLighting(true);
					mat->SetColor(Color::CreateFromRGBAFloat(r, g, b, a));
				}
			}
		}
	}
}

void XenityFFI::SetGameObjectName(int index, const char* name)
{
	if (index >= 0 && index < GameplayManager::gameObjectCount && name)
	{
		auto go = GameplayManager::GetGameObjects()[index];
		if (go)
		{
			go->SetName(name);
		}
	}
}

void XenityFFI::DeleteGameObject(int index)
{
	if (index >= 0 && index < GameplayManager::gameObjectCount)
	{
		auto go = GameplayManager::GetGameObjects()[index];
		if (go && go->GetName() != "Baseplate" && go->GetName() != "Camera")
		{
			Destroy(go);
		}
	}
}

int XenityFFI::CookProject(int platform_id, const char* export_path)
{
	if (!export_path)
		return -1;

	std::string out_path = export_path;
	if (!out_path.empty() && out_path.back() != '/' && out_path.back() != '\\')
	{
		out_path += "/";
	}

	try
	{
		std::filesystem::create_directories(out_path);
	}
	catch (...)
	{
	}

	CookSettings settings;
	switch (platform_id)
	{
	case 1:
		settings.platform = Platform::P_PSP;
		settings.assetPlatform = AssetPlatform::AP_PSP;
		break;
	case 2:
		settings.platform = Platform::P_PsVita;
		settings.assetPlatform = AssetPlatform::AP_PsVita;
		break;
	case 3:
		settings.platform = Platform::P_PS3;
		settings.assetPlatform = AssetPlatform::AP_PS3;
		break;
	case 0:
	default:
		settings.platform = Platform::P_Windows;
		settings.assetPlatform = AssetPlatform::AP_Standalone;
		break;
	}
	settings.exportPath = out_path;
	settings.exportShadersOnly = false;

	Cooker::PrepareCooking();
	bool ok = Cooker::CookAssets(settings);
	Cooker::ClearPreparedCooking();

	return ok ? 0 : 1;
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

XENITY_API void xenity_set_tool_mode(int mode)
{
	XenityFFI::SetToolMode(mode);
}

XENITY_API void xenity_select_gameobject(int index)
{
	XenityFFI::SelectGameObject(index);
}

XENITY_API int xenity_get_selected_gameobject()
{
	return XenityFFI::GetSelectedGameObject();
}

XENITY_API int xenity_pick_gameobject(float norm_x, float norm_y)
{
	return XenityFFI::PickGameObject(norm_x, norm_y);
}

XENITY_API void xenity_transform_selected(float delta_x, float delta_y, int tool_mode, int axis)
{
	XenityFFI::TransformSelected(delta_x, delta_y, tool_mode, axis);
}

XENITY_API int xenity_get_gameobject_info(int index, XenityGameObjectInfo* out_info)
{
	return XenityFFI::GetGameObjectInfo(index, out_info);
}

XENITY_API void xenity_set_gameobject_transform(int index, float px, float py, float pz, float rx, float ry, float rz, float sx, float sy, float sz)
{
	XenityFFI::SetGameObjectTransform(index, px, py, pz, rx, ry, rz, sx, sy, sz);
}

XENITY_API void xenity_set_gameobject_color(int index, float r, float g, float b, float a)
{
	XenityFFI::SetGameObjectColor(index, r, g, b, a);
}

XENITY_API void xenity_set_gameobject_name(int index, const char* name)
{
	XenityFFI::SetGameObjectName(index, name);
}

XENITY_API void xenity_delete_gameobject(int index)
{
	XenityFFI::DeleteGameObject(index);
}

XENITY_API int xenity_cook_project(int platform_id, const char* export_path)
{
	return XenityFFI::CookProject(platform_id, export_path);
}

}
