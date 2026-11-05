#!/usr/bin/env python3
"""Run the CUDA toy key-recovery benchmark on a CUDA-capable host."""

from __future__ import annotations

import argparse
import os
import shutil
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from common import (
    BENCH_DIR,
    CONFIG_DIR,
    DATA_DIR,
    RAW_DIR,
    SUMMARY_DIR,
    ensure_dirs,
    load_json,
    markdown_table,
    run_command,
    summarize,
    write_csv,
)

GPU_DIR = BENCH_DIR / "gpu-cubecl"
GPU_BINARY = GPU_DIR / "target" / "release" / "gpu-cubecl"


def build_gpu() -> None:
    manifest = GPU_DIR / "Cargo.toml"
    offline = ["cargo", "build", "--release", "--offline", "--manifest-path", str(manifest)]
    online = ["cargo", "build", "--release", "--manifest-path", str(manifest)]
    try:
        run_command(offline, cwd=BENCH_DIR)
    except Exception:
        run_command(online, cwd=BENCH_DIR)


def parse_list(value: str | None) -> list[int] | None:
    if not value:
        return None
    return [int(item) for item in value.split(",") if item]


def run_job(arguments: list[str]) -> dict[str, int]:
    output = run_command([str(GPU_BINARY), *arguments], cwd=BENCH_DIR)
    lines = [line for line in output.strip().splitlines() if line]
    if not lines:
        raise RuntimeError("GPU benchmark returned no output")
    fields = lines[-1].split(",")
    if len(fields) != 11:
        raise RuntimeError(f"unexpected GPU row: {lines[-1]}")
    return {
        "key_space": int(fields[4]),
        "candidates": int(fields[5]),
        "wall_ns": int(fields[6]),
        "kernel_ns": int(fields[7]),
        "recovered": int(fields[8]),
        "verified": int(fields[9]),
        "value": int(fields[10]),
    }


def execute_configuration(
    case: str,
    mode: str,
    bits: int,
    block: int,
    arguments_for,
    vectors: list[dict],
    runs: int,
    warmup: int,
) -> list[dict]:
    rows: list[dict] = []
    for _ in range(warmup):
        result = run_job(arguments_for(vectors[0], mode, block))
        if result["verified"] != 1:
            raise RuntimeError(f"GPU warm-up failed verification for {case}")

    for index in range(runs):
        vector_index = index % len(vectors) if mode == "recovery" else 0
        vector = vectors[vector_index]
        result = run_job(arguments_for(vector, mode, block))
        wall_ns = result["wall_ns"]
        rate = result["candidates"] / (wall_ns / 1_000_000_000) / 1_000_000 if wall_ns else 0.0
        rows.append(
            {
                "case": case,
                "mode": mode,
                "bits": bits,
                "block": block,
                "run": index,
                "vector": vector_index,
                "key_space": result["key_space"],
                "candidates": result["candidates"],
                "wall_ns": wall_ns,
                "kernel_ns": result["kernel_ns"],
                "rate_mcand_s": rate,
                "recovered": result["recovered"],
                "verified": result["verified"],
                "value": result["value"],
            }
        )
    return rows


def summarize_rows(rows: list[dict]) -> list[dict]:
    groups: dict[tuple, list[dict]] = {}
    for row in rows:
        key = (row["case"], row["mode"], int(row["bits"]), int(row["block"]))
        groups.setdefault(key, []).append(row)

    summary = []
    for (case, mode, bits, block), group in sorted(groups.items()):
        wall = [int(row["wall_ns"]) / 1_000_000 for row in group]
        kernel = [int(row["kernel_ns"]) / 1_000_000 for row in group]
        rates = [float(row["rate_mcand_s"]) for row in group]
        wall_stats = summarize(wall)
        kernel_stats = summarize(kernel)
        summary.append(
            {
                "case": case,
                "mode": mode,
                "bits": bits,
                "block": block,
                "runs": len(group),
                "median_ms": wall_stats["median"],
                "iqr_ms": wall_stats["iqr"],
                "min_ms": wall_stats["min"],
                "max_ms": wall_stats["max"],
                "median_kernel_ms": kernel_stats["median"],
                "median_rate_mcand_s": sorted(rates)[len(rates) // 2],
                "all_recovered": int(all(int(row["recovered"]) == 1 for row in group)),
                "all_verified": int(all(int(row["verified"]) == 1 for row in group)),
            }
        )
    return summary


def render_markdown(summary: list[dict]) -> str:
    sections = []
    for case, title in (("rsa", "RSA toy"), ("ecc", "ECDLP toy")):
        for mode, mode_title in (("full", "full scan"), ("recovery", "recovery")):
            rows = [row for row in summary if row["case"] == case and row["mode"] == mode]
            if not rows:
                continue
            table_rows = [
                [
                    row["bits"],
                    row["block"],
                    row["runs"],
                    f"{row['median_ms']:.3f}",
                    f"{row['median_kernel_ms']:.3f}",
                    f"{row['iqr_ms']:.3f}",
                    f"{row['median_rate_mcand_s']:.3f}" if mode == "full" else "-",
                    row["all_verified"],
                ]
                for row in rows
            ]
            sections.append(
                f"### {title} - {mode_title}\n\n"
                + markdown_table(
                    [
                        "Bits",
                        "Block",
                        "Runs",
                        "Median ms",
                        "Kernel ms",
                        "IQR ms",
                        "M candidates/s",
                        "Verified",
                    ],
                    table_rows,
                )
            )
    return "\n\n".join(sections) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--runs", type=int)
    parser.add_argument("--warmup", type=int)
    parser.add_argument("--block", type=int)
    parser.add_argument("--only", choices=["rsa", "ecc"])
    parser.add_argument("--bits", help="comma-separated toy sizes")
    parser.add_argument("--no-build", action="store_true")
    args = parser.parse_args()

    cpu_mode = os.environ.get("CUBECL_CPU") is not None
    if not cpu_mode:
        if shutil.which("nvidia-smi") is None:
            print("nvidia-smi not found; run this script on a CUDA-capable host")
            return 2
        try:
            run_command(["nvidia-smi", "--query-gpu=name", "--format=csv,noheader"])
        except Exception as error:
            print(f"GPU is not accessible: {error}")
            return 2

    ensure_dirs()
    config = load_json(CONFIG_DIR / "bench.json")
    vectors = load_json(DATA_DIR / "vectors.json")
    runs = args.runs if args.runs is not None else int(config["runs"])
    warmup = args.warmup if args.warmup is not None else int(config["warmup"])
    block = args.block if args.block is not None else int(config["gpu_block_size"])
    bits_filter = set(parse_list(args.bits)) if args.bits else None
    only = args.only

    if not args.no_build:
        build_gpu()

    rows: list[dict] = []

    if only in (None, "rsa"):
        for entry in vectors["rsa"]:
            bits = int(entry["bits"])
            if bits_filter and bits not in bits_filter:
                continue

            def arguments_for(vector: dict, mode: str, block_size: int = block, size: int = bits) -> list[str]:
                return [
                    "rsa",
                    "--n",
                    str(vector["n"]),
                    "--bits",
                    str(size),
                    "--mode",
                    mode,
                    "--block",
                    str(block_size),
                    "--label",
                    f"rsa_{size}",
                ]

            for mode in ("full", "recovery"):
                batch = execute_configuration(
                    "rsa", mode, bits, block, arguments_for, entry["vectors"], runs, warmup
                )
                rows.extend(batch)
                print(f"GPU RSA {bits} bits, block {block}, {mode}: {len(batch)} runs")

    if only in (None, "ecc"):
        for entry in vectors["ecc"]:
            bits = int(entry["bits"])
            if bits_filter and bits not in bits_filter:
                continue
            curve = entry["curve"]

            def arguments_for(vector: dict, mode: str, block_size: int = block, curve_data: dict = curve, size: int = bits) -> list[str]:
                return [
                    "ecc",
                    "--p",
                    str(curve_data["p"]),
                    "--a",
                    str(curve_data["a"]),
                    "--b",
                    str(curve_data["b"]),
                    "--gx",
                    str(curve_data["gx"]),
                    "--gy",
                    str(curve_data["gy"]),
                    "--q",
                    str(curve_data["q"]),
                    "--d",
                    str(vector["d"]),
                    "--bits",
                    str(size),
                    "--mode",
                    mode,
                    "--block",
                    str(block_size),
                    "--label",
                    f"ecc_{size}",
                ]

            for mode in ("full", "recovery"):
                batch = execute_configuration(
                    "ecc", mode, bits, block, arguments_for, entry["vectors"], runs, warmup
                )
                rows.extend(batch)
                print(f"GPU ECC {bits} bits, block {block}, {mode}: {len(batch)} runs")

    raw_path = RAW_DIR / "gpu_cubecl_raw.csv"
    write_csv(raw_path, rows)
    summary = summarize_rows(rows)
    summary_path = SUMMARY_DIR / "gpu_cubecl_summary.csv"
    write_csv(summary_path, summary)
    markdown_path = SUMMARY_DIR / "gpu_cubecl_summary.md"
    markdown_path.write_text("# GPU benchmark summary\n\n" + render_markdown(summary), encoding="utf-8")

    print(f"wrote {raw_path}")
    print(f"wrote {summary_path}")
    print(f"wrote {markdown_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
