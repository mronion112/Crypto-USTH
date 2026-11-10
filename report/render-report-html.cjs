const fs = require("fs");
const path = require("path");
const { marked } = require("marked");

const reportDir = __dirname;
const inputPath = path.join(reportDir, "FINAL_REPORT.md");
const outputPath = path.join(reportDir, "FINAL_REPORT.html");

function escapeHtml(value) {
  return value
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;");
}

function protectMath(markdown) {
  const blocks = [];
  const inline = [];
  const codeBlocks = [];
  const inlineCode = [];

  let protectedText = markdown.replace(/```[\s\S]*?```/g, (code) => {
    const index = codeBlocks.push(code) - 1;
    return `CODEBLOCKPLACEHOLDER${index}END`;
  });

  protectedText = protectedText.replace(/`[^`\n]+`/g, (code) => {
    const index = inlineCode.push(code) - 1;
    return `INLINECODEPLACEHOLDER${index}END`;
  });

  protectedText = protectedText.replace(/\$\$([\s\S]*?)\$\$/g, (_, expression) => {
    const index = blocks.push(expression.trim()) - 1;
    return `\n\nMATHBLOCKPLACEHOLDER${index}END\n\n`;
  });

  protectedText = protectedText.replace(/(?<!\\)\$((?:\\\$|[^\n$])+?)(?<!\\)\$/g, (_, expression) => {
    const index = inline.push(expression.trim()) - 1;
    return `MATHINLINEPLACEHOLDER${index}END`;
  });

  inlineCode.forEach((code, index) => {
    protectedText = protectedText.replaceAll(`INLINECODEPLACEHOLDER${index}END`, code);
  });
  codeBlocks.forEach((code, index) => {
    protectedText = protectedText.replaceAll(`CODEBLOCKPLACEHOLDER${index}END`, code);
  });

  return { protectedText, blocks, inline };
}

function restoreMath(html, blocks, inline) {
  let output = html;

  blocks.forEach((expression, index) => {
    const placeholder = `MATHBLOCKPLACEHOLDER${index}END`;
    const rendered = `<div class="math-display">\\[${escapeHtml(expression)}\\]</div>`;
    output = output
      .replace(`<p>${placeholder}</p>`, rendered)
      .replaceAll(placeholder, rendered);
  });

  inline.forEach((expression, index) => {
    const placeholder = `MATHINLINEPLACEHOLDER${index}END`;
    output = output.replaceAll(
      placeholder,
      `<span class="math-inline">\\(${escapeHtml(expression)}\\)</span>`,
    );
  });

  return output;
}

const markdown = fs.readFileSync(inputPath, "utf8");
const { protectedText, blocks, inline } = protectMath(markdown);
marked.setOptions({ gfm: true, breaks: false });
const body = restoreMath(marked.parse(protectedText), blocks, inline);

let html = `<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Digital Signature Algorithms and Their Resistance to Attacks in Banking</title>
  <script>
    window.MathJax = {
      tex: {
        inlineMath: [['\\\\(', '\\\\)']],
        displayMath: [['\\\\[', '\\\\]']],
        tags: 'ams'
      },
      options: {
        skipHtmlTags: ['script', 'noscript', 'style', 'textarea', 'annotation', 'annotation-xml']
      },
      chtml: { scale: 1 }
    };
  </script>
  <script defer src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-chtml.js"></script>
  <style>
    :root { color-scheme: light; }
    * { box-sizing: border-box; }
    html { scroll-behavior: smooth; }
    body {
      margin: 0;
      background: #eef1f5;
      color: #1c2430;
      font-family: Georgia, "Times New Roman", "Noto Serif", serif;
      font-size: 17px;
      line-height: 1.65;
    }
    main {
      width: min(1120px, calc(100% - 32px));
      margin: 32px auto;
      padding: 64px 72px;
      background: #fff;
      border: 1px solid #dfe3e8;
      border-radius: 10px;
      box-shadow: 0 12px 36px rgba(20, 30, 45, 0.09);
    }
    h1, h2, h3, h4 { color: #13243a; line-height: 1.25; scroll-margin-top: 18px; }
    h1 { margin-top: 2.4em; padding-bottom: .25em; border-bottom: 2px solid #244f7a; font-size: 2rem; }
    h1:first-child { margin-top: 0; border-bottom: 0; text-align: center; font-size: 2.35rem; }
    h2 { margin-top: 2em; font-size: 1.55rem; }
    h3 { margin-top: 1.6em; font-size: 1.25rem; }
    h4 { margin-top: 1.4em; font-size: 1.08rem; }
    p { text-align: justify; }
    a { color: #155a96; text-decoration-thickness: 1px; text-underline-offset: 2px; }
    blockquote { margin: 1.25em 0; padding: .7em 1.1em; border-left: 4px solid #4678a7; background: #f4f8fc; }
    table { width: 100%; margin: 1.25em 0 1.75em; border-collapse: collapse; font-family: Arial, "Noto Sans", sans-serif; font-size: .86rem; }
    th, td { padding: .55em .65em; border: 1px solid #aeb8c3; vertical-align: top; }
    th { background: #e9f0f7; color: #102a43; text-align: left; }
    tr:nth-child(even) td { background: #f8fafc; }
    code { padding: .12em .28em; border-radius: 4px; background: #eef2f6; font-family: Consolas, "Courier New", monospace; font-size: .9em; }
    pre { overflow-x: auto; padding: 1em; border: 1px solid #d5dce4; border-radius: 6px; background: #f7f9fb; }
    pre code { padding: 0; background: transparent; }
    img { display: block; max-width: 100%; height: auto; margin: 1.5em auto .5em; }
    .math-display { margin: 1.1em 0; overflow-x: auto; text-align: center; }
    .math-inline { white-space: nowrap; }
    hr { margin: 2.5em 0; border: 0; border-top: 1px solid #cbd3dc; }
    @media (max-width: 760px) {
      body { font-size: 15px; }
      main { width: 100%; margin: 0; padding: 24px 18px; border: 0; border-radius: 0; }
      table { display: block; overflow-x: auto; white-space: nowrap; }
    }
    @media print {
      @page { size: A4; margin: 18mm 16mm 20mm; }
      body { background: #fff; font-size: 11pt; }
      main { width: auto; margin: 0; padding: 0; border: 0; box-shadow: none; }
      h1 { break-before: page; }
      h1:first-child { break-before: auto; }
      h1, h2, h3, h4 { break-after: avoid; }
      table, figure, img, pre, blockquote { break-inside: avoid; }
      a { color: inherit; text-decoration: none; }
    }
  </style>
</head>
<body>
  <main>${body}</main>
</body>
</html>
`;

html = html.replace(/[ \t]+$/gm, "");
fs.writeFileSync(outputPath, html, "utf8");
if (/MATH(?:BLOCK|INLINE)PLACEHOLDER/.test(html)) {
  throw new Error("Unresolved math placeholder in generated HTML");
}
if (html.includes("�")) {
  throw new Error("Unicode replacement character found in generated HTML");
}

const count = (needle) => html.split(needle).length - 1;
console.log(`Wrote ${path.relative(process.cwd(), outputPath)} (${html.length} bytes)`);
console.log(`Math: ${blocks.length} display, ${inline.length} inline; tables: ${count("<table>")}; images: ${count("<img ")}`);
