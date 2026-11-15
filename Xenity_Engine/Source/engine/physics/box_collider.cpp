// SPDX-License-Identifier: MIT
//
// Copyright (c) 2022-2026 Gregory Machefer (Fewnity)
//
// This file is part of Xenity Engine

#include "box_collider.h"

#include <iostream>

#include <bullet/btBulletDynamicsCommon.h>

#if defined(EDITOR)
#include <editor/rendering/gizmo.h>
#endif

#include <engine/engine.h>
#include <engine/asset_management/asset_manager.h>
#include <engine/graphics/renderer/renderer.h>
#include <engine/graphics/graphics.h>
#include <engine/game_elements/transform.h>
#include <engine/game_elements/gameobject.h>
#include <engine/graphics/3d_graphics/mesh_renderer.h>
#include <engine/debug/stack_debug_object.h>
#include "rigidbody.h"
#include "physics_manager.h"

BoxCollider::BoxCollider()
{
	CalculateBoundingBox();
}

BoxCollider::~BoxCollider()
{
	GetTransform()->GetOnTransformScaled().Unbind(&BoxCollider::OnTransformScaled, this);
	GetTransform()->GetOnTransformUpdated().Unbind(&BoxCollider::OnTransformUpdated, this);
}

ReflectiveData BoxCollider::GetReflectiveData()
{
	ReflectiveData reflectedVariables;
	AddVariable(reflectedVariables, m_size, "size");
	AddVariable(reflectedVariables, m_offset, "offset");
	AddVariable(reflectedVariables, m_isTrigger, "isTrigger");
	AddVariable(reflectedVariables, m_generateCollisionEvents, "generateCollisionEvents");
	return reflectedVariables;
}

void BoxCollider::OnReflectionUpdated()
{
	STACK_DEBUG_OBJECT(STACK_MEDIUM_PRIORITY);

	CalculateBoundingBox();
	if (const std::shared_ptr<RigidBody> rb = m_attachedRigidbody.lock())
	{
		rb->UpdateGeneratesEvents();
	}
	OnTransformScaled();
	OnTransformUpdated();
}

void BoxCollider::OnTransformScaled()
{
	if (m_bulletCollisionShape)
	{
		const Vector3& scale = GetTransform()->GetScale();
		m_bulletCollisionShape->setLocalScaling(btVector3(m_size.x / 2.0f * scale.x, m_size.y / 2.0f * scale.y, m_size.z / 2.0f * scale.z));

		if (const std::shared_ptr<RigidBody> rb = m_attachedRigidbody.lock())
		{
			rb->RemoveShape(m_bulletCollisionShape);
			rb->RemoveTriggerShape(m_bulletCollisionShape);
			if (!m_isTrigger)
				rb->AddShape(m_bulletCollisionShape, m_offset * scale);
			else
				rb->AddTriggerShape(m_bulletCollisionShape, m_offset * scale);
			rb->Activate();
		}
	}
}

void BoxCollider::OnTransformUpdated()
{
	if (m_bulletCollisionObject)
	{
		const Transform& transform = *GetTransform();

		const glm::mat4x4& matrix = transform.GetTransformationMatrix();
		const Vector3 newPos = Vector3(matrix * glm::vec4(-m_offset.x, m_offset.y, m_offset.z, 1));

		m_bulletCollisionObject->setWorldTransform(btTransform(
			btQuaternion(transform.GetRotation().x, transform.GetRotation().y, transform.GetRotation().z, transform.GetRotation().w),
			btVector3(-newPos.x, newPos.y, newPos.z)));
	}
}

void BoxCollider::Awake()
{
	GetTransform()->GetOnTransformScaled().Bind(&BoxCollider::OnTransformScaled, this);
	GetTransform()->GetOnTransformUpdated().Bind(&BoxCollider::OnTransformUpdated, this);
	FindRigidbody();
}

void BoxCollider::Start()
{
	CreateCollision(false);
}

void BoxCollider::CreateCollision(bool forceCreation)
{
	if (!forceCreation && (m_bulletCollisionShape || m_bulletCollisionObject))
		return;

	if (m_bulletCollisionObject)
	{
		PhysicsManager::s_physDynamicsWorld->removeCollisionObject(m_bulletCollisionObject);

		delete m_bulletCollisionObject;
		m_bulletCollisionObject = nullptr;
	}

	const Vector3& scale = GetTransform()->GetScale();

	if (!m_bulletCollisionShape) 
	{
		m_bulletCollisionShape = new btBoxShape(btVector3(1, 1, 1));
	}

	m_bulletCollisionShape->setLocalScaling(btVector3(m_size.x / 2.0f * scale.x, m_size.y / 2.0f * scale.y, m_size.z / 2.0f * scale.z));
	m_bulletCollisionShape->setUserPointer(this);

	if (const std::shared_ptr<RigidBody> rb = m_attachedRigidbody.lock())
	{
		if (!m_isTrigger)
			rb->AddShape(m_bulletCollisionShape, m_offset * scale);
		else
			rb->AddTriggerShape(m_bulletCollisionShape, m_offset * scale);
	}
	else
	{
		const glm::mat4x4& matrix = GetTransform()->GetTransformationMatrix();
		const Vector3 newPos = Vector3(matrix * glm::vec4(-m_offset.x, m_offset.y, m_offset.z, 1));

		const Quaternion& rot = GetTransform()->GetRotation();

		btTransform startTransform;
		startTransform.setIdentity();
		startTransform.setOrigin(btVector3(-newPos.x, newPos.y, newPos.z));
		startTransform.setRotation(btQuaternion(rot.x, rot.y, rot.z, rot.w));

		m_bulletCollisionObject = new btCollisionObject();
		m_bulletCollisionObject->setCollisionShape(m_bulletCollisionShape);
		m_bulletCollisionObject->setWorldTransform(startTransform);
		m_bulletCollisionObject->setUserPointer(this);
		m_bulletCollisionObject->setRestitution(1);
		m_bulletCollisionObject->setActivationState(DISABLE_SIMULATION);

		if (m_isTrigger)
		{
			m_bulletCollisionObject->setCollisionFlags(m_bulletCollisionObject->getCollisionFlags() | btCollisionObject::CF_NO_CONTACT_RESPONSE);
		}

		PhysicsManager::s_physDynamicsWorld->addCollisionObject(m_bulletCollisionObject, btBroadphaseProxy::StaticFilter, btBroadphaseProxy::AllFilter ^ btBroadphaseProxy::StaticFilter);
	}
}

void BoxCollider::OnDrawGizmosSelected()
{
#if defined(EDITOR)
	// Comportamiento auténtico de Roblox Studio: no mostrar líneas verdes de colisión en selección
	return;
#endif
}

void BoxCollider::SetDefaultSize()
{
	const std::shared_ptr<MeshRenderer> mesh = GetGameObject()->GetComponent<MeshRenderer>();
	if (mesh && mesh->GetMeshData())
	{
		const std::shared_ptr<MeshData>& meshData = mesh->GetMeshData();
		m_size = meshData->GetMaxBoundingBox() - meshData->GetMinBoundingBox();
		m_offset = (meshData->GetMaxBoundingBox() + meshData->GetMinBoundingBox()) / 2.0f;
		m_offset.x = -m_offset.x;
		CalculateBoundingBox();
	}
}

void BoxCollider::CalculateBoundingBox()
{
	m_min = -m_size / 2.0f + m_offset;
	m_max = m_size / 2.0f + m_offset;
}

void BoxCollider::SetSize(const Vector3& size)
{
	m_size = size;
	CalculateBoundingBox();
}


void BoxCollider::SetOffset(const Vector3& offset)
{
	m_offset = offset;
}

std::string BoxCollider::ToString()
{
	return "Size: " + m_size.ToString() + ", Offset:" + m_offset.ToString();
}
