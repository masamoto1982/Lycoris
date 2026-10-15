#!/bin/bash
W=tools/lexicon-emergence/runs/algo-calibration-2026-10-15/solo/gen0/work/S0B
{ cat $W/defs.ajisai; echo "$1"; echo "$2"; } > $W/try.ajisai
node tools/lexicon-emergence/cli.mjs compute algo-calibration-2026-10-15 solo 0 S0B $W/try.ajisai | node -e 'let s=require("fs").readFileSync(0,"utf8");try{let j=JSON.parse(s);console.log(j.status,JSON.stringify(j.stackDisplay),j.message||"")}catch(e){console.log(s.slice(0,400))}'
