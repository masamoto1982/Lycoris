#!/bin/bash
# usage: run.sh id 'input'
D=tools/lexicon-emergence/runs/phase2-2026-10-15/bottleneck/gen2/work/G2B
code=$(grep "^$1|" $D/tasks.txt | cut -d'|' -f2-)
{ echo "[ 'W' BIND W CHARS 'C' BIND C 0 C 0 GET UPPER PUT JOIN ] 'G2B.CAP' DEF"; echo "$2"; echo "$code"; } > $D/try.ajisai
node tools/lexicon-emergence/cli.mjs compute phase2-2026-10-15 bottleneck 2 G2B $D/try.ajisai --words G0A.SUM,G0A.WORDS,G0A.BYNAME,G0A.MEAN,G0A.UNWORDS,G0A.ARGMAX,G0B.TOPKEY,G0A.MEDIAN | node -e "const j=JSON.parse(require(\"fs\").readFileSync(0,\"utf8\"));console.log(j.status,JSON.stringify(j.stackDisplay),j.message||\"\")"
