// Join
use crate::pga3d::{Line, PGA3D, Plane, Point};
use std::ops::{BitAnd, BitXor};

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
// Join
impl BitAnd<Point> for Line {
    type Output = Option<Plane>;

    fn bitand(self, b: Point) -> Option<Plane> {
        let obj = self.0 & b.0;
        if obj == PGA3D::zero() {
            None
        } else {
            Some(Plane(obj))
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
        self.map(|line| Point(line.0 ^ b.0))
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

// Wedge
// The outer product. (MEET)
impl BitXor<Line> for Line {
    type Output = Option<Point>;

    fn bitxor(self, b: Line) -> Option<Point> {
        let obj = self.0 ^ b.0;
        if obj == PGA3D::zero() {
            None
        } else {
            Some(Point(obj))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ApproxEq, assert_approx_eq};

    use super::*;

    #[test]
    fn two_points_join_in_a_line() {
        let p0 = Point::new(0.0, 0.0, 0.0);
        let p1 = Point::new(1.0, 0.0, 0.0);
        let line1: Option<Line> = p0 & p1;
        assert!(line1.is_some());
    }

    #[test]
    fn identical_points_do_not_join_in_a_line() {
        let p1 = &Point::new(1.0, 0.0, 0.0);
        let degenerate_line1: Option<Line> = p1 & p1;
        assert!(degenerate_line1.is_none());
    }

    #[test]
    fn three_points_join_in_a_plane() {
        let p0 = Point::new(0.0, 0.0, 0.0);
        let p1 = Point::new(1.0, 0.0, 0.0);
        let p2 = Point::new(0.0, 1.0, 0.0);
        let plane1: Option<Plane> = p0 & p1 & p2;
        assert!(plane1.is_some());
    }

    #[test]
    fn colinear_points_do_not_join_in_a_plane() {
        let p0 = Point::new(0.0, 0.0, 0.0);
        let p1 = Point::new(1.0, 0.0, 0.0);
        let p2 = Point::new(2.0, 0.0, 0.0);
        let plane1: Option<Plane> = p0 & p1 & p2;
        assert!(plane1.is_none());
    }

    #[test]
    fn two_planes_meet_in_a_line() {
        let line: Option<Line> = Plane::E1 ^ Plane::E2;
        assert!(line.is_some());
    }

    #[test]
    fn three_planes_meet_in_a_point() {
        let origin: Option<Point> = Plane::E1 ^ Plane::E2 ^ Plane::E3;
        assert!(origin.is_some());
        assert_eq!(origin, Some(Point::new(0.0, 0.0, 0.0)));
    }

    #[test]
    fn line_and_plane_meet_in_a_point() {
        let p0 = Point::new(0.0, 0.0, 0.0);
        let p1 = Point::new(1.0, 0.0, 0.0);
        let line: Line = (p0 & p1).unwrap();
        // Use a plane that doesn't pass through the origin: x = 0.5
        let plane = Plane::new(1.0, 0.0, 0.0, -0.5);
        let point = plane ^ line;
        assert_ne!(point.0, PGA3D::zero());
        // The intersection should be at (0.5, 0, 0)
        let expected = Point::new(0.5, 0.0, 0.0);
        assert_approx_eq!(point, expected);
    }

    #[test]
    fn line_and_point_join_in_a_plane() {
        let p0 = Point::new(0.0, 0.0, 0.0);
        let p1 = Point::new(1.0, 0.0, 0.0);
        let p2 = Point::new(0.0, 1.0, 0.0);
        let line: Line = (p0 & p1).unwrap();
        let plane = line & p2;
        assert!(plane.is_some());
    }

    #[test]
    fn colinear_line_and_point_do_not_join_in_a_plane() {
        let p0 = Point::new(0.0, 0.0, 0.0);
        let p1 = Point::new(1.0, 0.0, 0.0);
        let p2 = Point::new(2.0, 0.0, 0.0);
        let line: Line = (p0 & p1).unwrap();
        let plane = line & p2;
        assert!(plane.is_none());
    }

    // #[test]
    // fn coplanar_lines_meet_in_a_point() {
    //     let p0 = Point::new(0.0, 0.0, 0.0);
    //     let p1 = Point::new(1.0, 0.0, 0.0);
    //     let p2 = Point::new(0.0, 1.0, 0.0);
    //     let line1: Line = (p0 & p1).unwrap();
    //     let projected_line: Line = (Plane::E3 | line1).unwrap() * Plane::E3;
    //     assert!(projected_line.0 != PGA3D::zero());
    //     let line2: Line = (p0 & p2).unwrap();
    //     let plane = line1 ^ line2;
    //     assert!(plane.is_some());
    // }
}
