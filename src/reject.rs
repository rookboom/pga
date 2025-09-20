use std::ops::BitOr;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plane_rejects_line_in_a_perpendicular_plane() {
        let left = Plane::left;
        let plane: Plane = (left | Line::z_axis).unwrap();
        assert_eq!(plane, Plane::up);
    }

    #[test]
    fn plane_rejects_perpendicular_line_as_zero() {
        let left = Plane::left;
        let plane = left | Line::x_axis;
        assert!(plane.is_none());
    }

    #[test]
    fn plane_rejects_point_in_a_perpendicular_line() {
        let left = Plane::left;
        let point = Point::x1;
        let line: Line = left | point;
        assert_eq!(line, Line::x_axis);
    }

    #[test]
    fn line_rejects_point_in_a_perpendicular_plane() {
        let forward = Line::z_axis;
        let plane: Plane = forward | Point::x1;
        assert_eq!(plane, -Plane::forward);
    }
}
