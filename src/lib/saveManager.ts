import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import { readTextFile, writeTextFile } from "@tauri-apps/plugin-fs";
import type { PresentationSnapshot } from "./types";

export interface SaveSlotMetadata {
  slotId: number;
  workId: string;
  timestamp: string;
  sceneTitle: string;
  textExcerpt: string;
  saveJson: string;
}

export class SaveSystem {
  static getStorageKey(workId: string): string {
    return `vnp_saves_${workId}`;
  }

  static getSlots(workId: string): (SaveSlotMetadata | null)[] {
    const raw = localStorage.getItem(this.getStorageKey(workId));
    if (!raw) return [null, null, null, null];
    try {
      return JSON.parse(raw);
    } catch {
      return [null, null, null, null];
    }
  }

  static async saveToSlot(
    slotId: number,
    workId: string,
    sceneTitle: string,
    currentExcerpt: string
  ): Promise<SaveSlotMetadata> {
    const saveJson = await invoke<string>("export_save");
    const slots = this.getSlots(workId);

    const slotData: SaveSlotMetadata = {
      slotId,
      workId,
      timestamp: new Date().toISOString(),
      sceneTitle,
      textExcerpt: currentExcerpt,
      saveJson,
    };

    slots[slotId] = slotData;
    localStorage.setItem(this.getStorageKey(workId), JSON.stringify(slots));
    return slotData;
  }

  static async loadFromSlot(
    slotId: number,
    workId: string
  ): Promise<PresentationSnapshot> {
    const slots = this.getSlots(workId);
    const slot = slots[slotId];
    if (!slot) throw new Error("Save slot is empty.");

    return await invoke<PresentationSnapshot>("import_save", {
      saveJson: slot.saveJson,
    });
  }

  static async exportSaveToFile(workTitle: string): Promise<void> {
    const saveJson = await invoke<string>("export_save");
    const filePath = await save({
      filters: [{ name: "VNP Portable Save", extensions: ["json"] }],
      defaultPath: `${workTitle.toLowerCase().replace(/\s+/g, "-")}-save.json`,
    });

    if (filePath) {
      await writeTextFile(filePath, saveJson);
    }
  }

  static async importSaveFromFile(): Promise<PresentationSnapshot> {
    const selected = await open({
      multiple: false,
      filters: [{ name: "VNP Portable Save", extensions: ["json"] }],
    });

    if (typeof selected === "string") {
      const fileContent = await readTextFile(selected);
      return await invoke<PresentationSnapshot>("import_save", {
        saveJson: fileContent,
      });
    }
    throw new Error("No save file selected.");
  }
}
