# Agent Guidelines: nocap

## Project Overview

**Name**: nocap  
**Type**: Cross-platform desktop image viewer  
**Stack**: Tauri v2.9 (Rust) + Svelte 4 + TypeScript  
**License**: GPL-3.0  
**Status**: Initial development phase  

**Purpose**: Minimal, sleek image viewer with essential features only. No bloat, clean code, modern architecture.

## Philosophy

### Design Principles
1. **Minimalism First**: Only implement what users actually need. Resist feature creep.
2. **Performance**: Leverage Rust's speed. Keep frontend lightweight (Svelte chosen for this).
3. **Cross-Platform**: Code must work on Linux, macOS, Windows without platform-specific hacks.
4. **Clean Architecture**: Separation of concerns - Rust handles system/files, Svelte handles UI.
5. **User Experience**: Fast startup, smooth interactions, intuitive controls, no configuration complexity.

### Development Values
- **Incremental Progress**: Small, atomic commits over large changesets
- **Code Quality**: Readable > clever. Self-documenting code preferred.
- **Modern Practices**: Use latest stable tools, follow ecosystem conventions
- **Documentation**: Code explains "how", comments explain "why", docs explain "what"

## Project Structure

```
nocap/
├── docs/              # Technical documentation, plans
├── src/               # Frontend (Svelte + TypeScript)
│   ├── lib/
│   │   ├── components/  # Svelte UI components
│   │   ├── stores/      # State management
│   │   └── types.ts     # TypeScript interfaces
│   ├── App.svelte       # Root component
│   ├── main.ts          # Entry point
│   └── app.css          # Global styles
├── src-tauri/         # Backend (Rust)
│   ├── src/
│   │   ├── main.rs      # Tauri entry, window setup
│   │   ├── commands.rs  # IPC command handlers
│   │   ├── models.rs    # Data structures
│   │   └── utils.rs     # Helper functions
│   ├── capabilities/    # Tauri permissions
│   ├── Cargo.toml       # Rust deps
│   └── tauri.conf.json  # App config
├── public/            # Static assets
├── AGENTS.md          # This file
├── README.md          # User-facing docs
└── LICENSE            # GPL-3.0
```

## Git Conventions

### Branch Naming
- `feature/{name}` - New features (e.g., `feature/thumbnail-view`)
- `fix/{name}` - Bug fixes (e.g., `fix/zoom-reset`)
- `docs/{name}` - Documentation (e.g., `docs/api-reference`)
- `chore/{name}` - Maintenance (e.g., `chore/update-deps`)
- `refactor/{name}` - Code improvements (e.g., `refactor/image-loading`)
- `test/{name}` - Tests (e.g., `test/file-scanner`)
- `perf/{name}` - Performance (e.g., `perf/caching`)
- `hotfix/{name}` - Urgent fixes (e.g., `hotfix/crash-on-startup`)

Use kebab-case. Keep names concise, descriptive.

### Commit Messages

**CRITICAL: All commit messages MUST use lowercase prefixes**

Format: `<type>: <description>`

**Valid types** (lowercase only):
- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `chore:` - Maintenance, deps, config
- `refactor:` - Code restructuring (no behavior change)
- `test:` - Add/update tests
- `perf:` - Performance improvements
- `ci:` - CI/CD changes
- `build:` - Build system changes
- `style:` - Formatting, whitespace
- `revert:` - Revert previous commit

**Examples**:
```
feat: add zoom controls to toolbar
fix: handle empty image directories gracefully
docs: update readme with build instructions
chore: bump tauri to v2.9.1
refactor: extract metadata parsing to utils
```

**Rules**:
- Subject line: lowercase, imperative mood, ≤72 chars
- Scope optional: `feat(ui): add navigation arrows`
- One logical change per commit
- Don't mix refactors with features
- Avoid "drive-by" formatting in unrelated files

## Code Standards

### Rust (Backend)
- **Style**: `cargo fmt` (rustfmt) before every commit
- **Linting**: `cargo clippy` must pass with zero warnings
- **Error Handling**: Use `Result<T, String>` for commands, descriptive error messages
- **Naming**: snake_case for functions/variables, PascalCase for types
- **Documentation**: Doc comments (`///`) for public items
- **Dependencies**: Minimize crates, prefer std library when reasonable

### TypeScript/Svelte (Frontend)
- **Style**: Consistent with project (2-space indent, single quotes where applicable)
- **Types**: No `any`, explicit types for function signatures
- **Naming**: camelCase for functions/variables, PascalCase for components/interfaces
- **Components**: One component per file, named exports
- **Stores**: Centralized in `lib/stores/`, immutable updates
- **Documentation**: JSDoc for complex functions

## Implementation Workflow

### Before Starting Work
1. Read `docs/implementation_plan.md` for current phase
2. Understand the specific step you're implementing
3. Check existing code to maintain consistency
4. Verify you have correct context (file paths, dependencies)

### During Implementation
1. **One step at a time**: Follow implementation plan sequentially
2. **Create files carefully**: Use exact paths from plan
3. **Test incrementally**: Verify after each logical unit
4. **Commit atomically**: One feature/fix per commit
5. **Write clear commits**: Follow git conventions exactly

### After Implementation
1. **Verify**: Run verification command from plan step
2. **Test manually**: Check functionality works as expected
3. **Check style**: Run formatters (`cargo fmt`, etc.)
4. **Review changes**: Ensure no unintended modifications
5. **Commit**: Use proper conventional commit message

## Agent-Specific Rules

### File Operations
- **Read before modify**: Always read current file state before edits
- **Use SEARCH/REPLACE**: For targeted changes, use exact matching
- **Verify paths**: Double-check file paths against project structure
- **Create directories**: Ensure parent dirs exist before file creation

### Tauri-Specific
- **Commands**: All backend functions called from frontend must be registered in `main.rs`
- **Permissions**: Update `capabilities/default.json` when adding plugin features
- **Serialization**: Use `#[derive(Serialize, Deserialize)]` for data crossing IPC boundary
- **Asset Protocol**: Use `asset://` protocol for local file access in frontend

### Svelte-Specific
- **Reactivity**: Use `$:` for derived values, stores for global state
- **Components**: Props down, events up (no prop drilling)
- **Lifecycle**: `onMount` for initialization, cleanup in `onDestroy`
- **Stores**: Subscribe with `$storeName` syntax, avoid manual subscriptions

### Common Pitfalls
- ❌ Forgetting to register Tauri commands in `main.rs`
- ❌ Using uppercase in commit messages (must be lowercase)
- ❌ Not handling errors in async operations
- ❌ Hardcoding file paths (use Tauri path APIs)
- ❌ Committing formatted code separately from functional changes
- ❌ Adding features not in the current implementation plan

## Testing Strategy

### Current Phase (MVP)
Manual testing only. Focus on:
- Core functionality works
- No crashes on edge cases
- Cross-platform compatibility
- Performance is acceptable

### Future Phases
- **Unit tests**: Rust utility functions, TypeScript helpers
- **Component tests**: Svelte components with @testing-library/svelte
- **Integration tests**: Tauri commands via test harness
- **E2E tests**: Full application flows

## Communication

### When to Ask Questions
- Requirements are ambiguous
- Implementation approach unclear
- Conflicts with existing code
- Breaking changes needed
- Security/performance concerns

### When to Proceed
- Implementation plan is clear
- Step matches current phase
- No conflicts with existing code
- Standard patterns apply

## Quick Reference

### Build Commands
```bash
npm install          # Install frontend deps
npm run dev          # Dev server (Vite only)
npm run tauri dev    # Full app in dev mode
npm run tauri build  # Production build
cargo fmt            # Format Rust code
cargo clippy         # Lint Rust code
```

### Platform-Specific Notes

#### Linux (Wayland)
When running on Wayland, ensure the following environment variables are set:
```bash
# For native Wayland support
export GDK_BACKEND=wayland
export WAYLAND_DISPLAY=wayland-1

# Run the app
npm run tauri dev
```

If you encounter display protocol errors, verify your Wayland compositor is running and `$WAYLAND_DISPLAY` is properly set.

### Key Files
- `docs/implementation_plan.md` - Full implementation guide
- `src-tauri/tauri.conf.json` - App configuration
- `src-tauri/capabilities/default.json` - Permissions
- `src/lib/stores/viewer.ts` - Application state
- `src/lib/types.ts` - TypeScript interfaces

### Important Concepts
- **Tauri Commands**: Rust functions exposed to frontend via `#[tauri::command]`
- **IPC**: Communication between Rust and Svelte via `invoke()`
- **Stores**: Svelte's reactive state management (writable, readable, derived)
- **Capabilities**: Tauri v2's permission system (required for fs, dialog, etc.)

---

**Last Updated**: 2025-12-16  
**Plan Version**: Initial MVP  
**Target Tauri**: v2.9+