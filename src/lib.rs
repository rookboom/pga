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

/// Generate getter and setter methods for fields of a geometric entity
#[macro_export]
macro_rules! impl_geometric_entity_trait {
    ($type:ty, [$($coord:ident => $value:tt),*]) => {
        impl $crate::pgai::GeometricEntity for $type {
            $(
                #[inline]
                fn $coord(&self) -> f32 {
                    $crate::impl_geometric_entity_trait!(@get_value self, $value)
                }

                $crate::impl_geometric_entity_trait!(@maybe_setter $coord, $value);
            )*
        }
    };
    (@get_value $self:ident, $field:ident) => { $self.$field };
    (@get_value $self:ident, $literal:literal) => { $literal };

    (@maybe_setter $coord:ident, $field:ident) => {
        paste::paste! {
            #[inline]
            fn [<set_ $coord>](&mut self, value: f32) {
                self.$field = value;
            }
        }
    };
    (@maybe_setter $coord:ident, $literal:literal) => {};
}

// Generate const constructor
#[macro_export]
macro_rules! impl_constructor {
    ($type:ty, [$($field:ident),*]) => {
        impl $type {
            pub const fn new($($field: f32),*) -> Self {
                Self {
                    $($field),*
                }
            }
        }
    };
}

/// Generate all implementations for a geometric entity - composite macro
#[macro_export]
macro_rules! impl_geometric_entity {
    // For types with only field values (no literals)
    ($type:ty, [$($coord:ident => $field:ident),*]) => {
        $crate::impl_geometric_entity_trait!($type, [$($coord => $field),*]);
        $crate::impl_constructor!($type, [$($field),*]);
    };

    // For types with mixed field values and literals
    ($type:ty, [$($coord:ident => $value:tt),*], fields: [$($field:ident),*]) => {
        $crate::impl_geometric_entity_trait!($type, [$($coord => $value),*]);
        $crate::impl_constructor!($type, [$($field),*]);
    };
}

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
