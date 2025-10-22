# Build Script for rust_embed_web

This project includes a simple build script to help you build and run the React frontend and Rust backend.

## Build Script (`build.sh`)

A simple build script that handles both frontend and backend builds:

```bash
# Debug build (default)
./build.sh

# Release build
./build.sh release
```

## Build Process

The build process consists of two main steps:

1. **Frontend Build**: Builds the React application using Vite and outputs to `frontend/dist/`
2. **Backend Build**: Builds the Rust application using Cargo, embedding the frontend assets

## Output Locations

- **Frontend**: `frontend/dist/` (embedded in Rust binary)
- **Backend Debug**: `target/debug/rust_embed_web`
- **Backend Release**: `target/release/rust_embed_web`

## Usage Examples

### Basic Builds

```bash
# Debug build (default)
./build.sh

# Release build
./build.sh release
```

### Running the Application

```bash
# After building, run the server
./target/debug/rust_embed_web      # Debug version
./target/release/rust_embed_web    # Release version
```

### Development Workflow

```bash
# For development with hot reloading:
# Terminal 1: Frontend dev server
cd frontend && npm run dev

# Terminal 2: Backend server
cargo run
```

## Prerequisites

- **Node.js**: Required for frontend build
- **npm**: Required for frontend dependencies
- **Rust/Cargo**: Required for backend build

## Troubleshooting

### Frontend Build Issues

- Ensure Node.js is installed (`node --version`)
- Ensure npm is installed (`npm --version`)
- Install dependencies: `cd frontend && npm install`
- Check for Vite compatibility issues with Node.js version

### Backend Build Issues

- Ensure Rust is installed (`cargo --version`)
- Ensure frontend is built first (creates `frontend/dist/`)
- Check that all dependencies are available

### Clean Build

If you encounter issues, try cleaning and rebuilding:

```bash
# Clean frontend
rm -rf frontend/dist

# Clean Rust target
cargo clean

# Rebuild
./build.sh release
```

## Notes

- The Rust application uses `rust-embed` to serve the frontend from `frontend/dist/`
- The frontend must be built before the backend to ensure assets are available for embedding
- Development mode runs both servers simultaneously for hot reloading
- Production mode serves the embedded frontend from the Rust binary
- The server runs on `http://localhost:8080` by default
