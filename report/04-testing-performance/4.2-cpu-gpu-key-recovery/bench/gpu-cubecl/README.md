# CubeCL CUDA benchmark

CubeCL implementation of the toy RSA and ECDLP key-recovery searches. The
kernels are Rust `#[cube]` functions; the host uses CubeCL's CUDA runtime for
device discovery, buffers, launch, and readback.

## Requirements

- Rust 1.95 or newer.
- NVIDIA driver and CUDA toolkit visible to the process.
- Network access for the first dependency fetch.

## Run one case

```bash
cd report/04-testing-performance/4.2-cpu-gpu-key-recovery/bench/gpu-cubecl
bash run.sh
```

`run.sh` builds the crate and runs one 32-bit RSA recovery case. Expected
output ends with a recovered factor of `41203`.

For a local CPU-backend check without a GPU:

```bash
CUBECL_CPU=1 bash run.sh
```

## Run the full GPU benchmark

```bash
cd report/04-testing-performance/4.2-cpu-gpu-key-recovery
bash bench/scripts/run_gpu.sh
```

Outputs:

- `data/raw/gpu_cubecl_raw.csv`
- `data/summary/gpu_cubecl_summary.csv`
- `data/summary/gpu_cubecl_summary.md`

The CubeCL host reports wall time in both the wall and kernel fields because it
does not expose kernel-only timing in this path. The binary performs one
unrecorded warm-up launch before timing the measured launch, so the CubeCL JIT
compilation is not charged to the result.

The CubeCL implementation is the reported GPU path.
