import fs from 'fs';
import {execSync} from 'child_process';
const W='tools/lexicon-emergence/runs/hard-calibration-2026-10-15/solo/gen0/work/S0B';
const defs=[
['S0B.PRESENT',"[ NIL? NOT ] FILTER","Keep the present (non-NIL) elements of a Vector."],
['S0B.SUM',"0 [ ADD ] FOLD","Sum of a Vector of numbers (0 if empty)."],
['S0B.GAPRUNS',"[ NIL? ] MAP 0 [ 'E' BIND 'A' BIND A 1 ADD 0 E SELECT ] SCAN","Per element, the length of the run of NILs ending there (0 where present)."],
['S0B.ORDERS',"JSON-DECODE 'orders' GET","JSON String -> Vector of order Records."],
['S0B.COL',"'F' BIND [ F GET ] MAP","Vector of Records, key -> Vector of that key's values (NIL where missing)."],
['S0B.REVENUE',"S0B.ORDERS 'O' BIND O 'qty' S0B.COL O 'price' S0B.COL MUL S0B.PRESENT S0B.SUM","Total qty*price over priced orders, from JSON String."],
['S0B.QTYREC',"S0B.ORDERS 'O' BIND O 'sku' S0B.COL O 'qty' S0B.COL GROUP 'G' BIND G KEYS G VALUES [ S0B.SUM ] MAP RECORD","Record sku -> summed qty, from JSON String."],
['S0B.WORDS',"LOWER ' ' TOKENIZE [ '' EQ NOT ] FILTER","String -> Vector of lowercased words, ignoring extra spaces."],
['S0B.JOINSP',"[ 'W' BIND W ' ' 2 COLLECT JOIN ] MAP JOIN TRIM","Vector of Strings -> one String joined by single spaces."],
['S0B.CAP',"'W' BIND W CHARS 'C' BIND C 0 GET UPPER C 1 DROP JOIN 2 COLLECT JOIN","Uppercase the first character of a lowercase word."],
['S0B.TOPKEY',"'R' BIND R KEYS R VALUES -1 MUL ORDER 0 GET GET","Record key with the largest value, first on ties."],
];
const sol={
'sensor.present_mean':"'V' BIND V S0B.PRESENT 'P' BIND P S0B.SUM P LENGTH DIV",
'sensor.filled':"'V' BIND V S0B.PRESENT 0 GET 'F' BIND V F [ 'E' BIND 'A' BIND A E E NIL? SELECT ] SCAN",
'sensor.gap_count':"S0B.GAPRUNS [ 1 EQ ] FILTER LENGTH",
'sensor.longest_gap':"S0B.GAPRUNS 0 [ MAX ] FOLD",
'sensor.ratios':"'V' BIND V 1 DROP V -1 DROP DIV",
'sensor.scaled':"'V' BIND V S0B.PRESENT 'P' BIND V P P 0 GET [ MAX ] FOLD DIV",
'orders.revenue':"S0B.REVENUE",
'orders.report':"S0B.REVENUE 2 FORMAT",
'orders.unpriced':"S0B.ORDERS [ 'price' GET NIL? ] FILTER 'sku' S0B.COL UNIQUE",
'orders.qty_by_sku':"S0B.QTYREC",
'orders.top_sku':"S0B.QTYREC S0B.TOPKEY",
'orders.tag_counts':"S0B.ORDERS 'tags' S0B.COL S0B.PRESENT [ UNIQUE ] MAP FLATTEN TALLY",
'messy.count':"S0B.WORDS LENGTH",
'messy.normalize':"S0B.WORDS S0B.JOINSP",
'messy.distinct':"S0B.WORDS UNIQUE LENGTH",
'messy.top_word':"S0B.WORDS TALLY S0B.TOPKEY",
'messy.title':"S0B.WORDS [ S0B.CAP ] MAP S0B.JOINSP",
'messy.sorted_unique':"S0B.WORDS UNIQUE SORT S0B.JOINSP",
};
const O1=(o)=>JSON.stringify(JSON.stringify(o));
const ord=(s)=>"'"+s+"'";
const inputs={
sensor:["[ 3 NIL 5 1 NIL NIL 4 ]","[ NIL 2 ]","[ -3/2 ]","[ 0 NIL NIL -4 2 NIL ]","[ NIL NIL 7 ]"],
orders:[ord('{"orders":[{"sku":"a1","qty":2,"price":1.5},{"sku":"b2","qty":1,"price":4,"tags":["gift"]},{"sku":"a1","qty":3,"price":1.5,"tags":["bulk","gift"]},{"sku":"c3","qty":5}]}'),
 ord('{"orders":[{"sku":"z","qty":0.5,"tags":[]},{"sku":"z","qty":1,"price":0.01,"tags":["x","x"]}]}'),
 ord('{"orders":[{"sku":"q","qty":4,"price":2}]}')],
messy:[ord('  The cat saw THE dog   and the Cat  '),ord('Zed'),ord(' b a  B A c ')],
};
const defl=defs.map(([n,b])=>`[ ${b} ] '${n}' DEF`).join('\n');
const only=process.argv[2];
for(const [id,code] of Object.entries(sol)){
  const fam=id.split('.')[0];
  if(only && !id.startsWith(only)) continue;
  for(const inp of inputs[fam]){
    fs.writeFileSync(W+'/try.ajisai',`${defl}\n${inp}\n${code}\n`);
    let out;
    try{out=execSync(`node tools/lexicon-emergence/cli.mjs compute hard-calibration-2026-10-15 solo 0 S0B ${W}/try.ajisai`).toString();}catch(e){out=e.stdout.toString();}
    const j=JSON.parse(out);
    console.log(id,inp.slice(0,40),'=>',j.status,JSON.stringify(j.stackDisplay),j.message||'');
  }
}
if(process.argv[3]==='write'){
  fs.writeFileSync('tools/lexicon-emergence/runs/hard-calibration-2026-10-15/solo/gen0/submissions/S0B.json',JSON.stringify({agent:'S0B',generation:0,condition:'solo',definitions:defs.map(([name,body,note])=>({name,body,note})),solutions:sol},null,2));
}
