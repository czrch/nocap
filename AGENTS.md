# Agent Guidelines: nocap

Minimal, cross-platform image viewer. Keep it small, fast, and clean.

## Stack (Current)

- Tauri v2 (Rust)
- Svelte 5 + TypeScript + Vite

## Core Goals

- Minimalism first (avoid feature creep).
- Cross-platform correctness (Linux/macOS/Windows).
- Rust does filesystem/system work; Svelte does UI/state.
- Small, reviewable changes.

## Commit-Oriented Development

- Plan features as **a sequence of small commits** (vertical slices when possible).
- Each commit should be **buildable** and **validation-clean**.
- Don’t mix unrelated refactors/formatting with feature work.

### Commit messages

- Must use lowercase prefixes: `feat:`, `fix:`, `docs:`, `chore:`, `refactor:`, `test:`, `perf:`, `ci:`, `build:`, `style:`, `revert:`
- Format: `<type>: <description>` (≤72 chars, lowercase, imperative)

## Required Validation (Before Every Commit)

Frontend:
```bash
npm run format:check
npm run lint
npm run check
```

Backend:
```bash
cd src-tauri
cargo fmt --check
cargo clippy --all-targets --all-features
```

## Project Rules (Practical)

Tauri:

- Register all `#[tauri::command]` functions in `src-tauri/src/lib.rs`.
- If you add permissions/plugins, update `src-tauri/capabilities/default.json`.
- IPC types must derive `Serialize`/`Deserialize`.

Frontend:

- No `any`; prefer explicit types for exported functions and stores.
- Stores live in `src/lib/stores/` and should guard against stale async responses.
- Keep components small; props down, events up.

Testing (Current Phase)

- Manual testing only; keep changes easy to verify.

**Last Updated**: 2025-12-18
