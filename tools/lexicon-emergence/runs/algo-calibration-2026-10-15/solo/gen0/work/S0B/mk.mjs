import fs from 'fs';
const W='tools/lexicon-emergence/runs/algo-calibration-2026-10-15/solo/gen0';
const defs=[
['S0B.KEEP',"'M' BIND 'X' BIND X NIL M SELECT [ NIL? NOT ] FILTER","vector mask -> the elements of vector whose mask lane is TRUE"],
['S0B.COUNT',"'M' BIND 1 0 M SELECT 0 [ ADD ] FOLD","boolean vector -> number of TRUE lanes"],
['S0B.STARTS',"'V' BIND 0 V LENGTH 1 SUB RANGE 'I' BIND V 0 GET 1 SUB 1 COLLECT V -1 DROP CONCAT 'P' BIND I V P GT NOT V P LT NOT AND NOT S0B.KEEP","non-empty vector of numbers -> indices where a run of equal adjacent values starts"],
['S0B.DIVISORS',"'N' BIND 1 N RANGE 'J' BIND N J DIV 'Q' BIND J Q Q FLOOR SUB 0 GT NOT S0B.KEEP","positive integer -> ascending Vector of its divisors"],
['S0B.PASS',"'()' '' REPLACE '[]' '' REPLACE","string -> string with every () and [] pair deleted once"],
['S0B.REDUCE',"S0B.PASS S0B.PASS S0B.PASS S0B.PASS S0B.PASS S0B.PASS","bracket string (<=12 chars) -> string left after deleting matched pairs until none"],
].map(([name,body,note])=>({name,body,note}));
const sol={
'runs.rle':"'V' BIND V S0B.STARTS 'ST' BIND V ST GET ST 1 DROP V LENGTH 1 COLLECT CONCAT ST SUB 2 COLLECT ZIP",
'runs.upruns':"'V' BIND V 0 GET 1 SUB 1 COLLECT V -1 DROP CONCAT V 2 COLLECT ZIP 0 [ 'E' BIND 'A' BIND A 1 ADD 1 E 1 GET E 0 GET GT SELECT ] SCAN 0 [ MAX ] FOLD",
'runs.dedupe':"'V' BIND V V S0B.STARTS GET",
'runs.second_largest':"UNIQUE SORT -2 GET",
'runs.majority':"'V' BIND V TALLY 'R' BIND R KEYS NIL R VALUES 2 MUL V LENGTH GT SELECT SORT 0 GET",
'number.primes':"'N' BIND 1 N RANGE [ S0B.DIVISORS LENGTH 2 EQ ] FILTER",
'number.divisors':"S0B.DIVISORS",
'number.binary':"'N' BIND 0 6 RANGE 'K' BIND 2 K POW 'P' BIND N P DIV FLOOR N P 2 MUL DIV FLOOR 2 MUL SUB 'B' BIND B P N GT NOT S0B.KEEP REVERSE STR JOIN",
'number.collatz':"'N' BIND 1 130 RANGE N [ 'E' BIND 'X' BIND X 3 MUL 1 ADD X 2 DIV X 2 DIV X 2 DIV FLOOR SUB 0 GT SELECT 'NX' BIND 1 NX X 1 EQ SELECT ] SCAN 'T' BIND N 1 COLLECT T CONCAT 1 SUB 0 GT S0B.COUNT",
'number.digital_root':"'N' BIND N N 1 SUB 9 DIV FLOOR 9 MUL SUB",
'number.fib_index':"'N' BIND [ 1 1 2 3 5 8 13 21 34 55 89 ] N LT S0B.COUNT 1 ADD",
'brackets.depth':"CHARS [ 'C' BIND 1 -1 '([)]' C SEARCH 2 LT SELECT ] MAP 0 [ ADD ] SCAN 0 [ MAX ] FOLD",
'brackets.balanced':"S0B.REDUCE CHARS LENGTH 0 EQ",
'brackets.balanced_prefix':"'S' BIND 0 S CHARS LENGTH RANGE [ 'L' BIND S CHARS L TAKE JOIN S0B.REDUCE CHARS LENGTH 0 EQ ] MAP 'M' BIND 0 S CHARS LENGTH RANGE 0 M SELECT 0 [ MAX ] FOLD",
'brackets.unmatched':"S0B.REDUCE CHARS LENGTH",
};
fs.writeFileSync(`${W}/submissions/S0B.json`,JSON.stringify({agent:'S0B',generation:0,condition:'solo',definitions:defs,solutions:sol},null,2));
