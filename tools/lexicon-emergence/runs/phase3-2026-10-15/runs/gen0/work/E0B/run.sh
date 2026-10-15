#!/bin/bash
cd /home/user/Ajisai
W=tools/lexicon-emergence/runs/phase3-2026-10-15/runs/gen0/work/E0B
printf '%s\n' "$1" > $W/try.ajisai
node tools/lexicon-emergence/cli.mjs compute phase3-2026-10-15 runs 0 E0B $W/try.ajisai | node -e 'let s="";process.stdin.on("data",d=>s+=d).on("end",()=>{try{const j=JSON.parse(s);console.log(j.status,JSON.stringify(j.stackDisplay),j.message||"",j.error?JSON.stringify(j.error):"")}catch(e){console.log(s)}})'
