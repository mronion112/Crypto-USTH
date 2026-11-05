//! Toy elliptic-curve arithmetic on short Weierstrass curves over prime fields.

use crate::field::{add_mod, inv_mod, mul_mod, sub_mod};

/// Short Weierstrass curve `y^2 = x^3 + a*x + b` over the prime field `F_p`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Curve {
    /// Prime field modulus.
    pub p: u64,
    /// Curve coefficient `a`.
    pub a: u64,
    /// Curve coefficient `b`.
    pub b: u64,
}

/// Affine point `(x, y)` on a toy curve.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AffinePoint {
    /// Affine `x` coordinate.
    pub x: u64,
    /// Affine `y` coordinate.
    pub y: u64,
}

/// Jacobian point `(X, Y, Z)` with affine coordinates `(X/Z^2, Y/Z^3)`.
///
/// `Z = 0` represents the point at infinity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JacobianPoint {
    /// Jacobian `X` coordinate.
    pub x: u64,
    /// Jacobian `Y` coordinate.
    pub y: u64,
    /// Jacobian `Z` coordinate; zero denotes infinity.
    pub z: u64,
}

impl JacobianPoint {
    /// Returns the point at infinity.
    pub const fn infinity() -> Self {
        Self { x: 0, y: 1, z: 0 }
    }

    /// Reports whether this point is the point at infinity.
    pub fn is_infinity(self) -> bool {
        self.z == 0
    }
}

/// Tests whether `point` satisfies the curve equation.
pub fn is_on_curve(curve: &Curve, point: AffinePoint) -> bool {
    let lhs = mul_mod(point.y, point.y, curve.p);
    let x2 = mul_mod(point.x, point.x, curve.p);
    let x3 = mul_mod(x2, point.x, curve.p);
    let rhs = add_mod(x3, mul_mod(curve.a, point.x, curve.p), curve.p);
    let rhs = add_mod(rhs, curve.b, curve.p);
    lhs == rhs
}

/// Doubles a Jacobian point.
///
/// Returns infinity when the input is infinity or has `Y = 0`.
pub fn double(curve: &Curve, point: JacobianPoint) -> JacobianPoint {
    let p = curve.p;
    if point.z == 0 || point.y == 0 {
        return JacobianPoint::infinity();
    }

    let xx = mul_mod(point.x, point.x, p);
    let yy = mul_mod(point.y, point.y, p);
    let yyyy = mul_mod(yy, yy, p);
    let zz = mul_mod(point.z, point.z, p);
    let z4 = mul_mod(zz, zz, p);

    let x_plus_yy = add_mod(point.x, yy, p);
    let square = mul_mod(x_plus_yy, x_plus_yy, p);
    let inner = sub_mod(sub_mod(square, xx, p), yyyy, p);
    let s = mul_mod(2, inner, p);
    let m = add_mod(mul_mod(3, xx, p), mul_mod(curve.a, z4, p), p);

    let x3 = sub_mod(mul_mod(m, m, p), mul_mod(2, s, p), p);
    let y3 = sub_mod(mul_mod(m, sub_mod(s, x3, p), p), mul_mod(8, yyyy, p), p);
    let z3 = mul_mod(2, mul_mod(point.y, point.z, p), p);

    JacobianPoint {
        x: x3,
        y: y3,
        z: z3,
    }
}

/// Adds an affine point to a Jacobian point.
///
/// Handles the doubling and inverse cases without converting the Jacobian point
/// to affine coordinates.
pub fn add_affine(curve: &Curve, point: JacobianPoint, other: AffinePoint) -> JacobianPoint {
    let p = curve.p;
    if point.z == 0 {
        return JacobianPoint {
            x: other.x % p,
            y: other.y % p,
            z: 1,
        };
    }

    let z1z1 = mul_mod(point.z, point.z, p);
    let u2 = mul_mod(other.x, z1z1, p);
    let s2 = mul_mod(other.y, mul_mod(point.z, z1z1, p), p);
    let h = sub_mod(u2, point.x, p);
    let r = sub_mod(s2, point.y, p);

    if h == 0 {
        if r == 0 {
            return double(curve, point);
        }
        return JacobianPoint::infinity();
    }

    let hh = mul_mod(h, h, p);
    let hhh = mul_mod(h, hh, p);
    let v = mul_mod(point.x, hh, p);
    let x3 = sub_mod(sub_mod(mul_mod(r, r, p), hhh, p), mul_mod(2, v, p), p);
    let y3 = sub_mod(
        mul_mod(r, sub_mod(v, x3, p), p),
        mul_mod(point.y, hhh, p),
        p,
    );
    let z3 = mul_mod(point.z, h, p);

    JacobianPoint {
        x: x3,
        y: y3,
        z: z3,
    }
}

/// Adds two Jacobian points.
pub fn add(curve: &Curve, left: JacobianPoint, right: JacobianPoint) -> JacobianPoint {
    let p = curve.p;
    if left.z == 0 {
        return right;
    }
    if right.z == 0 {
        return left;
    }

    let z1z1 = mul_mod(left.z, left.z, p);
    let z2z2 = mul_mod(right.z, right.z, p);
    let u1 = mul_mod(left.x, z2z2, p);
    let u2 = mul_mod(right.x, z1z1, p);
    let s1 = mul_mod(left.y, mul_mod(right.z, z2z2, p), p);
    let s2 = mul_mod(right.y, mul_mod(left.z, z1z1, p), p);
    let h = sub_mod(u2, u1, p);
    let r = sub_mod(s2, s1, p);

    if h == 0 {
        if r == 0 {
            return double(curve, left);
        }
        return JacobianPoint::infinity();
    }

    let hh = mul_mod(h, h, p);
    let hhh = mul_mod(h, hh, p);
    let v = mul_mod(u1, hh, p);
    let x3 = sub_mod(sub_mod(mul_mod(r, r, p), hhh, p), mul_mod(2, v, p), p);
    let y3 = sub_mod(mul_mod(r, sub_mod(v, x3, p), p), mul_mod(s1, hhh, p), p);
    let z3 = mul_mod(mul_mod(left.z, right.z, p), h, p);

    JacobianPoint {
        x: x3,
        y: y3,
        z: z3,
    }
}

/// Converts a Jacobian point to affine coordinates.
///
/// Returns `None` for the point at infinity.
pub fn to_affine(curve: &Curve, point: JacobianPoint) -> Option<AffinePoint> {
    if point.z == 0 {
        return None;
    }
    let zinv = inv_mod(point.z, curve.p);
    let zinv2 = mul_mod(zinv, zinv, curve.p);
    Some(AffinePoint {
        x: mul_mod(point.x, zinv2, curve.p),
        y: mul_mod(point.y, mul_mod(zinv2, zinv, curve.p), curve.p),
    })
}

/// Multiplies an affine base point by a scalar with double-and-add.
pub fn scalar_mul(curve: &Curve, base: AffinePoint, scalar: u64) -> JacobianPoint {
    let mut result = JacobianPoint::infinity();
    for bit in (0..64).rev() {
        result = double(curve, result);
        if (scalar >> bit) & 1 == 1 {
            result = add_affine(curve, result, base);
        }
    }
    result
}

/// Compares a Jacobian point with an affine target without field inversion.
pub fn matches_affine(point: JacobianPoint, target: AffinePoint, p: u64) -> bool {
    if point.z == 0 {
        return false;
    }
    let z2 = mul_mod(point.z, point.z, p);
    if point.x != mul_mod(target.x, z2, p) {
        return false;
    }
    let z3 = mul_mod(z2, point.z, p);
    point.y == mul_mod(target.y, z3, p)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn toy_curve() -> (Curve, AffinePoint) {
        (Curve { p: 17, a: 2, b: 2 }, AffinePoint { x: 5, y: 1 })
    }

    #[test]
    fn known_point_is_on_curve_and_has_order_nineteen() {
        let (curve, base) = toy_curve();
        assert!(is_on_curve(&curve, base));
        assert_eq!(
            to_affine(&curve, scalar_mul(&curve, base, 2)),
            Some(AffinePoint { x: 6, y: 3 })
        );
        assert!(scalar_mul(&curve, base, 19).is_infinity());
    }

    #[test]
    fn affine_and_jacobian_additions_agree() {
        let (curve, base) = toy_curve();
        let point = scalar_mul(&curve, base, 7);
        let next = add_affine(&curve, point, base);
        assert_eq!(
            to_affine(&curve, next),
            to_affine(&curve, scalar_mul(&curve, base, 8))
        );
        assert!(matches_affine(
            next,
            to_affine(&curve, next).unwrap(),
            curve.p
        ));
    }

    #[test]
    fn infinity_is_reported_as_none() {
        let (curve, _) = toy_curve();
        assert_eq!(to_affine(&curve, JacobianPoint::infinity()), None);
    }
}
