#!/bin/bash
cd /home/user/Ajisai
P=tools/lexicon-emergence/runs/phase2-2026-10-15/solo/gen0/work/S0B
for t in "$@"; do
  g=${t%%.*}
  { cat $P/parts/defs.ajisai; cat $P/parts/in.$g; cat $P/parts/sol.$t; } > $P/try.ajisai
  echo "== $t"
  node tools/lexicon-emergence/cli.mjs compute phase2-2026-10-15 solo 0 S0B $P/try.ajisai | node -e "const j=JSON.parse(require(\"fs\").readFileSync(0));console.log(j.status,JSON.stringify(j.stackDisplay),j.message||\"\")"
  echo
done
