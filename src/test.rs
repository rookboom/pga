#[cfg(test)]
mod tests {
    use glam::Vec3;

    use crate::pgai::{Direction, Horizon, Line, Origin, Plane, PlaneDirection, Point};
    use crate::{ApproxEq, assert_approx_eq};

    #[test]
    fn two_points_join_in_a_line() {
        let p0 = Point::from([0.0, 0.0, 0.0]);
        let p1 = Point::from([1.0, 0.0, 0.0]);
        let line1 = p0 ^ p1;
        assert!(line1.is_valid());
    }

    #[test]
    fn identical_points_do_not_join_in_a_line() {
        let p1 = Point::from([1.0, 0.0, 0.0]);
        let degenerate_line1: Line = p1 ^ p1;
        assert!(!degenerate_line1.is_valid());
    }

    #[test]
    fn three_points_join_in_a_plane() {
        let p0 = Point::from([0.0, 0.0, 0.0]);
        let p1 = Point::from([1.0, 0.0, 0.0]);
        let p2 = Point::from([0.0, 1.0, 0.0]);
        let plane1: Plane = p0 ^ p1 ^ p2;
        assert!(plane1.is_valid());
    }

    #[test]
    fn colinear_points_do_not_join_in_a_plane() {
        let p0 = Point::from([0.0, 0.0, 0.0]);
        let p1 = Point::from([1.0, 0.0, 0.0]);
        let p2 = Point::from([2.0, 0.0, 0.0]);
        let plane1: Plane = p0 ^ p1 ^ p2;
        assert!(!plane1.is_valid());
    }

    #[test]
    fn two_planes_meet_in_a_line() {
        let line: Line = Plane::FORWARD & Plane::UP;
        assert!(line.is_valid());
    }

    #[test]
    fn three_planes_meet_in_a_point() {
        let origin: Point = Plane::FORWARD & Plane::UP & Plane::LEFT;
        assert_eq!(origin, Point::from([0.0, 0.0, 0.0]));
        assert!(origin.is_valid());
    }

    #[test]
    fn line_and_plane_meet_in_a_point() {
        let p0 = Point::from([0.0, 0.0, 0.0]);
        let p1 = Point::from([1.0, 0.0, 0.0]);
        let line: Line = p0 ^ p1;
        // Use a plane that doesn't pass through the origin: x = 0.5
        let plane = Plane::from_normal_distance(Vec3::new(1.0, 0.0, 0.0), -0.5);
        let point = plane & line;

        let expected = Vec3::new(0.5, 0.0, 0.0);
        assert_approx_eq!(point.project(), expected);
    }

    #[test]
    fn coplaner_line_and_plane_meet_in_a_direction() {
        let p0 = Point::from([0.0, 0.0, 0.0]);
        let p1 = Point::from([1.0, 0.0, 0.0]);
        let line: Line = p0 ^ p1;
        // Use a plane that doesn't pass through the origin: x = 0.5
        let plane = Plane::new(0.0, 0.0, 1.0, -0.5);
        let point = plane & line;
        let expected = Direction::from([-0.5, 0.0, 0.0]);
        assert_approx_eq!(point.direction, expected);
        assert_approx_eq!(point.origin, Origin::from(0.0));
    }

    #[test]
    fn coplaner_line_through_origin_and_plane_do_not_meet() {
        let p0 = Point::from([0.0, 0.0, 0.0]);
        let p1 = Point::from([1.0, 0.0, 0.0]);
        let line: Line = p0 ^ p1;
        let plane = Plane::new(0.0, 0.0, 1.0, 0.0);
        let p = plane & line;
        assert_approx_eq!(p.direction, Direction::from([0.0, 0.0, 0.0]));
        assert_approx_eq!(p.origin, Origin::from(0.0));
        assert!(!p.is_valid());
    }

    #[test]
    fn line_and_point_join_in_a_plane() {
        let p0 = Point::from([0.0, 0.0, 0.0]);
        let p1 = Point::from([1.0, 0.0, 0.0]);
        let p2 = Point::from([0.0, 1.0, 0.0]);
        let line: Line = p0 ^ p1;
        let plane = line ^ p2;
        assert!(plane.is_valid());
        assert_approx_eq!(plane.direction, PlaneDirection::from([0.0, 0.0, 1.0]));
    }

    #[test]
    fn colinear_line_and_point_do_not_join_in_a_plane() {
        let p0 = Point::new(0.0, 0.0, 0.0);
        let p1 = Point::new(1.0, 0.0, 0.0);
        let p2 = Point::new(2.0, 0.0, 0.0);
        let line: Line = p0 ^ p1;
        let plane = line ^ p2;
        assert!(plane.is_zero());
    }

    #[test]
    fn line_expands_to_a_perpendicular_plane() {
        let left = Plane::LEFT;
        let plane: Plane = Line::Z_AXIS ^ left.weight().dual();
        assert_eq!(plane, -Plane::UP);
    }

    #[test]
    fn perpendicular_line_expands_as_zero() {
        let left = Plane::LEFT;
        let plane = Line::X_AXIS ^ left.weight().dual();
        assert!(plane.is_zero());
    }

    #[test]
    fn point_expands_from_plane_in_a_perpendicular_line() {
        let left = Plane::LEFT;
        let point = Point::new(1.0, 0.0, 0.0);
        let mut line: Line = point ^ left.weight().dual();
        line.unitize();
        assert_eq!(line, -Line::X_AXIS);
    }

    #[test]
    fn point_expands_from_line_in_a_perpendicular_plane() {
        let forward = Line::Z_AXIS;
        let plane: Plane = Point::new(1.0, 0.0, 0.0) ^ forward.weight().dual();
        assert_eq!(plane, Plane::FORWARD);
    }

    #[test]
    fn project_plane_onto_point() {
        let left = Plane::LEFT;
        let point = Point::new(1.0, 0.0, 0.0);
        let line = point ^ left.weight().dual();

        let mut plane: Plane = point ^ line.weight().dual();
        plane.unitize();
        assert_eq!(plane, Plane::new(-1.0, 0.0, 0.0, 1.0)); // Note the direction of the resulting plane changed from the input plane.
    }

    #[test]
    fn project_point_onto_plane() {
        let plane = Plane::new(1.0, 0.0, 0.0, 0.0);
        let point = Point::new(3.0, 0.0, 0.0);
        let line = point ^ plane.weight().dual();
        let mut projected_point: Point = plane & line;
        projected_point.unitize();

        assert_eq!(projected_point, Point::new(0.0, 0.0, 0.0));
    }
}
