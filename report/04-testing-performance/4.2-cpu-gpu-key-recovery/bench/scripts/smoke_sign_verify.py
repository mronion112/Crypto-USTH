#!/usr/bin/env python3
"""Correctness smoke tests for the five signature families in the report."""

from __future__ import annotations

import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from common import RAW_DIR, ensure_dirs, write_csv

from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.asymmetric import ec, ed25519, mldsa, padding, rsa


MESSAGE = b"section4 correctness test message"


def record(rows: list[dict], algorithm: str, variant: str, key_bits: int, sig_bytes: int, verified: bool, notes: str = "") -> None:
    rows.append(
        {
            "algorithm": algorithm,
            "variant": variant,
            "key_bits": key_bits,
            "sig_bytes": sig_bytes,
            "verified": int(verified),
            "notes": notes,
        }
    )


def test_rsa(rows: list[dict]) -> None:
    for bits in (2048, 4096):
        try:
            key = rsa.generate_private_key(public_exponent=65537, key_size=bits)
            signature = key.sign(
                MESSAGE,
                padding.PSS(
                    mgf=padding.MGF1(hashes.SHA256()),
                    salt_length=padding.PSS.MAX_LENGTH,
                ),
                hashes.SHA256(),
            )
            key.public_key().verify(
                signature,
                MESSAGE,
                padding.PSS(
                    mgf=padding.MGF1(hashes.SHA256()),
                    salt_length=padding.PSS.MAX_LENGTH,
                ),
                hashes.SHA256(),
            )
            record(rows, "RSA-PSS", f"RSA-{bits}", bits, len(signature), True)
        except Exception as error:
            record(rows, "RSA-PSS", f"RSA-{bits}", bits, 0, False, type(error).__name__)


def test_ecdsa(rows: list[dict]) -> None:
    try:
        key = ec.generate_private_key(ec.SECP256R1())
        signature = key.sign(MESSAGE, ec.ECDSA(hashes.SHA256()))
        key.public_key().verify(signature, MESSAGE, ec.ECDSA(hashes.SHA256()))
        record(rows, "ECDSA", "P-256", 256, len(signature), True)
    except Exception as error:
        record(rows, "ECDSA", "P-256", 256, 0, False, type(error).__name__)


def test_ed25519(rows: list[dict]) -> None:
    try:
        key = ed25519.Ed25519PrivateKey.generate()
        signature = key.sign(MESSAGE)
        key.public_key().verify(signature, MESSAGE)
        record(rows, "Ed25519", "Ed25519", 256, len(signature), True)
    except Exception as error:
        record(rows, "Ed25519", "Ed25519", 256, 0, False, type(error).__name__)


def test_ml_dsa(rows: list[dict]) -> None:
    variants = (
        ("ML-DSA-44", mldsa.MLDSA44PrivateKey),
        ("ML-DSA-65", mldsa.MLDSA65PrivateKey),
        ("ML-DSA-87", mldsa.MLDSA87PrivateKey),
    )
    for name, key_type in variants:
        try:
            key = key_type.generate()
            signature = key.sign(MESSAGE)
            key.public_key().verify(signature, MESSAGE)
            record(rows, "ML-DSA", name, 0, len(signature), True, "FIPS 204")
        except Exception as error:
            record(rows, "ML-DSA", name, 0, 0, False, type(error).__name__)


def test_sm2(rows: list[dict]) -> None:
    openssl = shutil.which("openssl")
    if openssl is None:
        record(rows, "SM2", "SM2/SM3", 256, 0, False, "openssl not found")
        return

    temp_root = Path(tempfile.gettempdir())
    temp_root.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="sm2-", dir=temp_root) as temp_dir:
        directory = Path(temp_dir)
        message_path = directory / "message.bin"
        key_path = directory / "sm2-key.pem"
        public_path = directory / "sm2-public.pem"
        signature_path = directory / "sm2-signature.der"
        message_path.write_bytes(MESSAGE)

        try:
            subprocess.run(
                [openssl, "genpkey", "-algorithm", "SM2", "-out", str(key_path)],
                check=True,
                capture_output=True,
            )
            subprocess.run(
                [openssl, "pkey", "-in", str(key_path), "-pubout", "-out", str(public_path)],
                check=True,
                capture_output=True,
            )
            subprocess.run(
                [
                    openssl,
                    "dgst",
                    "-sm3",
                    "-sign",
                    str(key_path),
                    "-out",
                    str(signature_path),
                    str(message_path),
                ],
                check=True,
                capture_output=True,
            )
            verified = subprocess.run(
                [
                    openssl,
                    "dgst",
                    "-sm3",
                    "-verify",
                    str(public_path),
                    "-signature",
                    str(signature_path),
                    str(message_path),
                ],
                check=True,
                capture_output=True,
                text=True,
            )
            ok = "Verified OK" in (verified.stdout + verified.stderr)
            record(rows, "SM2", "SM2/SM3", 256, signature_path.stat().st_size, ok, "OpenSSL CLI")
        except Exception as error:
            record(rows, "SM2", "SM2/SM3", 256, 0, False, type(error).__name__)


def main() -> int:
    ensure_dirs()
    rows: list[dict] = []
    test_rsa(rows)
    test_ecdsa(rows)
    test_ed25519(rows)
    test_ml_dsa(rows)
    test_sm2(rows)

    output = RAW_DIR / "smoke_sign_verify.csv"
    write_csv(output, rows, ["algorithm", "variant", "key_bits", "sig_bytes", "verified", "notes"])
    for row in rows:
        status = "PASS" if row["verified"] == 1 else "FAIL"
        print(f"{status} {row['algorithm']} {row['variant']} sig={row['sig_bytes']}B {row['notes']}")
    print(f"wrote {output}")
    return 0 if all(row["verified"] == 1 for row in rows) else 1


if __name__ == "__main__":
    raise SystemExit(main())
