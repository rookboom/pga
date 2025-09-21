#!/bin/bash

# Build script for local development and testing

set -e

echo "🚀 Building PGA Visualization for Web..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack is not installed. Please install it:"
    echo "   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
    exit 1
fi

# Build the WebAssembly package
echo "📦 Building WebAssembly package..."
wasm-pack build \
    --target web \
    --out-dir web/pkg \
    --features web

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo ""
    echo "🌐 To serve locally:"
    echo "   cd web && python3 -m http.server 8000"
    echo "   Then open: http://localhost:8000"
    echo ""
    echo "📤 To deploy to GitHub Pages:"
    echo "   Push to main/master branch and GitHub Actions will handle deployment"
else
    echo "❌ Build failed!"
    exit 1
fi