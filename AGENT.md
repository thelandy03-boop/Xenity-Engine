# Xenity Engine - AI Assistant & Developer Guidelines

> This file is automatically read by AI coding agents (Antigravity, Claude Code, Cursor, Windsurf, Copilot, etc.).
> It establishes mandatory rules, architecture context, and documentation protocols for the Xenity Engine codebase.

---

## 🧠 1. Automatic Knowledge Base Protocol (MANDATORY)

Whenever you (the AI) are asked to research, design, optimize, or propose technical architecture for Xenity Engine:
1. **Always consult `knowledge_base/` first**:
   - `knowledge_base/graphics/`: Existing shaders, rendering pipelines, CryEngine adaptation plan.
   - `knowledge_base/platforms/`: Console hardware specs (PS3 256MB VRAM, Vita, PSP, 360) and memory budgets.
   - `knowledge_base/architecture/`: Engine core systems and design.
2. **Always document new technical proposals in `knowledge_base/`**:
   - Do NOT just write long transient text in the chat.
   - Use the template located at [`knowledge_base/templates/ai_document_template.md`](knowledge_base/templates/ai_document_template.md).
   - Place the new document in the corresponding folder (`graphics/`, `platforms/`, `architecture/`, `audio/`, `physics/`).
   - Automatically update the index in [`knowledge_base/README.md`](knowledge_base/README.md).

---

## 🛠️ 2. Project Architecture & Standards

* **Engine Core & Standalone Game**: C++17 (`Xenity_Engine/`)
  - Build system: CMake (`CMakeLists.txt`)
  - Target platforms: **Linux** (native ELF/CMake), **Windows** (MSVC/CMake), **PS3** (PSL1GHT/GCM), **PS Vita** (vitaGL/GXM), **PSP** (Sony GU).
  - Windowing & Input: SDL3 native.
  - Graphics API: Desktop OpenGL 3.3 Core (PC), OpenGL ES 2.0 (Vita), Sony GU (PSP).
* **Editor GUI**: Rust 2024 + `eframe` (egui) + `wgpu` (`xenity-editor/`)
  - Communicates with C++ engine via FFI (`extern "C"` exports in `xenity_ffi.h` / `libXenity_Editor.so`).
  - Backend on Linux: **Vulkan** (`WGPU_BACKEND="vulkan"` to avoid EGL/GLX context collisions with SDL3).
* **Asset Pipeline**:
  - Offline cooker compresses scenes, shaders, and meshes into `.xenb` binary archives.

---

## 🚫 3. Golden Rules & Constraints

1. **Strict Memory Budgets**: Remember that target consoles have severe RAM constraints:
   - **PSP**: 32/64 MB total RAM, no programmable shaders (fixed-function only).
   - **PS3**: 256 MB system RAM + 256 MB VRAM. Never allocate giant textures or deep deferred G-buffers.
   - **PS Vita**: 512 MB RAM + 128 MB VRAM.
2. **Multiplatform Code Integrity**:
   - Always wrap platform-specific APIs in preprocessor guards (`#if defined(__LINUX__)`, `#if defined(_WIN32)`, etc.).
   - Never use Windows-specific APIs (`<windows.h>`, `vcvars32.bat`, `.dll`) in generic engine code; use POSIX / standard C++ equivalents for Linux.
3. **Keep Workspaces Clean**:
   - Do not commit build artifacts (`build/`, `target/`, `.so`, temporary binaries).
