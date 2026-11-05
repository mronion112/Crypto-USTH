use toy_key_recovery::ec::{AffinePoint, Curve};
use toy_key_recovery::{
    EcMode, RsaMode, factor_rho, factor_toy, recover_scalar, recover_scalar_rho,
};

#[test]
fn rsa_api_recovers_and_verifies() {
    let outcome = factor_toy(10_403, RsaMode::Recovery, 2);
    assert_eq!(outcome.factor, 101);
    assert!(outcome.recovered);
    assert!(outcome.verified);
}

#[test]
fn ecc_api_recovers_and_verifies() {
    let curve = Curve { p: 17, a: 2, b: 2 };
    let base = AffinePoint { x: 5, y: 1 };
    let outcome = recover_scalar(&curve, base, 19, 7, EcMode::Recovery, 2).unwrap();
    assert_eq!(outcome.scalar, 7);
    assert!(outcome.recovered);
    assert!(outcome.verified);
}

#[test]
fn rho_rsa_api_recovers_and_verifies() {
    let outcome = factor_rho(10_403, 100_000, 7);
    assert!(outcome.recovered);
    assert!(outcome.verified);
}

#[test]
fn rho_ecc_api_recovers_and_verifies() {
    let curve = Curve { p: 17, a: 2, b: 2 };
    let base = AffinePoint { x: 5, y: 1 };
    let outcome = recover_scalar_rho(&curve, base, 19, 7, 100_000, 8).unwrap();
    assert_eq!(outcome.value, 7);
    assert!(outcome.recovered);
    assert!(outcome.verified);
}
