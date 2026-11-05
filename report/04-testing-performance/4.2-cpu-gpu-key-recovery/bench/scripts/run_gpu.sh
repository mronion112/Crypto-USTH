#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SECTION_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$SECTION_DIR"

# Usage: bash run_gpu.sh [other run_gpu.py options]

mkdir -p data/raw

if ! command -v nvidia-smi >/dev/null 2>&1; then
    echo "nvidia-smi not found; run this script on a CUDA-capable host" >&2
    exit 2
fi

if ! nvidia-smi >/dev/null 2>&1; then
    echo "The CUDA driver is not accessible from this environment" >&2
    echo "Run this script on a host with a working NVIDIA driver." >&2
    exit 2
fi

nvidia-smi --query-gpu=name,memory.total,driver_version,compute_cap --format=csv,noheader > data/raw/gpu_info.txt
nvcc --version >> data/raw/gpu_info.txt

python3 bench/scripts/run_gpu.py "$@"
python3 bench/scripts/plot_results.py
