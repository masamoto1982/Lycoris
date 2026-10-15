import fs from 'fs';
const defs = [
 ['G0B.SUM','0 [ ADD ] FOLD','sum of a Vector'],
 ['G0B.MEAN',"'V' BIND V G0B.SUM V LENGTH DIV",'mean of a Vector'],
 ['G0B.MEDIAN',"SORT 'S' BIND S LENGTH 'N' BIND S N 1 SUB 2 DIV FLOOR N 2 DIV FLOOR 2 COLLECT GET G0B.MEAN",'median of a Vector'],
 ['G0B.ABS',"'D' BIND D D -1 MUL MAX",'absolute value, lifts over vectors'],
 ['G0B.TOPKEY',"'T' BIND T KEYS T VALUES -1 MUL ORDER 0 GET GET",'key of a Record with the largest value; first on ties'],
 ['G0B.JOINSP',"[ 'X' BIND ' ' X 2 COLLECT JOIN ] MAP JOIN TRIM",'join a Vector of words with single spaces'],
 ['G0B.CAP',"CHARS 'C' BIND C 0 GET UPPER 1 COLLECT C 1 DROP CONCAT JOIN",'capitalize one word'],
 ['G0B.BYNAME',"ZIP 'Z' BIND Z 0 GET Z 1 GET GROUP 'G' BIND G KEYS G VALUES [ G0B.SUM ] MAP RECORD",'ledger entries to Record name->sum'],
];
const ledger = "[ [ 'ann' 30 ] [ 'bob' -5 ] [ 'ann' 12 ] [ 'cy' -20 ] [ 'bob' 2 ] ]";
const nums = "[ 3 1 4 1 5 9 2 6 ]";
const prose = "'the quick brown fox jumps over the lazy dog the end'";
const sols = {
 'series.median':[nums,'G0B.MEDIAN'],
 'series.mad':[nums,"'V' BIND V V G0B.MEDIAN SUB G0B.ABS G0B.MEDIAN"],
 'series.moving3':[nums,"'V' BIND V -2 DROP V 1 DROP -1 DROP ADD V 2 DROP ADD 3 DIV"],
 'series.runmax':[nums,"'V' BIND V V 0 GET [ MAX ] SCAN"],
 'series.mode':[nums,'SORT TALLY G0B.TOPKEY'],
 'series.trimmed':[nums,'SORT 1 DROP -1 DROP G0B.MEAN'],
 'prose.longest':[prose,"' ' TOKENIZE 'W' BIND W W [ CHARS LENGTH ] MAP -1 MUL ORDER 0 GET GET"],
 'prose.freq_top':[prose,"' ' TOKENIZE TALLY G0B.TOPKEY"],
 'prose.distinct_sorted':[prose,"' ' TOKENIZE UNIQUE SORT G0B.JOINSP"],
 'prose.reverse_words':[prose,"' ' TOKENIZE REVERSE G0B.JOINSP"],
 'prose.capitalize':[prose,"' ' TOKENIZE [ G0B.CAP ] MAP G0B.JOINSP"],
 'prose.initials_of_long':[prose,"' ' TOKENIZE [ CHARS LENGTH 3 GT ] FILTER [ CHARS 0 GET UPPER ] MAP JOIN"],
 'ledger.total':[ledger,"ZIP 1 GET G0B.SUM"],
 'ledger.by_name':[ledger,"G0B.BYNAME"],
 'ledger.top_name':[ledger,"G0B.BYNAME G0B.TOPKEY"],
 'ledger.debtors':[ledger,"G0B.BYNAME 'B' BIND B KEYS B VALUES 2 COLLECT ZIP [ 1 GET 0 LT ] FILTER [ 0 GET ] MAP"],
 'ledger.count_names':[ledger,"ZIP 0 GET UNIQUE LENGTH"],
 'ledger.largest_entry':[ledger,"'E' BIND E E ZIP 1 GET -1 MUL ORDER 0 GET GET"],
};
const extra = process.argv[2];
const defLines = defs.map(([n,b])=>`[ ${b} ] '${n}' DEF`).join('\n');
if (extra==='submit'){
  fs.writeFileSync('/home/user/Ajisai/tools/lexicon-emergence/runs/phase2-2026-10-15/bottleneck/gen0/submissions/G0B.json',JSON.stringify({agent:'G0B',generation:0,condition:'bottleneck',definitions:defs.map(([name,body,note])=>({name,body,note})),solutions:Object.fromEntries(Object.entries(sols).map(([k,v])=>[k,v[1]]))},null,2));
} else {
  fs.writeFileSync('try.ajisai', defLines+'\n'+(extra?extra:'')+'\n');
  const [k]=[extra];
}
export {defLines,sols};
