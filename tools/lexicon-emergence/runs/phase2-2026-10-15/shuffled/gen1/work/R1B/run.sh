#!/bin/bash
# usage: run.sh id input
cd /home/user/Ajisai
W=tools/lexicon-emergence/runs/phase2-2026-10-15/shuffled/gen1/work/R1B
sol=$(grep "^$1|" $W/sols.txt | cut -d'|' -f2-)
{ echo "[ 'W' BIND W CHARS 0 GET UPPER W CHARS 1 DROP JOIN 2 COLLECT JOIN ] 'R1B.CAP' DEF"; echo "$2 $sol"; } > $W/try.ajisai
node tools/lexicon-emergence/cli.mjs compute phase2-2026-10-15 shuffled 1 R1B $W/try.ajisai --words X1.RUKUTA,X1.GISUZU,X1.FIROSO,X1.VIDIKO,X1.PIMAPA,X1.FAGAMO,X1.RIVINO,X1.NILUGA | tr -d '\n' | grep -o '"status":"[a-z]*"\|"stackDisplay":[^]]*]\|"message":"[^"]*"'
echo
