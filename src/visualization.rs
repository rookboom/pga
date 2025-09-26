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
    scene,
};
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};
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

#[derive(Resource)]
pub struct SceneSelector {
    pub points: Vec<Entity>,
    pub lines: Vec<Entity>,
    pub planes: Vec<Entity>,
    pub directions: Vec<Entity>,
    pub scene_names: Vec<&'static str>,
    current_scene_index: usize,
}

enum PGAObject {
    Point(Point),
    Line(Line),
    Plane(Plane),
    Direction(Direction),
}

struct PGAScene {
    pub point_indices: Vec<(usize, Origin)>,
    pub line_indices: Vec<(usize, Origin)>,
    pub plane_indices: Vec<(usize, Origin)>,
    pub direction_indices: Vec<(usize, Origin)>,
}

#[derive(Default, Resource)]
pub struct SceneMaterials {
    pub white: Handle<StandardMaterial>,
    pub red: Handle<StandardMaterial>,
    pub green: Handle<StandardMaterial>,
    pub blue: Handle<StandardMaterial>,
    pub yellow: Handle<StandardMaterial>,
    pub cyan: Handle<StandardMaterial>,
    pub magenta: Handle<StandardMaterial>,
    pub orange: Handle<StandardMaterial>,
}

#[derive(Event)]
pub struct SceneChangedEvent;

#[derive(Event)]
pub struct PointsChangedEvent;

impl SceneMaterials {
    fn find(&self, color: SceneColor) -> Handle<StandardMaterial> {
        match color {
            SceneColor::YELLOW => self.yellow.clone(),
            SceneColor::RED => self.red.clone(),
            SceneColor::GREEN => self.green.clone(),
            SceneColor::BLUE => self.blue.clone(),
            SceneColor::MAGENTA => self.magenta.clone(),
            SceneColor::CYAN => self.cyan.clone(),
            SceneColor::ORANGE => self.orange.clone(),
            SceneColor::WHITE => self.white.clone(),
        }
    }
}
impl SceneSelector {
    pub fn new() -> Self {
        let scene_names = vec![
            PGAScene::EMPTY_SCENE,
            PGAScene::TWO_POINTS_JOIN_IN_A_LINE,
            PGAScene::THREE_POINTS_JOIN_IN_A_PLANE,
            PGAScene::LINE_AND_POINT_JOIN_IN_A_PLANE,
            PGAScene::THREE_PLANES_MEET_IN_A_POINT,
        ];
        Self {
            scene_names,
            current_scene_index: 0,
        }
    }

    pub fn current(&self) -> &str {
        self.scene_names[self.current_scene_index]
    }

    pub fn next_scene(&mut self) {
        self.current_scene_index = (self.current_scene_index + 1) % self.scene_names.len();
    }

    pub fn prev_scene(&mut self) {
        if self.current_scene_index == 0 {
            self.current_scene_index = self.scene_names.len() - 1;
        } else {
            self.current_scene_index -= 1;
        }
    }

    pub fn len(&self) -> usize {
        self.scene_names.len()
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
struct PointVisual(Point);

#[derive(Component)]
struct LineVisual(Line);

#[derive(Component)]
struct PlaneVisual(Plane);

#[derive(Component)]
struct DirectionVisual(Direction);

#[derive(Component)]
struct LinkedLabel(Entity);

#[derive(Component)]
struct Label;

#[derive(Component, Copy, Clone)]
enum SceneColor {
    YELLOW,
    RED,
    GREEN,
    BLUE,
    MAGENTA,
    CYAN,
    ORANGE,
    WHITE,
}

#[derive(Component, PartialEq)]
enum Origin {
    Computed,
    Input,
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

impl SceneColor {
    pub fn linear_rgba(&self) -> LinearRgba {
        match self {
            SceneColor::YELLOW => LinearRgba::rgb(1.0, 1.0, 0.0),
            SceneColor::RED => LinearRgba::rgb(1.0, 0.0, 0.0),
            SceneColor::GREEN => LinearRgba::rgb(0.0, 1.0, 0.0),
            SceneColor::BLUE => LinearRgba::rgb(0.0, 0.0, 1.0),
            SceneColor::MAGENTA => LinearRgba::rgb(1.0, 0.0, 1.0),
            SceneColor::CYAN => LinearRgba::rgb(0.0, 1.0, 1.0),
            SceneColor::ORANGE => LinearRgba::rgb(1.0, 0.5, 0.0),
            SceneColor::WHITE => LinearRgba::rgb(1.0, 1.0, 1.0),
        }
    }
}

impl PGAScene {
    const EMPTY_SCENE: &str = "Empty Scene";
    const TWO_POINTS_JOIN_IN_A_LINE: &str = "Two points join in a line: L1 = P0 V P1";
    const THREE_POINTS_JOIN_IN_A_PLANE: &str = "Three points join in a plane: P0 = P0 V P1 V P2";
    const LINE_AND_POINT_JOIN_IN_A_PLANE: &str = "A line and a point join in a plane: P0 = L0 V P2";
    const THREE_PLANES_MEET_IN_A_POINT: &str = "Three planes meet in a point: P9 = P1 ^ P2 ^ P3";
}

fn spawn_label(commands: &mut Commands, text: String, color: SceneColor) -> Entity {
    commands
        .spawn((
            Text::new(text),
            TextColor(color.linear_rgba().into()),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                ..default()
            },
            ZIndex(1000),
            Visibility::Visible,
            Label,
        ))
        .id()
}

fn spawn_point<'a>(
    commands: &'a mut Commands,
    color: SceneColor,
    point: Point,
    index: usize,
) -> EntityCommands<'a> {
    let label = spawn_label(commands, format!("P{}", index), color);

    commands.spawn((PointVisual(point), color, LinkedLabel(label)))
}

fn spawn_line<'a>(
    commands: &'a mut Commands,
    color: SceneColor,
    line: Line,
    index: usize,
) -> EntityCommands<'a> {
    let label = spawn_label(commands, format!("L{}", index), color);
    commands.spawn((LineVisual(line), color, LinkedLabel(label)))
}

fn spawn_input_points(commands: &mut Commands, points: &[&Point]) -> Vec<Entity> {
    points
        .iter()
        .enumerate()
        .map(|(i, &point)| {
            spawn_point(commands, SceneColor::WHITE, point.clone(), i)
                .insert(Origin::Input)
                .id()
        })
        .collect()
}

fn spawn_plane<'a>(
    commands: &'a mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    scene_materials: &Res<SceneMaterials>,
    color: SceneColor,
    plane: Plane,
    index: usize,
) -> EntityCommands<'a> {
    let mesh = Plane3d::new(Vec3::Y, Vec2::splat(3.0));
    let material = scene_materials.find(color);
    let label = spawn_label(commands, format!("p{}", index), color);
    let rotation = Quat::from_rotation_arc(Vec3::Y, plane.normal());
    let transform = Transform {
        translation: -plane.normal() * plane.distance(),
        rotation,
        ..Default::default()
    };
    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(material),
        transform,
        PlaneVisual(plane),
        color,
        LinkedLabel(label),
    ))
}

fn update_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut scene_materials: Res<SceneMaterials>,
    visual_objects: Query<(Entity, &LinkedLabel, &Origin)>,
    scene_selector: Res<SceneSelector>,
    mut scene_inputs: ResMut<SceneInputs>,
    mut on_points_changed: EventReader<PointsChangedEvent>,
    point_visuals: Query<&PointVisual>,
    line_visuals: Query<&LineVisual>,
    plane_visuals: Query<&PlaneVisual>,
    direction_visuals: Query<&DirectionVisual>,
) {
    let points_changed = on_points_changed.read().next().is_some();
    if !points_changed {
        return;
    }

    info!("Points changed, updating scene...");

    let scene_name = scene_selector.scene_names[scene_selector.current_scene_index];
}

fn rebuild_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut scene_materials: Res<SceneMaterials>,
    visual_objects: Query<(Entity, &LinkedLabel)>,

    scene_selector: Res<SceneSelector>,
    mut scene_inputs: ResMut<SceneInputs>,
    mut on_scene_changed: EventReader<SceneChangedEvent>,
    mut notify_points_changed: EventWriter<PointsChangedEvent>,
) {
    let scene_changed = on_scene_changed.read().next().is_some();
    if !scene_changed {
        return;
    }

    info!("Scene changed, updating scene...");

    for (entity, &LinkedLabel(label)) in visual_objects.iter() {
        commands.entity(entity).despawn();
        commands.entity(label).despawn();
    }

    scene_inputs.entities.clear();

    let existing_meshes: Vec<AssetId<Mesh>> = meshes.iter().map(|(id, _)| id.clone()).collect();
    for id in existing_meshes {
        meshes.remove(id);
    }

    let scene_name = scene_selector.scene_names[scene_selector.current_scene_index];

    let p0 = &Point::new(1.0, 0.0, 0.0);
    let p1 = &Point::new(0.0, 1.0, 0.0);
    let p2 = &Point::new(0.0, 0.0, 1.0);

    match scene_name {
        PGAScene::TWO_POINTS_JOIN_IN_A_LINE => {
            spawn_input_points(&mut commands, &[p0, p1]);
            if let Some(line) = (p0 & p1).value() {
                spawn_line(&mut commands, SceneColor::ORANGE, line, 0);
            }
        }
        PGAScene::THREE_POINTS_JOIN_IN_A_PLANE => {
            spawn_input_points(&mut commands, &[p0, p1, p2]);
            if let Some(plane) = (p0 & p1 & p2).value() {
                spawn_plane(
                    &mut commands,
                    &mut meshes,
                    &mut scene_materials,
                    SceneColor::ORANGE,
                    plane,
                    0,
                );
            }
        }
        PGAScene::LINE_AND_POINT_JOIN_IN_A_PLANE => {
            spawn_input_points(&mut commands, &[p0, p1, p2]);
            if let Some(line) = (p0 & p1).value() {
                spawn_line(&mut commands, SceneColor::ORANGE, line.clone(), 0)
                    .insert(Origin::Computed);
                if let Some(plane) = (line & p2).value() {
                    spawn_plane(
                        &mut commands,
                        &mut meshes,
                        &mut scene_materials,
                        SceneColor::ORANGE,
                        plane,
                        0,
                    );
                }
            }
        }
        PGAScene::THREE_PLANES_MEET_IN_A_POINT => {
            let p = &[
                &Point::new(1.0, 1.0, 0.0),
                &Point::new(1.0, 0.0, 2.0),
                &Point::new(1.0, 2.0, 0.0),
                &Point::new(0.0, 2.0, 1.0),
                &Point::new(0.0, 2.0, 3.0),
                &Point::new(3.0, 2.0, 0.0),
                &Point::new(0.0, 1.0, 1.0),
                &Point::new(0.0, 3.0, 1.0),
                &Point::new(3.0, 0.0, 1.0),
            ];

            let plane0 = p[0] & p[1] & p[2];
            let plane1 = p[3] & p[4] & p[5];
            let plane2 = p[6] & p[7] & p[8];

            if let Some(plane) = plane0.value() {
                spawn_plane(
                    &mut commands,
                    &mut meshes,
                    &mut scene_materials,
                    SceneColor::MAGENTA,
                    plane,
                    0,
                )
                .insert(Origin::Input);
            }
            if let Some(plane) = plane1.value() {
                spawn_plane(
                    &mut commands,
                    &mut meshes,
                    &mut scene_materials,
                    SceneColor::ORANGE,
                    plane,
                    1,
                )
                .insert(Origin::Input);
            }
            if let Some(plane) = plane2.value() {
                spawn_plane(
                    &mut commands,
                    &mut meshes,
                    &mut scene_materials,
                    SceneColor::CYAN,
                    plane,
                    2,
                )
                .insert(Origin::Input);
            }
        }
        _ => { /* Empty scene or unrecognized scene name */ }
    }

    notify_points_changed.write(PointsChangedEvent);
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
        .insert_resource(SceneInputs::default())
        .insert_resource(SceneSelector::new())
        .insert_resource(SceneMaterials::default())
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08))) // Very dark blue-gray
        .add_systems(Startup, (setup_scene, setup_ui))
        .add_systems(Update, draw_pga_gizmos)
        .add_systems(Update, input_map)
        .add_systems(Update, scene_selection_input)
        .add_systems(Update, update_scene_ui)
        .add_systems(Update, rebuild_scene)
        .add_systems(Update, update_scene)
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
fn setup_scene(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut scene_materials: ResMut<SceneMaterials>,
) {
    let mut create_material = |color: SceneColor| {
        materials.add(StandardMaterial {
            base_color: color.linear_rgba().with_alpha(0.3).into(),
            alpha_mode: AlphaMode::Blend,
            cull_mode: None, // Render both sides
            ..default()
        })
    };
    scene_materials.white = create_material(SceneColor::WHITE);
    scene_materials.red = create_material(SceneColor::RED);
    scene_materials.green = create_material(SceneColor::GREEN);
    scene_materials.blue = create_material(SceneColor::BLUE);
    scene_materials.yellow = create_material(SceneColor::YELLOW);
    scene_materials.cyan = create_material(SceneColor::CYAN);
    scene_materials.magenta = create_material(SceneColor::MAGENTA);
    scene_materials.orange = create_material(SceneColor::ORANGE);

    commands
        .spawn(Camera3d::default())
        .insert(OrbitCameraBundle::new(
            OrbitCameraController::default(),
            Vec3::new(5.0, 5.0, 5.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ));
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
    scene_selector: Res<SceneSelector>,
    mut on_scene_changed: EventReader<SceneChangedEvent>,
    mut query: Query<&mut Text, With<SceneNameText>>,
) {
    if on_scene_changed.read().next().is_none() {
        return; // No scene change, no need to update UI
    }

    for mut text in query.iter_mut() {
        **text = scene_selector.current().to_string();
    }
}

/// System for keyboard scene selection
fn scene_selection_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut scene_library: ResMut<SceneSelector>,
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
fn draw_pga_gizmos(
    mut gizmos: Gizmos<PGAGizmos>,
    points: Query<(&PointVisual, &SceneColor)>,
    lines: Query<(&LineVisual, &SceneColor)>,
    directions: Query<(&DirectionVisual, &SceneColor)>,
    planes: Query<(&PlaneVisual, &SceneColor)>,
) {
    // Draw coordinate axes
    gizmos.line(Vec3::ZERO, Vec3::X * 2.0, LinearRgba::RED);
    gizmos.line(Vec3::ZERO, Vec3::Y * 2.0, LinearRgba::GREEN);
    gizmos.line(Vec3::ZERO, Vec3::Z * 2.0, LinearRgba::BLUE);

    // Draw points as small spheres
    for (point, color) in &points {
        let pos = pga_point_to_vec3(&point.0);
        gizmos.sphere(pos, 0.01, color.linear_rgba());
    }

    // Draw directions as arrows from origin
    for (direction, color) in &directions {
        let dir = pga_direction_to_vec3(&direction.0);
        gizmos.arrow(Vec3::ZERO, dir * 2.0, color.linear_rgba());
    }

    // Draw lines
    for (line, color) in &lines {
        draw_pga_line(&mut gizmos, &line.0, color.linear_rgba());
    }

    // Draw plane normal arrows (planes themselves are drawn as meshes)
    for (plane, color) in &planes {
        info!("Drawing plane normal for plane: {:?}", plane.0);
        draw_plane_normal_arrow(&mut gizmos, &plane.0, color.linear_rgba());
    }
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

/// Draw just the normal arrow for a PGA plane (used when plane is drawn as mesh)
fn draw_plane_normal_arrow(gizmos: &mut Gizmos<PGAGizmos>, plane: &Plane, color: LinearRgba) {
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

    // Draw normal vector arrow
    gizmos.arrow(point_on_plane, point_on_plane + normal * 1.0, color);
}

/// Create a mesh and transform for a PGA plane
fn create_plane_mesh(plane: &Plane) -> Mesh {
    let pga = &plane.0;

    // Extract plane equation coefficients: ax + by + cz + d = 0
    let a = pga.mvec[2]; // e1
    let b = pga.mvec[3]; // e2
    let c = pga.mvec[4]; // e3
    let d = pga.mvec[1]; // e0

    let normal = Vec3::new(a, b, c);

    if normal.length() < f32::EPSILON {
        return Cuboid::new(0.0, 0.0, 0.0).mesh().build(); // Invalid plane
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
        (point_on_plane + u * size + v * -size).to_array(),  // Bottom-right
        (point_on_plane + u * size + v * size).to_array(),   // Top-right
        (point_on_plane + u * -size + v * size).to_array(),  // Top-left
    ];

    let normals = vec![
        normal.to_array(),
        normal.to_array(),
        normal.to_array(),
        normal.to_array(),
    ];

    let uvs = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];

    let indices = vec![
        0, 1, 2, // First triangle
        2, 3, 0, // Second triangle
    ];

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));

    mesh
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

fn update_label_positions(
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut labels: Query<&mut Node>,
    mut points: Query<(&PointVisual, &LinkedLabel)>,
    mut lines: Query<(&LineVisual, &LinkedLabel)>,
    mut planes: Query<(&PlaneVisual, &LinkedLabel)>,
    mut directions: Query<(&DirectionVisual, &LinkedLabel)>,
    windows: Query<&Window>,
) {
    let Ok((camera, camera_global_transform)) = camera_query.single() else {
        return;
    };

    let Ok(window) = windows.single() else {
        return;
    };

    let mut update_position = |label_entity, world_position| {
        if let Ok(mut node) = labels.get_mut(label_entity) {
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
    };

    for (point, label) in points.iter_mut() {
        update_position(label.0, pga_point_to_vec3(&point.0));
    }

    for (line, label) in lines.iter_mut() {
        update_position(label.0, pga_point_on_line(&line.0));
    }

    for (plane, label) in planes.iter_mut() {
        update_position(label.0, pga_point_on_plane(&plane.0));
    }

    for (direction, label) in directions.iter_mut() {
        update_position(label.0, pga_direction_to_vec3(&direction.0));
    }
}

/// System to display coordinate editor UI for points
fn coordinate_editor_ui(
    mut contexts: EguiContexts,
    mut points: Query<&mut PointVisual>,
    mut notify_points_changed: EventWriter<PointsChangedEvent>,
) {
    // Get the primary window context
    if let Ok(ctx) = contexts.ctx_mut() {
        egui::Window::new("Point Coordinates")
            .default_open(true)
            .resizable(false)
            .default_pos([0.0, 0.0])
            .max_width(200.0)
            .show(ctx, |ui| {
                let mut points_changed = false;

                // Create a list of points with editable coordinates
                for (index, mut point) in points.iter_mut().enumerate() {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(format!("P{}:", index));
                        });

                        ui.horizontal(|ui| {
                            // Extract current coordinates
                            let current_pos = pga_point_to_vec3(&point.0);
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
                                *point = PointVisual(Point::new(x, y, z));
                            }
                        });
                    });
                    ui.separator();
                }

                if points_changed {
                    notify_points_changed.write(PointsChangedEvent);
                }
            });
    }
}
