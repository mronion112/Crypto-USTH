#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SECTION_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$SECTION_DIR"

# Usage: bash run_rho.sh [other run_rho.py options]

python3 bench/scripts/run_rho.py "$@"
python3 bench/scripts/plot_results.py
