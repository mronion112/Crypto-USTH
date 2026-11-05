#!/usr/bin/env python3
"""Build figures from CPU and GPU benchmark summaries."""

from __future__ import annotations

import os
import sys
import tempfile
from pathlib import Path

os.environ.setdefault("MPLCONFIGDIR", str(Path(tempfile.gettempdir()) / "section4-matplotlib"))

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

sys.path.insert(0, str(Path(__file__).resolve().parent))

from common import FIGURES_DIR, SUMMARY_DIR, ensure_dirs, read_csv


def grouped(rows: list[dict], case: str, mode: str, field: str = "median_ms") -> dict[int, dict[int, float]]:
    table: dict[int, dict[int, float]] = {}
    for row in rows:
        if row["case"] != case or row["mode"] != mode:
            continue
        bits = int(row["bits"])
        threads = int(row.get("threads", row.get("block", 0)))
        table.setdefault(bits, {})[threads] = float(row[field])
    return table


def plot_metric(rows: list[dict], case: str, mode: str, field: str, ylabel: str, title: str, path: Path, log_y: bool = True) -> None:
    table = grouped(rows, case, mode, field)
    if not table:
        return
    figure, axis = plt.subplots(figsize=(7.0, 4.4))
    bits_values = sorted(table)
    for threads in sorted({thread for values in table.values() for thread in values}):
        values = [table[bits][threads] for bits in bits_values]
        axis.plot(bits_values, values, marker="o", label=f"{threads} thread(s)")
    axis.set_xlabel("Toy key size (bits)")
    axis.set_ylabel(ylabel)
    axis.set_title(title)
    axis.set_xticks(bits_values)
    if log_y:
        axis.set_yscale("log")
    axis.grid(True, which="both", alpha=0.3)
    axis.legend()
    figure.tight_layout()
    figure.savefig(path, dpi=200)
    plt.close(figure)
    print(f"wrote {path}")


def plot_scaling(rows: list[dict]) -> None:
    figure, axes = plt.subplots(1, 2, figsize=(10.0, 4.2), sharey=True)
    for axis, case, title in zip(axes, ("rsa", "ecc"), ("RSA toy", "ECDLP toy")):
        for mode, marker in (("full", "o"), ("recovery", "s")):
            table = grouped(rows, case, mode, "median_ms")
            bits_values = sorted(bits for bits, values in table.items() if 1 in values and 12 in values)
            if not bits_values:
                continue
            speedups = [table[bits][1] / table[bits][12] for bits in bits_values]
            axis.plot(bits_values, speedups, marker=marker, label=mode)
        axis.set_title(title)
        axis.set_xlabel("Toy key size (bits)")
        axis.set_xticks(sorted({int(row["bits"]) for row in rows if row["case"] == case}))
        axis.grid(True, alpha=0.3)
        axis.legend()
    axes[0].set_ylabel("Speedup, 12 threads / 1 thread")
    figure.tight_layout()
    path = FIGURES_DIR / "fig_cpu_scaling.png"
    figure.savefig(path, dpi=200)
    plt.close(figure)
    print(f"wrote {path}")


def plot_gpu(cpu_rows: list[dict], gpu_rows: list[dict], path: Path, label: str) -> None:
    if not gpu_rows:
        return
    figure, axes = plt.subplots(1, 2, figsize=(10.0, 4.2), sharey=True)
    for axis, case, title in zip(axes, ("rsa", "ecc"), ("RSA toy", "ECDLP toy")):
        for mode, offset in (("full", -0.2), ("recovery", 0.2)):
            cpu_table = grouped(cpu_rows, case, mode, "median_ms")
            gpu_table = grouped(gpu_rows, case, mode, "median_ms")
            bits_values = sorted(set(cpu_table) & set(gpu_table))
            if not bits_values:
                continue
            speedups = []
            for bits in bits_values:
                cpu_value = cpu_table[bits].get(12, cpu_table[bits][max(cpu_table[bits])])
                gpu_value = gpu_table[bits][min(gpu_table[bits])]
                speedups.append(cpu_value / gpu_value if gpu_value else 0.0)
            positions = [bits + offset for bits in bits_values]
            axis.bar(positions, speedups, width=0.35, label=mode)
        axis.set_title(title)
        axis.set_xlabel("Toy key size (bits)")
        axis.set_xticks(sorted({int(row["bits"]) for row in cpu_rows if row["case"] == case}))
        axis.set_yscale("log")
        axis.grid(True, which="both", alpha=0.3)
        axis.legend()
    axes[0].set_ylabel(f"Speedup, CPU 12 threads / {label}")
    figure.suptitle(label)
    figure.tight_layout()
    figure.savefig(path, dpi=200)
    plt.close(figure)
    print(f"wrote {path}")


def main() -> int:
    ensure_dirs()
    cpu_path = SUMMARY_DIR / "cpu_summary.csv"
    if not cpu_path.exists():
        print(f"missing {cpu_path}; run run_bench.py first")
        return 2
    cpu_rows = read_csv(cpu_path)

    plot_metric(
        cpu_rows,
        "rsa",
        "recovery",
        "median_ms",
        "Median recovery time (ms, log scale)",
        "RSA toy brute-force recovery",
        FIGURES_DIR / "fig_rsa_recovery_time.png",
    )
    plot_metric(
        cpu_rows,
        "rsa",
        "full",
        "median_rate_mcand_s",
        "Candidates per second (millions, log scale)",
        "RSA toy full-scan throughput",
        FIGURES_DIR / "fig_rsa_throughput.png",
    )
    plot_metric(
        cpu_rows,
        "ecc",
        "recovery",
        "median_ms",
        "Median recovery time (ms, log scale)",
        "ECDLP toy brute-force recovery",
        FIGURES_DIR / "fig_ecc_recovery_time.png",
    )
    plot_metric(
        cpu_rows,
        "ecc",
        "full",
        "median_rate_mcand_s",
        "Candidates per second (millions, log scale)",
        "ECDLP toy full-scan throughput",
        FIGURES_DIR / "fig_ecc_throughput.png",
    )
    plot_scaling(cpu_rows)

    gpu_sets = [
        (
            SUMMARY_DIR / "gpu_cubecl_summary.csv",
            FIGURES_DIR / "fig_gpu_cubecl_speedup.png",
            "CubeCL GPU",
        ),
    ]
    for gpu_path, figure_path, label in gpu_sets:
        if gpu_path.exists():
            plot_gpu(cpu_rows, read_csv(gpu_path), figure_path, label)
        else:
            print(f"no summary at {gpu_path}; skipping {label} figure")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
