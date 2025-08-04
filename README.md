# Siren: Real-Time Phishing Detector

Siren is a high-performance, real-time phishing detector that runs directly in your browser. It leverages a fine-tuned DistilBERT model, compiled to WebAssembly from Rust, to analyze URLs and provide instant threat assessments without sending your data to a server.

---

## Project Status

**Phase 3: Browser Extension Development - IN PROGRESS**

The core inference pipeline has been successfully implemented in Rust and compiled to WebAssembly. The browser extension UI is built and connected to the WASM module. We are currently debugging a silent failure where the WASM module is not loading correctly within the extension's environment.

## Tech Stack

*   **Machine Learning**:
    *   Python, PyTorch, Hugging Face `transformers`
    *   Hugging Face `optimum` for robust ONNX quantization and export.
*   **Core Engine**:
    *   **Rust** for performance-critical logic.
    *   **WebAssembly (WASM)** via `wasm-pack` for browser execution.
    *   `tract` crate for running ONNX inference in Rust.
*   **Browser Extension**:
    *   HTML, CSS, JavaScript
    *   **Vite** for a modern, fast build process.
    *   `vite-plugin-wasm` for seamless WASM integration.
*   **Versioning & CI/CD**:
    *   **Git & GitHub** for source code.
    *   **DVC** for data and model versioning.
    *   **DagsHub** as the remote storage backend for DVC.
    *   GitHub Actions for continuous integration.

## Project Structure

```
.siren/
├── .dvc/              # DVC metadata
├── .github/           # CI/CD workflows
├── docs/              # Project documentation (RFCs)
├── extension/
│   ├── app/           # Vite-based frontend (JS, HTML, CSS)
│   └── siren_engine/  # Rust WASM library for the core logic
├── model/             # Model training, quantization, and data scripts
└── ...                # Config files (.gitignore, justfile, etc.)
```

## How to Build and Run

Follow these steps to build and test the extension locally.

### Prerequisites

*   [Rust and Cargo](https://www.rust-lang.org/tools/install)
*   [Node.js and npm](https://nodejs.org/)
*   [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)
*   [DVC](https://dvc.org/doc/install)

### Steps

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/0xReLogic/Siren.git
    cd Siren
    ```

2.  **Pull DVC-tracked files (the model):**
    ```bash
    dvc pull
    ```

3.  **Build the WebAssembly module:**
    Navigate to the Rust engine directory and build it.
    ```bash
    cd extension/siren_engine
    wasm-pack build --target browser --out-dir pkg
    ```

4.  **Build the extension frontend:**
    Navigate to the Vite app directory, install dependencies, and build.
    ```bash
    cd ../app
    npm install
    npm run build
    ```

5.  **Load the extension in your browser:**
    *   Open Chrome/Edge and navigate to `chrome://extensions`.
    *   Enable **Developer mode**.
    *   Click **"Load unpacked"**.
    *   Select the `extension/app/dist` directory.

## Next Steps

-   **Debug WASM Loading**: Isolate and fix the silent failure preventing the WASM module from executing in the popup.
-   **Refine UI/UX**: Improve the user interface and provide more detailed feedback.
-   **Expand Threat Intelligence**: Integrate additional data sources or heuristics to improve detection accuracy.
