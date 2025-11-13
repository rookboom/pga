#[cfg(test)]
mod tests {
    use crate::pgai::{
        BulkWeight, Direction, Dual, GeometricEntity, Line, Origin, Plane, PlaneDirection, Point3,
        Point4,
    };
    use crate::{ApproxEq, assert_approx_eq};

    #[test]
    fn two_points_join_in_a_line() {
        let p0 = Point3::new(0.0, 0.0, 0.0);
        let p1 = Point3::new(1.0, 0.0, 0.0);
        let line1: Line = p0 ^ p1;
        assert!(!line1.is_zero());
        let p0 = Point4::new(0.0, 0.0, 0.0, 1.0);
        let p1 = Point4::new(1.0, 0.0, 0.0, 1.0);
        let line1: Line = p0 ^ p1;
        assert!(!line1.is_zero());
    }

    #[test]
    fn identical_points_do_not_join_in_a_line() {
        let p1 = Point3::new(1.0, 0.0, 0.0);
        let degenerate_line1: Line = p1 ^ p1;
        assert!(degenerate_line1.is_zero());
    }

    #[test]
    fn three_points_join_in_a_plane() {
        let p0 = Point3::new(0.0, 0.0, 0.0);
        let p1 = Point3::new(1.0, 0.0, 0.0);
        let p2 = Point3::new(0.0, 1.0, 0.0);
        let plane1: Plane = p0 ^ p1 ^ p2;
        assert!(!plane1.is_zero());
        let p0 = Point4::new(0.0, 0.0, 0.0, 1.0);
        let p1 = Point4::new(1.0, 0.0, 0.0, 1.0);
        let p2 = Point4::new(0.0, 1.0, 0.0, 1.0);
        let plane1: Plane = p0 ^ p1 ^ p2;
        assert!(!plane1.is_zero());
    }

    #[test]
    fn colinear_points_do_not_join_in_a_plane() {
        let p0 = Point3::new(0.0, 0.0, 0.0);
        let p1 = Point3::new(1.0, 0.0, 0.0);
        let p2 = Point3::new(2.0, 0.0, 0.0);
        let plane1: Plane = p0 ^ p1 ^ p2;
        assert!(plane1.is_zero());
    }

    #[test]
    fn two_planes_meet_in_a_line() {
        let line: Line = Plane::FORWARD & Plane::UP;
        assert!(!line.is_zero());
    }

    #[test]
    fn three_base_planes_meet_in_origin() {
        let origin = Point3::from(Plane::FORWARD & Plane::UP & Plane::LEFT);
        assert_eq!(origin, Point3::new(0.0, 0.0, 0.0));
        assert!(!origin.is_zero());
    }

    #[test]
    fn three_planes_meet_in_a_point() {
        let p0: Plane = Plane::new(1.0, 0.0, 0.0, -1.0); // x = 1
        let p1: Plane = Plane::new(0.0, 1.0, 0.0, -2.0); // y = 2

        let p2: Plane = Plane::new(0.0, 0.0, 1.0, -3.0); // z = 3

        let point = Point3::from(p0 & p1 & p2);
        let expected = Point3::new(1.0, 2.0, 3.0);
        assert_approx_eq!(point, expected);
    }

    #[test]
    fn line_through_origin_and_plane_meet_in_a_point() {
        let p0 = Point4::new(0.0, 0.0, 0.0, 1.0);
        let p1 = Point4::new(1.0, 0.0, 0.0, 1.0);
        let line: Line = p0 ^ p1;
        // Use a plane that doesn't pass through the origin: x = 0.5
        let plane = Plane::new(1.0, 0.0, 0.0, -0.5);
        let point = plane & line;
        let expected = Point3::new(0.5, 0.0, 0.0);
        assert_approx_eq!(Point3::from(point), expected);
        let point = line & plane;
        assert_approx_eq!(Point3::from(point), expected);
    }
    #[test]
    fn line_and_plane_meet_in_a_point() {
        let p0 = Point3::new(0.0, 1.0, 0.0);
        let p1 = Point3::new(1.0, 1.0, 0.0);
        let line: Line = p0 ^ p1;
        // Use a plane that doesn't pass through the origin: x = 0.5
        let plane = Plane::new(1.0, 0.0, 0.0, 1.0);
        let point = plane & line;
        let expected = Point3::new(-1.0, 1.0, 0.0);
        let actual = Point3::from(point);
        assert_approx_eq!(actual, expected);
        let point = line & plane;
        assert_approx_eq!(Point3::from(point), expected);
    }

    #[test]
    fn coplaner_line_and_plane_meet_in_a_direction() {
        let p0 = Point4::new(0.0, 0.0, 0.0, 1.0);
        let p1 = Point4::new(1.0, 0.0, 0.0, 1.0);
        let line: Line = p0 ^ p1;
        // Use a plane that doesn't pass through the origin: x = 0.5
        let plane = Plane::new(0.0, 0.0, 1.0, -0.5);
        let point = plane & line;
        let expected = Direction::new(-0.5, 0.0, 0.0);
        assert_approx_eq!(point.bulk(), expected);
        assert_approx_eq!(point.weight(), Origin::new(0.0));
    }

    #[test]
    fn coplaner_line_through_origin_and_plane_do_not_meet() {
        let p0 = Point4::new(0.0, 0.0, 0.0, 1.0);
        let p1 = Point4::new(1.0, 0.0, 0.0, 1.0);
        let line: Line = p0 ^ p1;
        let plane = Plane::new(0.0, 0.0, 1.0, 0.0);
        let p = plane & line;
        assert_approx_eq!(p.bulk(), Direction::new(0.0, 0.0, 0.0));
        assert_approx_eq!(p.weight(), Origin::new(0.0));
        assert!(p.is_zero());
    }

    #[test]
    fn line_and_point_join_in_a_plane() {
        let p0 = Point4::new(0.0, 0.0, 0.0, 1.0);
        let p1 = Point4::new(1.0, 0.0, 0.0, 1.0);
        let p2 = Point4::new(0.0, 1.0, 0.0, 1.0);
        let line: Line = p0 ^ p1;
        let plane = line ^ p2;
        assert!(!plane.is_zero());
        assert_approx_eq!(plane.weight(), PlaneDirection::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn colinear_line_and_point_do_not_join_in_a_plane() {
        let p0 = Point4::new(0.0, 0.0, 0.0, 1.0);
        let p1 = Point4::new(1.0, 0.0, 0.0, 1.0);
        let p2 = Point4::new(2.0, 0.0, 0.0, 1.0);
        let line: Line = p0 ^ p1;
        let plane = line ^ p2;
        assert!(plane.is_zero());
    }

    #[test]
    fn line_expands_to_a_perpendicular_plane() {
        let left = Plane::LEFT;
        let plane: Plane = Line::Z_AXIS ^ !left.weight();
        assert_eq!(plane, -Plane::UP);
    }

    #[test]
    fn perpendicular_line_expands_as_zero() {
        let left = Plane::LEFT;
        let plane = Line::X_AXIS ^ !left.weight();
        assert!(plane.is_zero());
    }

    #[test]
    fn point_expands_from_plane_in_a_perpendicular_line() {
        let left = Plane::LEFT;
        let point = Point4::new(1.0, 0.0, 0.0, 1.0);
        let line: Line = (point ^ !left.weight()).unitize();
        assert_eq!(line, -Line::X_AXIS);
    }

    #[test]
    fn point_expands_from_line_in_a_perpendicular_plane() {
        let forward = Line::Z_AXIS;
        let plane: Plane = Point3::new(1.0, 0.0, 0.0) ^ !forward.weight();
        assert_eq!(plane, Plane::FORWARD);
    }

    #[test]
    fn project_plane_onto_point() {
        let left = Plane::LEFT;
        let point = Point3::new(1.0, 0.0, 0.0);
        let line = point ^ !left.weight();

        let plane: Plane = (point ^ !line.weight()).unitize();
        assert_eq!(plane, Plane::new(-1.0, 0.0, 0.0, 1.0)); // Note the direction of the resulting plane changed from the input plane.
    }

    #[test]
    fn project_point_onto_plane() {
        let plane = Plane::new(1.0, 0.0, 0.0, 0.0);
        let point = Point3::new(3.0, 0.0, 0.0);
        let line = point ^ !plane.weight();
        let projected_point: Point3 = Point3::from(plane & line);

        assert_eq!(projected_point, Point3::new(0.0, 0.0, 0.0));
    }
}
