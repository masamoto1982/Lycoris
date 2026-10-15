#!/bin/bash
# usage: run.sh NAME INPUT SOLUTION
W=tools/lexicon-emergence/runs/phase3-2026-10-15/runs/gen0/work/E0C
cd /home/user/Ajisai
f=$W/t_$1.ajisai
{ cat $W/defs.ajisai; printf '%s\n' "$2"; printf '%s\n' "$3"; } > $f
out=$(node tools/lexicon-emergence/cli.mjs compute phase3-2026-10-15 runs 0 E0C $f 2>&1)
echo "$out" | node -e 'let s="";process.stdin.on("data",d=>s+=d);process.stdin.on("end",()=>{try{const j=JSON.parse(s);console.log(j.status,JSON.stringify(j.stackDisplay),JSON.stringify(j.output||[]),j.message||"")}catch(e){console.log("RAW",s.slice(0,300))}})' | sed "s/^/$1: /"
