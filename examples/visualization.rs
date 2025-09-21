use pga::visualization::{PGAScene, PGAVisualizationApp};

fn main() {
    // Create a demo scene with various PGA objects
    let scene = PGAScene::demo();

    // Create and configure the Bevy app
    let mut app = PGAVisualizationApp::new();

    // Insert our scene
    app.insert_resource(scene);

    // Run the visualization
    app.run();
}
