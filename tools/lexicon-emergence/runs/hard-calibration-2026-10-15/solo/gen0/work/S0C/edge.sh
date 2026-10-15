#!/bin/bash
W=/home/user/Ajisai/tools/lexicon-emergence/runs/hard-calibration-2026-10-15/solo/gen0/work/S0C
source <(sed -n '/^run() {/,/^}/p' "$W/run.sh")
for S in '[ NIL 2 ]' '[ 7 ]' '[ -2 NIL -4 0 NIL ]' '[ NIL NIL 3 NIL ]' '[ 1 2 3 ]' '[ NIL NIL 3 NIL 9 NIL NIL NIL ]' '[ -5 NIL -1 ]'; do
  run present_mean "$S" "'R' BIND R [ NIL? NOT ] FILTER 'P' BIND P S0C.SUM P LENGTH DIV"
  run filled "$S" "'R' BIND R R [ NIL? NOT ] FILTER 0 GET [ 'E' BIND 'A' BIND A E E NIL? SELECT ] SCAN"
  run gap_count "$S" "'R' BIND 1 0 R [ NIL? ] MAP SELECT 'G' BIND [ 0 ] G CONCAT G LENGTH TAKE 'GP' BIND G S0C.SUM G GP MUL S0C.SUM SUB"
  run longest_gap "$S" "'R' BIND R 0 [ 'E' BIND 'A' BIND A 1 ADD 0 E NIL? SELECT ] SCAN 0 [ MAX ] FOLD"
  run ratios "$S" "'R' BIND R 1 DROP R R LENGTH 1 SUB TAKE DIV"
  run scaled "$S" "'R' BIND R [ NIL? NOT ] FILTER 'P' BIND P 1 DROP P 0 GET [ MAX ] FOLD 'M' BIND R M DIV"
done
J2='{"orders":[{"sku":"x","qty":1},{"sku":"y","qty":2,"tags":[]}]}'
run revenue_none "'$J2'" "S0C.ORDERS S0C.REVENUE"
run report_none "'$J2'" "S0C.ORDERS S0C.REVENUE 2 FORMAT"
run unpriced_all "'$J2'" "S0C.ORDERS [ 'Q' BIND Q 'price' GET NIL? ] FILTER [ 'sku' GET ] MAP UNIQUE"
run tag_none "'$J2'" "S0C.ORDERS [ 'Q' BIND Q 'tags' GET NIL? NOT ] FILTER [ 'tags' GET UNIQUE ] MAP FLATTEN TALLY"
J3='{"orders":[{"sku":"p","qty":2,"price":3},{"sku":"q","qty":4,"price":-1},{"sku":"p","qty":2,"price":3},{"sku":"q","qty":1,"price":0}]}'
run report_neg "'$J3'" "S0C.ORDERS S0C.REVENUE 2 FORMAT"
run top_tie "'$J3'" "S0C.ORDERS S0C.QTY_BY_SKU S0C.ARGMAX_FIRST"
run qty_tie "'$J3'" "S0C.ORDERS S0C.QTY_BY_SKU"
run one_word "'hello'" "S0C.WORDS LENGTH"
run one_word_title "'  hELLO  '" "S0C.WORDS [ S0C.CAPWORD ' ' 2 COLLECT JOIN ] MAP JOIN TRIM"
run one_word_norm "'  hELLO  '" "LOWER S0C.WORDS [ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM"
run one_word_top "'  a A  '" "LOWER S0C.WORDS TALLY S0C.ARGMAX_FIRST"
run tie_top "'b a A a B b'" "LOWER S0C.WORDS TALLY S0C.ARGMAX_FIRST"
run sorted_one "'Zed'" "LOWER S0C.WORDS UNIQUE SORT [ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM"
run distinct_mixed "'Aa aA AA a'" "LOWER S0C.WORDS UNIQUE LENGTH"
