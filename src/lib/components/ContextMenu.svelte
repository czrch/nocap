<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import MenuList from './MenuList.svelte';

  export let x = 0;
  export let y = 0;
  export let show = false;

  let menuElement: HTMLDivElement;

  function closeMenu() {
    show = false;
  }

  function handleClickOutside(event: MouseEvent) {
    if (show && menuElement && !menuElement.contains(event.target as Node)) {
      closeMenu();
    }
  }

  function handleEscape(event: KeyboardEvent) {
    if (event.key === 'Escape' && show) {
      closeMenu();
    }
  }

  onMount(() => {
    document.addEventListener('click', handleClickOutside);
    document.addEventListener('keydown', handleEscape);
  });

  onDestroy(() => {
    document.removeEventListener('click', handleClickOutside);
    document.removeEventListener('keydown', handleEscape);
  });

  // Adjust menu position to stay within viewport bounds
  $: adjustedX = x;
  $: adjustedY = y;

  $: if (show && menuElement) {
    const rect = menuElement.getBoundingClientRect();
    const viewportWidth = window.innerWidth;
    const viewportHeight = window.innerHeight;

    adjustedX = x + rect.width > viewportWidth ? viewportWidth - rect.width - 10 : x;
    adjustedY = y + rect.height > viewportHeight ? viewportHeight - rect.height - 10 : y;
  }
</script>

{#if show}
  <div
    bind:this={menuElement}
    class="context-menu"
    style="left: {adjustedX}px; top: {adjustedY}px;"
    role="menu"
    aria-label="Context menu"
  >
    <MenuList closeMenu={closeMenu} />
  </div>
{/if}

<style>
  .context-menu {
    position: fixed;
    z-index: 1000;
    background: #1a1a1a;
    border: 1px solid #333;
    border-radius: 6px;
    padding: 0.4rem;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
    min-width: 200px;
  }
</style>