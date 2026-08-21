<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte";
  import { SaveSystem, type SaveSlotMetadata } from "$lib/saveManager";
  import type { PresentationSnapshot } from "$lib/types";

  export let mode: "save" | "load" = "save";
  export let workId: string;
  export let workTitle: string;
  export let currentSceneTitle: string;
  export let currentExcerpt: string;

  const dispatch = createEventDispatcher<{
    close: void;
    restored: PresentationSnapshot;
  }>();

  let slots: (SaveSlotMetadata | null)[] = [];

  function refreshSlots() {
    slots = SaveSystem.getSlots(workId);
  }

  onMount(() => {
    refreshSlots();
  });

  async function handleSlotClick(index: number) {
    try {
      if (mode === "save") {
        await SaveSystem.saveToSlot(index, workId, currentSceneTitle, currentExcerpt);
        refreshSlots();
      } else {
        if (slots[index]) {
          const snapshot = await SaveSystem.loadFromSlot(index, workId);
          dispatch("restored", snapshot);
          dispatch("close");
        }
      }
    } catch (err) {
      alert(`Save/Load operation failed: ${err}`);
    }
  }

  async function handleFileExport() {
    await SaveSystem.exportSaveToFile(workTitle);
  }

  async function handleFileImport() {
    try {
      const snapshot = await SaveSystem.importSaveFromFile();
      dispatch("restored", snapshot);
      dispatch("close");
    } catch (err) {
      alert(`Import error: ${err}`);
    }
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="modal-backdrop" on:click={() => dispatch("close")}>
  <div class="modal-content" on:click|stopPropagation>
    <header class="modal-header">
      <h2>{mode === "save" ? "Save Game" : "Load Game"}</h2>
      <button class="close-btn" on:click={() => dispatch("close")}>×</button>
    </header>

    <div class="slots-grid">
      {#each [0, 1, 2, 3] as slotIdx}
        <button
          class="slot-card"
          class:occupied={slots[slotIdx] !== null}
          on:click={() => handleSlotClick(slotIdx)}
        >
          <div class="slot-number">Slot {slotIdx + 1}</div>
          {#if slots[slotIdx]}
            <div class="slot-info">
              <span class="slot-title">{slots[slotIdx]?.sceneTitle}</span>
              <span class="slot-date">{new Date(slots[slotIdx]!.timestamp).toLocaleString()}</span>
              <p class="slot-excerpt">"{slots[slotIdx]?.textExcerpt}"</p>
            </div>
          {:else}
            <div class="slot-empty">Empty Slot</div>
          {/if}
        </button>
      {/each}
    </div>

    <footer class="modal-footer">
      {#if mode === "save"}
        <button class="util-btn" on:click={handleFileExport}>Export Save File (.json)</button>
      {:else}
        <button class="util-btn" on:click={handleFileImport}>Import Save File (.json)</button>
      {/if}
    </footer>
  </div>
</div>

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.8);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 100;
  }
  .modal-content {
    background: #161922;
    border: 1px solid #3b4252;
    border-radius: 8px;
    width: 800px;
    max-width: 90vw;
    padding: 2rem;
    color: #eceff4;
  }
  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
  }
  .close-btn {
    background: transparent;
    border: none;
    font-size: 1.8rem;
    color: #d8dee9;
    cursor: pointer;
  }
  .slots-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 1rem;
    margin-bottom: 1.5rem;
  }
  .slot-card {
    background: #242933;
    border: 1px solid #434c5e;
    border-radius: 6px;
    padding: 1rem;
    text-align: left;
    color: inherit;
    cursor: pointer;
    min-height: 120px;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
  }
  .slot-card:hover {
    border-color: #88c0d0;
    background: #2e3440;
  }
  .slot-number {
    font-weight: bold;
    color: #88c0d0;
    font-size: 0.9rem;
  }
  .slot-title {
    display: block;
    font-size: 1.1rem;
    margin-top: 0.25rem;
  }
  .slot-date {
    display: block;
    font-size: 0.8rem;
    color: #9aa5b8;
  }
  .slot-excerpt {
    font-size: 0.85rem;
    font-style: italic;
    color: #d8dee9;
    margin-top: 0.5rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .slot-empty {
    margin: auto;
    color: #616e88;
  }
  .modal-footer {
    display: flex;
    justify-content: flex-end;
  }
  .util-btn {
    background: #434c5e;
    color: #eceff4;
    border: none;
    padding: 0.6rem 1.2rem;
    border-radius: 4px;
    cursor: pointer;
  }
  .util-btn:hover {
    background: #4c566a;
  }
</style>