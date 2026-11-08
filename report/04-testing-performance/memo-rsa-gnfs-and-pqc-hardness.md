# Technical Memorandum: Mathematical Distinctions for Section 4 Benchmarks
**From:** Vu Minh Hoang (Section 2.3 RSA & Section 2.5 ML-DSA)  
**To:** Nguyen Quang Vu Hoang & Nguyen Hoang Anh (Section 4 Testing & Performance Analysis)  
**Subject:** Methodological Boundaries: Differentiating Brute-Force Key Recovery from GNFS, Pollard's Rho, and Lattice Reduction

---

### 1. Objective of this Memorandum
This memorandum provides the theoretical cryptography foundations required to prevent misinterpretations in Section 4.2 (*CPU/GPU Key-Recovery Benchmark*) and Section 4.3 (*Quantum Computer Theoretical Analysis*).

As established in the project guidelines and cryptographic literature:
1. **CPU/GPU Key-Recovery experiments conducted in Section 4 must be strictly confined to toy parameters / reduced key-spaces.**
2. **Exhaustive search (brute-force) on toy keys is NOT equivalent to the algebraic factoring algorithms (GNFS) or discrete log algorithms (Pollard's rho) used against real-world schemes.**
3. **ML-DSA post-quantum security cannot be tested or extrapolated via naive key-search benchmarks.**

---

### 2. RSA: Why Brute Force $\ne$ GNFS (General Number Field Sieve)

#### The Brute-Force Toy Benchmark Model
When running a CPU or GPU key-recovery test against a toy RSA modulus (e.g., 32-bit to 64-bit composite $n = pq$), the program typically tests trial divisors $d = 3, 5, 7, \dots, \lfloor\sqrt{n}\rfloor$.
* **Time Complexity:** $O(\sqrt{n}) = O(2^{k/2})$ operations.
* **Extrapolation Fallacy:** If a 32-bit toy key is recovered in 0.005 seconds on a GPU, students might be tempted to claim:
  $$\text{Time}(\text{RSA-2048}) \approx 0.005 \times \frac{2^{1024}}{2^{16}} \text{ seconds}$$
  **This claim is mathematically meaningless and must NOT appear in the report.**

#### The Actual Attack Model: GNFS
No competent adversary factors a real RSA modulus using trial division or key search. The actual state-of-the-art classical attack is the **General Number Field Sieve (GNFS)**:
* **Asymptotic Heuristic Complexity:**
  $$L_n\left[\frac{1}{3}, \left(\frac{64}{9}\right)^{1/3}\right] = \exp\left( (1.923 + o(1)) (\ln n)^{1/3} (\ln \ln n)^{2/3} \right)$$
* **Sub-exponential vs. Exponential:** GNFS runs in *sub-exponential* time, which grows substantially slower than $O(2^{k/2})$ but far faster than polynomial time $O(k^c)$.
* **Algorithmic Reality:** GNFS operates through algebraic number fields, polynomial sieving, and massive sparse linear algebra over $\text{GF}(2)$ (Block Lanczos/Wiedemann) to find congruent squares $x^2 \equiv y^2 \pmod n$.
* **Section 4 Requirement:** Section 4 must explicitly state that the GPU benchmark measures **search throughput across a truncated key space**, which serves as an empirical demonstration of search parallelism, **not an attack against the mathematical foundations of RSA-2048 or RSA-4096**.

---

### 3. Elliptic Curves (ECDSA / Ed25519 / SM2): Brute Force vs. Pollard's Rho
* **Toy Brute Force:** Tests private scalar $k \in [1, 2^B]$ by computing $k \cdot G$ sequentially until matching public key $Q$. Complexity: $O(2^B)$ scalar multiplications.
* **Pollard's Rho for ECDLP:** Uses pseudo-random walks and cycle finding (Floyd's or Brent's cycle detection) to find collisions in the curve group. Complexity: $O(\sqrt{\pi \cdot n / 2}) \approx O(2^{b/2})$.
* **Section 4 Requirement:** Distinguish between naive sequential key-space search and collision algorithms. Emphasize that while toy curve keys can be broken on a laptop, 256-bit curves (Ed25519, SM2, secp256k1) require $\sim 2^{128}$ operations, remaining impenetrable classically.

---

### 4. Post-Quantum Cryptography (ML-DSA): Why Toy Search Does Not Apply
* **No Linear Scalar Key:** In ML-DSA-65, the secret key consists of polynomial vectors $\mathbf{s}_1 \in R_q^5, \mathbf{s}_2 \in R_q^6$ with degree $n = 256$ and coefficients bounded by $\eta = 4$.
* **Secret Combinations:** The number of candidate secret vectors is:
  $$(2\eta + 1)^{256 \times (k + l)} = 9^{256 \times 11} = 9^{2816} \approx 10^{2687}$$
  Testing secret polynomials by brute-force search is physically impossible even on a microscopic toy subset.
* **Actual Cryptanalysis Model:** Attacks against Module-LWE and Module-SIS rely on **Lattice Basis Reduction (BKZ-2.0)**:
  * Primal attack (embedding into uSVP) and Dual attack (SIS on dual lattice).
  * Complexity is governed by the **Core-SVP hardness**: $T \approx 2^{0.292 \cdot \beta}$ (classical) or $T \approx 0.265 \cdot \beta$ (quantum), where block size $\beta \ge 600$ for ML-DSA-65.
* **Section 4 Requirement:** Do NOT attempt to run a brute-force benchmark on ML-DSA. In Section 4.3, contrast the polynomial vulnerability of RSA/ECC under Shor's algorithm ($O(k^3)$) with the sub-exponential / exponential resistance of lattice reduction algorithms under both classical and quantum computing paradigms.

---

### 5. Recommended Table for Section 4 to Summarize Hardness and Attacks

| Algorithm | Toy Benchmark Metric (Sec. 4.2) | Real-World Classical Attack (State-of-the-Art) | Classical Attack Complexity | Quantum Attack (Shor / Grover) | Quantum Attack Complexity (Sec. 4.3) |
|---|---|---|---|---|---|
| **RSA-2048 / 4096** | Trial division / modular exponentiation | General Number Field Sieve (GNFS) | Sub-exponential $L_n[1/3, 1.923]$ ($\approx 2^{112}$ for RSA-2048) | **Shor's Algorithm** (Period-finding via QFT) | Polynomial $\tilde{O}(k^3)$ ($\approx 4096$ logical qubits) |
| **ECDSA / SM2** | Sequential scalar walk on toy curve | Pollard's Rho algorithm | Square-root $O(\sqrt{\#E}) \approx 2^{128}$ | **Shor's Algorithm** (Discrete log via QFT) | Polynomial $\tilde{O}(k^3)$ ($\approx 2330$ logical qubits) |
| **Ed25519** | Exhaustive search on small scalar bits | Pollard's Rho algorithm | Square-root $O(\sqrt{\#E}) \approx 2^{128}$ | **Shor's Algorithm** (Discrete log via QFT) | Polynomial $\tilde{O}(k^3)$ ($\approx 2330$ logical qubits) |
| **ML-DSA-65** | N/A (Not amenable to toy scalar search) | Lattice Basis Reduction (BKZ-2.0 / Sieving) | Exponential $2^{0.292 \beta}$ ($\ge 2^{192}$ for Category 3) | **Grover's Algorithm** / Quantum Sieving ($2^{0.265 \beta}$) | **Exponential** ($\ge 2^{128}$ quantum security) |

