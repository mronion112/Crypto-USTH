#!/usr/bin/env python3
"""Demonstrate ECDSA private-key recovery after nonce reuse on P-256."""

from __future__ import annotations

import hashlib
import random
import sys
from pathlib import Path
from typing import Optional

sys.path.insert(0, str(Path(__file__).resolve().parent))

from common import RAW_DIR, ensure_dirs, write_csv

from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.asymmetric import ec
from cryptography.hazmat.primitives.asymmetric.utils import encode_dss_signature


P = 0xFFFFFFFF00000001000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFF
A = P - 3
B = 0x5AC635D8AA3A93E7B3EBBD55769886BC651D06B0CC53B0F63BCE3C3E27D2604B
GX = 0x6B17D1F2E12C4247F8BCE6E563A440F277037D812DEB33A0F4A13945D898C296
GY = 0x4FE342E2FE1A7F9B8EE7EB4A7C0F9E162BCE33576B315ECECBB6406837BF51F5
N = 0xFFFFFFFF00000000FFFFFFFFFFFFFFFFBCE6FAADA7179E84F3B9CAC2FC632551

Point = Optional[tuple[int, int]]


def ec_add(left: Point, right: Point) -> Point:
    if left is None:
        return right
    if right is None:
        return left
    x1, y1 = left
    x2, y2 = right
    if x1 == x2 and (y1 + y2) % P == 0:
        return None
    if left == right:
        slope = (3 * x1 * x1 + A) * pow(2 * y1, -1, P) % P
    else:
        slope = (y2 - y1) * pow(x2 - x1, -1, P) % P
    x3 = (slope * slope - x1 - x2) % P
    y3 = (slope * (x1 - x3) - y1) % P
    return x3, y3


def ec_mul(scalar: int, point: Point) -> Point:
    result: Point = None
    addend = point
    while scalar > 0:
        if scalar & 1:
            result = ec_add(result, addend)
        addend = ec_add(addend, addend)
        scalar >>= 1
    return result


def digest_scalar(message: bytes) -> int:
    return int.from_bytes(hashlib.sha256(message).digest(), "big") % N


def sign(message: bytes, private_key: int, nonce: int) -> tuple[int, int]:
    point = ec_mul(nonce, (GX, GY))
    if point is None:
        raise ValueError("nonce produced the point at infinity")
    r = point[0] % N
    z = digest_scalar(message)
    s = pow(nonce, -1, N) * (z + r * private_key) % N
    return r, s


def cryptography_verifies(message: bytes, signature: tuple[int, int], public_key: ec.EllipticCurvePublicKey) -> bool:
    r, s = signature
    try:
        der = encode_dss_signature(r, s)
        public_key.verify(der, message, ec.ECDSA(hashes.SHA256()))
        return True
    except Exception:
        return False


def main() -> int:
    ensure_dirs()
    rng = random.Random(20261105)

    while True:
        private_key = rng.randrange(1, N)
        nonce = rng.randrange(1, N)
        message_one = b"bank-transfer:100000:VND:nonce-reuse-demo-1"
        message_two = b"bank-transfer:250000:VND:nonce-reuse-demo-2"
        signature_one = sign(message_one, private_key, nonce)
        signature_two = sign(message_two, private_key, nonce)
        if signature_one[0] == 0 or signature_one[1] == 0 or signature_two[1] == 0:
            continue
        if signature_one[1] == signature_two[1]:
            continue
        break

    public_point = ec_mul(private_key, (GX, GY))
    assert public_point is not None
    public_key = ec.derive_private_key(private_key, ec.SECP256R1()).public_key()
    numbers = public_key.public_numbers()
    public_matches = numbers.x == public_point[0] and numbers.y == public_point[1]

    z1 = digest_scalar(message_one)
    z2 = digest_scalar(message_two)
    r, s1 = signature_one
    _, s2 = signature_two

    nonce_recovered = (z1 - z2) * pow((s1 - s2) % N, -1, N) % N
    private_recovered = (s1 * nonce_recovered - z1) * pow(r, -1, N) % N

    recovered_point = ec_mul(private_recovered, (GX, GY))
    recovered = nonce_recovered == nonce and private_recovered == private_key
    verified = recovered and recovered_point == public_point and public_matches
    signature_one_ok = cryptography_verifies(message_one, signature_one, public_key)
    signature_two_ok = cryptography_verifies(message_two, signature_two, public_key)

    rows = [
        {
            "case": "ecdsa_p256_nonce_reuse",
            "recovered": int(recovered),
            "verified": int(verified),
            "signature_verified_1": int(signature_one_ok),
            "signature_verified_2": int(signature_two_ok),
            "note": "algebraic recovery from reused nonce; no key search",
        }
    ]
    output = RAW_DIR / "nonce_reuse.csv"
    write_csv(output, rows)
    print(rows[0])
    print(f"wrote {output}")
    return 0 if verified and signature_one_ok and signature_two_ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
