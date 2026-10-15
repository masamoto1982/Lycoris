#!/bin/bash
# usage: rt.sh <name> <defs-file (DEF lines only, may be empty)> <input> <solution>
W=/home/user/Ajisai/tools/lexicon-emergence/runs/phase2-2026-10-15/bottleneck/gen0/work/G0C
cd /home/user/Ajisai
f="$W/rt_$1.ajisai"
{ if [ -s "$2" ]; then cat "$2"; fi; printf '%s\n' "$3" "$4"; } > "$f"
node tools/lexicon-emergence/cli.mjs compute phase2-2026-10-15 bottleneck 0 G0C "tools/lexicon-emergence/runs/phase2-2026-10-15/bottleneck/gen0/work/G0C/rt_$1.ajisai" | node -e 'let s="";process.stdin.on("data",d=>s+=d).on("end",()=>{try{const o=JSON.parse(s);console.log(o.status, JSON.stringify(o.stackDisplay), o.message||"")}catch(e){console.log("RAW",s.slice(0,300))}})'
