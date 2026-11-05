#!/usr/bin/env python3
"""Run the CPU Pollard rho comparison for the section 4.2 toy benchmark."""

from __future__ import annotations

import argparse
import statistics
import subprocess
import sys
from pathlib import Path
from typing import Any, TypedDict, cast

sys.path.insert(0, str(Path(__file__).resolve().parent))

from common import (
    BENCH_DIR,
    CONFIG_DIR,
    DATA_DIR,
    RAW_DIR,
    SUMMARY_DIR,
    binary,
    ensure_dirs,
    load_json,
    markdown_table,
    read_csv,
    run_command,
    summarize,
    write_csv,
)

DEFAULT_MAX_STEPS = 10_000_000


def build_binary() -> None:
    manifest = BENCH_DIR / "Cargo.toml"
    offline = [
        "cargo",
        "build",
        "--release",
        "--offline",
        "--manifest-path",
        str(manifest),
    ]
    online = ["cargo", "build", "--release", "--manifest-path", str(manifest)]
    try:
        run_command(offline, cwd=BENCH_DIR)
    except subprocess.CalledProcessError:
        run_command(online, cwd=BENCH_DIR)


def parse_list(value: str | None) -> list[int] | None:
    if not value:
        return None
    return [int(item) for item in value.split(",") if item]


class RhoResult(TypedDict):
    case: str
    bits: int
    seed: int
    steps: int
    wall_ns: int
    recovered: int
    verified: int
    value: int
    restarts: int


def run_job(arguments: list[str]) -> RhoResult:
    output = run_command([str(binary("rho-bench")), *arguments], cwd=BENCH_DIR)
    lines = [line for line in output.strip().splitlines() if line]
    if not lines:
        raise RuntimeError("rho benchmark returned no output")
    fields = lines[-1].split(",")
    if len(fields) != 9:
        raise RuntimeError(f"unexpected rho row: {lines[-1]}")
    return {
        "case": fields[0],
        "bits": int(fields[1]),
        "seed": int(fields[2]),
        "steps": int(fields[3]),
        "wall_ns": int(fields[4]),
        "recovered": int(fields[5]),
        "verified": int(fields[6]),
        "value": int(fields[7]),
        "restarts": int(fields[8]),
    }


def execute_case(
    case: str,
    bits: int,
    vectors: list[dict],
    runs: int,
    base_seed: int,
    max_steps: int,
) -> list[dict]:
    rows: list[dict] = []
    for run in range(runs):
        vector_index = run % len(vectors)
        vector = vectors[vector_index]
        seed = base_seed + bits * 10_000 + run
        if case == "rsa":
            arguments = [
                "rsa",
                "--n",
                str(vector["n"]),
                "--bits",
                str(bits),
                "--seed",
                str(seed),
                "--max-steps",
                str(max_steps),
            ]
        else:
            curve = vectors[vector_index]["curve"]
            arguments = [
                "ecc",
                "--p",
                str(curve["p"]),
                "--a",
                str(curve["a"]),
                "--b",
                str(curve["b"]),
                "--gx",
                str(curve["gx"]),
                "--gy",
                str(curve["gy"]),
                "--q",
                str(curve["q"]),
                "--d",
                str(vector["d"]),
                "--bits",
                str(bits),
                "--seed",
                str(seed),
                "--max-steps",
                str(max_steps),
            ]
        result = run_job(arguments)
        rows.append(
            {
                "case": case,
                "bits": bits,
                "run": run,
                "vector": vector_index,
                "seed": seed,
                "steps": result["steps"],
                "wall_ns": result["wall_ns"],
                "recovered": result["recovered"],
                "verified": result["verified"],
                "value": result["value"],
                "restarts": result["restarts"],
            }
        )
    return rows


def brute_force_reference() -> dict[tuple[str, int], dict[str, float]]:
    summary_path = SUMMARY_DIR / "cpu_summary.csv"
    raw_path = RAW_DIR / "cpu_raw.csv"
    if not summary_path.exists() or not raw_path.exists():
        raise RuntimeError("missing CPU brute-force data; run run_bench.py first")

    reference: dict[tuple[str, int], dict[str, float]] = {}
    for row in read_csv(summary_path):
        if row["mode"] != "recovery" or int(row["threads"]) != 1:
            continue
        reference[(row["case"], int(row["bits"]))] = {
            "median_ms": float(row["median_ms"]),
        }

    candidates: dict[tuple[str, int], list[int]] = {}
    for row in read_csv(raw_path):
        if row["mode"] != "recovery" or int(row["threads"]) != 1:
            continue
        key = (row["case"], int(row["bits"]))
        candidates.setdefault(key, []).append(int(row["candidates"]))
    for key, values in candidates.items():
        reference[key]["median_candidates"] = float(statistics.median(values))
    return reference


def summarize_rows(rows: list[dict], reference: dict[tuple[str, int], dict[str, float]]) -> list[dict]:
    groups: dict[tuple[str, int], list[dict]] = {}
    for row in rows:
        groups.setdefault((row["case"], row["bits"]), []).append(row)

    summary = []
    for (case, bits), group in sorted(groups.items()):
        steps = [int(row["steps"]) for row in group]
        times = [int(row["wall_ns"]) / 1_000_000 for row in group]
        step_stats = summarize(steps)
        time_stats = summarize(times)
        brute = reference.get((case, bits), {})
        brute_ms = float(brute.get("median_ms", 0.0))
        brute_candidates = float(brute.get("median_candidates", 0.0))
        speedup = brute_ms / time_stats["median"] if time_stats["median"] else 0.0
        step_reduction = (
            brute_candidates / step_stats["median"] if step_stats["median"] else 0.0
        )
        summary.append(
            {
                "case": case,
                "bits": bits,
                "runs": len(group),
                "median_steps": step_stats["median"],
                "iqr_steps": step_stats["iqr"],
                "min_steps": step_stats["min"],
                "max_steps": step_stats["max"],
                "median_ms": time_stats["median"],
                "iqr_ms": time_stats["iqr"],
                "min_ms": time_stats["min"],
                "max_ms": time_stats["max"],
                "median_restarts": statistics.median(
                    int(row["restarts"]) for row in group
                ),
                "success_rate": sum(
                    int(row["recovered"]) == 1 and int(row["verified"]) == 1
                    for row in group
                )
                / len(group),
                "brute_ms": brute_ms,
                "brute_candidates": brute_candidates,
                "speedup_time": speedup,
                "steps_ratio": step_reduction,
            }
        )
    return summary


def render_markdown(summary: list[dict]) -> str:
    sections = []
    for case, title in (("rsa", "RSA toy"), ("ecc", "ECDLP toy")):
        rows = [row for row in summary if row["case"] == case]
        if not rows:
            continue
        table_rows = [
            [
                row["bits"],
                row["runs"],
                f"{row['median_steps']:.0f}",
                f"{row['median_ms']:.3f}",
                f"{row['brute_ms']:.3f}",
                f"{row['speedup_time']:.2f}",
                f"{row['steps_ratio']:.3f}",
                f"{row['success_rate']:.2f}",
            ]
            for row in rows
        ]
        sections.append(
            f"### {title}\n\n"
            + markdown_table(
                [
                    "Bits",
                    "Runs",
                    "Rho steps",
                    "Rho ms",
                    "Brute 1T ms",
                    "Time speedup",
                    "Step reduction",
                    "Success",
                ],
                table_rows,
            )
        )
    return "\n\n".join(sections) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--runs", type=int)
    parser.add_argument("--only", choices=["rsa", "ecc"])
    parser.add_argument("--bits", help="comma-separated toy sizes")
    parser.add_argument("--max-steps", type=int, default=DEFAULT_MAX_STEPS)
    parser.add_argument("--no-build", action="store_true")
    args = parser.parse_args()

    if not args.no_build:
        build_binary()

    ensure_dirs()
    config = cast(dict[str, Any], load_json(CONFIG_DIR / "bench.json"))
    vectors = cast(dict[str, list[dict[str, Any]]], load_json(DATA_DIR / "vectors.json"))
    runs = args.runs if args.runs is not None else int(config["runs"])
    base_seed = int(config["seed"])
    bits_filter = set(parse_list(args.bits) or [])
    bits_filter = bits_filter or None
    only = args.only
    rows: list[dict] = []

    if only in (None, "rsa"):
        for entry in vectors["rsa"]:
            bits = int(entry["bits"])
            if bits_filter and bits not in bits_filter:
                continue
            batch = execute_case(
                "rsa",
                bits,
                entry["vectors"],
                runs,
                base_seed,
                args.max_steps,
            )
            rows.extend(batch)
            print(f"rho RSA {bits} bits: {len(batch)} runs")

    if only in (None, "ecc"):
        for entry in vectors["ecc"]:
            bits = int(entry["bits"])
            if bits_filter and bits not in bits_filter:
                continue
            curve = entry["curve"]
            vectors_with_curve = [
                {"d": vector["d"], "curve": curve} for vector in entry["vectors"]
            ]
            batch = execute_case(
                "ecc",
                bits,
                vectors_with_curve,
                runs,
                base_seed,
                args.max_steps,
            )
            rows.extend(batch)
            print(f"rho ECDLP {bits} bits: {len(batch)} runs")

    raw_path = RAW_DIR / "rho_raw.csv"
    write_csv(raw_path, rows)
    summary = summarize_rows(rows, brute_force_reference())
    summary_path = SUMMARY_DIR / "rho_summary.csv"
    write_csv(summary_path, summary)
    markdown_path = SUMMARY_DIR / "rho_summary.md"
    markdown_path.write_text(
        "# Pollard rho summary\n\n" + render_markdown(summary),
        encoding="utf-8",
    )

    print(f"wrote {raw_path}")
    print(f"wrote {summary_path}")
    print(f"wrote {markdown_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
