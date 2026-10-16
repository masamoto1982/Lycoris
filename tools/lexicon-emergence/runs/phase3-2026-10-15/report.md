# Lexicon-emergence run report

## Per generation

| condition | gen | agents | lexicon | correct | mean tokens | tokens ÷ expanded | Core-equivalent (H5) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| bottleneck | 0 | 4 | 0 | 132/132 | 19.4 | 0.541 | 132/132 |
| open | 1 | 4 | 27 | 131/132 | 8.2 | 0.221 | 131/131 |
| open | 2 | 4 | 42 | 132/132 | 3.7 | 0.096 | 132/132 |
| open | 3 | 4 | 38 | 132/132 | 2 | 0.052 | 132/132 |
| reinvention | 1 | 4 | 8 | 131/132 | 15 | 0.435 | 131/131 |
| reinvention | 2 | 4 | 8 | 130/132 | 8.3 | 0.217 | 130/130 |
| reinvention | 3 | 4 | 8 | 132/132 | 10.7 | 0.322 | 132/132 |
| runs | 0 | 4 | 0 | 132/132 | 13.7 | 0.383 | 132/132 |
| runs | 1 | 4 | 8 | 132/132 | 13.7 | 0.405 | 132/132 |
| runs | 2 | 4 | 8 | 132/132 | 9.4 | 0.286 | 132/132 |
| shuffled | 1 | 4 | 8 | 132/132 | 15.7 | 0.484 | 132/132 |
| shuffled | 2 | 4 | 8 | 131/132 | 13.4 | 0.404 | 131/131 |
| shuffled | 3 | 4 | 8 | 131/132 | 9.7 | 0.293 | 131/131 |
| solo | 0 | 4 | 0 | 130/132 | 10.9 | 0.304 | 130/130 |
| uses | 1 | 4 | 8 | 130/132 | 14.6 | 0.418 | 130/130 |
| uses | 2 | 4 | 8 | 132/132 | 12 | 0.339 | 132/132 |
| uses | 3 | 4 | 8 | 132/132 | 10.4 | 0.31 | 132/132 |

## Correct solutions by generation and model

| condition | gen | opus | sonnet | haiku | fable | mean runs |
| --- | --- | --- | --- | --- | --- | --- |
| bottleneck | 0 | 33/33 | 33/33 | 33/33 | 33/33 | — |
| open | 1 | 33/33 | 33/33 | 33/33 | 32/33 | — |
| open | 2 | 33/33 | 33/33 | 33/33 | 33/33 | — |
| open | 3 | 33/33 | 33/33 | 33/33 | 33/33 | — |
| reinvention | 1 | 33/33 | 33/33 | 32/33 | 33/33 | — |
| reinvention | 2 | 33/33 | 33/33 | 32/33 | 32/33 | — |
| reinvention | 3 | 33/33 | 33/33 | 33/33 | 33/33 | — |
| runs | 0 | 33/33 | 33/33 | 33/33 | 33/33 | 221.3 |
| runs | 1 | 33/33 | 33/33 | 33/33 | 33/33 | 272 |
| runs | 2 | 33/33 | 33/33 | 33/33 | 33/33 | 174.8 |
| shuffled | 1 | 33/33 | 33/33 | 33/33 | 33/33 | — |
| shuffled | 2 | 33/33 | 33/33 | 33/33 | 32/33 | — |
| shuffled | 3 | 33/33 | 33/33 | 32/33 | 33/33 | — |
| solo | 0 | 33/33 | 33/33 | 32/33 | 32/33 | — |
| uses | 1 | 31/33 | 33/33 | 33/33 | 33/33 | — |
| uses | 2 | 33/33 | 33/33 | 33/33 | 33/33 | — |
| uses | 3 | 33/33 | 33/33 | 33/33 | 33/33 | — |

## Correct solutions identical to the reference

- G0B: 1 (orders.unpriced)
- G0D: 2 (sensor.present_mean, sensor.filled)
- R2C: 1 (sensor.longest_gap)
- R3B: 1 (orders.unpriced)
- R3D: 1 (orders.unpriced)
- E0A: 1 (sensor.ratios)
- E0B: 2 (sensor.filled, sensor.ratios)
- E0C: 3 (number.divisors, sensor.present_mean, sensor.ratios)
- E1A: 1 (sensor.ratios)
- E1C: 1 (sensor.ratios)
- E2B: 1 (sensor.present_mean)
- E2C: 1 (sensor.ratios)
- E2D: 1 (sensor.ratios)
- Z1C: 1 (sensor.present_mean)
- Z2C: 2 (sensor.present_mean, sensor.ratios)
- S0B: 1 (orders.unpriced)
- S0C: 1 (sensor.ratios)
- U1A: 2 (sensor.present_mean, sensor.filled)
- U1C: 3 (sensor.present_mean, sensor.filled, sensor.ratios)
- U2C: 1 (sensor.ratios)
- U3A: 1 (orders.unpriced)

## Accuracy per model

| model | agents | correct |
| --- | --- | --- |
| opus | 17 | 559/561 |
| sonnet | 17 | 561/561 |
| haiku | 17 | 557/561 |
| fable | 17 | 557/561 |

## H1 — classes defined independently by two or more agents (81 of 189 classes, 795 Words; 66 reached from two or more models)

Baseline over subjects that inherited nothing — mean Jaccard of defined classes: same-model pairs 0.188, cross-model pairs 0.158.

- **22 agents** (O1A, O1D, R2D, O1B, R2B, E0A, E0D, E1A, E1D, E2A, E2D, Z1A, Z2B, Z3A, Z3B, Z3D, S0A, S0D, U1D, U2D, U3B, U3D; models opus, fable, sonnet), joined at d0/d1/d0a:
  - `O1A.PRESENT` = `[ NIL? NOT ] FILTER` — V with NILs -> the present elements, in order
  - `O1D.PRESENT` = `[ NIL? NOT ] FILTER` — Vector with NILs -> only its present (non-NIL) elements, in order
  - `R2D.PRESENT` = `[ NIL? NOT ] FILTER` — Vector -> the elements that are not NIL, in order
  - `O1A.FFILL` = `'O1A.FV' BIND O1A.FV O1A.FV O1A.PRESENT 0 GET [ G0B.FILLSTEP ] SCAN` — V with NILs -> forward-filled V, leading NILs take the first present value
  - `O1B.PRESENT` = `[ NIL? NOT ] FILTER` — Vector with NILs -> Vector of only the present elements, in order
  - `O1D.FILL` = `'V' BIND V V O1D.PRESENT 0 GET [ G0B.FILLSTEP ] SCAN` — Vector with NILs (at least one present) -> forward-filled Vector; leading NILs take the first present value
  - `O2A.FFILL` = `'O2A.FV' BIND O2A.FV O2A.FV O1D.PRESENT 0 GET [ G0B.FILLSTEP ] SCAN` — V with NILs (one present) -> forward-filled V, leading NILs taking the first present value
  - `O2D.FILLED` = `'O2D.V' BIND O2D.V O2D.V O1D.PRESENT 0 GET [ G0B.FILLSTEP ] SCAN` — Vector with NILs -> same-length Vector, each NIL replaced by the nearest present value before it (leading NILs take the first present value)
  - `O3A.FFILL` = `'O3A.FV' BIND O3A.FV O3A.FV O1D.PRESENT 0 GET [ G0B.FILLSTEP ] SCAN` — V with NILs (at least one present) -> forward-filled V, leading NILs take the first present value
  - `O3B.FILLED` = `'O3B.FV' BIND O3B.FV O3B.FV O1D.PRESENT 0 GET [ G0B.FILLSTEP ] SCAN` — Vector with NILs -> forward-filled Vector, leading NILs take the first present value
  - `O3D.FILLED` = `'O3D.V' BIND O3D.V O3D.V O1D.PRESENT 0 GET [ G0B.FILLSTEP ] SCAN` — V with NILs -> forward-filled V, leading NILs taking the first present value
  - `R2B.FILL0` = `[ 'R2B.ZE' BIND 0 R2B.ZE R2B.ZE NIL? SELECT ] MAP` — Vector with NILs -> same Vector with each NIL replaced by 0.
  - `R2B.FILLED` = `'R2B.FV' BIND R2B.FV R2B.FV R1A.GAPMASK 1 LT R2B.KEEP 0 GET 'R2B.F0' BIND R2B.FV R2B.F0 [ 'R2B.FE' BIND 'R2B.FA' BIND R2B.FA R2B.FE R2B.FE NIL? SELECT ] SCAN` — Vector with NILs -> each NIL replaced by the nearest present value before it (leading NILs take the first present value).
  - `R2D.FILLED` = `'R2D.FV' BIND R2D.FV R2D.FV R2D.PRESENT 0 GET [ 'R2D.FE' BIND 'R2D.FA' BIND R2D.FA R2D.FE R2D.FE NIL? SELECT ] SCAN` — Vector with NILs and at least one present value -> forward-filled Vector; leading NILs take the first present value
  - `R3A.FILLED` = `'R3A.LV' BIND R3A.LV R3A.LV R2D.PRESENT 0 GET [ 'R3A.LE' BIND 'R3A.LA' BIND R3A.LA R3A.LE R3A.LE NIL? SELECT ] SCAN` — Vector with NILs -> forward-filled Vector; leading NILs take the first present value
  - `E0A.PRESENT` = `[ NIL? NOT ] FILTER` — Vector -> the elements that are not NIL, in order
  - `E0D.PRESENT` = `[ NIL? NOT ] FILTER` — vector -> only its non-NIL elements, in order
  - `E0D.FILLED` = `'V' BIND V V E0D.PRESENT 0 GET [ 'X' BIND 'A' BIND A X X NIL? SELECT ] SCAN` — vector with NILs (some present) -> NILs replaced by the nearest earlier present value (first present for leading NILs)
  - `E1A.PRESENT` = `[ NIL? NOT ] FILTER` — Vector -> its non-NIL elements, in order
  - `E1D.PRESENT` = `[ NIL? NOT ] FILTER` — Vector -> only its non-NIL elements, in order
  - `E2A.PRESENT` = `[ NIL? NOT ] FILTER` — V -> its non-NIL elements, in order
  - `E2A.FILLED` = `'EV' BIND EV EV E2A.PRESENT 0 GET [ 'EX' BIND 'EA' BIND EA EX EX NIL? SELECT ] SCAN` — V with NILs -> each NIL replaced by the previous present value (leading NILs by the first present)
  - `E2D.PRESENT` = `[ NIL? NOT ] FILTER` — Vector with NILs -> the same Vector with the NIL lanes removed
  - `Z1A.PRESENT` = `[ NIL? NOT ] FILTER` — Vector -> its non-NIL elements, in order
  - `Z2B.PRESENT` = `'Z2B.PV' BIND 0 Z2B.PV LENGTH 1 SUB RANGE [ 'Z2B.PK' BIND Z2B.PV Z2B.PK GET NIL? NOT ] FILTER 'Z2B.PI' BIND Z2B.PV Z2B.PI GET` — Vector with NILs -> Vector of the present readings in order
  - `Z3A.PRESENT` = `[ NIL? NOT ] FILTER` — keep only the elements that are not NIL
  - `Z3B.FILLED` = `'ZV' BIND ZV ZV [ NIL? NOT ] FILTER 0 GET [ 'ZE' BIND ZE ZE NIL? SELECT ] SCAN` — forward-fill NILs, leading NILs take the first present value
  - `Z3D.PRESENT` = `[ NIL? NOT ] FILTER` — Vector -> the same Vector without its NIL elements
  - `Z3D.FILL` = `'Z3D.FV' BIND Z3D.FV Z3D.FV Z3D.PRESENT 0 GET [ 'Z3D.FE' BIND Z3D.FE Z3D.FE NIL? SELECT ] SCAN` — Vector with NILs -> each NIL replaced by the nearest present value before it (leading NILs take the first present value)
  - `S0A.PRESENT` = `[ NIL? NOT ] FILTER` — vector -> its non-NIL elements
  - `S0A.FILLED` = `'FV' BIND FV FV S0A.PRESENT 0 GET [ 'FE' BIND 'FA' BIND FA FE FE NIL? SELECT ] SCAN` — readings with NILs -> forward-filled, leading NILs take first present
  - `S0D.DEDUPE` = `'V' BIND V V S0D.STARTS GET` — collapse each run of equal adjacent values to one value
  - `S0D.PRESENT` = `[ NIL? NOT ] FILTER` — the non-NIL elements of a Vector
  - `S0D.FILLED` = `'V' BIND V V S0D.PRESENT 0 GET [ 'X' BIND 'A' BIND A X X NIL? SELECT ] SCAN` — forward-fill NILs with the nearest earlier present value (first present value before it)
  - `U1D.PRESENT` = `[ NIL? NOT ] FILTER` — Vector -> its non-NIL elements, in order
  - `U1D.FFILL` = `'U1D.FV' BIND U1D.FV U1D.FV U1D.PRESENT 0 GET [ 'U1D.FE' BIND 'U1D.FA' BIND U1D.FA U1D.FE U1D.FE NIL? SELECT ] SCAN` — Vector with NILs (at least one present) -> forward-filled Vector; leading NILs take the first present value
  - `U2D.PRESENT` = `[ NIL? NOT ] FILTER` — V -> the same Vector with every NIL element removed, order kept
  - `U2D.FILLED` = `'FV' BIND FV FV U2D.PRESENT 0 GET [ 'FE' BIND 'FA' BIND FA FE FE NIL? SELECT ] SCAN` — V with NIL gaps (at least one present) -> forward-filled V; leading NILs take the first present value
  - `U3B.PRESENT` = `[ NIL? NOT ] FILTER` — Vector with NILs -> Vector of its present (non-NIL) elements in order
  - `U3B.FILLED` = `'FX' BIND FX FX U3B.PRESENT 0 GET [ 'FE' BIND 'FA' BIND FA FE FE NIL? SELECT ] SCAN` — Vector with NILs (at least one present) -> same length, each NIL replaced by nearest earlier present, leading NILs by the first present
  - `U3D.PRESENT` = `[ NIL? NOT ] FILTER` — Vector -> the same Vector with every NIL element removed
- **19 agents** (G0B, R1D, R1A, R3A, E0D, E1B, E1D, E2A, E2D, Z1D, Z2A, Z2B, Z3C, S0A, S0B, U1A, U1D, U3B, U3D; models sonnet, fable, opus, haiku), joined at d1/d0a:
  - `G0B.TITLECASE` = `'TW' BIND TW CHARS 0 GET UPPER TW CHARS 1 DROP JOIN 2 COLLECT JOIN` — Lower-case word in, same word with first letter upper-cased.
  - `R1D.CAPITALIZE` = `LOWER CHARS 'R1D.TC' BIND R1D.TC 0 GET UPPER 1 COLLECT R1D.TC 1 DROP CONCAT JOIN` — non-empty word -> the word with its first letter upper case and the rest lower case
  - `R1A.TITLEWORD` = `CHARS 'R1A.TC' BIND R1A.TC 1 TAKE JOIN UPPER R1A.TC 1 DROP JOIN LOWER 2 COLLECT JOIN` — non-empty word -> first letter upper, rest lower
  - `R3A.TITLECASE` = `CHARS 'R3A.TC' BIND R3A.TC 1 TAKE JOIN UPPER R3A.TC 1 DROP JOIN LOWER 2 COLLECT JOIN` — non-empty word -> first letter upper case, rest lower case
  - `E0D.CAPITALIZE` = `CHARS 'CS' BIND CS 1 TAKE JOIN UPPER CS 1 DROP JOIN LOWER 2 COLLECT JOIN` — non-empty word -> first letter upper case, rest lower case
  - `E1B.CAP` = `LOWER CHARS 'TW' BIND TW 0 GET UPPER TW 1 DROP JOIN 2 COLLECT JOIN` — Word (String) -> first letter upper, rest lower
  - `E1D.CAP` = `LOWER CHARS 'E1D.C' BIND E1D.C 0 GET UPPER E1D.C 1 DROP JOIN 2 COLLECT JOIN` — word String -> the word with its first letter upper-cased and the rest lower-cased
  - `E2A.CAP` = `CHARS 'EC' BIND EC 0 GET UPPER EC 1 DROP JOIN LOWER 2 COLLECT JOIN` — non-empty word -> first letter upper case, rest lower case
  - `E2D.CAPWORD` = `CHARS 'C' BIND C 0 GET UPPER 1 COLLECT C 1 DROP CONCAT JOIN` — non-empty String -> same String with the first character upper-cased and the rest kept as given
  - `Z1D.CAP` = `LOWER 'Z1D.CW' BIND Z1D.CW CHARS 1 TAKE JOIN UPPER Z1D.CW CHARS 1 DROP JOIN 2 COLLECT JOIN` — word -> first letter upper case, rest lower case
  - `Z2A.CAP` = `LOWER CHARS 'Z2A.CC' BIND Z2A.CC 0 GET UPPER Z2A.CC 1 DROP JOIN 2 COLLECT JOIN` — Word -> first letter upper case, rest lower case
  - `Z2B.CAP` = `LOWER CHARS 'Z2B.TC' BIND Z2B.TC 0 GET UPPER 1 COLLECT Z2B.TC 1 DROP CONCAT JOIN` — word -> first letter upper, rest lower
  - `Z3C.CAP` = `'WD' BIND WD CHARS 0 GET UPPER WD CHARS 1 DROP JOIN LOWER 2 COLLECT JOIN` — Word with its first letter upper case and the rest lower case.
  - `S0A.CAP` = `CHARS 'TC' BIND TC 1 TAKE JOIN UPPER TC 1 DROP JOIN LOWER 2 COLLECT JOIN` — non-empty word -> first letter upper, rest lower
  - `S0B.CAP` = `LOWER CHARS 'CC' BIND CC 0 CC 0 GET UPPER PUT JOIN` — Word -> first letter upper, rest lower.
  - `U1A.TITLEWORD` = `CHARS 'C' BIND C 1 TAKE JOIN UPPER C 1 DROP JOIN LOWER 2 COLLECT JOIN` — non-empty word -> first letter upper case, rest lower case
  - `U1D.CAP` = `LOWER 'U1D.CW' BIND U1D.CW CHARS 0 GET UPPER U1D.CW CHARS 1 DROP JOIN 2 COLLECT JOIN` — non-empty word -> first letter upper case, the rest lower case
  - `U3B.CAP` = `LOWER 'CW' BIND CW CHARS 0 GET UPPER CW CHARS 1 DROP JOIN 2 COLLECT JOIN` — word String -> first letter upper case, rest lower case
  - `U3D.TITLECASE` = `'U3D.TW' BIND U3D.TW CHARS 0 GET UPPER U3D.TW CHARS 1 DROP JOIN LOWER 2 COLLECT JOIN` — non-empty word -> first character upper case, the rest lower case
- **18 agents** (G0B, U2B, R1A, R1B, R3B, R3C, R3D, E0A, E0B, E2A, E2B, E2C, S0A, S0B, S0C, S0D, U2A, U2D; models sonnet, opus, haiku, fable), joined at d1/d0/d0a:
  - `G0B.REDUCE` = `'()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE` — String of brackets in, the String left after deleting adjacent () and [] pairs six times (enough for 12 chars).
  - `O1C.CANCEL` = `'()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE` — Bracket String -> String left after six rounds of deleting adjacent () and [] pairs (enough for 12 chars).
  - `X1.TODIPU` = `'()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE`
  - `X2.SITUBO` = `'()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE`
  - `X3.ZOSEBI` = `'()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE`
  - `U2B.BRED` = `'()' '' REPLACE '[]' '' REPLACE` — String of brackets -> the String with each adjacent () and [] pair deleted once
  - `R1A.REDUCE` = `CHARS '' [ R1A.RSTEP ] FOLD` — bracket String -> what remains after deleting adjacent () and [] pairs repeatedly
  - `R1B.RED` = `'()' '' REPLACE '[]' '' REPLACE` — one pass deleting every adjacent () and [] pair from a String
  - `R3B.BPASS` = `'()' '' REPLACE '[]' '' REPLACE` — Bracket String -> the String with every adjacent () and [] pair deleted once.
  - `R3B.BREDUCE` = `R3B.BPASS R3B.BPASS R3B.BPASS R3B.BPASS R3B.BPASS R3B.BPASS` — Bracket String of at most 12 characters -> what remains after deleting matched pairs until none is left.
  - `R3C.REDUCE` = `'RS' BIND 1 12 RANGE RS [ 'E' BIND '()' '' REPLACE '[]' '' REPLACE ] FOLD` — Bracket String reduced by repeatedly deleting adjacent () and [] pairs (12 passes).
  - `R3D.REDUCE` = `'R3D.RS' BIND 1 6 RANGE R3D.RS [ 'R3D.RX' BIND '()' '' REPLACE '[]' '' REPLACE ] FOLD` — bracket String of at most 12 chars -> the String after repeatedly deleting adjacent () and [] pairs
  - `E0A.REDUCE` = `'S' BIND 0 S CHARS LENGTH RANGE S [ 'I' BIND '()' '' REPLACE '[]' '' REPLACE ] FOLD` — bracket String -> what remains after repeatedly deleting adjacent '()' and '[]' pairs
  - `E0B.REDUCE` = `'S' BIND 0 6 RANGE S [ 'E' BIND '()' '' REPLACE '[]' '' REPLACE ] FOLD` — Bracket String -> String left after repeatedly deleting adjacent () and [] pairs
  - `E2A.BRREDUCE` = `'ES' BIND 0 ES CHARS LENGTH 2 DIV FLOOR RANGE ES [ 'EI' BIND '()' '' REPLACE '[]' '' REPLACE ] FOLD` — bracket String -> what remains after repeatedly deleting adjacent () and [] pairs
  - `E2B.REDUCE` = `'()' '' REPLACE '[]' '' REPLACE` — String -> String with every adjacent () and [] pair deleted once
  - `E2B.REDUCE6` = `E2B.REDUCE E2B.REDUCE E2B.REDUCE E2B.REDUCE E2B.REDUCE E2B.REDUCE` — bracket String (<=12 chars) -> what remains after deleting matched pairs until none is left
  - `E2C.REDUCE` = `'S' BIND 1 6 RANGE S [ 'K' BIND '()' '' REPLACE '[]' '' REPLACE ] FOLD` — String -> String with every adjacent () and [] pair removed until none is left (six passes suffice for 12 characters)
  - `Z1C.RED` = `'()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE` — Deletes adjacent () and [] pairs for six rounds; this reaches the normal form for any String of at most twelve brackets.
  - `Z2A.REDUCE` = `'()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE` — Delete adjacent () and [] pairs, seven rounds (enough for 12 characters)
  - `Z2B.REDUCE` = `'()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE` — delete adjacent matched bracket pairs until none remain (up to 7 rounds)
  - `Z2C.REDUCE` = `'()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE` — Input a String of brackets; deletes adjacent () and [] pairs in six rounds (enough for twelve characters) and leaves the irreducible residue.
  - `Z3C.RED` = `'SS' BIND 1 8 RANGE SS [ Z3C.REDSTEP ] FOLD` — Bracket String reduced to its irreducible remainder (8 rounds cover 12 characters).
  - `S0A.BREDUCE` = `'()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE` — bracket String (<= 12 chars) -> what remains after deleting matched adjacent pairs repeatedly
  - `S0B.REDUCED` = `'()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE` — String of ()[] -> String after 6 passes deleting adjacent () and [] pairs.
  - `S0C.REDUCE` = `'RS' BIND 1 6 RANGE RS [ 'IG' BIND '()' '' REPLACE '[]' '' REPLACE ] FOLD` — Input String of brackets; six rounds of deleting adjacent matched round and square pairs, answering what remains (the reduced form is unique).
  - `S0D.BREDUCE1` = `'()' '' REPLACE '[]' '' REPLACE` — one pass deleting adjacent matched pairs () and []
  - `S0D.BREDUCE` = `S0D.BREDUCE1 S0D.BREDUCE1 S0D.BREDUCE1 S0D.BREDUCE1 S0D.BREDUCE1 S0D.BREDUCE1` — delete adjacent matched pairs until none remain (up to 12 chars)
  - `U2A.REDUCE` = `'RS' BIND 1 8 RANGE RS [ 'RE' BIND '()' '' REPLACE '[]' '' REPLACE ] FOLD` — bracket String (up to 16 chars) -> what is left after repeatedly deleting adjacent () and [] pairs
  - `U2B.BREDUCE` = `U2B.BRED U2B.BRED U2B.BRED U2B.BRED U2B.BRED U2B.BRED` — String of up to 12 brackets -> what remains after deleting matched adjacent pairs until none is left
  - `U2D.BREDUCE` = `'RS' BIND 1 7 RANGE RS [ 'RI' BIND '()' '' REPLACE '[]' '' REPLACE ] FOLD` — bracket String (up to 14 chars) -> what remains after repeatedly deleting adjacent '()' and '[]' pairs; empty exactly when balanced
  - `U3A.RED6` = `'()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE` — bracket String (up to 12 chars) -> fixpoint after repeatedly deleting adjacent () and [] pairs
  - `U3B.REDUCE` = `U2B.BRED U2B.BRED U2B.BRED U2B.BRED U2B.BRED U2B.BRED` — bracket String of at most 12 chars -> String left after deleting adjacent () and [] pairs until none remain
  - `U3C.RED` = `'S' BIND 1 6 RANGE S [ 'E' BIND U2B.BRED ] FOLD` — Bracket String -> its unmatched remainder after six passes of U2B.BRED (enough for up to twelve characters).
  - `U3D.BREDUCE` = `'U3D.BS' BIND 1 8 RANGE U3D.BS [ 'U3D.BI' BIND U2B.BRED ] FOLD` — String of brackets (at most 16 chars) -> what remains after repeatedly deleting adjacent () and [] pairs (8 passes of U2B.BRED)
- **17 agents** (O1A, R1A, U1D, O1D, R1D, R3A, R3C, R3D, E1D, Z1A, Z2D, Z3B, Z3D, S0A, U1A, U1B, U1C; models opus, fable, haiku, sonnet), joined at d1/d0a:
  - `O1A.TOPKEY` = `'O1A.KR' BIND O1A.KR KEYS O1A.KR VALUES -1 MUL ORDER 0 GET GET` — Record with numeric values -> first key holding the largest value
  - `R1A.ARGMAX` = `'R1A.AR' BIND R1A.AR KEYS 0 R1A.AR VALUES SUB ORDER 0 GET GET` — Record of numbers -> key with the largest value, earliest key on ties
  - `U1D.TOPKEY` = `'U1D.TR' BIND U1D.TR KEYS U1D.TR VALUES U1D.ARGMAX GET` — Record with numeric values -> the first key holding the largest value
  - `O1D.ARGMAX-KEY` = `'R' BIND R KEYS R VALUES -1 MUL ORDER 0 GET GET` — Record with numeric values -> the key holding the largest value; ties go to the earliest key
  - `R1D.ARGMAX-KEY` = `'R1D.AR' BIND R1D.AR KEYS R1D.AR VALUES -1 MUL ORDER 0 GET GET` — Record with numeric values -> the key holding the largest value; on a tie, the earliest key
  - `R3A.ARGMAXKEY` = `'R3A.AR' BIND R3A.AR KEYS R3A.AR VALUES -1 MUL ORDER 0 GET GET` — Record of numbers -> the key with the largest value, the earliest key on a tie
  - `R3C.TOPKEY` = `'T' BIND T KEYS 'K' BIND T VALUES 'V' BIND V 1 DROP V 0 GET [ MAX ] FOLD 'M' BIND K [ 'KK' BIND T KK GET M EQ ] FILTER 0 GET` — Record -> first key, in Record order, whose value is greatest.
  - `R3D.TOPKEY` = `'R3D.TR' BIND R3D.TR KEYS R3D.TR VALUES -1 MUL ORDER 0 GET GET` — Record with numeric values -> the key holding the largest value, the earliest key among ties
  - `E1D.TOPKEY` = `'E1D.TR' BIND E1D.TR KEYS E1D.TR VALUES E1D.ARGMAX GET` — Record with numeric values -> the key of the largest value, first key on ties
  - `Z1A.ARGMAXKEY` = `'AMR' BIND AMR KEYS AMR VALUES -1 MUL ORDER 0 GET GET` — Record of numbers -> key with the largest value, earliest key on ties
  - `Z2D.KEYOFMAX` = `'ZKR' BIND ZKR KEYS ZKR VALUES Z2D.ARGMAX GET` — Record with numeric values -> the key holding the largest value, first key on ties
  - `Z3B.TOPKEY` = `'ZRX' BIND ZRX KEYS ZRX VALUES 'ZWX' BIND ZWX SORT -1 GET ZWX SUB ORDER 0 GET GET` — from a Record of numbers: the key with the largest value, first one on ties
  - `Z3D.TOPKEY` = `'Z3D.TK' BIND Z3D.TK KEYS Z3D.TK VALUES -1 MUL ORDER 0 GET GET` — Record with numeric values -> the key with the largest value, first key on ties
  - `S0A.ARGMAXKEY` = `'AR' BIND AR KEYS AR VALUES -1 MUL ORDER 0 GET GET` — Record of numbers -> key with the largest value, first on ties
  - `U1A.ARGMAXKEY` = `'Q' BIND Q KEYS Q VALUES -1 MUL ORDER 0 GET GET` — Record of numbers -> the key with the largest value, the earliest key on ties
  - `U1B.TOPKEY` = `'TK' BIND TK KEYS TK VALUES -1 MUL ORDER 0 GET GET` — Record of numbers -> the key with the largest value, the first such key on ties
  - `U1C.PICK-MAX-KEY` = `'R' BIND R KEYS 'K' BIND R VALUES -1 MUL ORDER 0 GET 'I' BIND K I GET` — Record of counts -> its key with the largest value, the earliest key on a tie.
- **16 agents** (G0D, R2A, G0A, G0B, G0C, R2B, R2C, R2D, E0A, E0B, E0C, E0D, S0A, S0B, S0C, S0D; models fable, opus, sonnet, haiku), joined at d0/d1:
  - `G0D.WORDS` = `' ' TOKENIZE [ '' EQ NOT ] FILTER` — space-separated String -> Vector of its non-empty words
  - `R2A.WORDS` = `' ' TOKENIZE [ '' EQ NOT ] FILTER` — space-separated String -> Vector of its non-empty words
  - `X1.SEBUFE` = `' ' TOKENIZE [ '' EQ NOT ] FILTER`
  - `X2.ZAPEZO` = `' ' TOKENIZE [ '' EQ NOT ] FILTER`
  - `X3.KEGAFE` = `' ' TOKENIZE [ '' EQ NOT ] FILTER`
  - `G0A.WORDS` = `' ' TOKENIZE [ CHARS LENGTH 0 GT ] FILTER` — space-separated String -> Vector of its non-empty words
  - `G0A.LWORDS` = `G0A.WORDS [ LOWER ] MAP` — space-separated String -> Vector of its words in lower case
  - `G0B.WORDS` = `LOWER ' ' TOKENIZE [ '' EQ NOT ] FILTER` — String in, Vector of its lower-case words (empty pieces dropped).
  - `G0C.WORDS` = `' ' TOKENIZE [ 'MW' BIND MW '' EQ NOT ] FILTER` — Words of a space-separated String as a Vector, with empty pieces dropped.
  - `G0D.LOWER-WORDS` = `G0D.WORDS [ LOWER ] MAP` — space-separated String -> Vector of its words in lower case
  - `O1D.LOWER-WORDS` = `G0D.WORDS [ LOWER ] MAP` — space-separated String -> Vector of its non-empty words in lower case
  - `O3D.LWORDS` = `LOWER G0D.WORDS` — messy space-separated String -> Vector of its words in lower case, in order
  - `R2B.WORDS` = `LOWER ' ' TOKENIZE [ CHARS LENGTH 0 GT ] FILTER` — String of space separated words -> Vector of the non-empty words in lower case.
  - `R2C.WORDS` = `' ' TOKENIZE [ '' EQ NOT ] FILTER` — Text -> Vector of its non-empty space-separated words, in order, case kept
  - `R2D.WORDS` = `' ' TOKENIZE [ '' EQ NOT ] FILTER` — String of words separated by one or more spaces (spaces at the ends allowed) -> Vector of the words, in order, case kept
  - `R2D.LWORDS` = `R2D.WORDS [ LOWER ] MAP` — String of space-separated words -> Vector of the words in lower case
  - `E0A.WORDS` = `' ' TOKENIZE [ '' EQ NOT ] FILTER` — String -> Vector of its space-separated words, empty pieces dropped
  - `E0B.WORDS` = `LOWER ' ' TOKENIZE [ '' EQ NOT ] FILTER` — String -> Vector of its words in lower case, ignoring extra spaces
  - `E0C.WORDS` = `' ' TOKENIZE [ '' EQ NOT ] FILTER` — Splits a String on spaces and drops the empty pieces: the words in order.
  - `E0D.WORDS` = `' ' TOKENIZE [ '' EQ NOT ] FILTER` — String -> its space-separated words (empty tokens dropped)
  - `E2A.LWORDS` = `LOWER G0D.WORDS` — String -> Vector of its non-empty space-separated words, lower-cased
  - `E2B.LWORDS` = `LOWER G0D.WORDS` — String -> Vector of its words in lower case
  - `Z2A.WORDS` = `' ' TOKENIZE [ '' EQ NOT ] FILTER` — String -> Vector of its space-separated non-empty words
  - `Z2B.WORDS` = `' ' TOKENIZE [ '' EQ NOT ] FILTER` — String -> Vector of its non-empty space-separated words
  - `Z2D.WORDS` = `LOWER X2.ZAPEZO` — String -> Vector of its lower-cased space-separated words, empties dropped
  - `Z3A.WORDS` = `LOWER ' ' TOKENIZE [ '' EQ NOT ] FILTER` — messy text to Vector of lower-case words
  - `Z3D.WORDS` = `LOWER X3.KEGAFE` — space-separated String -> Vector of its words in lower case
  - `S0A.WORDS` = `' ' TOKENIZE [ '' EQ NOT ] FILTER` — String -> Vector of its space-separated non-empty words
  - `S0B.WORDS` = `' ' TOKENIZE [ CHARS LENGTH 0 GT ] FILTER` — String -> Vector of its non-empty space-separated words.
  - `S0C.WORDS` = `' ' TOKENIZE [ '' EQ NOT ] FILTER` — Input String; answers its nonempty space-separated words as a Vector of Strings.
  - `S0D.WORDS` = `' ' TOKENIZE [ '' EQ NOT ] FILTER` — split a String on spaces, dropping empty tokens
  - `S0D.LWORDS` = `S0D.WORDS [ LOWER ] MAP` — the words of a String, lower-cased
  - `U3D.LWORDS` = `G0D.WORDS [ LOWER ] MAP` — space-separated String -> Vector of its non-empty words in lower case
- **16 agents** (R1A, R1B, E1D, Z1A, Z1D, Z2B, Z2C, Z2D, S0D, U1A, U1B, U1D, U2A, U2B, U2C, U2D; models opus, sonnet, fable, haiku), joined at d1/d0a/d0:
  - `R1A.GAPMASK` = `[ 'R1A.GE' BIND 1 0 R1A.GE NIL? SELECT ] MAP` — Vector -> same-length Vector of 1 where NIL, 0 where present
  - `X3.NIFULO` = `[ 1 COLLECT [ NIL? ] FILTER LENGTH ] MAP`
  - `R1B.MISS` = `[ NIL? 'MB' BIND 1 0 MB SELECT ] MAP` — Vector with NILs -> Vector of 1 (missing) / 0 (present)
  - `E1D.MISSING` = `[ 'E1D.X' BIND 1 0 E1D.X NIL? SELECT ] MAP` — Vector -> same-length 0/1 Vector, 1 where the element is NIL
  - `Z1A.GAPRUN` = `[ NIL? ] MAP 0 [ 'GE' BIND 'GA' BIND GA 1 ADD 0 GE SELECT ] SCAN` — Vector -> running length of the current run of NILs at each position (0 where present)
  - `Z1D.NILMASK` = `[ 'Z1D.E' BIND 1 0 Z1D.E NIL? SELECT ] MAP` — Vector -> Vector of 1 where the element is NIL, 0 where present
  - `Z2B.MISS` = `'Z2B.MV' BIND 0 Z2B.MV LENGTH 1 SUB RANGE [ 'Z2B.MK' BIND 1 0 Z2B.MV Z2B.MK GET NIL? SELECT ] MAP` — Vector with NILs -> Vector of 1 where missing, 0 where present
  - `Z2C.RUNLEN` = `0 [ 'LANE' BIND 'A' BIND A 1 ADD 0 LANE NIL? SELECT ] SCAN` — Input a Vector of readings; leaves, per reading, the length of the NIL run ending there (0 for a present reading).
  - `Z2D.NILMASK` = `[ 1 COLLECT [ NIL? ] FILTER LENGTH ] MAP` — Vector -> Vector of 1 where the element is NIL and 0 where it is present
  - `S0D.GAPFLAGS` = `[ NIL? ] MAP 'M' BIND 1 0 M SELECT` — Vector of 1 where an element is NIL and 0 where present
  - `U1A.MISS01` = `[ 'E' BIND 1 0 E NIL? SELECT ] MAP` — Vector with NIL lanes -> same-length Vector of 1 where the lane is NIL, 0 where present
  - `U1B.NILFLAGS` = `[ 'NFE' BIND 1 0 NFE NIL? SELECT ] MAP` — Vector -> same-length Vector of 1 where the lane is NIL and 0 where it is present
  - `U1D.NILMASK` = `[ 'U1D.E' BIND 1 0 U1D.E NIL? SELECT ] MAP` — Vector -> same-length numeric Vector, 1 where the element is NIL and 0 elsewhere
  - `U2A.MISSMASK` = `[ NIL? ] MAP 'MB' BIND 1 0 MB SELECT` — Vector with NIL lanes -> same-length Vector of 1 where missing, 0 where present
  - `U2B.MISSING` = `[ NIL? 'UF' BIND 1 0 UF SELECT ] MAP` — Vector of readings -> Vector of 1 (missing, NIL) or 0 (present)
  - `U2C.FLAGS` = `'FV' BIND 1 0 FV [ NIL? ] MAP SELECT` — Vector of readings -> numeric 0/1 Vector, 1 where the reading is NIL.
  - `U2D.GAPRUNS` = `0 [ 'LE' BIND 'LA' BIND LA 1 ADD 0 LE NIL? SELECT ] SCAN` — V -> same-length Vector where each lane holds the length of the NIL run ending there (0 at a present lane)
- **15 agents** (O1D, R1C, O1B, R1D, E0C, Z1D, Z2C, Z3C, S0B, U1D, U2B, U2C, U3B, U3C, U3D; models fable, haiku, sonnet), joined at d1/d0a/d0:
  - `O1D.QTY-BY-SKU` = `'O' BIND O [ 'sku' GET ] MAP O [ 'qty' GET ] MAP GROUP 'G' BIND G KEYS G VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — Vector of order Records -> Record from each sku to the sum of its qty, skus in order of first appearance
  - `R1C.REVENUE` = `[ 'XR' BIND XR 'qty' GET XR 'price' GET MUL 0 XR 'price' GET NIL? NOT SELECT ] MAP 0 [ ADD ] FOLD` — Orders Vector -> exact total of qty times price over the orders that have a price.
  - `O1B.QTY-BY-SKU` = `'QO' BIND QO [ 'sku' GET ] MAP QO [ 'qty' GET ] MAP GROUP 'QG' BIND QG KEYS QG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — Vector of order Records -> Record from sku to summed qty, skus in order of first appearance
  - `O3A.TAG-COUNTS` = `[ O3A.TAGS-OF ] MAP FLATTEN TALLY` — Vector of order Records -> Record from each tag to the number of orders carrying it, first-appearance order
  - `R1C.QTY_BY_SKU` = `'XO' BIND XO [ 'XEA' BIND XEA 'sku' GET ] MAP 'XSK' BIND XO [ 'XEB' BIND XEB 'qty' GET ] MAP 'XQT' BIND XSK XQT GROUP 'XGR' BIND XGR KEYS 'XKK' BIND XGR VALUES [ 0 [ ADD ] FOLD ] MAP 'XVV' BIND XKK XVV RECORD` — Orders Vector -> Record from sku to total qty, skus in first-appearance order.
  - `R1D.SKU-QTY` = `'R1D.OS' BIND R1D.OS [ 'sku' GET ] MAP R1D.OS [ 'qty' GET ] MAP GROUP 'R1D.OG' BIND R1D.OG KEYS R1D.OG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — Vector of order Records -> Record from each sku to the sum of its qty, skus in order of first appearance
  - `R2A.REVENUE` = `[ R2A.LINE ] MAP 0 [ ADD ] FOLD` — orders Vector -> exact total of qty times price over priced orders
  - `R2A.QTYBYSKU` = `'R2A.QO' BIND R2A.QO [ 'sku' GET ] MAP R2A.QO [ 'qty' GET ] MAP GROUP 'R2A.QG' BIND R2A.QG KEYS R2A.QG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — orders Vector -> Record sku -> summed qty, skus in first-appearance order
  - `R2B.QBS` = `'R2B.OO' BIND R2B.OO [ 'sku' GET ] MAP R2B.OO [ 'qty' GET ] MAP GROUP 'R2B.OG' BIND R2B.OG KEYS R2B.OG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — Orders Vector -> Record from sku to its summed qty, skus in order of first appearance.
  - `R2C.QTYSKU` = `'OS' BIND OS [ 'XR' BIND XR 'sku' GET ] MAP 'SK' BIND OS [ 'XR' BIND XR 'qty' GET ] MAP 'QT' BIND SK QT GROUP 'GR' BIND GR KEYS GR VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — Orders Vector -> Record from sku to total qty, skus in first-appearance order
  - `R2D.QTY-BY-SKU` = `'R2D.QO' BIND R2D.QO [ 'sku' GET ] MAP R2D.QO [ 'qty' GET ] MAP GROUP 'R2D.QG' BIND R2D.QG KEYS R2D.QG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — orders Vector (Records with 'sku' and 'qty') -> Record from sku to summed qty, skus in order of first appearance
  - `R2D.TAG-COUNTS` = `[ 'tags' GET ] MAP R2D.PRESENT [ UNIQUE ] MAP FLATTEN TALLY` — orders Vector -> Record from tag to the number of orders carrying it, tags in order of first appearance
  - `R3A.QTYBYSKU` = `'R3A.OO' BIND R3A.OO [ 'sku' GET ] MAP R3A.OO [ 'qty' GET ] MAP GROUP 'R3A.OG' BIND R3A.OG KEYS R3A.OG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — orders Vector -> Record from sku to summed qty, skus in first-appearance order
  - `R3C.QTYSKU` = `'OS' BIND OS [ 'sku' GET ] MAP OS [ 'qty' GET ] MAP GROUP 'G' BIND G KEYS G VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — Vector of orders -> Record from sku to total qty, skus in first-appearance order.
  - `R3D.SKUQTY` = `'R3D.SO' BIND R3D.SO [ 'sku' GET ] MAP R3D.SO [ 'qty' GET ] MAP GROUP 'R3D.SG' BIND R3D.SG KEYS R3D.SG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — orders Vector -> Record from each sku to the sum of its qty, skus in order of first appearance
  - `E0C.SKUQTY` = `'OS' BIND OS [ 'sku' GET ] MAP OS [ 'qty' GET ] MAP GROUP 'G' BIND G KEYS G VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — From an orders Vector: a Record from each sku to its total qty, in first-appearance order.
  - `Z1D.QBS` = `'Z1D.OS' BIND Z1D.OS [ 'sku' GET ] MAP Z1D.OS [ 'qty' GET ] MAP GROUP 'Z1D.G' BIND Z1D.G KEYS Z1D.G VALUES [ Z1D.SUM ] MAP RECORD` — Vector of order Records -> Record sku -> total qty, skus in first-appearance order
  - `Z2C.QTYBYSKU` = `'OS' BIND OS [ 'sku' GET ] MAP UNIQUE 'K' BIND K K [ 'SK' BIND OS [ 'sku' GET SK EQ ] FILTER [ 'qty' GET ] MAP 0 [ ADD ] FOLD ] MAP RECORD` — Input the orders Vector; leaves a Record from each sku to its summed qty, skus in order of first appearance.
  - `Z3C.REVENUE` = `[ Z3C.LINE ] MAP 0 [ ADD ] FOLD` — Total of qty times price over an orders Vector.
  - `S0B.REVENUE` = `[ S0B.LINE ] MAP S0B.SUM` — Vector of order Records -> total revenue.
  - `S0B.BYSKU` = `'BO' BIND BO [ 'sku' GET ] MAP BO [ 'qty' GET ] MAP GROUP 'BG' BIND BG KEYS BG VALUES [ S0B.SUM ] MAP RECORD` — Vector of order Records -> Record sku -> summed qty.
  - `U1D.QTYS` = `'U1D.QO' BIND U1D.QO [ 'sku' GET ] MAP U1D.QO [ 'qty' GET ] MAP GROUP 'U1D.QG' BIND U1D.QG KEYS U1D.QG VALUES [ U1D.SUM ] MAP RECORD` — Vector of order Records -> Record from sku to its summed qty, skus in order of first appearance
  - `U2B.REV` = `[ 'UO' BIND UO 'price' GET 'UP' BIND UO 'qty' GET UP MUL 0 UP NIL? NOT SELECT ] MAP 0 [ ADD ] FOLD` — Vector of order Records -> total of qty times price over the orders that have a price
  - `U2B.QTYBY` = `'UO' BIND UO [ 'sku' GET ] MAP UO [ 'qty' GET ] MAP GROUP 'UG' BIND UG KEYS UG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — Vector of order Records -> Record from sku to summed qty, skus in order of first appearance
  - `U2C.QTYREC` = `'OS' BIND OS [ 'OA' BIND OA 'sku' GET ] MAP 'SK' BIND OS [ 'OB' BIND OB 'qty' GET ] MAP 'QT' BIND SK QT GROUP 'G' BIND G KEYS G VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — Vector of order Records -> Record from sku to total qty, skus in first-appearance order.
  - `U3B.QTYBYSKU` = `'QO' BIND QO [ 'sku' GET ] MAP QO [ 'qty' GET ] MAP GROUP 'QG' BIND QG KEYS QG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — Vector of order Records -> Record sku to summed qty, in order of first appearance
  - `U3C.QTYREC` = `'ORD' BIND ORD [ 'O' BIND O 'sku' GET ] MAP ORD [ 'O' BIND O 'qty' GET ] MAP GROUP 'R' BIND R KEYS R VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — Vector of order Records -> Record from each sku to its total qty, skus in order of first appearance.
  - `U3D.PRICED` = `[ 'price' GET NIL? NOT ] FILTER` — Vector of order Records -> only those with a present 'price'
- **14 agents** (G0A, R2B, G0B, R2A, R2C, R2D, E0A, E0B, E0C, E0D, S0A, S0B, S0C, S0D; models opus, sonnet, haiku, fable), joined at d0/d1/d0a:
  - `G0A.UNWORDS` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of space-free words -> one String joined by single spaces
  - `R2B.SPACEJOIN` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of Strings -> the Strings joined by single spaces.
  - `X1.FAGAMO` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM`
  - `X2.GUPOSO` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM`
  - `X3.FOZABA` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM`
  - `G0B.SPACEJOIN` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of Strings in, one String with single spaces between them.
  - `R2A.UNWORDS` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN CHARS -1 DROP JOIN` — non-empty Vector of Strings -> them joined by single spaces
  - `R2C.JOINSP` = `[ ' ' 2 COLLECT ] MAP FLATTEN JOIN TRIM` — Vector of texts -> one text with the texts joined by single spaces
  - `R2D.SPACED` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of Strings without spaces -> one String of them joined by single spaces
  - `E0A.UNWORDS` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of words -> one String joined by single spaces
  - `E0B.JOINSP` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of Strings -> one String joined with single spaces
  - `E0C.SPJOIN` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Joins a Vector of Strings with single spaces, no leading or trailing space.
  - `E0D.UNWORDS` = `[ 'X' BIND ' ' X 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of Strings -> joined with single spaces
  - `Z2A.JOINSP` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of Strings -> one String joined by single spaces
  - `Z2B.SPJOIN` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of Strings -> one String joined by single spaces
  - `S0A.JOINSP` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of space-free Strings -> joined by single spaces
  - `S0B.JOINSP` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of Strings -> one String joined by single spaces.
  - `S0C.JOINSP` = `[ 'WW' BIND ' ' WW 2 COLLECT JOIN ] MAP JOIN TRIM` — Input Vector of Strings; answers them joined by single spaces with no leading or trailing space.
  - `S0D.SPACEJOIN` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — join a Vector of Strings with single spaces
- **14 agents** (O1A, O1D, R2A, R3A, R3C, E1D, E2A, Z1A, Z2D, Z3D, U1A, U1D, U2D, U3D; models opus, fable, haiku), joined at d0a/d1/d0:
  - `O1A.RLE` = `G0D.RUNS [ 'O1A.RR' BIND O1A.RR 0 GET O1A.RR LENGTH 2 COLLECT ] MAP` — numeric V -> Vector of [ value count ] pairs, one per maximal run of equal adjacent values
  - `O1D.RLE` = `G0D.RUNS [ 'R' BIND R 0 GET R LENGTH 2 COLLECT ] MAP` — integer Vector -> run-length encoding as [ value count ] pairs, one per maximal run of equal adjacent values
  - `R2A.RLE` = `'R2A.RV' BIND R2A.RV R2A.RV 0 GET 0 2 COLLECT 1 COLLECT [ R2A.RLESTEP ] FOLD` — non-empty Vector -> run-length encoding, a Vector of [ value count ] pairs in order
  - `R3A.RLE` = `'R3A.RV' BIND R3A.RV R3A.RUNIDS R3A.RV GROUP VALUES [ 'R3A.RG' BIND R3A.RG 0 GET R3A.RG LENGTH 2 COLLECT ] MAP` — non-empty Vector -> run-length encoding, a Vector of [ value count ] pairs in order
  - `R3C.RLE` = `'RV' BIND RV 0 GET 1 ADD 1 COLLECT RV -1 DROP CONCAT RV SUB [ 0 EQ NOT ] MAP [ 'F' BIND 1 0 F SELECT ] MAP 0 [ ADD ] SCAN RV GROUP VALUES [ 'G' BIND G 0 GET G LENGTH 2 COLLECT ] MAP` — Run-length encoding of an integer Vector as [ value count ] pairs.
  - `E1D.RLE` = `G0D.RUNS [ 'E1D.R' BIND E1D.R 0 GET E1D.R LENGTH 2 COLLECT ] MAP` — Vector -> run-length encoding, a Vector of [ value count ] pairs per maximal run of equal adjacent values
  - `E2A.RLE` = `G0D.RUNS [ 'ER' BIND ER 0 GET ER LENGTH 2 COLLECT ] MAP` — numeric V -> run-length encoding: a Vector of [ value count ], one per maximal run, in order
  - `Z1A.RLE` = `X1.GUFIMI [ 'RR' BIND RR 0 GET RR LENGTH 2 COLLECT ] MAP` — Vector of numbers -> run-length encoding [ [ value count ] ... ]
  - `Z2D.RLE` = `X2.NEPEBE [ 'ZR' BIND ZR 0 GET ZR LENGTH 2 COLLECT ] MAP` — Vector of numbers -> run-length encoding as [ value count ] pairs in order
  - `Z3D.RLE` = `X3.FASENO [ 'Z3D.G' BIND Z3D.G 0 GET Z3D.G LENGTH 2 COLLECT ] MAP` — Vector -> run-length encoding, a Vector of [ value count ] per maximal run
  - `U1A.RLE` = `G0D.RUNS [ 'R' BIND R 0 GET R LENGTH 2 COLLECT ] MAP` — numeric Vector -> Vector of [ value count ] pairs, one per maximal run of equal adjacent values
  - `U1D.RLE` = `G0D.RUNS [ 'U1D.RR' BIND U1D.RR 0 GET U1D.RR LENGTH 2 COLLECT ] MAP` — numeric Vector -> run-length encoding, a Vector of [ value count ] pairs in order
  - `U2D.RLE` = `G0D.RUNS [ 'RLR' BIND RLR 0 GET RLR LENGTH 2 COLLECT ] MAP` — numeric V -> run-length encoding, a Vector of [ value count ] pairs, one per maximal run of equal adjacent values, in order
  - `U3D.RLE` = `G0D.RUNS [ 'U3D.RR' BIND U3D.RR 0 GET U3D.RR LENGTH 2 COLLECT ] MAP` — numeric V -> run-length encoding, a Vector of [ value count ] pairs, one per maximal run of equal adjacent values
- **14 agents** (O1D, O1A, R2B, R2D, R3A, E0D, E2A, Z3B, Z3D, S0A, U1D, U2D, U3B, U3D; models fable, opus, sonnet), joined at d1/d0a:
  - `O1D.MEAN` = `'P' BIND P 0 [ ADD ] FOLD P LENGTH DIV` — non-empty numeric Vector -> its exact mean
  - `O1A.PMEAN` = `O1A.PRESENT 'O1A.PM' BIND O1A.PM 0 [ ADD ] FOLD O1A.PM LENGTH DIV` — V with NILs -> exact mean of the present elements
  - `O3D.PRESENT-MEAN` = `O1D.PRESENT O1D.MEAN` — V with NILs (at least one present) -> exact mean of the present elements
  - `R2B.MEAN` = `'R2B.SV' BIND R2B.SV R2B.FILL0 0 [ ADD ] FOLD R2B.SV LENGTH R2B.SV R1A.GAPMASK 0 [ ADD ] FOLD SUB DIV` — Vector with NILs -> mean of the present readings.
  - `R2D.PRESENT-MEAN` = `R2D.PRESENT 'R2D.PP' BIND R2D.PP 0 [ ADD ] FOLD R2D.PP LENGTH DIV` — Vector of numbers and NILs with at least one number -> mean of the non-NIL elements
  - `R3A.PMEAN` = `R2D.PRESENT 'R3A.MP' BIND R3A.MP 0 [ ADD ] FOLD R3A.MP LENGTH DIV` — Vector with NILs -> exact mean of the present elements
  - `E0D.MEAN` = `'P' BIND P E0D.SUM P LENGTH DIV` — vector of numbers -> arithmetic mean
  - `E2A.MEAN` = `'EP' BIND EP 0 [ ADD ] FOLD EP LENGTH DIV` — non-empty numeric V -> arithmetic mean
  - `E2A.PMEAN` = `E2A.PRESENT E2A.MEAN` — V with NILs -> mean of the present values
  - `Z3B.PMEAN` = `[ NIL? NOT ] FILTER 'ZP' BIND ZP 0 [ ADD ] FOLD ZP LENGTH DIV` — mean of the present readings of a Vector that may hold NIL
  - `Z3D.MEAN` = `'Z3D.MS' BIND Z3D.MS 0 [ ADD ] FOLD Z3D.MS LENGTH DIV` — non-empty Vector of numbers -> its mean
  - `S0A.MEAN` = `'MP' BIND MP 0 [ ADD ] FOLD MP LENGTH DIV` — non-empty numeric vector -> arithmetic mean
  - `U1D.MEAN` = `'U1D.MM' BIND U1D.MM U1D.SUM U1D.MM LENGTH DIV` — non-empty numeric Vector -> its mean
  - `U2D.MEAN` = `'MNV' BIND MNV 0 [ ADD ] FOLD MNV LENGTH DIV` — non-empty numeric V -> its arithmetic mean
  - `U3B.PMEAN` = `U3B.PRESENT 'PX' BIND PX 0 [ ADD ] FOLD PX LENGTH DIV` — Vector with NILs -> exact mean of the present elements
  - `U3D.MEAN` = `'U3D.MNV' BIND U3D.MNV U3D.SUM U3D.MNV LENGTH DIV` — non-empty numeric Vector -> its mean
- **13 agents** (G0D, R1C, R2D, R3D, E0A, E0D, Z1A, Z1D, S0A, S0D, U1D, U3B, U3D; models fable, haiku, opus, sonnet), joined at d0a/d0:
  - `G0D.MOD` = `'MB' BIND 'MA' BIND MA MA MB DIV FLOOR MB MUL SUB` — a b G0D.MOD -> a modulo b (a - b*floor(a/b))
  - `X2.LELIDA` = `'Z1D.MB' BIND 'Z1D.MA' BIND Z1D.MA Z1D.MA Z1D.MB DIV FLOOR Z1D.MB MUL SUB`
  - `R1C.MOD` = `'MM' BIND 'MX' BIND MX MX MM DIV FLOOR MM MUL SUB` — x m R1C.MOD: remainder of x divided by a positive m, as x - m*floor(x/m).
  - `R2D.MOD` = `'R2D.MB' BIND 'R2D.MA' BIND R2D.MA R2D.MA R2D.MB DIV FLOOR R2D.MB MUL SUB` — a b -> a modulo b (a - b*floor(a/b)); a may be a Vector, b a scalar
  - `R3D.MOD` = `'R3D.MB' BIND 'R3D.MA' BIND R3D.MA R3D.MA R3D.MB DIV FLOOR R3D.MB MUL SUB` — a b -> a modulo b (a minus b times floor of a over b)
  - `E0A.MOD` = `'B' BIND 'A' BIND A A B DIV FLOOR B MUL SUB` — a b -> a mod b (floored remainder); a may be a Vector
  - `E0D.MOD` = `'B' BIND 'A' BIND A A B DIV FLOOR B MUL SUB` — a b -> a modulo b (floor remainder)
  - `Z1A.MOD` = `'MB' BIND 'MA' BIND MA MA MB DIV FLOOR MB MUL SUB` — a b -> a mod b (floor remainder), lifts over a Vector a
  - `Z1D.MOD` = `'Z1D.MB' BIND 'Z1D.MA' BIND Z1D.MA Z1D.MA Z1D.MB DIV FLOOR Z1D.MB MUL SUB` — a b -> a mod b (floor remainder); lifts over a Vector a
  - `Z2B.MOD` = `'Z2B.MB' BIND 'Z2B.MA' BIND Z2B.MA Z2B.MA Z2B.MB DIV FLOOR Z2B.MB MUL SUB` — a b -> a mod b
  - `S0A.MOD` = `'MB' BIND 'MA' BIND MA MA MB DIV FLOOR MB MUL SUB` — a b -> a mod b (floor remainder), lifts over vectors
  - `S0D.MOD` = `'B' BIND 'A' BIND A A B DIV FLOOR B MUL SUB` — a b -> a mod b (lifts over a Vector a)
  - `U1D.MOD` = `'U1D.MB' BIND 'U1D.MA' BIND U1D.MA U1D.MA U1D.MB DIV FLOOR U1D.MB MUL SUB` — a b -> a mod b (floor remainder); lifts over Vectors
  - `U3B.MOD` = `'MB' BIND 'MA' BIND MA MA MB DIV FLOOR MB MUL SUB` — a b -> a mod b (a minus b times floor of a over b)
  - `U3D.MOD` = `'U3D.MB' BIND 'U3D.MA' BIND U3D.MA U3D.MA U3D.MB DIV FLOOR U3D.MB MUL SUB` — a b -> a mod b (floor remainder, a - floor(a/b)*b); lifts over Vectors
- **12 agents** (G0C, G0A, G0D, R3A, R3B, R3C, E0A, E0B, E0C, E0D, S0A, S0D; models haiku, opus, fable, sonnet), joined at d0/d1/d0a:
  - `G0C.ORDERS` = `JSON-DECODE 'orders' GET` — Parse the orders JSON String into the Vector of order Records.
  - `X1.MULAFA` = `JSON-DECODE 'orders' GET`
  - `X2.FIZIPO` = `JSON-DECODE 'orders' GET`
  - `X3.SIBIGE` = `JSON-DECODE 'orders' GET`
  - `G0A.REVENUE` = `JSON-DECODE 'orders' GET [ 'G0A.RO' BIND G0A.RO 'price' GET 0 2 COLLECT 0 1 G0A.RO 'price' HAS? SELECT GET G0A.RO 'qty' GET MUL ] MAP 0 [ ADD ] FOLD` — orders JSON String -> total qty*price over priced orders
  - `G0A.QTYBYSKU` = `JSON-DECODE 'orders' GET 'G0A.KO' BIND G0A.KO [ 'sku' GET ] MAP G0A.KO [ 'qty' GET ] MAP GROUP 'G0A.KG' BIND G0A.KG KEYS G0A.KG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — orders JSON String -> Record sku -> summed qty, first-appearance order
  - `G0D.ORDERS` = `JSON-DECODE 'orders' GET` — orders JSON String -> Vector of order Records
  - `G0D.REVENUE` = `G0D.ORDERS [ 'price' HAS? ] FILTER [ 'O' BIND O 'qty' GET O 'price' GET MUL ] MAP 0 [ ADD ] FOLD` — orders JSON String -> total qty*price over priced orders
  - `G0D.QTY-BY-SKU` = `G0D.ORDERS 'QO' BIND QO [ 'sku' GET ] MAP QO [ 'qty' GET ] MAP GROUP 'QG' BIND QG KEYS QG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — orders JSON String -> Record sku -> summed qty, skus in first-appearance order
  - `O1A.QTYBY` = `G0C.ORDERS 'O1A.OR' BIND O1A.OR [ 'sku' GET ] MAP O1A.OR [ 'qty' GET ] MAP GROUP 'O1A.OG' BIND O1A.OG KEYS O1A.OG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — orders JSON String -> Record sku -> total qty, first-appearance order
  - `O1A.TAGCOUNTS` = `G0C.ORDERS [ 'tags' HAS? ] FILTER [ 'tags' GET UNIQUE ] MAP [ ] [ CONCAT ] FOLD TALLY` — orders JSON String -> Record tag -> number of orders carrying it
  - `O3B.UNPRICED` = `G0C.ORDERS [ 'O3B.UX' BIND O3B.UX 'price' HAS? NOT ] FILTER [ 'sku' GET ] MAP UNIQUE` — orders JSON String -> distinct skus of orders lacking a price, first-appearance order
  - `O3B.TAGCOUNTS` = `G0C.ORDERS [ 'O3B.TX' BIND O3B.TX 'tags' HAS? ] FILTER [ 'tags' GET UNIQUE ] MAP FLATTEN TALLY` — orders JSON String -> Record tag -> number of orders carrying it
  - `R1A.REVENUE` = `G0C.ORDERS [ R1A.OREV ] MAP 0 [ ADD ] FOLD` — orders JSON String -> total qty*price over priced orders
  - `R1A.QTYS` = `G0C.ORDERS 'R1A.OS' BIND R1A.OS [ 'sku' GET ] MAP R1A.OS [ 'qty' GET ] MAP GROUP 'R1A.G' BIND R1A.G KEYS R1A.G VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — orders JSON String -> Record sku -> summed qty, first-appearance order
  - `R1B.REVENUE` = `G0C.ORDERS [ 'ORD' BIND ORD 'price' GET 'PRC' BIND ORD 'qty' GET 0 PRC PRC NIL? SELECT MUL ] MAP 0 [ ADD ] FOLD` — orders JSON String -> total of qty times price over priced orders
  - `R1B.QBS` = `G0C.ORDERS 'QO' BIND QO [ 'sku' GET ] MAP QO [ 'qty' GET ] MAP GROUP 'QG' BIND QG KEYS QG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — orders JSON String -> Record from sku to summed qty, first-appearance order
  - `R3A.ORDERS` = `JSON-DECODE 'orders' GET` — JSON String -> the Vector of order Records under its 'orders' key
  - `R3B.QTYBYSKU` = `JSON-DECODE 'orders' GET 'QO' BIND QO [ 'sku' GET ] MAP QO [ 'qty' GET ] MAP GROUP 'QG' BIND QG KEYS QG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — Orders JSON String -> Record from each sku to the sum of its qty, in order of first appearance.
  - `R3C.ORDERS` = `JSON-DECODE 'orders' GET` — orders JSON String -> Vector of order Records.
  - `E0A.ORDERS` = `JSON-DECODE 'orders' GET` — JSON String -> Vector of the order Records under its 'orders' key
  - `E0A.REVENUE` = `E0A.ORDERS [ E0A.LINE ] MAP E0A.SUM` — orders JSON String -> total qty*price over priced orders
  - `E0A.QTYBYSKU` = `E0A.ORDERS 'O' BIND O [ 'sku' GET ] MAP O [ 'qty' GET ] MAP GROUP 'G' BIND G KEYS G VALUES [ E0A.SUM ] MAP RECORD` — orders JSON String -> Record sku -> summed qty, skus in first-appearance order
  - `E0B.QTYSKU` = `JSON-DECODE 'orders' GET 'OS' BIND OS [ 'sku' GET ] MAP OS [ 'qty' GET ] MAP GROUP 'G' BIND G KEYS G VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — Orders JSON String -> Record sku -> summed qty, in first-appearance order
  - `E0C.ORDERS` = `JSON-DECODE 'orders' GET` — Parses the JSON text and answers its orders Vector.
  - `E0D.ORDERS` = `JSON-DECODE 'orders' GET` — JSON String with an 'orders' array -> the Vector of order Records
  - `E0D.REVENUE` = `E0D.ORDERS [ 'O' BIND O 'qty' GET O 'price' GET MUL 0 O 'price' HAS? SELECT ] MAP E0D.SUM` — orders JSON String -> total qty*price over orders that have a price
  - `E0D.QTY-BY-SKU` = `E0D.ORDERS 'O' BIND O [ 'sku' GET ] MAP O [ 'qty' GET ] MAP GROUP 'G' BIND G KEYS G VALUES [ E0D.SUM ] MAP RECORD` — orders JSON String -> Record sku -> summed qty, skus in order of first appearance
  - `E1B.REVENUE` = `G0C.ORDERS [ 'ORD' BIND 0 ORD 'qty' GET ORD 'price' GET MUL ORD 'price' GET NIL? SELECT ] MAP 0 [ ADD ] FOLD` — orders JSON String -> total of qty*price over priced orders
  - `E1B.QTYSKU` = `G0C.ORDERS 'OL' BIND OL [ 'sku' GET ] MAP OL [ 'qty' GET ] MAP GROUP 'QG' BIND QG KEYS QG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — orders JSON String -> Record sku -> summed qty, first-appearance order
  - `E1D.REVENUE` = `G0C.ORDERS [ 'price' GET NIL? NOT ] FILTER [ 'E1D.O' BIND E1D.O 'qty' GET E1D.O 'price' GET MUL ] MAP E1D.SUM` — orders JSON String -> total of qty*price over the orders that have a price
  - `E1D.QTY-BY-SKU` = `G0C.ORDERS 'E1D.OS' BIND E1D.OS [ 'sku' GET ] MAP E1D.OS [ 'qty' GET ] MAP GROUP 'E1D.G' BIND E1D.G KEYS E1D.G VALUES [ E1D.SUM ] MAP RECORD` — orders JSON String -> Record from each sku to the sum of its qty, skus in first-appearance order
  - `E2A.REVENUE` = `G0C.ORDERS [ 'price' HAS? ] FILTER [ 'EO' BIND EO 'qty' GET EO 'price' GET MUL ] MAP 0 [ ADD ] FOLD` — orders JSON String -> total qty*price over orders having a price
  - `E2A.QTYBY` = `G0C.ORDERS 'EO' BIND EO [ 'sku' GET ] MAP EO [ 'qty' GET ] MAP GROUP 'EG' BIND EG KEYS EG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — orders JSON String -> Record sku -> summed qty, skus in first-appearance order
  - `E2A.UNPRICED` = `G0C.ORDERS [ 'price' HAS? NOT ] FILTER [ 'sku' GET ] MAP UNIQUE` — orders JSON String -> distinct skus of orders without a price, first-appearance order
  - `E2A.TAGCOUNTS` = `G0C.ORDERS [ 'tags' HAS? ] FILTER [ 'tags' GET UNIQUE ] MAP FLATTEN TALLY` — orders JSON String -> Record tag -> number of orders carrying it
  - `E2B.QTYSKU` = `G0C.ORDERS 'OS' BIND OS [ 'sku' GET ] MAP OS [ 'qty' GET ] MAP GROUP 'GG' BIND GG KEYS GG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — orders JSON String -> Record sku -> summed qty, skus in first-appearance order
  - `E2B.REVENUE` = `G0C.ORDERS [ 'price' GET NIL? NOT ] FILTER [ 'O' BIND O 'qty' GET O 'price' GET MUL ] MAP 0 [ ADD ] FOLD` — orders JSON String -> total of qty times price over the priced orders
  - `E2C.REVENUE` = `G0C.ORDERS [ 'O' BIND O 'qty' GET O 'price' GET MUL 'X' BIND 0 X X NIL? SELECT ] MAP 0 [ ADD ] FOLD` — orders JSON String -> total of qty times price, an order without a price counting 0
  - `E2C.QTYSKU` = `G0C.ORDERS 'OS' BIND OS [ 'sku' GET ] MAP OS [ 'qty' GET ] MAP GROUP 'G' BIND G KEYS G VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — orders JSON String -> Record from sku to summed qty, skus in first-appearance order
  - `E2D.QTYBYSKU` = `G0C.ORDERS 'O' BIND O [ 'sku' GET ] MAP O [ 'qty' GET ] MAP GROUP 'G' BIND G KEYS G VALUES [ E2D.SUM ] MAP RECORD` — orders JSON String -> Record from each sku to the sum of its qty, skus in order of first appearance
  - `E2D.REVENUE` = `G0C.ORDERS [ 'price' HAS? ] FILTER [ 'O' BIND O 'qty' GET O 'price' GET MUL ] MAP E2D.SUM` — orders JSON String -> total of qty times price over the orders that have a price
  - `Z1A.REVENUE` = `X1.MULAFA [ 'price' HAS? ] FILTER [ 'RO' BIND RO 'qty' GET RO 'price' GET MUL ] MAP 0 [ ADD ] FOLD` — orders JSON String -> total qty*price over priced orders
  - `Z1A.QTYSKU` = `X1.MULAFA 'OS' BIND OS [ 'sku' GET ] MAP OS [ 'qty' GET ] MAP GROUP 'G' BIND G KEYS G VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — orders JSON String -> Record sku -> summed qty, first-appearance order
  - `Z2A.REVENUE` = `JSON-DECODE 'orders' GET [ Z2A.OREV ] MAP 0 [ ADD ] FOLD` — Orders JSON String -> total qty*price over priced orders
  - `Z2A.QTYREC` = `JSON-DECODE 'orders' GET 'Z2A.QO' BIND Z2A.QO [ 'sku' GET ] MAP Z2A.QO [ 'qty' GET ] MAP GROUP 'Z2A.QG' BIND Z2A.QG KEYS Z2A.QG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — Orders JSON String -> Record sku -> summed qty, first-appearance order
  - `Z2B.ORDERS` = `JSON-DECODE 'orders' GET` — JSON text -> Vector of order Records
  - `Z2B.REVENUE` = `Z2B.ORDERS [ 'price' HAS? ] FILTER [ 'Z2B.OR' BIND Z2B.OR 'qty' GET Z2B.OR 'price' GET MUL ] MAP 0 [ ADD ] FOLD` — JSON text -> sum of qty*price over priced orders
  - `Z2B.QTYBYSKU` = `Z2B.ORDERS 'Z2B.OV' BIND Z2B.OV [ 'sku' GET ] MAP Z2B.OV [ 'qty' GET ] MAP GROUP 'Z2B.OG' BIND Z2B.OG KEYS Z2B.OG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — JSON text -> Record sku to summed qty
  - `Z2D.REVENUE` = `X2.FIZIPO [ 'price' HAS? ] FILTER [ 'ZO' BIND ZO 'qty' GET ZO 'price' GET MUL ] MAP 0 [ ADD ] FOLD` — orders JSON String -> total of qty times price over the orders that have a price
  - `Z2D.QTYBYSKU` = `X2.FIZIPO 'ZO' BIND ZO [ 'sku' GET ] MAP ZO [ 'qty' GET ] MAP GROUP 'ZG' BIND ZG KEYS ZG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — orders JSON String -> Record from sku (first-appearance order) to the sum of its qty
  - `Z3A.REVENUE` = `JSON-DECODE 'orders' GET [ 'price' HAS? ] FILTER [ 'Z3A.O' BIND Z3A.O 'qty' GET Z3A.O 'price' GET MUL ] MAP Z3A.SUM` — orders JSON String to total qty*price over priced orders
  - `Z3A.QTYBYSKU` = `JSON-DECODE 'orders' GET 'Z3A.L' BIND Z3A.L [ 'sku' GET ] MAP Z3A.L [ 'qty' GET ] MAP GROUP 'Z3A.G' BIND Z3A.G KEYS Z3A.G VALUES [ Z3A.SUM ] MAP RECORD` — orders JSON String to Record sku -> summed qty, first-appearance order
  - `Z3B.REVENUE` = `X3.SIBIGE [ 'ZO' BIND ZO 'qty' GET ZO 'price' GET MUL 'ZT' BIND 0 ZT ZT NIL? SELECT ] MAP 0 [ ADD ] FOLD` — from the orders JSON String: total of qty*price over orders that have a price
  - `Z3B.QTYBYSKU` = `X3.SIBIGE 'ZL' BIND ZL [ 'sku' GET ] MAP ZL [ 'qty' GET ] MAP GROUP 'ZG' BIND ZG KEYS ZG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — from the orders JSON String: Record sku -> summed qty, in first-appearance order
  - `Z3D.REVENUE` = `X3.SIBIGE [ 'price' HAS? ] FILTER [ 'Z3D.OR' BIND Z3D.OR 'qty' GET Z3D.OR 'price' GET MUL ] MAP 0 [ ADD ] FOLD` — orders JSON String -> total of qty*price over priced orders
  - `Z3D.QTYBYSKU` = `X3.SIBIGE 'Z3D.OO' BIND Z3D.OO [ 'sku' GET ] MAP Z3D.OO [ 'qty' GET ] MAP GROUP 'Z3D.OG' BIND Z3D.OG KEYS Z3D.OG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — orders JSON String -> Record sku -> summed qty, skus in order of first appearance
  - `Z3D.TAGCOUNTS` = `X3.SIBIGE [ 'tags' HAS? ] FILTER [ 'tags' GET ] MAP FLATTEN TALLY` — orders JSON String -> Record tag -> number of orders carrying it
  - `S0A.ORDERS` = `JSON-DECODE 'orders' GET` — JSON String -> Vector of order Records
  - `S0A.REVENUE` = `S0A.ORDERS [ S0A.LINEREV ] MAP 0 [ ADD ] FOLD` — orders JSON String -> total qty*price over priced orders
  - `S0A.QTYBYSKU` = `S0A.ORDERS 'QO' BIND QO [ 'sku' GET ] MAP QO [ 'qty' GET ] MAP GROUP 'QG' BIND QG KEYS QG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — orders JSON String -> Record sku -> summed qty
  - `S0A.TAGCOUNTS` = `S0A.ORDERS [ 'tags' GET NIL? NOT ] FILTER [ 'tags' GET UNIQUE ] MAP FLATTEN TALLY` — orders JSON String -> Record tag -> number of orders carrying it
  - `S0D.ORDERS` = `JSON-DECODE 'orders' GET` — JSON String -> Vector of order Records
  - `S0D.REVENUE` = `S0D.ORDERS [ 'O' BIND O 'qty' GET 0 O 'price' GET 'P' BIND P P NIL? SELECT MUL ] MAP 0 [ ADD ] FOLD` — sum of qty*price over priced orders
  - `S0D.QTYBYSKU` = `S0D.ORDERS 'O' BIND O [ 'sku' GET ] MAP O [ 'qty' GET ] MAP GROUP 'G' BIND G KEYS G VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — Record sku -> total qty, skus in order of first appearance
  - `S0D.TAGCOUNTS` = `S0D.ORDERS [ 'O' BIND [ ] O 'tags' GET 2 COLLECT 1 0 O 'tags' HAS? SELECT GET UNIQUE ] MAP FLATTEN TALLY` — Record tag -> number of orders carrying it
  - `U1A.REVENUE` = `G0C.ORDERS [ U1A.LINE ] MAP 0 [ ADD ] FOLD` — orders JSON String -> total qty*price over priced orders
  - `U1A.QTYREC` = `G0C.ORDERS 'O' BIND O [ 'sku' GET ] MAP O [ 'qty' GET ] MAP GROUP 'G' BIND G KEYS G VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — orders JSON String -> Record sku -> summed qty, skus in first-appearance order
  - `U1C.REVENUE` = `G0C.ORDERS [ 'price' HAS? ] FILTER 'PO' BIND PO [ 'qty' GET ] MAP PO [ 'price' GET ] MAP MUL 0 [ ADD ] FOLD` — Order JSON String -> total of qty times price over the orders that carry a price (exact rational).
  - `U1C.QTY-BY-SKU` = `G0C.ORDERS 'O' BIND O [ 'sku' GET ] MAP 'S' BIND O [ 'qty' GET ] MAP 'Q' BIND S Q GROUP 'R' BIND R KEYS R VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — Order JSON String -> Record from each sku to the sum of its qty, skus in first-appearance order.
  - `U1D.REVENUE` = `G0C.ORDERS [ 'price' HAS? ] FILTER [ 'U1D.RO' BIND U1D.RO 'qty' GET U1D.RO 'price' GET MUL ] MAP U1D.SUM` — orders JSON String -> total of qty times price over the orders that have a price
  - `U2A.REVENUE` = `G0C.ORDERS [ U2A.LINEREV ] MAP 0 [ ADD ] FOLD` — orders JSON String -> total of qty times price over priced orders
  - `U2A.QTYBYSKU` = `G0C.ORDERS 'QO' BIND QO [ 'sku' GET ] MAP QO [ 'qty' GET ] MAP GROUP 'QG' BIND QG KEYS QG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — orders JSON String -> Record sku -> summed qty, skus in first-appearance order
  - `U2D.PRICED` = `G0C.ORDERS [ 'price' GET NIL? NOT ] FILTER` — orders JSON String -> Vector of the order Records that carry a price
  - `U2D.REVENUE` = `U2D.PRICED [ 'ORD' BIND ORD 'qty' GET ORD 'price' GET MUL ] MAP 0 [ ADD ] FOLD` — orders JSON String -> total of qty times price over the priced orders
  - `U2D.QTYBYSKU` = `G0C.ORDERS 'OO' BIND OO [ 'sku' GET ] MAP OO [ 'qty' GET ] MAP GROUP 'QG' BIND QG KEYS QG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — orders JSON String -> Record from each sku to the sum of its qty, skus in order of first appearance
  - `U3A.REVENUE` = `JSON-DECODE 'orders' GET [ 'U3A.O' BIND U3A.O 'price' GET 'U3A.PP' BIND 0 U3A.PP U3A.PP NIL? SELECT U3A.O 'qty' GET MUL ] MAP 0 [ ADD ] FOLD` — orders JSON String -> total of qty*price over orders with a price (missing price counts 0)
  - `U3A.QTYREC` = `JSON-DECODE 'orders' GET 'U3A.OS' BIND U3A.OS [ 'sku' GET ] MAP U3A.OS [ 'qty' GET ] MAP GROUP 'U3A.G' BIND U3A.G KEYS U3A.G VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — orders JSON String -> Record sku -> summed qty, skus in first-appearance order
  - `U3B.REVENUE` = `G0C.ORDERS [ 'OO' BIND 0 OO 'qty' GET OO 'price' GET MUL OO 'price' GET NIL? SELECT ] MAP 0 [ ADD ] FOLD` — orders JSON String -> total of qty times price over orders that have a price
  - `U3D.REVENUE` = `G0C.ORDERS U3D.PRICED [ 'U3D.RO' BIND U3D.RO 'qty' GET U3D.RO 'price' GET MUL ] MAP U3D.SUM` — orders JSON String -> total of qty * price over the priced orders
  - `U3D.QTYBYSKU` = `G0C.ORDERS 'U3D.QO' BIND U3D.QO [ 'sku' GET ] MAP U3D.QO [ 'qty' GET ] MAP GROUP 'U3D.QG' BIND U3D.QG KEYS U3D.QG VALUES [ U3D.SUM ] MAP RECORD` — orders JSON String -> Record sku -> summed qty, skus in order of first appearance
- **12 agents** (E0A, E1A, E2A, E2B, E2C, Z1A, Z2D, Z3D, S0A, S0B, U3C, U3D; models opus, sonnet, haiku, fable), joined at d1/d0:
  - `E0A.BALANCED` = `E0A.REDUCE '' EQ` — bracket String -> TRUE when every ( ) [ ] is properly matched and nested
  - `E1A.ISBAL` = `G0B.REDUCE '' EQ` — bracket String (<=12 chars) -> TRUE when balanced and properly nested
  - `E2A.BALANCED` = `E2A.BRREDUCE '' EQ` — bracket String -> TRUE when properly nested and matched
  - `E2B.BAL` = `E2B.REDUCE6 '' EQ` — bracket String (<=12 chars) -> TRUE when properly nested and balanced
  - `E2C.BALANCED` = `E2C.REDUCE '' EQ` — String of brackets -> TRUE when the reduction leaves nothing, else FALSE
  - `Z1A.BALANCED` = `X1.TODIPU '' EQ` — bracket String of at most 12 chars -> TRUE when ( ) [ ] are properly matched
  - `Z2D.BALANCED` = `X2.SITUBO '' EQ` — bracket String -> TRUE when it reduces to nothing by deleting adjacent () and [] pairs
  - `Z3D.BALANCED` = `X3.ZOSEBI '' EQ` — bracket String (<= 12 chars) -> TRUE when properly balanced
  - `S0A.BALANCED` = `S0A.BREDUCE '' EQ` — bracket String -> TRUE when properly nested and matched
  - `S0B.ISBAL` = `S0B.REDUCED CHARS LENGTH 0 EQ` — String of ()[] -> TRUE when properly nested.
  - `U3C.BAL` = `U3C.RED CHARS LENGTH 0 EQ` — Bracket String -> TRUE when it reduces to the empty String, i.e. when it is balanced.
  - `U3D.BALANCED` = `U3D.BREDUCE '' EQ` — String of brackets (at most 16 chars) -> TRUE when properly nested and closed, FALSE otherwise
- **11 agents** (G0D, G0A, E0A, E0D, S0A, S0C, S0D, U3A, U3B, U3C, U3D; models fable, opus, haiku, sonnet), joined at d0/d1/d0a:
  - `G0D.DIVISORS` = `'DK' BIND 1 DK RANGE [ 'DD' BIND DK DD DIV FLOOR DD MUL DK EQ ] FILTER` — positive integer -> ascending Vector of its positive divisors
  - `X1.LAVIPA` = `'DK' BIND 1 DK RANGE [ 'DD' BIND DK DD DIV FLOOR DD MUL DK EQ ] FILTER`
  - `X2.TIGOLI` = `'DK' BIND 1 DK RANGE [ 'DD' BIND DK DD DIV FLOOR DD MUL DK EQ ] FILTER`
  - `X3.VEFAZE` = `'DK' BIND 1 DK RANGE [ 'DD' BIND DK DD DIV FLOOR DD MUL DK EQ ] FILTER`
  - `G0A.DIVISORS` = `'G0A.DN' BIND 1 G0A.DN RANGE [ 'G0A.DD' BIND G0A.DN G0A.DN G0A.DD DIV FLOOR G0A.DD MUL SUB 0 EQ ] FILTER` — positive integer -> Vector of its positive divisors, ascending
  - `E0A.DIVISORS` = `'N' BIND 1 N RANGE [ 'D' BIND N D E0A.MOD 0 EQ ] FILTER` — positive integer -> Vector of its positive divisors, ascending
  - `E0D.DIVISORS` = `'N' BIND 1 N RANGE [ 'D' BIND N D E0D.MOD 0 EQ ] FILTER` — positive integer -> its positive divisors ascending
  - `Z2B.DIVS` = `'Z2B.DK' BIND 1 Z2B.DK RANGE [ 'Z2B.DD' BIND Z2B.DK Z2B.DD DIV FLOOR Z2B.DD MUL Z2B.DK EQ ] FILTER` — positive integer -> ascending Vector of its divisors
  - `S0A.DIVISORS` = `'DN' BIND 1 DN RANGE [ 'DD' BIND DN DD S0A.MOD 0 EQ ] FILTER` — positive integer -> its positive divisors ascending
  - `S0C.DIVS` = `'DV' BIND 1 DV RANGE [ 'DD' BIND DV DD DIV FLOOR DD MUL DV EQ ] FILTER` — Input positive integer; answers its ascending positive divisors as a Vector.
  - `S0D.DIVISORS` = `'K' BIND 1 K RANGE [ 'D' BIND K D S0D.MOD 0 EQ ] FILTER` — ascending positive divisors of a positive integer
  - `U3A.DIVISORS` = `'U3A.DN' BIND 1 U3A.DN RANGE [ 'U3A.D' BIND U3A.DN U3A.D DIV FLOOR U3A.D MUL U3A.DN EQ ] FILTER` — positive integer N -> ascending Vector of its positive divisors
  - `U3B.DIVISORS` = `'DN' BIND 1 DN RANGE [ 'DD' BIND DN DD U3B.MOD 0 EQ ] FILTER` — positive integer n -> Vector of its positive divisors, ascending
  - `U3C.DIVS` = `'N' BIND 1 N RANGE [ 'D' BIND N D DIV FLOOR D MUL N EQ ] FILTER` — Positive integer on the stack -> ascending Vector of its divisors.
  - `U3D.DIVISORS` = `'U3D.DN' BIND 1 U3D.DN RANGE [ 'U3D.DD' BIND U3D.DN U3D.DD U3D.MOD 0 EQ ] FILTER` — positive integer -> Vector of its positive divisors, ascending
- **11 agents** (E0D, E2A, Z2A, Z3B, Z3C, Z3D, S0A, S0D, U2C, U2D, U3B; models fable, opus, sonnet, haiku), joined at d1:
  - `E0D.DEPTH` = `CHARS [ 'C' BIND 1 -1 C E0D.OPEN? SELECT ] MAP 0 [ ADD ] SCAN 0 [ MAX ] FOLD` — bracket String -> greatest nesting depth (max running open-minus-close count, at least 0)
  - `E2A.BRDEPTH` = `CHARS [ 'EC' BIND 1 -1 EC '(' EQ EC '[' EQ E2A.OR SELECT ] MAP 0 [ ADD 0 MAX ] SCAN E2A.MAXOF` — bracket String -> greatest number of brackets open at once
  - `E2A.BALPREFIX` = `CHARS 'EC' BIND 0 EC LENGTH RANGE [ 'EL' BIND EC EL TAKE JOIN E2A.BALANCED ] FILTER E2A.MAXOF` — bracket String -> length of its longest balanced prefix
  - `Z2A.PBLEN` = `'Z2A.PS' BIND Z2A.PS CHARS LENGTH 0 Z2A.PS Z2A.REDUCE '' EQ SELECT` — Bracket String -> its length if balanced, else 0
  - `Z3B.DEPTH` = `'[' '(' REPLACE ']' ')' REPLACE CHARS [ 'ZX' BIND 1 -1 ZX '(' EQ SELECT ] MAP 0 [ ADD 0 MAX ] SCAN 0 [ MAX ] FOLD` — greatest bracket nesting depth of a bracket String (running count clamped at 0)
  - `Z3B.BPREF` = `CHARS '' [ 2 COLLECT JOIN ] SCAN [ 'ZP' BIND ZP CHARS LENGTH 0 ZP X3.ZOSEBI '' EQ SELECT ] MAP 0 [ MAX ] FOLD` — length of the longest balanced prefix of a bracket String
  - `Z3C.SCORE` = `'PP' BIND PP CHARS LENGTH 0 PP Z3C.RED CHARS LENGTH 0 EQ SELECT` — Length of a bracket prefix if it is balanced, else 0.
  - `Z3D.BALPREFIX` = `CHARS 'Z3D.PC' BIND 0 Z3D.PC LENGTH RANGE [ 'Z3D.PK' BIND Z3D.PC Z3D.PK TAKE JOIN X3.ZOSEBI '' EQ ] FILTER -1 GET` — bracket String -> length of the longest balanced prefix
  - `S0A.BDEPTH` = `CHARS [ S0A.BDELTA ] MAP 0 [ ADD 0 MAX ] SCAN 0 [ MAX ] FOLD` — bracket String -> greatest number of brackets open at once
  - `S0A.BPREFIX` = `CHARS 'PC' BIND 0 PC LENGTH RANGE [ 'PL' BIND PC PL TAKE JOIN S0A.BALANCED ] FILTER 0 [ MAX ] FOLD` — bracket String -> length of the longest balanced prefix
  - `S0D.BALPREFIX` = `CHARS 'C' BIND 0 C LENGTH RANGE [ 'K' BIND C K TAKE JOIN S0D.BREDUCE CHARS LENGTH 0 EQ ] FILTER -1 GET` — length of the longest balanced prefix of a bracket String
  - `U2C.UNMATCHED` = `U2C.CODES U2C.REDUCE [ 0 EQ NOT ] FILTER LENGTH` — Bracket String -> count of characters left after repeatedly deleting adjacent matched pairs.
  - `U2D.BALPREFIX` = `CHARS 'PC' BIND 0 PC LENGTH RANGE [ 'PK' BIND PC PK TAKE JOIN U2D.BREDUCE '' EQ ] FILTER -1 GET` — bracket String -> length of its longest balanced prefix (0 when only the empty prefix is)
  - `U3B.BDEPTH` = `U3B.BDELTA 0 [ ADD 0 MAX ] SCAN 0 [ MAX ] FOLD` — bracket String -> greatest number of brackets open at once (count never below 0; 0 for empty)
  - `U3B.BPREFIX` = `'BS' BIND 0 BS CHARS LENGTH RANGE [ 'BL' BIND BL 0 BS CHARS BL TAKE JOIN U3B.REDUCE '' EQ SELECT ] MAP 0 [ MAX ] FOLD` — bracket String -> length of its longest balanced prefix (0 if only the empty one)
- **10 agents** (O1D, O1A, R2B, R2D, R3A, E0D, Z3D, S0D, U2D, U3D; models fable, opus, sonnet), joined at d1/d0a/d0:
  - `O1D.FIB-INDEX` = `'N' BIND [ 1 1 2 3 5 8 13 21 34 55 89 ] [ N LT ] FILTER LENGTH 1 ADD` — integer 1..89 -> smallest k with F(k) >= n, where F(1)=F(2)=1
  - `O1A.FIBIDX` = `'O1A.FN' BIND 1 30 RANGE [ 0 1 ] [ 'O1A.FE' BIND 'O1A.FP' BIND O1A.FP 1 GET O1A.FP 0 GET O1A.FP 1 GET ADD 2 COLLECT ] SCAN [ 0 GET ] MAP [ O1A.FN LT ] FILTER LENGTH 1 ADD` — positive integer n (<= F(30)) -> smallest k with Fibonacci F(k) >= n
  - `R2B.FIBIDX` = `'R2B.FN' BIND [ 1 1 2 3 5 8 13 21 34 55 89 ] [ R2B.FN LT ] FILTER LENGTH 1 ADD` — Integer 1 to 89 -> smallest k with Fibonacci F(k) at least the number.
  - `R2D.FIB-INDEX` = `'R2D.FN' BIND 1 1 COLLECT 1 20 RANGE 1 1 2 COLLECT [ 'R2D.FI' BIND [ 'R2D.FA' 'R2D.FB' ] BIND R2D.FB R2D.FA R2D.FB ADD 2 COLLECT ] SCAN [ 0 GET ] MAP CONCAT [ R2D.FN LT ] FILTER LENGTH 1 ADD` — positive integer n -> smallest k with F(k) >= n, F(1)=F(2)=1 (valid for n up to F(21)=10946)
  - `R3A.FIBINDEX` = `'R3A.FN' BIND 1 40 RANGE [ 0 1 ] [ 'R3A.FI' BIND 'R3A.FA' BIND R3A.FA 1 GET R3A.FA 0 GET R3A.FA 1 GET ADD 2 COLLECT ] SCAN [ 0 GET ] MAP [ R3A.FN LT ] FILTER LENGTH 1 ADD` — positive integer n -> smallest k with Fibonacci F(k) >= n (F(1)=F(2)=1)
  - `E0D.FIB-INDEX` = `'N' BIND 1 12 RANGE [ 0 1 ] [ 'X' BIND [ 'A' 'B' ] BIND B A B ADD 2 COLLECT ] SCAN [ 0 GET ] MAP [ N LT ] FILTER LENGTH 1 ADD` — positive integer n <= 144 -> smallest k with F(k) >= n (F(1)=F(2)=1)
  - `Z3D.FIBIDX` = `'Z3D.FN' BIND [ 1 1 2 3 5 8 13 21 34 55 89 ] [ Z3D.FN LT ] FILTER LENGTH 1 ADD` — integer n (<= 89) -> smallest k with F(k) >= n, F(1)=F(2)=1
  - `S0D.FIBINDEX` = `'N' BIND [ 1 1 2 3 5 8 13 21 34 55 89 ] [ N LT ] FILTER LENGTH 1 ADD` — smallest k with F(k) >= n, for n <= 89
  - `U2D.FIBIDX` = `'FN' BIND 1 20 RANGE 0 1 2 COLLECT [ 'FI' BIND [ 'FA' 'FB' ] BIND FB FA FB ADD 2 COLLECT ] SCAN [ 0 GET ] MAP [ FN LT ] FILTER LENGTH 1 ADD` — positive integer N (up to F(20)=6765) -> smallest k with F(k) >= N, where F(1)=F(2)=1
  - `U3D.FIBINDEX` = `'U3D.FN' BIND [ 1 1 2 3 5 8 13 21 34 55 89 144 233 377 610 987 ] [ U3D.FN LT ] FILTER LENGTH 1 ADD` — positive integer n (<= 987) -> smallest k with Fibonacci F(k) >= n, F(1) = F(2) = 1
- **10 agents** (E0A, E0D, E1A, E1D, E2D, Z1D, Z3A, S0B, U1D, U3D; models opus, fable, sonnet), joined at d0:
  - `E0A.SUM` = `0 [ ADD ] FOLD` — Vector of numbers -> their sum (0 for empty)
  - `E0D.SUM` = `0 [ ADD ] FOLD` — vector -> sum of its numbers (0 for empty)
  - `E1A.SUM` = `0 [ ADD ] FOLD` — numeric Vector -> sum of its elements (0 for empty)
  - `E1D.SUM` = `0 [ ADD ] FOLD` — numeric Vector -> its sum (0 for the empty Vector)
  - `E2D.SUM` = `0 [ ADD ] FOLD` — numeric Vector -> its sum (0 for an empty Vector)
  - `Z1D.SUM` = `0 [ ADD ] FOLD` — sum of a Vector of numbers (0 for empty)
  - `Z3A.SUM` = `0 [ ADD ] FOLD` — sum of a Vector of numbers (0 for empty)
  - `S0B.SUM` = `0 [ ADD ] FOLD` — Sum of a Vector of numbers (0 when empty).
  - `U1D.SUM` = `0 [ ADD ] FOLD` — numeric Vector -> its sum (0 for an empty Vector)
  - `U3D.SUM` = `0 [ ADD ] FOLD` — numeric Vector -> its sum (0 for the empty Vector)
- **9 agents** (G0A, G0D, E0C, E1D, Z1D, Z3A, U1D, U2D, U3D; models opus, fable, haiku), joined at d1/d0a/d0:
  - `G0A.BSCAN` = `CHARS '#' [ G0A.BSTEP ] SCAN` — bracket String -> Vector of matching stacks after each char; '#' marks a balanced prefix
  - `G0D.BRACKET-DELTAS` = `CHARS [ 'DC' BIND [ '(' '[' ')' ']' ] [ 1 1 -1 -1 ] RECORD DC GET ] MAP` — bracket String -> Vector of +1 per opener and -1 per closer
  - `O1D.BDELTA` = `CHARS [ 'C' BIND [ '(' '[' ')' ']' ] [ 1 1 -1 -1 ] RECORD C GET ] MAP` — bracket String -> Vector of +1 for each opening bracket and -1 for each closing one
  - `O3C.BDELTA` = `'O3C.BS' BIND [ '(' '[' ')' ']' ] [ 1 1 -1 -1 ] RECORD O3C.BS CHARS GET` — bracket String -> Vector of +1 per opening and -1 per closing bracket
  - `R1A.DELTAS` = `CHARS [ 'R1A.C' BIND [ '(' '[' ')' ']' ] [ 1 1 -1 -1 ] RECORD R1A.C GET ] MAP` — bracket String -> Vector of +1 per opener and -1 per closer
  - `E0C.CODES` = `CHARS [ 'C' BIND [ 1 -1 2 -2 ] '()[]' C SEARCH GET ] MAP` — Bracket String to a Vector of codes: ( = 1, ) = -1, [ = 2, ] = -2.
  - `E0C.STEPS` = `E0C.CODES [ 'C' BIND 1 -1 C 0 GT SELECT ] MAP` — Bracket String to a Vector of +1 (open) and -1 (close) per character.
  - `E1D.BSTACKS` = `CHARS '#' [ G0A.BSTEP ] SCAN` — bracket String -> Vector of the G0A.BSTEP stack String after each prefix (base #, failure X)
  - `Z1D.BDELTA` = `CHARS [ 'Z1D.K' BIND 1 -1 '()[]' Z1D.K SEARCH 2 Z1D.MOD 0 EQ SELECT ] MAP` — bracket String -> Vector of +1 for an opener, -1 for a closer
  - `Z3A.BRVAL` = `CHARS [ 'Z3A.C' BIND [ '(' '[' ')' ']' ] [ 1 1 -1 -1 ] RECORD Z3A.C GET ] MAP` — bracket String to Vector of +1 (opener) / -1 (closer)
  - `U1D.BSTATES` = `CHARS '#' [ G0A.BSTEP ] SCAN` — bracket String -> Vector of G0A.BSTEP stack Strings after each character ('#' means the prefix so far is balanced)
  - `U2D.BDELTA` = `[ '(' '[' ')' ']' ] [ 1 1 -1 -1 ] RECORD 'BDR' BIND CHARS [ 'BC' BIND BDR BC GET ] MAP` — bracket String -> Vector with 1 per opening bracket and -1 per closing bracket, in order
  - `U3D.BSTEPS` = `CHARS [ 'U3D.BC' BIND [ '(' '[' ')' ']' ] [ 1 1 -1 -1 ] RECORD U3D.BC GET ] MAP` — String of ( ) [ ] -> Vector with +1 per opening and -1 per closing bracket, in order
- **9 agents** (O1C, R2D, O1D, R1D, R2B, E0D, S0B, U3A, U3D; models haiku, fable, sonnet, opus), joined at d1/d0a:
  - `O1C.STREAK` = `0 [ 'SE' BIND 'SA' BIND SA 1 ADD 0 SE SELECT ] SCAN` — Boolean Vector -> Vector of the length of the run of TRUE lanes ending at each lane.
  - `R2D.STREAKS` = `0 [ 'R2D.KE' BIND 1 ADD 0 R2D.KE SELECT ] SCAN` — Boolean Vector -> same-length Vector of running counts of consecutive TRUEs, reset to 0 at each FALSE
  - `O1D.COLLATZ` = `'N' BIND N 1 COLLECT N 1 120 RANGE [ 'I' BIND G0B.COLSTEP ] SCAN CONCAT [ 1 GT ] FILTER LENGTH` — positive integer (at most 120 Collatz steps) -> number of Collatz steps to reach 1 (1 takes 0)
  - `R1D.STREAKS` = `0 [ 'R1D.SF' BIND 'R1D.SA' BIND R1D.SA 1 ADD 0 R1D.SF SELECT ] SCAN` — Vector of truths -> same-length Vector: at each lane, how many consecutive TRUE lanes end there (0 at a FALSE lane)
  - `R2B.RUNLEN` = `0 [ 'R2B.LE' BIND 'R2B.LA' BIND R2B.LA 1 ADD 0 R2B.LE SELECT ] SCAN 0 [ MAX ] FOLD` — Vector of truths -> length of the longest run of consecutive TRUE (0 when none).
  - `E0D.LONGEST-TRUE` = `0 [ 'T' BIND 'A' BIND A 1 ADD 0 T SELECT ] SCAN 0 [ MAX ] FOLD` — vector of truths -> length of the longest run of consecutive TRUE (0 if none)
  - `E0D.TRUE-RUNS` = `[ 0 FALSE ] [ 'T' BIND [ 'C' 'P' ] BIND C 1 ADD C T P NOT AND SELECT T 2 COLLECT ] FOLD 0 GET` — vector of truths -> number of maximal runs of consecutive TRUE
  - `S0B.WHERE` = `'WF' BIND 0 WF LENGTH 1 SUB RANGE -1 WF SELECT [ -1 GT ] FILTER` — Vector of booleans (length>=1) -> 0-based indices of the TRUE lanes.
  - `U3A.COUNT` = `[ NOT NOT ] FILTER LENGTH` — Vector of truth values -> how many are TRUE
  - `U3D.STREAKS` = `0 [ 'U3D.SE' BIND 'U3D.SA' BIND U3D.SA 1 ADD 0 U3D.SE SELECT ] SCAN` — Vector of truths -> per lane, the length of the run of TRUE ending there (0 at a FALSE lane)
- **9 agents** (O1D, O1A, R2B, R3A, E2A, Z3B, Z3D, U2D, U3D; models fable, opus, sonnet), joined at d1/d0a:
  - `O1D.DIGITAL-ROOT` = `1 SUB 9 G0D.MOD 1 ADD` — positive integer -> its digital root (repeated digit sum until one digit), via 1 + (n-1) mod 9
  - `O1A.DIGROOT` = `1 SUB 'O1A.DR' BIND O1A.DR O1A.DR 9 DIV FLOOR 9 MUL SUB 1 ADD` — positive integer -> digital root, 1 + (n-1) mod 9
  - `R2B.DROOT` = `'R2B.RN' BIND R2B.RN 1 SUB 'R2B.RA' BIND R2B.RA R2B.RA 9 DIV FLOOR 9 MUL SUB 1 ADD` — Positive integer -> digital root, 1 + (n-1) mod 9.
  - `R3A.DROOT` = `1 SUB 'R3A.RX' BIND R3A.RX R3A.RX 9 DIV FLOOR 9 MUL SUB 1 ADD` — positive integer -> its digital root, 1 + (n-1) mod 9
  - `E2A.DIGROOT` = `1 SUB 'EM' BIND EM EM 9 DIV FLOOR 9 MUL SUB 1 ADD` — positive integer -> its digital root
  - `Z3B.DROOT` = `1 SUB 'ZA' BIND ZA ZA 9 DIV FLOOR 9 MUL SUB 1 ADD` — digital root of a positive integer: 1 + (n-1) mod 9
  - `Z3D.DIGROOT` = `1 SUB 'Z3D.DN' BIND Z3D.DN Z3D.DN 9 DIV FLOOR 9 MUL SUB 1 ADD` — positive integer -> its digital root, 1 + (n-1) mod 9
  - `U2D.DIGROOT` = `1 SUB 'DR' BIND DR DR 9 DIV FLOOR 9 MUL SUB 1 ADD` — positive integer -> its digital root, as 1 + (n-1) mod 9
  - `U3D.DIGROOT` = `1 SUB 9 U3D.MOD 1 ADD` — positive integer -> its digital root (repeated decimal digit sum), as 1 + (n-1) mod 9
- **9 agents** (O1A, R2B, R2D, R3A, E2A, Z3B, Z3D, S0A, U3B; models opus, sonnet, fable), joined at d1/d0a:
  - `O1A.RATIOS` = `'O1A.RV' BIND O1A.RV 1 DROP O1A.IDX [ 'O1A.RI' BIND O1A.RV O1A.RI 1 ADD GET 'O1A.RB' BIND O1A.RV O1A.RI GET 'O1A.RA' BIND NIL 1 O1A.RB O1A.RB NIL? SELECT 1 O1A.RA O1A.RA NIL? SELECT DIV O1A.RA NIL? NOT O1A.RB NIL? NOT AND NOT SELECT ] MAP` — V with NILs -> Vector of V[i+1]/V[i], NIL where either is missing
  - `R2B.RATIOS` = `'R2B.QV' BIND 0 R2B.QV LENGTH 1 SUB RANGE 1 DROP [ 'R2B.QI' BIND R2B.QV R2B.QI GET R2B.QV R2B.QI 1 SUB GET DIV ] MAP` — Vector with NILs -> each reading divided by the one before, NIL where either is missing; one shorter.
  - `R2D.RATIOS` = `'R2D.TV' BIND R2D.TV 1 DROP R2D.TV 0 GET NIL 2 COLLECT [ 'R2D.TA' BIND 0 GET 'R2D.TB' BIND R2D.TA NIL 1 R2D.TA R2D.TA NIL? SELECT 1 R2D.TB R2D.TB NIL? SELECT DIV R2D.TA NIL? NOT R2D.TB NIL? NOT AND NOT SELECT 2 COLLECT ] SCAN [ 1 GET ] MAP` — non-empty Vector of numbers/NILs -> Vector one shorter of each element divided by the one before it, NIL where either is NIL
  - `R3A.RATIOS` = `[ NIL NIL ] [ 'R3A.TE' BIND 'R3A.TA' BIND R3A.TA 0 GET 'R3A.TP' BIND R3A.TE NIL 1 R3A.TE R3A.TE NIL? SELECT 1 R3A.TP R3A.TP NIL? SELECT DIV R3A.TE NIL? NOT R3A.TP NIL? NOT AND NOT SELECT 2 COLLECT ] SCAN [ 1 GET ] MAP 1 DROP` — Vector with NILs -> Vector one shorter of each element over its predecessor, NIL where either is missing
  - `E2A.RATIOS` = `'EV' BIND EV 1 DROP EV -1 DROP DIV` — V -> each element divided by the one before (NIL where either is NIL), one shorter
  - `Z3B.RATIOS` = `'ZV' BIND ZV 1 DROP ZV -1 DROP DIV` — each element divided by the one before it (one shorter); NIL lanes stay NIL
  - `Z3D.RATIOS` = `'Z3D.RV' BIND Z3D.RV 1 DROP Z3D.RV -1 DROP DIV` — Vector -> each element divided by the one before it (NIL where either is NIL)
  - `S0A.RATIOS` = `'RV' BIND 0 RV LENGTH 1 SUB RANGE 1 DROP [ 'RI' BIND RV RI GET 'RC' BIND RV RI 1 SUB GET 'RP' BIND 1 RC RC NIL? SELECT 1 RP RP NIL? SELECT DIV NIL RC NIL? NOT RP NIL? NOT AND SELECT ] MAP` — readings with NILs -> each reading over the previous one, NIL where either is missing
  - `U3B.RATIOS` = `'RX' BIND RX 1 DROP RX -1 DROP DIV` — Vector -> each element divided by the one before it (one shorter); NIL lanes stay NIL
- **8 agents** (O1A, O1D, R2B, R2D, R3A, E2A, Z3B, U2D; models opus, fable, sonnet), joined at d1:
  - `O1A.UPRUNS` = `'O1A.UV' BIND O1A.UV O1A.UV G0D.SHIFT GT 0 [ 'O1A.UB' BIND 'O1A.UA' BIND O1A.UA 1 ADD 1 O1A.UB SELECT ] SCAN 0 [ MAX ] FOLD` — numeric V -> length of the longest strictly increasing adjacent stretch
  - `O1D.UPRUNS` = `'V' BIND V V G0D.SHIFT SUB 0 GT [ 'T' BIND 1 0 T SELECT ] MAP 0 [ 'E' BIND 'A' BIND A 1 ADD E MUL ] SCAN 0 [ MAX ] FOLD 1 ADD` — numeric Vector -> length of the longest stretch of adjacent strictly increasing values (a lone value counts 1)
  - `R2B.UPRUNS` = `'R2B.UV' BIND R2B.UV R2B.UV 0 GET 1 ADD 1 COLLECT R2B.UV -1 DROP CONCAT SUB 0 GT R2B.RUNLEN 1 ADD` — Non-empty Vector of numbers -> length of the longest stretch of strictly increasing adjacent values.
  - `R2D.UPRUNS` = `'R2D.UV' BIND R2D.UV R2D.UV R2D.PREV GT R2D.STREAKS 0 [ MAX ] FOLD 1 ADD` — non-empty Vector of numbers -> length of the longest stretch of adjacent strictly increasing values (a single value counts 1)
  - `R3A.UPRUNS` = `'R3A.UV' BIND R3A.UV R3A.UV 0 GET 0 2 COLLECT [ 'R3A.UE' BIND 'R3A.UC' BIND R3A.UE R3A.UC 1 GET 1 ADD 1 R3A.UE R3A.UC 0 GET GT SELECT 2 COLLECT ] SCAN [ 1 GET ] MAP 0 [ MAX ] FOLD` — non-empty number Vector -> length of the longest strictly increasing stretch of adjacent values
  - `E2A.UPRUNS` = `'EV' BIND EV EV G0D.SHIFT SUB 0 GT 0 [ 'EF' BIND 'EA' BIND EA 1 ADD 1 EF SELECT ] SCAN E2A.MAXOF` — numeric V -> length of its longest stretch of strictly increasing adjacent values
  - `Z3B.UPRUNS` = `'ZV' BIND ZV ZV X3.LUDAFU SUB 0 MAX 1 MIN 0 [ 'ZE' BIND 1 ADD ZE MUL ] SCAN 0 [ MAX ] FOLD 1 ADD` — length of the longest strictly increasing adjacent stretch of a Vector
  - `U2D.UPRUNS` = `'UV' BIND UV UV G0D.SHIFT SUB 1 [ 'UE' BIND 'UA' BIND UA 1 ADD 1 UE 0 GT SELECT ] SCAN 1 [ MAX ] FOLD` — numeric V -> length of its longest stretch of adjacent strictly increasing values (a single value counts 1)
- **8 agents** (O1A, R2B, R2D, R3A, E0D, E2A, S0A, S0D; models opus, sonnet, fable), joined at d1/d0a:
  - `O1A.SCALED` = `'O1A.SV' BIND O1A.SV O1A.PRESENT 'O1A.SP' BIND O1A.SP O1A.SP 0 GET [ MAX ] FOLD 'O1A.SM' BIND O1A.SV [ 'O1A.SE' BIND NIL 1 O1A.SE O1A.SE NIL? SELECT O1A.SM DIV O1A.SE NIL? SELECT ] MAP` — V with NILs -> each present value divided by the largest present value, NILs kept
  - `R2B.SCALED` = `'R2B.XV' BIND R2B.XV R2B.XV R1A.GAPMASK 1 LT R2B.KEEP 'R2B.XP' BIND R2B.XP R2B.XP 0 GET [ MAX ] FOLD 'R2B.XM' BIND R2B.XV [ R2B.XM DIV ] MAP` — Vector with NILs -> each present reading divided by the largest present reading, NILs kept.
  - `R2D.SCALED` = `'R2D.SV' BIND R2D.SV R2D.PRESENT 'R2D.SP' BIND R2D.SP R2D.SP 0 GET [ MAX ] FOLD 'R2D.SM' BIND R2D.SV [ 'R2D.SE' BIND NIL 1 R2D.SE R2D.SE NIL? SELECT R2D.SM DIV R2D.SE NIL? SELECT ] MAP` — Vector of numbers/NILs with at least one number -> each present value divided by the largest present value, NILs kept
  - `R3A.SCALED` = `'R3A.WV' BIND R3A.WV R2D.PRESENT 'R3A.WP' BIND R3A.WP R3A.WP 0 GET [ MAX ] FOLD 'R3A.WM' BIND R3A.WV [ 'R3A.WE' BIND NIL 0 R3A.WE R3A.WE NIL? SELECT R3A.WM DIV R3A.WE NIL? SELECT ] MAP` — Vector with NILs -> each present element divided by the largest present one; NILs stay NIL
  - `E0D.SCALED` = `'V' BIND V E0D.PRESENT 'P' BIND V P P 0 GET [ MAX ] FOLD DIV` — vector with NILs (some present) -> each present value divided by the largest present value; NILs stay NIL
  - `E2A.SCALED` = `'EV' BIND EV E2A.PRESENT 'EP' BIND EV EP EP 0 GET [ MAX ] FOLD DIV` — V with NILs -> each present value divided by the largest present value, NILs kept
  - `S0A.SCALED` = `'CV' BIND CV S0A.PRESENT 'CP' BIND CP CP 0 GET [ MAX ] FOLD 'MX' BIND CV [ 'SE' BIND NIL 0 SE SE NIL? SELECT MX DIV SE NIL? SELECT ] MAP` — readings with NILs -> each present reading over the largest present, NIL kept
  - `S0D.SCALED` = `'V' BIND V S0D.PRESENT 'P' BIND P P 0 GET [ MAX ] FOLD 'M' BIND V [ 'X' BIND NIL 0 X X NIL? SELECT M DIV X NIL? SELECT ] MAP` — each present element divided by the largest present element; NILs stay NIL
- **8 agents** (R1A, R2A, Z1D, Z3C, S0A, S0B, U1A, U2A; models opus, fable, haiku, sonnet), joined at d1/d0a:
  - `R1A.OREV` = `'R1A.O' BIND R1A.O 'price' GET 'R1A.PR' BIND 0 R1A.PR R1A.PR NIL? SELECT R1A.O 'qty' GET MUL` — order Record -> qty times price, 0 when it has no price
  - `R2A.LINE` = `'R2A.OL' BIND R2A.OL R2A.OPRICE R2A.OL 'qty' GET MUL` — order Record -> qty times price (0 when unpriced)
  - `Z1D.LINE` = `'Z1D.O' BIND Z1D.O 'price' GET 0 Z1D.O 'price' HAS? SELECT Z1D.O 'qty' GET MUL` — order Record -> qty times price, or 0 when it has no price
  - `Z3C.LINE` = `'OD' BIND OD 'qty' GET OD 'price' GET 0 OD 'price' GET NIL? NOT SELECT MUL` — qty times price for an order record, 0 when the order has no price.
  - `S0A.LINEREV` = `'LO' BIND LO 'price' GET 'LP' BIND 0 LP LP NIL? SELECT LO 'qty' GET MUL` — order Record -> qty times price, 0 when unpriced
  - `S0B.LINE` = `'LO' BIND LO 'qty' GET LO 'price' GET 0 S0B.NILOR MUL` — Order Record -> qty*price, 0 when no price.
  - `U1A.LINE` = `'R' BIND R 'qty' GET R U1A.PRICE0 MUL` — order Record -> qty times price (0 when unpriced)
  - `U2A.LINEREV` = `'LR' BIND LR 'price' GET 'LP' BIND 0 LP LP NIL? SELECT LR 'qty' GET MUL` — order Record -> qty times price, or 0 when it has no price
- **7 agents** (G0D, E0D, Z2D, Z3D, S0A, S0D, U2C; models fable, opus, haiku), joined at d1/d0/d0a:
  - `G0D.PRIME` = `G0D.DIVISORS LENGTH 2 EQ` — positive integer -> TRUE when prime
  - `E0D.PRIME?` = `E0D.DIVISORS LENGTH 2 EQ` — positive integer -> TRUE when it is prime
  - `Z2D.ISPRIME` = `X2.TIGOLI LENGTH 2 EQ` — positive integer -> TRUE when it has exactly two divisors
  - `Z3D.ISPRIME` = `X3.VEFAZE LENGTH 2 EQ` — positive integer -> TRUE when it is prime (exactly two divisors)
  - `S0A.ISPRIME` = `'PK' BIND PK 1 PK RANGE S0A.MOD [ 0 EQ ] FILTER LENGTH 2 EQ` — positive integer -> TRUE when prime
  - `S0D.ISPRIME` = `S0D.DIVISORS LENGTH 2 EQ` — TRUE when a positive integer is prime
  - `U2C.ISPRIME` = `G0D.DIVISORS LENGTH 2 EQ` — TRUE when the positive integer has exactly two divisors (is prime).
- **7 agents** (O2A, R2D, O2D, R2B, E2A, Z3B, Z3D; models opus, fable, sonnet), joined at d1:
  - `O2A.LONGEST-GAP` = `G0D.GAPS 0 [ LENGTH MAX ] FOLD` — V with NILs -> length of the longest run of consecutive NILs (0 when none)
  - `R2D.LONGEST-GAP` = `[ NIL? ] MAP R2D.STREAKS 0 [ MAX ] FOLD` — Vector -> length of the longest run of consecutive NILs (0 when none)
  - `O2D.LONGEST-GAP` = `G0D.GAPS [ LENGTH ] MAP 0 [ MAX ] FOLD` — Vector with NILs -> length of the longest run of consecutive NILs (0 when none)
  - `O3D.GAP-COUNT` = `G0D.GAPS LENGTH` — V with NILs -> number of runs of consecutive NILs
  - `R2B.GAPS` = `R1A.GAPMASK 'R2B.GM' BIND R2B.GM 0 1 COLLECT R2B.GM -1 DROP CONCAT SUB [ 0 GT ] FILTER LENGTH` — Vector with NILs -> number of runs of consecutive NILs.
  - `R2D.GAP-COUNT` = `R1A.GAPMASK 'R2D.GM' BIND R2D.GM 0 1 COLLECT R2D.GM -1 DROP CONCAT SUB 0 MAX 0 [ ADD ] FOLD` — non-empty Vector -> number of runs of consecutive NILs
  - `E2A.GAPCOUNT` = `E2A.MISSING 'EM' BIND FALSE 1 COLLECT EM -1 DROP CONCAT NOT EM AND [ ] FILTER LENGTH` — V with NILs -> number of runs of consecutive NILs
  - `E2A.LONGGAP` = `E2A.MISSING 0 [ 'EF' BIND 'EA' BIND EA 1 ADD 0 EF SELECT ] SCAN E2A.MAXOF` — V with NILs -> length of the longest run of consecutive NILs (0 if none)
  - `Z3B.GAPS` = `X3.NIFULO 'ZM' BIND ZM 0 1 COLLECT ZM -1 DROP CONCAT SUB 0 MAX 0 [ ADD ] FOLD` — number of runs of consecutive NILs in a Vector
  - `Z3B.LONGGAP` = `X3.NIFULO 0 [ 'ZE' BIND 1 ADD ZE MUL ] SCAN 0 [ MAX ] FOLD` — length of the longest run of consecutive NILs
  - `Z3D.GAPCOUNT` = `X3.NIFULO 'Z3D.GI' BIND Z3D.GI [ 0 ] Z3D.GI -1 DROP CONCAT SUB 0 MAX 0 [ ADD ] FOLD` — Vector with NILs -> number of runs of consecutive NILs
- **6 agents** (G0D, E1D, E2D, S0A, U1D, U3B; models fable, opus, sonnet), joined at d1/d0a:
  - `G0D.GAPS` = `[ 'GE' BIND 1 0 GE NIL? SELECT ] MAP G0D.RUNS [ 0 GET 1 EQ ] FILTER` — Vector with NILs -> Vector of the runs of consecutive NILs (each run a Vector of 1s)
  - `E1D.GAPS` = `E1D.MISSING G0D.RUNS [ 0 GET 1 EQ ] FILTER` — Vector with NILs -> Vector of the maximal runs of NILs, each as a Vector of 1s (its length is the gap length)
  - `E2D.GAPS` = `[ NIL? ] MAP 'T' BIND 1 0 T SELECT G0D.RUNS [ 0 GET 1 EQ ] FILTER` — Vector with NILs -> Vector of its maximal runs of NIL lanes, each run a Vector of 1s (one per missing lane)
  - `S0A.GAPS` = `[ 'GE' BIND 'x' 'o' GE NIL? SELECT ] MAP JOIN 'o' TOKENIZE [ '' EQ NOT ] FILTER` — readings with NILs -> one String of x's per run of missing readings
  - `U1D.GAPS` = `U1D.NILMASK G0D.RUNS [ 0 GET 1 EQ ] FILTER` — Vector with NILs -> Vector of its runs of consecutive NILs, each run a Vector of 1s (LENGTH gives the gap length)
  - `U3B.MISSRUNS` = `[ NIL? ] MAP [ 'MQ' BIND 1 0 MQ SELECT ] MAP G0D.RUNS [ 0 GET 1 EQ ] FILTER` — Vector with NILs -> Vector of the runs (each a Vector of 1s) of consecutive missing elements
- **6 agents** (U1D, E0A, E1D, Z2D, Z3A, S0D; models fable, opus), joined at d0:
  - `U1D.ARGMAX` = `-1 MUL ORDER 0 GET` — numeric Vector -> index of its first largest element (ties go to the earliest)
  - `E0A.ARGMAX` = `-1 MUL ORDER 0 GET` — non-empty Vector of numbers -> index of the first occurrence of its largest value
  - `E1D.ARGMAX` = `-1 MUL ORDER 0 GET` — numeric Vector -> index of its largest element; ties go to the first
  - `Z2D.ARGMAX` = `-1 MUL ORDER 0 GET` — Vector of numbers -> index of its largest value, the first one on ties
  - `Z3A.ARGMAX` = `-1 MUL ORDER 0 GET` — index of the first largest number in a Vector
  - `S0D.ARGMAX` = `-1 MUL ORDER 0 GET` — index of the first largest element of a numeric Vector
- **6 agents** (R1D, E1D, S0D, U2A, U3B, U3C; models fable, opus, sonnet, haiku), joined at d1/d0a:
  - `R1D.DELTAS` = `CHARS [ 'R1D.BC' BIND 1 -1 R1D.BC '(' EQ NOT R1D.BC '[' EQ NOT AND NOT SELECT ] MAP` — bracket String -> Vector with 1 for each opener ( [ and -1 for each closer ) ]
  - `E1D.BDELTA` = `CHARS [ 'E1D.C' BIND 1 -1 '([' E1D.C SEARCH NIL? NOT SELECT ] MAP` — bracket String -> Vector of +1 per opener ( [ and -1 per closer ) ]
  - `S0D.BDELTA` = `'(' '[' REPLACE ')' ']' REPLACE CHARS [ '[' EQ ] MAP 'B' BIND 1 -1 B SELECT` — bracket String -> Vector of +1 per opener and -1 per closer
  - `U2A.DELTAS` = `CHARS [ 'BC' BIND 1 -1 '([' BC SEARCH NIL? NOT SELECT ] MAP` — bracket String -> Vector of +1 per opening and -1 per closing bracket
  - `U3B.BDELTA` = `CHARS [ 'BC' BIND 1 1 -1 BC '[' EQ SELECT BC '(' EQ SELECT ] MAP` — bracket String -> Vector of +1 for each opener and -1 for each closer
  - `U3C.DELTAS` = `'[' '(' REPLACE ']' ')' REPLACE CHARS [ 'C' BIND 1 -1 C '(' EQ SELECT ] MAP` — Bracket String -> Vector with +1 for each opening bracket and -1 for each closing bracket, per character.
- **6 agents** (R2B, R2D, E2A, Z3D, S0A, S0D; models sonnet, fable, opus), joined at d1:
  - `R2B.MAJ` = `'R2B.JV' BIND R2B.JV TALLY 'R2B.JT' BIND R2B.JT R1A.ARGMAX 'R2B.JK' BIND R2B.JK 1 COLLECT 0 1 R2B.JT R2B.JK GET 2 MUL R2B.JV LENGTH GT SELECT GET` — Vector -> the value occurring in more than half the positions, else NIL.
  - `R2D.MAJORITY` = `'R2D.JV' BIND R2D.JV TALLY 'R2D.JT' BIND R2D.JT R1A.ARGMAX 'R2D.JK' BIND R2D.JK NIL R2D.JT R2D.JK GET 2 MUL R2D.JV LENGTH GT SELECT` — non-empty Vector -> the value occurring in more than half of the positions, or NIL when none does
  - `E2A.MAJORITY` = `'EV' BIND EV TALLY 'ET' BIND ET U1D.TOPKEY 'EK' BIND EK NIL ET EK GET 2 MUL EV LENGTH GT SELECT` — V -> the value occupying more than half its positions, or NIL when none does
  - `Z3D.MAJORITY` = `'Z3D.MV' BIND Z3D.MV TALLY 'Z3D.MT' BIND Z3D.MT Z3D.TOPKEY NIL Z3D.MT VALUES 0 [ MAX ] FOLD 2 MUL Z3D.MV LENGTH GT SELECT` — Vector -> the value in more than half the positions, or NIL
  - `S0A.MAJORITY` = `'MV' BIND MV TALLY 'MT' BIND MT VALUES ORDER -1 GET 'MI' BIND MT KEYS MI GET NIL MT VALUES MI GET 2 MUL MV LENGTH GT SELECT` — vector -> value occurring in more than half the positions, or NIL
  - `S0D.MAJORITY` = `'V' BIND V TALLY 'T' BIND T VALUES 'C' BIND C ORDER -1 GET 'I' BIND T KEYS I GET NIL C I GET 2 MUL V LENGTH GT SELECT` — value occurring in more than half the positions, else NIL
- **5 agents** (G0C, R1D, R2C, E1C, Z2C; models haiku, fable), joined at d1/d0/d0a:
  - `G0C.REV` = `[ 'OX' BIND OX 'price' HAS? ] FILTER [ 'OY' BIND OY 'qty' GET OY 'price' GET MUL ] MAP 0 [ ADD ] FOLD` — Total of qty times price over the order Records that have a price (exact rational).
  - `O1D.TAG-COUNTS` = `[ 'tags' HAS? ] FILTER [ 'tags' GET ] MAP FLATTEN TALLY` — Vector of order Records -> Record from each tag to the number of orders carrying it, tags in order of first appearance
  - `O2A.TAG-COUNTS` = `[ 'tags' HAS? ] FILTER [ 'tags' GET UNIQUE ] MAP FLATTEN TALLY` — Vector of order Records -> Record from each tag to the number of orders carrying it, first-appearance order
  - `O2D.UNPRICED` = `[ 'price' HAS? NOT ] FILTER [ 'sku' GET ] MAP UNIQUE` — Vector of order Records -> distinct skus of the orders without a price, in order of first appearance
  - `O2D.TAG-COUNTS` = `[ 'tags' HAS? ] FILTER [ 'tags' GET ] MAP FLATTEN TALLY` — Vector of order Records -> Record from each tag to the number of orders carrying it, tags in order of first appearance
  - `O3A.UNPRICED` = `[ 'price' HAS? NOT ] FILTER [ 'sku' GET ] MAP UNIQUE` — Vector of order Records -> distinct skus of orders without a price, first-appearance order
  - `O3D.REPORT` = `G0C.REV 2 FORMAT` — Vector of order Records -> revenue (qty*price over priced orders) as a String with two decimals
  - `O3D.UNPRICED` = `[ 'price' HAS? NOT ] FILTER [ 'sku' GET ] MAP UNIQUE` — Vector of order Records -> distinct skus of the orders without a price, in first-appearance order
  - `O3D.TAG-COUNTS` = `[ 'tags' HAS? ] FILTER [ 'tags' GET UNIQUE ] MAP [ ] [ CONCAT ] FOLD TALLY` — Vector of order Records -> Record from each tag to the number of orders carrying it, first-appearance order
  - `R1D.REVENUE` = `[ 'price' HAS? ] FILTER [ 'R1D.OR' BIND R1D.OR 'qty' GET R1D.OR 'price' GET MUL ] MAP 0 [ ADD ] FOLD` — Vector of order Records -> total of qty times price over the orders that have a price
  - `R2C.TAGCOUNTS` = `[ 'XR' BIND XR 'tags' HAS? ] FILTER [ 'XR' BIND XR 'tags' GET UNIQUE ] MAP FLATTEN TALLY` — Orders Vector -> Record from tag to the number of orders carrying it, first-appearance order
  - `E1C.REVENUE` = `[ 'price' HAS? ] FILTER 'P' BIND P [ 'qty' GET ] MAP P [ 'price' GET ] MAP MUL 0 [ ADD ] FOLD` — Vector of order Records -> total of qty times price over the orders that have a price.
  - `Z2C.REVENUE` = `[ 'price' HAS? ] FILTER [ 'OV' BIND OV 'qty' GET OV 'price' GET MUL ] MAP 0 [ ADD ] FOLD` — Input the orders Vector; leaves the sum of qty times price over the orders that carry a price.
- **5 agents** (O1D, R2B, O1A, R2D, Z3D; models fable, sonnet, opus), joined at d1/d0a:
  - `O1D.UNMATCHED` = `G0D.BRACKET-STACK LENGTH 1 SUB` — bracket String -> how many characters remain after repeatedly deleting adjacent () and [] pairs
  - `R2B.BDEPTH` = `CHARS [ R2B.BSIGN ] MAP 0 [ ADD 0 MAX ] SCAN 0 [ MAX ] FOLD` — Bracket String -> greatest number of brackets open at once (open count never below 0).
  - `O1A.BDEPTH` = `CHARS [ 'O1A.DC' BIND [ '(' '[' ')' ']' ] [ 1 1 -1 -1 ] RECORD O1A.DC GET ] MAP 0 [ ADD 0 MAX ] SCAN 0 [ MAX ] FOLD` — bracket String -> greatest number of brackets open at once (stray closers close nothing)
  - `O1A.BPREFIX` = `G0A.BSCAN 'O1A.BV' BIND [ '#' ] O1A.BV CONCAT [ '#' EQ ] MAP 'O1A.BB' BIND 0 O1A.BB LENGTH 1 SUB RANGE 0 O1A.BB SELECT 0 [ MAX ] FOLD` — bracket String -> length of its longest balanced prefix
  - `O1D.BDEPTH` = `O1D.BDELTA 0 [ ADD ] SCAN 0 [ MAX ] FOLD` — bracket String -> greatest nesting depth (0 for the empty String)
  - `O1D.BALANCED-PREFIX` = `G0A.BSCAN [ 0 0 ] [ 'E' BIND 'A' BIND A 0 GET 1 ADD 'I' BIND I I A 1 GET E '#' EQ SELECT 2 COLLECT ] FOLD 1 GET` — bracket String -> length of its longest balanced prefix (0 when only the empty prefix is)
  - `O2A.DEPTH` = `CHARS [ 'O2A.DC' BIND [ '(' '[' ')' ']' ] [ 1 1 -1 -1 ] RECORD O2A.DC GET ] MAP 0 [ ADD ] SCAN 0 [ MAX ] FOLD` — bracket String -> greatest running count of opening minus closing brackets (0 for empty)
  - `O2A.BAL-PREFIX` = `G0A.BSCAN [ '#' EQ ] MAP 'O2A.PB' BIND O2A.PB O1A.IDX [ 'O2A.PI' BIND O2A.PI 1 ADD 0 O2A.PB O2A.PI GET SELECT ] MAP 0 [ MAX ] FOLD` — bracket String -> length of its longest balanced prefix (0 when only the empty prefix is)
  - `O2D.DEPTH` = `CHARS [ 'O2D.C' BIND [ '(' '[' ')' ']' ] [ 1 1 -1 -1 ] RECORD O2D.C GET ] MAP 0 [ ADD ] SCAN 0 [ MAX ] FOLD` — bracket String -> greatest nesting depth (max running count of open brackets, 0 for empty)
  - `O2D.BALANCED-PREFIX` = `G0A.BSCAN 'O2D.B' BIND [ '#' ] O2D.B CONCAT 'O2D.S' BIND O2D.S O1A.IDX 0 O2D.S [ '#' EQ ] MAP SELECT 0 [ MAX ] FOLD` — bracket String -> length of the longest balanced prefix (0 when only the empty prefix is), via G0A.BSCAN
  - `O3A.DEPTH` = `'O3A.DS' BIND [ '(' '[' ')' ']' ] [ 1 1 -1 -1 ] RECORD 'O3A.DR' BIND O3A.DS CHARS [ 'O3A.DK' BIND O3A.DR O3A.DK GET ] MAP 0 [ ADD 0 MAX ] SCAN 0 [ MAX ] FOLD` — bracket String -> greatest number of brackets open at once (running open-minus-close count, never below 0); 0 for empty
  - `O3A.BPREFIX` = `G0A.BSCAN 'O3A.PS' BIND '#' 1 COLLECT O3A.PS CONCAT 'O3A.PV' BIND O3A.PV O1A.IDX 0 O3A.PV [ '#' EQ ] MAP SELECT 0 [ MAX ] FOLD` — bracket String -> length of its longest balanced prefix (0 when only the empty prefix is); uses G0A.BSCAN, O1A.IDX
  - `O3B.DEPTH` = `CHARS [ 'O3B.DX' BIND [ '(' '[' ')' ']' ] [ 1 1 -1 -1 ] RECORD O3B.DX GET ] MAP 0 [ ADD 0 MAX ] SCAN 0 [ MAX ] FOLD` — bracket String -> greatest number of brackets open at once (running count, floored at 0)
  - `O3B.PREFIX` = `G0A.BSCAN 'O3B.PS' BIND [ '#' ] O3B.PS CONCAT 'O3B.PT' BIND O3B.PT O1A.IDX 0 O3B.PT [ '#' EQ ] MAP SELECT 0 [ MAX ] FOLD` — bracket String -> length of its longest balanced prefix (0 for none)
  - `O3C.BDEPTH` = `O3C.BDELTA 0 [ ADD 0 MAX ] SCAN 0 [ MAX ] FOLD` — bracket String -> greatest nesting depth (a closer with nothing open is ignored)
  - `O3C.BPREFIX` = `G0A.BSCAN [ '#' EQ ] MAP 'O3C.BT' BIND O3C.BT O1A.IDX 1 ADD 0 O3C.BT SELECT 0 [ MAX ] FOLD` — bracket String -> length of its longest balanced prefix (0 when none)
  - `O3D.DEPTH` = `CHARS [ 'O3D.C' BIND [ '(' '[' ')' ']' ] [ 1 1 -1 -1 ] RECORD O3D.C GET ] MAP 0 [ ADD ] SCAN 0 [ MAX ] FOLD` — bracket String -> greatest nesting depth (max of running open-count, 0 for empty)
  - `O3D.BALANCED-PREFIX` = `G0A.BSCAN 'O3D.B' BIND O3D.B O1A.IDX [ 'O3D.I' BIND O3D.I 1 ADD 0 O3D.B O3D.I GET '#' EQ SELECT ] MAP 0 [ MAX ] FOLD` — bracket String -> length of the longest balanced prefix (0 when only the empty prefix is)
  - `R2B.BPREFIX` = `'R2B.PV' BIND R2B.PV R1D.USTACKS [ CHARS LENGTH ] MAP 'R2B.PL' BIND 0 R2B.PL LENGTH 1 SUB RANGE 'R2B.PK' BIND 0 R2B.PK R2B.PL 1 GT SELECT 0 [ MAX ] FOLD` — Bracket String -> length of its longest balanced prefix.
  - `R2D.DEPTH` = `'R2D.DS' BIND [ '(' '[' ')' ']' ] [ 1 1 -1 -1 ] RECORD 'R2D.DR' BIND R2D.DS CHARS [ 'R2D.DC' BIND R2D.DR R2D.DC GET ] MAP 0 [ ADD ] SCAN 0 [ MAX ] FOLD` — bracket String of ( ) [ ] -> greatest nesting depth (0 for the empty String)
  - `R3A.BPREFIX` = `CHARS '' [ R3A.USTEP ] SCAN [ 0 0 ] [ 'R3A.QS' BIND 'R3A.QA' BIND R3A.QA 0 GET 1 ADD 'R3A.QP' BIND R3A.QP R3A.QP R3A.QA 1 GET R3A.QS '' EQ SELECT 2 COLLECT ] FOLD 1 GET` — bracket String -> length of its longest balanced prefix (0 if only the empty one)
  - `Z3D.BRDEPTH` = `CHARS Z3D.BRDELTA 0 [ ADD ] SCAN 0 [ MAX ] FOLD` — bracket String -> greatest nesting depth (0 for empty)
- **5 agents** (O1D, Z3B, Z3D, U2D, U3B; models fable, sonnet), joined at d1/d0a:
  - `O1D.SCALED` = `'V' BIND V O1D.PRESENT SORT -1 GET 'M' BIND V [ 'X' BIND 0 X X NIL? SELECT M DIV X X NIL? NOT SELECT ] MAP` — Vector with NILs -> each present element divided by the largest present element, NILs kept
  - `Z3B.SCALED` = `'ZV' BIND ZV ZV [ NIL? NOT ] FILTER SORT -1 GET DIV` — each reading divided by the largest present reading; NIL lanes stay NIL
  - `Z3D.SCALED` = `'Z3D.SV' BIND Z3D.SV Z3D.SV Z3D.PRESENT SORT -1 GET DIV` — Vector with NILs -> each present value divided by the largest present value, NILs kept
  - `U2D.SCALED` = `'SCV' BIND SCV U2D.PRESENT SORT -1 GET 'SM' BIND SCV [ 'SE' BIND 1 SE SE NIL? SELECT SM DIV NIL SE NIL? NOT SELECT ] MAP` — V with NIL gaps -> each present element divided by the largest present element; NILs stay NIL
  - `U3B.SCALED` = `'SX' BIND SX SX U3B.PRESENT SORT -1 GET DIV` — Vector with NILs -> each element divided by the largest present element; NIL stays NIL
- **4 agents** (G0C, R2B, R3A, R3C; models haiku, sonnet, opus), joined at d1/d0a:
  - `G0C.DIVS` = `'GDK' BIND 1 GDK RANGE [ 'GDD' BIND GDK GDD DIV 'GDQ' BIND GDQ FLOOR GDQ EQ ] FILTER` — Positive divisors of a positive integer, ascending.
  - `R2B.DIVS` = `'R2B.DN' BIND 1 R2B.DN RANGE [ 'R2B.DD' BIND R2B.DN R2B.DD DIV 'R2B.DQ' BIND R2B.DQ R2B.DQ FLOOR EQ ] FILTER` — Positive integer -> Vector of its positive divisors, ascending.
  - `R3A.DIVISORS` = `'R3A.DN' BIND 1 R3A.DN RANGE [ 'R3A.DD' BIND R3A.DN R3A.DD DIV 'R3A.DQ' BIND R3A.DQ R3A.DQ FLOOR EQ ] FILTER` — positive integer -> Vector of its positive divisors, ascending
  - `R3C.DIVISORS` = `'DN' BIND 1 DN RANGE [ 'D' BIND DN D DIV 'Q' BIND Q FLOOR Q EQ ] FILTER` — Positive divisors of a positive integer, ascending.
- **4 agents** (G0B, Z2A, Z3C, S0B; models sonnet, opus, haiku), joined at d0a/d1:
  - `G0B.FILLSTEP` = `'FE' BIND 'FA' BIND FA FE FE NIL? SELECT` — acc elem in: elem if present, else acc (for SCAN forward fill).
  - `Z2A.FF` = `'Z2A.FE' BIND 'Z2A.FA' BIND Z2A.FA Z2A.FE Z2A.FE NIL? SELECT` — SCAN step for forward fill: acc e -> e, or acc when e is NIL
  - `Z3C.FILLSTEP` = `'EL' BIND 'AC' BIND EL AC EL NIL? NOT SELECT` — SCAN step: a present reading replaces the carried value, a missing one keeps it.
  - `S0B.FILL1` = `'FE' BIND FE FE NIL? SELECT` — acc and element -> acc if element is NIL else element (SCAN block for forward fill).
- **4 agents** (G0C, R2B, R3A, R3C; models haiku, sonnet, opus), joined at d1/d0a:
  - `G0C.PRIMES` = `'GPN' BIND 1 GPN RANGE [ 'GPK' BIND GPK G0C.DIVS LENGTH 2 EQ ] FILTER` — Primes less than or equal to a positive integer, ascending (uses G0C.DIVS).
  - `R2B.PRIMES` = `'R2B.PN' BIND 1 R2B.PN RANGE [ R2B.DIVS LENGTH 2 EQ ] FILTER` — Positive integer -> Vector of primes up to it, ascending.
  - `R3A.PRIMES` = `'R3A.PN' BIND 1 R3A.PN RANGE [ R3A.DIVISORS LENGTH 2 EQ ] FILTER` — positive integer -> Vector of the primes up to it, ascending
  - `R3C.PRIMES` = `'PN' BIND 1 PN RANGE [ R3C.DIVISORS LENGTH 2 EQ ] FILTER` — Primes up to a positive integer, ascending (empty when none).
- **4 agents** (O1A, O1D, E2A, Z3D; models opus, fable), joined at d0/d1:
  - `O1A.DEDUPE` = `G0D.RUNS [ 0 GET ] MAP` — numeric V -> V with each run of equal adjacent values collapsed to one
  - `O1D.DEDUPE` = `G0D.RUNS [ 0 GET ] MAP` — integer Vector -> same Vector with each run of equal adjacent values collapsed to one value
  - `E2A.DEDUPE` = `G0D.RUNS [ 0 GET ] MAP` — numeric V -> V with each run of equal adjacent values collapsed to one value
  - `Z3D.DEDUPE` = `X3.FASENO [ 0 GET ] MAP` — Vector -> runs of equal adjacent values collapsed to one value each
- **4 agents** (O1A, O1D, E1D, Z3B; models opus, fable, sonnet), joined at d1/d0a:
  - `O1A.MAJORITY` = `'O1A.MV' BIND O1A.MV LENGTH 'O1A.MN' BIND O1A.MV TALLY 'O1A.MT' BIND O1A.MT KEYS O1A.MT VALUES 2 COLLECT ZIP [ 1 GET 2 MUL O1A.MN GT ] FILTER [ 0 GET ] MAP 0 GET` — V -> the value occupying more than half the positions, or NIL
  - `O1D.MAJORITY` = `'V' BIND V TALLY 'T' BIND T KEYS T VALUES 2 COLLECT ZIP [ 1 GET 2 MUL V LENGTH GT ] FILTER [ 0 GET ] MAP 'M' BIND M 0 GET NIL M LENGTH 0 GT SELECT` — Vector -> the value occurring in more than half of the positions, or NIL when none does
  - `E1D.MAJORITY` = `'E1D.V' BIND E1D.V TALLY 'E1D.T' BIND E1D.T KEYS E1D.T VALUES 2 COLLECT ZIP [ 1 GET E1D.V LENGTH 2 DIV GT ] FILTER [ 0 GET ] MAP 0 GET` — Vector -> the element present in more than half the positions, or NIL when none is
  - `Z3B.MAJORITY` = `'ZV' BIND ZV LENGTH 'ZN' BIND ZV TALLY 'ZT' BIND ZT KEYS ZT VALUES 2 COLLECT ZIP [ 1 GET 2 MUL ZN GT ] FILTER [ 0 GET ] MAP 0 GET` — value occurring in more than half the positions, else NIL
- **4 agents** (O3D, Z3D, S0D, U2D; models fable), joined at d1:
  - `O3D.TITLE` = `O3D.LWORDS [ G0B.TITLECASE ] MAP G0A.UNWORDS` — messy String -> words title-cased, joined by single spaces
  - `Z3D.TITLE` = `X3.KEGAFE [ CHARS 'Z3D.TC' BIND Z3D.TC 0 GET UPPER Z3D.TC 1 DROP JOIN LOWER 2 COLLECT JOIN ] MAP X3.FOZABA` — space-separated String -> words in Title Case joined by single spaces
  - `S0D.TITLE` = `S0D.WORDS [ 'W' BIND W CHARS 1 TAKE JOIN UPPER W CHARS 1 DROP JOIN LOWER 2 COLLECT JOIN ] MAP S0D.SPACEJOIN` — words in Title Case joined by single spaces
  - `U2D.TITLE` = `LOWER G0D.WORDS [ CHARS 'TC' BIND TC 0 GET UPPER 1 COLLECT TC 1 DROP CONCAT JOIN ] MAP G0A.UNWORDS` — messy String of words -> the words title-cased (first letter upper, rest lower), joined by single spaces
- **4 agents** (R1B, R1C, E1B, S0B; models sonnet, haiku), joined at d1/d0a:
  - `R1B.CNEXT` = `'CX' BIND CX 2 DIV CX 3 MUL 1 ADD CX CX 2 DIV FLOOR 2 MUL SUB 0 EQ SELECT 1 CX 1 EQ NOT SELECT` — one Collatz step; 1 stays 1
  - `R1C.CSTEP` = `'CX' BIND 1 CX 2 DIV CX 3 MUL 1 ADD CX 2 R1C.MOD 0 EQ SELECT CX 1 EQ SELECT` — One Collatz step on a positive integer; 1 stays 1.
  - `E1B.CSTEP` = `'CX' BIND CX 2 DIV CX 3 MUL 1 ADD CX CX 2 DIV FLOOR 2 MUL SUB 0 EQ SELECT 'CY' BIND 1 CY CX 1 EQ SELECT` — one Collatz step on a positive integer, staying at 1 once reached
  - `S0B.CSTEP` = `'CX' BIND CX 2 DIV CX 3 MUL 1 ADD CX CX 2 DIV FLOOR 2 MUL SUB 0 EQ SELECT 'CR' BIND 1 CR CX 1 EQ SELECT` — One Collatz step; 1 stays 1.
- **4 agents** (E1D, E2A, Z3D, U2D; models fable, opus), joined at d0a/d1:
  - `E1D.PRIMES` = `'E1D.N' BIND 1 E1D.N RANGE [ G0D.DIVISORS LENGTH 2 EQ ] FILTER` — positive integer N -> ascending Vector of the primes <= N (empty for N=1)
  - `E2A.PRIMES` = `'EN' BIND 1 EN RANGE [ G0D.DIVISORS LENGTH 2 EQ ] FILTER` — positive integer N -> ascending Vector of the primes <= N
  - `Z3D.PRIMES` = `'Z3D.PN' BIND 1 Z3D.PN RANGE [ Z3D.ISPRIME ] FILTER` — positive integer n -> ascending Vector of primes <= n
  - `U2D.PRIMES` = `'PN' BIND 1 PN RANGE [ G0D.DIVISORS LENGTH 2 EQ ] FILTER` — positive integer N -> ascending Vector of the primes <= N (empty when none)
- **4 agents** (E2B, Z3C, S0A, S0C; models sonnet, haiku, opus), joined at d0a/d0:
  - `E2B.DELTA` = `'DC' BIND 1 -1 DC '(' EQ NOT DC '[' EQ NOT AND NOT SELECT` — char -> 1 for an opening bracket, -1 for a closing one
  - `Z3C.DELTA` = `'CH' BIND 1 -1 CH '(' EQ NOT CH '[' EQ NOT AND NOT SELECT` — One-character bracket to +1 (open) or -1 (close).
  - `S0A.BDELTA` = `'BC' BIND 1 -1 BC '(' EQ NOT BC '[' EQ NOT AND NOT SELECT` — bracket character -> 1 for an opener, -1 for a closer
  - `S0C.DELTA` = `'DC' BIND 1 -1 DC '(' EQ NOT DC '[' EQ NOT AND NOT SELECT` — Input one bracket character; answers 1 for an opening bracket and -1 for a closing one.
- **3 agents** (G0B, R2A, Z2A; models sonnet, opus), joined at d1:
  - `G0B.DIVISORS` = `'DVN' BIND 1 DVN RANGE 'DVS' BIND DVS 0 DVN DVS DIV 'DVQ' BIND DVQ FLOOR DVQ SUB 0 LT NOT SELECT [ 0 GT ] FILTER` — Positive integer in, ascending Vector of its positive divisors out.
  - `R2A.DIVISORS` = `'R2A.DN' BIND 1 R2A.DN RANGE 'R2A.DR' BIND R2A.DN R2A.DR DIV 'R2A.DQ' BIND R2A.DR 0 R2A.DQ FLOOR R2A.DQ LT NOT SELECT [ 0 GT ] FILTER` — positive integer -> Vector of its positive divisors, ascending
  - `Z2A.DIVS` = `'Z2A.DN' BIND 1 Z2A.DN RANGE 'Z2A.DR' BIND Z2A.DN Z2A.DR DIV 'Z2A.DQ' BIND Z2A.DR 0 Z2A.DQ Z2A.DQ FLOOR SUB 0 GT NOT SELECT [ 0 GT ] FILTER` — Positive integer -> Vector of its positive divisors, ascending
- **3 agents** (O1A, O1D, E2A; models opus, fable), joined at d1:
  - `O1A.SECOND-MAX` = `UNIQUE SORT -2 GET` — numeric V -> second-largest distinct value, NIL when all equal
  - `O1D.SECOND-LARGEST` = `UNIQUE SORT 'U' BIND U -2 GET NIL U LENGTH 1 GT SELECT` — numeric Vector -> its second-largest distinct value, or NIL when all values are equal
  - `E2A.SECOND` = `UNIQUE SORT -2 GET 'ES' BIND NIL ES ES NIL? SELECT` — V -> its second-largest distinct value, or NIL when all values are equal
- **3 agents** (O1A, R3A, S0A; models opus), joined at d1:
  - `O1A.COLLATZ` = `'O1A.CN' BIND O1A.CN 1 COLLECT 1 150 RANGE O1A.CN [ 'O1A.CE' BIND G0B.COLSTEP ] SCAN CONCAT [ 1 GT ] FILTER LENGTH` — positive integer (<= ~100) -> number of Collatz steps to reach 1
  - `R3A.COLLATZ` = `'R3A.CN' BIND R3A.CN 1 COLLECT 1 200 RANGE R3A.CN [ 'R3A.CI' BIND 'R3A.CX' BIND R3A.CX 2 DIV 'R3A.CH' BIND 1 R3A.CH R3A.CX 3 MUL 1 ADD R3A.CH R3A.CH FLOOR EQ SELECT R3A.CX 1 EQ SELECT ] SCAN CONCAT [ 1 GT ] FILTER LENGTH` — positive integer (path under 200 steps) -> number of Collatz steps to reach 1
  - `S0A.COLLATZ` = `'CN' BIND CN 1 COLLECT 1 200 RANGE CN [ 'CE' BIND 'CA' BIND 1 CA 2 DIV CA 3 MUL 1 ADD CA 2 S0A.MOD 0 EQ SELECT CA 1 EQ SELECT ] SCAN CONCAT [ 1 GT ] FILTER LENGTH` — positive integer -> Collatz steps to reach 1 (up to 200 steps)
- **3 agents** (O1A, E0D, S0D; models opus, fable), joined at d1:
  - `O1A.IDX` = `-1 [ 'O1A.IE' BIND 1 ADD ] SCAN` — V -> Vector of its indices 0..n-1 (empty for empty)
  - `E0D.IOTA` = `[ 'X' BIND 1 ] MAP 0 [ ADD ] SCAN 1 SUB` — vector -> [ 0 .. n-1 ], its indices (empty for empty)
  - `S0D.STARTS` = `'V' BIND V LENGTH 'N' BIND [ 1 ] V 1 DROP V -1 DROP SUB CONCAT [ 0 EQ NOT ] MAP 'F' BIND 0 N 1 SUB RANGE -1 F SELECT [ -1 GT ] FILTER` — indices where a maximal run of equal adjacent values begins
- **3 agents** (O1D, S0D, U2D; models fable), joined at d1:
  - `O1D.RATIOS` = `'V' BIND V 1 DROP V -1 DROP 2 COLLECT ZIP [ [ 'A' 'B' ] BIND 0 A A NIL? SELECT 1 B B NIL? SELECT DIV NIL A NIL? NOT B NIL? NOT AND SELECT ] MAP` — Vector with NILs -> one-shorter Vector of each element divided by its predecessor, NIL where either is missing
  - `S0D.RATIOS` = `'V' BIND V -1 DROP V 1 DROP 2 COLLECT ZIP [ [ 'A' 'B' ] BIND NIL 1 B B NIL? SELECT 1 A A NIL? SELECT DIV A NIL? NOT B NIL? NOT AND NOT SELECT ] MAP` — each element divided by its predecessor; NIL where either is NIL
  - `U2D.RATIOS` = `'RTV' BIND RTV -1 DROP RTV 1 DROP 2 COLLECT ZIP [ [ 'RP' 'RC' ] BIND 1 RC RC NIL? SELECT 1 RP RP NIL? SELECT DIV NIL RP NIL? NOT RC NIL? NOT AND SELECT ] MAP` — V -> Vector one shorter of each element divided by its predecessor; NIL where either is NIL
- **3 agents** (O3A, R2A, Z3C; models opus, haiku), joined at d1:
  - `O3A.TAGS-OF` = `'tags' GET 'O3A.TG' BIND O3A.TG [ ] 2 COLLECT 0 1 O3A.TG NIL? NOT SELECT GET UNIQUE` — order Record -> Vector of its distinct tags, empty when it has no tags key
  - `R2A.OTAGS` = `'tags' GET 'R2A.OT' BIND [ ] R2A.OT 2 COLLECT 0 1 R2A.OT NIL? SELECT GET UNIQUE` — order Record -> its distinct tags, or an empty Vector when it has none
  - `Z3C.TAGS` = `'OD' BIND OD 'tags' GET 'TG' BIND TG [ ] TG NIL? NOT SELECT UNIQUE` — Distinct tags of one order record; an empty Vector when it has none.
- **3 agents** (R2A, Z2A, S0B; models opus, sonnet), joined at d0a:
  - `R2A.GSTEP` = `'R2A.GM' BIND 1 ADD R2A.GM MUL` — gap-length step: current run length and a 0/1 gap flag -> run length + 1 on a gap, 0 otherwise
  - `Z2A.GL` = `'Z2A.GM' BIND 1 ADD Z2A.GM MUL` — SCAN step for run length: acc m -> (acc+1)*m, m a 0/1 flag
  - `S0B.STREAK` = `'ST' BIND 1 ADD ST MUL` — acc and 0/1 flag -> (acc+1)*flag (running streak).
- **3 agents** (R2A, E0A, U1A; models opus), joined at d1:
  - `R2A.OPRICE` = `'R2A.OO' BIND R2A.OO 'price' GET 'R2A.OP' BIND 0 R2A.OP 2 COLLECT 0 1 R2A.OP NIL? SELECT GET` — order Record -> its price, or 0 when it has none
  - `E0A.LINE` = `'R' BIND R 'qty' GET R 'price' GET MUL 'X' BIND 0 X X NIL? SELECT` — order Record -> qty times price, or 0 when it has no price
  - `U1A.PRICE0` = `'price' GET 'Q' BIND 0 Q Q NIL? SELECT` — order Record -> its price, or 0 when it has none
- **3 agents** (R2D, R3D, E0D; models fable), joined at d0:
  - `R2D.KEEP` = `2 COLLECT ZIP [ 1 GET ] FILTER [ 0 GET ] MAP` — Vector, same-length Boolean mask -> the elements where the mask is TRUE, in order (both non-empty)
  - `R3D.MASK` = `2 COLLECT ZIP [ 1 GET ] FILTER [ 0 GET ] MAP` — Vector, same-length Boolean Vector -> the elements at the TRUE positions, in order
  - `E0D.MASK` = `2 COLLECT ZIP [ 1 GET ] FILTER [ 0 GET ] MAP` — values truths -> the values whose aligned truth is TRUE, in order
- **3 agents** (E0A, E1A, U1D; models opus, fable), joined at d0a:
  - `E0A.VMAX` = `'V' BIND V V 0 GET [ MAX ] FOLD` — non-empty Vector -> its largest element
  - `E1A.VMAX` = `'E1A.VM' BIND E1A.VM E1A.VM 0 GET [ MAX ] FOLD` — non-empty Vector -> its largest element
  - `U1D.MAXV` = `'U1D.MV' BIND U1D.MV U1D.MV 0 GET [ MAX ] FOLD` — non-empty numeric Vector -> its largest element (works for negatives, seeded with the first element)
- **3 agents** (E0D, E2A, E2D; models fable, opus), joined at d1:
  - `E0D.UNMATCHED` = `CHARS [ '#' ] [ E0D.USTEP ] FOLD LENGTH 1 SUB` — bracket String -> characters left after repeatedly deleting adjacent () and [] pairs
  - `E2A.UNMATCHED` = `E2A.BRREDUCE CHARS LENGTH` — bracket String -> count of characters left after deleting matched pairs
  - `E2D.UNMATCHED` = `CHARS [ '' ] [ E2D.BRSTEP ] FOLD LENGTH 1 SUB` — String of ( ) [ ] -> how many characters remain after deleting adjacent matched pairs until none is left
- **3 agents** (Z3D, S0A, S0D; models fable, opus), joined at d1:
  - `Z3D.UPRUNS` = `'Z3D.UV' BIND Z3D.UV 1 DROP Z3D.UV -1 DROP SUB 1 MIN 0 MAX Z3D.LONGEST1 1 ADD` — Vector of integers -> length of the longest strictly increasing stretch of adjacent values
  - `S0A.UPRUNS` = `'UV' BIND 0 UV LENGTH 1 SUB RANGE 'UI' BIND UV UV UI 1 SUB GET GT 0 FALSE PUT 0 [ 'UF' BIND 'UA' BIND UA 1 ADD 1 UF SELECT ] SCAN 0 [ MAX ] FOLD` — numeric vector -> length of longest strictly increasing adjacent stretch
  - `S0D.UPRUNS` = `'V' BIND 1 0 V 1 DROP V -1 DROP SUB 0 GT SELECT 1 [ MUL 1 ADD ] SCAN 1 [ MAX ] FOLD` — length of the longest strictly increasing stretch of adjacent values
- **3 agents** (S0D, U3B, U3D; models fable, sonnet), joined at d1:
  - `S0D.PRIMES` = `'N' BIND 1 N RANGE [ S0D.ISPRIME ] FILTER` — primes <= n, ascending
  - `U3B.PRIMES` = `'PN' BIND 1 PN RANGE [ U3B.DIVISORS LENGTH 2 EQ ] FILTER` — integer n >= 1 -> Vector of primes <= n, ascending (empty when none)
  - `U3D.PRIMES` = `'U3D.PN' BIND 1 U3D.PN RANGE 1 DROP [ U3D.DIVISORS LENGTH 2 EQ ] FILTER` — positive integer n -> Vector of the primes <= n, ascending (empty for n = 1)
- **3 agents** (S0D, U2D, U3D; models fable), joined at d1:
  - `S0D.COLLATZ` = `'N' BIND 1 150 RANGE N 0 2 COLLECT [ 'I' BIND [ 'C' 'S' ] BIND C C 2 DIV C 3 MUL 1 ADD C 2 S0D.MOD 0 EQ SELECT C 1 EQ SELECT S S 1 ADD C 1 EQ SELECT 2 COLLECT ] FOLD 1 GET` — Collatz steps to reach 1 (up to 150 steps)
  - `U2D.COLLATZ` = `'CN' BIND 1 150 RANGE CN 0 2 COLLECT [ 'CI' BIND [ 'CC' 'CS' ] BIND CC CC 2 DIV CC 3 MUL 1 ADD CC 2 DIV FLOOR 2 MUL CC EQ SELECT CC 1 EQ SELECT CS CS 1 ADD CC 1 EQ SELECT 2 COLLECT ] FOLD 1 GET` — positive integer -> number of Collatz steps to reach 1 (bounded at 150 iterations, enough for every start below 100)
  - `U3D.COLLATZ` = `'U3D.CN' BIND 1 150 RANGE U3D.CN 0 2 COLLECT [ 'U3D.CI' BIND U3D.CSTEP ] FOLD 1 GET` — positive integer (Collatz path of at most 150 steps, true for n <= 60) -> number of Collatz steps to reach 1
- **2 agents** (G0D, R2D; models fable), joined at d0/d0a:
  - `G0D.SHIFT` = `'SV' BIND SV 0 GET 1 COLLECT SV -1 DROP CONCAT` — V -> same-length Vector of each lane's previous element, first lane repeating V[0]
  - `X1.ZARORO` = `'SV' BIND SV 0 GET 1 COLLECT SV -1 DROP CONCAT`
  - `X2.LUSORE` = `'SV' BIND SV 0 GET 1 COLLECT SV -1 DROP CONCAT`
  - `X3.LUDAFU` = `'Z2A.SV' BIND Z2A.SV 0 GET 1 COLLECT Z2A.SV -1 DROP CONCAT`
  - `R2D.PREV` = `'R2D.PV' BIND R2D.PV 0 GET 1 COLLECT R2D.PV -1 DROP CONCAT` — non-empty Vector -> same-length Vector shifted right by one, the first element repeated at the front (lane i holds element i-1)
  - `Z2A.SHIFT` = `'Z2A.SV' BIND Z2A.SV 0 GET 1 COLLECT Z2A.SV -1 DROP CONCAT` — Vector shifted right by one, first element repeated: [a b c] -> [a a b]
- **2 agents** (G0A, R3A; models opus), joined at d1/d0:
  - `G0A.BSTEP` = `'G0A.SK' BIND 'G0A.SA' BIND [ '(' '[' ')' ']' ] [ 'O' 'O' '(' '[' ] RECORD G0A.SK GET 'G0A.SQ' BIND G0A.SA G0A.SK 2 COLLECT JOIN G0A.SA CHARS -1 DROP JOIN 'X' G0A.SA CHARS -1 GET G0A.SQ EQ SELECT G0A.SQ 'O' EQ SELECT` — bracket-matching step: stack String (base '#', failure 'X') and a bracket char -> next stack String
  - `R1D.USTEP` = `'R1D.UK' BIND 'R1D.US' BIND R1D.US CHARS -1 DROP JOIN R1D.US R1D.UK 2 COLLECT JOIN R1D.US CHARS -1 GET [ ')' ']' '(' '[' ] [ '(' '[' 'O' 'O' ] RECORD R1D.UK GET EQ SELECT` — pair-cancelling step: reduction stack String (base '#') and a bracket char -> next stack; a closer pops a matching opener on top, anything else is pushed
  - `X1.SOKOLO` = `'G0A.SK' BIND 'G0A.SA' BIND [ '(' '[' ')' ']' ] [ 'O' 'O' '(' '[' ] RECORD G0A.SK GET 'G0A.SQ' BIND G0A.SA G0A.SK 2 COLLECT JOIN G0A.SA CHARS -1 DROP JOIN 'X' G0A.SA CHARS -1 GET G0A.SQ EQ SELECT G0A.SQ 'O' EQ SELECT`
  - `G0A.USTEP` = `'G0A.UK' BIND 'G0A.UA' BIND [ '(' '[' ')' ']' ] [ 'O' 'O' '(' '[' ] RECORD G0A.UK GET 'G0A.UQ' BIND G0A.UA CHARS -1 DROP JOIN G0A.UA G0A.UK 2 COLLECT JOIN G0A.UA CHARS -1 GET G0A.UQ EQ SELECT` — pair-deletion step: residue String (base '#') and a bracket char -> residue with a matched pair cancelled or the char pushed
  - `R1C.USTEP` = `'UC' BIND 'UA' BIND UA CHARS -1 DROP JOIN UA UC 2 COLLECT JOIN UA CHARS -1 GET [ ')' ']' '(' '[' ] [ '(' '[' '?' '?' ] RECORD UC GET EQ SELECT` — Stack step for bracket reduction: acc String (sentinel '#') and a char; a closer matching the top pops it, anything else is pushed.
  - `R3A.USTEP` = `'R3A.KC' BIND 'R3A.KS' BIND [ '' ] R3A.KS CHARS CONCAT 'R3A.KK' BIND R3A.KK -1 DROP JOIN R3A.KS R3A.KC 2 COLLECT JOIN R3A.KK -1 GET [ ')' ']' '(' '[' ] [ '(' '[' 'none' 'none' ] RECORD R3A.KC GET EQ SELECT` — FOLD/SCAN step: pending-bracket String and next bracket char -> pending String after cancelling a matched pair (else the char is appended)
  - `E1C.FSTEP` = `'E1C.C' BIND 'E1C.S' BIND E1C.S CHARS -1 DROP JOIN E1C.S E1C.C 2 COLLECT JOIN [ '(' '[' ')' ']' ] [ 'O' 'O' '(' '[' ] RECORD E1C.C GET E1C.S CHARS -1 GET EQ SELECT` — Bracket stack step: String state (base '#') and a bracket char -> state with the char pushed, or with the matching opener popped when the char closes it.
- **2 agents** (G0A, E0C; models opus, haiku), joined at d1:
  - `G0A.RUNS` = `'G0A.RV' BIND G0A.RV G0A.RV 1 TAKE G0A.RV -1 DROP CONCAT SUB 'G0A.RD' BIND G0A.RD G0A.RD MUL 1 MIN 0 [ ADD ] SCAN G0A.RV GROUP VALUES` — integer Vector -> Vector of its maximal runs of equal adjacent values, each a Vector
  - `E0C.RUNS` = `'V' BIND V 1 DROP V -1 DROP SUB 'D' BIND [ TRUE ] D D MUL 0 GT CONCAT 'S' BIND 1 0 S SELECT 0 [ ADD ] SCAN 'ID' BIND ID V GROUP VALUES` — Run-length groups of a Vector: one Vector per maximal run of equal adjacent values, in order.
- **2 agents** (G0A, U3B; models opus, sonnet), joined at d1:
  - `G0A.BINARY` = `'G0A.BN' BIND [ 32 16 8 4 2 1 ] 'G0A.BP' BIND G0A.BN G0A.BP DIV FLOOR 'G0A.BQ' BIND G0A.BQ G0A.BQ 2 DIV FLOOR 2 MUL SUB 0 G0A.BP [ G0A.BN GT NOT ] FILTER LENGTH SUB TAKE STR JOIN` — integer 1..63 -> base-2 String without leading zeros
  - `U3B.BINARY` = `'BN' BIND [ 1 2 4 8 16 32 ] 'BP' BIND BN BP DIV FLOOR 'BF' BIND BF BF 2 DIV FLOOR 2 MUL SUB BP [ BN GT NOT ] FILTER LENGTH TAKE REVERSE STR JOIN` — integer 1..63 -> its base-2 String without leading zeros
- **2 agents** (R1D, R1B; models fable, sonnet), joined at d1:
  - `R1D.USTACKS` = `'R1D.UV' BIND '#' 1 COLLECT R1D.UV CHARS '#' [ R1D.USTEP ] SCAN CONCAT` — bracket String -> Vector of reduction stacks after each prefix, the empty prefix ('#') first; a prefix is balanced exactly when its stack is '#'
  - `R1B.BSTATES` = `G0A.BSCAN 'BS' BIND '#' 1 COLLECT BS CONCAT` — bracket String -> Vector of matching stacks, '#' for the empty prefix first (one more lane than characters)
- **2 agents** (R2B, R2A; models sonnet, opus), joined at d0a:
  - `R2B.BSIGN` = `'R2B.SC' BIND [ '(' '[' ')' ']' ] [ 1 1 -1 -1 ] RECORD R2B.SC GET` — Bracket char -> 1 for an opener, -1 for a closer.
  - `R2A.BVAL` = `'R2A.BC' BIND [ '(' '[' ')' ']' ] [ 1 1 -1 -1 ] RECORD R2A.BC GET` — bracket char -> 1 for an opener, -1 for a closer
- **2 agents** (R1D, Z2B; models fable, sonnet), joined at d1:
  - `R1D.CSTEP` = `'R1D.CX' BIND R1D.CX 2 DIV 'R1D.CH' BIND R1D.CH R1D.CX 3 MUL 1 ADD R1D.CH FLOOR R1D.CH EQ SELECT 1 R1D.CX 1 EQ NOT SELECT` — positive integer -> its Collatz successor (even halves, odd becomes 3n+1), with 1 staying 1
  - `Z2B.CSTEP` = `'Z2B.CX' BIND 1 Z2B.CX 2 DIV Z2B.CX 3 MUL 1 ADD Z2B.CX Z2B.CX 2 DIV FLOOR 2 MUL EQ SELECT Z2B.CX 1 EQ SELECT` — one Collatz step, 1 stays 1
- **2 agents** (R2A, Z3C; models opus, haiku), joined at d1:
  - `R2A.CSTEP` = `'R2A.CI' BIND 'R2A.CA' BIND R2A.CA 0 GET 'R2A.CV' BIND R2A.CV 2 DIV 'R2A.CH' BIND R2A.CV 1 EQ 'R2A.CD' BIND 1 R2A.CH R2A.CV 3 MUL 1 ADD R2A.CH FLOOR R2A.CH EQ SELECT R2A.CD SELECT R2A.CA 1 GET 0 1 R2A.CD SELECT ADD 2 COLLECT` — Collatz fold step: [ value steps ] and an ignored element -> next [ value steps ], standing still once value is 1
  - `Z3C.COLSTEP` = `'EL' BIND 'AC' BIND AC 0 GET 'CX' BIND AC 1 GET 'CS' BIND CX 1 EQ 'DN' BIND CX 2 DIV FLOOR 'HF' BIND HF 2 MUL CX EQ 'EV' BIND HF CX 3 MUL 1 ADD EV SELECT 'NV' BIND CX NV DN SELECT 'NX' BIND CS CS 1 ADD DN SELECT 'NS' BIND NX NS 2 COLLECT` — FOLD step on [x steps] applying one Collatz move unless x is already 1.
- **2 agents** (R2B, R2D; models sonnet, fable), joined at d1:
  - `R2B.RLE` = `'R2B.RV' BIND R2B.RV R2B.RV R2B.RUNMASK R2B.KEEP 0 R2B.RV LENGTH 1 SUB RANGE R2B.RV R2B.RUNMASK R2B.KEEP 'R2B.RS' BIND R2B.RS 1 DROP R2B.RV LENGTH 1 COLLECT CONCAT R2B.RS SUB 2 COLLECT ZIP` — Vector of numbers -> Vector of [ value count ] pairs, one per run of equal adjacent values.
  - `R2D.RLE` = `'R2D.RV' BIND R2D.RV R2D.STARTS 'R2D.RM' BIND R2D.RM R2D.WHERE 'R2D.RS' BIND R2D.RV R2D.RM R2D.KEEP R2D.RS 1 DROP R2D.RV LENGTH 1 COLLECT CONCAT R2D.RS SUB 2 COLLECT ZIP` — non-empty Vector of numbers -> run-length encoding, a Vector of [ value count ] pairs, one per maximal run of equal adjacent values
- **2 agents** (R2C, S0A; models haiku, opus), joined at d1:
  - `R2C.RUNSTARTS` = `'RV' BIND RV 1 DROP 'RC' BIND RV -1 DROP 'RP' BIND [ TRUE ] RC RP LT NOT RP RC LT NOT AND NOT CONCAT 'RS' BIND 0 RV LENGTH 1 SUB RANGE [ 'XI' BIND RS XI GET ] FILTER` — Vector -> indices where a new run of equal adjacent values starts (index 0 always starts a run)
  - `S0A.STARTS` = `'SV' BIND 0 SV LENGTH 1 SUB RANGE 'SI' BIND SV SV SI 1 SUB GET SUB 'SD' BIND SI -1 SD SD MUL 0 GT 0 TRUE PUT SELECT [ -1 GT ] FILTER` — numeric vector -> indices where a run of equal adjacent values starts
- **2 agents** (R2D, S0A; models fable, opus), joined at d1:
  - `R2D.PRIMES` = `1 'R2D.PN' BIND R2D.PN RANGE [ R2D.DIVISORS LENGTH 2 EQ ] FILTER` — positive integer n -> ascending Vector of the primes <= n (empty when none)
  - `S0A.PRIMES` = `'PN' BIND 1 PN RANGE [ S0A.ISPRIME ] FILTER` — positive integer n -> primes <= n ascending
- **2 agents** (R2D, E2D; models fable), joined at d1:
  - `R2D.COLLATZ` = `'R2D.CN' BIND 1 200 RANGE R2D.CN 0 2 COLLECT [ 'R2D.CI' BIND [ 'R2D.CC' 'R2D.CS' ] BIND 1 R2D.CC 2 DIV R2D.CC 3 MUL 1 ADD R2D.CC 2 DIV FLOOR 2 MUL R2D.CC EQ SELECT R2D.CC 1 EQ SELECT R2D.CS 1 ADD R2D.CS R2D.CC 1 EQ NOT SELECT 2 COLLECT ] FOLD 1 GET` — positive integer -> number of Collatz steps to reach 1 (runs 200 fixed steps, enough for every n below about 1000)
  - `E2D.COLLATZ` = `'N' BIND 0 199 RANGE N 0 2 COLLECT [ 'I' BIND [ 'C' 'K' ] BIND C 2 DIV 'H' BIND H FLOOR H EQ 'E' BIND H 3 C MUL 1 ADD E SELECT 'NX' BIND C K 2 COLLECT NX K 1 ADD 2 COLLECT C 1 EQ SELECT ] FOLD 1 GET` — positive integer -> number of Collatz steps to reach 1 (200 iterations of the rule, enough for n up to several thousand)
- **2 agents** (R2D, E0D; models fable), joined at d1:
  - `R2D.BALANCED-PREFIX` = `R1D.USTACKS [ '#' EQ ] MAP R2D.WHERE -1 GET` — bracket String -> length of its longest balanced prefix (0 when only the empty prefix is)
  - `E0D.BALANCED-PREFIX` = `CHARS [ [ '#' ] TRUE ] [ E0D.BSTEP ] SCAN [ E0D.BOK ] MAP 'T' BIND T E0D.IOTA 1 ADD T E0D.MASK 0 [ MAX ] FOLD` — bracket String -> length of the longest balanced prefix (0 when only the empty one)
- **2 agents** (R3A, R3B; models opus, sonnet), joined at d1:
  - `R3A.RUNIDS` = `'R3A.IV' BIND R3A.IV R3A.IV 0 GET 0 2 COLLECT [ 'R3A.IE' BIND 'R3A.IC' BIND R3A.IE R3A.IC 1 GET R3A.IC 1 GET 1 ADD R3A.IC 0 GET R3A.IE EQ SELECT 2 COLLECT ] SCAN [ 1 GET ] MAP` — non-empty Vector -> same-length Vector of run ids (0,0,1,...) that step up whenever the value changes from its left neighbour
  - `R3B.STARTS` = `'SV' BIND SV SV R3B.PREV SUB [ 0 EQ NOT ] MAP 'SM' BIND 0 SV LENGTH 1 SUB RANGE -1 SM SELECT [ -1 EQ NOT ] FILTER` — Number Vector (1+ elements) -> ascending indices where a run of equal adjacent values starts.
- **2 agents** (R3A, U2D; models opus, fable), joined at d1:
  - `R3A.MAJORITY` = `'R3A.MV' BIND R3A.MV LENGTH 'R3A.MN' BIND R3A.MV TALLY 'R3A.MT' BIND R3A.MT KEYS [ 'R3A.MK' BIND R3A.MT R3A.MK GET 2 MUL R3A.MN GT ] FILTER 0 GET` — Vector -> the value occurring in more than half the positions, or NIL
  - `U2D.MAJORITY` = `'MV' BIND MV TALLY 'MT' BIND MT KEYS [ 'MK' BIND MT MK GET 2 MUL MV LENGTH GT ] FILTER 0 GET` — V -> the value occurring in more than half of the positions, or NIL when none does
- **2 agents** (R3D, E0D; models fable), joined at d0a:
  - `R3D.PAIRS` = `'R3D.PV' BIND R3D.PV -1 DROP R3D.PV 1 DROP 2 COLLECT ZIP` — Vector -> Vector one shorter of adjacent pairs [ previous current ]
  - `E0D.ADJ` = `'V' BIND V -1 DROP V 1 DROP 2 COLLECT ZIP` — vector (n>=1) -> n-1 adjacent pairs [ v[i] v[i+1] ]
- **2 agents** (R3D, Z3C; models fable, haiku), joined at d0a:
  - `R3D.DIVISORS` = `'R3D.DN' BIND 1 R3D.DN RANGE [ R3D.DN DIV RATIO 0 GET 1 EQ ] FILTER` — positive integer -> ascending Vector of its positive divisors
  - `Z3C.DIVISORS` = `'DK' BIND 1 DK RANGE [ DK DIV RATIO 0 GET 1 EQ ] FILTER` — Ascending positive divisors of a positive integer (d divides k iff d/k has numerator 1).
- **2 agents** (R3D, Z3C; models fable, haiku), joined at d0a:
  - `R3D.ISPRIME` = `R3D.DIVISORS LENGTH 2 EQ` — positive integer -> TRUE when it is prime
  - `Z3C.ISPRIME` = `Z3C.DIVISORS LENGTH 2 EQ` — TRUE when a positive integer has exactly two divisors.
- **2 agents** (E0D, S0D; models fable), joined at d1:
  - `E0D.RLE` = `'V' BIND V E0D.DEDUPE V E0D.RUNLENS 2 COLLECT ZIP` — vector (n>=1) -> run-length encoding as [ [ value count ] ... ]
  - `S0D.RLE` = `'V' BIND V S0D.STARTS 'S' BIND V S GET S 1 DROP V LENGTH 1 COLLECT CONCAT S SUB 2 COLLECT ZIP` — run-length encoding: Vector of [ value count ] pairs
- **2 agents** (E0D, E1D; models fable), joined at d1:
  - `E0D.BINARY` = `'N' BIND 5 0 RANGE [ 'P' BIND N 2 P POW DIV FLOOR 2 E0D.MOD STR ] MAP JOIN 'S' BIND S CHARS S '1' SEARCH DROP JOIN` — integer 1..63 -> its base-2 digits as a String without leading zeros
  - `E1D.BINARY` = `'E1D.N' BIND 5 0 RANGE [ 'E1D.K' BIND E1D.N 2 E1D.K POW DIV FLOOR 'E1D.Q' BIND E1D.Q E1D.Q 2 DIV FLOOR 2 MUL SUB ] MAP STR JOIN 'E1D.S' BIND E1D.S CHARS E1D.S '1' SEARCH DROP JOIN` — integer 1..63 -> its base-2 digits as a String without leading zeros
- **2 agents** (E0D, E1D; models fable), joined at d1:
  - `E0D.COLLATZ` = `'N' BIND 1 130 RANGE N 0 2 COLLECT [ 'X' BIND [ 'C' 'S' ] BIND C 1 EQ 'D' BIND C C 2 DIV C 3 MUL 1 ADD C 2 E0D.MOD 0 EQ SELECT D SELECT S S 1 ADD D SELECT 2 COLLECT ] FOLD 1 GET` — positive integer (steps <= 130) -> number of Collatz steps to reach 1
  - `E1D.COLLATZ` = `'E1D.N' BIND 1 130 RANGE E1D.N 0 2 COLLECT [ 'E1D.I' BIND E1D.CSTEP ] FOLD 1 GET` — positive integer (up to about 60) -> number of Collatz steps to reach 1 (130 CSTEP iterations)
- **2 agents** (E1D, U3D; models fable), joined at d1:
  - `E1D.CSTEP` = `[ 'E1D.C' 'E1D.S' ] BIND E1D.C 2 DIV FLOOR 'E1D.H' BIND E1D.C E1D.S 2 COLLECT E1D.H E1D.C 3 MUL 1 ADD E1D.C E1D.H 2 MUL SUB 0 EQ SELECT E1D.S 1 ADD 2 COLLECT E1D.C 1 EQ SELECT` — [ current steps ] -> one Collatz step: [ next steps+1 ], or unchanged when current is 1
  - `U3D.CSTEP` = `[ 'U3D.CC' 'U3D.CK' ] BIND U3D.CC U3D.CC 2 DIV U3D.CC 3 MUL 1 ADD U3D.CC 2 U3D.MOD 0 EQ SELECT U3D.CC 1 EQ SELECT U3D.CK U3D.CK 1 ADD U3D.CC 1 EQ SELECT 2 COLLECT` — [ current steps ] -> one Collatz step: [ next steps+1 ], unchanged once current is 1
- **2 agents** (Z1C, Z2A; models haiku, opus), joined at d1:
  - `Z1C.DELTA` = `'C' BIND 1 -1 C ')' EQ NOT C ']' EQ NOT AND SELECT` — Maps one bracket character to +1 when it opens a bracket and -1 when it closes one.
  - `Z2A.BV` = `'Z2A.BC' BIND -1 1 Z2A.BC ']' EQ SELECT` — Character ']' -> -1, anything else -> 1
- **2 agents** (Z3D, U1B; models fable, sonnet), joined at d1:
  - `Z3D.LONGEST1` = `0 [ 'Z3D.LE' BIND Z3D.LE MUL Z3D.LE ADD ] SCAN 0 [ MAX ] FOLD` — 0/1 vector -> length of the longest run of 1s (0 for none)
  - `U1B.RUNLEN` = `0 [ 'RLE' BIND 1 ADD RLE MUL ] SCAN 0 [ MAX ] FOLD` — Vector of 0/1 flags -> length of the longest stretch of consecutive 1s (0 if none)
- **2 agents** (S0A, U2D; models opus, fable), joined at d1:
  - `S0A.BINARY` = `'BN' BIND 2 0 10 RANGE POW [ BN 1 ADD LT ] FILTER LENGTH 1 SUB 0 RANGE 'BE' BIND BN 2 BE POW DIV FLOOR 2 S0A.MOD STR JOIN` — positive integer (< 2048) -> base-2 String without leading zeros
  - `U2D.BINARY` = `'BN' BIND 0 5 RANGE [ 'BK' BIND 2 BK POW BN GT NOT ] FILTER LENGTH 1 SUB 0 RANGE [ 'BJ' BIND BN 2 BJ POW DIV FLOOR 'BQ' BIND BQ BQ 2 DIV FLOOR 2 MUL SUB ] MAP STR JOIN` — integer 1..63 -> its base-2 String of '0'/'1' without leading zeros

## Reuse of inherited Words

| condition | gen | correct solutions reaching one | inherited Words reached, per subject |
| --- | --- | --- | --- |
| open | 1 | 97/131 | O1A 18, O1B 19, O1C 15, O1D 18 |
| open | 2 | 128/132 | O2A 36, O2B 35, O2C 34, O2D 35 |
| open | 3 | 128/132 | O3A 37, O3B 35, O3C 36, O3D 36 |
| reinvention | 1 | 74/131 | R1A 6, R1B 8, R1C 8, R1D 6 |
| reinvention | 2 | 56/130 | R2A 6, R2B 7, R2C 6, R2D 7 |
| reinvention | 3 | 62/132 | R3A 8, R3B 8, R3C 8, R3D 8 |
| runs | 1 | 83/132 | E1A 7, E1B 8, E1C 6, E1D 8 |
| runs | 2 | 75/132 | E2A 8, E2B 8, E2C 8, E2D 8 |
| shuffled | 1 | 81/132 | Z1A 7, Z1B 7, Z1C 6, Z1D 7 |
| shuffled | 2 | 35/131 | Z2A 0, Z2B 0, Z2C 4, Z2D 8 |
| shuffled | 3 | 69/131 | Z3A 8, Z3B 8, Z3C 3, Z3D 8 |
| uses | 1 | 85/130 | U1A 8, U1B 8, U1C 7, U1D 8 |
| uses | 2 | 74/132 | U2A 8, U2B 8, U2C 8, U2D 8 |
| uses | 3 | 72/132 | U3A 6, U3B 8, U3C 8, U3D 8 |

## What subjects ran (cli route)

| subject | compute calls | naming inherited Words | prelude not the lexicon |
| --- | --- | --- | --- |
| E0A | 223 | 0 | 0 |
| E0B | 202 | 0 | 0 |
| E0C | 337 | 0 | 0 |
| E0D | 123 | 0 | 0 |
| E1A | 118 | 118 | 0 |
| E1B | 211 | 211 | 0 |
| E1C | 642 | 642 | 0 |
| E1D | 117 | 117 | 0 |
| E2A | 210 | 210 | 0 |
| E2B | 196 | 196 | 0 |
| E2C | 135 | 135 | 0 |
| E2D | 158 | 158 | 0 |

## H2 — lexicon entries built from other entries

- open gen1: 7/27
- open gen2: 14/42
- open gen3: 15/38
- reinvention gen1: 2/8
- reinvention gen2: 1/8
- reinvention gen3: 2/8
- runs gen1: 1/8
- runs gen2: 2/8
- shuffled gen1: 1/8
- shuffled gen2: 1/8
- shuffled gen3: 1/8
- uses gen1: 1/8
- uses gen2: 2/8
- uses gen3: 2/8

## H4 — Core Words by number of correct solutions reaching them

`BIND` 1765 · `GET` 1267 · `FILTER` 1254 · `MAP` 1143 · `ADD` 960 · `EQ` 959 · `NOT` 839 · `LENGTH` 806 · `FOLD` 804 · `SELECT` 766 · `COLLECT` 714 · `MUL` 683 · `DIV` 553 · `DROP` 523 · `SCAN` 523 · `SUB` 515 · `NIL?` 487 · `VALUES` 420 · `TOKENIZE` 411 · `CONCAT` 406 · `JSON-DECODE` 405 · `JOIN` 368 · `RANGE` 351 · `CHARS` 347 · `FLOOR` 345 · `LOWER` 343 · `UNIQUE` 332 · `MAX` 313 · `GROUP` 294 · `KEYS` 262 · `GT` 247 · `RECORD` 245 · `TALLY` 199 · `TRIM` 196 · `SORT` 186 · `ORDER` 160 · `HAS?` 156 · `REPLACE` 123 · `LT` 103 · `NIL` 89 · `TAKE` 88 · `AND` 69 · `ZIP` 69 · `UPPER` 68 · `FORMAT` 68 · `STR` 65 · `FLATTEN` 63 · `POW` 26 · `MIN` 22 · `REVERSE` 22 · `SEARCH` 22 · `TRUE` 16 · `FALSE` 15 · `PUT` 12 · `NUM` 5 · `RATIO` 4 · `MERGE` 3 · `FILL` 1

Unused (19): `RECONCILE` `ROUND` `SQRT` `GCD` `SHAPE` `RESHAPE` `DEPTH` `BSEARCH` `WITHOUT` `JSON-ENCODE` `EXEC` `CONTRACT` `FAIL` `NIL-REASON` `ABSENT` `DEF` `DEL` `DIGEST` `PRINT`

## H5 — correct solutions whose Core-only expansion disagreed

None.
