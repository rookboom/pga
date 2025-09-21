use crate::visualization::{PGAScene, PGAVisualizationApp};
use crate::{Direction, Line, Plane, Point};
use wasm_bindgen::prelude::*;

/// Initialize the PGA visualization for web
#[wasm_bindgen(start)]
pub fn start() {
    // Set panic hook for better error messages in web console
    console_error_panic_hook::set_once();

    // Create a scene with various PGA objects
    let scene = PGAScene::new()
        .with_point(Point::new(0.0, 0.0, 0.0)) // Origin
        .with_point(Point::new(1.0, 1.0, 1.0)) // Corner point
        .with_point(Point::new(-1.0, 0.5, 0.5)) // Another point
        .with_direction(Direction::new(1.0, 0.0, 0.0)) // X direction
        .with_direction(Direction::new(0.0, 1.0, 0.0)) // Y direction
        .with_line(Line::through_origin(1.0, 1.0, 0.0)) // Line through origin
        .with_plane(Plane::new(1.0, 0.0, 0.0, 1.0)); // Plane x = -1

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
