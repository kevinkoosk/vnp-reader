<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { listen } from "@tauri-apps/api/event";
  import Stage from "$lib/components/Stage.svelte";
  import DialogueBox from "$lib/components/DialogueBox.svelte";
  import SaveLoadModal from "$lib/components/SaveLoadModal.svelte";
  import type { ManifestSummary, PresentationSnapshot, StepEventDto, StepResponse } from "$lib/types";

  let manifest: ManifestSummary | null = null;
  let currentEvent: StepEventDto | null = null;
  let currentExcerpt: string = "";
  let modalMode: "save" | "load" | null = null;

  let snapshot: PresentationSnapshot = {
    objects: [],
    camera: { x: 0, y: 0, scale: 1, rotation: 0 },
    theme: "default",
    nvl_page: [],
  };

  onMount(async () => {
    try {
      await listen<{ paths?: string[] } | string[]>("tauri://drag-drop", async (event) => {
        let paths: string[] = [];
        if (Array.isArray(event.payload)) {
          paths = event.payload;
        } else if (event.payload && Array.isArray(event.payload.paths)) {
          paths = event.payload.paths;
        }

        if (paths.length > 0 && paths[0].endsWith(".vnp")) {
          await openPackageFile(paths[0]);
        }
      });
    } catch (err) {
      console.warn("Drag-drop listener registration deferred:", err);
    }
  });

  async function handleBrowsePackage() {
    try {
      const selected = await open({
        multiple: false,
        filters: [{ name: "Visual Narrative Package", extensions: ["vnp"] }],
      });

      if (typeof selected === "string") {
        await openPackageFile(selected);
      }
    } catch (err) {
      console.error("Browse failed:", err);
    }
  }

  async function openPackageFile(path: string) {
    try {
      manifest = await invoke<ManifestSummary>("load_package", { packagePath: path });
      await stepStory();
    } catch (err) {
      alert(`Package verification failed: ${err}`);
    }
  }

  async function stepStory() {
    try {
      const res = await invoke<StepResponse>("advance_story");
      currentEvent = res.event;
      snapshot = res.presentation;

      if (res.event.type === "Dialogue") {
        currentExcerpt = res.event.payload.text;
      } else if (res.event.type === "Narration") {
        currentExcerpt = res.event.payload.text;
      }
    } catch (err) {
      console.error("Step execution error:", err);
    }
  }

  async function selectChoice(optionId: string) {
    try {
      const res = await invoke<StepResponse>("choose_option", { optionId });
      currentEvent = res.event;
      snapshot = res.presentation;
    } catch (err) {
      console.error("Choice evaluation error:", err);
    }
  }

  function handleRestored(e: CustomEvent<PresentationSnapshot>) {
    snapshot = e.detail;
    stepStory();
  }
</script>

<main class="vnp-reader">
  {#if !manifest}
    <div class="welcome-screen">
      <h1 class="text-3xl font-bold text-slate-100">Visual Narrative Player</h1>
      <p class="text-slate-300">Drag and drop a <code>.vnp</code> package here, or browse from your disk.</p>
      <button class="open-btn" on:click={handleBrowsePackage}>
        Open .vnp Package...
      </button>
    </div>
  {:else}
    <header class="reader-toolbar">
      <span class="novel-title font-semibold">{manifest.title}</span>
      <div class="actions">
        <button on:click={() => (modalMode = "save")}>Save</button>
        <button on:click={() => (modalMode = "load")}>Load</button>
      </div>
    </header>

    <Stage
      background={snapshot.background}
      objects={snapshot.objects}
      camera={snapshot.camera}
    />

    <DialogueBox
      event={currentEvent}
      mode="adv"
      nvlPage={snapshot.nvl_page}
      on:advance={stepStory}
      on:choice={(e) => selectChoice(e.detail)}
    />

    {#if modalMode}
      <SaveLoadModal
        mode={modalMode}
        workId={manifest.id}
        workTitle={manifest.title}
        currentSceneTitle={snapshot.background || "Scene"}
        {currentExcerpt}
        on:close={() => (modalMode = null)}
        on:restored={handleRestored}
      />
    {/if}
  {/if}
</main>

<style>
  :global(body, html) {
    margin: 0;
    padding: 0;
    width: 100vw;
    height: 100vh;
    overflow: hidden;
    background: #0b0c10;
    font-family: system-ui, -apple-system, sans-serif;
  }
  .vnp-reader {
    width: 100%;
    height: 100%;
    position: relative;
  }
  .welcome-screen {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    color: #e5e9f0;
    gap: 1.2rem;
    border: 2px dashed #4c566a;
    margin: 2rem;
    border-radius: 12px;
    height: calc(100% - 4rem);
    box-sizing: border-box;
    background: #11141c;
  }
  .open-btn {
    background: #5e81ac;
    color: #fff;
    border: none;
    padding: 0.8rem 1.6rem;
    font-size: 1.1rem;
    border-radius: 6px;
    cursor: pointer;
    transition: background 0.2s;
  }
  .open-btn:hover {
    background: #81a1c1;
  }
  .reader-toolbar {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 48px;
    background: linear-gradient(to bottom, rgba(0, 0, 0, 0.85), transparent);
    z-index: 60;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 1.5rem;
    color: #fff;
  }
  .actions button {
    background: rgba(255, 255, 255, 0.15);
    border: 1px solid rgba(255, 255, 255, 0.25);
    color: #fff;
    padding: 0.35rem 0.85rem;
    border-radius: 4px;
    cursor: pointer;
    margin-left: 0.5rem;
  }
  .actions button:hover {
    background: rgba(255, 255, 255, 0.25);
  }
</style>