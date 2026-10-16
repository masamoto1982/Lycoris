#!/bin/bash
# usage: run.sh name input solution
cd /home/user/Ajisai
W=tools/lexicon-emergence/runs/phase3-2026-10-15/runs/gen1/work/E1B
cat $W/defs.ajisai > $W/t_$1.ajisai
echo "$2" >> $W/t_$1.ajisai
echo "$3" >> $W/t_$1.ajisai
echo "== $1 <= $2"
node tools/lexicon-emergence/cli.mjs compute phase3-2026-10-15 runs 1 E1B $W/t_$1.ajisai --words G0D.WORDS,G0C.ORDERS,G0A.UNWORDS,G0D.SHIFT,G0D.DIVISORS,G0D.RUNS,G0A.BSTEP,G0B.REDUCE | node -e 'let s=require("fs").readFileSync(0,"utf8");try{let j=JSON.parse(s);console.log(j.status||j.outcome, JSON.stringify(j.stackDisplay), j.message||"", j.error?JSON.stringify(j.error):"")}catch(e){console.log(s.slice(0,500))}'
