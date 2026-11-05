# GPU benchmark summary

### RSA toy - full scan

| Bits | Block | Runs | Median ms | IQR ms | M candidates/s | Verified |
|---|---|---|---|---|---|---|
| 32 | 256 | 10 | 0.069 | 0.034 | 308.196 | 1 |
| 40 | 256 | 10 | 0.081 | 0.026 | 3152.209 | 1 |
| 48 | 256 | 10 | 0.181 | 0.012 | 23302.334 | 1 |
| 56 | 256 | 10 | 2.171 | 0.076 | 33200.667 | 1 |

### RSA toy - recovery

| Bits | Block | Runs | Median ms | IQR ms | M candidates/s | Verified |
|---|---|---|---|---|---|---|
| 32 | 256 | 10 | 0.087 | 0.006 | - | 1 |
| 40 | 256 | 10 | 0.084 | 0.008 | - | 1 |
| 48 | 256 | 10 | 0.181 | 0.016 | - | 1 |
| 56 | 256 | 10 | 2.288 | 0.456 | - | 1 |

### ECDLP toy - full scan

| Bits | Block | Runs | Median ms | IQR ms | M candidates/s | Verified |
|---|---|---|---|---|---|---|
| 18 | 256 | 10 | 0.727 | 0.046 | 112.773 | 1 |
| 22 | 256 | 10 | 28.000 | 2.397 | 93.329 | 1 |
| 26 | 256 | 10 | 530.816 | 0.010 | 102.707 | 1 |

### ECDLP toy - recovery

| Bits | Block | Runs | Median ms | IQR ms | M candidates/s | Verified |
|---|---|---|---|---|---|---|
| 18 | 256 | 10 | 0.595 | 0.119 | - | 1 |
| 22 | 256 | 10 | 13.602 | 10.468 | - | 1 |
| 26 | 256 | 10 | 245.928 | 259.509 | - | 1 |
