# Siren: Real-Time Phishing Detector

Siren is a project to build a real-time phishing detector that runs directly in the browser. It utilizes an optimized machine learning model integrated into a browser extension built with Rust and WebAssembly (WASM) for maximum performance.

## Project Status

**Phase 2: Model Training & Optimization - COMPLETE**

The project has successfully completed the model training and optimization phase. The next step is to begin the development of the browser extension.

## Tech Stack

*   **Machine Learning**:
    *   Python
    *   PyTorch & Hugging Face `transformers` for the DistilBERT model.
    *   Hugging Face `optimum` for robust ONNX exporting.
*   **Versioning**:
    *   **Git & GitHub**: For source code.
    *   **DVC**: For data and model versioning.
    *   **DagsHub**: As a remote storage backend for DVC (via S3).
*   **Browser Extension (Phase 3)**:
    *   Rust
    *   WebAssembly (WASM)

## Current Progress

-   [x] **Phase 1: Project Initialization & Foundation**
    -   Project structure created.
    -   Git repository initialized.
    -   DVC successfully set up and connected to DagsHub for remote storage.

-   [x] **Phase 2: Model Training & Optimization**
    -   A baseline DistilBERT model was successfully fine-tuned for phishing URL classification.
    -   The model was successfully quantized (dynamic quantization) to reduce its size and accelerate inference.
    -   The final model was exported to the **ONNX format (`model_quantized.onnx`)**, making it ready for use in non-Python environments.
    -   All model files (both PyTorch and ONNX) are now tracked with DVC.

## Next Steps

1.  **Begin Phase 3: Browser Extension**: Build the foundational framework for the browser extension using Rust.
2.  **Model Integration**: Integrate the `model_quantized.onnx` model into the extension to perform local inference within the user's browser.

---
*This documentation will be updated as the project progresses.*
