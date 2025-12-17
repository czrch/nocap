# Troubleshooting Guide

## Common Issues and Solutions

### Issue 1: "Development Project Initializer" Appearing During `npm run tauri:dev`

**Symptoms:**
- When running `npm run tauri:dev`, an interactive menu appears asking to select project type
- Error: "Invalid choice"
- Dev server fails to start properly

**Root Cause:**
The shell configuration (`~/.zshrc`) had a function named `dev()` that conflicted with development commands. When npm scripts spawn subshells, this function was being triggered.

**Solution:**
Renamed the `dev()` function to `init_project()` in `~/.zshrc` (line 825).

```bash
# Backup created at: ~/.zshrc.backup
# Changed from: dev()
# Changed to: init_project()
```

**To use the project initializer now:**
```bash
init_project  # instead of dev
```

---

### Issue 2: Wayland Display Error (Error 71: Protocol error)

**Symptoms:**
```
Gdk-Message: Error 71 (Protocol error) dispatching to Wayland display.
```
- Application window may crash or fail to render
- Common on Linux systems using Wayland compositor

**Root Cause:**
Tauri/WebKit has compatibility issues with certain Wayland configurations, particularly with DMA-BUF rendering.

**Solution:**
Added Wayland compatibility environment variable to npm scripts in `package.json`:

```json
"tauri:dev": "WEBKIT_DISABLE_DMABUF_RENDERER=1 tauri dev"
```

**Alternative Solution (X11 fallback):**
If the primary fix doesn't work, use X11 compatibility mode:

```bash
npm run tauri:dev:x11
```

This forces the app to use XWayland instead of native Wayland.

**Environment Variables Used:**
- `WEBKIT_DISABLE_DMABUF_RENDERER=1` - Disables DMA-BUF rendering in WebKit
- `WAYLAND_DISPLAY=` - Unsets Wayland, forcing X11 fallback

---

### Issue 3: Accessibility Warning - Mouse Events on Non-Interactive Element

**Symptoms:**
```
A11y: Non-interactive element <div> should not be assigned mouse or keyboard event listeners.
```

**Root Cause:**
The ImageViewer component uses a `<div>` with mouse event handlers (wheel, mousedown, etc.) but wasn't properly marked as an interactive element.

**Solution:**
Updated `src/lib/components/ImageViewer.svelte`:

```svelte
<div 
  role="application"
  aria-label="Image viewer with zoom and pan controls"
  tabindex="0"
  ...
>
```

**Changes Made:**
- Changed `role="img"` to `role="application"` (more appropriate for interactive content)
- Enhanced `aria-label` to describe interactive capabilities
- Added `tabindex="0"` to make it keyboard focusable

---

## Quick Reference

### Recommended Development Command
```bash
npm run tauri:dev
```

### If Wayland Issues Persist
```bash
npm run tauri:dev:x11
```

### Check Environment
```bash
echo $WAYLAND_DISPLAY  # Should show: wayland-0
echo $XDG_SESSION_TYPE # Should show: wayland or x11
```

### Restore Original Shell Config (if needed)
```bash
cp ~/.zshrc.backup ~/.zshrc
```

---

## System Information (Reference)

**Resolved on:**
- OS: Linux 6.17
- Session: Wayland
- Shell: zsh

**Dependencies:**
- Tauri: v2.0.0
- Svelte: v4.2.0
- Vite: v5.0.0