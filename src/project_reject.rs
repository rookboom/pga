use std::ops::{BitOr, Mul};

use crate::{Line, Plane, Point, pga3d::PGA3D};

impl BitOr<Line> for Plane {
    type Output = Option<Plane>;

    fn bitor(self, b: Line) -> Option<Plane> {
        let obj = self.0 | b.0;
        if obj == PGA3D::zero() {
            None
        } else {
            Some(Plane(obj))
        }
    }
}

impl BitOr<Point> for Plane {
    type Output = Line;

    fn bitor(self, b: Point) -> Line {
        Line(self.0 | b.0)
    }
}

impl BitOr<Point> for Line {
    type Output = Plane;

    fn bitor(self, b: Point) -> Plane {
        Plane(self.0 | b.0)
    }
}

impl Mul<Point> for Line {
    type Output = Plane;

    fn mul(self, b: Point) -> Plane {
        Plane(self.0 * b.0)
    }
}

impl Mul<Plane> for Line {
    type Output = Point;

    fn mul(self, b: Plane) -> Point {
        Point(self.0 * b.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plane_rejects_line_in_a_perpendicular_plane() {
        let left = Plane::LEFT;
        let plane: Plane = (left | Line::Z_AXIS).unwrap();
        assert_eq!(plane, Plane::UP);
    }

    #[test]
    fn plane_rejects_perpendicular_line_as_zero() {
        let left = Plane::LEFT;
        let plane = left | Line::X_AXIS;
        assert!(plane.is_none());
    }

    #[test]
    fn plane_rejects_point_in_a_perpendicular_line() {
        let left = Plane::LEFT;
        let point = Point::X1;
        let line: Line = left | point;
        assert_eq!(line, Line::X_AXIS);
    }

    #[test]
    fn line_rejects_point_in_a_perpendicular_plane() {
        let forward = Line::Z_AXIS;
        let plane: Plane = forward | Point::X1;
        assert_eq!(plane, -Plane::FORWARD);
    }

    #[test]
    fn project_plane_onto_point() {
        let left = Plane::LEFT;
        let point = Point::X1;

        let plane: Plane = (left | point) * point;
        assert_eq!(plane, Plane::new(-1.0, 0.0, 0.0, 1.0)); // Note the direction of the resulting plane changed from the input plane.
    }

    #[test]
    fn project_point_onto_plane() {
        let plane = Plane::new(1.0, 0.0, 0.0, 0.0);
        let point = Point::new(3.0, 0.0, 0.0);
        let projected_point: Point = (plane | point) * plane;

        assert_eq!(projected_point, Point::new(0.0, 0.0, 0.0));
    }
}
