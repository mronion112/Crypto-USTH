#!/usr/bin/env python3
"""Generate toy curve parameters with the Rust generator."""

from __future__ import annotations

import argparse
import json
import os
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from common import BENCH_DIR, CONFIG_DIR, binary, dump_json, ensure_dirs, load_json, run_command


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--force", action="store_true", help="regenerate curves.json")
    args = parser.parse_args()

    ensure_dirs()
    config = load_json(CONFIG_DIR / "bench.json")
    output_path = CONFIG_DIR / "curves.json"

    if output_path.exists() and not args.force:
        print(f"reusing {output_path.relative_to(BENCH_DIR)}")
        return 0

    run_command(["cargo", "build", "--release", "--offline"], cwd=BENCH_DIR)
    generator = binary("gen-curve")
    threads = min(12, os.cpu_count() or 1)
    curves = []

    for bits in config["ecc_bits"]:
        seed = int(config["seed"]) + int(bits)
        stdout = run_command(
            [generator, "--bits", str(bits), "--seed", str(seed), "--threads", str(threads)]
        )
        curve = json.loads(stdout.strip())
        curves.append(curve)
        print(f"generated {bits}-bit curve: p={curve['p']} q={curve['q']}")

    dump_json(output_path, curves)
    print(f"wrote {output_path.relative_to(BENCH_DIR)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
