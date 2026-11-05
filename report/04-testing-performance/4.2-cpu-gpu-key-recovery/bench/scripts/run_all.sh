#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SECTION_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$SECTION_DIR"

export TMPDIR="${TMPDIR:-/tmp}"
mkdir -p "$TMPDIR"

bash ../4.1-test-cases-and-setup/scripts/capture_env.sh

cargo build --release --offline --manifest-path bench/Cargo.toml
cargo test --release --offline --lib --bins --tests --manifest-path bench/Cargo.toml

python3 bench/scripts/gen_curves.py
python3 bench/scripts/gen_vectors.py
python3 bench/scripts/smoke_sign_verify.py
python3 bench/scripts/nonce_reuse.py
python3 bench/scripts/run_bench.py
python3 bench/scripts/run_rho.py
python3 bench/scripts/plot_results.py

echo "CPU benchmark and Pollard rho comparison complete."
echo "Run bench/scripts/run_gpu.sh on a CUDA-capable host to collect GPU data."
