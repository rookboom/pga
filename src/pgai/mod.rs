mod types;
mod wedge;

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

    fn is_zero(&self) -> bool {
        self.length_squared() <= f32::EPSILON
    }
}

pub trait BulkWeight: GeometricEntity {
    type Bulk: Dual;
    type Weight: Dual;
    fn bulk(&self) -> Self::Bulk;
    fn weight(&self) -> Self::Weight;

    /// Unitizes the plane (normalizes the normal vector)
    fn unitize(&self) -> Self {
        let mut result = Self::default();
        let inv_mag = 1.0 / self.weight().norm();
        result.set_e0(self.e0() * inv_mag);
        result.set_e1(self.e1() * inv_mag);
        result.set_e2(self.e2() * inv_mag);
        result.set_e3(self.e3() * inv_mag);
        result.set_e41(self.e41() * inv_mag);
        result.set_e42(self.e42() * inv_mag);
        result.set_e43(self.e43() * inv_mag);
        result.set_e23(self.e23() * inv_mag);
        result.set_e31(self.e31() * inv_mag);
        result.set_e12(self.e12() * inv_mag);
        result.set_e423(self.e423() * inv_mag);
        result.set_e431(self.e431() * inv_mag);
        result.set_e412(self.e412() * inv_mag);
        result.set_e321(self.e321() * inv_mag);
        result.set_scalar(self.scalar() * inv_mag);
        result.set_antiscalar(self.antiscalar() * inv_mag);
        result
    }
}

pub trait Dual: GeometricEntity {
    type DualType: GeometricEntity + Dual<DualType = Self>;
    fn dual(&self) -> Self::DualType;
}

pub trait Grade: GeometricEntity {
    const GRADE: u8;
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
