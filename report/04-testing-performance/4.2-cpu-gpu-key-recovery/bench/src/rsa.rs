//! Toy RSA modulus recovery by trial division over `6k - 1` and `6k + 1`.

use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use crate::field::is_prime_u64;

const BLOCK_K: u64 = 65_536;

/// Search strategy for the RSA toy benchmark.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RsaMode {
    /// Scan the complete candidate space to measure throughput.
    Full,
    /// Stop at the first nontrivial factor to measure recovery time.
    Recovery,
}

/// Result of one RSA toy search.
#[derive(Clone, Debug)]
pub struct RsaOutcome {
    /// Number of candidates in the complete search space.
    pub key_space: u64,
    /// Number of candidates actually tested.
    pub candidates: u64,
    /// Recovered factor, or zero when no factor was found.
    pub factor: u64,
    /// Whether a factor was found.
    pub recovered: bool,
    /// Whether `factor * (n / factor) = n` and both factors pass primality checks.
    pub verified: bool,
    /// Wall time of the search.
    pub elapsed: Duration,
}

fn try_candidate(n: u64, divisor: u64, factor: &AtomicU64, found: &AtomicBool) -> bool {
    if divisor < 2 || !n.is_multiple_of(divisor) {
        return false;
    }
    let _ = factor.compare_exchange(0, divisor, Ordering::Relaxed, Ordering::Relaxed);
    found.store(true, Ordering::Relaxed);
    true
}

fn worker(
    n: u64,
    kmax: u64,
    blocks: u64,
    mode: RsaMode,
    next_block: &AtomicUsize,
    found: &AtomicBool,
    factor: &AtomicU64,
) -> u64 {
    let mut candidates = 0u64;

    loop {
        let block = next_block.fetch_add(1, Ordering::Relaxed) as u64;
        if block >= blocks {
            break;
        }
        if mode == RsaMode::Recovery && found.load(Ordering::Relaxed) {
            break;
        }

        let first_k = block * BLOCK_K + 1;
        let last_k = (first_k + BLOCK_K).min(kmax + 1);

        for k in first_k..last_k {
            if mode == RsaMode::Recovery && found.load(Ordering::Relaxed) {
                break;
            }

            candidates += 1;
            if try_candidate(n, 6 * k - 1, factor, found) && mode == RsaMode::Recovery {
                break;
            }

            candidates += 1;
            if try_candidate(n, 6 * k + 1, factor, found) && mode == RsaMode::Recovery {
                break;
            }
        }
    }

    candidates
}

/// Searches for a nontrivial factor of `n` with trial division.
///
/// `mode` selects a full scan or an early-exit recovery search. `threads` must
/// be at least one; the value is clamped to one when zero. The search tests
/// candidates of the form `6k - 1` and `6k + 1` up to `floor(sqrt(n))`.
///
/// Returns the recovered factor, candidate count, elapsed time, and a
/// verification flag. The verification checks `p * q = n` and deterministic
/// Miller-Rabin primality for both factors.
pub fn factor_toy(n: u64, mode: RsaMode, threads: usize) -> RsaOutcome {
    let threads = threads.max(1);
    let kmax = n.isqrt() / 6 + 2;
    let key_space = kmax * 2;
    let blocks = kmax.div_ceil(BLOCK_K);

    let found = AtomicBool::new(false);
    let factor = AtomicU64::new(0);
    let next_block = AtomicUsize::new(0);

    let start = Instant::now();
    let mut candidates = 0u64;

    if n.is_multiple_of(2) {
        candidates += 1;
        try_candidate(n, 2, &factor, &found);
    }
    if n.is_multiple_of(3) {
        candidates += 1;
        try_candidate(n, 3, &factor, &found);
    }

    if !(mode == RsaMode::Recovery && found.load(Ordering::Relaxed)) {
        let parallel_candidates = std::thread::scope(|scope| {
            let handles: Vec<_> = (0..threads)
                .map(|_| {
                    scope.spawn(|| worker(n, kmax, blocks, mode, &next_block, &found, &factor))
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
        candidates += parallel_candidates;
    }

    let elapsed = start.elapsed();
    let recovered = found.load(Ordering::Relaxed);
    let factor = factor.load(Ordering::Relaxed);
    let verified =
        factor > 1 && n.is_multiple_of(factor) && is_prime_u64(factor) && is_prime_u64(n / factor);

    RsaOutcome {
        key_space,
        candidates,
        factor,
        recovered,
        verified,
        elapsed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recovers_small_semiprime() {
        let result = factor_toy(10_403, RsaMode::Recovery, 1);
        assert_eq!(result.factor, 101);
        assert!(result.recovered);
        assert!(result.verified);
    }

    #[test]
    fn full_scan_is_consistent_across_threads() {
        let single = factor_toy(10_403, RsaMode::Full, 1);
        let multi = factor_toy(10_403, RsaMode::Full, 4);
        assert_eq!(single.factor, 101);
        assert_eq!(multi.factor, 101);
        assert_eq!(single.candidates, multi.candidates);
        assert!(single.verified && multi.verified);
    }

    #[test]
    fn handles_small_composite_with_precheck() {
        let result = factor_toy(15, RsaMode::Recovery, 1);
        assert_eq!(result.factor, 3);
        assert!(result.verified);
    }
}
