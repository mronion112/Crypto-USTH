//! CubeCL CUDA benchmark for the toy RSA and ECDLP key-recovery searches.
//!
//! The kernels are Rust `#[cube]` functions. The host uses CubeCL's CUDA
//! runtime for device discovery, buffers, launch, and readback.

use std::env;
use std::io::{Error as IoError, ErrorKind};
use std::process::ExitCode;
use std::time::Instant;

use cubecl::{Device, prelude::*};
use toy_key_recovery::ec::{self, AffinePoint, Curve};
use toy_key_recovery::field::is_prime_u64;

type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

fn select_device() -> AppResult<Device> {
    if env::var("CUBECL_CPU").is_ok() {
        return Ok(Device::cpu()?);
    }
    Ok(Device::cuda(0)?)
}

#[cube]
fn add_mod(a: u64, b: u64, p: u64) -> u64 {
    let a = a % p;
    let b = b % p;
    if a >= p - b { a - (p - b) } else { a + b }
}

#[cube]
fn sub_mod(a: u64, b: u64, p: u64) -> u64 {
    let a = a % p;
    let b = b % p;
    if a >= b { a - b } else { a + p - b }
}

#[cube]
fn mul_mod(a: u64, b: u64, p: u64) -> u64 {
    (a * b) % p
}

#[cube]
fn infinity() -> (u64, u64, u64) {
    (0u64, 1u64, 0u64)
}

#[cube]
fn ec_double(point: (u64, u64, u64), a: u64, p: u64) -> (u64, u64, u64) {
    let (px, py, pz) = point;
    let mut result = infinity();
    if pz != 0 && py != 0 {
        let xx = mul_mod(px, px, p);
        let yy = mul_mod(py, py, p);
        let yyyy = mul_mod(yy, yy, p);
        let zz = mul_mod(pz, pz, p);
        let z4 = mul_mod(zz, zz, p);
        let sum = add_mod(px, yy, p);
        let square = mul_mod(sum, sum, p);
        let inner = sub_mod(sub_mod(square, xx, p), yyyy, p);
        let s = mul_mod(2, inner, p);
        let m = add_mod(mul_mod(3, xx, p), mul_mod(a, z4, p), p);
        let x3 = sub_mod(mul_mod(m, m, p), mul_mod(2, s, p), p);
        let y3 = sub_mod(mul_mod(m, sub_mod(s, x3, p), p), mul_mod(8, yyyy, p), p);
        let z3 = mul_mod(2, mul_mod(py, pz, p), p);
        result = (x3, y3, z3);
    }
    result
}

#[cube]
fn ec_add_affine(point: (u64, u64, u64), x2: u64, y2: u64, a: u64, p: u64) -> (u64, u64, u64) {
    let (px, py, pz) = point;
    let mut result = infinity();
    if pz == 0 {
        result = (x2 % p, y2 % p, 1);
    } else {
        let z1z1 = mul_mod(pz, pz, p);
        let u2 = mul_mod(x2, z1z1, p);
        let s2 = mul_mod(y2, mul_mod(pz, z1z1, p), p);
        let h = sub_mod(u2, px, p);
        let r = sub_mod(s2, py, p);

        if h == 0 {
            if r == 0 {
                result = ec_double(point, a, p);
            }
        } else {
            let hh = mul_mod(h, h, p);
            let hhh = mul_mod(h, hh, p);
            let v = mul_mod(px, hh, p);
            let x3 = sub_mod(sub_mod(mul_mod(r, r, p), hhh, p), mul_mod(2, v, p), p);
            let y3 = sub_mod(mul_mod(r, sub_mod(v, x3, p), p), mul_mod(py, hhh, p), p);
            let z3 = mul_mod(pz, h, p);
            result = (x3, y3, z3);
        }
    }
    result
}

#[cube]
fn ec_scalar_mul(scalar: u64, gx: u64, gy: u64, a: u64, p: u64) -> (u64, u64, u64) {
    let mut result = infinity();
    for i in 0..32u32 {
        result = ec_double(result, a, p);
        let bit = 31u32 - i;
        if ((scalar >> (bit as u64)) & 1u64) == 1u64 {
            result = ec_add_affine(result, gx, gy, a, p);
        }
    }
    result
}

#[cube]
fn ec_matches(point: (u64, u64, u64), tx: u64, ty: u64, p: u64) -> bool {
    let (px, py, pz) = point;
    let mut matches = false;
    if pz != 0 {
        let z2 = mul_mod(pz, pz, p);
        if px == mul_mod(tx, z2, p) {
            let z3 = mul_mod(z2, pz, p);
            matches = py == mul_mod(ty, z3, p);
        }
    }
    matches
}

#[cube(launch_unchecked)]
fn rsa_scan(n: u64, total: u64, mode: u32, found: &[Atomic<u32>], factor: &[Atomic<u64>]) {
    let index = ABSOLUTE_POS as u64;
    if index >= total {
        terminate!();
    }
    if mode == 1 && found[0].load() != 0 {
        terminate!();
    }

    let k = 1 + index / 2;
    let mut divisor = 6 * k - 1;
    if index & 1 == 1 {
        divisor = 6 * k + 1;
    }

    if n.is_multiple_of(divisor) {
        factor[0].store(divisor);
        found[0].store(1);
    }
}

#[cube(launch_unchecked)]
fn ecc_scan(
    p: u64,
    a: u64,
    gx: u64,
    gy: u64,
    q: u64,
    target_x: u64,
    target_y: u64,
    mode: u32,
    found: &[Atomic<u32>],
    scalar: &[Atomic<u64>],
) {
    let index = ABSOLUTE_POS as u64;
    if index + 1 >= q {
        terminate!();
    }
    if mode == 1 && found[0].load() != 0 {
        terminate!();
    }

    let k = index + 1;
    let point = ec_scalar_mul(k, gx, gy, a, p);
    if ec_matches(point, target_x, target_y, p) {
        scalar[0].store(k);
        found[0].store(1);
    }
}

fn parse_u64(value: Option<&String>, name: &str) -> AppResult<u64> {
    let Some(value) = value else {
        return Err(
            IoError::new(ErrorKind::InvalidInput, format!("missing value for {name}")).into(),
        );
    };
    value.parse().map_err(|_| {
        IoError::new(
            ErrorKind::InvalidInput,
            format!("invalid integer for {name}"),
        )
        .into()
    })
}

fn parse_mode(value: Option<&String>) -> AppResult<String> {
    let mode = value.ok_or_else(|| IoError::new(ErrorKind::InvalidInput, "missing --mode"))?;
    if mode == "full" || mode == "recovery" {
        Ok(mode.clone())
    } else {
        Err(IoError::new(ErrorKind::InvalidInput, format!("invalid --mode: {mode}")).into())
    }
}

#[expect(clippy::too_many_arguments, reason = "the CSV row is kept flat")]
fn print_row(
    label: &str,
    mode: &str,
    block: u32,
    bits: u64,
    key_space: u64,
    candidates: u64,
    wall_ns: u128,
    recovered: bool,
    verified: bool,
    value: u64,
) {
    println!(
        "{label},{mode},{block},{bits},{key_space},{candidates},{wall_ns},{wall_ns},{},{},{value}",
        recovered as u8, verified as u8
    );
}

fn launch_rsa(
    client: &Client,
    n: u64,
    total: u64,
    mode_flag: u32,
    block: u32,
    grid: u32,
) -> AppResult<(bool, u64)> {
    let found_handle = client.create_from_slice(u32::as_bytes(&[0u32]));
    let factor_handle = client.create_from_slice(u64::as_bytes(&[0u64]));
    unsafe {
        rsa_scan::launch_unchecked(
            client,
            CubeCount::Static(grid, 1, 1),
            CubeDim::new_1d(block),
            n,
            total,
            mode_flag,
            BufferArg::from_raw_parts(found_handle.clone(), 1),
            BufferArg::from_raw_parts(factor_handle.clone(), 1),
        )
    };
    let found_bytes = client.read_one(found_handle)?;
    let factor_bytes = client.read_one(factor_handle)?;
    Ok((
        u32::from_bytes(&found_bytes)[0] != 0,
        u64::from_bytes(&factor_bytes)[0],
    ))
}

#[expect(
    clippy::too_many_arguments,
    reason = "launch arguments mirror the kernel signature"
)]
fn launch_ecc(
    client: &Client,
    p: u64,
    a: u64,
    gx: u64,
    gy: u64,
    q: u64,
    target_x: u64,
    target_y: u64,
    mode_flag: u32,
    block: u32,
    grid: u32,
) -> AppResult<(bool, u64)> {
    let found_handle = client.create_from_slice(u32::as_bytes(&[0u32]));
    let scalar_handle = client.create_from_slice(u64::as_bytes(&[0u64]));
    unsafe {
        ecc_scan::launch_unchecked(
            client,
            CubeCount::Static(grid, 1, 1),
            CubeDim::new_1d(block),
            p,
            a,
            gx,
            gy,
            q,
            target_x,
            target_y,
            mode_flag,
            BufferArg::from_raw_parts(found_handle.clone(), 1),
            BufferArg::from_raw_parts(scalar_handle.clone(), 1),
        )
    };
    let found_bytes = client.read_one(found_handle)?;
    let scalar_bytes = client.read_one(scalar_handle)?;
    Ok((
        u32::from_bytes(&found_bytes)[0] != 0,
        u64::from_bytes(&scalar_bytes)[0],
    ))
}

fn run_rsa(args: &[String]) -> AppResult<()> {
    let mut n = 0u64;
    let mut bits = 0u64;
    let mut block = 256u32;
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
            "--block" => {
                block = parse_u64(args.get(index + 1), "--block")? as u32;
                index += 2;
            }
            "--mode" => {
                mode = parse_mode(args.get(index + 1))?;
                index += 2;
            }
            "--label" => {
                label = args
                    .get(index + 1)
                    .ok_or_else(|| IoError::new(ErrorKind::InvalidInput, "missing --label"))?
                    .clone();
                index += 2;
            }
            other => {
                return Err(IoError::new(
                    ErrorKind::InvalidInput,
                    format!("unknown argument: {other}"),
                )
                .into());
            }
        }
    }

    if n < 9 || bits == 0 || !(1..=1024).contains(&block) {
        return Err(IoError::new(ErrorKind::InvalidInput, "invalid RSA arguments").into());
    }

    let device = select_device()?;
    let client = device.client();
    let kmax = n.isqrt() / 6 + 2;
    let total = kmax * 2;
    let grid = total.div_ceil(block as u64);
    if grid > u32::MAX as u64 {
        return Err(IoError::new(ErrorKind::InvalidInput, "grid dimension overflow").into());
    }

    let mode_flag = if mode == "recovery" { 1u32 } else { 0u32 };
    let grid = grid as u32;

    let _ = launch_rsa(&client, n, total, mode_flag, block, grid)?;
    let start = Instant::now();
    let (found, factor) = launch_rsa(&client, n, total, mode_flag, block, grid)?;
    let wall_ns = start.elapsed().as_nanos();
    let verified = found
        && factor > 1
        && n.is_multiple_of(factor)
        && is_prime_u64(factor)
        && is_prime_u64(n / factor);

    print_row(
        &label, &mode, block, bits, total, total, wall_ns, found, verified, factor,
    );
    Ok(())
}

fn run_ecc(args: &[String]) -> AppResult<()> {
    let mut p = 0u64;
    let mut a = 0u64;
    let mut b = 0u64;
    let mut gx = 0u64;
    let mut gy = 0u64;
    let mut q = 0u64;
    let mut d = 0u64;
    let mut bits = 0u64;
    let mut block = 256u32;
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
            "--block" => {
                block = parse_u64(args.get(index + 1), "--block")? as u32;
                index += 2;
            }
            "--mode" => {
                mode = parse_mode(args.get(index + 1))?;
                index += 2;
            }
            "--label" => {
                label = args
                    .get(index + 1)
                    .ok_or_else(|| IoError::new(ErrorKind::InvalidInput, "missing --label"))?
                    .clone();
                index += 2;
            }
            other => {
                return Err(IoError::new(
                    ErrorKind::InvalidInput,
                    format!("unknown argument: {other}"),
                )
                .into());
            }
        }
    }

    if p < 5 || q < 2 || d == 0 || d >= q || bits == 0 || !(1..=1024).contains(&block) {
        return Err(IoError::new(ErrorKind::InvalidInput, "invalid ECC arguments").into());
    }

    let curve = Curve { p, a, b };
    let base = AffinePoint { x: gx, y: gy };
    if !ec::is_on_curve(&curve, base) || !ec::scalar_mul(&curve, base, q).is_infinity() {
        return Err(IoError::new(ErrorKind::InvalidInput, "invalid toy curve").into());
    }
    let public_key = ec::to_affine(&curve, ec::scalar_mul(&curve, base, d))
        .ok_or_else(|| IoError::new(ErrorKind::InvalidInput, "point at infinity"))?;

    let device = select_device()?;
    let client = device.client();
    let total = q - 1;
    let grid = total.div_ceil(block as u64);
    if grid > u32::MAX as u64 {
        return Err(IoError::new(ErrorKind::InvalidInput, "grid dimension overflow").into());
    }

    let mode_flag = if mode == "recovery" { 1u32 } else { 0u32 };
    let grid = grid as u32;

    let _ = launch_ecc(
        &client,
        p,
        a,
        gx,
        gy,
        q,
        public_key.x,
        public_key.y,
        mode_flag,
        block,
        grid,
    )?;
    let start = Instant::now();
    let (found, scalar) = launch_ecc(
        &client,
        p,
        a,
        gx,
        gy,
        q,
        public_key.x,
        public_key.y,
        mode_flag,
        block,
        grid,
    )?;
    let wall_ns = start.elapsed().as_nanos();
    let verified = found
        && scalar == d
        && scalar > 0
        && scalar < q
        && ec::to_affine(&curve, ec::scalar_mul(&curve, base, scalar)) == Some(public_key);

    print_row(
        &label, &mode, block, bits, total, total, wall_ns, found, verified, scalar,
    );
    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let Some(command) = args.first() else {
        eprintln!("usage: gpu-cubecl rsa ... | gpu-cubecl ecc ...");
        return ExitCode::from(2);
    };

    let result = match command.as_str() {
        "rsa" => run_rsa(&args[1..]),
        "ecc" => run_ecc(&args[1..]),
        other => {
            Err(IoError::new(ErrorKind::InvalidInput, format!("unknown command: {other}")).into())
        }
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::from(2)
        }
    }
}
