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

static bool RayLocalAABB(const glm::vec3& rayOrigin, const glm::vec3& rayDir, const glm::vec3& minB, const glm::vec3& maxB, float& outDist)
{
	float tmin = -1e9f;
	float tmax = 1e9f;

	for (int i = 0; i < 3; ++i)
	{
		if (std::abs(rayDir[i]) < 1e-6f)
		{
			if (rayOrigin[i] < minB[i] || rayOrigin[i] > maxB[i])
				return false;
		}
		else
		{
			float invD = 1.0f / rayDir[i];
			float t0 = (minB[i] - rayOrigin[i]) * invD;
			float t1 = (maxB[i] - rayOrigin[i]) * invD;
			if (t0 > t1) std::swap(t0, t1);
			tmin = std::max(tmin, t0);
			tmax = std::min(tmax, t1);
			if (tmin > tmax) return false;
		}
	}

	if (tmax < 0.0f) return false;
	outDist = (tmin >= 0.0f) ? tmin : tmax;
	return true;
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
		s_camera->GetTransform()->SetPosition(Vector3(0.0f, 22.0f, -65.0f));
		s_camera->GetTransform()->SetEulerAngles(Vector3(18.0f, 0.0f, 0.0f));
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

	// 6b. Enable CryEngine-style subtle atmospheric height fog
	Graphics::s_settings.isFogEnabled = true;
	Graphics::s_settings.fogColor = Color::CreateFromRGBAFloat(0.65f, 0.75f, 0.88f, 1.0f);

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
					go->GetTransform()->SetPosition(Vector3(0.0f, -2.0f, 0.0f));
					go->GetTransform()->SetLocalScale(Vector3(80.0f, 4.0f, 80.0f));
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
			bp->GetTransform()->SetPosition(Vector3(0.0f, -2.0f, 0.0f));
			bp->GetTransform()->SetLocalScale(Vector3(80.0f, 4.0f, 80.0f));

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

	// 8. Setup CryEngine Tropical Ocean demo body
	if (!AssetManager::waterShader)
	{
		AssetManager::waterShader = AssetManager::LoadEngineAsset<Shader>("public_engine_assets/shaders/water.shader");
	}
	if (AssetManager::waterShader && AssetManager::waterShader->GetFileStatus() != FileStatus::FileStatus_Loaded)
	{
		FileReference::LoadOptions syncLoadOptions;
		syncLoadOptions.platform = Application::GetPlatform();
		syncLoadOptions.threaded = false;
		AssetManager::waterShader->LoadFileReference(syncLoadOptions);
	}

	std::shared_ptr<GameObject> oceanGO = nullptr;
	const int currentGoCount = GameplayManager::gameObjectCount;
	for (int i = 0; i < currentGoCount; i++)
	{
		const auto& go = GameplayManager::GetGameObjects()[i];
		if (go && go->GetName() == "TropicalOcean")
		{
			oceanGO = go;
			break;
		}
	}
	if (!oceanGO)
	{
		oceanGO = ShapeSpawner::SpawnPlane();
		if (oceanGO)
		{
			oceanGO->SetName("TropicalOcean");
		}
	}
	if (oceanGO)
	{
		oceanGO->SetName("TropicalOcean");
		oceanGO->GetTransform()->SetPosition(Vector3(0.0f, -0.6f, 0.0f));
		oceanGO->GetTransform()->SetLocalScale(Vector3(3000.0f, 1.0f, 3000.0f));
		auto oceanMesh = oceanGO->GetComponent<MeshRenderer>();
		if (oceanMesh && AssetManager::waterShader)
		{
			auto waterMat = Material::MakeMaterial();
			waterMat->SetShader(AssetManager::waterShader);
			waterMat->SetRenderingMode(MaterialRenderingMode::Transparent);
			auto waterNormTex = LoadTextureDirect("public_engine_assets/textures/water_normal.png");
			if (waterNormTex)
			{
				waterNormTex->SetWrapMode(WrapMode::Repeat);
				waterNormTex->SetFilter(Filter::Bilinear);
				waterNormTex->SetUseMipmap(true);
				waterMat->SetTexture(waterNormTex);
			}
			waterMat->SetColor(Color::CreateFromRGBAFloat(0.04f, 0.72f, 0.78f, 0.85f));
			waterMat->SetTiling(Vector2(1.0f, 1.0f));
			waterMat->SetUseLighting(true);
			oceanMesh->SetMaterial(waterMat, 0);
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

	return 1;
}

static float s_camPitch = 20.0f;
static float s_camYaw = 0.0f;
static bool s_camEulerInit = false;

void XenityFFI::CameraOrbit(float delta_yaw, float delta_pitch, float move_fwd, float move_right, float move_up)
{
	if (!s_camera)
		return;

	auto camTrans = s_camera->GetTransform();
	if (!camTrans)
		return;

	if (!s_camEulerInit)
	{
		Vector3 euler = camTrans->GetEulerAngles();
		s_camPitch = euler.x;
		s_camYaw = euler.y;
		s_camEulerInit = true;
	}

	// Rotation using yaw + pitch around global Y and local X, clamping pitch to prevent gimbal flipping
	if (delta_yaw != 0.0f || delta_pitch != 0.0f)
	{
		s_camYaw += delta_yaw;
		s_camPitch = std::clamp(s_camPitch + delta_pitch, -89.0f, 89.0f);
		Quaternion rotY = Quaternion::AngleAxis(s_camYaw, Vector3(0.0f, 1.0f, 0.0f));
		Quaternion rotX = Quaternion::AngleAxis(s_camPitch, Vector3(1.0f, 0.0f, 0.0f));
		camTrans->SetRotation(rotY * rotX);
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
	Editor::s_toolMode = static_cast<ToolMode>(mode);
	auto sceneMenu = Editor::GetMenu<SceneMenu>(false);
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

	// 1. Unproject mouse coordinate from NDC space directly to world space
	// In egui norm_x in [0, 1] (0 is left, 1 is right)
	// norm_y in [0, 1] (0 is TOP, 1 is BOTTOM)
	// In OpenGL NDC: ndc_x in [-1, 1], ndc_y in [-1, 1] where +1 is TOP, -1 is BOTTOM
	const float ndc_x = norm_x * 2.0f - 1.0f;
	const float ndc_y = 1.0f - norm_y * 2.0f;

	const glm::mat4 invVP = glm::inverse(s_camera->m_viewProjectionMatrix);
	const glm::vec4 pNearNDC(ndc_x, ndc_y, -1.0f, 1.0f);
	const glm::vec4 pFarNDC (ndc_x, ndc_y,  1.0f, 1.0f);

	const glm::vec4 pNearW4 = invVP * pNearNDC;
	const glm::vec4 pFarW4  = invVP * pFarNDC;

	if (pNearW4.w == 0.0f || pFarW4.w == 0.0f)
		return -1;

	const glm::vec3 pNearWorld = glm::vec3(pNearW4) / pNearW4.w;
	const glm::vec3 pFarWorld  = glm::vec3(pFarW4)  / pFarW4.w;

	float minDis = 999999.0f;
	int hitIndex = -1;
	std::shared_ptr<GameObject> hitGo = nullptr;

	const int count = GameplayManager::gameObjectCount;
	for (int i = 0; i < count; i++)
	{
		auto go = GameplayManager::GetGameObjects()[i];
		if (!go || !go->IsActive() || go->GetName() == "EditorCamera" || go->GetName() == "Camera")
			continue;

		glm::vec3 minB(-0.5f);
		glm::vec3 maxB(0.5f);

		auto meshRenderer = go->GetComponent<MeshRenderer>();
		if (meshRenderer && meshRenderer->GetMeshData() && meshRenderer->IsEnabled())
		{
			const Vector3& minV = meshRenderer->GetMeshData()->GetMinBoundingBox();
			const Vector3& maxV = meshRenderer->GetMeshData()->GetMaxBoundingBox();
			minB = glm::vec3(std::min(minV.x, maxV.x), std::min(minV.y, maxV.y), std::min(minV.z, maxV.z));
			maxB = glm::vec3(std::max(minV.x, maxV.x), std::max(minV.y, maxV.y), std::max(minV.z, maxV.z));
		}

		// Transform world ray into GameObject's local coordinate space
		const glm::mat4& modelMat = go->GetTransform()->GetTransformationMatrix();
		const glm::mat4 invModel = glm::inverse(modelMat);

		const glm::vec4 pNearL4 = invModel * glm::vec4(pNearWorld, 1.0f);
		const glm::vec4 pFarL4  = invModel * glm::vec4(pFarWorld, 1.0f);

		if (pNearL4.w == 0.0f || pFarL4.w == 0.0f)
			continue;

		const glm::vec3 localNear = glm::vec3(pNearL4) / pNearL4.w;
		const glm::vec3 localFar  = glm::vec3(pFarL4)  / pFarL4.w;
		const glm::vec3 localDir  = glm::normalize(localFar - localNear);

		float localHitDist = 0.0f;
		if (RayLocalAABB(localNear, localDir, minB, maxB, localHitDist))
		{
			// Calculate hit point in world coordinates
			const glm::vec3 localHitPoint = localNear + localDir * localHitDist;
			const glm::vec3 worldHitPoint = glm::vec3(modelMat * glm::vec4(localHitPoint, 1.0f));
			const float worldDist = glm::distance(pNearWorld, worldHitPoint);

			if (worldDist < minDis)
			{
				minDis = worldDist;
				hitIndex = i;
				hitGo = go;
			}
		}
	}

	if (hitGo)
	{
		Editor::SetSelectedGameObject(hitGo);
	}
	else
	{
		Editor::SetSelectedGameObject(nullptr);
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
		Vector3 camFwd = s_camera->GetTransform()->GetForward();
		Vector3 camRight = s_camera->GetTransform()->GetRight();
		float camDist = Vector3::Distance(trans->GetPosition(), s_camera->GetTransform()->GetPosition());
		float factor = std::max(0.001f, camDist * 0.0018f);

		Vector3 move(0.0f);
		if (axis == 1) // X axis
			move = Vector3(1.0f, 0.0f, 0.0f) * (delta_x * factor);
		else if (axis == 2) // Y axis
			move = Vector3(0.0f, 1.0f, 0.0f) * (-delta_y * factor);
		else if (axis == 3) // Z axis
			move = Vector3(0.0f, 0.0f, 1.0f) * (delta_x * factor);
		else // View aligned - Horizontal Ground plane (XZ plane) like Roblox Studio
		{
			Vector3 groundFwd = Vector3(camFwd.x, 0.0f, camFwd.z).Normalized();
			Vector3 groundRight = Vector3(camRight.x, 0.0f, camRight.z).Normalized();
			move = (groundRight * delta_x - groundFwd * delta_y) * factor;
		}

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
