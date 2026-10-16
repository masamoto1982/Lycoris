import fs from 'node:fs';
import { execFileSync } from 'node:child_process';
const ROOT = '/home/user/Ajisai';
const W = ROOT + '/tools/lexicon-emergence/runs/phase3-2026-10-15/runs/gen2/work/E2C';
const SUB = ROOT + '/tools/lexicon-emergence/runs/phase3-2026-10-15/runs/gen2/submissions/E2C.json';
const ORDERS = "'{\"orders\":[{\"sku\":\"a1\",\"qty\":2,\"price\":1.5},{\"sku\":\"b2\",\"qty\":1,\"price\":4,\"tags\":[\"gift\"]},{\"sku\":\"a1\",\"qty\":3,\"price\":1.5,\"tags\":[\"bulk\",\"gift\"]},{\"sku\":\"c3\",\"qty\":5}]}'";
const MESSY = "'  The cat saw THE dog   and the Cat  '";
const definitions = [
  { name: 'E2C.REDUCE', body: "'S' BIND 1 6 RANGE S [ 'K' BIND '()' '' REPLACE '[]' '' REPLACE ] FOLD", note: 'String -> String with every adjacent () and [] pair removed until none is left (six passes suffice for 12 characters)' },
  { name: 'E2C.BALANCED', body: "E2C.REDUCE '' EQ", note: 'String of brackets -> TRUE when the reduction leaves nothing, else FALSE' },
  { name: 'E2C.REVENUE', body: "G0C.ORDERS [ 'O' BIND O 'qty' GET O 'price' GET MUL 'X' BIND 0 X X NIL? SELECT ] MAP 0 [ ADD ] FOLD", note: 'orders JSON String -> total of qty times price, an order without a price counting 0' },
  { name: 'E2C.QTYSKU', body: "G0C.ORDERS 'OS' BIND OS [ 'sku' GET ] MAP OS [ 'qty' GET ] MAP GROUP 'G' BIND G KEYS G VALUES [ 0 [ ADD ] FOLD ] MAP RECORD", note: 'orders JSON String -> Record from sku to summed qty, skus in first-appearance order' },
];
const solutions = {
  'runs.rle': "G0D.RUNS [ 'R' BIND R 0 GET R LENGTH 2 COLLECT ] MAP",
  'runs.upruns': "'V' BIND V V G0D.SHIFT GT 0 [ 'E' BIND 'A' BIND A 1 ADD 1 E SELECT ] SCAN 0 [ MAX ] FOLD",
  'runs.dedupe': "G0D.RUNS [ 0 GET ] MAP",
  'runs.second_largest': "UNIQUE SORT -2 TAKE 0 GET",
  'runs.majority': "'V' BIND V TALLY 'T' BIND T U1D.TOPKEY 'K' BIND K NIL V LENGTH T K GET 2 MUL LT SELECT",
  'number.primes': "'N' BIND 1 N RANGE [ G0D.DIVISORS LENGTH 2 EQ ] FILTER",
  'number.divisors': "G0D.DIVISORS",
  'number.binary': "'N' BIND N [ 1 2 4 8 16 32 64 128 ] DIV FLOOR 'Q' BIND Q 7 TAKE Q 1 DROP 2 MUL SUB REVERSE STR JOIN NUM STR",
  'number.collatz': "'N' BIND 1 300 RANGE N 0 2 COLLECT [ 'E' BIND 'S' BIND S 0 GET 'X' BIND S 1 GET 'C' BIND X 2 DIV FLOOR 'H' BIND X 3 MUL 1 ADD 'T' BIND H T H 2 MUL X EQ SELECT 'NX' BIND X NX X 1 EQ SELECT C C 1 ADD X 1 EQ SELECT 2 COLLECT ] FOLD 1 GET",
  'number.digital_root': "1 SUB 'M' BIND M M 9 DIV FLOOR 9 MUL SUB 1 ADD",
  'number.fib_index': "'N' BIND [ 1 1 2 3 5 8 13 21 34 55 89 144 ] [ N LT ] FILTER LENGTH 1 ADD",
  'brackets.depth': "CHARS [ 'C' BIND [ '(' '[' ')' ']' ] [ 1 1 -1 -1 ] RECORD C GET ] MAP 0 [ ADD ] SCAN 0 [ MAX ] FOLD",
  'brackets.balanced': "E2C.BALANCED",
  'brackets.balanced_prefix': "'S' BIND S CHARS LENGTH 'N' BIND 0 N RANGE [ 'L' BIND S CHARS L TAKE JOIN E2C.BALANCED ] FILTER 0 [ MAX ] FOLD",
  'brackets.unmatched': "E2C.REDUCE CHARS LENGTH",
  'sensor.present_mean': "'V' BIND V [ NIL? NOT ] FILTER 'P' BIND P 0 [ ADD ] FOLD P LENGTH DIV",
  'sensor.filled': "'V' BIND V [ NIL? NOT ] FILTER 0 GET 'F' BIND V F [ 'E' BIND 'A' BIND E A E NIL? NOT SELECT ] SCAN",
  'sensor.gap_count': "0 0 2 COLLECT [ 'E' BIND 'S' BIND E NIL? 'M' BIND 1 0 M SELECT S 1 GET 1 ADD S 1 GET M S 0 GET 0 EQ AND SELECT 2 COLLECT ] FOLD 1 GET",
  'sensor.longest_gap': "0 [ 'E' BIND 'A' BIND A 1 ADD 0 E NIL? SELECT ] SCAN 0 [ MAX ] FOLD",
  'sensor.ratios': "'V' BIND V 1 DROP V -1 DROP DIV",
  'sensor.scaled': "'V' BIND V [ NIL? NOT ] FILTER 'P' BIND P 1 DROP P 0 GET [ MAX ] FOLD 'MX' BIND V MX DIV",
  'orders.revenue': "E2C.REVENUE",
  'orders.report': "E2C.REVENUE 2 FORMAT",
  'orders.unpriced': "G0C.ORDERS [ 'O' BIND O 'price' GET NIL? ] FILTER [ 'sku' GET ] MAP UNIQUE",
  'orders.qty_by_sku': "E2C.QTYSKU",
  'orders.top_sku': "E2C.QTYSKU U1D.TOPKEY",
  'orders.tag_counts': "G0C.ORDERS [ 'tags' GET NIL? NOT ] FILTER [ 'tags' GET UNIQUE ] MAP FLATTEN TALLY",
  'messy.count': "G0D.WORDS LENGTH",
  'messy.normalize': "G0D.WORDS [ LOWER ] MAP G0A.UNWORDS",
  'messy.distinct': "G0D.WORDS [ LOWER ] MAP UNIQUE LENGTH",
  'messy.top_word': "G0D.WORDS [ LOWER ] MAP TALLY U1D.TOPKEY",
  'messy.title': "G0D.WORDS [ 'W' BIND W CHARS 'C' BIND C 0 GET UPPER C 1 DROP JOIN LOWER 2 COLLECT JOIN ] MAP G0A.UNWORDS",
  'messy.sorted_unique': "G0D.WORDS [ LOWER ] MAP UNIQUE SORT G0A.UNWORDS",
};
const example = (id) => {
  const k = id.split('.')[0];
  return { runs: '[ 1 1 2 2 2 3 1 1 ]', number: '10', brackets: "'([]())'", sensor: '[ 3 NIL 5 1 NIL NIL 4 ]', orders: ORDERS, messy: MESSY }[k];
};
const sub = { agent: 'E2C', generation: 2, condition: 'runs', definitions, solutions };
fs.writeFileSync(SUB, JSON.stringify(sub, null, 2) + '\n');
const defText = definitions.map(d => `[ ${d.body} ] '${d.name}' DEF`).join('\n');
fs.writeFileSync(W + '/defs_final.txt', defText + '\n');
const words = 'G0C.ORDERS,G0D.WORDS,G0D.SHIFT,G0D.RUNS,G0A.UNWORDS,U1D.ARGMAX,U1D.TOPKEY,G0D.DIVISORS';
for (const [id, code] of Object.entries(solutions)) {
  fs.writeFileSync(W + '/graded.ajisai', `${defText}\n${example(id)} ${code}\n`);
  let out;
  try {
    out = execFileSync('node', ['tools/lexicon-emergence/cli.mjs', 'compute', 'phase3-2026-10-15', 'runs', '2', 'E2C', 'tools/lexicon-emergence/runs/phase3-2026-10-15/runs/gen2/work/E2C/graded.ajisai', '--words', words], { cwd: ROOT, encoding: 'utf8' });
  } catch (e) { out = e.stdout || String(e); }
  const j = JSON.parse(out);
  console.log(id.padEnd(22), j.status, JSON.stringify(j.stackDisplay), j.message || '');
}
console.log('wrote', SUB, 'solutions:', Object.keys(solutions).length);
