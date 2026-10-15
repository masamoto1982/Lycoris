import {defLines,sols} from './build.mjs';
import fs from 'fs';
import {execSync} from 'child_process';
for (const [k,[inp,sol]] of Object.entries(sols)) {
  fs.writeFileSync('try.ajisai', defLines+'\n'+inp+'\n'+sol+'\n');
  let out;
  try { out = execSync('node tools/lexicon-emergence/cli.mjs compute phase2-2026-10-15 bottleneck 0 G0B tools/lexicon-emergence/runs/phase2-2026-10-15/bottleneck/gen0/work/G0B/try.ajisai',{cwd:'/home/user/Ajisai'}).toString(); } catch(e){ out='ERR '+e.stdout+e.stderr }
  let j; try{ j=JSON.parse(out); console.log(k, j.status, j.stackDisplay, j.message||''); }catch{ console.log(k,out.slice(0,400)); }
}
