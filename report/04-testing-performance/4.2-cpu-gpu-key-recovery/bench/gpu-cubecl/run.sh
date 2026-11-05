#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

cargo build --release
cargo run --release -- rsa --n 2581738777 --bits 32 --mode recovery --block 256 --label rsa_32
