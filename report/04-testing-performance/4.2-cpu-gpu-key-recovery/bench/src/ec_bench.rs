//! Toy ECDLP recovery by brute-force scalar search over a known-order subgroup.

use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use crate::ec::{self, AffinePoint, Curve};
use crate::field::is_prime_u64;

const BLOCK_K: u64 = 65_536;

/// Search strategy for the ECDLP toy benchmark.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EcMode {
    /// Scan the complete subgroup to measure throughput.
    Full,
    /// Stop at the first matching scalar to measure recovery time.
    Recovery,
}

/// Result of one ECDLP toy search.
#[derive(Clone, Debug)]
pub struct EcOutcome {
    /// Number of scalars in the search space.
    pub key_space: u64,
    /// Number of scalars actually tested.
    pub candidates: u64,
    /// Recovered scalar, or zero when no match was found.
    pub scalar: u64,
    /// Whether a matching scalar was found.
    pub recovered: bool,
    /// Whether the recovered scalar satisfies `scalar * G = Q`.
    pub verified: bool,
    /// Wall time of the search.
    pub elapsed: Duration,
}

#[expect(
    clippy::too_many_arguments,
    reason = "the worker shares the search state"
)]
fn worker(
    curve: &Curve,
    base: AffinePoint,
    q: u64,
    public_key: AffinePoint,
    mode: EcMode,
    next_block: &AtomicUsize,
    found: &AtomicBool,
    scalar: &AtomicU64,
    blocks: u64,
) -> u64 {
    let mut candidates = 0u64;

    loop {
        let block = next_block.fetch_add(1, Ordering::Relaxed) as u64;
        if block >= blocks {
            break;
        }
        if mode == EcMode::Recovery && found.load(Ordering::Relaxed) {
            break;
        }

        let first_k = block * BLOCK_K + 1;
        let last_k = (first_k + BLOCK_K).min(q);
        let mut point = ec::scalar_mul(curve, base, first_k);

        for k in first_k..last_k {
            if mode == EcMode::Recovery && found.load(Ordering::Relaxed) {
                break;
            }

            candidates += 1;
            if ec::matches_affine(point, public_key, curve.p) {
                let _ = scalar.compare_exchange(0, k, Ordering::Relaxed, Ordering::Relaxed);
                found.store(true, Ordering::Relaxed);
                if mode == EcMode::Recovery {
                    break;
                }
            }

            point = ec::add_affine(curve, point, base);
        }
    }

    candidates
}

/// Recovers the discrete logarithm `target` from `public_key = target * base`.
///
/// The subgroup order `q` must be prime and `target` must satisfy
/// `0 < target < q`. `threads` must be at least one; the value is clamped to
/// one when zero. The search compares Jacobian points against the affine public
/// key without field inversion.
///
/// Returns an error when the subgroup order is not prime, the base point is not
/// on the curve, the base point does not have order `q`, or the public key is the
/// point at infinity.
pub fn recover_scalar(
    curve: &Curve,
    base: AffinePoint,
    q: u64,
    target: u64,
    mode: EcMode,
    threads: usize,
) -> Result<EcOutcome, String> {
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
    let threads = threads.max(1);
    let key_space = q - 1;
    let blocks = key_space.div_ceil(BLOCK_K);

    let found = AtomicBool::new(false);
    let scalar = AtomicU64::new(0);
    let next_block = AtomicUsize::new(0);

    let start = Instant::now();
    let candidates = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..threads)
            .map(|_| {
                scope.spawn(|| {
                    worker(
                        curve,
                        base,
                        q,
                        public_key,
                        mode,
                        &next_block,
                        &found,
                        &scalar,
                        blocks,
                    )
                })
            })
            .collect();

        handles
            .into_iter()
            .map(|handle| match handle.join() {
                Ok(value) => value,
                Err(payload) => std::panic::resume_unwind(payload),
            })
            .sum::<u64>()
    });
    let elapsed = start.elapsed();

    let recovered = found.load(Ordering::Relaxed);
    let scalar = scalar.load(Ordering::Relaxed);
    let verified = recovered
        && scalar == target
        && scalar > 0
        && scalar < q
        && ec::to_affine(curve, ec::scalar_mul(curve, base, scalar)) == Some(public_key);

    Ok(EcOutcome {
        key_space,
        candidates,
        scalar,
        recovered,
        verified,
        elapsed,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn toy_curve() -> (Curve, AffinePoint, u64) {
        (Curve { p: 17, a: 2, b: 2 }, AffinePoint { x: 5, y: 1 }, 19)
    }

    #[test]
    fn recovers_small_scalar() {
        let (curve, base, q) = toy_curve();
        let result = recover_scalar(&curve, base, q, 7, EcMode::Recovery, 1).unwrap();
        assert_eq!(result.scalar, 7);
        assert!(result.recovered);
        assert!(result.verified);
    }

    #[test]
    fn full_scan_is_consistent_across_threads() {
        let (curve, base, q) = toy_curve();
        let single = recover_scalar(&curve, base, q, 13, EcMode::Full, 1).unwrap();
        let multi = recover_scalar(&curve, base, q, 13, EcMode::Full, 4).unwrap();
        assert_eq!(single.scalar, 13);
        assert_eq!(multi.scalar, 13);
        assert_eq!(single.candidates, multi.candidates);
        assert!(single.verified && multi.verified);
    }

    #[test]
    fn rejects_invalid_target() {
        let (curve, base, q) = toy_curve();
        assert!(recover_scalar(&curve, base, q, 0, EcMode::Recovery, 1).is_err());
        assert!(recover_scalar(&curve, base, q, q, EcMode::Recovery, 1).is_err());
    }
}
