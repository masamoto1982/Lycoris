import fs from 'fs';
import {execSync} from 'child_process';
const W='tools/lexicon-emergence/runs/phase2-2026-10-15/bottleneck/gen1/work/G1B';
const defs=[
 ['G1B.ABSV',"'D' BIND D D -1 MUL MAX","elementwise absolute value of a numeric Vector"],
 ['G1B.MAD',"'V' BIND V G0A.MEDIAN 'M' BIND V M SUB G1B.ABSV G0A.MEDIAN","median absolute deviation of a numeric Vector"],
 ['G1B.MOVING3',"'V' BIND V LENGTH 2 SUB 'K' BIND V K TAKE V 1 DROP K TAKE ADD V 2 DROP ADD 3 DIV","means of every three consecutive numbers"],
 ['G1B.RUNMAX',"'V' BIND V V 0 GET [ MAX ] SCAN","running maximum of a numeric Vector"],
 ['G1B.MODE',"SORT TALLY G0B.TOPKEY","most frequent number, smallest on ties"],
 ['G1B.TRIMMED',"SORT 1 DROP -1 DROP G0A.MEAN","mean after dropping one min and one max"],
 ['G1B.LONGEST',"G0A.WORDS 'W' BIND W W [ CHARS LENGTH ] MAP G0A.ARGMAX GET","longest word of a String, first on ties"],
 ['G1B.CAP',"CHARS 'C' BIND C 0 GET UPPER 1 COLLECT C 1 DROP CONCAT JOIN","capitalize the first letter of one word"],
 ['G1B.DEBTORS',"G0A.BYNAME 'R' BIND R KEYS R VALUES 2 COLLECT ZIP [ 1 GET 0 LT ] FILTER [ 0 GET ] MAP","names whose summed amounts are negative"],
 ['G1B.LARGEST',"'V' BIND V V ZIP 1 GET G0A.ARGMAX GET","entry with the largest amount, first on ties"],
];
const sol={
 'series.median':'G0A.MEDIAN','series.mad':'G1B.MAD','series.moving3':'G1B.MOVING3','series.runmax':'G1B.RUNMAX','series.mode':'G1B.MODE','series.trimmed':'G1B.TRIMMED',
 'prose.longest':'G1B.LONGEST','prose.freq_top':'G0A.WORDS TALLY G0B.TOPKEY','prose.distinct_sorted':'G0A.WORDS UNIQUE SORT G0A.UNWORDS','prose.reverse_words':'G0A.WORDS REVERSE G0A.UNWORDS',
 'prose.capitalize':"G0A.WORDS [ G1B.CAP ] MAP G0A.UNWORDS",'prose.initials_of_long':"G0A.WORDS [ CHARS LENGTH 3 GT ] FILTER [ CHARS 0 GET UPPER ] MAP JOIN",
 'ledger.total':'ZIP 1 GET G0A.SUM','ledger.by_name':'G0A.BYNAME','ledger.top_name':'G0A.BYNAME G0B.TOPKEY','ledger.debtors':'G1B.DEBTORS','ledger.count_names':'ZIP 0 GET UNIQUE LENGTH','ledger.largest_entry':'G1B.LARGEST',
};
const ins={series:["[ 3 1 4 1 5 9 2 6 ]","[ 5 -2 7/2 7/2 1 ]","[ 1 2 3 ]"],
 prose:["'the quick brown fox jumps over the lazy dog the end'","'hello'","'a bb cc a'"],
 ledger:["[ [ 'ann' 30 ] [ 'bob' -5 ] [ 'ann' 12 ] [ 'cy' -20 ] [ 'bob' 2 ] ]","[ [ 'x' 5 ] ]","[ [ 'a' 1 ] [ 'b' 1 ] [ 'a' 0 ] ]"]};
const defl=defs.map(d=>`[ ${d[1]} ] '${d[0]}' DEF`).join('\n');
const words='G0A.SUM,G0A.MEAN,G0A.MEDIAN,G0A.WORDS,G0A.ARGMAX,G0A.UNWORDS,G0A.BYNAME,G0B.TOPKEY';
for(const [id,code] of Object.entries(sol)){
  for(const inp of ins[id.split('.')[0]]){
    fs.writeFileSync(W+'/try.ajisai',`${defl}\n${inp}\n${code}\n`);
    let out;
    try{out=execSync(`node tools/lexicon-emergence/cli.mjs compute phase2-2026-10-15 bottleneck 1 G1B ${W}/try.ajisai --words ${words}`,{encoding:'utf8'});}catch(e){out=e.stdout+e.stderr}
    const j=JSON.parse(out.slice(out.indexOf('{')));
    console.log(id,inp.slice(0,30),'=>',j.status,j.stackDisplay??j.message);
  }
}
fs.writeFileSync(W+'/../../submissions/G1B.json',JSON.stringify({agent:'G1B',generation:1,condition:'bottleneck',definitions:defs.map(d=>({name:d[0],body:d[1],note:d[2]})),solutions:sol},null,1));
