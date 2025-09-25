#[cfg(test)]
mod tests {
    use crate::pga3d::{Direction, Line, Plane, Point, ZeroOr};
    use crate::{ApproxEq, assert_approx_eq};

    #[test]
    fn two_points_join_in_a_line() {
        let p0 = Point::new(0.0, 0.0, 0.0);
        let p1 = Point::new(1.0, 0.0, 0.0);
        let line1: ZeroOr<Line> = p0 & p1;
        assert!(line1.is_valid());
    }

    #[test]
    fn identical_points_do_not_join_in_a_line() {
        let p1 = &Point::new(1.0, 0.0, 0.0);
        let degenerate_line1: ZeroOr<Line> = p1 & p1;
        assert!(degenerate_line1.is_zero());
    }

    #[test]
    fn three_points_join_in_a_plane() {
        let p0 = Point::new(0.0, 0.0, 0.0);
        let p1 = Point::new(1.0, 0.0, 0.0);
        let p2 = Point::new(0.0, 1.0, 0.0);
        let plane1: ZeroOr<Plane> = p0 & p1 & p2;
        assert!(plane1.is_valid());
    }

    #[test]
    fn colinear_points_do_not_join_in_a_plane() {
        let p0 = Point::new(0.0, 0.0, 0.0);
        let p1 = Point::new(1.0, 0.0, 0.0);
        let p2 = Point::new(2.0, 0.0, 0.0);
        let plane1: ZeroOr<Plane> = p0 & p1 & p2;
        assert!(plane1.is_zero());
    }

    #[test]
    fn two_planes_meet_in_a_line() {
        let line: ZeroOr<Line> = Plane::E1 ^ Plane::E2;
        assert!(line.is_valid());
    }

    #[test]
    fn three_planes_meet_in_a_point() {
        let origin: ZeroOr<Point> = Plane::E1 ^ Plane::E2 ^ Plane::E3;
        assert_eq!(origin.as_ref(), Some(&Point::new(0., 0.0, 0.0)));
        assert!(origin.is_valid());
    }

    #[test]
    fn line_and_plane_meet_in_a_point() {
        let p0 = Point::new(0.0, 0.0, 0.0);
        let p1 = Point::new(1.0, 0.0, 0.0);
        let line: Line = (p0 & p1).unwrap();
        // Use a plane that doesn't pass through the origin: x = 0.5
        let plane = Plane::new(1.0, 0.0, 0.0, -0.5);
        match plane ^ line {
            Some(crate::pga3d::PointOrDirection::Point(point)) => {
                let expected = Point::new(0.5, 0.0, 0.0);
                assert_approx_eq!(point, expected);
            }
            _ => panic!("Expected a point"),
        }
    }

    #[test]
    fn coplaner_line_and_plane_meet_in_a_direction() {
        let p0 = Point::new(0.0, 0.0, 0.0);
        let p1 = Point::new(1.0, 0.0, 0.0);
        let line: Line = (p0 & p1).unwrap();
        // Use a plane that doesn't pass through the origin: x = 0.5
        let plane = Plane::new(0.0, 0.0, 1.0, -0.5);
        match plane ^ line {
            Some(crate::pga3d::PointOrDirection::Direction(dir)) => {
                let expected = Direction::new(0.5, 0.0, 0.0);
                assert_approx_eq!(dir, expected);
            }
            _ => panic!("Expected a direction"),
        }
    }

    #[test]
    fn coplaner_line_through_origin_and_plane_do_not_meet() {
        let p0 = Point::new(0.0, 0.0, 0.0);
        let p1 = Point::new(1.0, 0.0, 0.0);
        let line: Line = (p0 & p1).unwrap();
        let plane = Plane::new(0.0, 0.0, 1.0, 0.0);
        let p = plane ^ line;
        assert!(p.is_none());
    }

    #[test]
    fn line_and_point_join_in_a_plane() {
        let p0 = Point::new(0.0, 0.0, 0.0);
        let p1 = Point::new(1.0, 0.0, 0.0);
        let p2 = Point::new(0.0, 1.0, 0.0);
        let line: Line = (p0 & p1).unwrap();
        let plane = line & p2;
        assert!(plane.is_valid());
    }

    #[test]
    fn colinear_line_and_point_do_not_join_in_a_plane() {
        let p0 = Point::new(0.0, 0.0, 0.0);
        let p1 = Point::new(1.0, 0.0, 0.0);
        let p2 = Point::new(2.0, 0.0, 0.0);
        let line: Line = (p0 & p1).unwrap();
        let plane = line & p2;
        assert!(plane.is_zero());
    }

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
        let point = &Point::X1;

        let plane: Plane = (left | point) * point;
        assert_eq!(plane, Plane::new(-1.0, 0.0, 0.0, 1.0)); // Note the direction of the resulting plane changed from the input plane.
    }

    #[test]
    fn project_point_onto_plane() {
        let plane = &Plane::new(1.0, 0.0, 0.0, 0.0);
        let point = Point::new(3.0, 0.0, 0.0);
        let projected_point: Point = (plane | point) * plane;

        assert_eq!(projected_point, Point::new(0.0, 0.0, 0.0));
    }
}
