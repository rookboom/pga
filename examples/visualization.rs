use pga::visualization::{PGAScene, PGAVisualizationApp};
use pga::{Direction, Line, Plane, Point};

fn main() {
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
