#![allow(dead_code)]
mod pgai;
mod test;

#[cfg(feature = "visualization")]
pub mod visualization;

// WASM entry point
#[cfg(all(target_arch = "wasm32", feature = "web"))]
mod web;

use glam::Vec3;
#[cfg(all(target_arch = "wasm32", feature = "web"))]
pub use web::*;

use crate::pgai::{Direction, Line, Origin, Plane, PlaneDirection, Point};

/// Trait for approximate equality comparisons, useful for floating-point tests
pub trait ApproxEq {
    /// Check if two values are approximately equal within a default epsilon
    fn approx_eq(&self, other: &Self) -> bool {
        self.approx_eq_eps(other, 1e-6)
    }

    /// Check if two values are approximately equal within a specified epsilon
    fn approx_eq_eps(&self, other: &Self, epsilon: f32) -> bool;
}

impl ApproxEq for f32 {
    fn approx_eq_eps(&self, other: &Self, epsilon: f32) -> bool {
        (self - other).abs() < epsilon
    }
}

impl ApproxEq for Vec3 {
    fn approx_eq_eps(&self, other: &Self, epsilon: f32) -> bool {
        (self.x - other.x).abs() < epsilon
            && (self.y - other.y).abs() < epsilon
            && (self.z - other.z).abs() < epsilon
    }
}

impl ApproxEq for Point {
    fn approx_eq_eps(&self, other: &Self, epsilon: f32) -> bool {
        self.x().approx_eq_eps(&other.x(), epsilon)
            && self.y().approx_eq_eps(&other.y(), epsilon)
            && self.z().approx_eq_eps(&other.z(), epsilon)
            && self.w().approx_eq_eps(&other.w(), epsilon)
    }
}

impl ApproxEq for Line {
    fn approx_eq_eps(&self, other: &Self, epsilon: f32) -> bool {
        Vec3::from(self.moment).approx_eq_eps(&Vec3::from(other.moment), epsilon)
            && Vec3::from(self.direction).approx_eq_eps(&Vec3::from(other.direction), epsilon)
    }
}

impl ApproxEq for Plane {
    fn approx_eq_eps(&self, other: &Self, epsilon: f32) -> bool {
        self.x().approx_eq_eps(&other.x(), epsilon)
            && self.y().approx_eq_eps(&other.y(), epsilon)
            && self.z().approx_eq_eps(&other.z(), epsilon)
            && self.w().approx_eq_eps(&other.w(), epsilon)
    }
}

impl ApproxEq for Direction {
    fn approx_eq_eps(&self, other: &Self, epsilon: f32) -> bool {
        self.x().approx_eq_eps(&other.x(), epsilon)
            && self.y().approx_eq_eps(&other.y(), epsilon)
            && self.z().approx_eq_eps(&other.z(), epsilon)
    }
}

impl ApproxEq for PlaneDirection {
    fn approx_eq_eps(&self, other: &Self, epsilon: f32) -> bool {
        self.x().approx_eq_eps(&other.x(), epsilon)
            && self.y().approx_eq_eps(&other.y(), epsilon)
            && self.z().approx_eq_eps(&other.z(), epsilon)
    }
}

impl ApproxEq for Origin {
    fn approx_eq_eps(&self, other: &Self, epsilon: f32) -> bool {
        f32::from(*self).approx_eq_eps(&f32::from(*other), epsilon)
    }
}

/// Macro for asserting approximate equality in tests
#[macro_export]
macro_rules! assert_approx_eq {
    ($left:expr, $right:expr) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !left_val.approx_eq(right_val) {
                    panic!(
                        "assertion failed: `(left approx== right)`\n  left: `{:?}`\n right: `{:?}`",
                        left_val, right_val
                    );
                }
            }
        }
    };
    ($left:expr, $right:expr, $epsilon:expr) => {
        match (&$left, &$right, &$epsilon) {
            (left_val, right_val, epsilon_val) => {
                if !left_val.approx_eq_eps(right_val, *epsilon_val) {
                    panic!(
                        "assertion failed: `(left approx== right)` with epsilon `{}`\n  left: `{:?}`\n right: `{:?}`",
                        epsilon_val, left_val, right_val
                    );
                }
            }
        }
    };
}

/// Macro for asserting approximate inequality in tests
#[macro_export]
macro_rules! assert_approx_ne {
    ($left:expr, $right:expr) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if left_val.approx_eq(right_val) {
                    panic!(
                        "assertion failed: `(left approx!= right)`\n  left: `{:?}`\n right: `{:?}`",
                        left_val, right_val
                    );
                }
            }
        }
    };
    ($left:expr, $right:expr, $epsilon:expr) => {
        match (&$left, &$right, &$epsilon) {
            (left_val, right_val, epsilon_val) => {
                if left_val.approx_eq_eps(right_val, *epsilon_val) {
                    panic!(
                        "assertion failed: `(left approx!= right)` with epsilon `{}`\n  left: `{:?}`\n right: `{:?}`",
                        epsilon_val, left_val, right_val
                    );
                }
            }
        }
    };
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
