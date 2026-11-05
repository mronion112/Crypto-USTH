#!/usr/bin/env bash
set -euo pipefail

export LC_ALL=C

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SECTION_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
OUT_DIR="$SECTION_DIR/data"
OUT_FILE="$OUT_DIR/environment.txt"
mkdir -p "$OUT_DIR"

cpu_model="$(lscpu | awk -F': +' '/Model name/ {print $2; exit}')"
cpu_cores="$(nproc)"
memory_total="$(free -h | awk '/^Mem:/ {print $2}')"
os_name="$(. /etc/os-release && printf '%s' "$PRETTY_NAME")"
kernel="$(uname -srm)"

gpu_lspci="$(lspci -nn 2>/dev/null | grep -iE 'vga|3d|display' || true)"
gpu_info_file="$(cd "$SECTION_DIR/.." && pwd)/4.2-cpu-gpu-key-recovery/data/raw/gpu_info.txt"
if command -v nvidia-smi >/dev/null 2>&1 && nvidia-smi --query-gpu=name >/dev/null 2>&1; then
    gpu_nvidia="$(nvidia-smi --query-gpu=name,memory.total,driver_version,compute_cap --format=csv,noheader 2>&1 || true)"
elif [ -s "$gpu_info_file" ]; then
    gpu_nvidia="$(head -1 "$gpu_info_file")"
else
    gpu_nvidia="not available"
fi

if command -v nvcc >/dev/null 2>&1; then
    nvcc_version="$(nvcc --version | awk '/release/ {print; exit}')"
else
    nvcc_version="nvcc not found"
fi

python_version="$(python3 --version 2>&1)"
cryptography_version="$(python3 -c 'import cryptography; print(cryptography.__version__)' 2>&1)"
openssl_version="$(openssl version 2>&1)"
rustc_version="$(rustc --version 2>&1)"
cargo_version="$(cargo --version 2>&1)"

{
    echo "timestamp_utc: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
    echo "os: $os_name"
    echo "kernel: $kernel"
    echo "cpu_model: $cpu_model"
    echo "cpu_logical_cores: $cpu_cores"
    echo "memory_total: $memory_total"
    echo "rustc: $rustc_version"
    echo "cargo: $cargo_version"
    echo "python: $python_version"
    echo "cryptography: $cryptography_version"
    echo "openssl: $openssl_version"
    echo "cuda_compiler: $nvcc_version"
    echo "gpu_lspci: $gpu_lspci"
    echo "gpu_nvidia_smi: $gpu_nvidia"
} > "$OUT_FILE"

cat "$OUT_FILE"
