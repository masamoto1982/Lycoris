#!/usr/bin/env node
// Pins the "did you mean" suggester `word_contract` answers an unmatched name
// with, over the same registry asset the tool reads in production.

import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { suggestWords } from "./word-candidates.js";

const registry = JSON.parse(
  readFileSync(new URL("./assets/words.json", import.meta.url), "utf8"),
);

// A one-letter transposition/omission on a short, medium, and longer name
// (each falls in a different distance-ceiling bucket); names with several
// equally-close matches, to exercise the distance-then-alphabetical
// tie-break; an unmatched name, which must come back empty; and full-width
// input, which a Japanese input method produces and which must fold to the
// ASCII name rather than resemble nothing.
const CASES = [
  ["LENGHT", ["LENGTH"]],
  ["MAPP", ["MAP", "MAX"]],
  ["FILTR", ["FILTER", "FILL"]],
  ["PRIN", ["PRINT", "JOIN", "MIN"]],
  ["ADDD", ["ADD", "AND"]],
  ["SQR", ["SQRT", "STR"]],
  ["EXECC", ["EXEC"]],
  ["ZZZZZZZZZZ", []],
  ["ＡＤＤ", ["ADD", "AND"]],
  ["ＬＥＮＧＨＴ", ["LENGTH"]],
  // An exact name is a match, not a near-miss of itself.
  ["ADD", ["AND"]],
];

for (const [word, expected] of CASES) {
  assert.deepEqual(suggestWords(word, registry.entries), expected, `suggestWords(${JSON.stringify(word)})`);
}

console.log(`word-candidates: ${CASES.length} cases.`);
