<script lang="ts">
  import { metadata } from '../stores/metadata';
  import { viewer } from '../stores/viewer';

  type InfoRow = {
    label: string;
    value: string;
  };

  $: exif = $metadata.exif;
  $: filename = $viewer.currentImage?.filename ?? '';

  $: tagsText = $metadata.userDraft.tags.join(', ');

  $: rows = ((): InfoRow[] => {
    if (!exif) return [];

    const candidates: Array<InfoRow | null> = [
      exif.date_taken ? { label: 'Date taken', value: exif.date_taken } : null,
      exif.camera_make ? { label: 'Camera make', value: exif.camera_make } : null,
      exif.camera_model ? { label: 'Camera model', value: exif.camera_model } : null,
      exif.lens_model ? { label: 'Lens model', value: exif.lens_model } : null,
      exif.author ? { label: 'Author', value: exif.author } : null,
      exif.description ? { label: 'Description', value: exif.description } : null,
      exif.software ? { label: 'Software', value: exif.software } : null,
      exif.copyright ? { label: 'Copyright', value: exif.copyright } : null,
      exif.orientation ? { label: 'Orientation', value: exif.orientation } : null,
    ];

    return candidates.filter((row): row is InfoRow => row !== null);
  })();

  function handleTagsInput(event: Event) {
    const target = event.target as HTMLInputElement;
    const tags = target.value
      .split(',')
      .map((t) => t.trim())
      .filter((t) => t.length > 0);
    metadata.updateUserDraft({ tags });
  }

  function handleRatingChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    const value = target.value;
    metadata.updateUserDraft({ rating: value === '' ? null : parseInt(value, 10) });
  }

  function handleDateChange(event: Event) {
    const target = event.target as HTMLInputElement;
    metadata.updateUserDraft({ date: target.value === '' ? null : target.value });
  }
</script>

<aside class="inspector" aria-label="Inspector" on:contextmenu|stopPropagation>
  <div class="header">
    <div class="title">Inspector</div>
    <div class="subtitle">{filename || 'No file selected'}</div>
  </div>

  <div class="content">
    {#if !$viewer.currentImage}
      <div class="state">Open an image to view and edit metadata.</div>
    {:else}
      {#if $metadata.error}
        <div class="state error">{$metadata.error}</div>
      {/if}

      <div class="card">
        <div class="card-header">
          <div class="card-title">Details</div>
          {#if $metadata.userDirty}
            <div class="badge" title="Unsaved changes">Unsaved</div>
          {/if}
        </div>

        <div class="field">
          <label class="field-label" for="meta-description">Description</label>
          <textarea
            id="meta-description"
            class="textarea"
            rows="3"
            value={$metadata.userDraft.description}
            on:input={(e) =>
              metadata.updateUserDraft({ description: (e.target as HTMLTextAreaElement).value })}
            placeholder="Add a description…"
          ></textarea>
        </div>

        <div class="field">
          <label class="field-label" for="meta-tags">Tags</label>
          <input
            id="meta-tags"
            class="input"
            type="text"
            value={tagsText}
            on:input={handleTagsInput}
            placeholder="tag1, tag2, …"
          />
        </div>

        <div class="field-row">
          <div class="field">
            <label class="field-label" for="meta-rating">Rating</label>
            <select
              id="meta-rating"
              class="select"
              value={$metadata.userDraft.rating ?? ''}
              on:change={handleRatingChange}
            >
              <option value="">—</option>
              <option value="1">1</option>
              <option value="2">2</option>
              <option value="3">3</option>
              <option value="4">4</option>
              <option value="5">5</option>
            </select>
          </div>

          <div class="field">
            <label class="field-label" for="meta-date">Date</label>
            <input
              id="meta-date"
              class="input"
              type="date"
              value={$metadata.userDraft.date ?? ''}
              on:change={handleDateChange}
            />
          </div>
        </div>

        <div class="field">
          <label class="field-label" for="meta-author">Author</label>
          <input
            id="meta-author"
            class="input"
            type="text"
            value={$metadata.userDraft.author}
            on:input={(e) =>
              metadata.updateUserDraft({ author: (e.target as HTMLInputElement).value })}
            placeholder="Author…"
          />
        </div>

        <div class="actions">
          <button
            type="button"
            class="button"
            on:click={() => metadata.discardUserDraft()}
            disabled={!$metadata.userDirty}
          >
            Discard
          </button>
          <button
            type="button"
            class="button primary"
            on:click={() => metadata.saveUserDraft()}
            disabled={!$metadata.userDirty}
          >
            Save
          </button>
        </div>
      </div>

      <div class="card">
        <div class="card-header">
          <div class="card-title">EXIF</div>
          {#if $metadata.loading && !exif}
            <div class="badge subtle">Loading</div>
          {/if}
        </div>

        {#if rows.length === 0}
          <div class="state">No EXIF data found.</div>
        {:else}
          <dl class="rows">
            {#each rows as row (row.label)}
              <div class="row">
                <dt class="label">{row.label}</dt>
                <dd class="value">{row.value}</dd>
              </div>
            {/each}
          </dl>
        {/if}
      </div>
    {/if}
  </div>
</aside>

<style>
  .inspector {
    width: 320px;
    flex: 0 0 auto;
    height: 100%;
    background: #0f0f0f;
    border-left: 1px solid #262626;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .header {
    padding: 0.85rem 0.85rem 0.75rem;
    border-bottom: 1px solid #262626;
  }

  .title {
    font-size: 0.9rem;
    color: #e6e6e6;
    user-select: none;
  }

  .subtitle {
    margin-top: 0.2rem;
    font-size: 0.75rem;
    color: #8a8a8a;
    user-select: none;
  }

  .content {
    flex: 1;
    overflow: auto;
    padding: 0.85rem;
    display: flex;
    flex-direction: column;
    gap: 0.9rem;
  }

  .state {
    color: #8a8a8a;
    font-size: 0.85rem;
    user-select: none;
  }

  .state.error {
    color: #ff8a80;
    white-space: pre-wrap;
    word-break: break-word;
  }

  .rows {
    display: flex;
    flex-direction: column;
    gap: 0.65rem;
    margin: 0;
  }

  .row {
    display: grid;
    grid-template-columns: 1fr;
    gap: 0.2rem;
  }

  .label {
    color: #a8a8a8;
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    user-select: none;
  }

  .value {
    color: #e2e2e2;
    font-size: 0.85rem;
    margin: 0;
    word-break: break-word;
  }

  .card {
    border: 1px solid #232323;
    background: #121212;
    border-radius: 12px;
    padding: 0.85rem;
  }

  .card-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
    margin-bottom: 0.7rem;
  }

  .card-title {
    font-size: 0.75rem;
    color: #a8a8a8;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    user-select: none;
  }

  .badge {
    padding: 0.15rem 0.45rem;
    border-radius: 999px;
    border: 1px solid #3a3a3a;
    background: #1a1a1a;
    color: #e6e6e6;
    font-size: 0.72rem;
    user-select: none;
  }

  .badge.subtle {
    color: #b0b0b0;
    border-color: #2a2a2a;
    background: #141414;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
    margin-bottom: 0.75rem;
  }

  .field-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.75rem;
  }

  .field-label {
    color: #cfcfcf;
    font-size: 0.8rem;
    user-select: none;
  }

  .input,
  .textarea,
  .select {
    background: #151515;
    border: 1px solid #2a2a2a;
    border-radius: 6px;
    color: #e6e6e6;
    font-size: 0.85rem;
    padding: 0.5rem 0.6rem;
  }

  .textarea {
    resize: vertical;
    min-height: 64px;
  }

  .input:focus,
  .textarea:focus,
  .select:focus {
    outline: none;
    border-color: #3a3a3a;
  }

  .actions {
    display: flex;
    gap: 0.6rem;
    justify-content: flex-end;
  }

  .button {
    padding: 0.5rem 0.75rem;
    border-radius: 6px;
    border: 1px solid #2a2a2a;
    background: #1a1a1a;
    color: #e6e6e6;
    font-size: 0.85rem;
  }

  .button:hover:not(:disabled) {
    border-color: #3a3a3a;
    background: #202020;
  }

  .button:disabled {
    opacity: 0.45;
    cursor: not-allowed;
  }

  .button.primary {
    border-color: #3a3a3a;
    background: #2a2a2a;
  }
</style>
