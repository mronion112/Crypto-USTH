# 2.3 RSA-2048 and RSA-4096 (RSA-PSS)

## 2.3.1 Mathematical Theory

### Scope and standards

In public-key cryptography, the RSA cryptosystem introduced by Rivest, Shamir, and Adleman [1] can be instantiated for either encryption or digital signatures, yet their algebraic workflows, padding constructions, and security reductions differ fundamentally:

- **RSA Encryption:** The sender encrypts a plaintext message $m$ under the recipient's public key $(n, e)$ via $c \equiv m^e \pmod n$. Only the holder of the private exponent $d$ recovers $m \equiv c^d \pmod n$.
- **RSA Digital Signature:** The signer computes a signature $s$ over an encoded message digest using the private key $(n, d)$ via $s \equiv m^d \pmod n$, and any verifier holding $(n, e)$ checks whether $s^e \equiv m \pmod n$.

Unpadded ("textbook") RSA signatures are algebraically malleable because modular exponentiation is multiplicatively homomorphic:

$$\text{If } s_1 \equiv m_1^d \pmod n \quad \text{and} \quad s_2 \equiv m_2^d \pmod n, \quad \text{then } (s_1 \cdot s_2)^e \equiv (m_1 \cdot m_2) \pmod n \tag{2.3.1}$$

An adversary observing two valid signatures can forge a valid signature $s_3 = (s_1 \cdot s_2) \bmod n$ on $m_3 = (m_1 \cdot m_2) \bmod n$ without knowing $d$. Moreover, textbook RSA admits trivial existential forgery: choosing any $s \in \mathbb{Z}_n^*$ and computing $m \equiv s^e \pmod n$ yields a valid message-signature pair $(m, s)$.

To eliminate algebraic malleability and provide a tight security reduction to the RSA problem in the Random Oracle Model (ROM) [4], modern standards mandate **RSA-PSS** (Probabilistic Signature Scheme), defined in PKCS #1 v2.2 (RFC 8017) [2] and approved in NIST FIPS 186-5 [3]. In this report, we analyse RSA-PSS at two standard modulus lengths: **RSA-2048** and **RSA-4096**. Table 2.3.1 summarizes the baseline parameters assumed in our analysis and tested in Section 4.1.

**Table 2.3.1. Assumed RSA-PSS parameters and configurations**

| Symbol / Parameter | Meaning | Assumed Configuration |
|---|---|---|
| $k$ | Modulus bit length $\lceil \log_2 n \rceil$ | 2048 bits (256 B) or 4096 bits (512 B) |
| $p, q$ | Secret prime factors of $n$ | Distinct primes of $k/2$ bits each ($1024$ or $2048$ bits) |
| $e$ | Public verification exponent | $65537 = 2^{16} + 1$ (Fermat prime $F_4$) |
| $H$ | Underlying hash function | SHA-256 ($hLen = 32$ bytes) |
| $\text{MGF}$ | Mask Generation Function | $\text{MGF1-SHA256}$ [2] |
| $sLen$ | Random salt length | $sLen = hLen = 32$ B (FIPS 186-5 standard) or $sLen_{\max} = emLen - hLen - 2$ (Section 4.1 test cases C1, C2) |

### Key generation and CRT representation

Let $k \in \{2048, 4096\}$ denote the target modulus bit length:

1. **Prime generation:** Generate two distinct random primes $p$ and $q$ of $k/2$ bits each such that $|p - q| > 2^{k/2 - 100}$, preventing Fermat factorization when $p$ and $q$ are too close [3].
2. **Modulus computation:** Compute the composite modulus
   $$n = p \cdot q \tag{2.3.2}$$
3. **Totient / Carmichael function:** Compute Euler's totient $\phi(n) = (p - 1)(q - 1)$ or Carmichael's function
   $$\lambda(n) = \operatorname{lcm}(p - 1, q - 1) = \frac{(p - 1)(q - 1)}{\gcd(p - 1, q - 1)} \tag{2.3.3}$$
4. **Public exponent:** Select $e = 65537$ satisfying $\gcd(e, \lambda(n)) = 1$. Its binary representation ($10000000000000001_2$) has Hamming weight 2, allowing modular exponentiation $s^e \bmod n$ in 16 squarings and 1 multiplication while avoiding low-exponent weaknesses.
5. **Private exponent:** Compute the modular inverse of $e$ modulo $\lambda(n)$ via the Extended Euclidean Algorithm:
   $$d \equiv e^{-1} \pmod{\lambda(n)} \iff e \cdot d \equiv 1 \pmod{\lambda(n)} \tag{2.3.4}$$
6. **Chinese Remainder Theorem (CRT) representation:** Hardware Security Modules (HSMs) store the private key as the 5-tuple $(p, q, d_p, d_q, q_{\text{inv}})$ where
   $$d_p = d \bmod (p - 1), \quad d_q = d \bmod (q - 1), \quad q_{\text{inv}} = q^{-1} \bmod p \tag{2.3.5}$$
   Computing modular exponentiation modulo $p$ and $q$ of size $k/2$ bits reduces cubic exponentiation cost $O(k^3)$ by roughly a factor of $4$.

### RSA-PSS signing and verification workflow

RSA-PSS binds a message $M$ to a fresh per-signature random salt via `EMSA-PSS-ENCODE` before applying the private RSA operation [2].

```text
Message M ──► mHash = H(M) ──┐
                             ├──► M' = (0x00)_8 || mHash || salt ──► H = Hash(M')
Random salt (sLen bytes) ────┴───────────────────────────────────────────┐
                                                                         ▼
DB = PS || 0x01 || salt ─────────► maskedDB = DB ⊕ MGF1(H) ──► EM = maskedDB || H || 0xbc
                                                                         │
                                                                         ▼
                                                       s = EM^d mod n (via CRT)
```

#### 1. Signature generation (`EMSA-PSS-ENCODE` + `RSASP1`)

Given message $M$, hash function $H$ with output length $hLen = 32$ bytes, salt length $sLen$, and encoded message length $emLen = \lceil (k - 1)/8 \rceil$:

1. Compute the message digest $mHash = H(M)$.
2. Sample a fresh random salt $salt \stackrel{\$}{\leftarrow} \{0, 1\}^{8 \cdot sLen}$.
3. Form $M' = (\text{0x00})_8 \parallel mHash \parallel salt$ and compute the intermediate hash $H_{\text{pss}} = H(M')$.
4. Construct the data block $DB = PS \parallel \text{0x01} \parallel salt$, where $PS$ consists of $emLen - sLen - hLen - 2$ zero bytes. (In Section 4.1 test cases C1 and C2, `padding.PSS.MAX_LENGTH` sets $sLen = emLen - hLen - 2$, so $PS$ is empty.)
5. Compute the mask $dbMask = \operatorname{MGF1}(H_{\text{pss}}, emLen - hLen - 1)$ and mask the data block:
   $$maskedDB = DB \oplus dbMask \tag{2.3.6}$$
6. Zero the leftmost $8 \cdot emLen - (k - 1)$ bits of $maskedDB$ so that the integer value of the encoded message $EM = maskedDB \parallel H_{\text{pss}} \parallel \text{0xbc}$ satisfies $EM < n$.
7. Compute the signature $s = EM^d \bmod n$ using Garner's CRT recombination:
   $$s_p = EM^{d_p} \bmod p, \quad s_q = EM^{d_q} \bmod q, \quad h = (s_p - s_q) \cdot q_{\text{inv}} \bmod p, \quad s = s_q + h \cdot q \tag{2.3.7}$$
   The resulting signature $s$ is $256$ bytes for RSA-2048 and $512$ bytes for RSA-4096 (verified in test cases C1 and C2 of Section 4.1).

#### 2. Signature verification (`RSAVP1` + `EMSA-PSS-VERIFY`)

Given message $M$, signature $s$, and public key $(n, e)$:

1. Check that $|s| = k/8$ bytes and $0 \le s < n$; otherwise reject.
2. Compute the encoded message representative:
   $$EM = s^e \bmod n \tag{2.3.8}$$
3. Check that the trailing byte of $EM$ equals $\text{0xbc}$, then parse $EM = maskedDB \parallel H_{\text{pss}} \parallel \text{0xbc}$.
4. Verify that the top $8 \cdot emLen - (k - 1)$ bits of $maskedDB$ are zero.
5. Recover $DB = maskedDB \oplus \operatorname{MGF1}(H_{\text{pss}}, emLen - hLen - 1)$ and clear its top $8 \cdot emLen - (k - 1)$ bits.
6. Verify that $DB$ starts with $emLen - sLen - hLen - 2$ zero bytes followed by $\text{0x01}$, and extract the trailing $sLen$ bytes as $salt$.
7. Recompute $H' = H((\text{0x00})_8 \parallel H(M) \parallel salt)$ and accept the signature if and only if $H' = H_{\text{pss}}$.

### Security assumptions and attack vectors

#### 1. Integer Factorization Problem (IFP) and key-size scaling

The security of RSA-PSS reduces in the Random Oracle Model to the **RSA problem** (inverting $x \mapsto x^e \bmod n$), which in turn relies on the hardness of the **Integer Factorization Problem (IFP)**: given a composite modulus $n = p \cdot q$, recover its prime factors $p$ and $q$ [1], [4]. Once $p$ and $q$ are known, an attacker immediately computes $\lambda(n) = \operatorname{lcm}(p-1, q-1)$ and recovers $d \equiv e^{-1} \pmod{\lambda(n)}$.

Per NIST SP 800-57 Part 1 Rev. 5 [5]:

- **RSA-2048 ($k = 2048$):** Offers approximately **112 bits of classical security** and is approved through 2030.
- **RSA-4096 ($k = 4096$):** Offers approximately **128–140 bits of classical security**, providing a wider classical safety margin beyond 2030 at the cost of substantially slower signing.

#### 2. General Number Field Sieve (GNFS) vs. trial division and Pollard's rho

A critical distinction must be drawn between three algorithmic regimes for factoring $n = p \cdot q$:

1. **Trial division (exhaustive divisor search):** Tests candidate divisors $d \le \lfloor\sqrt{n}\rfloor$ (e.g., filtering by $6k \pm 1$ as in Section 4.2.1). Its complexity is exponential in the bit length $k$:
   $$T_{\text{trial}}(k) = O(\sqrt{n}) = O(2^{k/2}) \tag{2.3.9}$$
2. **Pollard's rho / Brent cycle finding:** Uses a pseudorandom iteration $x_{i+1} = (x_i^2 + c) \bmod n$ and $\gcd(|x_i - x_j|, n)$ to find a factor $p$ in expected time $O(p^{1/2}) = O(n^{1/4})$ [15]:
   $$T_{\rho}(k) = O(n^{1/4}) = O(2^{k/4}) \tag{2.3.10}$$
3. **General Number Field Sieve (GNFS):** The asymptotically fastest known classical algorithm for factoring large balanced RSA moduli ($k \ge 400$ bits) [6]. GNFS operates in **sub-exponential** heuristic time:
   $$L_n\left[\frac{1}{3}, \left(\frac{64}{9}\right)^{1/3}\right] = \exp\left(\left(\left(\frac{64}{9}\right)^{1/3} + o(1)\right)(\ln n)^{1/3}(\ln \ln n)^{2/3}\right) \tag{2.3.11}$$
   where $(64/9)^{1/3} \approx 1.923$.

GNFS does **not** search the key space. Instead, it proceeds in four stages: (i) selecting irreducible polynomials $f_1(x), f_2(x) \in \mathbb{Z}[x]$ sharing a root $m \pmod n$; (ii) distributed relation sieving over pairs $(a, b)$ to find smooth values over rational and algebraic factor bases; (iii) sparse linear algebra over $\mathbb{F}_2$ (via Block Lanczos or Block Wiedemann) to find a null-space dependency; and (iv) algebraic square-root extraction to construct $x^2 \equiv y^2 \pmod n$, yielding a non-trivial factor via $\gcd(x - y, n)$.

Historically, GNFS factored **RSA-768** in 2009 (~1,500 core-years) [7] and **RSA-829** in 2020 (~2,700 core-years) [8]. Factoring RSA-2048 via GNFS requires roughly $2^{112}$ operations, remaining infeasible for classical computers.

#### 3. Padding, key-generation, and implementation vulnerabilities

- **Deterministic PKCS #1 v1.5 flaws vs. RSA-PSS:** Legacy PKCS #1 v1.5 signature verification is vulnerable to Bleichenbacher's low-exponent ($e = 3$) cube-root forgery when ASN.1 parsers fail to enforce right-justification of the digest structure [9]. RSA-PSS mitigates this class of flaws by randomizing every signature with a salt and masking the entire data block via $\text{MGF1}$.
- **Bellcore CRT fault injection:** If a hardware glitch (voltage spike or electromagnetic fault) corrupts only one branch of the CRT exponentiation in Eq. (2.3.7)—say $\tilde{s}_p \ne s_p$ while $s_q$ is intact—the faulty signature $\tilde{s}$ satisfies $\tilde{s}^e \equiv EM \pmod q$ but $\tilde{s}^e \not\equiv EM \pmod p$ [10]. A single GCD reveals the secret prime factor from one faulty signature:
  $$q = \gcd(\tilde{s}^e - EM \bmod n,\, n) \tag{2.3.12}$$
  Consequently, banking HSMs must verify $\tilde{s}^e \bmod n = EM$ internally before outputting any signature.
- **Low-entropy prime generation:** When embedded devices generate RSA keys with insufficient entropy after boot, different moduli $n_1 = p q_1$ and $n_2 = p q_2$ may share a prime factor $p$, allowing instant recovery via $p = \gcd(n_1, n_2)$ in polynomial time [11].
- **Timing side channels:** Non-constant-time modular exponentiation leaks bits of $d_p, d_q$. Countermeasures include multiplicative RSA blinding ($EM' = EM \cdot r^e \bmod n$, followed by $s = (EM')^d \cdot r^{-1} \bmod n$) and constant-time Montgomery ladders.

#### 4. Quantum threat (Shor's algorithm)

On a fault-tolerant quantum computer, Shor's algorithm [12] reduces integer factorization to order-finding in $\mathbb{Z}_n^*$ using the Quantum Fourier Transform (QFT), running in polynomial time $\tilde{O}(k^3)$. Gidney and Ekerå [13] estimate that factoring RSA-2048 requires roughly $4096$ logical qubits (or ~20 million noisy physical qubits over 8 hours with surface codes). Detailed quantum resource comparisons across all studied schemes are given in **Section 4.3**.

---

## 2.3.2 Application in Banking System

### Comparison of RSA-2048 and RSA-4096

Table 2.3.2 compares RSA-2048 and RSA-4096 across cryptographic security, key/signature footprints, and operational trade-offs in banking environments. (As emphasized in the project guidelines, security levels reflect cryptanalytic GNFS complexity rather than single-machine timing measurements.)

**Table 2.3.2. Comparison of RSA-2048 and RSA-4096 for banking deployment**

| Criterion | RSA-2048 (RSA-PSS) | RSA-4096 (RSA-PSS) | Banking Deployment Implications |
|---|---|---|---|
| **Modulus size ($k$)** | 2048 bits | 4096 bits | Doubling $k$ increases signing cost by roughly $2^3 = 8\times$ |
| **Classical security (NIST SP 800-57)** | $\approx 112$ bits | $\approx 128\text{–}140$ bits | RSA-2048 acceptable until ~2030; RSA-4096 for longer-lived roots |
| **Quantum security (Shor)** | Broken ($\tilde{O}(k^3)$) | Broken ($\tilde{O}(k^3)$) | Doubling key size does not protect against a CRQC |
| **Public key size (DER SubjectPublicKeyInfo)** | $\approx 294$ B | $\approx 550$ B | Larger certificate chains in TLS and ISO 20022 envelopes |
| **Private key size (CRT 5-tuple)** | $\approx 1190$ B | $\approx 2350$ B | Consumes more tamper-protected NVRAM slots inside HSMs |
| **Signature size ($s$)** | **256 B** (Test C1) | **512 B** (Test C2) | $4\times\text{–}8\times$ larger than 64 B Ed25519/ECDSA signatures |
| **Signing cost scaling** | Baseline ($1\times$, $O(k^3)$) | $\approx 6\times\text{–}8\times$ slower | High-throughput signing endpoints suffer TPS reduction with RSA-4096 |
| **Verification cost ($e = 65537$)** | Fast ($1\times$, $O(k^2)$) | $\approx 3.5\times\text{–}4\times$ slower than 2048 | Both verify quickly because $e = 65537$ requires only 17 multiplications |
| **Typical banking role** | Interbank XML signing, TLS leaf certs, EMV cards | Offline Root CAs, Intermediate CAs, long-term audit archives | Tiered deployment separates high-frequency signing from long-term trust anchors |

### Role and integration in banking workflows

In banking systems—where RSA-PSS serves as a primary classical baseline alongside ECDSA, Ed25519, and SM2—its deployment is governed by asymmetric performance characteristics (very fast verification with $e = 65537$, slower private-key signing) and wide legacy support:

1. **Hierarchical PKI and Certificate Authorities:** Banking PKIs (including SWIFT PKI, national clearing houses, and payment card root hierarchies) commonly use **RSA-4096** for offline Root and Issuing CAs, where signatures are generated infrequently (signing intermediate certificates or Certificate Revocation Lists) and verification speed on client terminals is prioritized.
2. **Interbank messaging and corporate treasury (ISO 20022 / EBICS):** Financial messaging standards such as ISO 20022 XML Digital Signatures (`rsa-sha256` / `sha256-rsa-MGF1`) and EBICS (Electronic Banking Internet Communication Standard, authentication version A006) mandate **RSA-PSS** with SHA-256 to sign payment batches and treasury instructions [14].
3. **EMV chip card authentication (SDA / DDA / CDA):** In contact and contactless payment cards, POS terminals must verify the Issuer and IC Card certificates within strict millisecond windows (~100–300 ms). Because RSA verification with $e = 65537$ requires only 16 modular squarings and 1 multiplication, low-power POS terminals can verify RSA-2048 certificates rapidly.
4. **Operational controls and HSM requirements:**
   - **Strict PSS parameter enforcement:** Verifiers must reject legacy PKCS #1 v1.5 signatures on endpoints configured for RSA-PSS and explicitly pin the hash algorithm (SHA-256/384/512), MGF1 hash, and acceptable salt length ($sLen = hLen$) [2], [14].
   - **HSM key protection and fault checking:** Private signing keys must be generated and retained inside FIPS 140-3 Level 3 HSMs, with mandatory post-CRT verification ($s^e \bmod n = EM$) enabled to prevent Bellcore fault attacks [10].

### Methodological note for Section 4 benchmarks

In **Section 4.1** (test cases C1 and C2), RSA-PSS correctness is validated at full **2048-bit (256 B)** and **4096-bit (512 B)** key sizes using Python `cryptography 50.0.2`. In **Section 4.2** (test cases K1, K3, G1), CPU and GPU key-recovery experiments operate strictly on **32, 40, 48, and 56-bit toy RSA moduli**:

- The CPU/GPU brute-force benchmark tests candidate divisors $6k \pm 1 \le \lfloor\sqrt{n}\rfloor$ with complexity $O(2^{k/2})$, while the CPU Pollard-Brent rho benchmark recovers a factor in $O(2^{k/4})$ steps (reducing steps by $27.5\times\text{–}2505\times$ on 32–56 bit moduli).
- Neither trial division nor Pollard's rho models **GNFS** ($L_n[1/3, 1.923]$), which is the relevant classical attack for RSA-2048 and RSA-4096. Conversely, GNFS has large polynomial-selection and linear-algebra overhead that makes it unsuitable for 32–56 bit toy moduli (directly answering Review Question 5 in Section 4.2.6).
- Therefore, toy RSA recovery times measured on CPU/GPU in Section 4.2 illustrate parallel search throughput and algorithmic scaling ($O(2^{k/2})$ vs. $O(2^{k/4})$), and **must never be extrapolated** to estimate the time required to break RSA-2048 or RSA-4096.

## References

1. R. L. Rivest, A. Shamir, and L. Adleman, "A method for obtaining digital signatures and public-key cryptosystems," *Communications of the ACM*, vol. 21, no. 2, pp. 120–126, Feb. 1978, doi: 10.1145/359340.359342.
2. K. Moriarty, B. Kaliski, J. Jonsson, and A. Rusch, "PKCS #1: RSA Cryptography Specifications Version 2.2," RFC 8017, Internet Engineering Task Force, Nov. 2016, doi: 10.17487/RFC8017.
3. National Institute of Standards and Technology, *Digital Signature Standard (DSS)*, FIPS PUB 186-5, U.S. Department of Commerce, Gaithersburg, MD, USA, Feb. 2023, doi: 10.6028/NIST.FIPS.186-5.
4. M. Bellare and P. Rogaway, "The exact security of digital signatures—How to sign with RSA and Rabin," in *Advances in Cryptology — EUROCRYPT '96*, Berlin, Heidelberg: Springer, 1996, pp. 399–416, doi: 10.1007/3-540-68339-9_34.
5. E. Barker, *Recommendation for Key Management: Part 1 – General*, NIST Special Publication 800-57 Part 1 Rev. 5, May 2020, doi: 10.6028/NIST.SP.800-57pt1r5.
6. A. K. Lenstra, H. W. Lenstra, Jr., M. S. Manasse, and J. M. Pollard, "The number field sieve," in *Proc. 22nd Annu. ACM Symp. Theory of Computing (STOC '90)*, 1990, pp. 564–572, doi: 10.1145/100216.100295.
7. T. Kleinjung *et al.*, "Factorization of a 768-bit RSA modulus," in *Advances in Cryptology — CRYPTO 2010*, Berlin, Heidelberg: Springer, 2010, pp. 333–350, doi: 10.1007/978-3-642-14623-7_18.
8. F. Boudot *et al.*, "Comparing the difficulty of factorization and discrete logarithm: A 240-digit experiment," in *Advances in Cryptology — CRYPTO 2020*, Cham: Springer, 2020, pp. 62–91, doi: 10.1007/978-3-030-56784-1_3.
9. U. Kühn, "Side-channel attacks on textbook RSA and PKCS #1 v1.5," and D. Bleichenbacher, "Forging PKCS #1 v1.5 RSA signatures with $e=3$," *CRYPTO 2006 Rump Session*, 2006.
10. D. Boneh, R. A. DeMillo, and R. J. Lipton, "On the importance of eliminating errors in cryptographic computations," *Journal of Cryptology*, vol. 14, no. 2, pp. 101–119, Mar. 2001, doi: 10.1007/s001450010016.
11. N. Heninger, Z. Durumeric, E. Wustrow, and J. A. Halderman, "Mining your Ps and Qs: Detection of widespread weak keys in network devices," in *Proc. 21st USENIX Security Symp.*, Bellevue, WA, USA, 2012, pp. 205–220.
12. P. W. Shor, "Algorithms for quantum computation: Discrete logarithms and factoring," in *Proc. 35th Annu. Symp. Foundations of Computer Science (FOCS)*, Santa Fe, NM, USA, Nov. 1994, pp. 124–134, doi: 10.1109/SFCS.1994.365700.
13. C. Gidney and M. Ekerå, "How to factor 2048 bit RSA integers in 8 hours using 20 million noisy qubits," *Quantum*, vol. 5, p. 433, Apr. 2021, doi: 10.22331/q-2021-04-15-433.
14. National Institute of Standards and Technology, *Transitioning the Use of Cryptographic Algorithms and Key Lengths*, NIST Special Publication 800-131A Rev. 2, Mar. 2019, doi: 10.6028/NIST.SP.800-131Ar2.
15. R. P. Brent, "An improved Monte Carlo factorization algorithm," *BIT Numerical Mathematics*, vol. 20, no. 2, pp. 176–184, 1980, doi: 10.1007/BF01933190.
