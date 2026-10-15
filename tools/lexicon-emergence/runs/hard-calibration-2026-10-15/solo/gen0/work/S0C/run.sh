#!/bin/bash
W=/home/user/Ajisai/tools/lexicon-emergence/runs/hard-calibration-2026-10-15/solo/gen0/work/S0C
run() {
  f="$W/t.ajisai"
  { cat "$W/defs.ajisai"; printf '%s\n' "$2"; printf '%s\n' "$3"; } > "$f"
  echo "== $1 :: $2"
  (cd /home/user/Ajisai && node tools/lexicon-emergence/cli.mjs compute hard-calibration-2026-10-15 solo 0 S0C "$f") | node -e 'let s="";process.stdin.on("data",d=>s+=d).on("end",()=>{try{const j=JSON.parse(s);console.log("  ",j.status,JSON.stringify(j.stackDisplay),JSON.stringify(j.output??""),j.message??"")}catch(e){console.log("  RAW",s.slice(0,600))}})'
}
S='[ 3 NIL 5 1 NIL NIL 4 ]'
run present_mean "$S" "'R' BIND R [ NIL? NOT ] FILTER 'P' BIND P S0C.SUM P LENGTH DIV"
run filled "$S" "'R' BIND R R [ NIL? NOT ] FILTER 0 GET [ 'E' BIND 'A' BIND A E E NIL? SELECT ] SCAN"
run gap_count "$S" "'R' BIND 1 0 R [ NIL? ] MAP SELECT 'G' BIND [ 0 ] G CONCAT G LENGTH TAKE 'GP' BIND G S0C.SUM G GP MUL S0C.SUM SUB"
run longest_gap "$S" "'R' BIND R 0 [ 'E' BIND 'A' BIND A 1 ADD 0 E NIL? SELECT ] SCAN 0 [ MAX ] FOLD"
run ratios "$S" "'R' BIND R 1 DROP R R LENGTH 1 SUB TAKE DIV"
run scaled "$S" "'R' BIND R [ NIL? NOT ] FILTER 'P' BIND P 1 DROP P 0 GET [ MAX ] FOLD 'M' BIND R M DIV"
J='{"orders":[{"sku":"a1","qty":2,"price":1.5},{"sku":"b2","qty":1,"price":4,"tags":["gift"]},{"sku":"a1","qty":3,"price":1.5,"tags":["bulk","gift"]},{"sku":"c3","qty":5}]}'
run revenue "'$J'" "S0C.ORDERS S0C.REVENUE"
run report "'$J'" "S0C.ORDERS S0C.REVENUE 2 FORMAT"
run unpriced "'$J'" "S0C.ORDERS [ 'Q' BIND Q 'price' GET NIL? ] FILTER [ 'sku' GET ] MAP UNIQUE"
run qty_by_sku "'$J'" "S0C.ORDERS S0C.QTY_BY_SKU"
run top_sku "'$J'" "S0C.ORDERS S0C.QTY_BY_SKU S0C.ARGMAX_FIRST"
run tag_counts "'$J'" "S0C.ORDERS [ 'Q' BIND Q 'tags' GET NIL? NOT ] FILTER [ 'tags' GET UNIQUE ] MAP FLATTEN TALLY"
M="'  The cat saw THE dog   and the Cat  '"
run count "$M" "S0C.WORDS LENGTH"
run normalize "$M" "LOWER S0C.WORDS [ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM"
run distinct "$M" "LOWER S0C.WORDS UNIQUE LENGTH"
run top_word "$M" "LOWER S0C.WORDS TALLY S0C.ARGMAX_FIRST"
run title "$M" "S0C.WORDS [ S0C.CAPWORD ' ' 2 COLLECT JOIN ] MAP JOIN TRIM"
run sorted_unique "$M" "LOWER S0C.WORDS UNIQUE SORT [ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM"
