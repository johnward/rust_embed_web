#!/bin/bash

# Simple build script for rust_embed_web project
# Usage: ./build-simple.sh [debug|release]

set -e

BUILD_TYPE=${1:-debug}

echo "🔨 Building rust_embed_web in $BUILD_TYPE mode..."

# Build frontend
echo "📦 Building React frontend..."
cd frontend
npm run build
cd ..

# Build Rust backend
echo "🦀 Building Rust backend..."
if [[ "$BUILD_TYPE" == "release" ]]; then
    cargo build --release
    echo "✅ Release build complete! Binary: target/release/rust_embed_web"
else
    cargo build
    echo "✅ Debug build complete! Binary: target/debug/rust_embed_web"
fi

echo "🎉 Build finished successfully!"
