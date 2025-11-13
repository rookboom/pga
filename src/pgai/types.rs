use crate::{
    impl_geometric_entity,
    pgai::{BulkWeight, Dual},
};
use glam::{Vec3, Vec4};
use std::ops::{Neg, Not};

// ================================================================================================
// GEOMETRIC ENTITIES
// ================================================================================================
#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct Point4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct Line {
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
    pub mx: f32,
    pub my: f32,
    pub mz: f32,
}

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct Plane {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct Direction {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct LineDirection {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct LineMoment {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct PlaneDirection {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct Origin {
    pub w: f32,
}

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct Horizon {
    pub w: f32,
}

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PointOrDirection {
    Point(Point3),
    Direction(Direction),
}

// ================================================================================================
// IMPLEMENTATIONS USING MACRO
// ================================================================================================

fn neg<T: crate::pgai::GeometricEntity>(value: T) -> T {
    let mut result = T::default();
    result.set_e0(-value.e0());
    result.set_e1(-value.e1());
    result.set_e2(-value.e2());
    result.set_e3(-value.e3());
    result.set_e41(-value.e41());
    result.set_e42(-value.e42());
    result.set_e43(-value.e43());
    result.set_e23(-value.e23());
    result.set_e31(-value.e31());
    result.set_e12(-value.e12());
    result.set_e423(-value.e423());
    result.set_e431(-value.e431());
    result.set_e412(-value.e412());
    result.set_e321(-value.e321());
    result.set_scalar(-value.scalar());
    result.set_antiscalar(-value.antiscalar());
    result
}

macro_rules! neg_geometric_entity {
    ($t:ty) => {
        impl Neg for $t {
            type Output = $t;

            fn neg(self) -> Self::Output {
                neg(self)
            }
        }
    };
}

macro_rules! geometric_entity_dual {
    ($t:ty, $d:ty) => {
        impl Not for $t {
            type Output = $d;

            fn not(self) -> Self::Output {
                self.dual()
            }
        }
        impl Not for $d {
            type Output = $t;

            fn not(self) -> Self::Output {
                self.dual()
            }
        }
    };
}

neg_geometric_entity!(Point3);
neg_geometric_entity!(Point4);
neg_geometric_entity!(Line);
neg_geometric_entity!(Plane);
neg_geometric_entity!(PlaneDirection);
neg_geometric_entity!(LineDirection);
neg_geometric_entity!(LineMoment);
neg_geometric_entity!(Direction);
neg_geometric_entity!(Horizon);
neg_geometric_entity!(Origin);

geometric_entity_dual!(Direction, PlaneDirection);
geometric_entity_dual!(LineDirection, LineMoment);
geometric_entity_dual!(Horizon, Origin);

impl_geometric_entity!(Point4, [
    e1 => x,
    e2 => y,
    e3 => z,
    e0 => w
]);

impl_geometric_entity!(Line, [
    e41 => vx,
    e42 => vy,
    e43 => vz,
    e23 => mx,
    e31 => my,
    e12 => mz
]);

impl_geometric_entity!(Plane, [
    e423 => x,
    e431 => y,
    e412 => z,
    e321 => w
]);

impl_geometric_entity!(Point3, [
    e0 => 1.0,
    e1 => x,
    e2 => y,
    e3 => z
], fields: [x, y, z]);

impl_geometric_entity!(Direction, [
    e1 => x,
    e2 => y,
    e3 => z
]);

impl_geometric_entity!(LineDirection, [
    e41 => x,
    e42 => y,
    e43 => z
]);

impl_geometric_entity!(LineMoment, [
    e23 => x,
    e31 => y,
    e12 => z
]);

impl_geometric_entity!(PlaneDirection, [
    e423 => x,
    e431 => y,
    e412 => z
]);

impl_geometric_entity!(Horizon, [
    e321 => w
]);

impl_geometric_entity!(Origin, [
    e0 => w
]);

// ================================================================================================
// IMPLEMENTATIONS
// ================================================================================================

impl Line {
    pub const X_AXIS: Line = Line::new(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    pub const Y_AXIS: Line = Line::new(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
    pub const Z_AXIS: Line = Line::new(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);

    pub fn through_origin(x: f32, y: f32, z: f32) -> Self {
        Line::new(x, y, z, 0.0, 0.0, 0.0)
    }
}

impl Plane {
    pub const LEFT: Plane = Plane::new(1.0, 0.0, 0.0, 0.0);
    pub const UP: Plane = Plane::new(0.0, 1.0, 0.0, 0.0);
    pub const FORWARD: Plane = Plane::new(0.0, 0.0, 1.0, 0.0);
}

impl Direction {
    const ZERO: Direction = Direction::new(0.0, 0.0, 0.0);
}

impl Origin {
    const ZERO: Origin = Origin::new(0.0);
}

impl Horizon {
    const ZERO: Horizon = Horizon::new(0.0);
}

impl PlaneDirection {
    const ZERO: PlaneDirection = PlaneDirection::new(0.0, 0.0, 0.0);
}

// ================================================================================================
// TRAIT IMPLEMENTATIONS
// ================================================================================================
impl BulkWeight for Point4 {
    type Bulk = Direction;
    type Weight = Origin;

    fn bulk(&self) -> Direction {
        Direction {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    fn weight(&self) -> Origin {
        Origin { w: self.w }
    }
}

impl BulkWeight for Plane {
    type Bulk = Horizon;
    type Weight = PlaneDirection;

    fn weight(&self) -> PlaneDirection {
        PlaneDirection {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    fn bulk(&self) -> Horizon {
        Horizon { w: self.w }
    }
}

impl BulkWeight for Line {
    type Bulk = LineMoment;
    type Weight = LineDirection;

    fn weight(&self) -> LineDirection {
        LineDirection {
            x: self.vx,
            y: self.vy,
            z: self.vz,
        }
    }

    fn bulk(&self) -> LineMoment {
        LineMoment {
            x: self.mx,
            y: self.my,
            z: self.mz,
        }
    }
}

impl Dual for Direction {
    type DualType = PlaneDirection;
    fn dual(&self) -> Self::DualType {
        PlaneDirection {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl Dual for PlaneDirection {
    type DualType = Direction;
    fn dual(&self) -> Self::DualType {
        Direction {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Dual for LineDirection {
    type DualType = LineMoment;
    fn dual(&self) -> Self::DualType {
        LineMoment {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Dual for LineMoment {
    type DualType = LineDirection;
    fn dual(&self) -> Self::DualType {
        LineDirection {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Dual for Origin {
    type DualType = Horizon;
    fn dual(&self) -> Self::DualType {
        Horizon { w: self.w }
    }
}

impl Dual for Horizon {
    type DualType = Origin;
    fn dual(&self) -> Self::DualType {
        Origin { w: -self.w }
    }
}

// ================================================================================================
// CONVERSIONS
// ================================================================================================

impl From<Point4> for PointOrDirection {
    fn from(p: Point4) -> Self {
        if p.w.abs() < f32::EPSILON {
            PointOrDirection::Direction(p.bulk())
        } else {
            PointOrDirection::Point(p.into())
        }
    }
}

impl From<Point4> for Point3 {
    fn from(p: Point4) -> Self {
        Point3 {
            x: p.x / p.w,
            y: p.y / p.w,
            z: p.z / p.w,
        }
    }
}

impl From<Vec3> for Direction {
    fn from(v: Vec3) -> Self {
        Direction {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl From<Vec3> for LineDirection {
    fn from(v: Vec3) -> Self {
        LineDirection {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl From<Vec3> for LineMoment {
    fn from(v: Vec3) -> Self {
        LineMoment {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl From<LineDirection> for Line {
    fn from(ld: LineDirection) -> Self {
        Line {
            vx: ld.x,
            vy: ld.y,
            vz: ld.z,
            mx: 0.0,
            my: 0.0,
            mz: 0.0,
        }
    }
}

impl From<PlaneDirection> for Vec3 {
    fn from(d: PlaneDirection) -> Self {
        Vec3::new(d.x, d.y, d.z)
    }
}

impl From<LineDirection> for Vec3 {
    fn from(d: LineDirection) -> Self {
        Vec3::new(d.x, d.y, d.z)
    }
}

impl From<Direction> for Vec3 {
    fn from(d: Direction) -> Self {
        Vec3::new(d.x, d.y, d.z)
    }
}

impl From<Point3> for Vec3 {
    fn from(p: Point3) -> Self {
        Vec3::new(p.x, p.y, p.z)
    }
}

impl From<LineMoment> for Vec3 {
    fn from(d: LineMoment) -> Self {
        Vec3::new(d.x, d.y, d.z)
    }
}

impl From<Plane> for Vec4 {
    fn from(p: Plane) -> Self {
        Vec4::new(p.x, p.y, p.z, p.w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_axes() {
        assert_eq!(-Line::Z_AXIS, Plane::LEFT & Plane::UP);
        assert_eq!(-Line::Y_AXIS, Plane::FORWARD & Plane::LEFT);
        assert_eq!(-Line::X_AXIS, Plane::UP & Plane::FORWARD);
    }
}
