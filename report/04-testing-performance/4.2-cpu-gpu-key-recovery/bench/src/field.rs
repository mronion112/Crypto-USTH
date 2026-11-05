//! Modular arithmetic helpers for toy fields and 64-bit primality testing.

/// Adds two field elements; requires `p < 2^32` so the intermediate sum fits.
pub fn add_mod(left: u64, right: u64, p: u64) -> u64 {
    let left = left % p;
    let right = right % p;
    if left >= p - right {
        left - (p - right)
    } else {
        left + right
    }
}

/// Subtracts two field elements; requires `p < 2^32`.
pub fn sub_mod(left: u64, right: u64, p: u64) -> u64 {
    let left = left % p;
    let right = right % p;
    if left >= right {
        left - right
    } else {
        left + p - right
    }
}

/// Multiplies two field elements; the product must fit in `u64`, so `p < 2^32`.
pub fn mul_mod(left: u64, right: u64, p: u64) -> u64 {
    left.wrapping_mul(right) % p
}

/// Computes `base^exponent mod p` by square-and-multiply.
pub fn pow_mod(mut base: u64, mut exponent: u64, p: u64) -> u64 {
    let mut result = 1 % p;
    base %= p;
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = mul_mod(result, base, p);
        }
        base = mul_mod(base, base, p);
        exponent >>= 1;
    }
    result
}

/// Computes the inverse modulo a prime `p` with Fermat's little theorem.
pub fn inv_mod(value: u64, p: u64) -> u64 {
    pow_mod(value, p - 2, p)
}

fn mul_mod_wide(left: u64, right: u64, modulus: u64) -> u64 {
    ((left as u128 * right as u128) % modulus as u128) as u64
}

fn pow_mod_wide(base: u64, exponent: u64, modulus: u64) -> u64 {
    let mut result = 1 % modulus;
    let mut base = base % modulus;
    let mut exponent = exponent;
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = mul_mod_wide(result, base, modulus);
        }
        base = mul_mod_wide(base, base, modulus);
        exponent >>= 1;
    }
    result
}

/// Deterministic Miller-Rabin test for all `u64` inputs using the first 12 primes.
pub fn is_prime_u64(value: u64) -> bool {
    const BASES: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

    if value < 2 {
        return false;
    }
    for base in BASES {
        if value == base {
            return true;
        }
        if value.is_multiple_of(base) {
            return false;
        }
    }

    let mut odd = value - 1;
    let mut shifts = 0u32;
    while odd & 1 == 0 {
        odd >>= 1;
        shifts += 1;
    }

    for base in BASES {
        let mut witness = pow_mod_wide(base % value, odd, value);
        if witness == 1 || witness == value - 1 {
            continue;
        }

        let mut composite = true;
        for _ in 1..shifts {
            witness = mul_mod_wide(witness, witness, value);
            if witness == value - 1 {
                composite = false;
                break;
            }
        }
        if composite {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modular_operations_match_small_examples() {
        let p = 17;
        assert_eq!(add_mod(16, 2, p), 1);
        assert_eq!(sub_mod(1, 2, p), 16);
        assert_eq!(mul_mod(5, 7, p), 1);
        assert_eq!(pow_mod(2, 8, p), 1);
        assert_eq!(inv_mod(5, p), 7);
    }

    #[test]
    fn primality_handles_boundaries() {
        assert!(!is_prime_u64(0));
        assert!(!is_prime_u64(1));
        assert!(is_prime_u64(2));
        assert!(is_prime_u64(3));
        assert!(!is_prime_u64(4));
        assert!(is_prime_u64(104729));
        assert!(!is_prime_u64(104730));
        assert!(is_prime_u64(4_294_967_291));
    }

    #[test]
    fn integer_square_root_is_floor() {
        assert_eq!(0u64.isqrt(), 0);
        assert_eq!(1u64.isqrt(), 1);
        assert_eq!(15u64.isqrt(), 3);
        assert_eq!(16u64.isqrt(), 4);
        assert_eq!(17u64.isqrt(), 4);
    }
}
