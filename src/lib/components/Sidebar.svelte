<script lang="ts">
  import { onDestroy } from 'svelte';
  import FileTreeNode from './FileTreeNode.svelte';
  import type { FsEntry } from '../types';
  import { fileTree } from '../stores/fileTree';
  import { viewer } from '../stores/viewer';
  import { imageFileFromPath, isSupportedImage } from '../utils/images';

  const MIN_WIDTH = 200;
  const MAX_WIDTH = 420;
  let width = 260;
  let dragStartX = 0;
  let dragStartWidth = 0;
  let dragging = false;

  function handleToggle(event: CustomEvent<string>) {
    fileTree.toggle(event.detail);
  }

  function handleSelect(event: CustomEvent<FsEntry>) {
    const entry = event.detail;
    if (entry.kind !== 'file') return;
    if (!isSupportedImage(entry.path)) return;
    viewer.loadImage(imageFileFromPath(entry.path));
  }

  function clampWidth(nextWidth: number) {
    return Math.max(MIN_WIDTH, Math.min(MAX_WIDTH, nextWidth));
  }

  function handleResizeStart(event: PointerEvent) {
    if (event.button !== 0) return;
    dragging = true;
    dragStartX = event.clientX;
    dragStartWidth = width;
    window.addEventListener('pointermove', handleResizeMove);
    window.addEventListener('pointerup', handleResizeEnd);
  }

  function handleResizeMove(event: PointerEvent) {
    if (!dragging) return;
    const delta = event.clientX - dragStartX;
    width = clampWidth(dragStartWidth + delta);
  }

  function handleResizeEnd() {
    if (!dragging) return;
    dragging = false;
    window.removeEventListener('pointermove', handleResizeMove);
    window.removeEventListener('pointerup', handleResizeEnd);
  }

  onDestroy(() => {
    window.removeEventListener('pointermove', handleResizeMove);
    window.removeEventListener('pointerup', handleResizeEnd);
  });
</script>

<aside class="sidebar" style={`width: ${width}px`} role="navigation" aria-label="Folder">
  <div class="sidebar-header">
    <div class="title">Folder</div>
    {#if $fileTree.rootPath}
      <div class="path" title={$fileTree.rootPath}>{$fileTree.rootPath}</div>
    {/if}
  </div>

  <div class="tree" role="tree">
    {#if $fileTree.tree}
      <FileTreeNode
        entry={$fileTree.tree}
        level={0}
        expanded={$fileTree.expanded}
        on:toggle={handleToggle}
        on:select={handleSelect}
      />
    {:else}
      <div class="empty">No folder open</div>
    {/if}
  </div>

  <div class="resize-handle" on:pointerdown={handleResizeStart}></div>
</aside>

<style>
  .sidebar {
    position: relative;
    flex: 0 0 auto;
    min-width: 200px;
    max-width: 420px;
    background: #111;
    border-right: 1px solid rgba(255, 255, 255, 0.08);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .sidebar-header {
    padding: 0.75rem 0.75rem 0.5rem;
    border-bottom: 1px solid rgba(255, 255, 255, 0.06);
  }

  .title {
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: #7a7a7a;
  }

  .path {
    margin-top: 0.35rem;
    font-size: 0.8rem;
    color: #c5c5c5;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .tree {
    flex: 1;
    overflow: auto;
    padding: 0.5rem 0.25rem 0.75rem 0.5rem;
  }

  .empty {
    color: #666;
    font-size: 0.85rem;
    padding: 0.5rem 0.25rem;
  }

  .resize-handle {
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    width: 6px;
    cursor: col-resize;
  }

  .resize-handle:hover {
    background: rgba(255, 255, 255, 0.05);
  }
</style>
