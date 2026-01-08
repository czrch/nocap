<script lang="ts">
  import FileTreeNode from './FileTreeNode.svelte';
  import type { FsEntry } from '../types';
  import { fileTree } from '../stores/fileTree';
  import { viewer } from '../stores/viewer';
  import { imageFileFromPath, isSupportedImage } from '../utils/images';

  function handleToggle(event: CustomEvent<string>) {
    fileTree.toggle(event.detail);
  }

  function handleSelect(event: CustomEvent<FsEntry>) {
    const entry = event.detail;
    if (entry.kind !== 'file') return;
    if (!isSupportedImage(entry.path)) return;
    viewer.loadImage(imageFileFromPath(entry.path));
  }
</script>

<aside class="sidebar" role="navigation" aria-label="Folder">
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
</aside>

<style>
  .sidebar {
    width: 260px;
    min-width: 200px;
    max-width: 320px;
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
</style>
