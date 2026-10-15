import {defLines,sols} from './build.mjs';
import fs from 'fs';
import {execSync} from 'child_process';
const cases = {
 'series.median':["[ -1/2 3 2 ]","[ 5 5 5 ]","[ 4 -2 7 1/3 ]"],
 'series.mad':["[ 1 2 3 ]","[ -1 -5 1/2 7 ]"],
 'series.moving3':["[ 1 2 4 ]"],
 'series.runmax':["[ -5 -7 -2 -9 ]"],
 'series.mode':["[ 2 2 -1 -1 7 ]","[ 5 5 5 ]","[ 1/2 2/4 3 ]"],
 'series.trimmed':["[ 1 2 3 ]","[ -1 -1 5 5 1/2 ]"],
 'prose.longest':["'a'","'ab cd e'"],
 'prose.freq_top':["'a'","'b a b a c'"],
 'prose.distinct_sorted':["'a'","'b a b'"],
 'prose.reverse_words':["'solo'"],
 'prose.capitalize':["'a'","'a bc d'"],
 'prose.initials_of_long':["'a is of'","'abcd'"],
 'ledger.total':["[ [ 'a' -3 ] ]"],
 'ledger.by_name':["[ [ 'a' -3 ] ]"],
 'ledger.top_name':["[ [ 'a' -3 ] ]","[ [ 'a' 1 ] [ 'b' 1 ] ]","[ [ 'z' -1 ] [ 'y' -2 ] ]"],
 'ledger.debtors':["[ [ 'a' 3 ] ]","[ [ 'a' 3 ] [ 'a' -3 ] ]"],
 'ledger.count_names':["[ [ 'a' 3 ] ]"],
 'ledger.largest_entry':["[ [ 'a' 3 ] ]","[ [ 'a' 3 ] [ 'b' 3 ] [ 'c' 1 ] ]","[ [ 'a' -3 ] [ 'b' -1/2 ] ]"],
};
for (const [k,ins] of Object.entries(cases)) for (const inp of ins) {
  fs.writeFileSync('try.ajisai', defLines+'\n'+inp+'\n'+sols[k][1]+'\n');
  let out;
  try { out = execSync('node tools/lexicon-emergence/cli.mjs compute phase2-2026-10-15 bottleneck 0 G0B tools/lexicon-emergence/runs/phase2-2026-10-15/bottleneck/gen0/work/G0B/try.ajisai',{cwd:'/home/user/Ajisai'}).toString(); } catch(e){ out='ERR '+e.stdout+e.stderr }
  try{ const j=JSON.parse(out); console.log(k,inp,'=>',j.status,j.stackDisplay.join(' | '),j.message||''); }catch{ console.log(k,inp,out.slice(0,300)); }
}
