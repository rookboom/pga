# 🌐 PGA Web Visualization

This directory contains the web version of the PGA (Projective Geometric Algebra) visualization built with Bevy and WebAssembly.

## 🚀 Quick Start

### Local Development

1. **Install wasm-pack** (if not already installed):
   ```bash
   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
   ```

2. **Build the web version**:
   ```bash
   ./build_web.sh
   ```

3. **Serve locally**:
   ```bash
   cd web && python3 -m http.server 8000
   ```
   
   Then open: http://localhost:8000

### Using npm (alternative)

```bash
npm run dev
```

## 📤 GitHub Pages Deployment

The visualization automatically deploys to GitHub Pages when you push to the main/master branch.

### Setup GitHub Pages

1. Go to your repository settings
2. Navigate to "Pages" section
3. Set source to "GitHub Actions"
4. Push to main/master branch

The site will be available at: `https://<username>.github.io/<repository-name>/`

## 🎮 Controls

- **Arrow Keys**: Orbit the camera around the scene
- **+/-**: Zoom in and out
- **Mouse**: Look around (if mouse controls are enabled)

## 🎨 What You'll See

- **Red/Green/Blue Lines**: X/Y/Z coordinate axes
- **Yellow Spheres**: Points in 3D space
- **Cyan Arrows**: Direction vectors
- **Orange Lines**: PGA lines with direction indicators
- **Purple Grids**: Planes with normal vectors

## 🛠️ Technical Details

- Built with [Bevy Engine](https://bevyengine.org/) 0.16.1
- Compiled to WebAssembly using [wasm-pack](https://rustwasm.github.io/wasm-pack/)
- Uses WebGL for hardware-accelerated 3D rendering
- Optimized for modern browsers

## 📁 File Structure

```
web/
├── index.html          # Main HTML file
├── pkg/               # Generated WASM files (after build)
│   ├── web_visualization.js
│   ├── web_visualization_bg.wasm
│   └── ...
└── README.md          # This file
```

## 🔧 Troubleshooting

### Build Issues

- Ensure Rust is up to date: `rustup update`
- Ensure wasm32 target is installed: `rustup target add wasm32-unknown-unknown`
- Clear cargo cache: `cargo clean`

### Runtime Issues

- Check browser console for JavaScript errors
- Ensure WebGL is supported and enabled
- Try in a different browser (Chrome, Firefox, Safari, Edge)

### Performance

- The visualization is GPU-accelerated but may be slower than native
- Close other tabs/applications for better performance
- Consider reducing the number of objects in complex scenes

## 🤝 Contributing

To contribute to the web version:

1. Make changes to the Rust code in `src/` or `examples/`
2. Test locally with `./build_web.sh && cd web && python3 -m http.server 8000`
3. Commit and push - GitHub Actions will handle deployment

## 📄 License

Same as the main PGA crate - see repository root for license details.