use crate::pgai::{Direction, GeometricEntity, LineMoment};

use crate::pgai::types::{Line, Plane, Point3, Point4};
use std::ops::{BitAnd, BitXor};

fn wedge<L, R, O>(lhs: L, rhs: R) -> O
where
    L: GeometricEntity,
    R: GeometricEntity,
    O: GeometricEntity,
{
    let mut result = O::default();

    // Extract coordinates once
    let (px, py, pz, pw) = (lhs.e1(), lhs.e2(), lhs.e3(), lhs.e0());
    let (qx, qy, qz, qw) = (rhs.e1(), rhs.e2(), rhs.e3(), rhs.e0());

    // Point ^ Point -> Line
    result.set_e41(qx * pw - px * qw);
    result.set_e42(qy * pw - py * qw);
    result.set_e43(qz * pw - pz * qw);
    result.set_e23(py * qz - pz * qy);
    result.set_e31(pz * qx - px * qz);
    result.set_e12(px * qy - py * qx);

    // Line coordinates
    let (lvx, lvy, lvz) = (lhs.e41(), lhs.e42(), lhs.e43());
    let (lmx, lmy, lmz) = (lhs.e23(), lhs.e31(), lhs.e12());

    // Line ^ Point -> Plane (both directions and combine)
    let lp_e423 = lvy * qz - lvz * qy + lmx * qw;
    let lp_e431 = lvz * qx - lvx * qz + lmy * qw;
    let lp_e412 = lvx * qy - lvy * qx + lmz * qw;
    let lp_e321 = -lmx * qx - lmy * qy - lmz * qz;

    let (rlvx, rlvy, rlvz) = (rhs.e41(), rhs.e42(), rhs.e43());
    let (rlmx, rlmy, rlmz) = (rhs.e23(), rhs.e31(), rhs.e12());

    let pl_e423 = rlvy * pz - rlvz * py + rlmx * pw;
    let pl_e431 = rlvz * px - rlvx * pz + rlmy * pw;
    let pl_e412 = rlvx * py - rlvy * px + rlmz * pw;
    let pl_e321 = -rlmx * px - rlmy * py - rlmz * pz;

    result.set_e423(lp_e423 - pl_e423);
    result.set_e431(lp_e431 - pl_e431);
    result.set_e412(lp_e412 - pl_e412);
    result.set_e321(lp_e321 - pl_e321);

    result
}

fn antiwedge<L, R, O>(lhs: L, rhs: R) -> O
where
    L: GeometricEntity,
    R: GeometricEntity,
    O: GeometricEntity,
{
    let mut result = O::default();
    // Plane & Plane -> Line
    let lpx = lhs.e423();
    let lpy = lhs.e431();
    let lpz = lhs.e412();
    let lpw = lhs.e321();
    let rpx = rhs.e423();
    let rpy = rhs.e431();
    let rpz = rhs.e412();
    let rpw = rhs.e321();
    result.set_e41(lpz * rpy - lpy * rpz);
    result.set_e42(lpx * rpz - lpz * rpx);
    result.set_e43(lpy * rpx - lpx * rpy);
    result.set_e23(lpx * rpw - lpw * rpx);
    result.set_e31(lpy * rpw - lpw * rpy);
    result.set_e12(lpz * rpw - lpw * rpz);

    let rvx = rhs.e41();
    let rvy = rhs.e42();
    let rvz = rhs.e43();
    let rmx = rhs.e23();
    let rmy = rhs.e31();
    let rmz = rhs.e12();

    let lvx = lhs.e41();
    let lvy = lhs.e42();
    let lvz = lhs.e43();
    let lmx = lhs.e23();
    let lmy = lhs.e31();
    let lmz = lhs.e12();

    // Plane & Line -> Point4
    let pl_e1 = lpz * rmy - lpy * rmz + lpw * rvx;
    let pl_e2 = lpx * rmz - lpz * rmx + lpw * rvy;
    let pl_e3 = lpy * rmx - lpx * rmy + lpw * rvz;
    let pl_e0 = -lpx * rvx - lpy * rvy - lpz * rvz;

    // Line & Plane -> Point4
    // let lp_e1 = lmz * rpy - lmy * rpz + lvx * rpw;
    // let lp_e2 = lmx * rpz - lmz * rpx + lvy * rpw;
    // let lp_e3 = lmy * rpx - lmx * rpy + lvz * rpw;
    // let lp_e0 = -lvx * rpx - lvy * rpy - lvz * rpz;

    let lp_e1 = rpz * lmy - rpy * lmz + rpw * lvx;
    let lp_e2 = rpx * lmz - rpz * lmx + rpw * lvy;
    let lp_e3 = rpy * lmx - rpx * lmy + rpw * lvz;
    let lp_e0 = -rpx * lvx - rpy * lvy - rpz * lvz;

    result.set_e1(pl_e1 + lp_e1);
    result.set_e2(pl_e2 + lp_e2);
    result.set_e3(pl_e3 + lp_e3);
    result.set_e0(pl_e0 + lp_e0);

    result
}

// This implement the wedge product (^) between different geometric entities
// It is implemented as a macro to get around some of the constraints of implementing
// traits for generic types.
macro_rules! impl_wedge {
    ($a:ty, $b:ty, $out:ident) => {
        impl BitXor<$b> for $a {
            type Output = $out;

            fn bitxor(self, rhs: $b) -> Self::Output {
                wedge(self, rhs)
            }
        }
    };
}
macro_rules! impl_anti_wedge {
    ($a:ty, $b:ty, $out:ident) => {
        impl BitAnd<$b> for $a {
            type Output = $out;

            fn bitand(self, rhs: $b) -> Self::Output {
                antiwedge(self, rhs)
            }
        }
    };
}

impl_wedge!(Point4, Point4, Line);
impl_wedge!(Point3, Point3, Line);
impl_wedge!(Line, Point3, Plane);
impl_wedge!(Line, Direction, Plane);
impl_wedge!(Point4, Direction, Line);
impl_wedge!(Point3, Direction, Line);
impl_wedge!(Direction, Point3, Line);
impl_wedge!(Point4, LineMoment, Plane);
impl_wedge!(Point3, LineMoment, Plane);
impl_wedge!(Line, Point4, Plane);

impl_anti_wedge!(Plane, Plane, Line);
impl_anti_wedge!(Plane, Line, Point4);
impl_anti_wedge!(Line, Plane, Point4);
