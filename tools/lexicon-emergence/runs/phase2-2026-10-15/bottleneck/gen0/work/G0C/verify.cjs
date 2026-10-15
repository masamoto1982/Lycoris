const fs = require('fs');
const sub = JSON.parse(fs.readFileSync(process.argv[2], 'utf8'));
const W = process.argv[3];
const inputs = {
  series: '[ 3 1 4 1 5 9 2 6 ]',
  prose: "'the quick brown fox jumps over the lazy dog the end'",
  ledger: "[ [ 'ann' 30 ] [ 'bob' -5 ] [ 'ann' 12 ] [ 'cy' -20 ] [ 'bob' 2 ] ]",
};
const defs = sub.definitions.map(d => `[ ${d.body} ] '${d.name}' DEF`);
const { execFileSync } = require('child_process');
for (const [task, code] of Object.entries(sub.solutions)) {
  const fam = task.split('.')[0];
  const file = `${W}/graded_${task.replace('.', '_')}.ajisai`;
  fs.writeFileSync(file, [...defs, inputs[fam], code].join('\n') + '\n');
  const rel = file.replace('/home/user/Ajisai/', '');
  const out = execFileSync('node', ['tools/lexicon-emergence/cli.mjs', 'compute', 'phase2-2026-10-15', 'bottleneck', '0', 'G0C', rel], { cwd: '/home/user/Ajisai', encoding: 'utf8' });
  const o = JSON.parse(out);
  console.log(task.padEnd(24), o.status, JSON.stringify(o.stackDisplay), o.message || '');
}
