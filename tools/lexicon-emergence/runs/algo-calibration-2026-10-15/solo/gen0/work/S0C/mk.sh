#!/bin/sh
# usage: mk.sh <defs-file> <input-literal> <solution-line>
W=/home/user/Ajisai/tools/lexicon-emergence/runs/algo-calibration-2026-10-15/solo/gen0/work/S0C
{ cat "$1"; echo "$2"; echo "$3"; } > "$W/cur.ajisai"
"$W/run.sh" "$W/cur.ajisai"
