#![allow(dead_code)]
mod incidence;
mod pga3d;
mod project_reject;

#[cfg(feature = "visualization")]
pub mod visualization;

// WASM entry point
#[cfg(all(target_arch = "wasm32", feature = "web"))]
mod web;

#[cfg(all(target_arch = "wasm32", feature = "web"))]
pub use web::*;

use std::ops::Neg;

use pga3d::PGA3D;

use crate::pga3d::{Direction, Line, Plane, Point};

/// Trait for approximate equality comparisons, useful for floating-point tests
pub trait ApproxEq {
    /// Check if two values are approximately equal within a default epsilon
    fn approx_eq(&self, other: &Self) -> bool;

    /// Check if two values are approximately equal within a specified epsilon
    fn approx_eq_eps(&self, other: &Self, epsilon: f32) -> bool;
}

impl ApproxEq for f32 {
    fn approx_eq(&self, other: &Self) -> bool {
        self.approx_eq_eps(other, 1e-6)
    }

    fn approx_eq_eps(&self, other: &Self, epsilon: f32) -> bool {
        (self - other).abs() < epsilon
    }
}

impl ApproxEq for PGA3D {
    fn approx_eq(&self, other: &Self) -> bool {
        self.approx_eq_eps(other, 1e-6)
    }

    fn approx_eq_eps(&self, other: &Self, epsilon: f32) -> bool {
        self.mvec
            .iter()
            .zip(other.mvec.iter())
            .all(|(a, b)| a.approx_eq_eps(b, epsilon))
    }
}

impl ApproxEq for Point {
    fn approx_eq(&self, other: &Self) -> bool {
        self.0.approx_eq(&other.0)
    }

    fn approx_eq_eps(&self, other: &Self, epsilon: f32) -> bool {
        self.0.approx_eq_eps(&other.0, epsilon)
    }
}

impl ApproxEq for Line {
    fn approx_eq(&self, other: &Self) -> bool {
        self.0.approx_eq(&other.0)
    }

    fn approx_eq_eps(&self, other: &Self, epsilon: f32) -> bool {
        self.0.approx_eq_eps(&other.0, epsilon)
    }
}

impl ApproxEq for Plane {
    fn approx_eq(&self, other: &Self) -> bool {
        self.0.approx_eq(&other.0)
    }

    fn approx_eq_eps(&self, other: &Self, epsilon: f32) -> bool {
        self.0.approx_eq_eps(&other.0, epsilon)
    }
}

impl ApproxEq for Direction {
    fn approx_eq(&self, other: &Self) -> bool {
        self.0.approx_eq(&other.0)
    }

    fn approx_eq_eps(&self, other: &Self, epsilon: f32) -> bool {
        self.0.approx_eq_eps(&other.0, epsilon)
    }
}

/// Macro for asserting approximate equality in tests
#[macro_export]
macro_rules! assert_approx_eq {
    ($left:expr, $right:expr) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !left_val.approx_eq(right_val) {
                    panic!(
                        "assertion failed: `(left approx== right)`\n  left: `{:?}`\n right: `{:?}`",
                        left_val, right_val
                    );
                }
            }
        }
    };
    ($left:expr, $right:expr, $epsilon:expr) => {
        match (&$left, &$right, &$epsilon) {
            (left_val, right_val, epsilon_val) => {
                if !left_val.approx_eq_eps(right_val, *epsilon_val) {
                    panic!(
                        "assertion failed: `(left approx== right)` with epsilon `{}`\n  left: `{:?}`\n right: `{:?}`",
                        epsilon_val, left_val, right_val
                    );
                }
            }
        }
    };
}

/// Macro for asserting approximate inequality in tests
#[macro_export]
macro_rules! assert_approx_ne {
    ($left:expr, $right:expr) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if left_val.approx_eq(right_val) {
                    panic!(
                        "assertion failed: `(left approx!= right)`\n  left: `{:?}`\n right: `{:?}`",
                        left_val, right_val
                    );
                }
            }
        }
    };
    ($left:expr, $right:expr, $epsilon:expr) => {
        match (&$left, &$right, &$epsilon) {
            (left_val, right_val, epsilon_val) => {
                if left_val.approx_eq_eps(right_val, *epsilon_val) {
                    panic!(
                        "assertion failed: `(left approx!= right)` with epsilon `{}`\n  left: `{:?}`\n right: `{:?}`",
                        epsilon_val, left_val, right_val
                    );
                }
            }
        }
    };
}

impl Point {
    const ORIGIN: Point = Point(PGA3D::e123());
    const X1: Point = Point(PGA3D::e123().with(13, 1.0));
    const Y1: Point = Point(PGA3D::e123().with(12, 1.0));
    const Z1: Point = Point(PGA3D::e123().with(11, 1.0));

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point(PGA3D::point(x, y, z))
    }
}

impl Plane {
    const E1: Plane = Plane(PGA3D::new(1.0, 2));
    const E2: Plane = Plane(PGA3D::new(1.0, 3));
    const E3: Plane = Plane(PGA3D::new(1.0, 4));
    const LEFT: Plane = Self::E1;
    const UP: Plane = Self::E2;
    const FORWARD: Plane = Self::E3;
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Plane(PGA3D::plane(a, b, c, d))
    }

    pub fn perpendicular_direction(&self) -> Direction {
        Direction(&self.0 | PGA3D::e0123())
    }
}

impl Line {
    const Z_AXIS: Line = Line(PGA3D::e12());
    const X_AXIS: Line = Line(PGA3D::e23());
    const Y_AXIS: Line = Line(PGA3D::e31());

    pub fn new(
        dir_x: f32,
        dir_y: f32,
        dir_z: f32,
        moment_x: f32,
        moment_y: f32,
        moment_z: f32,
    ) -> Self {
        let mut line = PGA3D::zero();
        // pub const e01: PGA3D = PGA3D::new(1.0, 5);
        // pub const e02: PGA3D = PGA3D::new(1.0, 6);
        // pub const e03: PGA3D = PGA3D::new(1.0, 7);
        // pub const e12: PGA3D = PGA3D::new(1.0, 8);
        // pub const e31: PGA3D = PGA3D::new(1.0, 9);
        // pub const e23: PGA3D = PGA3D::new(1.0, 10);
        line.mvec[5] = moment_z; // e01 / z
        line.mvec[6] = moment_x; // e02 / x
        line.mvec[7] = moment_y; // e03 / y
        line.mvec[8] = dir_z; // e12 / z
        line.mvec[9] = dir_x; // e31 / x
        line.mvec[10] = dir_y; // e23 / y
        Line(line)
    }

    pub fn ideal(moment_x: f32, moment_y: f32, moment_z: f32) -> Self {
        Line::new(0.0, 0.0, 0.0, moment_x, moment_y, moment_z)
    }

    pub fn through_origin(dir_x: f32, dir_y: f32, dir_z: f32) -> Self {
        Line::new(dir_x, dir_y, dir_z, 0.0, 0.0, 0.0)
    }

    pub fn with_moment(&self, moment_x: f32, moment_y: f32, moment_z: f32) -> Self {
        Line::new(
            self.0.mvec[8],
            self.0.mvec[9],
            self.0.mvec[10],
            moment_z,
            moment_x,
            moment_y,
        )
    }

    pub fn with_direction(&self, dir_x: f32, dir_y: f32, dir_z: f32) -> Self {
        Line::new(
            dir_x,
            dir_y,
            dir_z,
            self.0.mvec[5],
            self.0.mvec[6],
            self.0.mvec[7],
        )
    }

    pub fn perpendicular_line(&self) -> Line {
        Line(&self.0 | PGA3D::e0123())
    }
}

/// Right-handed y-up coordinate system
impl Direction {
    const UP: Direction = Direction(PGA3D::e013());
    const LEFT: Direction = Direction(PGA3D::e032());
    const FORWARD: Direction = Direction(PGA3D::e021());

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Direction(PGA3D::direction(x, y, z))
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        let mut obj = self.0;
        obj.mvec[11] = -obj.mvec[11];
        obj.mvec[12] = -obj.mvec[12];
        obj.mvec[13] = -obj.mvec[13];

        Point(obj)
    }
}

impl Neg for Plane {
    type Output = Plane;

    fn neg(self) -> Plane {
        let mut obj = self.0;
        obj.mvec[1] = -obj.mvec[1];
        obj.mvec[2] = -obj.mvec[2];
        obj.mvec[3] = -obj.mvec[3];
        obj.mvec[4] = -obj.mvec[4];
        Plane(obj)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_to_plane() {
        let plane = Plane::new(1.0, 0.0, 0.0, 1.0);
        assert_eq!(
            Direction::new(1.0, 0.0, 0.0),
            plane.perpendicular_direction()
        );
        let plane = Plane::new(1.0, 0.0, 0.0, 0.0);
        assert_eq!(
            Direction::new(1.0, 0.0, 0.0),
            plane.perpendicular_direction()
        );
        let plane = Plane::new(-1.0, -1.0, 0.0, 0.0);
        assert_eq!(
            Direction::new(-1.0, -1.0, 0.0),
            plane.perpendicular_direction()
        );
        let plane = Plane::new(-1.0, -1.0, 0.0, 1.0);
        assert_eq!(
            Direction::new(-1.0, -1.0, 0.0),
            plane.perpendicular_direction()
        );
    }

    #[test]
    fn ideal_line_perpendicular_to_line() {
        let line = (Plane::new(0.0, 0.0, 1.0, 1.0) ^ Plane::UP).unwrap();
        assert_eq!(Line::ideal(0.0, 0.0, 1.0), line.perpendicular_line());
        let line = (Plane::FORWARD ^ Plane::UP).unwrap();
        assert_eq!(Line::ideal(0.0, 0.0, 1.0), line.perpendicular_line());
    }

    #[test]
    fn line_axes() {
        assert_eq!(Line::Z_AXIS, (Plane::LEFT ^ Plane::UP).unwrap());
        assert_eq!(Line::Y_AXIS, (Plane::FORWARD ^ Plane::LEFT).unwrap());
        assert_eq!(Line::X_AXIS, (Plane::UP ^ Plane::FORWARD).unwrap());
    }
}
