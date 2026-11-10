# 4.1 Test Cases and Experimental Setup

## 4.1.1 Objectives and Scope

The experimental procedure covers two distinct objectives:

1. **Correctness and implementation risk:** verify representative configurations of the signature families analyzed in Section 2 and demonstrate the consequence of ECDSA nonce reuse.
2. **Toy-parameter key recovery:** measure search time and throughput for reduced RSA and elliptic-curve discrete-logarithm (ECDLP) instances.

All keys, messages, and curves were generated for testing. No real banking key, transaction, service, RSA-2048/RSA-4096 modulus, or standard 256-bit elliptic curve was subjected to brute-force recovery.

## 4.1.2 Experimental Environment

Table 4.1 records the environment used to collect the CPU and GPU measurements.

**Table 4.1. Hardware and software configuration**

| Component | Value |
|---|---|
| CPU | AMD Ryzen 5 7535HS, 6 cores / 12 threads |
| RAM | 14 GiB |
| Operating system | CachyOS, Linux kernel 7.3.1-1-cachyos x86_64 |
| Rust | rustc 1.99.1, cargo 1.99.1 |
| Python | 3.14.9 |
| `cryptography` | 50.0.2 |
| OpenSSL | 3.6.5 |
| CUDA toolkit | 13.4 (V13.4.92) |
| CubeCL | 0.11.3 |
| GPU | NVIDIA GeForce RTX 4050 Laptop GPU (AD107M), 6141 MiB; driver 615.78.08; compute capability 8.9 |

The GPU experiments ran on the same laptop. Driver, memory, and compute-capability data were captured with `nvidia-smi`. CPU and GPU results are reported separately in Section 4.2, and the GPU tables contain measured rather than estimated values. The captured CPU environment is retained in `4.1-test-cases-and-setup/data/environment.txt`.

## 4.1.3 Test-Case Design

The test cases are divided into four groups.

| Group | Purpose | Cases |
|---|---|---|
| Correctness | Generate keys, sign, and verify for each selected signature family | C1-C8 |
| Implementation risk | Recover an ECDSA private key after nonce reuse, without exhaustive search | I1 |
| Key recovery | Measure toy-key recovery time and throughput | K1-K3 |
| Scaling and platform | Compare one CPU thread with 12 threads and execute the GPU baseline | S1, G1 |

The complete inputs, expected outputs, and observed results are recorded in `test-cases.md`. Correctness cases exercise supported library configurations; they are not performance measurements and do not rank the algorithms. Key-recovery cases use only toy parameters.

C1-C2 follow the RSA-PSS configuration in FIPS 186-5 [1]; C4 uses Ed25519 as specified by RFC 8032 [2]; C5 uses SM2 according to GB/T 32918.2-2016 [3]; and C6-C8 use ML-DSA as standardized by FIPS 204 [4]. SM2 was exercised through the OpenSSL CLI [5], while the other families used the Python `cryptography` package [6].

## 4.1.4 Toy Parameters and Data Generation

### Toy RSA

Each sample is a modulus $n=pq$ of 32, 40, 48, or 56 bits. The random primes $p$ and $q$ have approximately half the modulus length. Ten moduli at each size were generated from the fixed seed `20261105`. A full scan used the first sample; the ten recovery measurements used the ten samples in sequence.

The recovery program searches for a non-trivial factor by testing candidates of the form $6k-1$ and $6k+1$. A result is accepted only when $pq=n$ and both factors pass deterministic Miller-Rabin validation.

### Toy ECDLP

Three short-Weierstrass curves $y^2\equiv x^3+ax+b\pmod p$ were generated with 18-, 22-, and 26-bit field primes. The generator counts points with the Legendre symbol, factors the group order, and selects a prime-order subgroup of order $q$.

**Table 4.2. Toy-curve parameters**

| $p$ bits | $p$ | $q$ | Group order | Cofactor |
|---|---|---|---|---|
| 18 | 163987 | 81737 | 163474 | 2 |
| 22 | 2610967 | 2609701 | 2609701 | 1 |
| 26 | 54523747 | 54518393 | 54518393 | 1 |

For each curve, ten private scalars $d\in[1,q-1]$ were generated and the public point was computed as $Q=dG$. The full scan used the first key; the ten recovery trials used the ten keys in sequence. These ordinary but very small curves measure exhaustive-search behavior only. They do not represent a standardized curve or the security of ECDSA, Ed25519, or SM2.

### Nonce-Reuse Demonstration and Correctness Smoke Tests

Case I1 uses standard P-256, one test key, and two deliberately constructed signatures sharing the same nonce. The key is recovered algebraically from the two signature equations and validated by checking $dG=Q$ and independently verifying both signatures through `cryptography`. This is an implementation-failure demonstration, not an ECDLP search. FIPS 186-5 and RFC 6979 document the nonce requirements and deterministic option relevant to this case [1], [7].

Cases C1-C8 create ephemeral keys in memory, sign one fixed message, and immediately verify it. No private key is written to the result dataset.

## 4.1.5 Measurement Procedure

### Timing and Repetitions

- CPU wall time is measured with `std::time::Instant` around the complete search. The release profile uses `opt-level = 3`, thin LTO, and one code-generation unit.
- GPU wall time is also measured with `std::time::Instant`; the CubeCL host path does not expose kernel-only time.
- Each configuration has one unrecorded warm-up followed by ten measured trials.
- CPU and GPU trials run on AC power to reduce power-policy variation.
- The report gives median, interquartile range (IQR), minimum, maximum, and mean without deleting outliers.
- A result is accepted only when `verified` equals 1. A failed warm-up or inaccessible GPU stops the relevant script; GPU return code 2 preserves an existing GPU dataset.
- CPU trials use one and 12 threads. GPU trials use blocks of 256 threads.

### Measurement Modes

- **Full scan:** enumerate the entire toy search space. This measures candidate throughput with minimal dependence on target position.
- **Recovery:** stop after finding a randomly positioned target. This measures end-to-end recovery time but has greater dispersion because target position varies.

For recovery trials, throughput is descriptive rather than a platform-independent rate because early termination and thread scheduling affect the number of tested candidates.

### CPU/GPU Equivalence Boundary

The CPU and GPU programs cover the same candidate spaces and apply the same result checks. Their implementations differ: the CPU advances through blocks and updates ECDLP candidates with point addition, whereas the GPU assigns one candidate or scalar multiplication to each thread. The comparison is therefore valid only for the same toy size and measurement mode; it is an end-to-end implementation comparison, not an isolated comparison of processor hardware.

## 4.1.6 Reproducibility

The complete CPU workflow is executed with:

```bash
cd report/04-testing-performance/4.2-cpu-gpu-key-recovery
bash bench/scripts/run_all.sh
```

Individual commands and the GPU procedure are documented in the Section 4.2 README. Raw data are stored in `data/raw/`, summaries in `data/summary/`, and figures in `figures/`. Seeds, sizes, and repetition counts are in `bench/config/bench.json`; curve parameters are in `bench/config/curves.json`.

The validation commands are:

```bash
cargo test --release --offline --lib --bins --tests --manifest-path bench/Cargo.toml
python3 bench/scripts/smoke_sign_verify.py
python3 bench/scripts/nonce_reuse.py
```

## 4.1.7 Limitations

- Toy sizes are not representative of standard keys, and their timings must not be extrapolated to RSA-2048, RSA-4096, or 256-bit ECC.
- The generated curves are small and non-standard; they measure only the selected search procedures.
- The laptop CPU is affected by temperature, scheduling, and power management. Warm-up, ten trials, medians, and IQRs reduce but do not remove that variation.
- Correctness smoke tests are pass/fail integration checks rather than signing or verification benchmarks.
- GPU block size, block count, and candidate mapping were not exhaustively tuned; the reported GPU results use blocks of 256.
- The tests do not establish certification, production readiness, or cross-vendor interoperability.
