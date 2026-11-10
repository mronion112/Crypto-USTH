# 4.3 Quantum Computer — Theoretical Analysis

## 4.3.1 Scope and Evidence Boundary

This section analyzes how a sufficiently capable fault-tolerant quantum computer would change the security assumptions of the signature schemes in this report. It is not a quantum benchmark and does not claim that the tested laptop, a current cloud quantum service, or any machine evaluated by the project can recover a standard private key. The discussion separates three kinds of evidence:

1. **Algorithmic results**, such as Shor's polynomial-time algorithms for factoring and discrete logarithms [1].
2. **Resource estimates**, which translate an algorithm into logical or physical qubits, gates, error rates, and runtime under explicit architectural assumptions [2], [3].
3. **Migration guidance**, which identifies quantum-vulnerable standards and recommends transition to standardized post-quantum mechanisms [6].

A resource estimate is not a demonstration. Its numerical result depends on the circuit, error-correcting code, physical error rate, connectivity, cycle time, classical control latency, and engineering assumptions. Estimates can fall as algorithms and architectures improve without implying that the required fault-tolerant machine already exists.

## 4.3.2 Shor's Algorithm and the Classical Signature Schemes

Shor showed that integer factorization and discrete logarithms can be solved in polynomial time on an appropriate quantum computer [1]. This changes the asymptotic foundation of RSA and elliptic-curve signatures rather than merely providing a constant-factor speedup.

### RSA-PSS

RSA public keys expose the modulus $n=pq$. A successful quantum factorization of $n$ reveals $p$ and $q$, from which the private exponent can be reconstructed. RSA-PSS padding remains important against classical forgery and encoding failures, but it does not prevent recovery of the private key after factorization. Increasing a conventional RSA modulus delays both classical and quantum attacks but does not remove the polynomial-time quantum algorithm.

Gidney and Ekerå estimated that one specific surface-code construction could factor RSA-2048 in approximately eight hours using about 20 million noisy physical qubits under their stated gate-error, cycle-time, connectivity, and reaction-time assumptions [3]. This is a resource estimate, not a report of an executed RSA-2048 factorization. It is useful because it makes the error-correction and engineering scale explicit, but it should not be treated as a fixed threshold or forecast date.

### ECDSA, Ed25519, and SM2

ECDSA P-256, Ed25519, and SM2 use different curves and signature equations, but each public key has the form of a scalar multiple of a known base point. A quantum algorithm for the elliptic-curve discrete-logarithm problem can recover that scalar and therefore the signing key. Deterministic nonces, strong random-number generation, point validation, and HSM isolation remain necessary classical controls, but they do not make an ECDLP-based scheme post-quantum secure.

Roetteler, Naehrig, Svore, and Lauter constructed and simulated reversible arithmetic circuits for prime-field ECDLP. Their circuit bound uses at most $9n+2\lceil\log_2 n\rceil+10$ logical qubits for an $n$-bit prime field [2]. Substituting $n=256$ gives an upper bound of 2,330 logical qubits for the modeled circuit, before the physical-qubit overhead required for fault tolerance. The paper also reports substantial Toffoli-gate costs and notes that ECC can require fewer quantum resources than RSA at comparable classical security levels. The logical-qubit figure must therefore not be compared directly with a raw physical-qubit count from a hardware announcement.

## 4.3.3 ML-DSA Under the Quantum Threat Model

ML-DSA is based on structured module-lattice problems rather than integer factorization or discrete logarithms. No known application of Shor's algorithm converts its public key into the private signing key. FIPS 204 standardizes ML-DSA-44, ML-DSA-65, and ML-DSA-87 and describes the family as believed to resist adversaries equipped with a large-scale quantum computer [4]. Its design derives from CRYSTALS-Dilithium and relies principally on Module-LWE and Module-SIS assumptions [5].

“Post-quantum” does not mean unconditionally secure. ML-DSA remains exposed to implementation errors, weak randomness, side channels, fault attacks, key theft, protocol misuse, and future cryptanalysis. Quantum search can also affect generic search and hash-related cost models, so parameter selection must follow the standardized construction rather than an informal comparison of raw key lengths. Banks should track errata, validation requirements, implementation guidance, and new cryptanalytic results throughout the algorithm lifecycle.

**Table 4.9. Effect of the quantum threat on the selected signature families**

| Signature family | Classical foundation emphasized in this report | Effect of a sufficiently capable fault-tolerant quantum computer | Migration implication |
|---|---|---|---|
| RSA-PSS | RSA inversion / integer factorization | Shor's factoring algorithm enables recovery of the private key from the public modulus | Replace quantum-vulnerable signing profiles; larger conventional RSA keys are not a long-term solution |
| ECDSA P-256 | ECDLP on NIST P-256 | Quantum ECDLP recovers the private scalar | Migrate certificates, HSM mechanisms, and signed-object profiles to a post-quantum-capable design |
| Ed25519 | ECDLP on edwards25519 | Quantum ECDLP recovers the signing scalar/seed-derived secret | Preserve classical controls while planning replacement for long-lived trust relationships |
| SM2 | ECDLP on the SM2 curve | Quantum ECDLP recovers the private scalar | Coordinate migration with ecosystems that mandate SM2/SM3 and its certificate profiles |
| ML-DSA | Module-LWE and Module-SIS | No known Shor-type polynomial-time key recovery; security remains assumption- and parameter-dependent | Validate implementation maturity, artifact sizes, HSM support, and continuing cryptanalysis |

## 4.3.4 Banking Risk and Migration Priorities

The quantum risk for signatures differs from the familiar “harvest now, decrypt later” confidentiality scenario. Public verification keys are already available by design. If an adversary later obtains a cryptographically relevant quantum computer while a classical certificate remains trusted, the adversary could recover its private key and create new forgeries. Long-retained signatures can also become difficult to defend if the verifier cannot establish that the signature and trusted timestamp predate key compromise. Archival evidence therefore needs trusted time, certificate-status records, algorithm identifiers, validation policy, and renewal or evidence-record procedures rather than a signature value alone.

NISTIR 8547 identifies quantum-vulnerable public-key standards and the post-quantum standards to which products and services need to transition [6]. For a bank, the immediate engineering task is crypto-agility rather than predicting a single “Q-day.” A transition program should:

1. inventory algorithms, certificates, keys, HSM mechanisms, APIs, databases, audit archives, and retention periods;
2. identify signatures whose trust lifetime extends beyond the planned migration window;
3. remove fixed-size assumptions from certificate, message, database, and logging schemas;
4. test ML-DSA key and signature sizes across ISO 20022 envelopes, gateways, HSMs, backup systems, and long-term archives;
5. define algorithm identifiers, downgrade rejection, revocation, rollover, and failure behavior before introducing hybrid or composite signatures;
6. retain classical protections during migration, because a post-quantum algorithm does not correct weak authorization, exposed keys, replay, or malformed message handling; and
7. preserve reproducible test vectors and cross-vendor interoperability evidence.

Hybrid deployment can reduce dependence on either one classical assumption or one relatively new post-quantum implementation, but only if the acceptance rule is explicit. A verifier must know whether both signatures are required, how certificate paths are bound, how downgrade is rejected, and what happens when one component fails. Simply attaching two signatures without a defined policy does not produce a secure migration.

## 4.3.5 Section Conclusion

The theoretical result is clear: a sufficiently capable fault-tolerant quantum computer would invalidate the long-term hardness assumptions of RSA-PSS, ECDSA, Ed25519, and SM2. The practical date and engineering resources remain uncertain and cannot be inferred from the toy CPU/GPU measurements in Section 4.2. ML-DSA changes the mathematical foundation and is standardized for post-quantum use, but its security remains conditional on its assumptions, parameters, implementation, and continuing analysis.

The defensible banking response is therefore staged migration: maintain strong classical controls, inventory quantum-vulnerable dependencies, build format and HSM agility, test ML-DSA in production-representative paths, and preserve trustworthy long-term validation evidence.

## References

1. P. W. Shor, “Algorithms for quantum computation: Discrete logarithms and factoring,” in *Proc. 35th Annu. Symp. Foundations of Computer Science*, pp. 124-134, 1994, doi: 10.1109/SFCS.1994.365700.
2. M. Roetteler, M. Naehrig, K. M. Svore, and K. Lauter, “Quantum resource estimates for computing elliptic curve discrete logarithms,” in *Advances in Cryptology - ASIACRYPT 2017*, pp. 241-270, 2017, doi: 10.1007/978-3-319-70697-9_9.
3. C. Gidney and M. Ekerå, “How to factor 2048 bit RSA integers in 8 hours using 20 million noisy qubits,” *Quantum*, vol. 5, p. 433, 2021, doi: 10.22331/q-2021-04-15-433.
4. National Institute of Standards and Technology, *Module-Lattice-Based Digital Signature Standard*, FIPS PUB 204, Aug. 2024, doi: 10.6028/NIST.FIPS.204.
5. L. Ducas *et al.*, “CRYSTALS-Dilithium: A lattice-based digital signature scheme,” *IACR Trans. Cryptographic Hardware and Embedded Systems*, vol. 2018, no. 1, pp. 238-268, 2018, doi: 10.13154/tches.v2018.i1.238-268.
6. D. Moody, R. Perlner, A. Robinson, A. Regenscheid, and D. Cooper, *Transition to Post-Quantum Cryptography Standards*, NISTIR 8547, Oct. 2026, doi: 10.6028/NIST.IR.8547.
