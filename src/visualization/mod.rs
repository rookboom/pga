use bevy::{
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

mod scenes;

use crate::pgai::{Direction, Line, Plane, Point};
use crate::visualization::scenes::PGAScene;

#[derive(Default, Resource)]
pub struct ObjectPool {
    pub points: Vec<Entity>,
    pub lines: Vec<Entity>,
    pub planes: Vec<Entity>,
    pub directions: Vec<Entity>,
}

#[derive(Default, Resource)]
pub struct SceneSelector {
    pub scenes: Vec<PGAScene>,
    current_scene_index: usize,
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
pub struct InputChangedEvent;

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
struct DirectionVisual(Point);

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
    pub fn current(&self) -> &PGAScene {
        &self.scenes[self.current_scene_index]
    }
    pub fn current_mut(&mut self) -> &mut PGAScene {
        &mut self.scenes[self.current_scene_index]
    }

    pub fn next_scene(&mut self) {
        self.current_scene_index = (self.current_scene_index + 1) % self.len();
    }

    pub fn prev_scene(&mut self) {
        if self.current_scene_index == 0 {
            self.current_scene_index = self.len() - 1;
        } else {
            self.current_scene_index -= 1;
        }
    }

    pub fn len(&self) -> usize {
        self.scenes.len()
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
            Visibility::Hidden,
            Label,
        ))
        .id()
}

fn spawn_object(commands: &mut Commands, color: SceneColor, text: String) -> Entity {
    let label = spawn_label(commands, text, color);
    commands
        .spawn((color, LinkedLabel(label), Visibility::Hidden))
        .id()
}

fn plane_transform(plane: &Plane) -> Transform {
    let mut plane = plane.clone();
    plane.unitize();
    let distance = plane.w();
    let normal = Vec3::from(plane.direction).normalize();
    let rotation = Quat::from_rotation_arc(Vec3::Y, normal);
    Transform {
        translation: -normal * distance,
        rotation,
        ..Default::default()
    }
}
fn spawn_plane(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    scene_materials: &SceneMaterials,
    color: SceneColor,
    plane: Plane,
    index: usize,
) -> Entity {
    let mesh = Plane3d::new(Vec3::Y, Vec2::splat(3.0));
    let material = scene_materials.find(color);
    let label = spawn_label(commands, format!("p{}", index), color);
    commands
        .spawn((
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(material),
            plane_transform(&plane),
            color,
            LinkedLabel(label),
            Visibility::Hidden,
        ))
        .id()
}

fn update_plane_transforms(
    scene_selector: Res<SceneSelector>,
    object_pool: Res<ObjectPool>,
    mut transforms: Query<&mut Transform>,
) {
    info!("Updating plane transforms...");
    let scene = scene_selector.current();

    for (plane, plane_entity) in scene.planes.iter().zip(object_pool.planes.iter()) {
        if let Ok(mut transform) = transforms.get_mut(*plane_entity) {
            *transform = plane_transform(plane);
        }
    }
}

fn set_visibility<T>(
    label_query: &mut Query<&mut Visibility, (With<Label>, Without<LinkedLabel>)>,
    object_query: &mut Query<(&mut Visibility, &LinkedLabel)>,
    entities: &Vec<Entity>,
    objects: &Vec<T>,
) {
    for (index, entity) in entities.iter().enumerate() {
        let visibility = if index < objects.len() {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        object_query
            .get_mut(*entity)
            .map(|(mut v, linked_label)| {
                *v = visibility;
                if let Ok(mut label_vis) = label_query.get_mut(linked_label.0) {
                    *label_vis = visibility;
                }
            })
            .ok();
    }
}

fn update_visibility(
    mut label_query: Query<&mut Visibility, (With<Label>, Without<LinkedLabel>)>,
    mut object_query: Query<(&mut Visibility, &LinkedLabel)>,
    object_pool: ResMut<ObjectPool>,
    scene_selector: ResMut<SceneSelector>,
) {
    let scene = scene_selector.current();
    set_visibility(
        &mut label_query,
        &mut object_query,
        &object_pool.points,
        &scene.points,
    );

    set_visibility(
        &mut label_query,
        &mut object_query,
        &object_pool.lines,
        &scene.lines,
    );

    set_visibility(
        &mut label_query,
        &mut object_query,
        &object_pool.planes,
        &scene.planes,
    );

    set_visibility(
        &mut label_query,
        &mut object_query,
        &object_pool.directions,
        &scene.directions,
    );
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
        .add_event::<InputChangedEvent>()
        .insert_resource(ObjectPool::default())
        .insert_resource(SceneSelector::default())
        .insert_resource(SceneMaterials::default())
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08))) // Very dark blue-gray
        .add_systems(Startup, (setup_scene, PGAScene::setup, setup_ui))
        .add_systems(Update, (draw_pga_gizmos, input_map, scene_selection_input))
        .add_systems(
            Update,
            (update_scene_ui, update_visibility).run_if(on_event::<SceneChangedEvent>),
        )
        .add_systems(
            Update,
            (PGAScene::rebuild, update_plane_transforms)
                .chain()
                .run_if(on_event::<InputChangedEvent>.or(on_event::<SceneChangedEvent>)),
        )
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
    mut object_pool: ResMut<ObjectPool>,
    mut meshes: ResMut<Assets<Mesh>>,
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

    let max_objects = 10;
    object_pool.points = (0..max_objects)
        .map(|i| spawn_object(&mut commands, SceneColor::WHITE, format!("P{}", i)))
        .collect();

    object_pool.lines = (0..max_objects)
        .map(|i| spawn_object(&mut commands, SceneColor::YELLOW, format!("L{}", i)))
        .collect();

    object_pool.directions = (0..max_objects)
        .map(|i| spawn_object(&mut commands, SceneColor::ORANGE, format!("D{}", i)))
        .collect();

    object_pool.planes = (0..max_objects)
        .map(|i| {
            let plane = Plane::new(1.0, 0.0, 0.0, 0.0);
            spawn_plane(
                &mut commands,
                &mut meshes,
                scene_materials.as_ref(),
                SceneColor::CYAN,
                plane,
                i,
            )
        })
        .collect();
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
    mut query: Query<&mut Text, With<SceneNameText>>,
) {
    for mut text in query.iter_mut() {
        **text = scene_selector.current().name.to_string();
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
    mut gizmos: Gizmos,
    scene_selector: Res<SceneSelector>,
    // points: Query<(&PointVisual, &SceneColor)>,
    // lines: Query<(&LineVisual, &SceneColor)>,
    // directions: Query<(&DirectionVisual, &SceneColor)>,
    // planes: Query<(&PlaneVisual, &SceneColor)>,
) {
    let scene = scene_selector.current();
    // Draw coordinate axes
    gizmos.line(Vec3::ZERO, Vec3::X * 2.0, LinearRgba::RED);
    gizmos.line(Vec3::ZERO, Vec3::Y * 2.0, LinearRgba::GREEN);
    gizmos.line(Vec3::ZERO, Vec3::Z * 2.0, LinearRgba::BLUE);

    // Draw points as small spheres
    for point in &scene.points {
        let pos = point.project();
        gizmos.sphere(pos, 0.01, SceneColor::WHITE.linear_rgba());
    }

    // Draw directions as arrows from origin
    for &direction in &scene.directions {
        let dir = Vec3::from(direction);
        gizmos.arrow(Vec3::ZERO, dir * 2.0, SceneColor::ORANGE.linear_rgba());
    }

    // Draw lines
    for line in &scene.lines {
        draw_pga_line(&mut gizmos, line, SceneColor::YELLOW.linear_rgba());
    }

    // Draw plane normal arrows (planes themselves are drawn as meshes)
    for plane in &scene.planes {
        draw_plane_normal_arrow(&mut gizmos, plane, SceneColor::CYAN.linear_rgba());
    }
}

fn pga_point_on_plane(plane: &Plane) -> Vec3 {
    let plane = plane.unitized();
    let w = plane.w();

    if w.abs() < f32::EPSILON {
        // TODO: This should return None..., since the plane does not go through the origin.
        Vec3::ZERO // Ideal plane, no finite point
    } else {
        let direction = Vec3::from(plane.direction);
        direction * -w
    }
}

fn pga_point_on_line(line: &Line) -> Vec3 {
    let direction = line.direction;
    // If direction is zero, this is an ideal line (line at infinity)
    if direction.is_zero() {
        return Vec3::ZERO; // Ideal line, return origin as placeholder
    }

    let direction = Vec3::from(direction);
    let moment = Vec3::from(line.moment);
    // Find a point on the line using the relationship: point = direction × moment / |direction|²
    let dir_length_sq = direction.length_squared();
    if dir_length_sq > f32::EPSILON {
        direction.cross(moment) / dir_length_sq
    } else {
        Vec3::ZERO
    }
}

/// Convert a PGA Direction to a Bevy Vec3
fn pga_direction_to_vec3(direction: Direction) -> Vec3 {
    Vec3::from(direction)
}

/// Draw a PGA line using gizmos
fn draw_pga_line(gizmos: &mut Gizmos, line: &Line, color: LinearRgba) {
    let direction = line.direction;
    // If direction is zero, this is an ideal line (line at infinity)
    if direction.is_zero() {
        return;
    }
    // Extract direction and moment components
    let direction = Vec3::from(direction);
    let moment = Vec3::from(line.moment);

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
fn draw_plane_normal_arrow(gizmos: &mut Gizmos, plane: &Plane, color: LinearRgba) {
    let point_on_plane = pga_point_on_plane(plane);
    let normal = Vec3::from(plane.direction).normalize();

    // Draw normal vector arrow
    gizmos.arrow(point_on_plane, point_on_plane + normal, color);
}

/// Create a mesh and transform for a PGA plane
fn create_plane_mesh(plane: &Plane) -> Mesh {
    let point_on_plane = pga_point_on_plane(plane);
    let normal = Vec3::from(plane.direction).normalize();

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
    scene_selector: Res<SceneSelector>,
    object_pool: Res<ObjectPool>,
    windows: Query<&Window>,
    mut labels: Query<&mut Node>,
    mut object_query: Query<(&mut Visibility, &LinkedLabel)>,
) {
    let Ok((camera, camera_global_transform)) = camera_query.single() else {
        return;
    };

    let Ok(window) = windows.single() else {
        return;
    };

    let scene = scene_selector.current();

    let mut update_position = |obj_entity, world_position| {
        let label_entity = if let Ok((_, linked_label)) = object_query.get_mut(obj_entity) {
            linked_label.0
        } else {
            return;
        };
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

    for (point, obj_entity) in scene.points.iter().zip(object_pool.points.iter()) {
        update_position(*obj_entity, point.project());
    }

    for (line, obj_entity) in scene.lines.iter().zip(object_pool.lines.iter()) {
        update_position(*obj_entity, pga_point_on_line(line));
    }

    for (plane, obj_entity) in scene.planes.iter().zip(object_pool.planes.iter()) {
        update_position(*obj_entity, pga_point_on_plane(plane));
    }

    for (&direction, obj_entity) in scene.directions.iter().zip(object_pool.directions.iter()) {
        update_position(*obj_entity, Vec3::from(direction));
    }
}

/// System to display coordinate editor UI for points
fn coordinate_editor_ui(
    mut contexts: EguiContexts,
    mut scene_selector: ResMut<SceneSelector>,
    mut notify_input_changed: EventWriter<InputChangedEvent>,
) {
    let scene = scene_selector.current_mut();

    let edit_value = |label, ui: &mut egui::Ui, value: &mut f32| {
        ui.label(label);
        ui.add(egui::DragValue::new(value).speed(0.1).range(-10.0..=10.0))
            .changed()
    };

    let edit_vec3 = |label, ui: &mut egui::Ui, value: &mut Vec3, index| {
        let mut points_changed = false;
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label(format!("{label}{index}:"));
            });

            ui.horizontal(|ui| {
                points_changed = edit_value("X:", ui, &mut value.x)
                    | edit_value("Y:", ui, &mut value.y)
                    | edit_value("Z:", ui, &mut value.z)
            })
        });
        ui.separator();
        points_changed
    };

    let edit_plane = |label, ui: &mut egui::Ui, value: &mut Vec4, index| {
        let mut points_changed = false;
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label(format!("{label}{index}:"));
            });

            ui.horizontal(|ui| {
                points_changed = edit_value("x:", ui, &mut value.x)
                    | edit_value("y:", ui, &mut value.y)
                    | edit_value("z:", ui, &mut value.z)
                    | edit_value("w:", ui, &mut value.w)
            })
        });
        ui.separator();
        points_changed
    };

    // Get the primary window context
    if let Ok(ctx) = contexts.ctx_mut() {
        egui::Window::new("Inputs")
            .default_open(true)
            .resizable(false)
            .default_pos([0.0, 0.0])
            .max_width(200.0)
            .show(ctx, |ui| {
                let mut points_changed = false;

                for i in 0..scene.input_point_count {
                    if let Some(point) = scene.points.get_mut(i) {
                        let mut vec = point.project();
                        points_changed |= edit_vec3("Point P", ui, &mut vec, i);
                        if points_changed {
                            *point = Point::new(vec[0], vec[1], vec[2]);
                        }
                    }
                }

                for i in 0..scene.input_direction_count {
                    if let Some(direction) = scene.directions.get_mut(i) {
                        let mut vec = Vec3::from(*direction);
                        points_changed |= edit_vec3("Direction D", ui, &mut vec, i);
                        if points_changed {
                            *direction = Direction::new(vec[0], vec[1], vec[2]);
                        }
                    }
                }

                // NOTE: We don't use lines as input since the moment and direction
                // depend on each other. Construct a line with two points or a direction and a point instead.

                for i in 0..scene.input_plane_count {
                    if let Some(plane) = scene.planes.get_mut(i) {
                        let mut values = Vec4::from(*plane);
                        points_changed |= edit_plane("Plane p", ui, &mut values, i);
                        if points_changed {
                            *plane = Plane::new(values.x, values.y, values.z, values.w);
                        }
                    }
                }

                if points_changed {
                    notify_input_changed.write(InputChangedEvent);
                }
            });
    }
}
