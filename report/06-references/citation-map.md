# Local-to-Global Citation Map

Use this map only when assembling the final report. The left side is the citation number in the section draft; the right side is the number in `references.md`.

| Section | Local-to-global mapping |
|---|---|
| 1 Introduction | 1→1, 2→2, 3→3, 4→4, 5→5, 6→6, 7→7 |
| 2.1 Ed25519 | 1→8, 2→9, 3→10, 4→1, 5→11, 6→12 |
| 2.2 SM2 | 1→13, 2→14, 3→15, 4→16 |
| 2.3 RSA-PSS | 1→17, 2→18, 3→1, 4→19, 5→3, 6→20, 7→21, 8→22, 9→23, 10→24, 11→6, 12→25, 13→26, 14→27 |
| 2.4 ECDSA | 1→1, 2→28, 3→29, 4→30, 5→31, 6→32, 7→3, 8→4, 9→8, 10→16, 11→13 |
| 2.5 ML-DSA | 1→2, 2→34, 3→33, 4→35, 5→36, 6→37, 7→6, 8→38, 9→39 |
| 3 Banking workflow | 1→5, 2→40, 3→41, 4→7, 5→8, 6→18, 7→30, 8→4, 9→1, 10→2, 11→3, 12→42, 13→43, 14→13, 15→44, 16→45 |
| 4 Testing and benchmark | 1→1, 2→8, 3→13, 4→2, 5→46, 6→47, 7→30, 8→48, 9→49, 10→50, 11→31, 12→20, 13→51, 14→52, 15→27, 16→53 |

## Assembly check

After replacing local numbers, verify that every bracketed citation in the assembled body exists in `references.md`, every listed reference is cited, and ranges such as `[3]-[6]` were not altered as if they were subtraction or prose punctuation.
