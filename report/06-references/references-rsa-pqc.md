# Reference Contributions for Section 2.3 (RSA) & Section 2.5 (PQC: ML-DSA)

**Contributor:** Vu Minh Hoang  
**Target Citation Style:** IEEE Style (as required in `E_Voting_Report.pdf` and `report/06-references/GUIDE.md`)

---

### Mapped References

[1] R. L. Rivest, A. Shamir, and L. Adleman, "A method for obtaining digital signatures and public-key cryptosystems," *Communications of the ACM*, vol. 21, no. 2, pp. 120–126, Feb. 1978, doi: 10.1145/359340.359342.  
*Supports:* Foundational definition of RSA algorithm, integer factorization hardness assumption, and textbook RSA signature/encryption duality.

[2] K. Moriarty, B. Kaliski, J. Jonsson, and A. Rusch, "PKCS #1: RSA Cryptography Specifications Version 2.2," Internet Engineering Task Force (IETF), RFC 8017, Nov. 2016, doi: 10.17487/RFC8017.  
*Supports:* Detailed specification of RSA-PSS encoding (EMSA-PSS-ENCODE / EMSA-PSS-VERIFY), MGF1 mask generation, and salt parameter requirements.

[3] National Institute of Standards and Technology (NIST), "Digital Signature Standard (DSS)," Federal Information Processing Standards Publication (FIPS PUB) 186-5, Feb. 2023, doi: 10.6028/NIST.FIPS.186-5.  
*Supports:* U.S. Federal mandate approving RSA-PSS and specifying approved key sizes (2048, 3072, 4096 bits) and hash functions.

[4] M. Bellare and P. Rogaway, "The exact security of digital signatures-how to sign with RSA and Rabin," in *Advances in Cryptology — EUROCRYPT '96*, Berlin, Heidelberg: Springer, 1996, pp. 399–416, doi: 10.1007/3-540-68339-9_34.  
*Supports:* Provable security reduction of RSA-PSS to the standard RSA problem in the Random Oracle Model (ROM).

[5] E. Barker, "Recommendation for Key Management: Part 1 – General," National Institute of Standards and Technology (NIST), Special Publication (SP) 800-57 Part 1 Rev. 5, May 2020, doi: 10.6028/NIST.SP.800-57pt1r5.  
*Supports:* Cryptographic security levels (112 bits for RSA-2048 vs. 128 bits for RSA-4096) and transition timelines post-2030.

[6] A. K. Lenstra and H. W. Lenstra, Jr., Eds., *The Development of the Number Field Sieve*, Lecture Notes in Mathematics, vol. 1554. Berlin, Heidelberg: Springer-Verlag, 1993, doi: 10.1007/BFb0091534.  
*Supports:* Mathematical architecture and sub-exponential complexity of the General Number Field Sieve (GNFS).

[7] T. Kleinjung et al., "Factorization of a 768-bit RSA modulus," in *Advances in Cryptology — CRYPTO 2010*, Berlin, Heidelberg: Springer, 2010, pp. 333–350, doi: 10.1007/978-3-642-14623-7_18.  
*Supports:* Empirical benchmark demonstrating factoring of RSA-768 via GNFS across academic compute clusters.

[8] F. Boudot et al., "Comparing the Difficulty of Factorization and Discrete Logarithm: A 240-Digit Experiment," in *Advances in Cryptology — CRYPTO 2020*, Cham: Springer, 2020, pp. 62–91, doi: 10.1007/978-3-030-56784-1_3.  
*Supports:* Factoring of RSA-829 (250 decimal digits) establishing modern limits of classical computation.

[9] D. Bleichenbacher, "Chosen ciphertext attacks against protocols based on the RSA encryption standard PKCS #1," in *Advances in Cryptology — CRYPTO '98*, Berlin, Heidelberg: Springer, 1998, pp. 1–12, doi: 10.1007/BFb0055716.  
*Supports:* Vulnerabilities of legacy PKCS #1 v1.5 padding and justification for transitioning to RSA-PSS.

[10] D. Boneh, R. A. DeMillo, and R. J. Lipton, "On the importance of eliminating errors in cryptographic computations," *Journal of Cryptology*, vol. 14, no. 2, pp. 101–119, Mar. 2001, doi: 10.1007/s001450010016.  
*Supports:* The Bellcore differential fault analysis attack against RSA with Chinese Remainder Theorem (RSA-CRT) in HSMs.

[11] N. Heninger, Z. Durumeric, E. Wustrow, and J. A. Halderman, "Mining Your Ps and Qs: Detection of widespread weak keys in network devices," in *21st USENIX Security Symposium*, Bellevue, WA, 2012, pp. 205–220.  
*Supports:* Real-world entropy failures leading to shared RSA primes in financial and network devices.

[12] P. W. Shor, "Algorithms for quantum computation: discrete logarithms and factoring," in *Proceedings 35th Annual Symposium on Foundations of Computer Science*, Santa Fe, NM, Nov. 1994, pp. 124–134, doi: 10.1109/SFCS.1994.365700.  
*Supports:* Theoretical proof of polynomial-time factoring of RSA and discrete logarithms on quantum computers via QFT.

[13] C. Gidney and M. Ekerå, "How to factor 2048 bit RSA integers in 8 hours using 20 million noisy qubits," *Quantum*, vol. 5, p. 433, Apr. 2021, doi: 10.22331/q-2021-04-15-433.  
*Supports:* Quantum circuit resource estimation for factoring RSA-2048 under surface code error correction.

[14] National Institute of Standards and Technology (NIST), "Transitioning the Use of Cryptographic Algorithms and Key Lengths," NIST Special Publication (SP) 800-131A Rev. 2, Mar. 2019, doi: 10.6028/NIST.SP.800-131Ar2.  
*Supports:* PCI-DSS and NIST deprecation timelines for legacy RSA configurations in financial systems.

[15] National Institute of Standards and Technology (NIST), "Module-Lattice-Based Digital Signature Standard," Federal Information Processing Standards Publication (FIPS PUB) 204, Aug. 2024, doi: 10.6028/NIST.FIPS.204.  
*Supports:* Official standard for ML-DSA-44, ML-DSA-65, and ML-DSA-87 parameter sets, NTT polynomial rings, and signing/verification algorithms.

[16] V. Lyubashevsky, L. Ducas, E. Kiltz, T. Lepoint, P. Schwabe, G. Seiler, and D. Stehlé, "CRYSTALS-Dilithium: Algorithm Specifications and Supporting Documentation (Version 3.1)," NIST PQC Round 3 Submission, Feb. 2021.  
*Supports:* Core design rationale of Dilithium/ML-DSA, Module-LWE, and Module-SIS parameters.

[17] V. Lyubashevsky, "Fiat-Shamir with aborts: Applications to lattice and factoring-based signatures," in *Advances in Cryptology — ASIACRYPT 2009*, Berlin, Heidelberg: Springer, 2009, pp. 598–616, doi: 10.1007/978-3-642-10366-7_35.  
*Supports:* Theoretical foundation of rejection sampling in lattice signatures to prevent secret-key leakage.

[18] S. Bai, L. Ducas, E. Kiltz, T. Lepoint, V. Lyubashevsky, P. Schwabe, G. Seiler, and D. Stehlé, "New lattice-based cryptographic constructions," *Journal of Mathematical Cryptology*, 2018. (Also BDGL16 Sieve complexity $2^{0.292 \beta}$).  
*Supports:* Core-SVP hardness complexity formulas for BKZ lattice reduction attacks.

[19] L. K. Grover, "A fast quantum mechanical algorithm for database search," in *Proceedings of the Twenty-Eighth Annual ACM Symposium on Theory of Computing (STOC '96)*, Philadelphia, PA, 1996, pp. 212–219, doi: 10.1145/237814.237866.  
*Supports:* Analysis of quantum quadratic speedup against unstructured search and symmetric primitives.

[20] M. Ounsworth, J. Gray, M. Pala, J. Klaußner, and S. Fluhrer, "Composite Module-Lattice-Based Digital Signature Algorithm (ML-DSA) for Use in X.509 Public Key Infrastructure," RFC 10073, Nov. 2026, doi: 10.17487/RFC10073.
*Supports:* Standardized composite ML-DSA combinations with traditional signature algorithms for X.509 PKI, subject to the security and operational constraints stated in the RFC.

