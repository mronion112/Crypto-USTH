use std::env;
use std::process::ExitCode;

use toy_key_recovery::ec::{AffinePoint, Curve};
use toy_key_recovery::{EcMode, RsaMode, factor_toy, recover_scalar};

fn parse_u64(value: Option<&String>, name: &str) -> Result<u64, String> {
    value
        .ok_or_else(|| format!("missing value for {name}"))?
        .parse::<u64>()
        .map_err(|_| format!("invalid integer for {name}"))
}

fn parse_mode(value: Option<&String>, name: &str) -> Result<String, String> {
    let mode = value.ok_or_else(|| format!("missing value for {name}"))?;
    if mode == "full" || mode == "recovery" {
        Ok(mode.clone())
    } else {
        Err(format!("invalid {name}: {mode}"))
    }
}

#[expect(clippy::too_many_arguments, reason = "the CSV row is kept flat")]
fn print_row(
    label: &str,
    mode: &str,
    threads: usize,
    bits: u64,
    key_space: u64,
    candidates: u64,
    wall_ns: u128,
    recovered: bool,
    verified: bool,
    value: u64,
) {
    println!(
        "{label},{mode},{threads},{bits},{key_space},{candidates},{wall_ns},{},{},{value}",
        recovered as u8, verified as u8
    );
}

fn run_rsa(args: &[String]) -> Result<(), String> {
    let mut n = 0u64;
    let mut bits = 0u64;
    let mut threads = 1usize;
    let mut mode = "recovery".to_string();
    let mut label = "rsa".to_string();

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
            "--threads" => {
                threads = parse_u64(args.get(index + 1), "--threads")? as usize;
                index += 2;
            }
            "--mode" => {
                mode = parse_mode(args.get(index + 1), "--mode")?;
                index += 2;
            }
            "--label" => {
                label = args
                    .get(index + 1)
                    .ok_or_else(|| "missing value for --label".to_string())?
                    .clone();
                index += 2;
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    if n < 9 || bits == 0 || threads == 0 {
        return Err("rsa requires --n >= 9, --bits, and --threads".to_string());
    }

    let mode = if mode == "full" {
        RsaMode::Full
    } else {
        RsaMode::Recovery
    };
    let outcome = factor_toy(n, mode, threads);
    print_row(
        &label,
        &mode_name(mode),
        threads,
        bits,
        outcome.key_space,
        outcome.candidates,
        outcome.elapsed.as_nanos(),
        outcome.recovered,
        outcome.verified,
        outcome.factor,
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
    let mut threads = 1usize;
    let mut mode = "recovery".to_string();
    let mut label = "ecc".to_string();

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
            "--threads" => {
                threads = parse_u64(args.get(index + 1), "--threads")? as usize;
                index += 2;
            }
            "--mode" => {
                mode = parse_mode(args.get(index + 1), "--mode")?;
                index += 2;
            }
            "--label" => {
                label = args
                    .get(index + 1)
                    .ok_or_else(|| "missing value for --label".to_string())?
                    .clone();
                index += 2;
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    if p < 5 || q < 2 || d == 0 || d >= q || bits == 0 || threads == 0 {
        return Err("ecc requires valid --p, --q, --d, --bits, and --threads".to_string());
    }

    let mode = if mode == "full" {
        EcMode::Full
    } else {
        EcMode::Recovery
    };
    let curve = Curve { p, a, b };
    let base = AffinePoint { x: gx, y: gy };
    let outcome = recover_scalar(&curve, base, q, d, mode, threads)?;
    print_row(
        &label,
        &mode_name(mode),
        threads,
        bits,
        outcome.key_space,
        outcome.candidates,
        outcome.elapsed.as_nanos(),
        outcome.recovered,
        outcome.verified,
        outcome.scalar,
    );
    Ok(())
}

fn mode_name(mode: impl Into<ModeName>) -> String {
    mode.into().0
}

struct ModeName(String);

impl From<RsaMode> for ModeName {
    fn from(value: RsaMode) -> Self {
        match value {
            RsaMode::Full => Self("full".to_string()),
            RsaMode::Recovery => Self("recovery".to_string()),
        }
    }
}

impl From<EcMode> for ModeName {
    fn from(value: EcMode) -> Self {
        match value {
            EcMode::Full => Self("full".to_string()),
            EcMode::Recovery => Self("recovery".to_string()),
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let Some(command) = args.first() else {
        eprintln!("usage: toy-bench rsa ... | toy-bench ecc ...");
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
