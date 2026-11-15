// SPDX-License-Identifier: MIT
//
// Copyright (c) 2022-2026 Gregory Machefer (Fewnity)
//
// This file is part of Xenity Engine

// Header to include and use API macro when you want to use a class from the DLL
#pragma warning( disable : 4251 )
#if defined(EXPORT)
	#if defined(_WIN32) || defined(_WIN64)
		#define API __declspec(dllexport)
	#else
		#define API __attribute__((visibility("default")))
	#endif
#elif defined(IMPORT)
	#if defined(_WIN32) || defined(_WIN64)
		#define API __declspec(dllimport)
	#else
		#define API
	#endif
#else
	#define API
#endif