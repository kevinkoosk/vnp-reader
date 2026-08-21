export interface ManifestSummary {
  id: string;
  title: string;
  version: string;
  default_language: string;
  entry_scene: string;
}

export interface ChoiceOptionDto {
  id: string;
  label: string;
}

export type StepEventDto =
  | { type: "Narration"; payload: { beat_id: string; text: string } }
  | { type: "Dialogue"; payload: { beat_id: string; speaker: string; speaker_display: string; text: string; voice_resource?: string } }
  | { type: "Choice"; payload: { choice_id: string; options: ChoiceOptionDto[] } }
  | { type: "Ended"; payload: { title: string; result: string } };

export interface StagedObject {
  id: string;
  expression?: string;
  position?: "far-left" | "left" | "centre-left" | "centre" | "centre-right" | "right" | "far-right";
  layer: "rear" | "stage" | "front" | "interface";
  scale: number;
  opacity: number;
}

export interface PresentationSnapshot {
  background?: string;
  objects: StagedObject[];
  camera: { x: number; y: number; scale: number; rotation: number };
  theme: string;
  nvl_page: string[];
  music?: { resource: string; volume: number };
  ambient?: { resource: string; volume: number };
}

export interface StepResponse {
  event: StepEventDto;
  presentation: PresentationSnapshot;
}
