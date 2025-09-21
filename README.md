# 🔮 Projective Geometric Algebra (PGA)

A Rust library for 3D Projective Geometric Algebra with beautiful web visualization.

This project builds on the generated implementation of PGA R301 from [Bivector.net](https://bivector.net), creating type-safe operators for geometric objects in 3D plane-based geometric algebra: **planes**, **lines**, and **points**.

## ✨ Features

- 🧮 **Type-safe PGA operations** for points, planes, lines, and directions
- 🎮 **Interactive 3D visualization** using Bevy Engine
- 🌐 **Web deployment** via WebAssembly (see it live!)
- 📚 **Comprehensive API** for geometric transformations
- 🚀 **Zero-cost abstractions** over the underlying algebra

## 🌐 Live Demo

**[View the interactive visualization here!](https://rookboom.github.io/pga/)**

The web version shows:
- Points as yellow spheres with cross markers
- Direction vectors as cyan arrows  
- Lines as orange segments with direction indicators
- Planes as purple grids with normal vectors
- RGB coordinate axes for reference

### Controls:
- **Arrow Keys**: Orbit camera around the scene
- **+/-**: Zoom in and out
- **Mouse**: Look around

## 🚀 Quick Start

### Desktop Visualization

```bash
# Run the desktop visualization
cargo run --example visualization --features visualization
```

### Web Development

```bash
# Install wasm-pack (one-time setup)
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build and serve the web version
./build_web.sh
cd web && python3 -m http.server 8000
# Open http://localhost:8000
```

### Library Usage

```rust
use pga::{Point, Plane, Line, Direction};

// Create geometric objects
let origin = Point::new(0.0, 0.0, 0.0);
let unit_plane = Plane::new(1.0, 0.0, 0.0, 0.0);  // yz-plane
let line = Line::through_origin(1.0, 1.0, 0.0);   // diagonal line
let direction = Direction::new(0.0, 1.0, 0.0);    // y-direction

// Use with visualization
#[cfg(feature = "visualization")]
{
    use pga::visualization::{PGAVisualizationApp, PGAScene};
    
    let scene = PGAScene::new()
        .with_point(origin)
        .with_plane(unit_plane)
        .with_line(line)
        .with_direction(direction);
    
    let mut app = PGAVisualizationApp::new();
    app.insert_resource(scene);
    app.run();
}
```

## 📦 Features

The crate supports optional features:

```toml
[dependencies]
pga = { version = "0.1", features = ["visualization"] }

# For web deployment
pga = { version = "0.1", features = ["web"] }
```

- `visualization`: Enables Bevy-based 3D visualization
- `web`: Enables WebAssembly compilation with WASM bindings

## 🛠️ Development

### Prerequisites

- Rust 1.70+ with `wasm32-unknown-unknown` target
- For web development: `wasm-pack`

### Building

```bash
# Desktop library and visualization
cargo build --features visualization

# WebAssembly for web
wasm-pack build --target web --features web
```

### Testing

```bash
# Run tests
cargo test

# Test visualization  
cargo run --example visualization --features visualization
```

## 🌐 Web Deployment

### GitHub Pages (Automatic)

1. Enable GitHub Pages in repository settings with "GitHub Actions" source
2. Push to main/master branch
3. The site will be automatically built and deployed

### Manual Deployment

```bash
# Build the web version
./build_web.sh

# Deploy the web/ directory to your hosting provider
```

## 🔧 Technical Details

### Architecture

- **Core**: Pure Rust PGA implementation with type-safe wrappers
- **Visualization**: Bevy Engine with custom gizmo rendering
- **Web**: WebAssembly compilation via `wasm-pack`
- **Deployment**: GitHub Actions with Pages integration

### Geometric Objects

- **Points**: Represented as normalized homogeneous coordinates
- **Planes**: Defined by normal vector and distance from origin  
- **Lines**: Dual representation with direction and moment vectors
- **Directions**: Ideal points representing infinite directions

### Performance

- Native: Zero-cost abstractions over SIMD-optimized algebra
- Web: Hardware-accelerated WebGL rendering with WASM

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes and add tests
4. Test both desktop and web versions
5. Submit a pull request

### Development Workflow

```bash
# Test changes locally
cargo test
cargo run --example visualization --features visualization

# Test web version
./build_web.sh && cd web && python3 -m http.server 8000
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Bivector.net](https://bivector.net) for the foundational PGA implementation
- [Bevy Engine](https://bevyengine.org/) for the visualization framework
- The Rust community for excellent WebAssembly tooling

---

**[🌐 Try the live demo!](https://rookboom.github.io/pga/)**
