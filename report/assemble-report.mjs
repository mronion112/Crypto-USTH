import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const reportDir = path.dirname(fileURLToPath(import.meta.url));

const citationMaps = {
  introduction: [0, 1, 2, 3, 4, 5, 6, 7],
  ed25519: [0, 8, 9, 10, 1, 11, 12],
  sm2: [0, 13, 14, 15, 16],
  rsa: [0, 17, 18, 1, 19, 3, 20, 21, 22, 23, 24, 6, 25, 26, 27],
  ecdsa: [0, 1, 28, 29, 30, 31, 32, 3, 4, 8, 16, 13],
  mldsa: [0, 2, 34, 33, 35, 36, 37, 6, 38, 39],
  workflow: [0, 5, 40, 41, 7, 8, 18, 30, 4, 1, 2, 3, 42, 43, 13, 44, 45],
  testing: [0, 1, 8, 13, 2, 46, 47, 30, 48, 49, 50, 31, 20, 51, 52, 27, 53],
  quantum: [0, 6, 54, 25, 2, 34, 55],
};

const sections = [
  {
    file: "01-introduction/draft.md",
    map: citationMaps.introduction,
    stripAt: /^## References\s*$/m,
  },
  {
    file: "02-cryptographic-algorithms/2.1-ed25519/draft.md",
    map: citationMaps.ed25519,
    stripAt: /^## References\s*$/m,
  },
  {
    file: "02-cryptographic-algorithms/2.2-sm2/draft.md",
    map: citationMaps.sm2,
    stripAt: /^## References\s*$/m,
  },
  {
    file: "02-cryptographic-algorithms/2.3-rsa/draft.md",
    map: citationMaps.rsa,
    stripAt: /^## References\s*$/m,
  },
  {
    file: "02-cryptographic-algorithms/2.4-ecdsa/draft.md",
    map: citationMaps.ecdsa,
    stripAt: /^## References\s*$/m,
  },
  {
    file: "02-cryptographic-algorithms/2.5-post-quantum-cryptography/draft.md",
    map: citationMaps.mldsa,
    stripAt: /^## References\s*$/m,
  },
  {
    file: "03-banking-workflow/draft.md",
    map: citationMaps.workflow,
    stripAt: /^## References\s*$/m,
    replacements: [
      [/\*\*Author:\*\*[^\n]*\n\*\*Section:\*\*[^\n]*\n\*\*Topic:\*\*[^\n]*/g, ""],
      [/```mermaid[\s\S]*?```/g, ""],
      [/\]\(workflow\.svg\)/g, "](03-banking-workflow/workflow.svg)"],
      [/\]\(workflow\.png\)/g, "](03-banking-workflow/workflow.png)"],
    ],
  },
  {
    file: "04-testing-performance/4.1-test-cases-and-setup/final-en.md",
    map: citationMaps.testing,
    replacements: [
      [/#ref-\d+\)/g, ")"],
      [/\]\(\.\.\/4\.2-cpu-gpu-key-recovery\/references\.md\)/g, "](#6-references)"],
    ],
  },
  {
    file: "04-testing-performance/4.2-cpu-gpu-key-recovery/final-en.md",
    map: citationMaps.testing,
    stripAt: /^## Tài liệu tham khảo của mục\s*$/m,
    replacements: [
      [/\]\(figures\//g, "](04-testing-performance/4.2-cpu-gpu-key-recovery/figures/"],
    ],
  },
  {
    file: "04-testing-performance/4.3-quantum-theoretical-analysis/draft.md",
    map: citationMaps.quantum,
    stripAt: /^## References\s*$/m,
  },
];

const frontMatter = `# Digital Signature Algorithms and Their Resistance to Attacks in Banking

> **Consolidated review draft**<br>
> University: [To be completed]<br>
> Faculty/Department: [To be completed]<br>
> Course: [To be completed]<br>
> Lecturer: [To be completed]<br>
> Group: [To be completed]<br>
> Date: [To be completed]

## Team members

| Member | Assigned contribution |
|---|---|
| Hoàng Thu An | Ed25519 |
| Nguyễn Tiến Đạt | SM2 |
| Vũ Minh Hoàng | RSA-PSS and ML-DSA |
| Nguyễn Hoàng Minh | ECDSA; coordination of Introduction, Conclusion, and References |
| Nguyễn Văn Đức | Banking workflow |
| Nguyễn Quang Vũ Hoàng | Testing and performance implementation |
| Nguyễn Hoàng Anh | Introduction, testing analysis, Conclusion, References, and report integration |

## Table of contents

1. [Introduction](#1-introduction)
2. [Cryptographic Algorithms](#2-cryptographic-algorithms)
   - [2.1 Ed25519](#21-ed25519)
   - [2.2 SM2](#22-sm2)
   - [2.3 RSA-2048 and RSA-4096 (RSA-PSS)](#23-rsa-2048-and-rsa-4096-rsa-pss)
   - [2.4 ECDSA](#24-ecdsa)
   - [2.5 Post-Quantum Cryptography — ML-DSA](#25-post-quantum-cryptography--ml-dsa)
3. [Mechanism of Operation — Banking Workflow](#3-mechanism-of-operation--banking-workflow)
4. [Testing and Performance Analysis](#4-testing-and-performance-analysis)
   - [4.1 Test Cases and Experimental Setup](#41-test-cases-and-experimental-setup)
   - [4.2 CPU/GPU Key-Recovery Benchmark](#42-cpugpu-key-recovery-benchmark)
   - [4.3 Quantum Computer — Theoretical Analysis](#43-quantum-computer--theoretical-analysis)
5. [Conclusion and Future Work](#5-conclusion-and-future-work)
6. [References](#6-references)
`;

function read(relativePath) {
  return fs.readFileSync(path.join(reportDir, relativePath), "utf8").replace(/\r\n/g, "\n");
}

function remapCitations(text, map, source) {
  const convert = (localText) => {
    const local = Number(localText);
    const global = map[local];
    if (!global) {
      throw new Error(`No citation mapping for [${local}] in ${source}`);
    }
    return `[${global}]`;
  };

  let output = text.replace(/\[\[(\d+)\]\]\([^)]*\)/g, (_, n) => `@@CITE${convert(n).slice(1, -1)}@@`);
  output = output.replace(/(?<![\w$])\[(\d+)\](?![\w[])/g, (_, n) => convert(n));
  output = output.replace(/@@CITE(\d+)@@/g, "[$1]");
  return output;
}

function prepareSection(config) {
  let content = read(config.file).trim();

  if (config.stripAt) {
    const match = config.stripAt.exec(content);
    if (match) content = content.slice(0, match.index).trim();
  }

  content = remapCitations(content, config.map, config.file);

  for (const [pattern, replacement] of config.replacements ?? []) {
    content = content.replace(pattern, replacement);
  }

  content = content.replace(/^---\s*$/gm, "").replace(/\n{3,}/g, "\n\n");

  return content;
}

function reduceStrongEmphasis(text) {
  return text
    .split("\n")
    .map((line) => {
      // Keep table captions visually distinct, but avoid decorative bold text
      // throughout ordinary prose, lists, and table cells.
      if (/^\s*\*\*Table\s+[^*]+\*\*\s*$/.test(line)) return line;
      return line.replace(/\*\*([^*\n]+)\*\*/g, "$1");
    })
    .join("\n");
}

const assembled = [];
assembled.push(frontMatter.trim());
assembled.push(prepareSection(sections[0]));
assembled.push("# 2 Cryptographic Algorithms");
for (const section of sections.slice(1, 6)) assembled.push(prepareSection(section));
assembled.push(prepareSection(sections[6]));
assembled.push("# 4 Testing and Performance Analysis");
assembled.push(prepareSection(sections[7]));
assembled.push(prepareSection(sections[8]));
assembled.push(prepareSection(sections[9]));
assembled.push(prepareSection({
  file: "05-conclusion/draft.md",
  map: citationMaps.introduction,
}));
assembled.push(read("06-references/references.md")
  .replace(/The following consolidated IEEE-style list[^\n]*\n/, "")
  .trim());

const output = `${reduceStrongEmphasis(assembled.join("\n\n---\n\n")).replace(/[ \t]+$/gm, "")}\n`;
fs.writeFileSync(path.join(reportDir, "FINAL_REPORT.md"), output, "utf8");
console.log(`Wrote report/FINAL_REPORT.md (${output.split("\n").length - 1} lines)`);
