import fs from 'fs'; import {execSync} from 'child_process';
const W='tools/lexicon-emergence/runs/algo-calibration-2026-10-15/solo/gen0';
const s=JSON.parse(fs.readFileSync(`${W}/submissions/S0B.json`));
const inp={runs:"[ 1 1 2 2 2 3 1 1 ]",number:"10",brackets:"'([]())'"};
for(const [id,code] of Object.entries(s.solutions)){
  const src=s.definitions.map(d=>`[ ${d.body} ] '${d.name}' DEF`).join('\n')+'\n'+inp[id.split('.')[0]]+'\n'+code+'\n';
  fs.writeFileSync(`${W}/work/S0B/try.ajisai`,src);
  const o=JSON.parse(execSync(`node tools/lexicon-emergence/cli.mjs compute algo-calibration-2026-10-15 solo 0 S0B ${W}/work/S0B/try.ajisai`).toString());
  console.log(id,o.status,JSON.stringify(o.stackDisplay));
}
