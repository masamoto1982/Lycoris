import fs from 'fs'; import {execSync} from 'child_process';
const D='tools/lexicon-emergence/runs/phase2-2026-10-15/shuffled/gen2/work/R2A';
const defs=[
 {name:'R2A.ABS',body:"'X' BIND X 0 X SUB MAX",note:'absolute value of a number (lifts over a Vector)'},
 {name:'R2A.CAP',body:"CHARS 'C' BIND C 0 GET UPPER C 1 DROP JOIN 2 COLLECT JOIN",note:'String with its first letter upper-cased'},
 {name:'R2A.MAD',body:"'V' BIND V V X2.ZILOLE SUB R2A.ABS X2.ZILOLE",note:'median absolute deviation of a numeric Vector'},
 {name:'R2A.MOVING3',body:"'V' BIND V -2 DROP V 1 DROP -1 DROP ADD V 2 DROP ADD 3 DIV",note:'means of every three consecutive numbers'},
 {name:'R2A.RUNMAX',body:"'V' BIND V V 0 GET [ MAX ] SCAN",note:'running maximum of a numeric Vector'},
 {name:'R2A.MODE',body:"SORT TALLY X2.PIDOZE",note:'most frequent element, smallest on ties'},
 {name:'R2A.NEGKEYS',body:"'R' BIND R KEYS R VALUES 2 COLLECT ZIP [ 1 GET 0 LT ] FILTER [ 0 GET ] MAP",note:'keys of a Record whose values are below zero, in key order'},
];
const sol={
 'series.median':'X2.ZILOLE',
 'series.mad':'R2A.MAD',
 'series.moving3':'R2A.MOVING3',
 'series.runmax':'R2A.RUNMAX',
 'series.mode':'R2A.MODE',
 'series.trimmed':'SORT 1 DROP -1 DROP X2.NEGEKE',
 'prose.longest':"X2.VIVIKA 'W' BIND W W [ CHARS LENGTH ] MAP X2.FOMOZO GET",
 'prose.freq_top':'X2.VIVIKA TALLY X2.PIDOZE',
 'prose.distinct_sorted':'X2.VIVIKA UNIQUE SORT X2.GUPOSO',
 'prose.reverse_words':'X2.VIVIKA REVERSE X2.GUPOSO',
 'prose.capitalize':'X2.VIVIKA [ R2A.CAP ] MAP X2.GUPOSO',
 'prose.initials_of_long':'X2.VIVIKA [ CHARS LENGTH 3 GT ] FILTER [ CHARS 0 GET UPPER ] MAP JOIN',
 'ledger.total':'ZIP 1 GET X2.TUZIGA',
 'ledger.by_name':'X2.NAZEDA',
 'ledger.top_name':'X2.NAZEDA X2.PIDOZE',
 'ledger.debtors':'X2.NAZEDA R2A.NEGKEYS',
 'ledger.count_names':'ZIP 0 GET UNIQUE LENGTH',
 'ledger.largest_entry':"'E' BIND E E ZIP 1 GET X2.FOMOZO GET",
};
const inputs={series:['[ 3 1 4 1 5 9 2 6 ]','[ -1/2 3 3 -1/2 7 ]','[ 5 2 8 ]'],prose:["'the quick brown fox jumps over the lazy dog the end'","'a bb a bb cc'","'hi'"],ledger:["[ [ 'ann' 30 ] [ 'bob' -5 ] [ 'ann' 12 ] [ 'cy' -20 ] [ 'bob' 2 ] ]","[ [ 'z' 5 ] ]","[ [ 'a' 1 ] [ 'b' 4 ] [ 'a' 3 ] ]"]};
const pre=defs.map(d=>`[ ${d.body} ] '${d.name}' DEF`).join('\n');
for(const [t,s] of Object.entries(sol)){for(const inp of inputs[t.split('.')[0]]){
 fs.writeFileSync(D+'/try.ajisai',pre+'\n'+inp+'\n'+s+'\n');
 let o; try{o=execSync(`node tools/lexicon-emergence/cli.mjs compute phase2-2026-10-15 shuffled 2 R2A ${D}/try.ajisai --words X2.TUZIGA,X2.VIVIKA,X2.NEGEKE,X2.PIDOZE,X2.GUPOSO,X2.NAZEDA,X2.ZILOLE,X2.FOMOZO`).toString()}catch(e){o=e.stdout.toString()}
 const j=JSON.parse(o); console.log(t.padEnd(24),j.status,JSON.stringify(j.stackDisplay),j.message||'');}}
if(process.argv[2]==='write') fs.writeFileSync('tools/lexicon-emergence/runs/phase2-2026-10-15/shuffled/gen2/submissions/R2A.json',JSON.stringify({agent:'R2A',generation:2,condition:'shuffled',definitions:defs,solutions:sol},null,2)+'\n');
