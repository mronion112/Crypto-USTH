# CPU/GPU Toy Key-Recovery Benchmark

This directory contains the benchmark used in section 4.2 of the report. The CPU
implementation and orchestration scripts are Rust and Python. The reported GPU
implementation uses CubeCL with Rust `#[cube]` kernels.

## Scope

- RSA toy moduli: recover a factor by trial division over $6k - 1$ and $6k + 1$.
- ECDLP toy curves: recover a scalar $d$ from $Q = dG$ by brute force.
- CPU Pollard rho comparison: Brent rho for RSA and rho with Floyd cycle
  detection for ECDLP.
- CPU scaling: 1 and 12 threads for brute force.
- GPU: brute-force search with the same space and verification criteria as the
  CPU version. GPU rho is not benchmarked.
- Correctness smoke tests: RSA-PSS, ECDSA P-256, Ed25519, SM2, ML-DSA.
- ECDSA nonce-reuse demonstration on P-256.

The benchmark never targets RSA-2048, RSA-4096, or a standard 256-bit ECC key.
The toy results do not imply the cost of attacking standard parameters.

## Requirements

- Rust 1.99 or newer, with `cargo`.
- Python 3.14 or newer.
- Python packages: `cryptography`, `matplotlib`.
- OpenSSL 3.6 or newer for the SM2 smoke test.
- For the GPU part: an NVIDIA GPU with an accessible driver. The reported path
  uses CubeCL 0.11.3.

The CPU crate has no external Rust dependencies. The CubeCL crate depends on
`cubecl`. If the dependency is not in the local Cargo cache, the first GPU build
needs network access.

## Layout

```text
bench/
├── Cargo.toml
├── config/bench.json
├── config/curves.json
├── src/                 CPU implementation and curve generator
├── gpu-cubecl/          CubeCL GPU implementation
├── scripts/             vector generation, runs, plots, smoke tests
├── data/raw/            raw measurements
├── data/summary/        aggregated measurements
└── tests/               integration tests
```

Raw data and summary data are written below `../data/` relative to `bench/`.

## Reproduce the CPU results

From the section directory:

```bash
cd report/04-testing-performance/4.2-cpu-gpu-key-recovery
bash bench/scripts/run_all.sh
```

The script captures the environment, builds the Rust binaries, runs unit tests,
generates the toy curves and vectors, runs the smoke tests, the CPU brute-force
benchmark and the Pollard rho comparison, and writes figures.

Individual steps:

```bash
cargo build --release --offline --manifest-path bench/Cargo.toml
cargo test --release --offline --lib --bins --tests --manifest-path bench/Cargo.toml
python3 bench/scripts/gen_curves.py
python3 bench/scripts/gen_vectors.py
python3 bench/scripts/smoke_sign_verify.py
python3 bench/scripts/nonce_reuse.py
python3 bench/scripts/run_bench.py
python3 bench/scripts/run_rho.py
python3 bench/scripts/plot_results.py
```

`run_bench.py` accepts `--runs`, `--warmup`, `--threads`, `--only`, and
`--bits` for pilot runs. The default configuration is in `config/bench.json`.

## Pollard rho comparison

```bash
bash bench/scripts/run_rho.sh
```

The script builds `rho-bench`, runs 10 seeds per configuration, writes
`data/raw/rho_raw.csv`, `data/summary/rho_summary.csv`,
`data/summary/rho_summary.md`, and updates `figures/fig_rho_vs_bruteforce.png`.
It uses the existing CPU brute-force summary as the comparison baseline.

Options: `--runs`, `--only rsa|ecc`, `--bits`, `--max-steps`, `--no-build`.
The rho implementation is CPU-only. GPU rho is not benchmarked because parallel
rho needs coordinated walks and collision detection, and its expected step count
at the current toy sizes is below GPU launch and synchronization overhead.

## Tests

```bash
cargo test --release --offline --lib --bins --tests --manifest-path bench/Cargo.toml
python3 bench/scripts/smoke_sign_verify.py
python3 bench/scripts/nonce_reuse.py
```

The Rust tests cover field arithmetic, curve addition, brute-force scalar
recovery, Pollard rho factorization and ECDLP recovery, and the public API. The
Python checks cover sign/verify for the five signature families and the ECDSA
nonce-reuse case.

## Run the GPU benchmark

Run this on a host where `nvidia-smi` can access the GPU:

```bash
cd report/04-testing-performance/4.2-cpu-gpu-key-recovery
bash bench/scripts/run_gpu.sh
```

The script runs the CubeCL implementation and writes:

- `data/raw/gpu_info.txt`
- `data/raw/gpu_cubecl_raw.csv`
- `data/summary/gpu_cubecl_summary.csv`
- `data/summary/gpu_cubecl_summary.md`
- `figures/fig_gpu_cubecl_speedup.png`

If the GPU is not accessible, the script exits with status 2 and leaves the CPU
results unchanged.

For a single-case check without a GPU:

```bash
CUBECL_CPU=1 bash bench/gpu-cubecl/run.sh
```

See `bench/gpu-cubecl/README.md` for kernel and timing details.

## Measurement protocol

- CPU clock: `std::time::Instant` around the search.
- GPU clock: `std::time::Instant` around launch and readback. The CubeCL path
  reports wall time only; it does not expose kernel-only timing.
- Warm-up: one unrecorded run per configuration; the CubeCL binary also performs
  an internal warm-up launch before timing.
- Measured runs: ten.
- Reported statistics: median, interquartile range, minimum, maximum, and mean.
- Full scan: the complete toy key space is tested; this gives throughput.
- Recovery: one random target per run; this gives time-to-recovery.
- Pollard rho: ten seeds per configuration; reports median steps, median time,
  restart count, and success rate. The step count is the more stable metric at
  small sizes.

## Verification

- RSA: $p \cdot q = n$, and both factors pass deterministic Miller-Rabin.
- ECDLP: the recovered scalar satisfies $dG = Q$; the subgroup order is prime.
- Pollard rho: the recovered factor or scalar passes the same checks.
- Smoke tests: each library or OpenSSL verification must return success.

## Limitations

- The toy curves are small random ordinary curves; they are not standard curves.
- The brute-force RSA search is trial division, not GNFS.
- The brute-force ECDLP search is exhaustive, not Pollard's rho.
- The rho comparison is CPU-only; GPU rho is not benchmarked.
- The results are hardware- and implementation-specific.
- GPU wall time includes host/device transfers and kernel launch overhead; the
  CubeCL path does not expose kernel-only timing.
