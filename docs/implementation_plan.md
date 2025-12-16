# Implementation Plan: nocap Image Viewer

## [Overview]
Build minimal cross-platform image viewer with Tauri v2 (Rust) + Svelte + TypeScript. Features: file/folder open, image navigation, zoom/pan, keyboard shortcuts. MVP-focused, clean architecture, incremental git commits.

## [Types]

**src/lib/types.ts**:
```typescript
export interface ImageFile {
  path: string;
  filename: string;
  extension: string;
}

export interface ImageMetadata {
  path: string;
  width: number;
  height: number;
  size: number;
  format: string;
}

export interface ViewerState {
  currentImage: ImageFile | null;
  imageList: ImageFile[];
  currentIndex: number;
  zoomLevel: number;
  fitToWindow: boolean;
}
```

**src-tauri/src/models.rs**:
```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImageFile {
    pub path: String,
    pub filename: String,
    pub extension: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImageMetadata {
    pub path: String,
    pub width: u32,
    pub height: u32,
    pub size: u64,
    pub format: String,
}
```

## [Files]

**Directory Structure**:
```
nocap/
├── docs/implementation_plan.md
├── src/
│   ├── lib/
│   │   ├── components/{ImageViewer,Toolbar,NavigationControls}.svelte
│   │   ├── stores/viewer.ts
│   │   └── types.ts
│   ├── App.svelte
│   ├── main.ts
│   └── app.css
├── src-tauri/
│   ├── src/{main,commands,models,utils}.rs
│   ├── capabilities/default.json
│   ├── Cargo.toml
│   └── tauri.conf.json
├── public/
├── {package.json,vite.config.ts,tsconfig.json}
└── {.gitignore,LICENSE,README.md}
```

**Create**:
- Frontend: `src/{App.svelte,main.ts,app.css}`, `src/lib/types.ts`, `src/lib/stores/viewer.ts`, `src/lib/components/{ImageViewer,Toolbar,NavigationControls}.svelte`, `vite.config.ts`, `tsconfig.json`, `package.json`
- Backend: `src-tauri/src/{main,commands,models,utils}.rs`, `src-tauri/{Cargo.toml,tauri.conf.json}`, `src-tauri/capabilities/default.json`

**Modify**: `README.md`, `.gitignore`

## [Functions]

**src-tauri/src/commands.rs**:
- `open_image_dialog() -> Result<Option<ImageFile>, String>` - Native file picker for images
- `open_folder_dialog() -> Result<Vec<ImageFile>, String>` - Folder picker, returns all images in dir
- `get_adjacent_images(current_path: String) -> Result<Vec<ImageFile>, String>` - All images in same dir as current
- `get_image_metadata(path: String) -> Result<ImageMetadata, String>` - Extract dimensions, size, format

**src-tauri/src/utils.rs**:
- `is_supported_image(path: &Path) -> bool` - Check extension (jpg|jpeg|png|gif|bmp|webp|svg)
- `scan_directory_for_images(dir_path: &Path) -> Vec<ImageFile>` - Scan dir for images
- `extract_image_info(path: &Path) -> Result<ImageMetadata, String>` - Read metadata via `image` crate

**src/lib/stores/viewer.ts**:
- `loadImage(file: ImageFile)` - Update currentImage, currentIndex
- `loadFolder(folderPath: string)` - Call tauri command, update imageList
- `nextImage()` / `previousImage()` - Navigate, update index
- `setZoom(level: number)` - Update zoomLevel
- `toggleFitToWindow()` - Toggle fitToWindow boolean

## [Classes]

**Svelte Components**:

1. **App.svelte**: Root layout, global keyboard handlers (arrows, ±, F), compose children
2. **ImageViewer.svelte**: Display image, CSS zoom/fit-to-window, mouse wheel zoom, drag pan (local state: panX, panY)
3. **Toolbar.svelte**: Buttons (open file/folder, zoom ±/reset/fit), filename display
4. **NavigationControls.svelte**: Prev/next buttons, counter (e.g., "3/15"), disable at boundaries

## [Dependencies]

**package.json**:
```json
{
  "dependencies": {
    "@tauri-apps/api": "^2.9.0",
    "@tauri-apps/plugin-dialog": "^2.9.0",
    "@tauri-apps/plugin-fs": "^2.9.0"
  },
  "devDependencies": {
    "@sveltejs/vite-plugin-svelte": "^4.0.0",
    "@tsconfig/svelte": "^5.0.0",
    "svelte": "^4.2.0",
    "svelte-check": "^4.0.0",
    "typescript": "^5.6.0",
    "vite": "^6.0.0",
    "tslib": "^2.8.0"
  }
}
```

**Cargo.toml**:
```toml
[dependencies]
tauri = { version = "2.9", features = ["macos-private-api"] }
tauri-plugin-dialog = "2.9"
tauri-plugin-fs = "2.9"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
image = "0.25"
walkdir = "2.4"
```

## [Testing]

Manual testing MVP. Automated tests deferred to future.

**Checklist**:
- File ops: open file/folder, supported formats (jpg,png,gif,bmp,webp,svg), handle unsupported/empty
- Display: fit-to-window, aspect ratio, centering, resolution scaling
- Navigation: prev/next, boundary disable, counter, 1-image & 100+ image folders
- Zoom: in/out/reset/fit toggle, mouse wheel
- Keyboard: ←/→ (nav), ±/0 (zoom), F (fit)
- Cross-platform: build/run on Linux/macOS/Windows, native dialogs, path handling

## [Implementation Order]

### Phase 1: Setup
**1. chore: initialize tauri v2 project with svelte and typescript**
```bash
npm create tauri-app@latest . -- -m npm -t svelte-ts --tauri-version 2 -y
npm install
```
Verify: `npm run tauri dev` launches

**2. chore: add fs and dialog plugins with permissions**
- Modify `src-tauri/Cargo.toml`: Add `tauri-plugin-{dialog,fs} = "2.9"`
- Modify `src-tauri/tauri.conf.json`: Add plugins array
- Modify `src-tauri/capabilities/default.json`: Set fs/dialog permissions
Verify: Plugins import without errors

### Phase 2: Backend
**3. feat: add image file and metadata models**
- Create `src-tauri/src/models.rs` with structs
- Modify `src-tauri/src/main.rs`: Add `mod models;`
Verify: Cargo build success

**4. feat: add image file scanning utilities**
- Create `src-tauri/src/utils.rs` with helper functions
- Modify `src-tauri/Cargo.toml`: Add `image = "0.25"`, `walkdir = "2.4"`
- Modify `src-tauri/src/main.rs`: Add `mod utils;`
Verify: Cargo build success

**5. feat: implement tauri commands for file operations**
- Create `src-tauri/src/commands.rs` with all command functions
- Modify `src-tauri/src/main.rs`: Add `mod commands;` + register via `.invoke_handler()`
Verify: Commands callable from frontend

### Phase 3: Frontend Foundation
**6. feat: add typescript types and viewer store**
- Create `src/lib/types.ts`
- Create `src/lib/stores/viewer.ts` with writable store + actions
Verify: Imports work, store initializes

**7. feat: implement base ui layout and styling**
- Modify `src/App.svelte`: Layout structure
- Modify `src/app.css`: Global styles
Verify: Layout renders

### Phase 4: Components
**8. feat: add image viewer component with zoom**
- Create `src/lib/components/ImageViewer.svelte`
- Modify `src/App.svelte`: Import + use ImageViewer
Verify: Component displays image

**9. feat: add toolbar with file operations and zoom controls**
- Create `src/lib/components/Toolbar.svelte`
- Modify `src/App.svelte`: Import + use Toolbar
- Modify `src/lib/stores/viewer.ts`: Add missing actions if needed
Verify: Toolbar buttons functional

**10. feat: add image navigation controls**
- Create `src/lib/components/NavigationControls.svelte`
- Modify `src/App.svelte`: Import + use NavigationControls
Verify: Navigation works

### Phase 5: Polish
**11. feat: add keyboard shortcuts for navigation and zoom**
- Modify `src/App.svelte`: Add `on:keydown` handler
Verify: All shortcuts work

**12. docs: update readme with usage instructions**
- Modify `README.md`: Add description, usage, build instructions
- Modify `src/App.svelte`: UI polish
- Modify `src/app.css`: Final styles
Verify: Manual testing checklist complete

**13. chore: configure build settings for distribution**
- Modify `src-tauri/tauri.conf.json`: Bundle identifier, icons, build config
Verify: `npm run tauri build` succeeds

---

## Navigation Commands

```bash
# Overview
sed -n '/## \[Overview\]/,/## \[Types\]/p' docs/implementation_plan.md | head -n -1

# Types
sed -n '/## \[Types\]/,/## \[Files\]/p' docs/implementation_plan.md | head -n -1

# Files
sed -n '/## \[Files\]/,/## \[Functions\]/p' docs/implementation_plan.md | head -n -1

# Functions
sed -n '/## \[Functions\]/,/## \[Classes\]/p' docs/implementation_plan.md | head -n -1

# Classes
sed -n '/## \[Classes\]/,/## \[Dependencies\]/p' docs/implementation_plan.md | head -n -1

# Dependencies
sed -n '/## \[Dependencies\]/,/## \[Testing\]/p' docs/implementation_plan.md | head -n -1

# Testing
sed -n '/## \[Testing\]/,/## \[Implementation Order\]/p' docs/implementation_plan.md | head -n -1

# Implementation Order
sed -n '/## \[Implementation Order\]/,/## Navigation Commands/p' docs/implementation_plan.md | head -n -1
```
