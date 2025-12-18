# nocap

A minimal, cross-platform image viewer built with Tauri v2, Svelte, and Rust.

## Features

- 🖼️ **Image Viewing**: Support for JPG, PNG, GIF, BMP, WebP, SVG
- 🧾 **EXIF Inspector**: View common EXIF fields (read-only) for JPEG/TIFF
- 🔍 **Zoom Controls**: Zoom in/out, fit to window, reset zoom
- ⌨️ **Keyboard Shortcuts**: Fast navigation and control
- 📁 **Folder Support**: Open entire folders, browse through images
- 🎨 **Clean Interface**: Minimal, distraction-free design
- ⚡ **Fast & Lightweight**: Native performance with Rust backend

## Installation

### Prerequisites

- [Node.js](https://nodejs.org/) (v20.19.0 or later)
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/czrch/nocap.git
cd nocap

# Install dependencies
npm install

# Run in development mode
npm run tauri:dev

# Build for production
npm run tauri:build
```

The built application will be in `src-tauri/target/release/bundle/`.

## Usage

### Opening Images

- **Open File**: Click "📁 Open File" button or use toolbar
- **Open Folder**: Click "📂 Open Folder" to load all images in a directory

### Navigation

- **Next Image**: Click `►` or press `→` (Right Arrow)
- **Previous Image**: Click `◄` or press `←` (Left Arrow)
- **Image Counter**: View current position (e.g., "3/15")

### Zoom Controls

- **Zoom In**: Click `🔍+` or press `+`/`=`
- **Zoom Out**: Click `🔍−` or press `-`
- **Reset Zoom**: Click percentage display or press `0`
- **Fit to Window**: Click `⊞`/`⊡` or press `F`
- **Mouse Wheel**: Scroll to zoom in/out
- **Pan**: Click and drag when zoomed in

### EXIF Inspector

When an image is open, the right-side inspector panel displays common EXIF fields (when present).

Supported for EXIF viewing:

- JPEG/JPG
- TIFF/TIF

## Keyboard Shortcuts

| Action         | Shortcut   |
| -------------- | ---------- |
| Next Image     | `→`        |
| Previous Image | `←`        |
| Zoom In        | `+` or `=` |
| Zoom Out       | `-`        |
| Reset Zoom     | `0`        |
| Fit to Window  | `F`        |

## Supported Formats

- JPEG/JPG
- PNG
- GIF
- BMP
- WebP
- SVG
- TIFF/TIF (viewing + EXIF inspector)

## Development

```bash
# Run development server (frontend only)
npm run dev

# Run full Tauri app in dev mode
npm run tauri:dev

# Type check
npm run check

# Format Rust code
cd src-tauri && cargo fmt

# Lint Rust code
cd src-tauri && cargo clippy
```

## Tech Stack

- **Frontend**: Svelte 4 + TypeScript + Vite
- **Backend**: Rust + Tauri v2
- **Styling**: Vanilla CSS (minimal, dark theme)

## Project Structure

```
nocap/
├── src/                    # Frontend source
│   ├── lib/
│   │   ├── components/     # Svelte components
│   │   ├── stores/         # State management
│   │   └── types.ts        # TypeScript types
│   ├── App.svelte          # Root component
│   └── main.ts             # Entry point
├── src-tauri/              # Backend source
│   ├── src/
│   │   ├── commands.rs     # Tauri commands
│   │   ├── models.rs       # Data structures
│   │   ├── utils.rs        # Utility functions
│   │   └── lib.rs          # Main entry
│   └── Cargo.toml
└── docs/                   # Documentation
```

## License

GPL-3.0 - See [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
