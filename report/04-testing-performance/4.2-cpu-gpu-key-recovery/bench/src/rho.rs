//! Pollard rho key recovery for toy RSA and ECDLP instances.
//!
//! The routines here are CPU-only. RSA uses Brent's variant of Pollard rho
//! with batched gcd checks. ECDLP uses Floyd cycle detection over randomized
//! affine walks. Both routines verify the recovered value before reporting
//! success.

use std::time::{Duration, Instant};

use crate::ec::{self, AffinePoint, Curve};
use crate::field::{add_mod, inv_mod, is_prime_u64, mul_mod, sub_mod};

const MAX_RESTARTS: u32 = 32;
const BRENT_BATCH: u64 = 128;

/// Result of one Pollard rho recovery attempt.
#[derive(Clone, Debug)]
pub struct RhoOutcome {
    /// Number of group operations or polynomial evaluations performed.
    pub steps: u64,
    /// Recovered factor or scalar; zero when recovery failed.
    pub value: u64,
    /// Whether a candidate was found.
    pub recovered: bool,
    /// Whether the recovered candidate passed verification.
    pub verified: bool,
    /// Wall time of the complete search, including restarts.
    pub elapsed: Duration,
    /// Number of randomized restarts used.
    pub restarts: u32,
}

struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed.max(1))
    }

    fn next_u64(&mut self) -> u64 {
        let mut value = self.0;
        value ^= value >> 12;
        value ^= value << 25;
        value ^= value >> 27;
        self.0 = value;
        value.wrapping_mul(2_685_821_657_736_338_717)
    }

    fn below(&mut self, bound: u64) -> u64 {
        if bound == 0 {
            0
        } else {
            self.next_u64() % bound
        }
    }
}

fn gcd(mut left: u64, mut right: u64) -> u64 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left
}

fn mul_mod_wide(left: u64, right: u64, modulus: u64) -> u64 {
    ((left as u128 * right as u128) % modulus as u128) as u64
}

fn add_mod_wide(left: u64, right: u64, modulus: u64) -> u64 {
    ((left as u128 + right as u128) % modulus as u128) as u64
}

fn rho_step(value: u64, modulus: u64, constant: u64) -> u64 {
    add_mod_wide(mul_mod_wide(value, value, modulus), constant, modulus)
}

fn finish_outcome(
    start: Instant,
    steps: u64,
    value: u64,
    recovered: bool,
    verified: bool,
    restarts: u32,
) -> RhoOutcome {
    RhoOutcome {
        steps,
        value,
        recovered,
        verified,
        elapsed: start.elapsed(),
        restarts,
    }
}

/// Factors a toy semiprime with Brent's Pollard rho algorithm.
///
/// `max_steps` bounds the total number of polynomial evaluations across all
/// restarts. The returned factor is verified with `factor * (n / factor) = n`
/// and deterministic Miller-Rabin primality checks.
pub fn factor_rho(n: u64, max_steps: u64, seed: u64) -> RhoOutcome {
    let start = Instant::now();

    if n < 4 || max_steps == 0 || is_prime_u64(n) {
        return finish_outcome(start, 0, 0, false, false, 0);
    }
    for small in [2u64, 3] {
        if n.is_multiple_of(small) {
            let other = n / small;
            let verified = is_prime_u64(small) && is_prime_u64(other);
            return finish_outcome(start, 1, small, true, verified, 0);
        }
    }

    let mut rng = Rng::new(seed);
    let mut steps = 0u64;
    let mut restarts = 0u32;

    for _ in 0..MAX_RESTARTS {
        if steps >= max_steps {
            break;
        }
        restarts += 1;

        let constant = 1 + rng.below(n - 1);
        let mut y = 2 + rng.below(n - 2);
        let mut x = 0u64;
        let mut ys = 0u64;
        let mut g = 1u64;
        let mut r = 1u64;
        let mut q = 1u64;

        while g == 1 && steps < max_steps {
            x = y;
            for _ in 0..r {
                y = rho_step(y, n, constant);
                steps += 1;
                if steps >= max_steps {
                    break;
                }
            }

            let mut k = 0u64;
            while k < r && g == 1 && steps < max_steps {
                ys = y;
                let limit = BRENT_BATCH.min(r - k);
                for _ in 0..limit {
                    y = rho_step(y, n, constant);
                    steps += 1;
                    q = mul_mod_wide(q, x.abs_diff(y), n);
                }
                g = gcd(q, n);
                k += limit;
            }
            r = r.saturating_mul(2);
        }

        if g == n {
            while steps < max_steps {
                ys = rho_step(ys, n, constant);
                steps += 1;
                g = gcd(x.abs_diff(ys), n);
                if g > 1 {
                    break;
                }
            }
        }

        if g > 1 && g < n {
            let other = n / g;
            let verified = n.is_multiple_of(g) && is_prime_u64(g) && is_prime_u64(other);
            return finish_outcome(start, steps, g, true, verified, restarts);
        }
    }

    finish_outcome(start, steps, 0, false, false, restarts)
}

#[derive(Clone, Copy)]
struct Walk {
    point: AffinePoint,
    coefficient_g: u64,
    coefficient_q: u64,
}

fn double_affine(curve: &Curve, point: AffinePoint) -> Option<AffinePoint> {
    if point.y == 0 {
        return None;
    }
    let p = curve.p;
    let xx = mul_mod(point.x, point.x, p);
    let numerator = add_mod(mul_mod(3, xx, p), curve.a, p);
    let denominator = mul_mod(2, point.y, p);
    let slope = mul_mod(numerator, inv_mod(denominator, p), p);
    let x3 = sub_mod(mul_mod(slope, slope, p), mul_mod(2, point.x, p), p);
    let y3 = sub_mod(mul_mod(slope, sub_mod(point.x, x3, p), p), point.y, p);
    Some(AffinePoint { x: x3, y: y3 })
}

fn add_affine_points(curve: &Curve, left: AffinePoint, right: AffinePoint) -> Option<AffinePoint> {
    if left == right {
        return double_affine(curve, left);
    }
    if left.x == right.x {
        return None;
    }
    let p = curve.p;
    let dx = sub_mod(right.x, left.x, p);
    let dy = sub_mod(right.y, left.y, p);
    let slope = mul_mod(dy, inv_mod(dx, p), p);
    let x3 = sub_mod(sub_mod(mul_mod(slope, slope, p), left.x, p), right.x, p);
    let y3 = sub_mod(mul_mod(slope, sub_mod(left.x, x3, p), p), left.y, p);
    Some(AffinePoint { x: x3, y: y3 })
}

fn walk_step(
    curve: &Curve,
    base: AffinePoint,
    public_key: AffinePoint,
    q: u64,
    walk: Walk,
) -> Option<Walk> {
    match walk.point.x % 3 {
        0 => {
            let point = add_affine_points(curve, walk.point, base)?;
            Some(Walk {
                point,
                coefficient_g: add_mod(walk.coefficient_g, 1, q),
                coefficient_q: walk.coefficient_q,
            })
        }
        1 => {
            let point = add_affine_points(curve, walk.point, public_key)?;
            Some(Walk {
                point,
                coefficient_g: walk.coefficient_g,
                coefficient_q: add_mod(walk.coefficient_q, 1, q),
            })
        }
        _ => {
            let point = double_affine(curve, walk.point)?;
            Some(Walk {
                point,
                coefficient_g: mul_mod(2, walk.coefficient_g, q),
                coefficient_q: mul_mod(2, walk.coefficient_q, q),
            })
        }
    }
}

/// Recovers the ECDLP scalar `target` from `public_key = target * base`.
///
/// The search uses Pollard rho with Floyd cycle detection. `max_steps` bounds
/// the total number of affine group operations across restarts. The subgroup
/// order must be prime, and the recovered scalar is verified with `target * G = Q`.
pub fn recover_scalar_rho(
    curve: &Curve,
    base: AffinePoint,
    q: u64,
    target: u64,
    max_steps: u64,
    seed: u64,
) -> Result<RhoOutcome, String> {
    if q < 2 || target == 0 || target >= q {
        return Err("invalid target or subgroup order".to_string());
    }
    if !is_prime_u64(q) {
        return Err("toy subgroup order must be prime".to_string());
    }
    if !ec::is_on_curve(curve, base) {
        return Err("base point is not on the toy curve".to_string());
    }
    if !ec::scalar_mul(curve, base, q).is_infinity() {
        return Err("base point does not have the declared order".to_string());
    }

    let public_key = ec::to_affine(curve, ec::scalar_mul(curve, base, target))
        .ok_or_else(|| "public key is the point at infinity".to_string())?;
    let start = Instant::now();
    let mut rng = Rng::new(seed);
    let mut steps = 0u64;
    let mut restarts = 0u32;

    if max_steps == 0 {
        return Ok(finish_outcome(start, 0, 0, false, false, 0));
    }

    for _ in 0..MAX_RESTARTS {
        if steps >= max_steps {
            break;
        }
        restarts += 1;

        let a0 = rng.below(q);
        let b0 = rng.below(q);
        let start_point = ec::to_affine(
            curve,
            ec::add(
                curve,
                ec::scalar_mul(curve, base, a0),
                ec::scalar_mul(curve, public_key, b0),
            ),
        );
        let Some(point) = start_point else {
            continue;
        };

        let start_walk = Walk {
            point,
            coefficient_g: a0,
            coefficient_q: b0,
        };
        let mut tortoise = start_walk;
        let mut hare = start_walk;

        loop {
            if steps >= max_steps {
                break;
            }
            let Some(next_tortoise) = walk_step(curve, base, public_key, q, tortoise) else {
                break;
            };
            tortoise = next_tortoise;
            steps += 1;

            if steps >= max_steps {
                break;
            }
            let Some(next_hare) = walk_step(curve, base, public_key, q, hare) else {
                break;
            };
            let Some(next_hare) = walk_step(curve, base, public_key, q, next_hare) else {
                break;
            };
            hare = next_hare;
            steps += 2;

            if tortoise.point != hare.point {
                continue;
            }

            let denominator = sub_mod(hare.coefficient_q, tortoise.coefficient_q, q);
            if denominator == 0 {
                break;
            }
            let numerator = sub_mod(tortoise.coefficient_g, hare.coefficient_g, q);
            let scalar = mul_mod(numerator, inv_mod(denominator, q), q);
            if scalar == 0 || scalar >= q {
                break;
            }
            let verified =
                ec::to_affine(curve, ec::scalar_mul(curve, base, scalar)) == Some(public_key);
            if verified {
                return Ok(finish_outcome(start, steps, scalar, true, true, restarts));
            }
            break;
        }
    }

    Ok(finish_outcome(start, steps, 0, false, false, restarts))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factor_rho_recovers_small_semiprime() {
        let outcome = factor_rho(10_403, 100_000, 1);
        assert!(outcome.recovered);
        assert!(outcome.verified);
        assert!(outcome.value == 101 || outcome.value == 103);
        assert!(outcome.steps > 0);
    }

    #[test]
    fn factor_rho_does_not_claim_prime_factorization() {
        let outcome = factor_rho(104_729, 1_000, 2);
        assert!(!outcome.recovered);
        assert!(!outcome.verified);
    }

    #[test]
    fn rho_ecdlp_recovers_small_scalar() {
        let curve = Curve { p: 17, a: 2, b: 2 };
        let base = AffinePoint { x: 5, y: 1 };
        let outcome = recover_scalar_rho(&curve, base, 19, 7, 100_000, 3).unwrap();
        assert!(outcome.recovered);
        assert!(outcome.verified);
        assert_eq!(outcome.value, 7);
    }

    #[test]
    fn rho_ecdlp_rejects_invalid_target() {
        let curve = Curve { p: 17, a: 2, b: 2 };
        let base = AffinePoint { x: 5, y: 1 };
        assert!(recover_scalar_rho(&curve, base, 19, 0, 100_000, 4).is_err());
        assert!(recover_scalar_rho(&curve, base, 19, 19, 100_000, 5).is_err());
    }
}
