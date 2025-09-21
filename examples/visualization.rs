use pga::visualization::{PGAVisualizationApp, demo};

fn main() {
    // Create a demo scene with various PGA objects
    let scene = demo();

    // Create and configure the Bevy app
    let mut app = PGAVisualizationApp::new();

    // Insert our scene
    app.insert_resource(scene);

    // Run the visualization
    app.run();
}
