# 2.2 SM2

This section examines the SM2 digital signature algorithm defined in the GB/T 32918 series. SM2 is an elliptic-curve public-key system that also includes encryption and key-exchange mechanisms, but the scope here is limited to digital signatures. The selected signature configuration uses the standard SM2 curve, the SM3 hash function, and an explicitly configured signer identity [1]-[4].

## 2.2.1 Mathematical Theory

### Domain parameters and key generation

SM2 operates on a short Weierstrass elliptic curve over a prime field $\mathbb{F}_p$:

$$
y^2 \equiv x^3 + ax + b \pmod p. \tag{2.2.1}
$$

The curve must be non-singular:

$$
4a^3 + 27b^2 \not\equiv 0 \pmod p. \tag{2.2.2}
$$

The domain parameters are the field prime $p$, curve coefficients $a$ and $b$, base point $G=(x_G,y_G)$, prime order $n$ of $G$, and cofactor $h$ [2]. This report uses the standardized 256-bit SM2 curve commonly identified as `sm2p256v1`. Its intended generic classical security level is approximately 128 bits because the best generic attacks on a prime-order elliptic-curve subgroup require work proportional to $\sqrt{n}$.

The signer chooses a private key

$$
d_A \in [1,n-2]
$$

using an approved cryptographic random-number generator and computes the public key

$$
P_A=[d_A]G. \tag{2.2.3}
$$

The private key must remain secret. The public key may be distributed through a certificate or another authenticated mechanism, but the verifier must also know the curve, hash, signer identity, and signature encoding expected by the protocol.

### Signer identity and message digest

A distinguishing feature of SM2 is that the signer identity and public-key context are hashed into an auxiliary value $Z_A$ before the message is signed:

$$
Z_A=\operatorname{SM3}(ENTL_A \parallel ID_A \parallel a \parallel b \parallel x_G \parallel y_G \parallel x_A \parallel y_A). \tag{2.2.4}
$$

Here, $ID_A$ is the signer identifier, $ENTL_A$ is its bit length encoded according to the signature profile, and $(x_A,y_A)$ are the coordinates of $P_A$. The message representative is

$$
e=\operatorname{SM3}(Z_A \parallel M). \tag{2.2.5}
$$

SM3 produces a 256-bit digest [3]. Including $Z_A$ binds the calculation to the configured identity and public key, but it does not authenticate the human or organization by itself. The signer and verifier must obtain the same identity value from a trusted protocol context. A commonly shown default identity is the 16-byte ASCII string `1234567812345678`, for which $ENTL_A=128$, but a banking system must use the identity defined by its own certificate, protocol, or application profile. Silent fallback to a default value can cause interoperability or identity-binding errors.

### Signature generation

To sign message $M$, the signer first computes $e$ using Eq. (2.2.5), then performs the following steps [1]:

1. Generate a fresh, secret, uniformly distributed nonce $k\in[1,n-1]$.
2. Compute the curve point $(x_1,y_1)=[k]G$.
3. Compute

   $$
   r=(e+x_1)\bmod n. \tag{2.2.6}
   $$

   If $r=0$ or $r+k=n$, generate a new $k$ and restart.
4. Compute

   $$
   s=((1+d_A)^{-1}(k-rd_A))\bmod n. \tag{2.2.7}
   $$

   If $s=0$, generate a new $k$ and restart.
5. Output the signature pair $(r,s)$ in the encoding required by the surrounding protocol.

The modular inverse exists because the key-generation range excludes $d_A=n-1$. The nonce is a one-time secret. Reusing the same $k$, generating it with inadequate entropy, or leaking information about it through timing, power, cache, or fault behavior can expose the long-term private key without solving the elliptic-curve discrete-logarithm problem.

### Signature verification

Given $M$, signer identity $ID_A$, public key $P_A$, and signature $(r,s)$, the verifier [1]:

1. Validates the public key and checks $r,s\in[1,n-1]$.
2. Recomputes $Z_A$ and $e=\operatorname{SM3}(Z_A\parallel M)$ using the exact configured identity and domain parameters.
3. Computes

   $$
   t=(r+s)\bmod n. \tag{2.2.8}
   $$

   If $t=0$, reject.
4. Computes

   $$
   (x_1',y_1')=[s]G+[t]P_A. \tag{2.2.9}
   $$

5. Computes

   $$
   R=(e+x_1')\bmod n \tag{2.2.10}
   $$

   and accepts if and only if $R=r$.

Verification must reject malformed encodings, out-of-range scalars, the point at infinity, points outside the selected curve, and public keys that do not satisfy the required subgroup checks. A mathematically valid signature is meaningful only when the public key, identity, certificate status, and transaction context have also been validated.

### Security assumptions and attacks

The public-key relation $P_A=[d_A]G$ is easy to compute in the forward direction. Recovering $d_A$ from $P_A$ is the elliptic-curve discrete-logarithm problem. Exhaustive search over a standard 256-bit scalar space is infeasible, and Pollard's rho reduces generic attack cost to approximately $O(\sqrt{n})$, corresponding to an intended classical security level of about 128 bits for the selected subgroup.

The ECDLP assumption does not protect a faulty implementation. The main practical risks are:

- **Nonce failure:** reuse, bias, partial leakage, or predictability of $k$ can expose $d_A$ algebraically or through statistical/lattice methods.
- **Private-key exposure:** keys stored in application memory can be copied or misused even when the curve remains mathematically secure.
- **Invalid public keys:** missing curve or subgroup validation can create invalid-point and small-subgroup attack paths.
- **Side channels and faults:** scalar multiplication and modular arithmetic require side-channel-resistant implementations and appropriate fault checks.
- **Configuration mismatch:** different signer IDs, SM3 inputs, curve identifiers, or signature encodings cause rejection or unintended identity semantics.

SM2 is not post-quantum secure. A sufficiently capable fault-tolerant quantum computer running Shor's algorithm could solve the underlying ECDLP in polynomial time. This is a theoretical migration risk, not evidence that current quantum computers or the Section 4 toy-key experiments can recover a standard SM2 private key.

## 2.2.2 Application in Banking System

### Role in a banking signature workflow

SM2 can provide transaction integrity and signer-key attribution in banking systems that require the SM2/SM3 ecosystem. The signature should be applied to a canonical representation that binds the transaction identifier, source and destination accounts, amount, currency, timestamp, replay-prevention value, purpose, and relevant approval context. The signer identity used in $Z_A$ must come from the banking protocol or certificate profile and must remain identical at verification.

SM2 does not replace transport encryption, requester authentication, authorization, certificate validation, replay prevention, fraud screening, sanctions controls, or audit logging. A valid signature proves only that the verification equation succeeds for the supplied message representation, identity, and public key. Section 3 therefore treats SM2 as one possible signature suite inside a larger acceptance and rejection workflow.

### Key management and implementation controls

For a banking deployment, private SM2 keys should be generated and retained in a protected cryptographic module when the platform supports the required curve and SM3 operations. Access to the signing operation should be authenticated and authorized, with role separation or dual control for high-value keys. The lifecycle must include activation, rotation, certificate renewal, revocation, compromise response, archival policy, and destruction.

The signing implementation must use a cryptographically secure source for each nonce and must prevent nonce or scalar leakage. Signer and verifier configurations should pin:

- the SM2 curve and domain parameters;
- SM3 as the digest function;
- the exact source and encoding of $ID_A$ and $ENTL_A$;
- the public-key and certificate profile;
- the signature encoding and canonical message format;
- validation, rejection, and audit behavior.

The banking service should not treat a library's generic “SM2” label as a complete protocol profile. Interoperability tests must include different signer IDs, modified messages, malformed points, out-of-range scalars, altered encodings, revoked certificates, and replayed transactions.

### Evidence and deployment position

Correctness test C5 in Section 4.1 generated and verified an SM2/SM3 signature through the tested OpenSSL CLI configuration. The observed DER signature was 72 bytes. This is one encoded output, not a universal fixed length: DER encodes the two signature integers and their lengths, so the total can vary.

SM2 provides a compact elliptic-curve signature with explicit signer-identity binding and an intended classical security level comparable to other 256-bit elliptic-curve schemes in this report. Its deployment is most appropriate where regulatory, protocol, partner, or infrastructure requirements already support SM2 and SM3. Its limitations are the fragility of nonce handling, the need for exact identity and encoding agreement, dependence on suitable certificate/HSM support, and exposure to a future cryptographically relevant quantum computer.

The report's CPU/GPU ECDLP experiments use separate 18-, 22-, and 26-bit toy curves. They illustrate search scaling and implementation effects only. They do not benchmark SM2 signing or verification and do not support an estimate for recovering a standard SM2 key.

## References

1. Standardization Administration of China, *Information Security Technology - Public Key Cryptographic Algorithm SM2 Based on Elliptic Curves - Part 2: Digital Signature Algorithm*, GB/T 32918.2-2016, 2016.
2. Standardization Administration of China, *Information Security Technology - Public Key Cryptographic Algorithm SM2 Based on Elliptic Curves - Part 5: Parameter Definition*, GB/T 32918.5-2017, 2017.
3. Standardization Administration of China, *Information Security Technology - SM3 Cryptographic Hash Algorithm*, GB/T 32905-2016, 2016.
4. S. Zhang, D. Lee, R. Wang, D. Zhang, and T. Yao, "SM2 Digital Signature Algorithm for DNSSEC," RFC 9563, Dec. 2024, doi: 10.17487/RFC9563.
