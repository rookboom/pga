use std::ops::{BitAnd, BitXor, Neg};

use glam::{Vec3, Vec4};

// pub mod rigid3d_deprecated;

// ================================================================================================
// TYPES
// ================================================================================================
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Point {
    pub direction: Direction,
    pub origin: Origin,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Line {
    pub direction: LineDirection,
    pub moment: LineMoment,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Plane {
    pub direction: PlaneDirection,
    pub horizon: Horizon,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Direction(Vec3);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct LineDirection(Vec3);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct LineMoment(Vec3);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct PlaneDirection(Vec3);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Origin(f32);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Horizon(f32);

// ================================================================================================
// TRAITS
// ================================================================================================

pub trait Join<Rhs = Self> {
    type Output;
    fn join(&self, rhs: &Rhs) -> Self::Output;
}

// ================================================================================================
// IMPLEMENTATIONS
// ================================================================================================
impl Point {
    #[inline]
    pub fn x(&self) -> f32 {
        self.direction.0.x
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.direction.0.y
    }

    #[inline]
    pub fn z(&self) -> f32 {
        self.direction.0.z
    }
    #[inline]
    pub fn w(&self) -> f32 {
        self.origin.0
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point {
            direction: Direction(Vec3::new(x, y, z)),
            origin: Origin(1.0),
        }
    }

    pub fn infinate(x: f32, y: f32, z: f32) -> Self {
        Point {
            direction: Direction(Vec3::new(x, y, z)),
            origin: Origin(0.0),
        }
    }

    pub fn bulk(&self) -> Direction {
        self.direction
    }

    pub fn weight(&self) -> Origin {
        self.origin
    }

    pub fn bulk_dual(&self) -> PlaneDirection {
        PlaneDirection(self.direction.0)
    }

    pub fn weight_dual(&self) -> Horizon {
        Horizon(self.origin.0)
    }

    pub fn is_valid(&self) -> bool {
        self.origin.is_valid()
    }

    pub fn is_zero(&self) -> bool {
        self.direction.is_zero() && self.origin.is_zero()
    }

    pub fn project(&self) -> Vec3 {
        if self.origin.is_zero() {
            self.direction.0
        } else {
            self.direction.0 / self.origin.0
        }
    }

    pub fn unitize(&mut self) -> &mut Self {
        self.direction = Direction(self.project());
        self.origin = Origin(1.0);
        self
    }
}

impl Line {
    pub const X_AXIS: Line = Line {
        direction: LineDirection(Vec3::new(1.0, 0.0, 0.0)),
        moment: LineMoment(Vec3::new(0.0, 0.0, 0.0)),
    };
    pub const Y_AXIS: Line = Line {
        direction: LineDirection(Vec3::new(0.0, 1.0, 0.0)),
        moment: LineMoment(Vec3::new(0.0, 0.0, 0.0)),
    };
    pub const Z_AXIS: Line = Line {
        direction: LineDirection(Vec3::new(0.0, 0.0, 1.0)),
        moment: LineMoment(Vec3::new(0.0, 0.0, 0.0)),
    };

    pub fn through_origin(x: f32, y: f32, z: f32) -> Self {
        Line {
            direction: LineDirection(Vec3::new(x, y, z)),
            moment: LineMoment(Vec3::ZERO),
        }
    }

    pub fn bulk(&self) -> LineMoment {
        self.moment
    }

    pub fn weight(&self) -> LineDirection {
        self.direction
    }

    pub fn is_valid(&self) -> bool {
        self.direction.is_valid()
    }

    pub fn is_zero(&self) -> bool {
        self.direction.is_zero() && self.moment.is_zero()
    }

    pub fn unitize(&mut self) -> &mut Self {
        let inv_mag = 1.0 / self.direction.0.length();
        self.direction.0 *= inv_mag;
        self.moment.0 *= inv_mag;
        self
    }
}

impl Plane {
    pub const LEFT: Plane = Plane {
        direction: PlaneDirection(Vec3::new(1.0, 0.0, 0.0)),
        horizon: Horizon(0.0),
    };

    pub const UP: Plane = Plane {
        direction: PlaneDirection(Vec3::new(0.0, 1.0, 0.0)),
        horizon: Horizon(0.0),
    };
    pub const FORWARD: Plane = Plane {
        direction: PlaneDirection(Vec3::new(0.0, 0.0, 1.0)),
        horizon: Horizon(0.0),
    };

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Plane {
            direction: PlaneDirection(Vec3::new(x, y, z)),
            horizon: Horizon(w),
        }
    }

    pub fn from_normal_distance(normal: Vec3, distance: f32) -> Self {
        Plane {
            direction: PlaneDirection(normal),
            horizon: Horizon(distance),
        }
    }

    pub fn from_normal_point(normal: Vec3, point: Vec3) -> Self {
        let normal = normal.normalize();
        Self {
            direction: PlaneDirection(normal),
            horizon: Horizon(-(normal.x * point.x + normal.y * point.y + normal.z * point.z)),
        }
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.direction.0.x
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.direction.0.y
    }

    #[inline]
    pub fn z(&self) -> f32 {
        self.direction.0.z
    }

    #[inline]
    pub fn w(&self) -> f32 {
        self.horizon.0
    }

    pub fn bulk(&self) -> Horizon {
        self.horizon
    }

    pub fn weight(&self) -> PlaneDirection {
        self.direction
    }

    pub fn is_valid(&self) -> bool {
        self.direction.is_valid()
    }

    pub fn is_zero(&self) -> bool {
        self.direction.is_zero() && self.horizon.is_zero()
    }

    /// Unitizes the plane (normalizes the normal vector)
    pub fn unitize(&mut self) -> &mut Self {
        *self = self.unitized();
        self
    }

    /// Unitizes the plane (normalizes the normal vector)
    pub fn unitized(&self) -> Self {
        let inv_mag = 1.0 / self.direction.norm();
        Plane {
            direction: PlaneDirection(self.direction.0 * inv_mag),
            horizon: Horizon(self.horizon.0 * inv_mag),
        }
    }
}

impl Direction {
    #[inline]
    pub fn x(&self) -> f32 {
        self.0.x
    }
    #[inline]
    pub fn y(&self) -> f32 {
        self.0.y
    }
    #[inline]
    pub fn z(&self) -> f32 {
        self.0.z
    }
    const ZERO: Direction = Direction(Vec3::ZERO);

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Direction(Vec3::new(x, y, z))
    }

    pub fn norm(&self) -> f32 {
        self.0.length()
    }

    pub fn dual(&self) -> PlaneDirection {
        PlaneDirection(self.0)
    }

    pub fn is_valid(&self) -> bool {
        self.norm() > f32::EPSILON
    }

    pub fn is_zero(&self) -> bool {
        self.norm() <= f32::EPSILON
    }

    pub fn unitize(&mut self) -> &mut Self {
        self.0 = self.0.normalize();
        self
    }
}

impl Origin {
    const ZERO: Origin = Origin(0.0);

    pub fn norm(&self) -> f32 {
        self.0.abs()
    }

    pub fn dual(&self) -> Horizon {
        Horizon(self.0)
    }

    pub fn is_valid(&self) -> bool {
        self.norm() > f32::EPSILON
    }

    pub fn is_zero(&self) -> bool {
        self.norm() <= f32::EPSILON
    }
}

impl Horizon {
    const ZERO: Horizon = Horizon(0.0);
    pub fn norm(&self) -> f32 {
        self.0.abs()
    }

    pub fn dual(&self) -> Origin {
        Origin(-self.0)
    }

    pub fn is_valid(&self) -> bool {
        self.norm() > f32::EPSILON
    }

    pub fn is_zero(&self) -> bool {
        self.norm() <= f32::EPSILON
    }
}

impl PlaneDirection {
    #[inline]
    pub fn x(&self) -> f32 {
        self.0.x
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.0.y
    }

    #[inline]
    pub fn z(&self) -> f32 {
        self.0.z
    }

    const ZERO: PlaneDirection = PlaneDirection(Vec3::ZERO);
    pub fn norm(&self) -> f32 {
        self.0.length()
    }

    pub fn dual(&self) -> Direction {
        Direction(-self.0)
    }

    pub fn is_valid(&self) -> bool {
        self.norm() > f32::EPSILON
    }

    pub fn is_zero(&self) -> bool {
        self.norm() <= f32::EPSILON
    }
}

impl LineMoment {
    #[inline]
    pub fn x(&self) -> f32 {
        self.0.x
    }
    #[inline]
    pub fn y(&self) -> f32 {
        self.0.y
    }
    #[inline]
    pub fn z(&self) -> f32 {
        self.0.z
    }
    pub fn dual(&self) -> LineDirection {
        LineDirection(-self.0)
    }

    pub fn is_valid(&self) -> bool {
        self.0.length() > f32::EPSILON
    }

    pub fn is_zero(&self) -> bool {
        self.0.length() <= f32::EPSILON
    }
}

impl LineDirection {
    #[inline]
    pub fn x(&self) -> f32 {
        self.0.x
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.0.y
    }

    #[inline]
    pub fn z(&self) -> f32 {
        self.0.z
    }

    pub fn dual(&self) -> LineMoment {
        LineMoment(-self.0)
    }

    pub fn is_valid(&self) -> bool {
        self.0.length() > f32::EPSILON
    }

    pub fn is_zero(&self) -> bool {
        self.0.length() <= f32::EPSILON
    }
}

// Wedge operations (Join)
impl BitXor<Point> for Point {
    type Output = Line;
    fn bitxor(self, other: Point) -> Self::Output {
        Line {
            direction: LineDirection(Vec3::new(
                self.w() * other.x() - self.x() * other.w(),
                self.w() * other.y() - self.y() * other.w(),
                self.w() * other.z() - self.z() * other.w(),
            )),
            moment: LineMoment(Vec3::new(
                self.y() * other.z() - self.z() * other.y(),
                self.z() * other.x() - self.x() * other.z(),
                self.x() * other.y() - self.y() * other.x(),
            )),
        }
    }
}

impl BitXor<Direction> for Point {
    type Output = Line;
    fn bitxor(self, other: Direction) -> Self::Output {
        Line {
            direction: LineDirection(Vec3::new(
                self.w() * other.x(),
                self.w() * other.y(),
                self.w() * other.z(),
            )),
            moment: LineMoment(Vec3::new(
                self.y() * other.z() - self.z() * other.y(),
                self.z() * other.x() - self.x() * other.z(),
                self.x() * other.y() - self.y() * other.x(),
            )),
        }
    }
}

impl BitXor<Point> for Line {
    type Output = Plane;
    fn bitxor(self, other: Point) -> Self::Output {
        Plane {
            direction: PlaneDirection(Vec3::new(
                self.direction.y() * other.direction.z() - self.direction.z() * other.direction.y()
                    + self.moment.x() * other.origin.0,
                self.direction.z() * other.direction.x() - self.direction.x() * other.direction.z()
                    + self.moment.y() * other.origin.0,
                self.direction.x() * other.direction.y() - self.direction.y() * other.direction.x()
                    + self.moment.z() * other.origin.0,
            )),
            horizon: Horizon(
                -self.moment.x() * other.direction.x()
                    - self.moment.y() * other.direction.y()
                    - self.moment.z() * other.direction.z(),
            ),
        }
    }
}

impl BitXor<Point> for LineMoment {
    type Output = Plane;
    fn bitxor(self, other: Point) -> Self::Output {
        Plane {
            direction: PlaneDirection(Vec3::new(
                self.x() * other.origin.0,
                self.y() * other.origin.0,
                self.z() * other.origin.0,
            )),
            horizon: Horizon(
                -self.x() * other.direction.x()
                    - self.y() * other.direction.y()
                    - self.z() * other.direction.z(),
            ),
        }
    }
}

impl BitXor<LineMoment> for Point {
    type Output = Plane;
    fn bitxor(self, other: LineMoment) -> Self::Output {
        -(other ^ self)
    }
}

impl BitXor<Direction> for Line {
    type Output = Plane;
    fn bitxor(self, other: Direction) -> Self::Output {
        Plane {
            direction: PlaneDirection(Vec3::new(
                self.direction.y() * other.z() - self.direction.z() * other.y(),
                self.direction.z() * other.x() - self.direction.x() * other.z(),
                self.direction.x() * other.y() - self.direction.y() * other.x(),
            )),
            horizon: Horizon(
                -self.moment.x() * other.x()
                    - self.moment.y() * other.y()
                    - self.moment.z() * other.z(),
            ),
        }
    }
}

// Meet operations (Antiwedge)
impl BitAnd<Plane> for Plane {
    type Output = Line;
    fn bitand(self, other: Plane) -> Self::Output {
        Line {
            direction: LineDirection(Vec3::new(
                self.z() * other.y() - self.y() * other.z(),
                self.x() * other.z() - self.z() * other.x(),
                self.y() * other.x() - self.x() * other.y(),
            )),
            moment: LineMoment(Vec3::new(
                self.x() * other.w() - self.w() * other.x(),
                self.y() * other.w() - self.w() * other.y(),
                self.z() * other.w() - self.w() * other.z(),
            )),
        }
    }
}

impl BitAnd<Line> for Plane {
    type Output = Point;
    fn bitand(self, line: Line) -> Self::Output {
        Point {
            direction: Direction(Vec3::new(
                line.moment.y() * self.z() - line.moment.z() * self.y()
                    + line.direction.x() * self.w(),
                line.moment.z() * self.x() - line.moment.x() * self.z()
                    + line.direction.y() * self.w(),
                line.moment.x() * self.y() - line.moment.y() * self.x()
                    + line.direction.z() * self.w(),
            )),
            origin: Origin(
                -line.direction.x() * self.x()
                    - line.direction.y() * self.y()
                    - line.direction.z() * self.z(),
            ),
        }
    }
}

impl BitAnd<Plane> for Line {
    type Output = Point;
    fn bitand(self, line: Plane) -> Self::Output {
        -(line & self)
    }
}

impl Neg for Plane {
    type Output = Plane;
    fn neg(self) -> Self::Output {
        Plane {
            direction: PlaneDirection(-self.direction.0),
            horizon: Horizon(-self.horizon.0),
        }
    }
}

impl Neg for Line {
    type Output = Line;
    fn neg(self) -> Self::Output {
        Line {
            direction: LineDirection(-self.direction.0),
            moment: LineMoment(-self.moment.0),
        }
    }
}

impl Neg for Point {
    type Output = Point;
    fn neg(self) -> Self::Output {
        Point {
            direction: Direction(-self.direction.0),
            origin: Origin(-self.origin.0),
        }
    }
}

// From implementations for conversions
impl From<Direction> for Vec3 {
    fn from(v: Direction) -> Self {
        v.0
    }
}

impl From<PlaneDirection> for Vec3 {
    fn from(v: PlaneDirection) -> Self {
        v.0
    }
}

impl From<LineDirection> for Vec3 {
    fn from(v: LineDirection) -> Self {
        v.0
    }
}

impl From<LineMoment> for Vec3 {
    fn from(v: LineMoment) -> Self {
        v.0
    }
}

impl From<Plane> for Vec4 {
    fn from(p: Plane) -> Self {
        Vec4::new(p.x(), p.y(), p.z(), p.w())
    }
}

impl From<Vec3> for Point {
    fn from(v: Vec3) -> Self {
        Point {
            direction: Direction(v),
            origin: Origin(1.0),
        }
    }
}

impl From<[f32; 3]> for Direction {
    fn from(v: [f32; 3]) -> Self {
        Direction(Vec3::from(v))
    }
}

impl From<[f32; 3]> for PlaneDirection {
    fn from(v: [f32; 3]) -> Self {
        PlaneDirection(Vec3::from(v))
    }
}

impl From<[f32; 3]> for Point {
    fn from(v: [f32; 3]) -> Self {
        Point {
            direction: Direction(Vec3::from(v)),
            origin: Origin(1.0),
        }
    }
}

impl From<Vec4> for Point {
    fn from(v: Vec4) -> Self {
        Point {
            direction: Direction(Vec3::new(v.x, v.y, v.z)),
            origin: Origin(v.w),
        }
    }
}

impl From<Vec3> for Direction {
    fn from(v: Vec3) -> Self {
        Direction(v)
    }
}
impl From<Vec3> for PlaneDirection {
    fn from(v: Vec3) -> Self {
        PlaneDirection(v)
    }
}

impl From<Vec3> for LineDirection {
    fn from(v: Vec3) -> Self {
        LineDirection(v)
    }
}

impl From<Vec3> for LineMoment {
    fn from(v: Vec3) -> Self {
        LineMoment(v)
    }
}

impl From<f32> for Origin {
    fn from(v: f32) -> Self {
        Origin(v)
    }
}

impl From<f32> for Horizon {
    fn from(v: f32) -> Self {
        Horizon(v)
    }
}

impl From<Origin> for f32 {
    fn from(v: Origin) -> Self {
        v.0
    }
}

impl From<Horizon> for f32 {
    fn from(v: Horizon) -> Self {
        v.0
    }
}
