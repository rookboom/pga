//! PGA Visualization module
//!
//! This module provides 3D visualization capabilities for Projective Geometric Algebra objects
//! using the Bevy game engine.

use bevy::{
    gizmos::config::GizmoConfigGroup,
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
};
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};
use smooth_bevy_cameras::{
    LookTransformPlugin,
    controllers::orbit::{
        ControlEvent, OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin,
    },
};

use crate::{Direction, Line, Plane, Point, pga3d::ZeroOr};

/// Configuration for the PGA visualization
#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct PGAGizmos;

type PGASceneBuilder = Box<fn(points: &[Point]) -> PGAScene>;
#[derive(Resource)]
pub struct SceneLibrary {
    pub scenes: Vec<PGAScene>,
    current_scene_index: usize,
}

#[derive(Resource)]
pub struct SceneBuilders {
    pub scene_builders: Vec<PGASceneBuilder>,
}

#[derive(Event)]
pub struct SceneChangedEvent;

#[derive(Event)]
pub struct PointsChangedEvent;

impl SceneLibrary {
    pub fn new() -> Self {
        let scenes = build_scenes();
        Self {
            scenes,
            current_scene_index: 0,
        }
    }

    pub fn current(&self) -> &PGAScene {
        &self.scenes[self.current_scene_index]
    }

    pub fn current_mut(&mut self) -> &mut PGAScene {
        &mut self.scenes[self.current_scene_index]
    }

    pub fn next_scene(&mut self) -> &PGAScene {
        self.current_scene_index = (self.current_scene_index + 1) % self.scenes.len();
        self.current()
    }

    pub fn prev_scene(&mut self) -> &PGAScene {
        if self.current_scene_index == 0 {
            self.current_scene_index = self.scenes.len() - 1;
        } else {
            self.current_scene_index -= 1;
        }
        self.current()
    }

    pub fn len(&self) -> usize {
        self.scenes.len()
    }
}

/// Component to mark the scene name text UI element
#[derive(Component)]
struct SceneNameText;

/// Component to mark plane mesh entities
#[derive(Component)]
struct PlaneMesh {
    plane_index: usize,
}

#[derive(Component)]
struct Label((usize, GeometricObject));

enum GeometricObject {
    Point,
    Line,
    Plane,
    Direction,
}

/// Resource containing PGA objects to visualize
#[derive(Clone)]
pub struct PGAScene {
    pub name: &'static str,
    pub points: Vec<(Point, LinearRgba)>,
    pub planes: Vec<(Plane, LinearRgba)>,
    pub lines: Vec<(Line, LinearRgba)>,
    pub directions: Vec<(Direction, LinearRgba)>,
}

impl Default for PGAScene {
    fn default() -> Self {
        Self {
            name: Self::EMPTY_SCENE,
            points: Vec::new(),
            planes: Vec::new(),
            lines: Vec::new(),
            directions: Vec::new(),
        }
    }
}

impl PGAScene {
    const EMPTY_SCENE: &str = "Empty Scene";
    const TWO_POINTS_JOIN_IN_A_LINE: &str = "Two points join in a line: L1 = P0 V P1";
    const THREE_POINTS_JOIN_IN_A_PLANE: &str = "Three points join in a plane: P0 = P0 V P1 V P2";
    const LINE_AND_POINT_JOIN_IN_A_PLANE: &str = "A line and a point join in a plane: P0 = L0 V P2";
    const THREE_PLANES_MEET_IN_A_POINT: &str = "Three planes meet in a point: P9 = P1 ^ P2 ^ P3";
    const YELLOW: LinearRgba = LinearRgba::rgb(1.0, 1.0, 0.0);
    const RED: LinearRgba = LinearRgba::rgb(1.0, 0.0, 0.0);
    const GREEN: LinearRgba = LinearRgba::rgb(0.0, 1.0, 0.0);
    const BLUE: LinearRgba = LinearRgba::rgb(0.0, 0.0, 1.0);
    const MAGENTA: LinearRgba = LinearRgba::rgb(1.0, 0.0, 1.0);
    const CYAN: LinearRgba = LinearRgba::rgb(0.0, 1.0, 1.0);
    const ORANGE: LinearRgba = LinearRgba::rgb(1.0, 0.5, 0.0);
    const WHITE: LinearRgba = LinearRgba::rgb(1.0, 1.0, 1.0);

    pub fn new(name: &'static str, points: &[&Point]) -> Self {
        let mut scene = Self {
            name,
            points: points
                .iter()
                .map(|&p| (p.clone(), PGAScene::WHITE))
                .collect(),
            planes: Vec::new(),
            lines: Vec::new(),
            directions: Vec::new(),
        };

        scene.rebuild();
        scene
    }

    pub fn with_point(&mut self, point: ZeroOr<Point>, color: LinearRgba) {
        if let Some(point) = point.value() {
            self.points.push((point, color));
        }
    }

    pub fn with_plane(&mut self, plane: ZeroOr<Plane>, color: LinearRgba) {
        if let Some(plane) = plane.value() {
            self.planes.push((plane, color));
        }
    }

    pub fn with_line(&mut self, line: ZeroOr<Line>, color: LinearRgba) {
        if let Some(line) = line.value() {
            self.lines.push((line, color));
        }
    }

    pub fn with_direction(mut self, direction: ZeroOr<Direction>, color: LinearRgba) {
        if let Some(direction) = direction.value() {
            self.directions.push((direction, color));
        }
    }

    pub fn point(&self, index: usize) -> &Point {
        &self.points[index].0
    }

    pub fn rebuild(&mut self) {
        self.lines.clear();
        self.planes.clear();
        self.directions.clear();
        match self.name {
            Self::TWO_POINTS_JOIN_IN_A_LINE => {
                self.with_line(self.point(0) & self.point(1), PGAScene::ORANGE);
            }
            Self::THREE_POINTS_JOIN_IN_A_PLANE => {
                self.with_plane(
                    self.point(0) & self.point(1) & self.point(2),
                    PGAScene::CYAN,
                );
            }
            Self::LINE_AND_POINT_JOIN_IN_A_PLANE => {
                self.with_line(self.point(0) & self.point(1), PGAScene::ORANGE);
                self.with_plane(
                    self.point(0) & self.point(1) & self.point(2),
                    PGAScene::CYAN,
                );
            }
            Self::THREE_PLANES_MEET_IN_A_POINT => {
                let plane0 = self.point(0) & self.point(1) & self.point(2);
                let plane1 = self.point(3) & self.point(4) & self.point(5);
                let plane2 = self.point(6) & self.point(7) & self.point(8);
                while self.points.len() > 9 {
                    self.points.pop();
                }
                self.with_point(&plane0 ^ &plane1 ^ &plane2, PGAScene::GREEN);
                self.with_plane(plane0, PGAScene::MAGENTA);
                self.with_plane(plane1, PGAScene::ORANGE);
                self.with_plane(plane2, PGAScene::CYAN);
            }
            _ => {}
        }
    }
}

fn build_scenes() -> Vec<PGAScene> {
    let p0 = &Point::new(1.0, 0.0, 0.0);
    let p1 = &Point::new(0.0, 1.0, 0.0);
    let p2 = &Point::new(0.0, 0.0, 1.0);

    let scenes = vec![
        PGAScene::default(),
        PGAScene::new(PGAScene::TWO_POINTS_JOIN_IN_A_LINE, &[p0, p1]),
        PGAScene::new(PGAScene::THREE_POINTS_JOIN_IN_A_PLANE, &[p0, p1, p2]),
        PGAScene::new(PGAScene::LINE_AND_POINT_JOIN_IN_A_PLANE, &[p0, p1, p2]),
        PGAScene::new(
            PGAScene::THREE_PLANES_MEET_IN_A_POINT,
            &[
                &Point::new(1.0, 1.0, 0.0),
                &Point::new(1.0, 0.0, 2.0),
                &Point::new(1.0, 2.0, 0.0),
                &Point::new(0.0, 2.0, 1.0),
                &Point::new(0.0, 2.0, 3.0),
                &Point::new(3.0, 2.0, 0.0),
                &Point::new(0.0, 1.0, 1.0),
                &Point::new(0.0, 3.0, 1.0),
                &Point::new(3.0, 0.0, 1.0),
            ],
        ),
    ];

    scenes
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
                resolution: (1280.0, 900.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(LookTransformPlugin)
        .add_plugins(OrbitCameraPlugin {
            override_input_system: true,
        })
        .add_plugins(EguiPlugin::default())
        .add_event::<SceneChangedEvent>()
        .add_event::<PointsChangedEvent>()
        .init_gizmo_group::<PGAGizmos>()
        .insert_resource(SceneLibrary::new())
        .add_systems(Startup, (setup_scene, setup_ui))
        .add_systems(Update, draw_pga_gizmos)
        .add_systems(Update, spawn_plane_meshes)
        .add_systems(Update, input_map)
        .add_systems(Update, scene_selection_input)
        .add_systems(Update, update_scene_ui)
        .add_systems(Update, update_point_labels)
        .add_systems(PostUpdate, update_label_positions)
        .add_systems(EguiPrimaryContextPass, coordinate_editor_ui);

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

    let create_label = |index, kind| {
        let text = match kind {
            GeometricObject::Point => format!("P{}", index),
            GeometricObject::Line => format!("L{}", index),
            GeometricObject::Plane => format!("p{}", index),
            GeometricObject::Direction => format!("D{}", index),
        };
        (
            Text::new(text),
            TextColor(Color::WHITE),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                ..default()
            },
            ZIndex(1000),
            Label((index, kind)),
            Visibility::Hidden,
        )
    };
    for i in 0..10 {
        commands.spawn(create_label(i, GeometricObject::Point));
        commands.spawn(create_label(i, GeometricObject::Line));
        commands.spawn(create_label(i, GeometricObject::Plane));
        commands.spawn(create_label(i, GeometricObject::Direction));
    }
}

/// Setup UI elements
fn setup_ui(mut commands: Commands, windows: Query<&Window>) {
    let Ok(window) = windows.single() else {
        return;
    };
    // Create UI text for scene name in top-left corner
    commands.spawn((
        Text::new("Left Mouse Down to orbit. Scroll to zoom. Press arrows to change scene."),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(window.height() - 30.0),
            ..default()
        },
        SceneNameText,
    ));
}

/// Update the scene name text when scene changes
fn update_scene_ui(
    mut scenes: ResMut<SceneLibrary>,
    mut on_scene_changed: EventReader<SceneChangedEvent>,
    mut query: Query<&mut Text, With<SceneNameText>>,
) {
    if on_scene_changed.read().next().is_none() {
        return; // No scene change, no need to update UI
    }

    for mut text in query.iter_mut() {
        let current_scene = scenes.current_mut();
        **text = current_scene.name.to_string();
    }
}

/// System for keyboard scene selection
fn scene_selection_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut scene_library: ResMut<SceneLibrary>,
    mut notify_scene_changed: EventWriter<SceneChangedEvent>,
) {
    if keyboard.just_pressed(KeyCode::ArrowRight) || keyboard.just_pressed(KeyCode::BracketRight) {
        scene_library.next_scene();
        notify_scene_changed.write(SceneChangedEvent);
    } else if keyboard.just_pressed(KeyCode::ArrowLeft)
        || keyboard.just_pressed(KeyCode::BracketLeft)
    {
        scene_library.prev_scene();
        notify_scene_changed.write(SceneChangedEvent);
    }
}

/// System to draw PGA objects using Bevy's gizmo API
fn draw_pga_gizmos(mut gizmos: Gizmos<PGAGizmos>, scenes: Res<SceneLibrary>) {
    let scene = scenes.current();

    // Draw coordinate axes
    gizmos.line(Vec3::ZERO, Vec3::X * 2.0, LinearRgba::RED);
    gizmos.line(Vec3::ZERO, Vec3::Y * 2.0, LinearRgba::GREEN);
    gizmos.line(Vec3::ZERO, Vec3::Z * 2.0, LinearRgba::BLUE);

    // Draw points as small spheres
    for (point, color) in &scene.points {
        let pos = pga_point_to_vec3(point);
        gizmos.sphere(pos, 0.01, *color);
    }

    // Draw directions as arrows from origin
    for (direction, color) in &scene.directions {
        let dir = pga_direction_to_vec3(direction);
        gizmos.arrow(Vec3::ZERO, dir * 2.0, *color);
    }

    // Draw lines
    for (line, color) in &scene.lines {
        draw_pga_line(&mut gizmos, line, *color);
    }

    // Note: Planes are now drawn as meshes in the spawn_plane_meshes system
}

/// Convert a PGA Point to a Bevy Vec3
fn pga_point_to_vec3(point: &Point) -> Vec3 {
    // Extract coordinates from the PGA point
    // In PGA, a point is represented as x*e032 + y*e013 + z*e021 + e123
    // We need to extract x, y, z coordinates
    let pga = &point.0;

    // Check if the point is at infinity (e123 component is zero)
    if pga.mvec[14].abs() < f32::EPSILON {
        return Vec3::ZERO; // Handle points at infinity
    }

    // Extract coordinates by dividing by the e123 component
    let w = pga.mvec[14]; // e123 component
    let x = pga.mvec[13] / w; // e032 component
    let y = pga.mvec[12] / w; // e013 component  
    let z = pga.mvec[11] / w; // e021 component

    Vec3::new(x, y, z)
}

fn pga_point_on_plane(plane: &Plane) -> Vec3 {
    let pga = &plane.0;

    // Extract plane equation coefficients: ax + by + cz + d = 0
    let a = pga.mvec[2]; // e1
    let b = pga.mvec[3]; // e2
    let c = pga.mvec[4]; // e3
    let d = pga.mvec[1]; // e0

    let normal = Vec3::new(a, b, c);

    if normal.length() < f32::EPSILON {
        return Vec3::ZERO; // Invalid plane
    }

    let normal = normal.normalize();

    // Find a point on the plane
    let distance = -d / Vec3::new(a, b, c).length();
    normal * distance
}

fn pga_point_on_line(line: &Line) -> Vec3 {
    let pga = &line.0;

    // Extract direction and moment components
    let dir_x = pga.mvec[10]; // e31
    let dir_y = pga.mvec[9]; // e23
    let dir_z = pga.mvec[8]; // e12

    let mom_x = pga.mvec[5]; // e02
    let mom_y = pga.mvec[6]; // e03
    let mom_z = pga.mvec[7]; // e01

    let direction = Vec3::new(dir_x, dir_y, dir_z);
    let moment = Vec3::new(mom_x, mom_y, mom_z);

    // If direction is zero, this is an ideal line (line at infinity)
    if direction.length() < f32::EPSILON {
        return Vec3::ZERO; // Ideal line, return origin as placeholder
    }

    // Find a point on the line using the relationship: point = direction × moment / |direction|²
    let dir_length_sq = direction.length_squared();
    if dir_length_sq > f32::EPSILON {
        direction.cross(moment) / dir_length_sq
    } else {
        Vec3::ZERO
    }
}

/// Convert a PGA Direction to a Bevy Vec3
fn pga_direction_to_vec3(direction: &Direction) -> Vec3 {
    let pga = &direction.0;

    // Direction vectors are represented as x*e032 + y*e013 + z*e021
    let x = pga.mvec[13]; // e032 component
    let y = pga.mvec[12]; // e013 component
    let z = pga.mvec[11]; // e021 component

    Vec3::new(x, y, z)
}

/// Draw a PGA line using gizmos
fn draw_pga_line(gizmos: &mut Gizmos<PGAGizmos>, line: &Line, color: LinearRgba) {
    let pga = &line.0;

    // Extract direction and moment components
    let dir_x = pga.mvec[10]; // e31
    let dir_y = pga.mvec[9]; // e23
    let dir_z = pga.mvec[8]; // e12

    let mom_x = pga.mvec[5]; // e02
    let mom_y = pga.mvec[6]; // e03
    let mom_z = pga.mvec[7]; // e01

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
fn draw_pga_plane(gizmos: &mut Gizmos<PGAGizmos>, plane: &Plane, color: LinearRgba) {
    let pga = &plane.0;

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

/// System to spawn plane meshes
fn spawn_plane_meshes(
    mut commands: Commands,
    scenes: Res<SceneLibrary>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    existing_planes: Query<Entity, With<PlaneMesh>>,
) {
    if !scenes.is_changed() {
        return;
    }

    // Remove existing plane meshes
    for entity in existing_planes.iter() {
        commands.entity(entity).despawn();
    }

    let scene = scenes.current();

    // Create meshes for each plane
    for (index, (plane, color)) in scene.planes.iter().enumerate() {
        if let Some((mesh, transform)) = create_plane_mesh(plane) {
            let material = materials.add(StandardMaterial {
                base_color: Color::LinearRgba(*color).with_alpha(0.3),
                alpha_mode: AlphaMode::Blend,
                cull_mode: None, // Render both sides
                ..default()
            });

            commands.spawn((
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(material),
                transform,
                PlaneMesh { plane_index: index },
            ));
        }
    }
}

/// Create a mesh and transform for a PGA plane
fn create_plane_mesh(plane: &Plane) -> Option<(Mesh, Transform)> {
    let pga = &plane.0;

    // Extract plane equation coefficients: ax + by + cz + d = 0
    let a = pga.mvec[2]; // e1
    let b = pga.mvec[3]; // e2
    let c = pga.mvec[4]; // e3
    let d = pga.mvec[1]; // e0

    let normal = Vec3::new(a, b, c);

    if normal.length() < f32::EPSILON {
        return None; // Invalid plane
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
    let v = normal.cross(u).normalize();

    // Create a quad mesh
    let size = 3.0;
    let positions = vec![
        (point_on_plane + u * -size + v * -size).to_array(), // Bottom-left
        (point_on_plane + u *  size + v * -size).to_array(), // Bottom-right
        (point_on_plane + u *  size + v *  size).to_array(), // Top-right
        (point_on_plane + u * -size + v *  size).to_array(), // Top-left
    ];

    let normals = vec![
        normal.to_array(),
        normal.to_array(),
        normal.to_array(),
        normal.to_array(),
    ];

    let uvs = vec![
        [0.0, 0.0],
        [1.0, 0.0],
        [1.0, 1.0],
        [0.0, 1.0],
    ];

    let indices = vec![
        0, 1, 2, // First triangle
        2, 3, 0, // Second triangle
    ];

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));

    Some((mesh, Transform::IDENTITY))
}

pub fn input_map(
    mut events: EventWriter<ControlEvent>,
    mut mouse_wheel_reader: EventReader<MouseWheel>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    controllers: Query<&OrbitCameraController>,
    mut contexts: EguiContexts,
) {
    // Check if egui is using the mouse - if so, don't process camera input
    if let Ok(ctx) = contexts.ctx_mut() {
        if ctx.is_pointer_over_area() || ctx.wants_pointer_input() {
            // Clear the events to prevent camera movement when interacting with egui
            for _ in mouse_motion_events.read() {}
            for _ in mouse_wheel_reader.read() {}
            return;
        }
    }

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

    if scalar != 1.0 {
        events.write(ControlEvent::Zoom(scalar));
    }
}

fn update_point_labels(
    scenes: Res<SceneLibrary>,
    // Query existing labels to clean them up when scene changes
    mut existing_labels: Query<(&mut Visibility, &Label)>,
    mut on_scene_changed: EventReader<SceneChangedEvent>,
) {
    if on_scene_changed.read().next().is_none() {
        return; // No scene change, no need to update labels
    }

    let scene = scenes.current();

    for (mut visibility, Label((index, kind))) in existing_labels.iter_mut() {
        let num_points = match kind {
            GeometricObject::Point => scene.points.len(),
            GeometricObject::Line => scene.lines.len(),
            GeometricObject::Plane => scene.planes.len(),
            GeometricObject::Direction => scene.directions.len(),
        };
        if *index < num_points {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

fn update_label_positions(
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut labels: Query<(&mut Node, &mut TextColor, &Label)>,
    scenes: Res<SceneLibrary>,
    windows: Query<&Window>,
) {
    let scene = scenes.current();
    let Ok((camera, camera_global_transform)) = camera_query.single() else {
        return;
    };

    let Ok(window) = windows.single() else {
        return;
    };

    for (mut node, mut text_color, Label((index, kind))) in labels.iter_mut() {
        let (world_position, color) = match kind {
            GeometricObject::Point => {
                if let Some((point, color)) = scene.points.get(*index) {
                    (pga_point_to_vec3(point), color)
                } else {
                    continue;
                }
            }
            GeometricObject::Line => {
                if let Some((line, color)) = scene.lines.get(*index) {
                    (pga_point_on_line(line), color)
                } else {
                    continue;
                }
            }
            GeometricObject::Plane => {
                if let Some((plane, color)) = scene.planes.get(*index) {
                    (pga_point_on_plane(plane), color)
                } else {
                    continue;
                }
            }
            GeometricObject::Direction => {
                if let Some((direction, color)) = scene.directions.get(*index) {
                    (pga_direction_to_vec3(direction), color)
                } else {
                    continue;
                }
            }
        };

        *text_color = TextColor(Color::from(*color));
        if let Ok(viewport_position) =
            camera.world_to_viewport(camera_global_transform, world_position)
        {
            // Clamp positions to ensure they stay within reasonable bounds
            let clamped_x = viewport_position.x.clamp(0.0, window.width() - 100.0);
            let clamped_y = viewport_position.y.clamp(0.0, window.height() - 30.0);
            node.left = Val::Px(clamped_x);
            node.top = Val::Px(clamped_y);
        }
    }
}

/// System to display coordinate editor UI for points
fn coordinate_editor_ui(
    mut contexts: EguiContexts,
    mut scenes: ResMut<SceneLibrary>,
    mut notify_points_changed: EventWriter<PointsChangedEvent>,
) {
    // Get the primary window context
    if let Ok(ctx) = contexts.ctx_mut() {
        let scene = scenes.current_mut();
        egui::Window::new("Point Coordinates")
            .default_open(true)
            .resizable(false)
            .default_pos([0.0, 0.0])
            .max_width(200.0)
            .show(ctx, |ui| {
                let mut points_changed = false;

                // Create a list of points with editable coordinates
                for (index, (point, _)) in scene.points.iter_mut().enumerate() {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(format!("P{}:", index));
                        });

                        ui.horizontal(|ui| {
                            // Extract current coordinates
                            let current_pos = pga_point_to_vec3(point);
                            let mut x = current_pos.x;
                            let mut y = current_pos.y;
                            let mut z = current_pos.z;

                            ui.label("X:");
                            if ui
                                .add(egui::DragValue::new(&mut x).speed(0.1).range(-10.0..=10.0))
                                .changed()
                            {
                                points_changed = true;
                            }

                            ui.label("Y:");
                            if ui
                                .add(egui::DragValue::new(&mut y).speed(0.1).range(-10.0..=10.0))
                                .changed()
                            {
                                points_changed = true;
                            }

                            ui.label("Z:");
                            if ui
                                .add(egui::DragValue::new(&mut z).speed(0.1).range(-10.0..=10.0))
                                .changed()
                            {
                                points_changed = true;
                            }

                            // Update the point if any coordinate changed
                            if points_changed {
                                *point = Point::new(x, y, z);
                            }
                        });
                    });
                    ui.separator();
                }

                if points_changed {
                    scene.rebuild();
                    notify_points_changed.write(PointsChangedEvent);
                }
            });
    }
}
