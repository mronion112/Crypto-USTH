# 2.4 ECDSA

This section analyzes the Elliptic Curve Digital Signature Algorithm (ECDSA) using **NIST P-256** (also named `secp256r1`) with **SHA-256** as the report's fixed configuration. This choice is consistent with NIST FIPS 186-5 and SP 800-186, provides approximately 128 bits of classical security, and matches the ECDSA correctness and nonce-reuse test cases in Section 4.1 [1], [2].

## 2.4.1 Mathematical Theory

### Scope, standards, and fixed configuration

ECDSA is the elliptic-curve analogue of the Digital Signature Algorithm. Its security is based on the difficulty of recovering a secret scalar from a public elliptic-curve point. ECDSA provides message integrity, signer authentication, and evidence supporting non-repudiation when the private key, certificate identity, signing process, and audit controls are managed correctly. It does not encrypt the message and does not by itself provide user authentication, authorization, replay protection, or transport confidentiality.

NIST FIPS 186-5 specifies ECDSA key generation, per-message secret generation, signing, and verification. NIST SP 800-186 specifies the recommended elliptic-curve domain parameters. For interoperability, RFC 5480 identifies the combination of P-256 and SHA-256 with a target security strength of approximately 128 bits [1], [2], [3]. Table 2.4.1 fixes the configuration used throughout this report.

**Table 2.4.1. ECDSA configuration used in this report**

| Item | Selected value | Rationale |
|---|---|---|
| Signature algorithm | ECDSA | Approved in NIST FIPS 186-5 |
| Elliptic curve | NIST P-256 (`secp256r1`) | NIST-recommended 256-bit prime-field curve; approximately 128-bit classical security |
| Curve model | Short Weierstrass curve over $\mathbb{F}_p$ | Matches the P-256 definition in SP 800-186 |
| Hash function | SHA-256 | Security strength and output size are aligned with P-256 |
| Per-message secret | Random generation approved by FIPS 186-5, or deterministic generation following FIPS 186-5/RFC 6979 | Prevents key recovery caused by weak, biased, or repeated nonces |
| Signature representation | Mathematical pair $(r,s)$; DER-encoded ASN.1 in the Section 4.1 test | DER size varies with integer encoding; Test C3 observed 72 bytes |

### P-256 domain parameters

P-256 is a short-Weierstrass curve over the prime field $\mathbb{F}_p$:

$$
E: y^2 \equiv x^3 + ax + b \pmod p. \tag{2.4.1}
$$

Its domain parameters are $D=(p,a,b,G,n,h)$, where $G=(G_x,G_y)$ is a base point of prime order $n$ and the cofactor is $h=1$. The values in Table 2.4.2 are taken from NIST SP 800-186 [2].

**Table 2.4.2. NIST P-256 domain parameters**

| Symbol | Meaning | Value |
|---|---|---|
| $p$ | Prime defining $\mathbb{F}_p$ | `FFFFFFFF00000001000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFF` |
| $a$ | Curve coefficient | `FFFFFFFF00000001000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFC` ($p-3$) |
| $b$ | Curve coefficient | `5AC635D8AA3A93E7B3EBBD55769886BC651D06B0CC53B0F63BCE3C3E27D2604B` |
| $G_x$ | Base-point x-coordinate | `6B17D1F2E12C4247F8BCE6E563A440F277037D812DEB33A0F4A13945D898C296` |
| $G_y$ | Base-point y-coordinate | `4FE342E2FE1A7F9B8EE7EB4A7C0F9E162BCE33576B315ECECBB6406837BF51F5` |
| $n$ | Prime order of $G$ | `FFFFFFFF00000000FFFFFFFFFFFFFFFFBCE6FAADA7179E84F3B9CAC2FC632551` |
| $h$ | Cofactor | $1$ |

The base point generates a prime-order subgroup:

$$
[n]G=\mathcal{O}, \tag{2.4.2}
$$

where $\mathcal{O}$ is the point at infinity. Because $n$ is close to $2^{256}$, generic attacks against the corresponding discrete-logarithm problem require work on the order of $2^{128}$ group operations rather than $2^{256}$ when Pollard's rho is used.

### Key generation

For the fixed domain parameters $D$, the signer generates a private scalar

$$
d \stackrel{\$}{\leftarrow} \{1,2,\ldots,n-1\} \tag{2.4.3}
$$

using an approved cryptographically secure random bit generator. The public key is the curve point

$$
Q=[d]G. \tag{2.4.4}
$$

Computing $Q$ from $d$ is efficient through elliptic-curve scalar multiplication. Recovering $d$ from the public values $(G,Q)$ is the elliptic-curve discrete-logarithm problem (ECDLP). The private scalar must remain confidential and integrity-protected throughout its cryptoperiod. The public point must remain associated with the P-256 domain parameters and should be validated before it is used to verify signatures [1], [2].

### Signature generation

Let $M$ be the message and let

$$
H=\operatorname{SHA\text{-}256}(M). \tag{2.4.5}
$$

FIPS 186-5 converts the leftmost $\min(\operatorname{len}(n),\operatorname{len}(H))$ bits of $H$ to an integer $e$. Because both the P-256 subgroup order and SHA-256 output are approximately 256 bits, this configuration uses the complete SHA-256 output as the input integer.

To sign $M$ with private key $d$:

1. Generate a secret per-message value $k$ satisfying $1\leq k\leq n-1$. It may be produced by an approved random process or deterministically from the private key and message as specified by FIPS 186-5 and RFC 6979 [1], [4].
2. Compute the curve point
   $$
   R=[k]G=(x_R,y_R). \tag{2.4.6}
   $$
3. Compute
   $$
   r=x_R\bmod n. \tag{2.4.7}
   $$
   If $r=0$, a randomly generated $k$ must be replaced. For deterministic ECDSA, FIPS 186-5 specifies failure because regenerating the same deterministic value would reproduce the same result.
4. Compute
   $$
   s=k^{-1}(e+r d)\bmod n. \tag{2.4.8}
   $$
   If $s=0$, handle the failure in the same way as for $r=0$.
5. Output the signature $(r,s)$ and securely destroy $k$ and $k^{-1}$.

The secrecy and integrity of $k$ are as important as those of the long-term private key. Knowledge of a single nonce and its signature immediately reveals $d$:

$$
d=(s k-e)r^{-1}\bmod n. \tag{2.4.9}
$$

The mathematical signature consists of two integers. Some protocols serialize them as a fixed-width 64-byte string, while X.509- and ASN.1-based implementations commonly use a DER `SEQUENCE` containing two signed `INTEGER` values. DER length is therefore variable; the P-256/SHA-256 smoke test in Section 4.1 observed one 72-byte encoding and must not be interpreted as a universal ECDSA signature length.

### Signature verification

The verifier receives $(M,r,s)$, the purported public key $Q$, and the P-256 domain parameters. Before performing the verification equation, the implementation must ensure that $Q$ is a valid non-identity point on the selected curve and that the key is associated with the intended domain parameters [1], [2]. It then performs the following steps:

1. Reject unless $r$ and $s$ are integers in $[1,n-1]$.
2. Compute $H=\operatorname{SHA\text{-}256}(M)$ and derive the integer $e$ in the same way as the signer.
3. Compute
   $$
   w=s^{-1}\bmod n,\qquad u_1=e w\bmod n,\qquad u_2=r w\bmod n. \tag{2.4.10}
   $$
4. Compute
   $$
   X=[u_1]G+[u_2]Q. \tag{2.4.11}
   $$
   Reject if $X=\mathcal{O}$.
5. Accept if and only if
   $$
   r=x_X\bmod n. \tag{2.4.12}
   $$

Correctness follows by substituting $Q=[d]G$ and $s=k^{-1}(e+rd)$:

$$
\begin{aligned}
[u_1]G+[u_2]Q
&=[e s^{-1}]G+[r s^{-1}][d]G\\
&=[(e+rd)s^{-1}]G\\
&=[k]G=R.
\end{aligned} \tag{2.4.13}
$$

The verifier therefore reconstructs a point whose x-coordinate produces the original value $r$, provided that the signature, message, public key, and domain parameters are consistent.

### Security foundation and classical attacks

The central mathematical assumption is the hardness of ECDLP: given $G$ and $Q=[d]G$, recover $d$. Exhaustively testing every possible scalar has complexity $O(n)$. Pollard's rho uses pseudorandom walks and collision detection to reduce the expected generic attack cost to approximately

$$
\sqrt{\frac{\pi n}{2}}=O(\sqrt n). \tag{2.4.14}
$$

For the P-256 subgroup, $n\approx2^{256}$, so the generic classical cost is on the order of $2^{128}$ curve operations. This is the basis for describing P-256 as providing approximately 128 bits of classical security [1], [2], [5]. Pollard's rho is not brute force: it searches for a useful collision in a pseudorandom walk and uses that collision to solve the discrete logarithm. Conversely, the sequential scalar search in Section 4.2 is a deliberately simple toy baseline with $O(n)$ work.

This estimate assumes correct domain parameters, a valid public key, constant-time and fault-resistant arithmetic, and uncompromised key storage. It does not imply that every P-256 implementation provides 128-bit operational security. Side-channel leakage, faulty point arithmetic, weak private-key generation, invalid-point handling, or exposed HSM credentials can bypass the ECDLP assumption without solving the mathematical problem.

### Nonce reuse, nonce bias, and partial leakage

Nonce failure is the most direct ECDSA-specific implementation risk. Suppose the same private key $d$ signs two different message hashes $e_1$ and $e_2$ using the same nonce $k$. The point $[k]G$ is identical, so both signatures share the same $r$:

$$
s_1=k^{-1}(e_1+r d)\bmod n,\qquad
s_2=k^{-1}(e_2+r d)\bmod n. \tag{2.4.15}
$$

Subtracting the equations gives

$$
k=(e_1-e_2)(s_1-s_2)^{-1}\bmod n, \tag{2.4.16}
$$

after which the long-term private key is recovered as

$$
d=(s_1 k-e_1)r^{-1}\bmod n. \tag{2.4.17}
$$

This is an algebraic key-recovery attack. It requires neither brute force nor Pollard's rho and is exactly the failure illustrated by test I1 in Section 4.1. Biased nonces, partially predictable nonces, or side-channel leakage of nonce bits can also expose the private key across multiple signatures through hidden-number-problem and lattice techniques [6].

The primary mitigations are:

- use the approved random-generation procedures in FIPS 186-5 with a correctly seeded cryptographic random bit generator, or use deterministic ECDSA as defined by FIPS 186-5 and RFC 6979;
- generate and use $k$ inside a protected signing boundary, and destroy $k$ and $k^{-1}$ after signing;
- implement scalar multiplication and modular inversion in constant time, with protections against timing, cache, power, electromagnetic, and fault attacks;
- validate P-256 public keys and domain-parameter associations before verification;
- protect the long-term key in a controlled cryptographic module, enforce authorization and audit policies around signing, and rotate or revoke a key after suspected compromise.

Deterministic nonce generation removes dependence on fresh randomness during each signature, but it does not eliminate all implementation risks. The private key still requires strong randomness at key-generation time, and deterministic implementations remain exposed to side-channel and fault attacks if sensitive computations are not protected [1], [4].

### Quantum threat and benchmark boundary

ECDSA is not post-quantum secure. A sufficiently large fault-tolerant quantum computer running Shor's algorithm could solve elliptic-curve discrete logarithms in polynomial time. This is a theoretical migration risk rather than evidence that current quantum computers can recover P-256 private keys; the report treats the issue in Section 4.3.

The Section 4.2 ECDLP benchmark uses small, generated curves with 18-, 22-, and 26-bit field sizes. Those measurements demonstrate search scaling, CPU/GPU implementation effects, and the difference between exhaustive search and Pollard's rho. They do not measure the security of P-256, and their running times must not be extrapolated to claim that a standard 256-bit ECDSA key can be recovered on the tested hardware.

The implementation artifacts align with this section as follows:

- Test C3 calls the `cryptography` library with `SECP256R1` and SHA-256, verifies the generated signature, and records the length of that one DER encoding. It is a correctness test, not a signing-performance or security-strength benchmark.
- Test I1 implements the P-256 equations in (2.4.15)-(2.4.17), confirms that both signatures verify through the independent library API, and checks that the recovered scalar produces the original public point. Its seeded pseudorandom generator is used only to make the demonstration reproducible and is not presented as an approved production key or nonce generator.
- Tests K2 and K3 recover scalars on generated toy curves by exhaustive search and Pollard's rho, respectively. Their results illustrate algorithmic scaling and implementation behavior, not key recovery against P-256.

## 2.4.2 Application in Banking System

### Role in a banking signature architecture

ECDSA P-256 is suitable as a classical signature option in banking systems that require compact certificates and signatures, standardized X.509 integration, and approximately 128 bits of classical security. Potential uses include signing canonical transaction envelopes, service-to-service messages, approval records, settlement instructions, audit evidence, and software or device-management artifacts. The algorithm should be treated as one selectable signature suite in the Section 3 workflow rather than combined with every other signature algorithm in a single mandatory pipeline.

The object signed by ECDSA must be an unambiguous byte sequence. For a banking transaction, canonicalization should bind at least the transaction identifier, source and destination accounts, amount, currency, timestamp, replay-prevention value, and business purpose. Signing only a partial or ambiguously encoded structure can permit a valid signature to be interpreted under unintended transaction data. The signature provides evidence that the protected key approved those bytes; it does not prove that the transaction complied with account limits, sanctions screening, fraud controls, or dual-authorization policy.

A deployable ECDSA transaction flow therefore requires controls outside the mathematical signature:

1. **Authorization before signing:** Authenticate the requester and apply role, amount, and approval-quorum policies before the signing service invokes the private key.
2. **Canonical message construction:** Convert the complete transaction envelope into one deterministic representation and record the exact algorithm identifier (`ECDSA-P256-SHA256`).
3. **Protected signature generation:** Generate and retain the private key in a controlled cryptographic module. Permit signing only through an authenticated service interface and log each authorized operation.
4. **Protected transport:** Send the signed envelope through TLS or another authenticated confidential channel. ECDSA does not encrypt transaction data.
5. **Verification and trust validation:** Validate the certificate path and revocation status, confirm that the public key is a valid P-256 point, enforce the expected hash and signature encoding, and verify the signature.
6. **Replay and business checks:** Reject duplicate transaction identifiers or nonces, enforce timestamp windows, and execute account, fraud, compliance, and settlement checks independently of signature verification.
7. **Audit and response:** Preserve the canonical payload, signature, algorithm identifiers, certificate evidence, verification result, and decision trail. Revoke and replace the key when compromise is suspected.

### Key management and HSM requirements

The operational security of ECDSA depends heavily on key and nonce handling. NIST SP 800-57 treats key generation, storage, use, cryptoperiod, backup, compromise recovery, revocation, and destruction as parts of one key-management lifecycle [7]. For a banking deployment, the ECDSA private key should therefore be non-exportable where practical, separated by environment and purpose, and accessible only to authenticated roles and approved signing services.

A hardware security module (HSM) or equivalent controlled cryptographic module can enforce private-key isolation, role-based access, audit logging, self-tests, and protections for sensitive security parameters. FIPS 140-3 defines security requirements for cryptographic modules across interfaces, roles and authentication, sensitive parameter management, physical security, self-tests, lifecycle assurance, and attack mitigation [8]. Certification alone does not make an overall banking workflow secure: the deployed module, firmware, operating mode, algorithm implementation, surrounding authorization service, and key-management procedures must all be included in the assessment.

An ECDSA signing service should enforce the following configuration controls:

- pin P-256 and SHA-256 rather than accepting arbitrary caller-selected curves or hashes;
- use an approved random nonce generator or a reviewed deterministic-ECDSA implementation conforming to FIPS 186-5/RFC 6979;
- prohibit reuse of a private key for unrelated cryptographic purposes;
- keep nonce generation, modular inversion, and scalar multiplication inside the protected implementation boundary;
- use constant-time, fault-aware cryptographic libraries and current validated implementations where policy requires them;
- check return codes and reject malformed, out-of-range, or non-canonical signature encodings;
- maintain key identifiers, certificate bindings, activation and expiration dates, revocation status, and an auditable record of administrative changes.

Deterministic ECDSA reduces the risk that a failed random source will repeat or bias $k$, but it must not be described as a substitute for HSM controls or side-channel resistance. A deterministic signer repeats the same sensitive computation for the same key and message, so timing, power, electromagnetic, cache, and fault protections remain necessary [1], [4].

### Input validation and interoperability

Banking integrations often fail at the encoding and policy boundaries rather than in the ECDSA equation itself. The verifier should enforce all of the following before accepting a transaction:

- the certificate and algorithm identifier select P-256 rather than a caller-supplied or unknown curve;
- the public key is a valid non-identity point on P-256 and is associated with the validated certificate;
- both signature integers satisfy $1\le r,s\le n-1$;
- SHA-256 is applied to the same canonical bytes used by the signer;
- the signature encoding follows the protocol's required format, such as strict DER or a fixed-width raw $r\parallel s$ representation;
- trailing data, non-minimal integer encodings, unknown algorithm parameters, and format conversions are rejected rather than silently normalized;
- certificate validity, revocation, key usage, policy, signer identity, and authorization are checked separately from the ECDSA result.

RFC 5480 defines the ECC public-key information format used in X.509 and identifies `secp256r1` for P-256 interoperability [3]. A system must nevertheless specify the complete signature profile. Curve name alone is insufficient because the hash, message canonicalization, signature encoding, certificate policy, and nonce-generation behavior also affect compatibility and security.

### Comparison with Ed25519 and SM2

ECDSA P-256, Ed25519, and SM2 are all elliptic-curve signature schemes with roughly 256-bit public-key groups and approximately 128-bit classical security targets, but they are not interchangeable. Their domain parameters, signature equations, hash processing, nonce rules, public-key encodings, and deployment standards differ.

**Table 2.4.3. Deployment-oriented comparison of ECDSA P-256, Ed25519, and SM2**

| Criterion | ECDSA P-256 | Ed25519 | SM2 |
|---|---|---|---|
| Main curve/profile in this report | NIST P-256 / `secp256r1` | `edwards25519` | `sm2p256v1` |
| Hash configuration | SHA-256 | SHA-512 as an internal component of Ed25519 | SM3 in the selected SM2 profile |
| Per-message secret | Fresh random $k$ or deterministic ECDSA | Derived deterministically from the private-key expansion and message | Fresh secret nonce in the selected profile |
| Signature encoding | Pair $(r,s)$; commonly variable-length DER or protocol-defined 64-byte raw form | Fixed 64-byte encoding | Pair $(r,s)$; encoding depends on the protocol |
| Identity binding inside the signature input | No built-in signer-ID value; identity normally comes from certificate/protocol context | No built-in signer-ID value; context depends on the Ed25519 variant/protocol | Includes signer identity and public-key context through $Z_A$ in the standard signature construction |
| Primary implementation concern | Catastrophic key exposure from repeated, biased, or leaked nonce; strict curve and point validation | Correct variant selection, key/point decoding, side-channel-safe implementation | Consistent curve, SM3, signer ID, encoding, and secure nonce generation |
| Integration consideration | Strong X.509 and existing ECC interoperability through standardized P-256 identifiers | Fixed small keys/signatures and deterministic signing; compatibility must be confirmed in the target PKI/HSM stack | Appropriate where the applicable protocol, regulation, or ecosystem requires the SM2/SM3 profile |

RFC 8032 specifies that Ed25519 does not require a unique random number for each signature and uses 32-byte public keys and 64-byte signatures [9]. SM2 uses a distinct 256-bit curve and, in its commonly standardized configuration, SM3 plus signer-identity context; the signer and verifier must agree on all parameters [10], [11]. These differences have direct operational consequences. A bank cannot replace one scheme with another by changing only an algorithm label while leaving key formats, certificates, signing APIs, and verification rules unchanged.

ECDSA P-256 is a reasonable classical choice when interoperability with existing P-256 certificate and cryptographic-module ecosystems is the dominant requirement. Ed25519 can reduce signature and implementation complexity in systems whose policies and hardware support it. SM2 may be required for specific standards or regional interoperability. The final choice should be based on policy, HSM and certificate support, protocol compatibility, threat model, and migration requirements rather than signature speed alone.

### Advantages, limitations, and deployment position

The principal advantages of ECDSA P-256 are compact keys and signatures relative to RSA, mature standardization, an approximately 128-bit classical security target, and established use in certificate-based protocols. Its main limitations are the fragility of nonce handling, the need for careful point and encoding validation, exposure to side-channel and fault attacks, and vulnerability to a future cryptographically relevant quantum computer.

For the report's banking workflow, ECDSA P-256 should be presented as a currently deployable classical signature profile, not as a complete security architecture and not as a long-term post-quantum solution. Banks adopting it should preserve algorithm identifiers and variable-length fields, separate signature policy from application logic, maintain an inventory of keys and dependent systems, and design for replacement or hybrid migration as post-quantum standards and compatible banking infrastructure mature.

## References

1. National Institute of Standards and Technology, *Digital Signature Standard (DSS)*, FIPS PUB 186-5, U.S. Department of Commerce, Feb. 2023, doi: 10.6028/NIST.FIPS.186-5.
2. L. Chen, D. Moody, A. Regenscheid, A. Robinson, and K. Randall, *Recommendations for Discrete Logarithm-Based Cryptography: Elliptic Curve Domain Parameters*, NIST Special Publication 800-186, Feb. 2023, doi: 10.6028/NIST.SP.800-186.
3. S. Turner, D. Brown, K. Yiu, R. Housley, and T. Polk, "Elliptic Curve Cryptography Subject Public Key Information," RFC 5480, Internet Engineering Task Force, Mar. 2009, doi: 10.17487/RFC5480.
4. T. Pornin, "Deterministic Usage of the Digital Signature Algorithm (DSA) and Elliptic Curve Digital Signature Algorithm (ECDSA)," RFC 6979, Aug. 2013, doi: 10.17487/RFC6979.
5. J. M. Pollard, "Monte Carlo methods for index computation (mod p)," *Mathematics of Computation*, vol. 32, no. 143, pp. 918-924, July 1978, doi: 10.1090/S0025-5718-1978-0491431-9.
6. J. Breitner and N. Heninger, "Biased Nonce Sense: Lattice Attacks Against Weak ECDSA Signatures in Cryptocurrencies," in *Financial Cryptography and Data Security*, LNCS 11598, pp. 3-20, 2019, doi: 10.1007/978-3-030-32101-7_1.
7. E. Barker, *Recommendation for Key Management: Part 1 - General*, NIST Special Publication 800-57 Part 1 Rev. 5, May 2020, doi: 10.6028/NIST.SP.800-57pt1r5.
8. National Institute of Standards and Technology, *Security Requirements for Cryptographic Modules*, FIPS PUB 140-3, Mar. 2019, doi: 10.6028/NIST.FIPS.140-3.
9. S. Josefsson and I. Liusvaara, "Edwards-Curve Digital Signature Algorithm (EdDSA)," RFC 8032, Internet Research Task Force, Jan. 2017, doi: 10.17487/RFC8032.
10. S. Zhang, D. Lee, R. Wang, D. Zhang, and T. Yao, "SM2 Digital Signature Algorithm for DNSSEC," RFC 9563, Dec. 2024, doi: 10.17487/RFC9563.
11. Standardization Administration of China, *Information Security Technology - Public Key Cryptographic Algorithm SM2 Based on Elliptic Curves - Part 2: Digital Signature Algorithm*, GB/T 32918.2-2016, 2016.
