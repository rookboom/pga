//! PGA Visualization module
//!
//! This module provides 3D visualization capabilities for Projective Geometric Algebra objects
//! using the Bevy game engine.

use bevy::{
    gizmos::config::GizmoConfigGroup,
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
};
use smooth_bevy_cameras::{
    LookTransformPlugin,
    controllers::orbit::{
        ControlEvent, OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin,
    },
};

use crate::{Direction, Line, Plane, Point};

/// Configuration for the PGA visualization
#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct PGAGizmos;

/// Available scene types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SceneType {
    Demo,
    Points,
    Lines,
    Planes,
    Mixed,
}

impl SceneType {
    pub fn all() -> Vec<SceneType> {
        vec![
            SceneType::Demo,
            SceneType::Points,
            SceneType::Lines,
            SceneType::Planes,
            SceneType::Mixed,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            SceneType::Demo => "Demo Scene",
            SceneType::Points => "Points Only",
            SceneType::Lines => "Lines Only",
            SceneType::Planes => "Planes Only",
            SceneType::Mixed => "Mixed Objects",
        }
    }

    pub fn next(&self) -> SceneType {
        match self {
            SceneType::Demo => SceneType::Points,
            SceneType::Points => SceneType::Lines,
            SceneType::Lines => SceneType::Planes,
            SceneType::Planes => SceneType::Mixed,
            SceneType::Mixed => SceneType::Demo,
        }
    }

    pub fn previous(&self) -> SceneType {
        match self {
            SceneType::Demo => SceneType::Mixed,
            SceneType::Points => SceneType::Demo,
            SceneType::Lines => SceneType::Points,
            SceneType::Planes => SceneType::Lines,
            SceneType::Mixed => SceneType::Planes,
        }
    }
}

/// Resource for tracking current scene selection
#[derive(Resource)]
pub struct SceneSelection {
    pub current: SceneType,
}

impl Default for SceneSelection {
    fn default() -> Self {
        Self {
            current: SceneType::Demo,
        }
    }
}

/// Component to mark the scene name text UI element
#[derive(Component)]
struct SceneNameText;

/// Resource containing PGA objects to visualize
#[derive(Resource, Default)]
pub struct PGAScene {
    pub points: Vec<Point>,
    pub planes: Vec<Plane>,
    pub lines: Vec<Line>,
    pub directions: Vec<Direction>,
}

pub fn demo() -> PGAScene {
    create_scene(SceneType::Demo)
}

impl PGAScene {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a scene based on the scene type
    pub fn from_type(scene_type: SceneType) -> Self {
        create_scene(scene_type)
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
}

/// Create different scenes based on scene type
fn create_scene(scene_type: SceneType) -> PGAScene {
    match scene_type {
        SceneType::Demo => PGAScene::new()
            .with_point(Point::new(0.0, 0.0, 0.0)) // Origin
            .with_point(Point::new(1.0, 1.0, 1.0)) // Corner point
            .with_point(Point::new(1.0, 0.5, 0.5)) // Another point
            .with_direction(Direction::new(1.0, 0.0, 0.0)), // X direction

        SceneType::Points => PGAScene::new()
            .with_point(Point::new(0.0, 0.0, 0.0)) // Origin
            .with_point(Point::new(1.0, 0.0, 0.0)) // X axis
            .with_point(Point::new(0.0, 1.0, 0.0)) // Y axis
            .with_point(Point::new(0.0, 0.0, 1.0)) // Z axis
            .with_point(Point::new(1.0, 1.0, 1.0)) // Corner
            .with_point(Point::new(-1.0, -1.0, -1.0)), // Opposite corner

        SceneType::Lines => PGAScene::new()
            .with_line(Line::through_origin(1.0, 0.0, 0.0)) // X axis line
            .with_line(Line::through_origin(0.0, 1.0, 0.0)) // Y axis line
            .with_line(Line::through_origin(0.0, 0.0, 1.0)) // Z axis line
            .with_line(Line::through_origin(1.0, 1.0, 0.0)), // Diagonal line

        SceneType::Planes => PGAScene::new()
            .with_plane(Plane::new(1.0, 0.0, 0.0, 1.0)) // X = -1 plane
            .with_plane(Plane::new(0.0, 1.0, 0.0, 1.0)) // Y = -1 plane
            .with_plane(Plane::new(0.0, 0.0, 1.0, 1.0)), // Z = -1 plane

        SceneType::Mixed => PGAScene::new()
            .with_point(Point::new(0.0, 0.0, 0.0)) // Origin
            .with_direction(Direction::new(1.0, 0.0, 0.0)) // X direction
            .with_direction(Direction::new(0.0, 1.0, 0.0)) // Y direction
            .with_line(Line::through_origin(1.0, 1.0, 0.0)) // Diagonal line
            .with_plane(Plane::new(1.0, 0.0, 0.0, 1.0)), // X = -1 plane
    }
}

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
        .add_plugins(LookTransformPlugin)
        .add_plugins(OrbitCameraPlugin {
            override_input_system: true,
        })
        .init_gizmo_group::<PGAGizmos>()
        .init_resource::<SceneSelection>()
        .add_systems(Startup, (setup_scene, setup_ui))
        .add_systems(Update, draw_pga_gizmos)
        .add_systems(Update, input_map)
        .add_systems(Update, handle_scene_reload)
        .add_systems(Update, scene_selection_input)
        .add_systems(Update, update_scene_ui);

        app
    }

    /// Runs the visualization app
    pub fn run(self) {
        Self::new().run();
    }
}

/// Setup the initial scene with camera and lighting
fn setup_scene(mut commands: Commands) {
    commands
        .spawn(Camera3d::default())
        .insert(OrbitCameraBundle::new(
            OrbitCameraController::default(),
            Vec3::new(3.0, 3.0, 3.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ));

    commands.insert_resource(demo());

    // Print controls
    info!("=== PGA Visualization Controls ===");
    info!("Scene Selection:");
    info!("  1-5: Select scene type directly");
    info!("  [ / ]: Previous scene");
    info!("  ] / →: Next scene");
    info!("Camera Controls:");
    info!("  Left mouse + drag: Orbit camera");
    info!("  Mouse wheel: Zoom in/out");
    info!("Other:");
    info!("  Enter: Reload current scene");
}

/// System to handle scene reloading when Enter is pressed
fn handle_scene_reload(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut scene: ResMut<PGAScene>,
    scene_selection: Res<SceneSelection>,
) {
    if keyboard.just_pressed(KeyCode::Enter) {
        // Clear the existing scene and replace with current selection
        *scene = create_scene(scene_selection.current);
        info!("Scene reloaded!");
    }
}

/// Setup UI elements
fn setup_ui(mut commands: Commands) {
    // Create UI text for scene name in top-left corner
    commands.spawn((
        Text::new("Demo Scene\nPress 1-5 or ←/→ to change"),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            ..default()
        },
        SceneNameText,
    ));
}

/// Update the scene name text when scene changes
fn update_scene_ui(
    scene_selection: Res<SceneSelection>,
    mut query: Query<&mut Text, With<SceneNameText>>,
) {
    if scene_selection.is_changed() {
        for mut text in query.iter_mut() {
            **text = format!(
                "{}\nPress 1-5 or ←/→ to change",
                scene_selection.current.name()
            );
        }
    }
}

/// System for keyboard scene selection shortcuts
fn scene_selection_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut scene_selection: ResMut<SceneSelection>,
    mut scene: ResMut<PGAScene>,
) {
    let mut changed = false;

    // Use number keys 1-5 to select scene types
    if keyboard.just_pressed(KeyCode::Digit1) {
        scene_selection.current = SceneType::Demo;
        changed = true;
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        scene_selection.current = SceneType::Points;
        changed = true;
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        scene_selection.current = SceneType::Lines;
        changed = true;
    } else if keyboard.just_pressed(KeyCode::Digit4) {
        scene_selection.current = SceneType::Planes;
        changed = true;
    } else if keyboard.just_pressed(KeyCode::Digit5) {
        scene_selection.current = SceneType::Mixed;
        changed = true;
    }
    // Use arrow keys to cycle through scenes
    else if keyboard.just_pressed(KeyCode::ArrowRight)
        || keyboard.just_pressed(KeyCode::BracketRight)
    {
        scene_selection.current = scene_selection.current.next();
        changed = true;
    } else if keyboard.just_pressed(KeyCode::ArrowLeft)
        || keyboard.just_pressed(KeyCode::BracketLeft)
    {
        scene_selection.current = scene_selection.current.previous();
        changed = true;
    }

    if changed {
        *scene = create_scene(scene_selection.current);
        info!(
            "Scene changed to: {} ({})",
            scene_selection.current.name(),
            scene_selection.current as u8 + 1
        );
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
        gizmos.sphere(pos, 0.01, LinearRgba::new(1.0, 1.0, 0.0, 1.0)); // Yellow
    }

    // Draw directions as arrows from origin
    for direction in &scene.directions {
        let dir = pga_direction_to_vec3(*direction);
        gizmos.arrow(Vec3::ZERO, dir * 2.0, LinearRgba::new(0.0, 1.0, 1.0, 1.0));
        // Cyan
    }

    // Draw lines
    for line in &scene.lines {
        draw_pga_line(&mut gizmos, *line, LinearRgba::new(1.0, 0.5, 0.0, 1.0)); // Orange
    }

    // Draw planes as grids
    for plane in &scene.planes {
        draw_pga_plane(&mut gizmos, *plane, LinearRgba::new(0.5, 0.0, 0.5, 1.0));
        // Purple
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
    let x = pga.mvec[13] / w; // e032 component
    let y = pga.mvec[11] / w; // e021 component
    let z = pga.mvec[12] / w; // e013 component

    Vec3::new(x, y, z)
}

/// Convert a PGA Direction to a Bevy Vec3
fn pga_direction_to_vec3(direction: Direction) -> Vec3 {
    let pga = direction.0;

    // Direction vectors are represented as x*e032 + y*e013 + z*e021
    let x = pga.mvec[13]; // e032 component
    let y = pga.mvec[11]; // e021 component
    let z = pga.mvec[12]; // e013 component

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

pub fn input_map(
    mut events: EventWriter<ControlEvent>,
    mut mouse_wheel_reader: EventReader<MouseWheel>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    controllers: Query<&OrbitCameraController>,
) {
    // Can only control one camera at a time.
    let controller = if let Some(controller) = controllers.iter().find(|c| c.enabled) {
        controller
    } else {
        return;
    };
    let OrbitCameraController {
        mouse_rotate_sensitivity,
        mouse_wheel_zoom_sensitivity,
        pixels_per_line,
        ..
    } = *controller;

    let mut cursor_delta = Vec2::ZERO;
    for event in mouse_motion_events.read() {
        cursor_delta += event.delta;
    }

    if mouse_buttons.pressed(MouseButton::Left) {
        events.write(ControlEvent::Orbit(mouse_rotate_sensitivity * cursor_delta));
    }

    let mut scalar = 1.0;
    for event in mouse_wheel_reader.read() {
        // scale the event magnitude per pixel or per line
        let scroll_amount = match event.unit {
            MouseScrollUnit::Line => event.y,
            MouseScrollUnit::Pixel => event.y / pixels_per_line,
        };
        scalar *= 1.0 - scroll_amount * mouse_wheel_zoom_sensitivity;
    }
    events.write(ControlEvent::Zoom(scalar));
}
