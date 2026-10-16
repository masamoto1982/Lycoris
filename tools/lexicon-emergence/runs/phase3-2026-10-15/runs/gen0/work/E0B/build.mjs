import fs from 'fs';
import {execFileSync} from 'child_process';
const W='tools/lexicon-emergence/runs/phase3-2026-10-15/runs/gen0/work/E0B';
const defs=[
['E0B.STARTS',"'V' BIND V 0 MUL 1 ADD 0 [ ADD ] SCAN 1 SUB 'I' BIND V 0 GET 1 SUB 1 COLLECT V -1 DROP CONCAT 'P' BIND V P SUB 'D' BIND D D MUL 0 GT 'F' BIND I F 2 COLLECT ZIP [ 1 GET ] FILTER [ 0 GET ] MAP",'Vector of ints -> indices where a new run of equal adjacent values begins'],
['E0B.DIVISORS',"'N' BIND 1 N RANGE [ 'D' BIND N N D DIV FLOOR D MUL SUB 1 LT ] FILTER",'Positive integer -> ascending Vector of its divisors'],
['E0B.REDUCE',"'S' BIND 0 6 RANGE S [ 'E' BIND '()' '' REPLACE '[]' '' REPLACE ] FOLD",'Bracket String -> String left after repeatedly deleting adjacent () and [] pairs'],
['E0B.JOINSP',"[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM",'Vector of Strings -> one String joined with single spaces'],
['E0B.WORDS',"LOWER ' ' TOKENIZE [ '' EQ NOT ] FILTER",'String -> Vector of its words in lower case, ignoring extra spaces'],
['E0B.QTYSKU',"JSON-DECODE 'orders' GET 'OS' BIND OS [ 'sku' GET ] MAP OS [ 'qty' GET ] MAP GROUP 'G' BIND G KEYS G VALUES [ 0 [ ADD ] FOLD ] MAP RECORD",'Orders JSON String -> Record sku -> summed qty, in first-appearance order'],
];
const REV="JSON-DECODE 'orders' GET [ 'O' BIND O 'qty' GET O 'price' GET MUL 0 O 'price' HAS? SELECT ] MAP 0 [ ADD ] FOLD";
const sol={
'runs.rle':"'V' BIND V E0B.STARTS 'S' BIND V S GET S 1 DROP V LENGTH 1 COLLECT CONCAT S SUB 2 COLLECT ZIP",
'runs.upruns':"'V' BIND V 1 DROP V -1 DROP SUB 0 GT 'F' BIND 1 0 F SELECT 0 [ 'M' BIND 1 ADD M MUL ] SCAN 0 [ MAX ] FOLD 1 ADD",
'runs.dedupe':"'V' BIND V V E0B.STARTS GET",
'runs.second_largest':"UNIQUE SORT -2 GET",
'runs.majority':"'V' BIND V TALLY 'R' BIND R KEYS 'K' BIND R VALUES 'C' BIND C ORDER -1 GET 'J' BIND C J GET 'MX' BIND K J GET NIL MX 2 MUL V LENGTH GT SELECT",
'number.primes':"'N' BIND 1 N RANGE [ E0B.DIVISORS LENGTH 2 EQ ] FILTER",
'number.divisors':"E0B.DIVISORS",
'number.binary':"'N' BIND 0 5 RANGE [ 'K' BIND 2 K POW N GT NOT ] FILTER REVERSE [ 'K' BIND N 2 K POW DIV FLOOR 'Q' BIND Q Q 2 DIV FLOOR 2 MUL SUB ] MAP STR JOIN",
'number.collatz':"'N' BIND 0 149 RANGE N [ 'E' BIND 'X' BIND 1 X 2 DIV X 3 MUL 1 ADD X X 2 DIV FLOOR 2 MUL SUB 0 EQ SELECT X 1 EQ SELECT ] SCAN 'SQ' BIND N 1 COLLECT SQ CONCAT 1 SUB 1 MIN 0 [ ADD ] FOLD",
'number.digital_root':"'N' BIND N N 1 SUB 9 DIV FLOOR 9 MUL SUB",
'number.fib_index':"'N' BIND [ 1 1 2 3 5 8 13 21 34 55 89 ] [ N LT ] FILTER LENGTH 1 ADD",
'brackets.depth':"'S' BIND S '[' '(' REPLACE ']' ')' REPLACE CHARS [ 'C' BIND 1 -1 C '(' EQ SELECT ] MAP 0 [ ADD 0 MAX ] SCAN 0 [ MAX ] FOLD",
'brackets.balanced':"E0B.REDUCE '' EQ",
'brackets.balanced_prefix':"'S' BIND S CHARS LENGTH 'N' BIND 0 N RANGE [ 'L' BIND L 0 S CHARS L TAKE JOIN E0B.REDUCE '' EQ SELECT ] MAP 0 [ MAX ] FOLD",
'brackets.unmatched':"E0B.REDUCE CHARS LENGTH",
'sensor.present_mean':"'V' BIND V [ NIL? NOT ] FILTER 'P' BIND P 0 [ ADD ] FOLD P LENGTH DIV",
'sensor.filled':"'V' BIND V V [ NIL? NOT ] FILTER 0 GET [ 'E' BIND 'A' BIND A E E NIL? SELECT ] SCAN",
'sensor.gap_count':"'V' BIND 1 0 V [ NIL? ] MAP SELECT 'M' BIND 0 1 COLLECT M -1 DROP CONCAT 'Q' BIND M Q SUB 0 MAX 0 [ ADD ] FOLD",
'sensor.longest_gap':"'V' BIND 1 0 V [ NIL? ] MAP SELECT 0 [ 'X' BIND 1 ADD X MUL ] SCAN 0 [ MAX ] FOLD",
'sensor.ratios':"'V' BIND V 1 DROP V -1 DROP DIV",
'sensor.scaled':"'V' BIND V [ NIL? NOT ] FILTER 'P' BIND V P P 0 GET [ MAX ] FOLD DIV",
'orders.revenue':REV,
'orders.report':REV+" 2 FORMAT",
'orders.unpriced':"JSON-DECODE 'orders' GET [ 'O' BIND O 'price' HAS? NOT ] FILTER [ 'sku' GET ] MAP UNIQUE",
'orders.qty_by_sku':"E0B.QTYSKU",
'orders.top_sku':"E0B.QTYSKU 'R' BIND R KEYS R VALUES -1 MUL ORDER 0 GET GET",
'orders.tag_counts':"JSON-DECODE 'orders' GET [ 'O' BIND O 'tags' HAS? ] FILTER [ 'tags' GET UNIQUE ] MAP FLATTEN TALLY",
'messy.count':"E0B.WORDS LENGTH",
'messy.normalize':"E0B.WORDS E0B.JOINSP",
'messy.distinct':"E0B.WORDS UNIQUE LENGTH",
'messy.top_word':"E0B.WORDS 'W' BIND W TALLY 'R' BIND R KEYS R VALUES -1 MUL ORDER 0 GET GET",
'messy.title':"E0B.WORDS [ 'W' BIND W CHARS 0 GET UPPER W CHARS 1 DROP JOIN 2 COLLECT JOIN ] MAP E0B.JOINSP",
'messy.sorted_unique':"E0B.WORDS UNIQUE SORT E0B.JOINSP",
};
const OJ=`'{"orders":[{"sku":"a1","qty":2,"price":1.5},{"sku":"b2","qty":1,"price":4,"tags":["gift"]},{"sku":"a1","qty":3,"price":1.5,"tags":["bulk","gift"]},{"sku":"c3","qty":5}]}'`;
const inputs={runs:'[ 1 1 2 2 2 3 1 1 ]',number:'10',brackets:"'([]())'",sensor:'[ 3 NIL 5 1 NIL NIL 4 ]',orders:OJ,messy:"'  The cat saw THE dog   and the Cat  '"};
const out={agent:'E0B',generation:0,condition:'runs',definitions:defs.map(([name,body,note])=>({name,body,note})),solutions:sol};
fs.mkdirSync('tools/lexicon-emergence/runs/phase3-2026-10-15/runs/gen0/submissions',{recursive:true});
fs.writeFileSync('tools/lexicon-emergence/runs/phase3-2026-10-15/runs/gen0/submissions/E0B.json',JSON.stringify(out,null,1));
for(const [id,code] of Object.entries(sol)){
  const prog=out.definitions.map(d=>`[ ${d.body} ] '${d.name}' DEF`).join('\n')+'\n'+inputs[id.split('.')[0]]+'\n'+code+'\n';
  fs.writeFileSync(W+'/try.ajisai',prog);
  const r=JSON.parse(execFileSync('node',['tools/lexicon-emergence/cli.mjs','compute','phase3-2026-10-15','runs','0','E0B',W+'/try.ajisai'],{encoding:'utf8'}));
  console.log(id,r.status,JSON.stringify(r.stackDisplay),r.message||'');
}
