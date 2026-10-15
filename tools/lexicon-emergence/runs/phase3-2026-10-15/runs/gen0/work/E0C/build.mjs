import { writeFileSync, readFileSync } from "node:fs";
import { execFileSync } from "node:child_process";
const W = "tools/lexicon-emergence/runs/phase3-2026-10-15/runs/gen0/work/E0C";
const SUB = "tools/lexicon-emergence/runs/phase3-2026-10-15/runs/gen0/submissions/E0C.json";
const PREF = "E0C.CODES [ 0 ] [ 'C' BIND 'S' BIND S -1 DROP S C 1 COLLECT CONCAT 2 COLLECT 0 1 S -1 GET C ADD 0 EQ S -1 GET 0 GT AND SELECT GET ] SCAN [ LENGTH ] MAP 'L' BIND L 1 GT NOT 'B' BIND L [ 'X' BIND 1 ] MAP 0 [ ADD ] SCAN 0 B SELECT 0 [ MAX ] FOLD";
const JIN = `'{"orders":[{"sku":"a1","qty":2,"price":1.5},{"sku":"b2","qty":1,"price":4,"tags":["gift"]},{"sku":"a1","qty":3,"price":1.5,"tags":["bulk","gift"]},{"sku":"c3","qty":5}]}'`;
const MIN = "'  The cat saw THE dog   and the Cat  '";
const defs = [
 { name: "E0C.RUNS", note: "Run-length groups of a Vector: one Vector per maximal run of equal adjacent values, in order.",
   body: "'V' BIND V 1 DROP V -1 DROP SUB 'D' BIND [ TRUE ] D D MUL 0 GT CONCAT 'S' BIND 1 0 S SELECT 0 [ ADD ] SCAN 'ID' BIND ID V GROUP VALUES" },
 { name: "E0C.SPJOIN", note: "Joins a Vector of Strings with single spaces, no leading or trailing space.",
   body: "[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM" },
 { name: "E0C.WORDS", note: "Splits a String on spaces and drops the empty pieces: the words in order.",
   body: "' ' TOKENIZE [ '' EQ NOT ] FILTER" },
 { name: "E0C.ORDERS", note: "Parses the JSON text and answers its orders Vector.",
   body: "JSON-DECODE 'orders' GET" },
 { name: "E0C.SKUQTY", note: "From an orders Vector: a Record from each sku to its total qty, in first-appearance order.",
   body: "'OS' BIND OS [ 'sku' GET ] MAP OS [ 'qty' GET ] MAP GROUP 'G' BIND G KEYS G VALUES [ 0 [ ADD ] FOLD ] MAP RECORD" },
 { name: "E0C.CODES", note: "Bracket String to a Vector of codes: ( = 1, ) = -1, [ = 2, ] = -2.",
   body: "CHARS [ 'C' BIND [ 1 -1 2 -2 ] '()[]' C SEARCH GET ] MAP" },
 { name: "E0C.REDUCE", note: "Bracket String to its unmatched residue as a stack: [0] first, then unmatched letters; [0] alone means balanced.",
   body: "E0C.CODES [ 0 ] [ 'C' BIND 'S' BIND S -1 DROP S C 1 COLLECT CONCAT 2 COLLECT 0 1 S -1 GET C ADD 0 EQ S -1 GET 0 GT AND SELECT GET ] FOLD" },
 { name: "E0C.STEPS", note: "Bracket String to a Vector of +1 (open) and -1 (close) per character.",
   body: "E0C.CODES [ 'C' BIND 1 -1 C 0 GT SELECT ] MAP" },
];
const solutions = {
 "runs.rle": "E0C.RUNS 'G' BIND G [ 0 GET ] MAP G [ LENGTH ] MAP 2 COLLECT ZIP",
 "runs.dedupe": "E0C.RUNS [ 0 GET ] MAP",
 "runs.upruns": "'V' BIND V 1 DROP V -1 DROP SUB 0 GT NOT 'C' BIND [ TRUE ] C CONCAT 'S' BIND 1 0 S SELECT 0 [ ADD ] SCAN TALLY VALUES 0 [ MAX ] FOLD",
 "runs.second_largest": "UNIQUE SORT 'SO' BIND SO SO LENGTH 2 SUB GET NIL SO LENGTH 1 GT SELECT",
 "runs.majority": "'V' BIND V TALLY 'R' BIND R VALUES 'C' BIND R KEYS 'K' BIND C ORDER -1 GET 'I' BIND K I GET NIL C I GET 2 MUL V LENGTH GT SELECT",
 "number.primes": "'N' BIND 1 N RANGE [ 'K' BIND 1 K RANGE [ 'D' BIND K D DIV FLOOR D MUL K EQ ] FILTER LENGTH 2 EQ ] FILTER",
 "number.divisors": "'N' BIND 1 N RANGE [ 'D' BIND N D DIV FLOOR D MUL N EQ ] FILTER",
 "number.binary": "'N' BIND 5 0 RANGE [ 'K' BIND N 2 K POW LT NOT ] FILTER [ 'K' BIND N 2 K POW DIV FLOOR 'F' BIND F F 2 DIV FLOOR 2 MUL SUB ] MAP STR JOIN",
 "number.collatz": "'N' BIND 0 199 RANGE N [ 'I' BIND 'X' BIND 1 X 2 DIV 3 X MUL 1 ADD X 2 DIV FLOOR 2 MUL X EQ SELECT X 1 EQ SELECT ] SCAN N 1 COLLECT CONCAT [ 1 GT ] FILTER LENGTH",
 "number.digital_root": "'N' BIND N N 1 SUB 9 DIV FLOOR 9 MUL SUB",
 "number.fib_index": "'N' BIND [ 1 1 2 3 5 8 13 21 34 55 89 144 ] [ N LT ] FILTER LENGTH 1 ADD",
 "brackets.depth": "E0C.STEPS 0 [ ADD ] SCAN 0 [ MAX ] FOLD",
 "brackets.balanced": "E0C.REDUCE LENGTH 1 EQ",
 "brackets.balanced_prefix": PREF,
 "brackets.unmatched": "E0C.REDUCE LENGTH 1 SUB",
 "sensor.present_mean": "[ NIL? NOT ] FILTER 'P' BIND P 0 [ ADD ] FOLD P LENGTH DIV",
 "sensor.filled": "'R' BIND R R [ NIL? NOT ] FILTER 0 GET [ 'X' BIND 'A' BIND X A X NIL? NOT SELECT ] SCAN",
 "sensor.gap_count": "'R' BIND R [ NIL? ] MAP 'M' BIND [ FALSE ] M -1 DROP CONCAT 'P' BIND M P NOT AND 'S' BIND 1 0 S SELECT 0 [ ADD ] FOLD",
 "sensor.longest_gap": "0 [ 'X' BIND 'A' BIND A 1 ADD 0 X NIL? SELECT ] SCAN 0 [ MAX ] FOLD",
 "sensor.ratios": "'V' BIND V 1 DROP V -1 DROP DIV",
 "sensor.scaled": "'R' BIND R [ NIL? NOT ] FILTER 'P' BIND R P P 0 GET [ MAX ] FOLD DIV",
 "orders.revenue": "E0C.ORDERS [ 'O' BIND O 'qty' GET O 'price' GET MUL 0 O 'price' HAS? SELECT ] MAP 0 [ ADD ] FOLD",
 "orders.report": "E0C.ORDERS [ 'O' BIND O 'qty' GET O 'price' GET MUL 0 O 'price' HAS? SELECT ] MAP 0 [ ADD ] FOLD 2 FORMAT",
 "orders.unpriced": "E0C.ORDERS [ 'price' HAS? NOT ] FILTER [ 'sku' GET ] MAP UNIQUE",
 "orders.qty_by_sku": "E0C.ORDERS E0C.SKUQTY",
 "orders.top_sku": "E0C.ORDERS E0C.SKUQTY 'Q' BIND Q KEYS Q VALUES -1 MUL ORDER 0 GET GET",
 "orders.tag_counts": "E0C.ORDERS [ 'tags' HAS? ] FILTER [ 'tags' GET ] MAP FLATTEN TALLY",
 "messy.count": "E0C.WORDS LENGTH",
 "messy.normalize": "E0C.WORDS [ LOWER ] MAP E0C.SPJOIN",
 "messy.distinct": "E0C.WORDS [ LOWER ] MAP UNIQUE LENGTH",
 "messy.top_word": "E0C.WORDS [ LOWER ] MAP TALLY 'T' BIND T KEYS T VALUES -1 MUL ORDER 0 GET GET",
 "messy.title": "E0C.WORDS [ LOWER 'W' BIND W CHARS 0 GET UPPER W CHARS 1 DROP JOIN 2 COLLECT JOIN ] MAP E0C.SPJOIN",
 "messy.sorted_unique": "E0C.WORDS [ LOWER ] MAP UNIQUE SORT E0C.SPJOIN",
};
const examples = {};
for (const k of Object.keys(solutions)) {
  if (k.startsWith("runs.")) examples[k] = "[ 1 1 2 2 2 3 1 1 ]";
  else if (k.startsWith("number.")) examples[k] = "10";
  else if (k.startsWith("brackets.")) examples[k] = "'([]())'";
  else if (k.startsWith("sensor.")) examples[k] = "[ 3 NIL 5 1 NIL NIL 4 ]";
  else if (k.startsWith("orders.")) examples[k] = JIN;
  else examples[k] = MIN;
}
const sub = { agent: "E0C", generation: 0, condition: "runs", definitions: defs, solutions };
writeFileSync(SUB, JSON.stringify(sub, null, 2) + "\n");
// verify: defs as DEF lines, example input, solution, through compute
const defText = defs.map(d => `[ ${d.body} ] '${d.name}' DEF`).join("\n") + "\n";
let bad = 0;
for (const k of Object.keys(solutions)) {
  const f = `${W}/final_${k}.ajisai`;
  writeFileSync(f, defText + examples[k] + "\n" + solutions[k] + "\n");
  let out;
  try { out = execFileSync("node", ["tools/lexicon-emergence/cli.mjs", "compute", "phase3-2026-10-15", "runs", "0", "E0C", f], { encoding: "utf8" }); }
  catch (e) { out = e.stdout || String(e); }
  const j = JSON.parse(out);
  if (j.status !== "ok") bad++;
  console.log(k.padEnd(24), j.status, JSON.stringify(j.stackDisplay), j.message || "");
}
console.log("solutions:", Object.keys(solutions).length, "non-ok:", bad);
