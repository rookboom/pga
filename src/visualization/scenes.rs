use crate::visualization::SceneSelector;
use crate::{Direction, Line, Plane, Point};
use bevy::prelude::*;

#[derive(Default)]
pub struct PGAScene {
    pub name: &'static str,
    pub points: Vec<Point>,
    pub lines: Vec<Line>,
    pub planes: Vec<Plane>,
    pub directions: Vec<Direction>,
    pub input_point_count: usize,
    pub input_plane_count: usize,
    pub input_direction_count: usize,
}

impl PGAScene {
    // Examples from: https://bivector.net/tools.html?p=3&q=0&r=1
    // Incidence
    pub const EMPTY_SCENE: &str = "Empty Scene";
    pub const TWO_POINTS_JOIN_IN_A_LINE: &str = "Two points join in a line: L0 = P0 V P1";
    pub const DIRECTIONS_AND_POINTS_JOIN_IN_A_LINE: &str =
        "A direction and a point join in a line: L0 = D0 V P0";
    pub const THREE_POINTS_JOIN_IN_A_PLANE: &str =
        "Three points join in a plane: p0 = P0 V P1 V P2";
    pub const LINE_AND_POINT_JOIN_IN_A_PLANE: &str =
        "A line and a point join in a plane: p0 = L0 V P0";
    pub const LINE_AND_PLANE_MEET_IN_A_POINT: &str =
        "A line and a plane meet in a point: P2 = L0 ^ p0";
    pub const THREE_PLANES_MEET_IN_A_POINT: &str =
        "Three planes meet in a point: P0 = p0 ^ p1 ^ p2";
    pub const TWO_PLANES_MEET_IN_A_LINE: &str = "Two planes meet in a line: L0 = p0 ^ p1";

    // Project & reject
    pub const PLANE_PERP_THROUGH_LINE: &str =
        "The plane perpendicular to plane p0 through line L0: p1 = p0 | L0";
    pub const LINE_PERP_THROUGH_POINT: &str =
        "The line perpendicular to plane p0 through point L0: p1 = P0 | p0";
    pub const PLANE_PERP_THROUGH_POINT: &str =
        "The plane perpendicular to line L0 through point P0: p0 = L0 | P0";
    pub const PROJECT_PLANE_ONTO_POINT: &str =
        "The projection of plane p0 onto point P0: p1 = (p0 | P0) * P0";
    pub const PROJECT_POINT_ONTO_PLANE: &str =
        "The projection of point P0 onto plane p0: P1 = (p0 | P0) * p0";
    pub const PROJECT_LINE_ONTO_PLANE: &str =
        "The projection of line L0 onto plane p0: L1 = (p0 | L0) ^ p0";
    /// Setup the initial scene with camera and lighting
    pub fn setup(mut scene_selector: ResMut<SceneSelector>) {
        let p0 = &Point::new(1.0, 0.0, 0.0);
        let p1 = &Point::new(0.0, 1.0, 0.0);
        let p2 = &Point::new(0.0, 0.0, 1.0);

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

        let plane0 = (p[0] & p[1] & p[2]).value().unwrap();
        let plane1 = (p[3] & p[4] & p[5]).value().unwrap();
        let plane2 = (p[6] & p[7] & p[8]).value().unwrap();

        scene_selector.scenes = vec![
            PGAScene {
                name: PGAScene::EMPTY_SCENE,
                points: vec![],
                lines: vec![],
                planes: vec![],
                directions: vec![],
                ..default()
            },
            PGAScene {
                name: PGAScene::TWO_POINTS_JOIN_IN_A_LINE,
                points: vec![p0.clone(), p1.clone()],
                lines: vec![Line::through_origin(1.0, 0.0, 0.0)],
                planes: vec![],
                directions: vec![],
                input_point_count: 2,
                ..default()
            },
            PGAScene {
                name: PGAScene::DIRECTIONS_AND_POINTS_JOIN_IN_A_LINE,
                points: vec![Point::new(1.0, 1.0, 0.0)],
                lines: vec![Line::through_origin(0.0, 0.0, 0.0)],
                planes: vec![],
                directions: vec![Direction::new(0.0, 1.0, 0.0)],
                input_point_count: 1,
                input_direction_count: 1,
                ..default()
            },
            PGAScene {
                name: PGAScene::THREE_POINTS_JOIN_IN_A_PLANE,
                points: vec![p0.clone(), p1.clone(), p2.clone()],
                lines: vec![],
                planes: vec![Plane::new(1.0, 0.0, 0.0, 0.0)],
                directions: vec![],
                input_point_count: 3,
                ..default()
            },
            PGAScene {
                name: PGAScene::LINE_AND_POINT_JOIN_IN_A_PLANE,
                points: vec![p0.clone(), p1.clone(), p2.clone()],
                lines: vec![(p1 & p2).value().unwrap()],
                planes: vec![Plane::new(1.0, 0.0, 0.0, 0.0)],
                directions: vec![],
                input_point_count: 3,
                ..default()
            },
            PGAScene {
                name: PGAScene::LINE_AND_PLANE_MEET_IN_A_POINT,
                points: vec![p0.clone(), p1.clone(), p2.clone()],
                lines: vec![(p1 & p2).value().unwrap()],
                planes: vec![Plane::new(1.0, 0.0, 1.0, 1.0)],
                directions: vec![],
                input_point_count: 2,
                input_plane_count: 1,

                ..default()
            },
            PGAScene {
                name: PGAScene::TWO_PLANES_MEET_IN_A_LINE,
                points: vec![],
                lines: vec![Line::through_origin(1.0, 0.0, 0.0)],
                planes: vec![plane0.clone(), plane1.clone()],
                directions: vec![],
                input_plane_count: 2,
                ..default()
            },
            PGAScene {
                name: PGAScene::THREE_PLANES_MEET_IN_A_POINT,
                points: vec![Point::new(0.0, 0.0, 0.0)],
                lines: vec![],
                planes: vec![plane0, plane1, plane2],
                directions: vec![],
                input_plane_count: 3,
                ..default()
            },
            PGAScene {
                name: PGAScene::PLANE_PERP_THROUGH_LINE,
                points: vec![p0.clone(), p1.clone()],
                lines: vec![Line::through_origin(1.0, 0.0, 0.0)],
                planes: vec![
                    Plane::new(1.0, 0.0, 1.0, 1.0),
                    Plane::new(0.0, 0.0, 0.0, 0.0),
                ],
                directions: vec![],
                input_plane_count: 1,
                input_point_count: 2,
                ..default()
            },
            PGAScene {
                name: PGAScene::LINE_PERP_THROUGH_POINT,
                points: vec![p0.clone()],
                lines: vec![Line::through_origin(1.0, 0.0, 0.0)],
                planes: vec![Plane::new(1.0, 0.0, 1.0, 1.0)],
                directions: vec![],
                input_plane_count: 1,
                input_point_count: 1,
                ..default()
            },
            PGAScene {
                name: PGAScene::PLANE_PERP_THROUGH_POINT,
                points: vec![Point::new(1.0, 0.0, 1.0), p1.clone(), p2.clone()],
                lines: vec![(p1 & p2).value().unwrap()],
                planes: vec![Plane::new(1.0, 0.0, 0.0, 0.0)],
                directions: vec![],
                input_point_count: 3,
                ..default()
            },
            PGAScene {
                name: PGAScene::PROJECT_PLANE_ONTO_POINT,
                points: vec![Point::new(1.0, 2.0, 3.0)],
                lines: vec![],
                planes: vec![
                    Plane::new(-1.0, 1.0, 1.0, 1.0),
                    Plane::new(0.0, 0.0, 0.0, 0.0),
                ],
                directions: vec![],
                input_point_count: 1,
                input_plane_count: 1,
                ..default()
            },
            PGAScene {
                name: PGAScene::PROJECT_POINT_ONTO_PLANE,
                points: vec![Point::new(0.0, 1.0, 0.0), Point::new(0.0, 0.0, 0.0)],
                lines: vec![],
                planes: vec![Plane::new(-1.0, 1.0, 1.0, 1.0)],
                directions: vec![],
                input_point_count: 1,
                input_plane_count: 1,
                ..default()
            },
            PGAScene {
                name: PGAScene::PROJECT_LINE_ONTO_PLANE,
                points: vec![p0.clone(), p1.clone()],
                lines: vec![
                    (p1 & p2).value().unwrap(),
                    Line::through_origin(0.0, 0.0, 0.0),
                ],
                planes: vec![Plane::new(1.0, 0.0, 1.0, 1.0)],
                directions: vec![],
                input_point_count: 2,
                input_plane_count: 1,

                ..default()
            },
        ];
    }

    pub fn rebuild(mut scene_selector: ResMut<SceneSelector>) {
        info!("Input/Scene changed, rebuilding scene...");

        let scene = scene_selector.current_mut();

        match scene.name {
            PGAScene::TWO_POINTS_JOIN_IN_A_LINE => {
                let p0 = &scene.points[0];
                let p1 = &scene.points[1];
                // Output
                let line = p0 & p1;
                if let Some(line) = line.value() {
                    scene.lines[0] = line;
                }
            }
            PGAScene::DIRECTIONS_AND_POINTS_JOIN_IN_A_LINE => {
                // Inputs
                let p0 = &scene.points[0];
                let d0 = &scene.directions[0];
                // Output
                if let Some(line) = (p0 & d0).value() {
                    scene.lines[0] = line;
                }
            }
            PGAScene::THREE_POINTS_JOIN_IN_A_PLANE => {
                // Inputs
                let p0 = &scene.points[0];
                let p1 = &scene.points[1];
                let p2 = &scene.points[2];
                // Output
                if let Some(plane) = (p0 & p1 & p2).value() {
                    scene.planes[0] = plane;
                }
            }
            PGAScene::LINE_AND_POINT_JOIN_IN_A_PLANE => {
                // Inputs
                let p0 = &scene.points[0];
                let p1 = &scene.points[1];
                let p2 = &scene.points[2];

                // Output
                if let Some(line) = (p1 & p2).value() {
                    scene.lines[0] = line.clone();
                    if let Some(plane) = (line & p0).value() {
                        scene.planes[0] = plane;
                    }
                }
            }
            PGAScene::LINE_AND_PLANE_MEET_IN_A_POINT => {
                // Inputs
                let plane0 = &scene.planes[0];
                let p0 = &scene.points[0];
                let p1 = &scene.points[1];

                // Output
                if let Some(line) = (p0 & p1).value() {
                    scene.lines[0] = line.clone();
                    if let Some(point) = (line ^ plane0).value() {
                        scene.points[2] = point;
                    }
                }
            }
            PGAScene::TWO_PLANES_MEET_IN_A_LINE => {
                let plane0 = &scene.planes[0];
                let plane1 = &scene.planes[1];

                // Output
                let line = plane0 ^ plane1;
                if let Some(line) = line.value() {
                    scene.lines[0] = line;
                }
            }
            PGAScene::THREE_PLANES_MEET_IN_A_POINT => {
                let plane0 = &scene.planes[0];
                let plane1 = &scene.planes[1];
                let plane2 = &scene.planes[2];

                // Output
                let point = plane0 ^ plane1 ^ plane2;
                if let Some(point) = point.value() {
                    scene.points[0] = point;
                }
            }
            PGAScene::PLANE_PERP_THROUGH_LINE => {
                let p0 = &scene.points[0];
                let p1 = &scene.points[1];
                let plane0 = &scene.planes[0];

                // Output
                if let Some(line) = (p0 & p1).value() {
                    scene.lines[0] = line.clone();
                    if let Some(plane) = plane0 | line {
                        scene.planes[1] = plane;
                    }
                }
            }
            PGAScene::LINE_PERP_THROUGH_POINT => {
                let p0 = &scene.points[0];
                let plane0 = &scene.planes[0];

                // Output
                scene.lines[0] = plane0 | p0;
            }
            PGAScene::PLANE_PERP_THROUGH_POINT => {
                // Inputs
                let p0 = &scene.points[0];
                let p1 = &scene.points[1];
                let p2 = &scene.points[2];

                // Output
                if let Some(line) = (p1 & p2).value() {
                    scene.lines[0] = line.clone();
                    scene.planes[0] = line | p0;
                }
            }
            PGAScene::PROJECT_PLANE_ONTO_POINT => {
                let p0 = &scene.points[0];
                let plane0 = &scene.planes[0];

                // Output
                scene.planes[1] = (plane0 | p0) * p0;
            }
            PGAScene::PROJECT_POINT_ONTO_PLANE => {
                let p0 = &scene.points[0];
                let plane0 = &scene.planes[0];

                // Output
                scene.points[1] = (plane0 | p0) * plane0;
            }
            PGAScene::PROJECT_LINE_ONTO_PLANE => {
                let p0 = &scene.points[0];
                let p1 = &scene.points[1];
                let plane0 = &scene.planes[0];

                // Output
                if let Some(line) = (p0 & p1).value() {
                    scene.lines[0] = line.clone();
                    if let Some(line) = (plane0 | line).and_then(|p| (p ^ plane0).value()) {
                        scene.lines[1] = line;
                    }
                }
            }
            _ => { /* Empty scene or unrecognized scene name */ }
        }
    }
}
