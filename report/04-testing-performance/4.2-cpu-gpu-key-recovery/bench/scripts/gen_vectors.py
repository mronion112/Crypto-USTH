#!/usr/bin/env python3
"""Generate deterministic toy RSA moduli and ECDLP instances."""

from __future__ import annotations

import random
import sys
from pathlib import Path
from typing import Optional

sys.path.insert(0, str(Path(__file__).resolve().parent))

from common import CONFIG_DIR, DATA_DIR, dump_json, ensure_dirs, load_json


Point = Optional[tuple[int, int]]


def is_prime(value: int) -> bool:
    if value < 2:
        return False
    small_primes = (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37)
    for prime in small_primes:
        if value == prime:
            return True
        if value % prime == 0:
            return False

    odd = value - 1
    shifts = 0
    while odd % 2 == 0:
        odd //= 2
        shifts += 1

    for base in small_primes:
        witness = pow(base, odd, value)
        if witness in (1, value - 1):
            continue
        composite = True
        for _ in range(1, shifts):
            witness = pow(witness, 2, value)
            if witness == value - 1:
                composite = False
                break
        if composite:
            return False
    return True


def random_prime(rng: random.Random, bits: int) -> int:
    low = 1 << (bits - 1)
    high = 1 << bits
    while True:
        candidate = rng.randrange(low, high) | 1
        if candidate < high and is_prime(candidate):
            return candidate


def generate_semiprime(rng: random.Random, bits: int) -> tuple[int, int, int]:
    left_bits = bits // 2
    right_bits = bits - left_bits
    while True:
        left = random_prime(rng, left_bits)
        right = random_prime(rng, right_bits)
        if left == right:
            continue
        modulus = left * right
        if modulus.bit_length() != bits:
            continue
        if left > right:
            left, right = right, left
        return left, right, modulus


def ec_add(left: Point, right: Point, p: int, a: int) -> Point:
    if left is None:
        return right
    if right is None:
        return left
    x1, y1 = left
    x2, y2 = right
    if x1 == x2 and (y1 + y2) % p == 0:
        return None
    if left == right:
        slope = (3 * x1 * x1 + a) * pow(2 * y1, -1, p) % p
    else:
        slope = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (slope * slope - x1 - x2) % p
    y3 = (slope * (x1 - x3) - y1) % p
    return x3, y3


def ec_mul(scalar: int, point: Point, p: int, a: int) -> Point:
    result: Point = None
    addend = point
    while scalar > 0:
        if scalar & 1:
            result = ec_add(result, addend, p, a)
        addend = ec_add(addend, addend, p, a)
        scalar >>= 1
    return result


def validate_curve(curve: dict) -> None:
    p, a, b = int(curve["p"]), int(curve["a"]), int(curve["b"])
    gx, gy = int(curve["gx"]), int(curve["gy"])
    q = int(curve["q"])
    if (gy * gy - (gx * gx * gx + a * gx + b)) % p != 0:
        raise ValueError(f"point is not on the {curve['bits']}-bit curve")
    if not is_prime(q):
        raise ValueError(f"subgroup order is not prime for {curve['bits']}-bit curve")
    if ec_mul(q, (gx, gy), p, a) is not None:
        raise ValueError(f"base point order mismatch for {curve['bits']}-bit curve")


def main() -> int:
    ensure_dirs()
    config = load_json(CONFIG_DIR / "bench.json")
    curves = load_json(CONFIG_DIR / "curves.json")
    runs = int(config["runs"])
    rng = random.Random(int(config["seed"]))

    vectors: dict[str, object] = {
        "seed": int(config["seed"]),
        "runs": runs,
        "rsa": [],
        "ecc": [],
    }

    for bits in config["rsa_bits"]:
        entries = []
        for _ in range(runs):
            left, right, modulus = generate_semiprime(rng, int(bits))
            entries.append({"n": modulus, "p": left, "q": right})
        vectors["rsa"].append({"bits": int(bits), "vectors": entries})
        print(f"RSA {bits} bits: {runs} moduli")

    for curve in curves:
        validate_curve(curve)
        p, a = int(curve["p"]), int(curve["a"])
        q = int(curve["q"])
        generator = (int(curve["gx"]), int(curve["gy"]))
        entries = []
        for _ in range(runs):
            scalar = rng.randrange(1, q)
            public = ec_mul(scalar, generator, p, a)
            if public is None:
                raise ValueError("unexpected point at infinity")
            entries.append({"d": scalar, "qx": public[0], "qy": public[1]})
        vectors["ecc"].append(
            {
                "bits": int(curve["bits"]),
                "curve": {
                    "p": p,
                    "a": a,
                    "b": int(curve["b"]),
                    "gx": generator[0],
                    "gy": generator[1],
                    "q": q,
                },
                "vectors": entries,
            }
        )
        print(f"ECC {curve['bits']} bits: {runs} scalars (q={q})")

    dump_json(DATA_DIR / "vectors.json", vectors)
    print(f"wrote {(DATA_DIR / 'vectors.json').relative_to(DATA_DIR.parents[2])}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
