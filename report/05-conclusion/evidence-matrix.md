# Evidence Matrix for Section 5

This matrix limits the conclusion to claims that can be traced to Sections 2-4. It is a working control document for drafting Section 5 and is not a substitute for the report body.

## 1. Evidence availability

| Report input | Status | Evidence usable in Section 5 | Restriction |
|---|---|---|---|
| Section 2.1 Ed25519 | Draft available | Mathematical foundation, deterministic nonce design, implementation risks, 64-byte signature, banking integration constraints | Do not infer measured signing or verification performance; Section 4 contains correctness only |
| Section 2.2 SM2 | Normalized Markdown draft available; source DOCX preserved | SM2/SM3 workflow, signer-ID binding, ECDLP basis, nonce and validation risks, banking integration requirements | No standalone performance benchmark; the observed DER size is one correctness-test output |
| Section 2.3 RSA-PSS | Draft available | RSA-PSS configuration, RSA-2048/4096 trade-offs, GNFS boundary, CRT/fault risks, legacy integration | Do not extrapolate toy factorization time to standard RSA keys |
| Section 2.4 ECDSA | Draft available | P-256/SHA-256 theory, nonce-reuse algebra, public-key validation, HSM and certificate controls | The 72-byte DER signature is an observed instance, not a fixed ECDSA signature length |
| Section 2.5 ML-DSA | Draft available | Module-lattice assumptions, parameter sizes, implementation and migration trade-offs | “Post-quantum” is conditional on current cryptanalysis, not an unconditional security guarantee |
| Section 3 banking workflow | Draft available | Canonicalization, authorization, protected signing, certificate validation, replay checks, business/fraud gates, audit paths | A valid signature alone does not authorize or settle a transaction |
| Section 4.1 test setup and correctness | Draft and test-case table available | C1-C8 signature generation/verification, observed sizes; I1 nonce-reuse recovery; K1-K3/G1 test scope | Correctness tests are not performance benchmarks; keys are ephemeral test keys |
| Section 4.2 CPU/GPU recovery | Draft, raw data, summaries, and figures available | Toy-key scaling, CPU/GPU comparison, Pollard rho comparison, reproducibility limits | Results apply only to the measured toy sizes, machine, code paths, and configurations |
| Section 4.3 quantum analysis | **Draft missing; GUIDE only** | The intended scope and review constraints are known | Do not finalize cross-algorithm quantum conclusions, resource estimates, or present-capability claims until `4.3-quantum-theoretical-analysis/draft.md` exists |

## 2. Research-question traceability

| Research question from Section 1 | Supporting evidence | Conclusion permitted | Required qualification |
|---|---|---|---|
| RQ1: Security foundations | Sections 2.1-2.5 define ECDLP, integer-factorization/RSA, and module-lattice foundations; Section 4.2 distinguishes brute force from Pollard rho and GNFS | The schemes rely on different hard problems and cannot be ranked from key length or toy timing alone | Final quantum comparison remains provisional until Section 4.3 is delivered |
| RQ2: Implementation and key management | Sections 2.1-2.5 cover nonce, validation, side-channel, fault, randomness, and lifecycle risks; Test I1 recovers an ECDSA key after nonce reuse | Implementation and key custody can defeat a sound mathematical scheme without solving its hard problem | I1 demonstrates one constructed failure mode; it is not evidence of a P-256 ECDLP break |
| RQ3: Banking integration | Section 3 defines canonical data, authorization gates, HSM signing, PKI checks, anti-replay state, fraud/business checks, and acceptance/rejection audit paths | A banking signature must be embedded in a layered control workflow | A valid signature supplies integrity and signer-key evidence, not confidentiality, human identity, authorization, freshness, or business validity by itself |
| RQ4: Experimental interpretation | Section 4.1 records scope and environment; Section 4.2 reports CPU/GPU/rho results for RSA 32-56 bit and ECDLP 18/22/26 bit | Parallel hardware can accelerate some exhaustive searches, while algorithm choice and implementation strategy can dominate hardware choice | No reported measurement supports recovery of RSA-2048/4096 or a standard 256-bit elliptic-curve private key |
| RQ5: Migration and crypto-agility | Section 2.5 discusses ML-DSA sizes and staged migration; classical sections note Shor exposure; Section 3 provides algorithm-suite boundaries | Crypto-agility, inventory, HSM readiness, data-format capacity, and staged transition are justified future-work themes | Do not state a migration deadline or quantum resource threshold until Section 4.3 supplies sourced analysis |

## 3. Cross-algorithm evidence

| Scheme and fixed configuration | Security basis in Section 2 | Correctness evidence in Section 4.1 | Main implementation risk evidenced in report | Banking position supported by current drafts |
|---|---|---|---|---|
| Ed25519 | ECDLP in the prime-order subgroup of edwards25519; approximately 128-bit classical design target | C4 passed; fixed 64-byte signature | Seed protection, correct point/scalar validation, side-channel resistance; deterministic nonce avoids dependence on fresh per-signature randomness but does not remove all implementation risk | Compact and suitable for new high-volume components, subject to certified HSM, policy, and legacy-stack support |
| SM2 with SM3 and configured signer ID | ECDLP on SM2P256V1; approximately 128-bit classical security in the draft | C5 passed; 72-byte DER signature observed | Fresh secret nonce, consistent signer ID, public-key validation, side-channel protection | Relevant where SM2 standards and identity binding are required; requires agreement on ID, curve, SM3, encoding, certificates, and key controls |
| RSA-PSS-2048/SHA-256/MGF1-SHA-256 | RSA problem and integer factorization; GNFS is the relevant large-modulus classical factoring method | C1 passed; fixed 256-byte signature | Prime generation, strict PSS parameters, CRT fault protection, HSM custody | Strong legacy interoperability and fast public verification; larger footprint than ECC and no protection from Shor by merely increasing RSA size |
| RSA-PSS-4096/SHA-256/MGF1-SHA-256 | Same foundation with a larger classical margin than RSA-2048 | C2 passed; fixed 512-byte signature | Same as RSA-2048, with greater signing/storage cost | Best supported for compatibility-sensitive or relatively infrequent signing where its cost is acceptable; not a post-quantum migration |
| ECDSA P-256 with SHA-256 | ECDLP in the P-256 subgroup; approximately 128-bit classical security | C3 passed; 72-byte DER signature observed | Nonce reuse/bias/leakage, invalid-point handling, side channels, key lifecycle; I1 recovered the private key after deliberate nonce reuse | Mature PKI/HSM interoperability, but deployment must enforce nonce safety, validation, certificate policy, replay control, and audit separation |
| ML-DSA-44/65/87 | Module-LWE and Module-SIS under current classical and quantum lattice cryptanalysis | C6-C8 passed; fixed 2420/3309/4627-byte signatures | Randomness/implementation quality, parameter and context handling, side channels, young ecosystem | Post-quantum candidate with substantially larger records and new HSM/protocol requirements; staged or hybrid transition is more defensible than a blind field replacement |

## 4. Experimental findings eligible for the conclusion

| Evidence | Measured result | What it supports | What it does not support |
|---|---|---|---|
| Correctness C1-C8 | All selected library/OpenSSL configurations signed and verified; observed signature sizes were 256, 512, 72, 64, 72, 2420, 3309, and 4627 bytes | The selected configurations were executable in the test environment and their tested outputs verified | Relative performance, production readiness, certification, or universal DER size |
| ECDSA nonce reuse I1 | The constructed P-256 case recovered the exact private key algebraically and independently verified it | Nonce uniqueness/secrecy is a critical ECDSA control | A brute-force or ECDLP break of P-256 |
| CPU scaling | At sufficiently large toy sizes, 12 logical threads produced about 4.88-5.82x speedup over one thread | Parallelism helps when work is large enough to amortize overhead | Linear 12x scaling, or a standard-key recovery estimate |
| GPU RSA exhaustive search | GPU was 3.69-9.13x faster than 12-thread CPU for full scan and 3.00-7.63x for recovery on RSA 32-56-bit toy cases | The measured GPU mapping accelerated this simple independent-candidate workload | GNFS performance or feasibility of factoring RSA-2048/4096 |
| GPU ECDLP exhaustive search | GPU was faster at 18 bits but only 0.46-0.48x the 12-thread CPU time ratio at 22-26 bits, so it was slower there | Hardware advantage depends on the algorithm and implementation mapping | A general claim that GPUs are faster or slower for elliptic-curve cryptanalysis |
| Pollard rho comparison | CPU rho reduced steps by 27.5-2505.2x and time by 9.05-448.81x against one-thread brute force across the measured toy cases | Algorithm selection can dominate brute-force hardware acceleration | Standard-parameter attack cost; GPU rho performance; GNFS performance |

## 5. Claims reserved until Section 4.3 is available

The following points must remain placeholders in the final conclusion:

1. Any cross-scheme table that labels a scheme “broken by quantum computers” without stating the fault-tolerant, cryptographically relevant machine condition.
2. Any numerical logical-qubit, physical-qubit, runtime, error-rate, or calendar prediction.
3. Any statement about what current quantum hardware can recover in practice.
4. Any definitive comparison between Shor's effect on RSA/ECC and the best known quantum attacks on ML-DSA.
5. Any migration deadline derived from quantum-resource forecasts.

Until the missing draft is supplied, Section 5 may state only the qualitative, already-supported result: Shor's algorithm is a theoretical threat to the factoring and elliptic-curve discrete-logarithm assumptions, while ML-DSA is designed around different assumptions believed to resist known large-scale quantum attacks; neither statement is an unconditional guarantee or evidence of a present standard-key break.

## 6. Drafting rule for Section 5

Every paragraph in Section 5 must answer at least one research question and map to an evidence row above. If a statement introduces a new number, deployment fact, attack result, product capability, deadline, or prediction that is absent from Sections 2-4, it must be moved to the relevant body section and sourced before it can appear in the conclusion.
