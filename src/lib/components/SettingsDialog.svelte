<script lang="ts">
  import { createEventDispatcher, onDestroy, onMount } from 'svelte';

  export let open = false;

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
        <p class="hint">Settings coming soon.</p>
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

  .hint {
    color: #9a9a9a;
    font-size: 0.9rem;
  }
</style>
