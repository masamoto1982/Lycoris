#!/bin/sh
cd /home/user/Ajisai
node tools/lexicon-emergence/cli.mjs compute hard-calibration-2026-10-15 solo 0 S0B tools/lexicon-emergence/runs/hard-calibration-2026-10-15/solo/gen0/work/S0B/try.ajisai | node -e 'let s="";process.stdin.on("data",d=>s+=d).on("end",()=>{const j=JSON.parse(s);console.log(j.status,JSON.stringify(j.stackDisplay),j.message||"",JSON.stringify(j.error||""))})'
