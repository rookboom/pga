use crate::visualization::{PGAScene, PGAVisualizationApp};
use wasm_bindgen::prelude::*;

/// Initialize the PGA visualization for web
#[wasm_bindgen(start)]
pub fn start() {
    // Set panic hook for better error messages in web console
    console_error_panic_hook::set_once();

    // Create a demo scene
    let scene = PGAScene::demo();

    // Create and configure the Bevy app
    let mut app = PGAVisualizationApp::new();

    // Insert our scene
    app.insert_resource(scene);

    // Run the visualization
    app.run();
}

/// WASM-bindgen exported function for visibility changes
#[wasm_bindgen]
pub fn handle_visibility_change(visible: bool) {
    // Could be used to pause/resume the app when tab is not visible
    web_sys::console::log_1(&format!("Visibility changed: {}", visible).into());
}
