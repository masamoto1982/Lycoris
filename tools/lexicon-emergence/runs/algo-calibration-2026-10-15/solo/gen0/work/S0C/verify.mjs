import fs from 'node:fs';
import { DEFS, SOL, defsText, runProgram } from './build.mjs';
const W = '/home/user/Ajisai/tools/lexicon-emergence/runs/algo-calibration-2026-10-15/solo/gen0/work/S0C';
fs.writeFileSync(`${W}/defs.ajisai`, '# S0C definitions\n' + defsText());

// reference implementations (independent of the Ajisai code)
const fmt = v => v === null ? 'NIL' : typeof v === 'boolean' ? (v ? 'TRUE' : 'FALSE') : typeof v === 'string' ? `'${v}'` :
  typeof v === 'number' ? `${v}/1` : Array.isArray(v) ? (v.length === 0 ? '[ ]' : `[ ${v.map(fmt).join(' ')} ]`) : String(v);
const runs = v => { const out = []; for (const x of v) { if (out.length && out[out.length - 1][0] === x) out[out.length - 1][1]++; else out.push([x, 1]); } return out; };
const upruns = v => { let best = 1, cur = 1; for (let i = 1; i < v.length; i++) { cur = v[i] > v[i - 1] ? cur + 1 : 1; best = Math.max(best, cur); } return best; };
const dedupe = v => runs(v).map(p => p[0]);
const second = v => { const d = [...new Set(v)].sort((a, b) => b - a); return d.length >= 2 ? d[1] : null; };
const majority = v => { const n = v.length; for (const x of new Set(v)) if (v.filter(y => y === x).length * 2 > n) return x; return null; };
const isPrime = k => { if (k < 2) return false; for (let d = 2; d < k; d++) if (k % d === 0) return false; return true; };
const primes = n => Array.from({ length: n }, (_, i) => i + 1).filter(isPrime);
const divisors = n => Array.from({ length: n }, (_, i) => i + 1).filter(d => n % d === 0);
const binary = n => n.toString(2);
const collatz = n => { let s = 0; while (n !== 1) { n = n % 2 === 0 ? n / 2 : 3 * n + 1; s++; } return s; };
const droot = n => { while (n >= 10) { n = String(n).split('').reduce((a, c) => a + Number(c), 0); } return n; };
const fibIndex = n => { let a = 1, b = 1, k = 1; while (a < n) { [a, b] = [b, a + b]; k++; } return k; };
const reduceBr = s => { let t = s; for (;;) { const u = t.replace(/\(\)|\[\]/g, ''); if (u === t) return t; t = u; } };
const depth = s => { let d = 0, best = 0; for (const c of s) { if ('(['.includes(c)) d++; else d = Math.max(0, d - 1); best = Math.max(best, d); } return best; };
const balancedPrefix = s => { let best = 0; for (let k = 0; k <= s.length; k++) if (reduceBr(s.slice(0, k)) === '') best = k; return best; };
const unmatched = s => reduceBr(s).length;

const rnd = (() => { let x = 12345; return () => (x = (x * 1103515245 + 12345) & 0x7fffffff) / 0x7fffffff; })();
const ri = (a, b) => a + Math.floor(rnd() * (b - a + 1));
const vec = () => Array.from({ length: ri(1, 10) }, () => rnd() < 0.5 ? ri(0, 3) : ri(-5, 5));
const brk = () => Array.from({ length: ri(0, 12) }, () => '()[]'[ri(0, 3)]).join('');

const cases = [];
const vecs = [[1, 1, 2, 2, 2, 3, 1, 1], [7], [3, 3, 3], [1, 2, 3, 4], [5, 4, 3], [2, 2, 1, 5], [1, 2, 1, 1, 1], [4, 4, 1, 4, 4]];
for (let i = 0; i < 40; i++) vecs.push(vec());
for (const v of vecs) {
  const lit = `[ ${v.join(' ')} ]`;
  cases.push(['runs.rle', lit, fmt(runs(v))]);
  cases.push(['runs.upruns', lit, fmt(upruns(v))]);
  cases.push(['runs.dedupe', lit, fmt(dedupe(v))]);
  cases.push(['runs.second_largest', lit, fmt(second(v))]);
  cases.push(['runs.majority', lit, fmt(majority(v))]);
}
const nums = []; for (let n = 1; n <= 60; n++) nums.push(n);
for (const n of nums) {
  cases.push(['number.primes', `${n}`, fmt(primes(n))]);
  cases.push(['number.divisors', `${n}`, fmt(divisors(n))]);
  cases.push(['number.binary', `${n}`, fmt(binary(n))]);
  cases.push(['number.collatz', `${n}`, fmt(collatz(n))]);
  cases.push(['number.digital_root', `${n}`, fmt(droot(n))]);
  cases.push(['number.fib_index', `${n}`, fmt(fibIndex(n))]);
}
const strs = ['', '()', '([]())', '(]', ')(', '([)]', '(()', '())(', '[[[', ']]', '([)]()[]', '((()))[]', '(]]]]]', '()[]()', '(', '[(])', '][', '((((((((((((', '()()()()()()'];
for (let i = 0; i < 60; i++) strs.push(brk());
for (const s of strs) {
  cases.push(['brackets.depth', `'${s}'`, fmt(depth(s))]);
  cases.push(['brackets.balanced', `'${s}'`, fmt(reduceBr(s) === '')]);
  cases.push(['brackets.balanced_prefix', `'${s}'`, fmt(balancedPrefix(s))]);
  cases.push(['brackets.unmatched', `'${s}'`, fmt(unmatched(s))]);
}

const only = process.argv[2];
const fails = []; let count = 0;
for (const [task, input, expect] of cases) {
  if (only && !task.startsWith(only)) continue;
  const src = `# S0C test\n${defsText()}${input}\n${SOL[task]}\n`;
  const r = runProgram(src);
  count++;
  const got = r.status === 'ok' && r.stackDisplay.length === 1 ? r.stackDisplay[0] : `ERR ${r.message || r.stackDisplay}`;
  if (got !== expect) fails.push({ task, input, expect, got });
}
console.log(`ran ${count}, failures ${fails.length}`);
for (const f of fails.slice(0, 20)) console.log(JSON.stringify(f));
