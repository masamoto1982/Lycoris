#!/bin/bash
W=/home/user/Ajisai/tools/lexicon-emergence/runs/phase3-2026-10-15/runs/gen2/work/E2C
cd /home/user/Ajisai
while IFS='|' read -r name inp sol; do
  printf '%s\n%s %s\n' "$(cat $W/defs.txt)" "$inp" "$sol" > $W/case.ajisai
  printf '%-22s %-30s ' "$name" "${inp:0:30}"
  node tools/lexicon-emergence/cli.mjs compute phase3-2026-10-15 runs 2 E2C tools/lexicon-emergence/runs/phase3-2026-10-15/runs/gen2/work/E2C/case.ajisai --words G0C.ORDERS,G0D.WORDS,G0D.SHIFT,G0D.RUNS,G0A.UNWORDS,U1D.ARGMAX,U1D.TOPKEY,G0D.DIVISORS | node $W/show.mjs
done < "${1:-$W/cases.txt}"
