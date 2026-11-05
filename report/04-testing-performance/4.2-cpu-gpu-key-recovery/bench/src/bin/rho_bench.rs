use std::env;
use std::process::ExitCode;

use toy_key_recovery::ec::{AffinePoint, Curve};
use toy_key_recovery::{factor_rho, recover_scalar_rho};

const DEFAULT_MAX_STEPS: u64 = 10_000_000;

fn parse_u64(value: Option<&String>, name: &str) -> Result<u64, String> {
    value
        .ok_or_else(|| format!("missing value for {name}"))?
        .parse::<u64>()
        .map_err(|_| format!("invalid integer for {name}"))
}

#[expect(clippy::too_many_arguments, reason = "the CSV row is kept flat")]
fn print_row(
    label: &str,
    bits: u64,
    seed: u64,
    steps: u64,
    wall_ns: u128,
    recovered: bool,
    verified: bool,
    value: u64,
    restarts: u32,
) {
    println!(
        "{label},{bits},{seed},{steps},{wall_ns},{},{},{value},{restarts}",
        recovered as u8, verified as u8
    );
}

fn run_rsa(args: &[String]) -> Result<(), String> {
    let mut n = 0u64;
    let mut bits = 0u64;
    let mut seed = 1u64;
    let mut max_steps = DEFAULT_MAX_STEPS;

    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--n" => {
                n = parse_u64(args.get(index + 1), "--n")?;
                index += 2;
            }
            "--bits" => {
                bits = parse_u64(args.get(index + 1), "--bits")?;
                index += 2;
            }
            "--seed" => {
                seed = parse_u64(args.get(index + 1), "--seed")?;
                index += 2;
            }
            "--max-steps" => {
                max_steps = parse_u64(args.get(index + 1), "--max-steps")?;
                index += 2;
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    if n < 9 || bits == 0 {
        return Err("rsa requires --n >= 9 and --bits".to_string());
    }

    let outcome = factor_rho(n, max_steps, seed);
    print_row(
        "rsa",
        bits,
        seed,
        outcome.steps,
        outcome.elapsed.as_nanos(),
        outcome.recovered,
        outcome.verified,
        outcome.value,
        outcome.restarts,
    );
    Ok(())
}

fn run_ecc(args: &[String]) -> Result<(), String> {
    let mut p = 0u64;
    let mut a = 0u64;
    let mut b = 0u64;
    let mut gx = 0u64;
    let mut gy = 0u64;
    let mut q = 0u64;
    let mut d = 0u64;
    let mut bits = 0u64;
    let mut seed = 1u64;
    let mut max_steps = DEFAULT_MAX_STEPS;

    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--p" => {
                p = parse_u64(args.get(index + 1), "--p")?;
                index += 2;
            }
            "--a" => {
                a = parse_u64(args.get(index + 1), "--a")?;
                index += 2;
            }
            "--b" => {
                b = parse_u64(args.get(index + 1), "--b")?;
                index += 2;
            }
            "--gx" => {
                gx = parse_u64(args.get(index + 1), "--gx")?;
                index += 2;
            }
            "--gy" => {
                gy = parse_u64(args.get(index + 1), "--gy")?;
                index += 2;
            }
            "--q" => {
                q = parse_u64(args.get(index + 1), "--q")?;
                index += 2;
            }
            "--d" => {
                d = parse_u64(args.get(index + 1), "--d")?;
                index += 2;
            }
            "--bits" => {
                bits = parse_u64(args.get(index + 1), "--bits")?;
                index += 2;
            }
            "--seed" => {
                seed = parse_u64(args.get(index + 1), "--seed")?;
                index += 2;
            }
            "--max-steps" => {
                max_steps = parse_u64(args.get(index + 1), "--max-steps")?;
                index += 2;
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    if p < 5 || q < 2 || d == 0 || d >= q || bits == 0 {
        return Err("ecc requires valid --p, --q, --d, and --bits".to_string());
    }

    let curve = Curve { p, a, b };
    let base = AffinePoint { x: gx, y: gy };
    let outcome = recover_scalar_rho(&curve, base, q, d, max_steps, seed)?;
    print_row(
        "ecc",
        bits,
        seed,
        outcome.steps,
        outcome.elapsed.as_nanos(),
        outcome.recovered,
        outcome.verified,
        outcome.value,
        outcome.restarts,
    );
    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let Some(command) = args.first() else {
        eprintln!("usage: rho-bench rsa ... | rho-bench ecc ...");
        return ExitCode::from(2);
    };

    let result = match command.as_str() {
        "rsa" => run_rsa(&args[1..]),
        "ecc" => run_ecc(&args[1..]),
        other => Err(format!("unknown command: {other}")),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::from(2)
        }
    }
}
