import fs from 'fs';
const notes = {
 'S0A.PICK':'x y t -> x when t is TRUE else y; safe for Vectors of any length',
 'S0A.OR':'a b -> logical OR of two truth values',
 'S0A.RLESTEP':'FOLD step: extend a run-length list [[v c]...] with one element',
 'S0A.RLE':'Vector -> run-length encoding [[value count]...]',
 'S0A.UPSTEP':'FOLD step over [prev cur best] for longest strictly increasing stretch',
 'S0A.UPRUNS':'Vector -> length of longest strictly increasing adjacent stretch',
 'S0A.DDSTEP':'FOLD step: append element unless equal to last',
 'S0A.DEDUPE':'Vector -> Vector with adjacent duplicates collapsed',
 'S0A.SECOND':'Vector -> second-largest distinct value, or NIL',
 'S0A.MAJORITY':'Vector -> value occurring in more than half the positions, or NIL',
 'S0A.DIVISORS':'n -> ascending Vector of positive divisors of n',
 'S0A.PRIMES':'n -> ascending Vector of primes <= n',
 'S0A.BINARY':'n (1..63) -> base-2 String without leading zeros',
 'S0A.CZSTEP':'FOLD step over [x steps] applying one Collatz step until x is 1',
 'S0A.COLLATZ':'n (small) -> number of Collatz steps to reach 1 (up to 150)',
 'S0A.DROOT':'n >= 1 -> digital root',
 'S0A.FIBINDEX':'n (<= 144) -> smallest k with F(k) >= n',
 'S0A.DEPTH':'bracket String -> maximum nesting depth',
 'S0A.RED':'FOLD step: push a bracket char, or pop when it closes the matching top ("()" or "[]")',
 'S0A.BALANCED':'bracket String -> TRUE when properly nested and matched',
 'S0A.BPREFIX':'bracket String -> length of longest balanced prefix',
 'S0A.UNMATCHED':'bracket String -> characters left after deleting matched pairs repeatedly',
};
const defs = fs.readFileSync('defs.ajisai','utf8').trim().split('\n').map(l=>{
  const m = l.match(/^\[ (.*) \] '([^']+)' DEF$/); return {name:m[2], body:m[1], note:notes[m[2]]};
});
const solutions = {
 'runs.rle':'S0A.RLE','runs.upruns':'S0A.UPRUNS','runs.dedupe':'S0A.DEDUPE','runs.second_largest':'S0A.SECOND','runs.majority':'S0A.MAJORITY',
 'number.primes':'S0A.PRIMES','number.divisors':'S0A.DIVISORS','number.binary':'S0A.BINARY','number.collatz':'S0A.COLLATZ','number.digital_root':'S0A.DROOT','number.fib_index':'S0A.FIBINDEX',
 'brackets.depth':'S0A.DEPTH','brackets.balanced':'S0A.BALANCED','brackets.balanced_prefix':'S0A.BPREFIX','brackets.unmatched':'S0A.UNMATCHED'};
const inputs = {runs:'[ 1 1 2 2 2 3 1 1 ]', number:'10', brackets:"'([]())'"};
const sub = {agent:'S0A',generation:0,condition:'solo',definitions:defs,solutions};
fs.writeFileSync('../../submissions-draft.json', '');
fs.unlinkSync('../../submissions-draft.json');
fs.writeFileSync('sub.json', JSON.stringify(sub,null,2)+'\n');
let prog = defs.map(d=>`[ ${d.body} ] '${d.name}' DEF`).join('\n')+'\n';
for (const [k,v] of Object.entries(solutions)) prog += `${inputs[k.split('.')[0]]} ${v}\n`;
fs.writeFileSync('try.ajisai', prog);
