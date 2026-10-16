import fs from 'fs';
const defs=[
["E1B.CAP","LOWER CHARS 'TW' BIND TW 0 GET UPPER TW 1 DROP JOIN 2 COLLECT JOIN","Word (String) -> first letter upper, rest lower"],
["E1B.CSTEP","'CX' BIND CX 2 DIV CX 3 MUL 1 ADD CX CX 2 DIV FLOOR 2 MUL SUB 0 EQ SELECT 'CY' BIND 1 CY CX 1 EQ SELECT","one Collatz step on a positive integer, staying at 1 once reached"],
["E1B.REVENUE","G0C.ORDERS [ 'ORD' BIND 0 ORD 'qty' GET ORD 'price' GET MUL ORD 'price' GET NIL? SELECT ] MAP 0 [ ADD ] FOLD","orders JSON String -> total of qty*price over priced orders"],
["E1B.QTYSKU","G0C.ORDERS 'OL' BIND OL [ 'sku' GET ] MAP OL [ 'qty' GET ] MAP GROUP 'QG' BIND QG KEYS QG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD","orders JSON String -> Record sku -> summed qty, first-appearance order"]];
const J1=`'{"orders":[{"sku":"a1","qty":2,"price":1.5},{"sku":"b2","qty":1,"price":4,"tags":["gift"]},{"sku":"a1","qty":3,"price":1.5,"tags":["bulk","gift"]},{"sku":"c3","qty":5}]}'`;
const S={
"runs.rle":["[ 1 1 2 2 2 3 1 1 ]","G0D.RUNS [ 'RR' BIND RR 0 GET RR LENGTH 2 COLLECT ] MAP"],
"runs.upruns":["[ 1 1 2 2 2 3 1 1 ]","'UV' BIND UV UV G0D.SHIFT GT 0 [ 'UE' BIND 'UA' BIND UA 1 ADD 1 UE SELECT ] SCAN 0 [ MAX ] FOLD"],
"runs.dedupe":["[ 1 1 2 2 2 3 1 1 ]","G0D.RUNS [ 0 GET ] MAP"],
"runs.second_largest":["[ 1 1 2 2 2 3 1 1 ]","UNIQUE SORT -2 GET"],
"runs.majority":["[ 1 1 2 2 2 3 1 1 ]","'MV' BIND MV LENGTH 'MN' BIND MV TALLY 'MT' BIND MT VALUES 'MC' BIND MT KEYS MC ORDER -1 GET GET NIL MC SORT -1 GET 2 MUL MN GT SELECT"],
"number.primes":["10","'PN' BIND 1 PN RANGE [ G0D.DIVISORS LENGTH 2 EQ ] FILTER"],
"number.divisors":["10","G0D.DIVISORS"],
"number.binary":["10","'BN' BIND [ 32 16 8 4 2 1 ] [ BN GT NOT ] FILTER [ 'BP' BIND BN BP DIV FLOOR 'BQ' BIND BQ BQ 2 DIV FLOOR 2 MUL SUB ] MAP STR JOIN"],
"number.collatz":["10","'CN' BIND CN 1 COLLECT 1 130 RANGE CN [ 'CE' BIND E1B.CSTEP ] SCAN CONCAT [ 1 EQ NOT ] FILTER LENGTH"],
"number.digital_root":["10","'DN' BIND DN 1 SUB 'DX' BIND DX DX 9 DIV FLOOR 9 MUL SUB 1 ADD"],
"number.fib_index":["10","'FN' BIND [ 1 1 2 3 5 8 13 21 34 55 89 ] [ FN LT ] FILTER LENGTH 1 ADD"],
"brackets.depth":["'([]())'","CHARS [ 'BC' BIND [ '(' '[' ')' ']' ] [ 1 1 -1 -1 ] RECORD BC GET ] MAP 0 [ ADD 0 MAX ] SCAN 0 [ MAX ] FOLD"],
"brackets.balanced":["'([]())'","CHARS '#' [ G0A.BSTEP ] FOLD '#' EQ"],
"brackets.balanced_prefix":["'([]())'","'BQS' BIND BQS CHARS LENGTH 'BQN' BIND [ '#' ] BQS CHARS '#' [ G0A.BSTEP ] SCAN CONCAT [ '#' EQ ] MAP 'BQB' BIND 0 BQN RANGE 0 BQB SELECT 0 [ MAX ] FOLD"],
"brackets.unmatched":["'([]())'","G0B.REDUCE CHARS LENGTH"],
"sensor.present_mean":["[ 3 NIL 5 1 NIL NIL 4 ]","[ NIL? NOT ] FILTER 'SP' BIND SP 0 [ ADD ] FOLD SP LENGTH DIV"],
"sensor.filled":["[ 3 NIL 5 1 NIL NIL 4 ]","'FV' BIND FV FV [ NIL? NOT ] FILTER 0 GET [ 'FE' BIND 'FA' BIND FA FE FE NIL? SELECT ] SCAN"],
"sensor.gap_count":["[ 3 NIL 5 1 NIL NIL 4 ]","[ NIL? ] MAP 'GB' BIND 1 0 GB SELECT 'GM' BIND [ 0 ] GM CONCAT 'GP' BIND GP 1 DROP GP -1 DROP SUB [ 1 EQ ] FILTER LENGTH"],
"sensor.longest_gap":["[ 3 NIL 5 1 NIL NIL 4 ]","[ NIL? ] MAP 0 [ 'LE' BIND 'LA' BIND LA 1 ADD 0 LE SELECT ] SCAN 0 [ MAX ] FOLD"],
"sensor.ratios":["[ 3 NIL 5 1 NIL NIL 4 ]","'RV' BIND RV 1 DROP RV -1 DROP DIV"],
"sensor.scaled":["[ 3 NIL 5 1 NIL NIL 4 ]","'SV' BIND SV [ NIL? NOT ] FILTER SORT -1 GET 'SM' BIND SV SM DIV"],
"orders.revenue":[J1,"E1B.REVENUE"],
"orders.report":[J1,"E1B.REVENUE 2 FORMAT"],
"orders.unpriced":[J1,"G0C.ORDERS [ 'price' GET NIL? ] FILTER [ 'sku' GET ] MAP UNIQUE"],
"orders.qty_by_sku":[J1,"E1B.QTYSKU"],
"orders.top_sku":[J1,"E1B.QTYSKU 'QR' BIND QR KEYS QR VALUES -1 MUL ORDER 0 GET GET"],
"orders.tag_counts":[J1,"G0C.ORDERS [ 'tags' GET NIL? NOT ] FILTER [ 'tags' GET UNIQUE ] MAP FLATTEN TALLY"],
"messy.count":["'  The cat saw THE dog   and the Cat  '","G0D.WORDS LENGTH"],
"messy.normalize":["'  The cat saw THE dog   and the Cat  '","LOWER G0D.WORDS G0A.UNWORDS"],
"messy.distinct":["'  The cat saw THE dog   and the Cat  '","LOWER G0D.WORDS UNIQUE LENGTH"],
"messy.top_word":["'  The cat saw THE dog   and the Cat  '","LOWER G0D.WORDS TALLY 'MT' BIND MT KEYS MT VALUES -1 MUL ORDER 0 GET GET"],
"messy.title":["'  The cat saw THE dog   and the Cat  '","G0D.WORDS [ E1B.CAP ] MAP G0A.UNWORDS"],
"messy.sorted_unique":["'  The cat saw THE dog   and the Cat  '","LOWER G0D.WORDS UNIQUE SORT G0A.UNWORDS"],
};
const sub={agent:"E1B",generation:1,condition:"runs",definitions:defs.map(([name,body,note])=>({name,body,note})),solutions:Object.fromEntries(Object.entries(S).map(([k,v])=>[k,v[1]]))};
fs.writeFileSync('../../submissions/E1B.json',JSON.stringify(sub,null,2));
const dl=defs.map(([n,b])=>`[ ${b} ] '${n}' DEF`).join('\n');
fs.mkdirSync('final',{recursive:true});
for(const [k,[inp,sol]] of Object.entries(S)) fs.writeFileSync(`final/${k}.ajisai`,`${dl}\n${inp}\n${sol}\n`);
