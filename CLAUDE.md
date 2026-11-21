# Claude Code & Anthropic AI Instructions - Xenity Engine

See [AGENT.md](AGENT.md) for detailed guidelines.

- **Live Status & State**: Always read `knowledge_base/CURRENT_STATUS.md` first to understand the active phase, completed items, and next tasks.
- **Reference Code & Provenance**: Check `knowledge_base/references/01_source_provenance_and_citations.md` for official CryEngine, OpenLara, and Urho3D codebase references and academic papers. All implementations are mathematically derived, not guessed.
- **Mandatory Knowledge Base**: When designing features, rendering pipelines, or system proposals, always store and expand documents in `knowledge_base/` following `knowledge_base/templates/ai_document_template.md` and update `CURRENT_STATUS.md`.
- **Build & Run Commands**:
  - Engine & C++ Editor Library: `cmake --build build -j$(nproc)`
  - Rust Editor GUI: ALWAYS run with `cargo run --release --manifest-path xenity-editor/Cargo.toml` (never run debug profile due to pixel conversion overhead).
- **Architectural Rules**:
  - C++17 Core (`Xenity_Engine/`) + Rust 2024 UI (`xenity-editor/`).
  - Respect memory budgets: 256MB VRAM on PS3, 32MB total on PSP, 128MB VRAM on Vita.
