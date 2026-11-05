#!/usr/bin/env python3
"""Run CPU toy key-recovery benchmarks and write raw/summary data."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

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
    run_command,
    summarize,
    write_csv,
)


def parse_list(value: str | None) -> list[int] | None:
    if not value:
        return None
    return [int(item) for item in value.split(",") if item]


def run_job(arguments: list[str]) -> dict[str, int]:
    output = run_command([str(binary("toy-bench")), *arguments], cwd=BENCH_DIR)
    lines = [line for line in output.strip().splitlines() if line]
    if not lines:
        raise RuntimeError("benchmark binary returned no output")
    fields = lines[-1].split(",")
    if len(fields) != 10:
        raise RuntimeError(f"unexpected benchmark row: {lines[-1]}")
    return {
        "key_space": int(fields[4]),
        "candidates": int(fields[5]),
        "wall_ns": int(fields[6]),
        "recovered": int(fields[7]),
        "verified": int(fields[8]),
        "value": int(fields[9]),
    }


def make_row(
    case: str,
    mode: str,
    bits: int,
    threads: int,
    run_index: int,
    vector_index: int,
    result: dict[str, int],
) -> dict[str, object]:
    wall_ns = result["wall_ns"]
    rate = result["candidates"] / (wall_ns / 1_000_000_000) / 1_000_000 if wall_ns else 0.0
    return {
        "case": case,
        "mode": mode,
        "bits": bits,
        "threads": threads,
        "run": run_index,
        "vector": vector_index,
        "key_space": result["key_space"],
        "candidates": result["candidates"],
        "wall_ns": wall_ns,
        "rate_mcand_s": rate,
        "recovered": result["recovered"],
        "verified": result["verified"],
        "value": result["value"],
    }


def execute_configuration(
    case: str,
    mode: str,
    bits: int,
    threads: int,
    arguments_for: callable,
    vectors: list[dict],
    runs: int,
    warmup: int,
) -> list[dict]:
    rows: list[dict] = []
    for _ in range(warmup):
        result = run_job(arguments_for(vectors[0], mode, threads))
        if result["verified"] != 1:
            raise RuntimeError(f"warm-up failed verification for {case}")

    for index in range(runs):
        vector_index = index % len(vectors) if mode == "recovery" else 0
        vector = vectors[vector_index]
        result = run_job(arguments_for(vector, mode, threads))
        rows.append(make_row(case, mode, bits, threads, index, vector_index, result))

    return rows


def summarize_rows(rows: list[dict]) -> list[dict]:
    groups: dict[tuple, list[dict]] = {}
    for row in rows:
        key = (row["case"], row["mode"], int(row["bits"]), int(row["threads"]))
        groups.setdefault(key, []).append(row)

    summary = []
    for (case, mode, bits, threads), group in sorted(groups.items()):
        times = [int(row["wall_ns"]) / 1_000_000 for row in group]
        rates = [float(row["rate_mcand_s"]) for row in group]
        stats = summarize(times)
        summary.append(
            {
                "case": case,
                "mode": mode,
                "bits": bits,
                "threads": threads,
                "runs": len(group),
                "median_ms": stats["median"],
                "iqr_ms": stats["iqr"],
                "min_ms": stats["min"],
                "max_ms": stats["max"],
                "mean_ms": stats["mean"],
                "median_rate_mcand_s": sorted(rates)[len(rates) // 2],
                "all_recovered": int(all(int(row["recovered"]) == 1 for row in group)),
                "all_verified": int(all(int(row["verified"]) == 1 for row in group)),
            }
        )
    return summary


def render_markdown(summary: list[dict]) -> str:
    sections: list[str] = []

    for case, title in (("rsa", "RSA toy"), ("ecc", "ECDLP toy")):
        for mode, mode_title in (("full", "full scan"), ("recovery", "recovery")):
            rows = [
                row
                for row in summary
                if row["case"] == case and row["mode"] == mode
            ]
            if not rows:
                continue
            if mode == "full":
                table_rows = [
                    [
                        row["bits"],
                        row["threads"],
                        row["runs"],
                        f"{row['median_ms']:.3f}",
                        f"{row['iqr_ms']:.3f}",
                        f"{row['median_rate_mcand_s']:.3f}",
                        row["all_verified"],
                    ]
                    for row in rows
                ]
                sections.append(
                    f"### {title} - {mode_title}\n\n"
                    + markdown_table(
                        [
                            "Bits",
                            "Threads",
                            "Runs",
                            "Median ms",
                            "IQR ms",
                            "M candidates/s",
                            "Verified",
                        ],
                        table_rows,
                    )
                )
            else:
                table_rows = [
                    [
                        row["bits"],
                        row["threads"],
                        row["runs"],
                        f"{row['median_ms']:.3f}",
                        f"{row['iqr_ms']:.3f}",
                        f"{row['min_ms']:.3f}",
                        f"{row['max_ms']:.3f}",
                        row["all_verified"],
                    ]
                    for row in rows
                ]
                sections.append(
                    f"### {title} - {mode_title}\n\n"
                    + markdown_table(
                        [
                            "Bits",
                            "Threads",
                            "Runs",
                            "Median ms",
                            "IQR ms",
                            "Min ms",
                            "Max ms",
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
    parser.add_argument("--threads", help="comma-separated CPU thread counts")
    parser.add_argument("--only", choices=["rsa", "ecc"])
    parser.add_argument("--bits", help="comma-separated toy sizes")
    parser.add_argument("--no-build", action="store_true")
    args = parser.parse_args()

    ensure_dirs()
    config = load_json(CONFIG_DIR / "bench.json")
    vectors = load_json(DATA_DIR / "vectors.json")
    runs = args.runs if args.runs is not None else int(config["runs"])
    warmup = args.warmup if args.warmup is not None else int(config["warmup"])
    threads_list = parse_list(args.threads) or [int(item) for item in config["threads"]]
    bits_filter = set(parse_list(args.bits)) if args.bits else None
    only = args.only

    if not args.no_build:
        run_command(["cargo", "build", "--release", "--offline"], cwd=BENCH_DIR)

    rows: list[dict] = []

    if only in (None, "rsa"):
        for entry in vectors["rsa"]:
            bits = int(entry["bits"])
            if bits_filter and bits not in bits_filter:
                continue
            for threads in threads_list:
                def arguments_for(vector: dict, mode: str, thread_count: int = threads, size: int = bits) -> list[str]:
                    return [
                        "rsa",
                        "--n",
                        str(vector["n"]),
                        "--bits",
                        str(size),
                        "--mode",
                        mode,
                        "--threads",
                        str(thread_count),
                        "--label",
                        f"rsa_{size}",
                    ]

                for mode in ("full", "recovery"):
                    batch = execute_configuration(
                        "rsa",
                        mode,
                        bits,
                        threads,
                        arguments_for,
                        entry["vectors"],
                        runs,
                        warmup,
                    )
                    rows.extend(batch)
                    print(f"RSA {bits} bits, {threads} threads, {mode}: {len(batch)} runs")

    if only in (None, "ecc"):
        for entry in vectors["ecc"]:
            bits = int(entry["bits"])
            if bits_filter and bits not in bits_filter:
                continue
            curve = entry["curve"]
            for threads in threads_list:
                def arguments_for(vector: dict, mode: str, thread_count: int = threads, curve_data: dict = curve, size: int = bits) -> list[str]:
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
                        "--threads",
                        str(thread_count),
                        "--label",
                        f"ecc_{size}",
                    ]

                for mode in ("full", "recovery"):
                    batch = execute_configuration(
                        "ecc",
                        mode,
                        bits,
                        threads,
                        arguments_for,
                        entry["vectors"],
                        runs,
                        warmup,
                    )
                    rows.extend(batch)
                    print(f"ECC {bits} bits, {threads} threads, {mode}: {len(batch)} runs")

    raw_path = RAW_DIR / "cpu_raw.csv"
    write_csv(raw_path, rows)
    summary = summarize_rows(rows)
    summary_path = SUMMARY_DIR / "cpu_summary.csv"
    write_csv(summary_path, summary)
    markdown_path = SUMMARY_DIR / "cpu_summary.md"
    markdown_path.write_text("# CPU benchmark summary\n\n" + render_markdown(summary), encoding="utf-8")

    print(f"wrote {raw_path}")
    print(f"wrote {summary_path}")
    print(f"wrote {markdown_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
