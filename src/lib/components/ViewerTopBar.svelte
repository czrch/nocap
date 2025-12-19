<script lang="ts">
  import { isTauri, invoke } from '@tauri-apps/api/core';
  import { save } from '@tauri-apps/plugin-dialog';
  import { tick } from 'svelte';
  import { metadata } from '../stores/metadata';
  import { viewer } from '../stores/viewer';
  import DropdownMenu from './DropdownMenu.svelte';
  import MenuList from './MenuList.svelte';
  import type { ImageFile } from '../types';

  const isNative = isTauri();

  let menuOpen = false;

  let editingName = false;
  let nameDraft = '';
  let busy = false;
  let error: string | null = null;
  let nameInputEl: HTMLInputElement | null = null;

  $: current = $viewer.currentImage;
  $: image = $metadata.image;

  $: typeLabel = image?.format
    ? current?.extension
      ? `${image.format.toUpperCase()} (.${current.extension})`
      : image.format.toUpperCase()
    : current?.extension
      ? `.${current.extension}`
      : '—';

  $: sizeLabel = image ? `${formatBytes(image.size)}` : '—';
  $: resolutionLabel =
    image && image.width > 0 && image.height > 0 ? `${image.width} × ${image.height}` : '—';

  $: canSaveName =
    Boolean(current) &&
    nameDraft.trim().length > 0 &&
    nameDraft.trim() !== (current?.filename ?? '');

  $: if (current && !editingName) {
    nameDraft = current.filename;
  }

  function formatBytes(bytes: number): string {
    if (!Number.isFinite(bytes) || bytes <= 0) return '0 B';
    const units = ['B', 'KB', 'MB', 'GB', 'TB'] as const;
    const unitIndex = Math.min(units.length - 1, Math.floor(Math.log(bytes) / Math.log(1024)));
    const value = bytes / Math.pow(1024, unitIndex);
    const formatted = value >= 10 || unitIndex === 0 ? value.toFixed(0) : value.toFixed(1);
    return `${formatted} ${units[unitIndex]}`;
  }

  async function startEditing() {
    if (!current) return;
    editingName = true;
    error = null;
    nameDraft = current.filename;
    await tick();
    nameInputEl?.focus();
    nameInputEl?.select();
  }

  function cancelEditing() {
    editingName = false;
    error = null;
    nameDraft = current?.filename ?? '';
  }

  async function renameCurrent() {
    if (!current) return;
    if (!canSaveName) return;
    if (!isNative) return;

    busy = true;
    error = null;
    const oldPath = current.path;

    try {
      const updated = await invoke<ImageFile>('rename_image_file', {
        path: oldPath,
        newName: nameDraft.trim(),
      });

      metadata.moveUserMetadata(oldPath, updated.path);
      viewer.replaceCurrentImage(updated);
      editingName = false;
    } catch (err) {
      error = String(err);
    } finally {
      busy = false;
    }
  }

  async function saveAsCopy() {
    if (!current) return;
    if (!isNative) return;

    busy = true;
    error = null;
    const oldPath = current.path;

    try {
      const selected = await save({
        defaultPath: current.path,
      });

      if (!selected) return;

      const copied = await invoke<ImageFile>('save_as_copy', {
        path: oldPath,
        destinationPath: selected,
      });

      metadata.copyUserMetadata(oldPath, copied.path);
      viewer.loadImage(copied);
      editingName = false;
    } catch (err) {
      error = String(err);
    } finally {
      busy = false;
    }
  }

  function handleNameKeydown(event: KeyboardEvent) {
    if (!editingName) return;
    if (event.key === 'Enter') {
      event.preventDefault();
      void renameCurrent();
    } else if (event.key === 'Escape') {
      event.preventDefault();
      cancelEditing();
    }
  }
</script>

<div class="topbar" aria-label="Viewer toolbar">
  <div class="left">
    <DropdownMenu bind:open={menuOpen} menuAriaLabel="Application menu" align="left">
      <svelte:fragment slot="trigger" let:open let:toggle>
        <button
          type="button"
          class="icon-button"
          on:click={toggle}
          aria-haspopup="menu"
          aria-expanded={open}
          title="Menu"
          aria-label="Menu"
          disabled={!isNative}
        >
          ☰
        </button>
      </svelte:fragment>

      <svelte:fragment slot="menu" let:close>
        <MenuList closeMenu={close} />
      </svelte:fragment>
    </DropdownMenu>

    <div class="file">
      {#if current}
        {#if editingName}
          <input
            class="name-input"
            type="text"
            bind:this={nameInputEl}
            value={nameDraft}
            on:input={(e) => (nameDraft = (e.target as HTMLInputElement).value)}
            on:keydown={handleNameKeydown}
            disabled={busy || !isNative}
            aria-label="File name"
          />
        {:else}
          <button
            type="button"
            class="name-button"
            on:click={startEditing}
            disabled={!isNative}
            title={isNative ? 'Rename' : 'Rename (native only)'}
          >
            {current.filename}
          </button>
        {/if}
      {:else}
        <div class="name-placeholder">No file selected</div>
      {/if}

      {#if error}
        <div class="error" title={error}>{error}</div>
      {/if}
    </div>
  </div>

  <div class="center" aria-label="File info">
    {#if current}
      <div class="chip" title="File type">{typeLabel}</div>
      <div class="chip" title="Resolution">{resolutionLabel}</div>
      <div class="chip" title="File size">{sizeLabel}</div>
    {/if}
  </div>

  <div class="right">
    <button
      type="button"
      class="action"
      on:click={renameCurrent}
      disabled={!canSaveName || busy || !isNative}
      title="Save (rename)"
    >
      Save
    </button>
    <button
      type="button"
      class="action primary"
      on:click={saveAsCopy}
      disabled={!current || busy || !isNative}
      title="Save as (copy)"
    >
      Save As…
    </button>
  </div>
</div>

<style>
  .topbar {
    height: 42px;
    background: #0f0f0f;
    border-bottom: 1px solid #222;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 10px;
    gap: 10px;
    flex: 0 0 auto;
  }

  .left,
  .center,
  .right {
    display: flex;
    align-items: center;
    gap: 10px;
    min-width: 0;
  }

  .left {
    flex: 1;
  }

  .center {
    flex: 0 0 auto;
    justify-content: center;
  }

  .right {
    flex: 0 0 auto;
    justify-content: flex-end;
  }

  .icon-button {
    width: 34px;
    height: 30px;
    padding: 0;
    border: 1px solid #2a2a2a;
    border-radius: 8px;
    background: #151515;
    color: #e6e6e6;
    font-size: 1rem;
  }

  .icon-button:hover:not(:disabled) {
    background: #1c1c1c;
    border-color: #3a3a3a;
  }

  .icon-button:disabled {
    opacity: 0.45;
    cursor: not-allowed;
  }

  .file {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  .name-button {
    border: 0;
    background: transparent;
    color: #ededed;
    font-size: 0.9rem;
    text-align: left;
    padding: 0;
    max-width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .name-button:hover:not(:disabled) {
    text-decoration: underline;
    text-underline-offset: 3px;
  }

  .name-button:disabled {
    opacity: 0.6;
  }

  .name-input {
    width: min(520px, 52vw);
    max-width: 100%;
    background: #151515;
    border: 1px solid #2a2a2a;
    border-radius: 8px;
    color: #e6e6e6;
    font-size: 0.9rem;
    padding: 0.45rem 0.6rem;
  }

  .name-input:focus {
    outline: none;
    border-color: #3a3a3a;
  }

  .name-placeholder {
    color: #888;
    font-size: 0.9rem;
    user-select: none;
  }

  .error {
    margin-top: 2px;
    font-size: 0.75rem;
    color: #ff8a80;
    max-width: min(520px, 52vw);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .chip {
    padding: 0.28rem 0.55rem;
    border: 1px solid #2a2a2a;
    background: #141414;
    border-radius: 999px;
    font-size: 0.75rem;
    color: #cfcfcf;
    user-select: none;
  }

  .action {
    height: 30px;
    padding: 0 0.75rem;
    border-radius: 8px;
    border: 1px solid #2a2a2a;
    background: #151515;
    color: #e6e6e6;
    font-size: 0.85rem;
  }

  .action:hover:not(:disabled) {
    background: #1c1c1c;
    border-color: #3a3a3a;
  }

  .action:disabled {
    opacity: 0.45;
    cursor: not-allowed;
  }

  .action.primary {
    background: #242424;
    border-color: #3a3a3a;
  }
</style>
