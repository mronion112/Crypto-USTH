"""Shared paths and helpers for the section 4 benchmark scripts."""

from __future__ import annotations

import csv
import json
import statistics
import subprocess
from pathlib import Path
from typing import Iterable, Sequence


BENCH_DIR = Path(__file__).resolve().parents[1]
SECTION_DIR = BENCH_DIR.parent
CONFIG_DIR = BENCH_DIR / "config"
DATA_DIR = SECTION_DIR / "data"
RAW_DIR = DATA_DIR / "raw"
SUMMARY_DIR = DATA_DIR / "summary"
FIGURES_DIR = SECTION_DIR / "figures"


def ensure_dirs() -> None:
    for path in (RAW_DIR, SUMMARY_DIR, FIGURES_DIR):
        path.mkdir(parents=True, exist_ok=True)


def load_json(path: Path) -> object:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def dump_json(path: Path, value: object) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8") as handle:
        json.dump(value, handle, indent=2, sort_keys=False)
        handle.write("\n")


def write_csv(path: Path, rows: Sequence[dict], fieldnames: Sequence[str] | None = None) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    if fieldnames is None:
        if not rows:
            raise ValueError(f"no rows and no fieldnames for {path}")
        fieldnames = list(rows[0].keys())
    with path.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(handle, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(rows)


def read_csv(path: Path) -> list[dict]:
    with path.open("r", newline="", encoding="utf-8") as handle:
        return list(csv.DictReader(handle))


def run_command(command: Sequence[str], cwd: Path | None = None) -> str:
    completed = subprocess.run(
        [str(part) for part in command],
        cwd=str(cwd) if cwd else None,
        check=True,
        text=True,
        capture_output=True,
    )
    return completed.stdout


def binary(name: str) -> Path:
    return BENCH_DIR / "target" / "release" / name


def median(values: Iterable[float]) -> float:
    return statistics.median(list(values))


def quartiles(values: Sequence[float]) -> tuple[float, float]:
    if len(values) < 2:
        return values[0], values[0]
    q1, _, q3 = statistics.quantiles(values, n=4, method="inclusive")
    return q1, q3


def summarize(values: Sequence[float]) -> dict[str, float]:
    ordered = sorted(values)
    q1, q3 = quartiles(ordered)
    return {
        "median": statistics.median(ordered),
        "iqr": q3 - q1,
        "min": ordered[0],
        "max": ordered[-1],
        "mean": statistics.fmean(ordered),
    }


def markdown_table(headers: Sequence[str], rows: Sequence[Sequence[object]]) -> str:
    lines = ["| " + " | ".join(headers) + " |"]
    lines.append("|" + "|".join(["---"] * len(headers)) + "|")
    for row in rows:
        lines.append("| " + " | ".join(str(value) for value in row) + " |")
    return "\n".join(lines)
