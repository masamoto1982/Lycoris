#!/bin/sh
cd /home/user/Ajisai && node tools/lexicon-emergence/cli.mjs compute algo-calibration-2026-10-15 solo 0 S0C "$1" | node -e 'let s="";process.stdin.on("data",d=>s+=d).on("end",()=>{const j=JSON.parse(s);console.log(j.status, JSON.stringify(j.stackDisplay), j.message||"", j.output?JSON.stringify(j.output):"")})'
