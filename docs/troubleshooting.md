# Troubleshooting Guide

## Common Issues and Solutions

### Issue 1: Wayland Display Error (Error 71: Protocol error)

**Symptoms:**
```
Gdk-Message: Error 71 (Protocol error) dispatching to Wayland display.
```
- Application window may crash or fail to render
- Common on Linux systems using Wayland compositor

**Root Cause:**
Tauri/WebKit has compatibility issues with certain Wayland configurations, particularly with DMA-BUF rendering.

**Solution:**
Use the Wayland compatibility dev script:

```bash
npm run tauri:dev:wayland
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

### Issue 2: Accessibility Warning - Mouse Events on Non-Interactive Element

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

### Issue 3: Image Fails to Load After Selecting a File

**Symptoms:**
- You can select an image via the file picker, but the viewer stays blank (or the image doesn’t render)

**Root Cause:**
The app loads images via Tauri’s `asset:` protocol (`convertFileSrc()`), which is restricted by `app.security.assetProtocol.scope` in `src-tauri/tauri.conf.json`.

**Solution:**
Either move the image into one of the allowed directories (default: your home/pictures/desktop/documents/downloads), or widen the scope in `src-tauri/tauri.conf.json` under `app.security.assetProtocol.scope`.

---

## Quick Reference

### Recommended Development Command
```bash
npm run tauri:dev
```

### If Wayland Issues Persist
```bash
npm run tauri:dev:wayland
```

### X11 Fallback (Linux)
```bash
npm run tauri:dev:x11
```

### Check Environment
```bash
echo $WAYLAND_DISPLAY  # Should show: wayland-0
echo $XDG_SESSION_TYPE # Should show: wayland or x11
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
