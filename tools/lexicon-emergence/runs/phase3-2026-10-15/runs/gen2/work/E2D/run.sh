#!/bin/bash
# usage: run.sh <file-with-code>  -> prints stackDisplay or error
D=tools/lexicon-emergence/runs/phase3-2026-10-15/runs/gen2/work/E2D
cp "$1" $D/try.ajisai
node tools/lexicon-emergence/cli.mjs compute phase3-2026-10-15 runs 2 E2D $D/try.ajisai --words G0C.ORDERS,G0D.WORDS,G0D.SHIFT,G0D.RUNS,G0A.UNWORDS,U1D.ARGMAX,U1D.TOPKEY,G0D.DIVISORS 2>&1 | node -e '
let s="";process.stdin.on("data",d=>s+=d).on("end",()=>{try{const j=JSON.parse(s);if(j.status==="ok")console.log("OK:",JSON.stringify(j.stackDisplay),j.output&&j.output.length?("OUT:"+JSON.stringify(j.output)):"");else console.log("ERR:",j.message,JSON.stringify(j.error));}catch(e){console.log(s)}})'
