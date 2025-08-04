# Justfile for the Siren project

# Start the development environment (placeholder)
dev:
    @echo "Starting development environment..."

# Build the browser extension (WASM)
build-ext:
    @echo "Building browser extension..."
    cd extension && cargo build --release

# Train the model (placeholder)
train:
    @echo "Starting model training process..."
    python model/train.py
