#!/usr/bin/env node
// CI gate: every Core-classified surface word (docs/word-manifest.json) must
// appear in at least one `ajisai-source` program of
// tests/conformance/index.html. Exits non-zero if any is missing.
//
// Usage:
//   node scripts/check-conformance-coverage.mjs            # human summary
//   node scripts/check-conformance-coverage.mjs --json     # JSON report
//   node scripts/check-conformance-coverage.mjs --suite F  # alternate suite file

import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { decodeEntities, readJson, repoRoot } from './lib/common.mjs';

const args = process.argv.slice(2);
const asJson = args.includes('--json');
const suiteFlag = args.indexOf('--suite');
const suitePath = suiteFlag !== -1 && args[suiteFlag + 1]
  ? resolve(args[suiteFlag + 1])
  : resolve(repoRoot, 'tests/conformance/index.html');

const manifest = readJson('docs/word-manifest.json');

const coreWords = new Set(
  manifest.entries
    .filter((e) => e.classification === 'Core')
    .map((e) => e.surface),
);

const html = readFileSync(suitePath, 'utf8');
const sources = [];
const srcPattern = /<pre class="ajisai-source">([\s\S]*?)<\/pre>/g;
for (const m of html.matchAll(srcPattern)) {
  sources.push(decodeEntities(m[1]));
}
if (sources.length === 0) {
  console.error(`no ajisai-source programs found in ${suitePath}`);
  process.exit(2);
}

const seen = new Set();
for (const src of sources) {
  for (const tok of src.split(/\s+/)) {
    if (tok) seen.add(tok);
  }
}

const covered = [...coreWords].filter((w) => seen.has(w)).sort();
const missing = [...coreWords].filter((w) => !seen.has(w)).sort();
const pct = (100 * covered.length) / coreWords.size;

if (asJson) {
  process.stdout.write(`${JSON.stringify({
    schemaVersion: 3,
    suite: suitePath,
    caseSources: sources.length,
    coreWords: coreWords.size,
    covered: covered.length,
    coveragePercent: Number(pct.toFixed(1)),
    missing,
  }, null, 2)}\n`);
} else {
  console.log(`conformance suite: ${sources.length} case sources`);
  console.log(`Core Profile words: ${coreWords.size}`);
  console.log(`covered: ${covered.length} (${pct.toFixed(1)}%)`);
  if (missing.length) {
    console.log(`missing (${missing.length}): ${missing.join(' ')}`);
  } else {
    console.log('missing: none');
  }
}

if (missing.length > 0) {
  console.error(
    `[conformance-coverage] ${missing.length} Core word(s) `
    + 'have no conformance case. Add an `ajisai-source` case that uses each, '
    + 'or run with --json to see the full list.',
  );
  process.exitCode = 1;
}
