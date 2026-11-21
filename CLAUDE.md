# Claude Code & Anthropic AI Instructions - Xenity Engine

See [AGENT.md](AGENT.md) for detailed guidelines.

- **Mandatory Knowledge Base**: When designing features, rendering pipelines, or system proposals, always store and expand documents in `knowledge_base/` following `knowledge_base/templates/ai_document_template.md`.
- **Build Commands**:
  - Engine & C++ Editor Library: `cmake --build build -j$(nproc)`
  - Rust Editor GUI: `cargo run --release --manifest-path xenity-editor/Cargo.toml`
- **Architectural Rules**:
  - C++17 Core (`Xenity_Engine/`) + Rust 2024 UI (`xenity-editor/`).
  - Respect memory budgets: 256MB VRAM on PS3, 32MB total on PSP, 128MB VRAM on Vita.
