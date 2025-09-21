use bevy::{gizmos::config::GizmoConfigGroup, prelude::*};

use crate::{Direction, Line, Plane, Point};

/// Configuration for the PGA visualization
#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct PGAGizmos;

/// Bevy app builder for PGA visualization
pub struct PGAVisualizationApp;

impl PGAVisualizationApp {
    /// Creates a new Bevy app configured for PGA visualization
    pub fn new() -> App {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "PGA Geometric Algebra Visualization".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .init_gizmo_group::<PGAGizmos>()
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (update_camera_controller, draw_pga_gizmos));

        app
    }

    /// Runs the visualization app
    pub fn run(self) {
        Self::new().run();
    }
}

/// Camera controller component for orbiting around the scene
#[derive(Component)]
pub struct CameraController {
    pub radius: f32,
    pub theta: f32,
    pub phi: f32,
    pub target: Vec3,
    pub sensitivity: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            radius: 5.0,
            theta: 0.0,
            phi: std::f32::consts::PI / 4.0,
            target: Vec3::ZERO,
            sensitivity: 2.0,
        }
    }
}

/// Resource containing PGA objects to visualize
#[derive(Resource, Default)]
pub struct PGAScene {
    pub points: Vec<Point>,
    pub planes: Vec<Plane>,
    pub lines: Vec<Line>,
    pub directions: Vec<Direction>,
}

impl PGAScene {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_point(mut self, point: Point) -> Self {
        self.points.push(point);
        self
    }

    pub fn with_plane(mut self, plane: Plane) -> Self {
        self.planes.push(plane);
        self
    }

    pub fn with_line(mut self, line: Line) -> Self {
        self.lines.push(line);
        self
    }

    pub fn with_direction(mut self, direction: Direction) -> Self {
        self.directions.push(direction);
        self
    }

    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }

    pub fn add_plane(&mut self, plane: Plane) {
        self.planes.push(plane);
    }

    pub fn add_line(&mut self, line: Line) {
        self.lines.push(line);
    }

    pub fn add_direction(&mut self, direction: Direction) {
        self.directions.push(direction);
    }
}

/// Setup the initial scene with camera and lighting
fn setup_scene(mut commands: Commands) {
    // Add a camera with orbit controller
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(5.0, 3.0, 5.0)).looking_at(Vec3::ZERO, Vec3::Y),
        CameraController::default(),
    ));

    // Add basic lighting
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            std::f32::consts::PI / 4.0,
            -std::f32::consts::PI / 4.0,
        )),
    ));

    // Add ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.3,
        affects_lightmapped_meshes: true,
    });

    // Initialize the PGA scene with a point at the origin
    let mut scene = PGAScene::new();
    scene.add_point(Point::new(0.0, 0.0, 0.0));

    commands.insert_resource(scene);
}

/// Simple camera controller for orbiting around the scene
fn update_camera_controller(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut CameraController)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (mut transform, mut controller) in query.iter_mut() {
        let dt = time.delta_secs();

        // Rotate camera around the target
        if input.pressed(KeyCode::ArrowLeft) {
            controller.theta -= controller.sensitivity * dt;
        }
        if input.pressed(KeyCode::ArrowRight) {
            controller.theta += controller.sensitivity * dt;
        }
        if input.pressed(KeyCode::ArrowUp) {
            controller.phi = (controller.phi - controller.sensitivity * dt)
                .clamp(0.1, std::f32::consts::PI - 0.1);
        }
        if input.pressed(KeyCode::ArrowDown) {
            controller.phi = (controller.phi + controller.sensitivity * dt)
                .clamp(0.1, std::f32::consts::PI - 0.1);
        }

        // Zoom in/out
        if input.pressed(KeyCode::Equal) || input.pressed(KeyCode::NumpadAdd) {
            controller.radius = (controller.radius - 5.0 * dt).max(1.0);
        }
        if input.pressed(KeyCode::Minus) {
            controller.radius += 5.0 * dt;
        }

        // Calculate new camera position
        let x = controller.radius * controller.phi.sin() * controller.theta.cos();
        let y = controller.radius * controller.phi.cos();
        let z = controller.radius * controller.phi.sin() * controller.theta.sin();

        let position = controller.target + Vec3::new(x, y, z);
        transform.translation = position;
        transform.look_at(controller.target, Vec3::Y);
    }
}

/// System to draw PGA objects using Bevy's gizmo API
fn draw_pga_gizmos(mut gizmos: Gizmos<PGAGizmos>, scene: Option<Res<PGAScene>>) {
    let Some(scene) = scene else {
        return;
    };

    // Draw coordinate axes
    gizmos.line(Vec3::ZERO, Vec3::X * 2.0, LinearRgba::RED);
    gizmos.line(Vec3::ZERO, Vec3::Y * 2.0, LinearRgba::GREEN);
    gizmos.line(Vec3::ZERO, Vec3::Z * 2.0, LinearRgba::BLUE);

    // Draw points as small spheres
    for point in &scene.points {
        let pos = pga_point_to_vec3(*point);
        gizmos.sphere(pos, 0.1, LinearRgba::new(1.0, 1.0, 0.0, 1.0)); // Yellow

        // Also draw a small cross to make points more visible
        let size = 0.2;
        let yellow = LinearRgba::new(1.0, 1.0, 0.0, 1.0);
        gizmos.line(pos - Vec3::X * size, pos + Vec3::X * size, yellow);
        gizmos.line(pos - Vec3::Y * size, pos + Vec3::Y * size, yellow);
        gizmos.line(pos - Vec3::Z * size, pos + Vec3::Z * size, yellow);
    }

    // Draw directions as arrows from origin
    for direction in &scene.directions {
        let dir = pga_direction_to_vec3(*direction);
        gizmos.arrow(Vec3::ZERO, dir * 2.0, LinearRgba::new(0.0, 1.0, 1.0, 1.0)); // Cyan
    }

    // Draw lines
    for line in &scene.lines {
        draw_pga_line(&mut gizmos, *line, LinearRgba::new(1.0, 0.5, 0.0, 1.0)); // Orange
    }

    // Draw planes as grids
    for plane in &scene.planes {
        draw_pga_plane(&mut gizmos, *plane, LinearRgba::new(0.5, 0.0, 0.5, 1.0)); // Purple
    }
}

/// Convert a PGA Point to a Bevy Vec3
fn pga_point_to_vec3(point: Point) -> Vec3 {
    // Extract coordinates from the PGA point
    // In PGA, a point is represented as x*e032 + y*e013 + z*e021 + e123
    // We need to extract x, y, z coordinates
    let pga = point.0;

    // Check if the point is at infinity (e123 component is zero)
    if pga.mvec[14].abs() < f32::EPSILON {
        return Vec3::ZERO; // Handle points at infinity
    }

    // Extract coordinates by dividing by the e123 component
    let w = pga.mvec[14]; // e123 component
    let x = -pga.mvec[13] / w; // e032 component
    let y = pga.mvec[11] / w; // e021 component  
    let z = -pga.mvec[12] / w; // e013 component

    Vec3::new(x, y, z)
}

/// Convert a PGA Direction to a Bevy Vec3
fn pga_direction_to_vec3(direction: Direction) -> Vec3 {
    let pga = direction.0;

    // Direction vectors are represented as x*e032 + y*e013 + z*e021
    let x = -pga.mvec[13]; // e032 component
    let y = pga.mvec[11]; // e021 component
    let z = -pga.mvec[12]; // e013 component

    Vec3::new(x, y, z)
}

/// Draw a PGA line using gizmos
fn draw_pga_line(gizmos: &mut Gizmos<PGAGizmos>, line: Line, color: LinearRgba) {
    let pga = line.0;

    // Extract direction and moment components
    let dir_x = pga.mvec[9]; // e31
    let dir_y = pga.mvec[10]; // e23  
    let dir_z = pga.mvec[8]; // e12

    let mom_x = pga.mvec[6]; // e02
    let mom_y = pga.mvec[7]; // e03
    let mom_z = pga.mvec[5]; // e01

    let direction = Vec3::new(dir_x, dir_y, dir_z);
    let moment = Vec3::new(mom_x, mom_y, mom_z);

    // If direction is zero, this is an ideal line (line at infinity)
    if direction.length() < f32::EPSILON {
        // Draw as a small indicator for ideal lines
        gizmos.sphere(Vec3::ZERO, 0.05, color);
        return;
    }

    // Find a point on the line using the relationship: point = direction × moment / |direction|²
    let dir_length_sq = direction.length_squared();
    if dir_length_sq > f32::EPSILON {
        let point_on_line = direction.cross(moment) / dir_length_sq;

        // Draw line segment
        let length = 4.0;
        let start = point_on_line - direction.normalize() * length;
        let end = point_on_line + direction.normalize() * length;

        gizmos.line(start, end, color);

        // Draw direction arrow at the point on the line
        gizmos.arrow(
            point_on_line,
            point_on_line + direction.normalize() * 0.5,
            color,
        );
    }
}

/// Draw a PGA plane using gizmos
fn draw_pga_plane(gizmos: &mut Gizmos<PGAGizmos>, plane: Plane, color: LinearRgba) {
    let pga = plane.0;

    // Extract plane equation coefficients: ax + by + cz + d = 0
    let a = pga.mvec[2]; // e1
    let b = pga.mvec[3]; // e2  
    let c = pga.mvec[4]; // e3
    let d = pga.mvec[1]; // e0

    let normal = Vec3::new(a, b, c);

    if normal.length() < f32::EPSILON {
        return; // Invalid plane
    }

    let normal = normal.normalize();

    // Find a point on the plane
    let distance = -d / Vec3::new(a, b, c).length();
    let point_on_plane = normal * distance;

    // Create two orthogonal vectors in the plane
    let up = if normal.abs().dot(Vec3::Y) < 0.9 {
        Vec3::Y
    } else {
        Vec3::X
    };

    let u = normal.cross(up).normalize();
    let v = normal.cross(u);

    // Draw a grid on the plane
    let size = 3.0;
    let divisions = 6;
    let step = size * 2.0 / divisions as f32;

    // Draw grid lines in u direction
    for i in 0..=divisions {
        let offset = -size + i as f32 * step;
        let start = point_on_plane + u * offset - v * size;
        let end = point_on_plane + u * offset + v * size;
        gizmos.line(start, end, color.with_alpha(0.3));
    }

    // Draw grid lines in v direction
    for i in 0..=divisions {
        let offset = -size + i as f32 * step;
        let start = point_on_plane + v * offset - u * size;
        let end = point_on_plane + v * offset + u * size;
        gizmos.line(start, end, color.with_alpha(0.3));
    }

    // Draw normal vector
    gizmos.arrow(point_on_plane, point_on_plane + normal * 1.0, color);
}
