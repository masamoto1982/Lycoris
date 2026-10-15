// S0C working build: holds definitions and solutions, writes DEF file, verifies through compute.
import { execFileSync } from 'node:child_process';
import fs from 'node:fs';

const W = '/home/user/Ajisai/tools/lexicon-emergence/runs/algo-calibration-2026-10-15/solo/gen0/work/S0C';

export const DEFS = [
  { name: 'S0C.STARTS', note: 'Vector of positions where a run of equal adjacent values starts (position 0 included)',
    body: "'S0C.RV' BIND S0C.RV LENGTH 'S0C.RN' BIND S0C.RV -1 DROP 'S0C.RP' BIND S0C.RV 1 DROP 'S0C.RC' BIND S0C.RP S0C.RC LT NOT S0C.RP S0C.RC GT NOT AND NOT 'S0C.RF' BIND 0 S0C.RN 1 SUB RANGE 1 DROP 'S0C.RI' BIND S0C.RF S0C.RI 2 COLLECT ZIP [ 0 GET ] FILTER [ 1 GET ] MAP 'S0C.RJ' BIND [ 0 ] S0C.RJ CONCAT" },
  { name: 'S0C.RLE', note: 'Run-length encoding of the input Vector as a Vector of [ value count ] pairs',
    body: "'S0C.RV' BIND S0C.RV S0C.STARTS 'S0C.RS' BIND S0C.RV LENGTH 'S0C.RN' BIND S0C.RV S0C.RS GET S0C.RS 1 DROP S0C.RN 1 COLLECT CONCAT S0C.RS SUB 2 COLLECT ZIP" },
  { name: 'S0C.DIVS', note: 'Ascending divisors of a positive integer, as a Vector',
    body: "'S0C.DK' BIND 1 S0C.DK RANGE 'S0C.DD' BIND S0C.DK S0C.DD DIV 'S0C.DQ' BIND S0C.DQ S0C.DQ FLOOR SUB 0 GT NOT S0C.DD 2 COLLECT ZIP [ 0 GET ] FILTER [ 1 GET ] MAP" },
  { name: 'S0C.NEXT', note: 'One Collatz step from v: v/2 when v is even, 3v+1 when odd',
    body: "'S0C.NW' BIND S0C.NW 2 DIV S0C.NW 3 MUL 1 ADD S0C.NW S0C.NW 2 DIV FLOOR 2 MUL SUB 0 EQ SELECT" },
  { name: 'S0C.CSTEP', note: 'Collatz state step on [ value steps ]: holds at 1, otherwise advances value and counts one step',
    body: "'S0C.CA' BIND S0C.CA 0 GET 'S0C.CV' BIND S0C.CA 1 GET 'S0C.CS' BIND S0C.CV S0C.CV S0C.NEXT S0C.CV 1 EQ SELECT S0C.CS S0C.CS 1 ADD S0C.CV 1 EQ SELECT 2 COLLECT" },
  { name: 'S0C.BSTEP', note: 'Bracket-stack step: (state [ stack-code depth ], code) -> state; a closer that matches the top pops, otherwise pushes; codes 1 ( 2 [ 3 ) 4 ]',
    body: "'S0C.BX' BIND 'S0C.BST' BIND S0C.BX 2 GT S0C.BX 2 SUB S0C.BST 0 GET S0C.BST 0 GET 5 DIV FLOOR 5 MUL SUB EQ AND 'S0C.BP' BIND S0C.BST 0 GET 5 DIV FLOOR S0C.BST 0 GET 5 MUL S0C.BX ADD S0C.BP SELECT S0C.BST 1 GET 1 SUB S0C.BST 1 GET 1 ADD S0C.BP SELECT 2 COLLECT" },
  { name: 'S0C.BRKCODES', note: 'String of brackets to Vector of codes: ( 1, [ 2, ) 3, ] 4',
    body: "'(' '1' REPLACE '[' '2' REPLACE ')' '3' REPLACE ']' '4' REPLACE CHARS NUM" },
  { name: 'S0C.REDUCE', note: 'Final bracket-stack state [ stack-code depth ] of a code Vector; depth is the count left after cancelling matched pairs',
    body: "'S0C.RC' BIND S0C.RC 0 0 2 COLLECT [ S0C.BSTEP ] FOLD" },
];

export const SOL = {
  'runs.rle': 'S0C.RLE',
  'runs.upruns': "'S0C.UV' BIND S0C.UV -1 DROP 'S0C.UP' BIND S0C.UV 1 DROP 'S0C.UC' BIND S0C.UV LENGTH 'S0C.UN' BIND 0 S0C.UN 1 SUB RANGE -1 DROP 'S0C.UI' BIND S0C.UP S0C.UC LT NOT 'S0C.UT' BIND S0C.UT S0C.UI 2 COLLECT ZIP [ 0 GET ] FILTER [ 1 GET ] MAP 'S0C.UK' BIND [ -1 ] S0C.UK CONCAT S0C.UN 1 SUB 1 COLLECT CONCAT 'S0C.UB' BIND S0C.UB 1 DROP S0C.UB -1 DROP SUB 0 [ MAX ] FOLD",
  'runs.dedupe': "'S0C.X' BIND S0C.X S0C.X S0C.STARTS GET",
  'runs.second_largest': "'S0C.X' BIND S0C.X UNIQUE SORT 'S0C.SU' BIND S0C.SU -2 GET NIL S0C.SU LENGTH 1 GT SELECT",
  'runs.majority': "'S0C.V' BIND S0C.V TALLY 'S0C.T' BIND S0C.T KEYS 'S0C.K' BIND S0C.T VALUES 'S0C.CN' BIND S0C.V LENGTH 2 DIV 'S0C.H' BIND S0C.CN S0C.H GT 'S0C.M' BIND S0C.M S0C.K 2 COLLECT ZIP [ 0 GET ] FILTER [ 1 GET ] MAP 'S0C.R' BIND S0C.R 0 GET NIL S0C.R LENGTH 0 GT SELECT",
  'number.primes': "'S0C.PN' BIND 1 S0C.PN RANGE 1 DROP 'S0C.PK' BIND S0C.PK [ S0C.DIVS LENGTH 2 EQ ] MAP 'S0C.PM' BIND S0C.PM S0C.PK 2 COLLECT ZIP [ 0 GET ] FILTER [ 1 GET ] MAP",
  'number.divisors': 'S0C.DIVS',
  'number.binary': "'S0C.BN' BIND S0C.BN 2 0 6 RANGE POW DIV FLOOR 'S0C.BX' BIND S0C.BX S0C.BX 2 DIV FLOOR 2 MUL SUB REVERSE STR JOIN 'S0C.BS' BIND S0C.BS '1' SEARCH 'S0C.BQ' BIND S0C.BS CHARS S0C.BQ DROP JOIN",
  'number.collatz': "'S0C.CN' BIND 0 199 RANGE S0C.CN 0 2 COLLECT [ 0 MUL ADD S0C.CSTEP ] FOLD 1 GET",
  'number.digital_root': "'S0C.DN' BIND S0C.DN 1 SUB 'S0C.DM' BIND S0C.DM S0C.DM 9 DIV FLOOR 9 MUL SUB 1 ADD",
  'number.fib_index': "'S0C.FN' BIND [ 1 1 2 3 5 8 13 21 34 55 89 144 ] [ S0C.FN LT ] FILTER LENGTH 1 ADD",
  'brackets.depth': "S0C.BRKCODES 'S0C.C' BIND 1 -1 S0C.C 3 LT SELECT 0 [ ADD 0 MAX ] SCAN 0 [ MAX ] FOLD",
  'brackets.balanced': 'S0C.BRKCODES S0C.REDUCE 1 GET 0 EQ',
  'brackets.balanced_prefix': "S0C.BRKCODES 'S0C.C' BIND S0C.C LENGTH 'S0C.M' BIND 0 0 2 COLLECT 1 COLLECT S0C.C 0 0 2 COLLECT [ S0C.BSTEP ] SCAN CONCAT [ 1 GET ] MAP 'S0C.L' BIND 0 S0C.M RANGE 0 S0C.L 1 LT SELECT 0 [ MAX ] FOLD",
  'brackets.unmatched': 'S0C.BRKCODES S0C.REDUCE 1 GET',
};

export function defsText() {
  return DEFS.map(d => `[ ${d.body} ] '${d.name}' DEF`).join('\n') + '\n';
}

export function runProgram(src, file = `${W}/cur.ajisai`) {
  fs.writeFileSync(file, src);
  const out = execFileSync('node', ['tools/lexicon-emergence/cli.mjs', 'compute', 'algo-calibration-2026-10-15', 'solo', '0', 'S0C', file],
    { cwd: '/home/user/Ajisai', encoding: 'utf8', maxBuffer: 1 << 26 });
  return JSON.parse(out);
}
