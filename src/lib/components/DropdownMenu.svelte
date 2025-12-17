<script lang="ts">
  import { createEventDispatcher, onDestroy, onMount } from 'svelte';

  export let open = false;
  export let menuAriaLabel: string;
  export let align: 'left' | 'right' = 'left';

  const dispatch = createEventDispatcher<{ open: void; close: void }>();

  let container: HTMLDivElement | null = null;

  function close() {
    if (!open) return;
    open = false;
    dispatch('close');
  }

  function toggle() {
    open = !open;
    dispatch(open ? 'open' : 'close');
  }

  function handleGlobalPointerDown(event: PointerEvent) {
    if (!open) return;
    const target = event.target;
    if (!(target instanceof Node)) return;
    if (!container) return;
    if (!container.contains(target)) close();
  }

  function handleGlobalKeydown(event: KeyboardEvent) {
    if (!open) return;
    if (event.key === 'Escape') {
      event.preventDefault();
      close();
    }
  }

  onMount(() => {
    window.addEventListener('pointerdown', handleGlobalPointerDown, true);
    window.addEventListener('keydown', handleGlobalKeydown);
  });

  onDestroy(() => {
    window.removeEventListener('pointerdown', handleGlobalPointerDown, true);
    window.removeEventListener('keydown', handleGlobalKeydown);
  });
</script>

<div class="menu-container" bind:this={container}>
  <slot name="trigger" {open} {toggle} {close} />

  {#if open}
    <div class="menu {align}" role="menu" aria-label={menuAriaLabel}>
      <slot name="menu" {close} />
    </div>
  {/if}
</div>

<style>
  .menu-container {
    position: relative;
    display: flex;
    align-items: center;
  }

  .menu {
    position: absolute;
    top: calc(100% + 6px);
    min-width: 180px;
    background: #1f1f1f;
    border: 1px solid #333;
    border-radius: 6px;
    padding: 0.25rem;
    box-shadow: 0 10px 24px rgba(0, 0, 0, 0.5);
    z-index: 20;
  }

  .menu.left {
    left: 0;
  }

  .menu.right {
    right: 0;
  }
</style>

