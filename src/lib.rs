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

use crate::pga3d::I;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point(PGA3D);
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Direction(PGA3D);
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line(PGA3D);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane(PGA3D);

impl Point {
    const origin: Point = Point(pga3d::e123);
    const x1: Point = Point(pga3d::e123.with(13, 1.0));
    const y1: Point = Point(pga3d::e123.with(12, 1.0));
    const z1: Point = Point(pga3d::e123.with(11, 1.0));

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point(PGA3D::point(x, y, z))
    }
}

impl Plane {
    const e1: Plane = Plane(PGA3D::new(1.0, 2));
    const e2: Plane = Plane(PGA3D::new(1.0, 3));
    const e3: Plane = Plane(PGA3D::new(1.0, 4));
    const left: Plane = Self::e1;
    const up: Plane = Self::e2;
    const forward: Plane = Self::e3;
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Plane(PGA3D::plane(a, b, c, d))
    }

    pub fn perpendicular_direction(&self) -> Direction {
        Direction(self.0 | I)
    }
}

impl Line {
    const z_axis: Line = Line(pga3d::e12);
    const x_axis: Line = Line(pga3d::e23);
    const y_axis: Line = Line(pga3d::e31);

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
        Line(self.0 | I)
    }
}

/// Right-handed y-up coordinate system
impl Direction {
    const up: Direction = Direction(pga3d::e013);
    const left: Direction = Direction(pga3d::e032);
    const forward: Direction = Direction(pga3d::e021);

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
        let line = (Plane::new(0.0, 0.0, 1.0, 1.0) ^ Plane::up).unwrap();
        assert_eq!(Line::ideal(0.0, 0.0, 1.0), line.perpendicular_line());
        let line = (Plane::forward ^ Plane::up).unwrap();
        assert_eq!(Line::ideal(0.0, 0.0, 1.0), line.perpendicular_line());
    }

    #[test]
    fn line_axes() {
        assert_eq!(Line::z_axis, (Plane::left ^ Plane::up).unwrap());
        assert_eq!(Line::y_axis, (Plane::forward ^ Plane::left).unwrap());
        assert_eq!(Line::x_axis, (Plane::up ^ Plane::forward).unwrap());
    }
}
