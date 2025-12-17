<script lang="ts">
  import { createEventDispatcher, onDestroy, onMount } from 'svelte';
  import { settings, type UiScale } from '../stores/settings';

  export let open = false;

  const scaleOptions: UiScale[] = [100, 110, 125, 150, 175, 200];

  function handleScaleChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    const scale = parseInt(target.value) as UiScale;
    settings.setUiScale(scale);
  }

  const dispatch = createEventDispatcher<{ close: void }>();

  function close() {
    dispatch('close');
  }

  function handleOverlayClick(event: MouseEvent) {
    if (event.target !== event.currentTarget) return;
    close();
  }

  function handleOverlayKeydown(event: KeyboardEvent) {
    if (event.target !== event.currentTarget) return;
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      close();
    }
  }

  function handleGlobalKeydown(event: KeyboardEvent) {
    if (!open) return;
    if (event.key === 'Escape') {
      event.preventDefault();
      close();
    }
  }

  onMount(() => {
    window.addEventListener('keydown', handleGlobalKeydown);
  });

  onDestroy(() => {
    window.removeEventListener('keydown', handleGlobalKeydown);
  });
</script>

{#if open}
  <div
    class="overlay"
    role="button"
    tabindex="0"
    on:click={handleOverlayClick}
    on:keydown={handleOverlayKeydown}
    aria-label="Close settings"
  >
    <div class="dialog" role="dialog" aria-modal="true" aria-label="Settings">
      <div class="header">
        <div class="title">Settings</div>
        <button class="close" type="button" on:click={close} aria-label="Close">
          ×
        </button>
      </div>

      <div class="content">
        <div class="setting-row">
          <label for="ui-scale" class="setting-label">UI Scale</label>
          <select
            id="ui-scale"
            class="setting-select"
            value={$settings.uiScale}
            on:change={handleScaleChange}
          >
            {#each scaleOptions as scale}
              <option value={scale}>{scale}%</option>
            {/each}
          </select>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.55);
    display: flex;
    align-items: flex-start;
    justify-content: center;
    padding: 64px 16px 16px;
    z-index: 100;
  }

  .dialog {
    width: min(560px, 100%);
    background: #141414;
    border: 1px solid #2a2a2a;
    border-radius: 10px;
    box-shadow: 0 16px 40px rgba(0, 0, 0, 0.6);
    overflow: hidden;
  }

  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.75rem 0.9rem;
    border-bottom: 1px solid #2a2a2a;
  }

  .title {
    font-size: 0.95rem;
    color: #e6e6e6;
    user-select: none;
  }

  .close {
    width: 34px;
    height: 34px;
    padding: 0;
    border: 0;
    border-radius: 6px;
    background: transparent;
    color: #cfcfcf;
    font-size: 1.1rem;
  }

  .close:hover {
    background: #2a2a2a;
  }

  .content {
    padding: 1rem 0.9rem 1.1rem;
  }

  .setting-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
  }

  .setting-label {
    color: #e6e6e6;
    font-size: 0.9rem;
    user-select: none;
  }

  .setting-select {
    padding: 0.4rem 0.6rem;
    background: #1e1e1e;
    border: 1px solid #3a3a3a;
    border-radius: 5px;
    color: #e6e6e6;
    font-size: 0.85rem;
    cursor: pointer;
    min-width: 100px;
  }

  .setting-select:hover {
    border-color: #4a4a4a;
  }

  .setting-select:focus {
    outline: none;
    border-color: #5a5a5a;
  }
</style>