mod pga3d;

use std::ops::{BitAnd, BitXor};

use pga3d::PGA3D;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point(PGA3D);
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Direction(PGA3D);
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line(PGA3D);
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane(PGA3D);

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point(PGA3D::point(x, y, z))
    }
}

// Join
impl BitAnd for Point {
    type Output = Option<Line>;

    fn bitand(self, b: Point) -> Option<Line> {
        let obj = self.0 & b.0;
        if obj == PGA3D::zero() {
            None
        } else {
            Some(Line(obj))
        }
    }
}
// Join
impl BitAnd<Point> for Option<Line> {
    type Output = Option<Plane>;

    fn bitand(self, b: Point) -> Option<Plane> {
        match self {
            Some(line) => {
                let obj = line.0 & b.0;
                if obj == PGA3D::zero() {
                    None
                } else {
                    Some(Plane(obj))
                }
            }
            None => None,
        }
    }
}

// Wedge
// The outer product. (MEET)
impl BitXor for Plane {
    type Output = Option<Line>;

    fn bitxor(self, b: Plane) -> Option<Line> {
        let obj = self.0 ^ b.0;
        if obj == PGA3D::zero() {
            None
        } else {
            Some(Line(obj))
        }
    }
}

// Wedge
// The outer product. (MEET)
impl BitXor<Plane> for Option<Line> {
    type Output = Option<Point>;

    fn bitxor(self, b: Plane) -> Option<Point> {
        match self {
            Some(line) => Some(Point(line.0 ^ b.0)),
            None => None,
        }
    }
}

// Wedge
// The outer product. (MEET)
impl BitXor<Line> for Plane {
    type Output = Point;

    fn bitxor(self, b: Line) -> Point {
        Point(self.0 ^ b.0)
    }
}

impl Plane {
    const e1: Plane = Plane(PGA3D::new(1.0, 2));
    const e2: Plane = Plane(PGA3D::new(1.0, 3));
    const e3: Plane = Plane(PGA3D::new(1.0, 4));
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Plane(PGA3D::plane(a, b, c, d))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let p0 = Point::new(0.0, 0.0, 0.0);
        let p1 = Point::new(1.0, 0.0, 0.0);
        let p2 = Point::new(0.0, 1.0, 0.0);
        let p3 = Point::new(0.0, 0.0, 1.0);
        let line1: Option<Line> = p1 & p2; // Create line by joining two points
        assert!(line1.is_some());
        let degenerate_line1: Option<Line> = p1 & p1; // Joining a point with itself is zero
        assert!(degenerate_line1.is_none());
        let plane: Option<Plane> = p1 & p2 & p3; // Create plane by joining three points
        assert!(plane.is_some());
        let degenerate_plane1: Option<Plane> = p1 & p2 & p2; // Joining two identical points is zero
        assert!(degenerate_plane1.is_none());
        let origin: Option<Point> = Plane::e1 ^ Plane::e2 ^ Plane::e3; // Create a point by meeting three planes
        assert!(origin.is_some());
        let line = Plane::e1 ^ Plane::e2; // Create a line by meeting two planes
        assert!(line.is_some());
        let origin2 = line ^ Plane::e3; // Create a point by meeting a line and a plane
        assert!(origin2.is_some());
        assert_eq!(origin, origin2);
        assert_eq!(origin, Some(p0));
        let plane2 = line1 & p3; // Create a plane joining a line and a point
        assert!(plane2.is_some());
        let plane3 = line1 & p1; // If the point is co-linear with the line, the result is zero
        assert!(plane3.is_none());
    }
}
