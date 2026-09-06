# VNP Reader

A lightweight, secure desktop runtime and player for **Visual Novel Package (`.vnp`)** digital interactive stories and visual novels.

Built on **Tauri v2**, **Rust**, and **Svelte 5 / TypeScript**.

The [VNP specification](https://github.com/kevinkoosk/vnp-specification) allows a visual novel to be created with Markdown and images, and distributed as a zip file with the extension of .vnp.

## Features

- **Strict Container Verification**: Validates uncompressed `mimetype` markers and enforces SHA-256 digest integrity across all scenes and assets via `META-INF/integrity.json`.

- **Deterministic Execution Engine**: Core state machine written in Rust with bounded integer/boolean/string registries, condition evaluation, and deterministic 32-bit PRNG (`VNP-PRNG1`).

- **Standardized Presentation Layer**: Responsive staging with depth layering (`rear` to `interface`), normalized coordinates, camera transformations, and dialogue/narration display modes (ADV and NVL).

- **Portable Save System**: Rest-point based save serializer generating standard JSON saves with multi-slot local storage and external file import/export.

- **Cross-Platform Desktop App**: Minimal resource footprint powered by Tauri v2 and native OS WebViews.


## Architecture Overview

```
+--------------------------------------------------------+  
|               Svelte 5 Frontend (UI)                   |  
|   (Stage Rendering, ADV/NVL Dialogue, Save/Load Modal) |  
+--------------------------------------------------------+  
                            | Tauri IPC (Commands & Events)  
+---------------------------v----------------------------+  
|               Rust Core Backend (vnp\_core)             |  
|   +-- Container & SHA-256 Verifier (VnpPackage)        |  
|   +-- State Registry & Expression Evaluator            |  
|   +-- Deterministic Abstract Machine (RuntimeMachine)  |  
|   +-- Portable Save Manager (SaveManager)              |  
+--------------------------------------------------------+
```


## Prerequisites

Ensure you have the following installed on your development system:

- [Node.js](https://nodejs.org/) (v18 or newer)

- [Rust & Cargo](https://www.rust-lang.org/tools/install) (latest stable)

- Build tools for your OS (e.g., C++ build tools on Windows)


## Getting Started

### 1. Clone the Repository

```
git clone \[https://github.com/YOUR-USERNAME/vnp-reader.git\](https://github.com/YOUR-USERNAME/vnp-reader.git)  
cd vnp-reader
```

### 2. Install Node Dependencies

```
npm install
```

### 3. Run Development Server

```
npm run tauri dev
```

### 4. Build for Production

```
npm run tauri build
```

The compiled standalone installer and executable will be generated in `src-tauri/target/release/`.


## Usage

1. Launch **VNP Reader**.

2. Drag and drop any compliant `.vnp` file directly onto the window, or click **"Open .vnp Package..."** to browse from disk.

3. Click or press `Space` / `Enter` to advance dialogue beats and narration.

4. Use the **Save** and **Load** buttons in the top toolbar to manage playthrough states.


## Specification Conformance

VNP Reader adheres to the open Visual Narrative specifications:

- **Container Format**: Constrained ZIP archive with uncompressed `mimetype` first entry and SHA-256 integrity map.

- **Document Canonicalization**: Deterministic JSON scene representations.

- **Save Conformance**: RFC 3339 UTC timestamps and complete execution snapshot state capture.


## License

This project is licensed under the **MIT License**. See the [LICENSE](https://www.google.com/search?q=LICENSE) file for full details.

