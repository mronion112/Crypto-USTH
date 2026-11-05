use std::env;
use std::process::ExitCode;

use toy_key_recovery::ec::{self, AffinePoint, Curve};
use toy_key_recovery::field::{is_prime_u64, mul_mod, pow_mod};

struct Rng(u64);

impl Rng {
    fn next_u64(&mut self) -> u64 {
        let mut value = self.0;
        value ^= value >> 12;
        value ^= value << 25;
        value ^= value >> 27;
        self.0 = value;
        value.wrapping_mul(2_685_821_657_736_338_717)
    }

    fn below(&mut self, bound: u64) -> u64 {
        self.next_u64() % bound
    }
}

fn random_prime(rng: &mut Rng, bits: u32) -> u64 {
    let high = 1u64 << (bits - 1);
    let mask = (1u64 << bits) - 1;
    loop {
        let mut candidate = (rng.next_u64() & mask) | high;
        candidate = (candidate & !3) | 3;
        if candidate >= high && is_prime_u64(candidate) {
            return candidate;
        }
    }
}

fn legendre(value: u64, p: u64) -> i64 {
    if value.is_multiple_of(p) {
        return 0;
    }
    let result = pow_mod(value % p, (p - 1) / 2, p);
    if result == 1 {
        1
    } else if result == p - 1 {
        -1
    } else {
        0
    }
}

fn count_points(curve: &Curve, threads: usize) -> u64 {
    let p = curve.p;
    let threads = threads.max(1) as u64;
    let sum: i64 = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..threads)
            .map(|thread| {
                let curve = *curve;
                let start = p * thread / threads;
                let end = p * (thread + 1) / threads;
                scope.spawn(move || {
                    let mut partial = 0i64;
                    for x in start..end {
                        let mut rhs = mul_mod(x, x, curve.p);
                        rhs = mul_mod(rhs, x, curve.p);
                        rhs = (rhs + mul_mod(curve.a, x, curve.p)) % curve.p;
                        rhs = (rhs + curve.b) % curve.p;
                        partial += legendre(rhs, curve.p);
                    }
                    partial
                })
            })
            .collect();

        handles
            .into_iter()
            .map(|handle| match handle.join() {
                Ok(value) => value,
                Err(payload) => std::panic::resume_unwind(payload),
            })
            .sum()
    });
    (p as i64 + 1 + sum) as u64
}

fn largest_prime_factor(n: u64) -> Option<(u64, u32)> {
    let mut remaining = n;
    let mut best = 1u64;
    let mut best_exponent = 0u32;
    let mut divisor = 2u64;

    while divisor * divisor <= remaining {
        if !remaining.is_multiple_of(divisor) {
            divisor += 1;
            continue;
        }
        let mut exponent = 0u32;
        while remaining.is_multiple_of(divisor) {
            remaining /= divisor;
            exponent += 1;
        }
        if divisor > best {
            best = divisor;
            best_exponent = exponent;
        }
        divisor += 1;
    }

    if remaining > 1 && remaining > best {
        best = remaining;
        best_exponent = 1;
    }

    if best > 1 && is_prime_u64(best) {
        Some((best, best_exponent))
    } else {
        None
    }
}

fn find_generator(curve: &Curve, order: u64, q: u64, rng: &mut Rng) -> Option<AffinePoint> {
    for _ in 0..200 {
        let x = rng.below(curve.p);
        let mut rhs = mul_mod(x, x, curve.p);
        rhs = mul_mod(rhs, x, curve.p);
        rhs = (rhs + mul_mod(curve.a, x, curve.p)) % curve.p;
        rhs = (rhs + curve.b) % curve.p;
        if legendre(rhs, curve.p) != 1 {
            continue;
        }

        let y = pow_mod(rhs, (curve.p + 1) / 4, curve.p);
        if mul_mod(y, y, curve.p) != rhs {
            continue;
        }

        let candidate = AffinePoint { x, y };
        let scaled = ec::scalar_mul(curve, candidate, order / q);
        let Some(point) = ec::to_affine(curve, scaled) else {
            continue;
        };
        if ec::scalar_mul(curve, point, q).is_infinity() {
            return Some(point);
        }
    }
    None
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut bits = 0u32;
    let mut seed = 1u64;
    let mut threads = 1usize;

    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--bits" => {
                let Some(value) = args.get(index + 1) else {
                    eprintln!("missing value for --bits");
                    return ExitCode::from(2);
                };
                bits = match value.parse() {
                    Ok(value) => value,
                    Err(_) => {
                        eprintln!("invalid --bits");
                        return ExitCode::from(2);
                    }
                };
                index += 2;
            }
            "--seed" => {
                let Some(value) = args.get(index + 1) else {
                    eprintln!("missing value for --seed");
                    return ExitCode::from(2);
                };
                seed = match value.parse() {
                    Ok(value) => value,
                    Err(_) => {
                        eprintln!("invalid --seed");
                        return ExitCode::from(2);
                    }
                };
                index += 2;
            }
            "--threads" => {
                let Some(value) = args.get(index + 1) else {
                    eprintln!("missing value for --threads");
                    return ExitCode::from(2);
                };
                threads = match value.parse() {
                    Ok(value) => value,
                    Err(_) => {
                        eprintln!("invalid --threads");
                        return ExitCode::from(2);
                    }
                };
                index += 2;
            }
            other => {
                eprintln!("unknown argument: {other}");
                return ExitCode::from(2);
            }
        }
    }

    if !(8..=28).contains(&bits) {
        eprintln!("bits must be between 8 and 28");
        return ExitCode::from(2);
    }

    let mut rng = Rng(if seed == 0 { 1 } else { seed });

    for _ in 0..200 {
        let p = random_prime(&mut rng, bits);
        let curve = Curve {
            p,
            a: 1 + rng.below(p - 1),
            b: 1 + rng.below(p - 1),
        };

        let a2 = mul_mod(curve.a, curve.a, p);
        let a3 = mul_mod(a2, curve.a, p);
        let b2 = mul_mod(curve.b, curve.b, p);
        let discriminant = (4 * a3 + 27 * b2) % p;
        if discriminant == 0 {
            continue;
        }

        let order = count_points(&curve, threads);
        if order == p + 1 {
            continue;
        }

        let Some((q, exponent)) = largest_prime_factor(order) else {
            continue;
        };
        if exponent > 1 || q <= (1u64 << (bits - 3)) {
            continue;
        }

        let Some(generator) = find_generator(&curve, order, q, &mut rng) else {
            continue;
        };

        println!(
            "{{\"bits\":{bits},\"p\":{},\"a\":{},\"b\":{},\"gx\":{},\"gy\":{},\"q\":{},\"order\":{},\"cofactor\":{},\"seed\":{seed}}}",
            curve.p,
            curve.a,
            curve.b,
            generator.x,
            generator.y,
            q,
            order,
            order / q
        );
        return ExitCode::SUCCESS;
    }

    eprintln!("failed to generate curve for {bits} bits");
    ExitCode::from(1)
}
