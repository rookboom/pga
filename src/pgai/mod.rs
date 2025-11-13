mod types;
mod wedge;

use glam::Vec3;
use std::ops::Neg;
pub use types::*;

// ================================================================================================
// TRAITS
// ================================================================================================
pub trait GeometricEntity: Default + Neg {
    #[inline]
    fn e0(&self) -> f32 {
        0.0
    }
    #[inline]
    fn e1(&self) -> f32 {
        0.0
    }
    #[inline]
    fn e2(&self) -> f32 {
        0.0
    }
    #[inline]
    fn e3(&self) -> f32 {
        0.0
    }
    #[inline]
    fn e41(&self) -> f32 {
        0.0
    }
    #[inline]
    fn e42(&self) -> f32 {
        0.0
    }
    #[inline]
    fn e43(&self) -> f32 {
        0.0
    }
    #[inline]
    fn e23(&self) -> f32 {
        0.0
    }
    #[inline]
    fn e31(&self) -> f32 {
        0.0
    }
    #[inline]
    fn e12(&self) -> f32 {
        0.0
    }
    #[inline]
    fn e423(&self) -> f32 {
        0.0
    }
    #[inline]
    fn e431(&self) -> f32 {
        0.0
    }
    #[inline]
    fn e412(&self) -> f32 {
        0.0
    }
    #[inline]
    fn e321(&self) -> f32 {
        0.0
    }
    #[inline]
    fn scalar(&self) -> f32 {
        0.0
    }
    #[inline]
    fn antiscalar(&self) -> f32 {
        0.0
    }

    #[inline]
    fn set_e0(&mut self, _value: f32) {}
    #[inline]
    fn set_e1(&mut self, _value: f32) {}
    #[inline]
    fn set_e2(&mut self, _value: f32) {}
    #[inline]
    fn set_e3(&mut self, _value: f32) {}
    #[inline]
    fn set_e41(&mut self, _value: f32) {}
    #[inline]
    fn set_e42(&mut self, _value: f32) {}
    #[inline]
    fn set_e43(&mut self, _value: f32) {}
    #[inline]
    fn set_e23(&mut self, _value: f32) {}
    #[inline]
    fn set_e31(&mut self, _value: f32) {}
    #[inline]
    fn set_e12(&mut self, _value: f32) {}
    #[inline]
    fn set_e423(&mut self, _value: f32) {}
    #[inline]
    fn set_e431(&mut self, _value: f32) {}
    #[inline]
    fn set_e412(&mut self, _value: f32) {}
    #[inline]
    fn set_e321(&mut self, _value: f32) {}
    #[inline]
    fn set_scalar(&mut self, _value: f32) {}
    #[inline]
    fn set_antiscalar(&mut self, _value: f32) {}

    fn length_squared(&self) -> f32 {
        self.e0() * self.e0()
            + self.e1() * self.e1()
            + self.e2() * self.e2()
            + self.e3() * self.e3()
            + self.e41() * self.e41()
            + self.e42() * self.e42()
            + self.e43() * self.e43()
            + self.e23() * self.e23()
            + self.e31() * self.e31()
            + self.e12() * self.e12()
            + self.e423() * self.e423()
            + self.e431() * self.e431()
            + self.e412() * self.e412()
            + self.e321() * self.e321()
            + self.scalar() * self.scalar()
            + self.antiscalar() * self.antiscalar()
    }

    fn norm(&self) -> f32 {
        self.length_squared().sqrt()
    }

    fn normalize(&self) -> Self
    where
        Self: Sized,
    {
        let len = self.norm();
        if len > f32::EPSILON {
            let inv_len = 1.0 / len;
            let mut result = Self::default();
            result.set_e0(self.e0() * inv_len);
            result.set_e1(self.e1() * inv_len);
            result.set_e2(self.e2() * inv_len);
            result.set_e3(self.e3() * inv_len);
            result.set_e41(self.e41() * inv_len);
            result.set_e42(self.e42() * inv_len);
            result.set_e43(self.e43() * inv_len);
            result.set_e23(self.e23() * inv_len);
            result.set_e31(self.e31() * inv_len);
            result.set_e12(self.e12() * inv_len);
            result.set_e423(self.e423() * inv_len);
            result.set_e431(self.e431() * inv_len);
            result.set_e412(self.e412() * inv_len);
            result.set_e321(self.e321() * inv_len);
            result.set_scalar(self.scalar() * inv_len);
            result.set_antiscalar(self.antiscalar() * inv_len);
            result
        } else {
            Self::default()
        }
    }
    fn is_zero(&self) -> bool {
        self.length_squared() <= f32::EPSILON
    }
}

pub trait BulkWeight {
    type Bulk: Dual;
    type Weight: Dual;
    fn bulk(&self) -> Self::Bulk;
    fn weight(&self) -> Self::Weight;
}

trait Dual {
    type DualType: GeometricEntity + Dual<DualType = Self>;
    fn dual(&self) -> Self::DualType;
}

// ================================================================================================
// TRAIT IMPLEMENTATIONS
// ================================================================================================

impl<T> crate::ApproxEq for T
where
    T: GeometricEntity,
{
    fn approx_eq_eps(&self, other: &Self, epsilon: f32) -> bool {
        (self.e0() - other.e0()).abs() <= epsilon
            && (self.e1() - other.e1()).abs() <= epsilon
            && (self.e2() - other.e2()).abs() <= epsilon
            && (self.e3() - other.e3()).abs() <= epsilon
            && (self.e41() - other.e41()).abs() <= epsilon
            && (self.e42() - other.e42()).abs() <= epsilon
            && (self.e43() - other.e43()).abs() <= epsilon
            && (self.e23() - other.e23()).abs() <= epsilon
            && (self.e31() - other.e31()).abs() <= epsilon
            && (self.e12() - other.e12()).abs() <= epsilon
            && (self.e423() - other.e423()).abs() <= epsilon
            && (self.e431() - other.e431()).abs() <= epsilon
            && (self.e412() - other.e412()).abs() <= epsilon
            && (self.e321() - other.e321()).abs() <= epsilon
            && (self.scalar() - other.scalar()).abs() <= epsilon
            && (self.antiscalar() - other.antiscalar()).abs() <= epsilon
    }
}

// ================================================================================================
// IMPLEMENTATIONS
// ================================================================================================

impl Line {
    pub const X_AXIS: Line = Line::new(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    pub const Y_AXIS: Line = Line::new(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
    pub const Z_AXIS: Line = Line::new(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);

    pub fn through_origin(x: f32, y: f32, z: f32) -> Self {
        Line::new(x, y, z, 0.0, 0.0, 0.0)
    }

    pub fn bulk(&self) -> LineMoment {
        LineMoment::new(self.mx, self.my, self.mz)
    }

    pub fn weight(&self) -> LineDirection {
        LineDirection::new(self.vx, self.vy, self.vz)
    }

    // pub fn is_valid(&self) -> bool {
    //     self.weight().norm() > f32::EPSILON
    // }

    pub fn unitize(&mut self) -> &mut Self {
        let inv_mag = 1.0 / (self.vx * self.vx + self.vy * self.vy + self.vz * self.vz).sqrt();
        self.vx *= inv_mag;
        self.vy *= inv_mag;
        self.vz *= inv_mag;
        self.mx *= inv_mag;
        self.my *= inv_mag;
        self.mz *= inv_mag;

        self
    }
}

impl Plane {
    pub const LEFT: Plane = Plane::new(1.0, 0.0, 0.0, 0.0);
    pub const UP: Plane = Plane::new(0.0, 1.0, 0.0, 0.0);
    pub const FORWARD: Plane = Plane::new(0.0, 0.0, 1.0, 0.0);

    pub fn from_normal_distance(normal: Vec3, distance: f32) -> Self {
        Plane::new(normal.x, normal.y, normal.z, distance)
    }

    pub fn from_normal_point(normal: Vec3, point: Vec3) -> Self {
        let normal = normal.normalize();
        Plane::new(
            normal.x,
            normal.y,
            normal.z,
            -(normal.x * point.x + normal.y * point.y + normal.z * point.z),
        )
    }

    pub fn bulk(&self) -> Horizon {
        Horizon::new(self.w)
    }

    pub fn weight(&self) -> PlaneDirection {
        PlaneDirection::new(self.x, self.y, self.z)
    }

    // pub fn is_valid(&self) -> bool {
    //     self.direction.is_valid()
    // }

    /// Unitizes the plane (normalizes the normal vector)
    pub fn unitize(&mut self) -> &mut Self {
        *self = self.unitized();
        self
    }

    /// Unitizes the plane (normalizes the normal vector)
    pub fn unitized(&self) -> Self {
        let inv_mag = 1.0 / self.weight().norm();
        Plane::new(
            self.x * inv_mag,
            self.y * inv_mag,
            self.z * inv_mag,
            self.w * inv_mag,
        )
    }
}

impl Direction {
    const ZERO: Direction = Direction::new(0.0, 0.0, 0.0);

    pub fn dual(&self) -> PlaneDirection {
        PlaneDirection::new(self.x, self.y, self.z)
    }

    pub fn is_valid(&self) -> bool {
        self.norm() > f32::EPSILON
    }

    pub fn unitize(&mut self) -> &mut Self {
        let n = self.norm();
        *self = if n > f32::EPSILON {
            Self::new(self.x / n, self.y / n, self.z / n)
        } else {
            Self::ZERO
        };
        self
    }
}

impl Origin {
    const ZERO: Origin = Origin::new(0.0);

    pub fn dual(&self) -> Horizon {
        Horizon::new(self.w)
    }

    pub fn is_valid(&self) -> bool {
        self.norm() > f32::EPSILON
    }
}

impl Horizon {
    const ZERO: Horizon = Horizon::new(0.0);

    pub fn dual(&self) -> Origin {
        Origin::new(-self.w)
    }

    pub fn is_valid(&self) -> bool {
        self.norm() > f32::EPSILON
    }
}

impl PlaneDirection {
    const ZERO: PlaneDirection = PlaneDirection::new(0.0, 0.0, 0.0);

    pub fn dual(&self) -> Direction {
        Direction::new(-self.x, -self.y, -self.z)
    }

    pub fn is_valid(&self) -> bool {
        self.norm() > f32::EPSILON
    }
}

impl LineMoment {
    pub fn dual(&self) -> LineDirection {
        LineDirection::new(-self.x, -self.y, -self.z)
    }

    pub fn is_valid(&self) -> bool {
        self.norm() > f32::EPSILON
    }
}

impl LineDirection {
    pub fn dual(&self) -> LineMoment {
        LineMoment::new(-self.x, -self.y, -self.z)
    }

    pub fn is_valid(&self) -> bool {
        self.norm() > f32::EPSILON
    }
}
