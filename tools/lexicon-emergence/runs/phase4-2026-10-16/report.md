# Lexicon-emergence run report

## Per generation

| condition | gen | agents | lexicon | correct | mean tokens | tokens ÷ expanded | Core-equivalent (H5) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| base | 0 | 4 | 0 | 175/176 | 6.5 | 0.21 | 175/175 |
| breadth | 1 | 4 | 8 | 154/156 | 8.2 | 0.23 | 154/154 |
| breadth | 2 | 4 | 8 | 176/176 | 8.5 | 0.293 | 176/176 |
| breadth | 3 | 4 | 8 | 156/156 | 8.9 | 0.231 | 156/156 |
| fixed | 1 | 4 | 8 | 175/176 | 6.7 | 0.223 | 175/175 |
| fixed | 2 | 4 | 8 | 176/176 | 5.2 | 0.169 | 176/176 |
| fixed | 3 | 4 | 8 | 172/176 | 8 | 0.269 | 172/172 |
| open | 1 | 4 | 39 | 155/156 | 8.1 | 0.245 | 155/155 |
| open | 2 | 4 | 44 | 176/176 | 7.7 | 0.263 | 176/176 |
| open | 3 | 4 | 37 | 155/156 | 8 | 0.243 | 155/155 |
| uses | 1 | 4 | 8 | 156/156 | 9 | 0.258 | 156/156 |
| uses | 2 | 4 | 8 | 176/176 | 8.2 | 0.284 | 176/176 |
| uses | 3 | 4 | 8 | 155/156 | 11.1 | 0.33 | 155/155 |

## Training and held-out tasks

Held-out results never rank a Word for the next lexicon. "reaching" is correct solutions that call an inherited Word; "tasks" is how many distinct tasks those are.

| condition | gen | train correct | train ÷ expanded | train reaching | held-out correct | held-out ÷ expanded | held-out reaching | held-out tasks |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| base | 0 | 103/104 | 0.218 | — | 72/72 | 0.199 | — | — |
| breadth | 1 | 83/84 | 0.244 | 38/83 | 71/72 | 0.209 | 49/71 | 15 |
| breadth | 2 | 104/104 | 0.289 | 50/104 | 72/72 | 0.299 | 35/72 | 9 |
| breadth | 3 | 84/84 | 0.218 | 22/84 | 72/72 | 0.253 | 30/72 | 11 |
| fixed | 1 | 104/104 | 0.219 | 93/104 | 71/72 | 0.229 | 44/71 | 11 |
| fixed | 2 | 104/104 | 0.15 | 90/104 | 72/72 | 0.198 | 46/72 | 13 |
| fixed | 3 | 103/104 | 0.283 | 91/103 | 69/72 | 0.246 | 44/69 | 13 |
| open | 1 | 83/84 | 0.28 | 41/83 | 72/72 | 0.197 | 52/72 | 13 |
| open | 2 | 104/104 | 0.264 | 69/104 | 72/72 | 0.26 | 54/72 | 15 |
| open | 3 | 84/84 | 0.267 | 49/84 | 71/72 | 0.206 | 52/71 | 14 |
| uses | 1 | 84/84 | 0.291 | 31/84 | 72/72 | 0.205 | 45/72 | 12 |
| uses | 2 | 104/104 | 0.295 | 68/104 | 72/72 | 0.268 | 54/72 | 15 |
| uses | 3 | 83/84 | 0.371 | 30/83 | 72/72 | 0.268 | 44/72 | 11 |

## Correct solutions by generation and model

| condition | gen | opus | sonnet | haiku | fable | mean runs |
| --- | --- | --- | --- | --- | --- | --- |
| base | 0 | 44/44 | 44/44 | 43/44 | 44/44 | — |
| breadth | 1 | 39/39 | 39/39 | 37/39 | 39/39 | — |
| breadth | 2 | 44/44 | 44/44 | 44/44 | 44/44 | — |
| breadth | 3 | 39/39 | 39/39 | 39/39 | 39/39 | — |
| fixed | 1 | 44/44 | 44/44 | 43/44 | 44/44 | — |
| fixed | 2 | 44/44 | 44/44 | 44/44 | 44/44 | — |
| fixed | 3 | 44/44 | 44/44 | 40/44 | 44/44 | — |
| open | 1 | 39/39 | 38/39 | 39/39 | 39/39 | — |
| open | 2 | 44/44 | 44/44 | 44/44 | 44/44 | — |
| open | 3 | 39/39 | 39/39 | 38/39 | 39/39 | — |
| uses | 1 | 39/39 | 39/39 | 39/39 | 39/39 | — |
| uses | 2 | 44/44 | 44/44 | 44/44 | 44/44 | — |
| uses | 3 | 39/39 | 39/39 | 38/39 | 39/39 | — |

## Correct solutions identical to the reference

- G0A: 1 (orders.unpriced)
- G0B: 6 (orders.unpriced, text.count, text.frequency, text.distinct, text.long-words, text.acronym)
- G0D: 1 (series.runmax)
- W1A: 2 (ledger.count_names, series.runmax)
- W1C: 3 (sensor.ratios, series.moving3, series.runmax)
- W1D: 2 (sensor.ratios, series.runmax)
- W2C: 2 (orders.unpriced, stats.sum)
- W2D: 4 (series.moving3, series.runmax, stats.sum, stats.range)
- W3A: 1 (ledger.count_names)
- W3D: 1 (ledger.count_names)
- F1A: 1 (ledger.count_names)
- F1C: 2 (series.runmax, stats.range)
- F1D: 1 (ledger.count_names)
- F3A: 1 (ledger.count_names)
- F3C: 1 (series.runmax)
- O1B: 1 (ledger.count_names)
- O1C: 3 (sensor.ratios, series.moving3, series.runmax)
- O2B: 1 (orders.unpriced)
- O2C: 3 (series.moving3, series.runmax, stats.range)
- O2D: 2 (series.moving3, series.runmax)
- O3B: 2 (series.moving3, series.runmax)
- O3C: 3 (sensor.ratios, series.moving3, series.runmax)
- U1C: 2 (series.moving3, series.runmax)
- U1D: 2 (ledger.count_names, series.runmax)
- U2B: 3 (series.moving3, series.runmax, stats.range)
- U2C: 3 (series.runmax, stats.range, stats.normalize)
- U2D: 4 (series.moving3, series.runmax, stats.range, stats.normalize)
- U3A: 1 (ledger.count_names)
- U3C: 1 (series.runmax)
- U3D: 2 (sensor.ratios, series.runmax)

## Accuracy per model

| model | agents | correct |
| --- | --- | --- |
| opus | 13 | 542/542 |
| sonnet | 13 | 541/542 |
| haiku | 13 | 532/542 |
| fable | 13 | 542/542 |

## H1 — classes defined independently by two or more agents (98 of 223 classes, 827 Words; 77 reached from two or more models)

Baseline over subjects that inherited nothing — mean Jaccard of defined classes: same-model pairs null, cross-model pairs 0.383.

- **52 agents** (G0A, G0B, G0C, G0D, W1A, W1B, W1C, W1D, W2A, W2B, W2C, W2D, W3A, W3B, W3C, W3D, F1A, F1B, F1C, F1D, F2A, F2B, F2C, F2D, F3A, F3B, F3C, F3D, O1A, O1B, O1C, O1D, O2A, O2B, O2C, O2D, O3A, O3B, O3C, O3D, U1A, U1B, U1C, U1D, U2A, U2B, U2C, U2D, U3A, U3B, U3C, U3D; models opus, sonnet, haiku, fable), joined at d0a/d1/d0:
  - `G0A.MEDIAN` = `SORT 'MS' BIND MS LENGTH 'ML' BIND MS ML 1 SUB 2 DIV FLOOR GET MS ML 2 DIV FLOOR GET ADD 2 DIV` — median of a non-empty Vector of numbers
  - `G0B.MEDIAN` = `SORT 'MS' BIND MS LENGTH 'MN' BIND MS MN 1 SUB 2 DIV FLOOR GET MS MN 2 DIV FLOOR GET ADD 2 DIV` — median of a Vector of numbers
  - `G0C.MEDIAN` = `SORT 'LS' BIND LS LENGTH 'LN' BIND LS LN 1 SUB 2 DIV FLOOR LN 2 DIV FLOOR 2 COLLECT GET G0C.SUM 2 DIV` — Median of a numeric Vector (mean of the two middle values when the count is even).
  - `G0D.MEDIAN` = `SORT 'S' BIND S LENGTH 'N' BIND S N 1 SUB 2 DIV FLOOR GET S N 2 DIV FLOOR GET ADD 2 DIV` — Median of a non-empty Vector of numbers (mean of the two middle values when even).
  - `W1A.MEDIAN` = `SORT 'S' BIND S LENGTH 'N' BIND S N 1 SUB 2 DIV FLOOR N 2 DIV FLOOR 2 COLLECT GET G0A.MEAN` — median of a non-empty numeric Vector
  - `W1B.MEDIAN` = `SORT 'SV' BIND SV SV LENGTH 1 SUB 2 DIV FLOOR GET SV SV LENGTH 2 DIV FLOOR GET ADD 2 DIV` — median of a non-empty numeric Vector
  - `W1C.MEDIAN` = `SORT 'S' BIND S S LENGTH 1 SUB 2 DIV FLOOR S LENGTH 2 DIV FLOOR 2 COLLECT GET G0A.MEAN` — Median of a non-empty numeric Vector: mean of the middle one or two sorted values.
  - `W1D.MEDIAN` = `SORT 'S' BIND S LENGTH 'N' BIND S N 1 SUB 2 DIV FLOOR GET S N 2 DIV FLOOR GET ADD 2 DIV` — median of a non-empty numeric Vector (mean of the two middle values when even)
  - `W2A.MEDIAN` = `SORT 'MS' BIND MS LENGTH 'ML' BIND MS ML 1 SUB 2 DIV FLOOR GET MS ML 2 DIV FLOOR GET ADD 2 DIV` — median of a non-empty numeric Vector
  - `W2B.MEDIAN` = `SORT 'MS' BIND MS LENGTH 'MN' BIND MS MN 1 SUB 2 DIV FLOOR GET MS MN 2 DIV FLOOR GET ADD 2 DIV` — median of a numeric Vector (mean of two middles when even)
  - `W2C.MEDIAN` = `SORT 'S' BIND S LENGTH 'N' BIND S N 1 SUB 2 DIV FLOOR N 2 DIV FLOOR 2 COLLECT GET 0 [ ADD ] FOLD 2 DIV` — Median of a numeric Vector: mean of the sorted elements at floor((n-1)/2) and floor(n/2).
  - `W2D.MEDIAN` = `SORT 'MS' BIND MS LENGTH 'ML' BIND MS ML 1 SUB 2 DIV FLOOR GET MS ML 2 DIV FLOOR GET ADD 2 DIV` — median of a non-empty numeric Vector (mean of the two middle values when the count is even)
  - `W3A.MEDIAN` = `SORT 'MS' BIND MS LENGTH 'MQ' BIND MS MQ 1 SUB 2 DIV FLOOR MQ 2 DIV FLOOR 2 COLLECT GET 0 [ ADD ] FOLD 2 DIV` — median of a non-empty numeric Vector
  - `W3B.MEDIAN` = `SORT 'MS' BIND MS LENGTH 'MN' BIND MS MN 1 SUB 2 DIV FLOOR GET MS MN 2 DIV FLOOR GET ADD 2 DIV` — median of a non-empty numeric Vector
  - `W3C.MED` = `SORT 'SM' BIND SM LENGTH 'NM' BIND SM NM 1 SUB 2 DIV FLOOR GET SM NM 2 DIV FLOOR GET ADD 2 DIV` — Median of a non-empty numeric Vector (mean of the two middle sorted values, which coincide for odd counts).
  - `W3D.MEDIAN` = `SORT 'SS' BIND SS LENGTH 'SN' BIND SS SN 1 SUB 2 DIV FLOOR GET SS SN 2 DIV FLOOR GET ADD 2 DIV` — non-empty numeric Vector -> median (mean of the two middle values when even)
  - `F1A.MEDIAN` = `SORT 'MS' BIND MS LENGTH 'ML' BIND MS ML 1 SUB 2 DIV FLOOR GET MS ML 2 DIV FLOOR GET ADD 2 DIV` — median of a non-empty Vector of numbers
  - `F1B.MEDIAN` = `SORT 'MDV' BIND MDV LENGTH 'MLN' BIND MDV MLN 1 SUB 2 DIV FLOOR GET MDV MLN 2 DIV FLOOR GET ADD 2 DIV` — median of a Vector of numbers
  - `F1C.MEDIAN` = `SORT 'S' BIND S LENGTH 'N' BIND S N 1 SUB 2 DIV FLOOR GET S N 2 DIV FLOOR GET ADD 2 DIV` — median of a Vector of numbers (mean of the two middle numbers when the count is even)
  - `F1D.MEDIAN` = `SORT 'MSV' BIND MSV LENGTH 'MLN' BIND MSV MLN 1 SUB 2 DIV FLOOR GET MSV MLN 2 DIV FLOOR GET ADD 2 DIV` — median of a non-empty Vector of numbers (mean of the two middles when even)
  - `F2A.MEDIAN` = `SORT 'SS' BIND SS LENGTH 'SL' BIND SS SL 1 SUB 2 DIV FLOOR SL 2 DIV FLOOR 2 COLLECT GET G0A.SUM 2 DIV` — median of a non-empty Vector of numbers
  - `F2B.MEDIAN` = `SORT 'MS' BIND MS LENGTH 'ML' BIND MS ML 1 SUB 2 DIV FLOOR GET MS ML 2 DIV FLOOR GET ADD 2 DIV` — median of a Vector of numbers
  - `F2C.MED` = `'MQ' BIND MQ SORT 'MS' BIND MS MQ LENGTH 1 SUB 2 DIV FLOOR GET MS MQ LENGTH 2 DIV FLOOR GET ADD 2 DIV` — median of a Vector of exact numbers
  - `F2D.MEDIAN` = `SORT 'SV' BIND SV SV LENGTH 1 SUB 2 DIV FLOOR GET SV SV LENGTH 2 DIV FLOOR GET ADD 2 DIV` — median of a non-empty Vector of numbers (mean of the two middle ones when the count is even)
  - `F3A.MEDIAN` = `SORT 'MS' BIND MS MS LENGTH 1 SUB 2 DIV FLOOR GET MS MS LENGTH 2 DIV FLOOR GET ADD 2 DIV` — median of a non-empty Vector of numbers
  - `F3B.MEDIAN` = `SORT 'KS' BIND KS LENGTH 'KN' BIND KS KN 1 SUB 2 DIV FLOOR KN 2 DIV FLOOR 2 COLLECT GET G0A.MEAN` — median of a Vector of numbers
  - `F3C.MEDIAN` = `'V' BIND V SORT 'S' BIND V LENGTH 'N' BIND S N 1 SUB 2 DIV FLOOR GET S N 2 DIV FLOOR GET ADD 2 DIV` — median of a non-empty Vector of exact numbers
  - `F3D.MEDIAN` = `SORT 'MS' BIND MS LENGTH 'ML' BIND MS ML 1 SUB 2 DIV FLOOR GET MS ML 2 DIV FLOOR GET ADD 2 DIV` — Vector of numbers -> median (mean of the two middle values when the count is even)
  - `O1A.MEDIAN` = `SORT 'MS' BIND MS MS LENGTH 1 SUB 2 DIV FLOOR GET MS MS LENGTH 2 DIV FLOOR GET ADD 2 DIV` — median of a non-empty numeric Vector
  - `O1B.MEDIAN` = `SORT 'MS' BIND MS LENGTH 'MN' BIND MS MN 1 SUB 2 DIV FLOOR GET MS MN 2 DIV FLOOR GET ADD 2 DIV` — median of a non-empty Vector of numbers
  - `O1C.MEDIAN` = `'V' BIND V SORT 'S' BIND S V LENGTH 1 SUB 2 DIV FLOOR GET S V LENGTH 2 DIV FLOOR GET ADD 2 DIV` — Median of a non-empty numeric Vector (mean of the two middle values when the count is even).
  - `O1D.MEDIAN` = `SORT 'S' BIND S LENGTH 'N' BIND S N 1 SUB 2 DIV FLOOR GET S N 2 DIV FLOOR GET ADD 2 DIV` — median of a non-empty Vector of numbers (mean of the two middle ones when even)
  - `O2A.MEDIAN` = `SORT 'MS' BIND MS LENGTH 'MN' BIND MS MN 1 SUB 2 DIV FLOOR GET MS MN 2 DIV FLOOR GET ADD 2 DIV` — median of a non-empty numeric Vector (mean of the two middle values when the count is even)
  - `O2B.MEDIAN` = `SORT 'MS' BIND MS LENGTH 'MN' BIND MS MN 1 SUB 2 DIV FLOOR GET MS MN 2 DIV FLOOR GET ADD 2 DIV` — median of a non-empty numeric Vector (mean of the two middle values when even)
  - `O2C.MEDIAN` = `'V' BIND V SORT 'S' BIND V LENGTH 'N' BIND S N 1 SUB 2 DIV FLOOR N 2 DIV FLOOR 2 COLLECT GET G0A.MEAN` — median of a Vector of numbers (mean of the two middle values when even)
  - `O2D.MEDIAN` = `SORT 'S' BIND S LENGTH 'L' BIND S L 1 SUB 2 DIV FLOOR GET S L 2 DIV FLOOR GET ADD 2 DIV` — median of a non-empty Vector of numbers (mean of the two middle values when the count is even)
  - `O3A.MEDIAN` = `SORT 'MS' BIND MS LENGTH 'MN' BIND MS MN 1 SUB 2 DIV FLOOR GET MS MN 2 DIV FLOOR GET ADD 2 DIV` — median of a non-empty Vector of numbers (mean of the two middle ones when even)
  - `O3B.MEDIAN` = `SORT 'MS' BIND MS LENGTH 'MN' BIND MS MN 1 SUB 2 DIV FLOOR GET MS MN 2 DIV FLOOR GET ADD 2 DIV` — median of a non-empty Vector of numbers (mean of the two middle ones when the count is even)
  - `O3C.MED` = `SORT 'S' BIND S LENGTH 'N' BIND S N 1 SUB 2 DIV FLOOR GET S N 2 DIV FLOOR GET ADD 2 DIV` — Median of a non-empty Vector of numbers (mean of the two middle values when even).
  - `O3D.MEDIAN` = `SORT 'MS' BIND MS LENGTH 'MN' BIND MS MN 1 SUB 2 DIV FLOOR GET MS MN 2 DIV FLOOR GET ADD 2 DIV` — median of a non-empty Vector of numbers (mean of the two middle values when the count is even)
  - `U1A.MEDIAN` = `SORT 'MS' BIND MS MS LENGTH 1 SUB 2 DIV FLOOR GET MS MS LENGTH 2 DIV FLOOR GET ADD 2 DIV` — non-empty Vector of numbers -> median (mean of the two middles when even)
  - `U1B.MEDIAN` = `SORT 'MS' BIND MS LENGTH 'MN' BIND MS MN 1 SUB 2 DIV FLOOR GET MS MN 2 DIV FLOOR GET ADD 2 DIV` — Vector of numbers -> median (mean of the two middle ones when even)
  - `U1C.MED` = `SORT 'SV' BIND SV LENGTH 'N' BIND SV N 1 SUB 2 DIV FLOOR GET SV N 2 DIV FLOOR GET ADD 2 DIV` — median of a Vector of exact numbers
  - `U1D.MEDIAN` = `SORT 'S' BIND S LENGTH 'N' BIND S N 1 SUB 2 DIV FLOOR GET S N 2 DIV FLOOR GET ADD 2 DIV` — non-empty Vector of numbers -> median (mean of the two middle values when the count is even)
  - `U2A.MEDIAN` = `SORT 'MS' BIND MS MS LENGTH 1 SUB 2 DIV FLOOR GET MS MS LENGTH 2 DIV FLOOR GET ADD 2 DIV` — non-empty Vector of numbers -> its median (mean of the two middle values when even)
  - `U2B.MEDIAN` = `SORT 'S' BIND S LENGTH 'N' BIND S N 1 SUB 2 DIV FLOOR GET S N 2 DIV FLOOR GET ADD 2 DIV` — Vector of numbers -> median
  - `U2C.MEDIAN` = `SORT 'S' BIND S LENGTH 'N' BIND S N 1 SUB 2 DIV FLOOR N 2 DIV FLOOR 2 COLLECT GET G0A.MEAN` — median of a numeric vector: mean of the middle one or two sorted elements
  - `U2D.MEDIAN` = `SORT 'S' BIND S LENGTH 'N' BIND S N 1 SUB 2 DIV FLOOR GET S N 2 DIV FLOOR GET ADD 2 DIV` — non-empty Vector of numbers -> median (mean of the two middle values when the count is even)
  - `U3A.MEDIAN` = `SORT 'MT' BIND MT LENGTH 'MN' BIND MT MN 1 SUB 2 DIV FLOOR GET MT MN 2 DIV FLOOR GET ADD 2 DIV` — non-empty Vector of numbers -> median (mean of the two middle values when even)
  - `U3B.MEDIAN` = `SORT 'MS' BIND MS MS LENGTH 1 SUB 2 DIV FLOOR GET MS MS LENGTH 2 DIV FLOOR GET ADD 2 DIV` — non-empty Vector of numbers -> median
  - `U3C.MED` = `SORT 'S' BIND S S LENGTH 1 SUB 2 DIV FLOOR GET S S LENGTH 2 DIV FLOOR GET ADD 2 DIV` — Vector of numbers -> median (mean of the two middle numbers when the count is even)
  - `U3D.MEDIAN` = `SORT 'MS' BIND MS LENGTH 'MN' BIND MS MN 1 SUB 2 DIV FLOOR GET MS MN 2 DIV FLOOR GET ADD 2 DIV` — non-empty Vector of numbers -> median (mean of the two middle values when the count is even)
- **35 agents** (G0B, U1B, G0A, G0C, G0D, W1A, W1B, W1D, W2A, W2B, W2C, W2D, W3A, W3B, W3C, W3D, F1A, F1B, F1C, F1D, F2A, F2B, F2C, F2D, F3A, F3B, F3C, F3D, U1A, U1C, U1D, U3A, U3B, U3C, U3D; models sonnet, opus, haiku, fable), joined at d1/d0a/d0:
  - `G0B.CAP` = `CHARS 'CC' BIND CC 0 GET UPPER CC 1 DROP JOIN 2 COLLECT JOIN` — upper-case the first letter of a String
  - `U1B.CAP1` = `LOWER CHARS 'CC' BIND CC 0 CC 0 GET UPPER PUT JOIN` — word -> first letter upper case, rest lower case
  - `G0A.CAP` = `CHARS 'CC' BIND CC 0 GET UPPER CC 1 DROP JOIN LOWER 2 COLLECT JOIN` — first letter upper case, rest lower case
  - `G0C.CAP` = `LOWER CHARS 'LC' BIND LC 0 GET UPPER LC 1 DROP JOIN 2 COLLECT JOIN` — Word to its first letter upper-cased and the rest lower-cased.
  - `G0D.CAP` = `'W' BIND W CHARS 1 TAKE JOIN UPPER W CHARS 1 DROP JOIN LOWER 2 COLLECT JOIN` — A word with its first letter upper-cased and the rest lower-cased.
  - `W1A.CAP` = `CHARS 'CH' BIND CH 1 TAKE JOIN UPPER CH 1 DROP JOIN LOWER 2 COLLECT JOIN` — non-empty String with first letter upper case, rest lower case
  - `W1B.CAP` = `CHARS 'CS' BIND CS 0 GET UPPER CS 1 DROP JOIN 2 COLLECT JOIN` — non-empty String with first letter upper-cased, rest unchanged
  - `W1D.CAP` = `LOWER CHARS 'CS' BIND CS 0 GET UPPER 1 COLLECT CS 1 DROP CONCAT JOIN` — a non-empty word with its first letter upper-cased and the rest lower-cased
  - `W2A.CAP` = `LOWER CHARS 'CC' BIND CC 1 TAKE JOIN UPPER CC 1 DROP JOIN 2 COLLECT JOIN` — a non-empty word with first letter upper case, rest lower case
  - `W2B.CAP` = `LOWER CHARS 'CC' BIND CC 0 GET UPPER CC 1 DROP JOIN 2 COLLECT JOIN` — word -> first letter upper case, rest lower case
  - `W2C.CAP` = `'WD' BIND WD LOWER CHARS 'CH' BIND CH 0 GET UPPER CH 1 DROP JOIN 2 COLLECT JOIN` — Capitalize one word: first letter upper case, the rest lower case.
  - `W2D.CAP` = `LOWER CHARS 'CC' BIND CC 0 GET UPPER CC 1 DROP JOIN 2 COLLECT JOIN` — non-empty word to Title case: first letter upper, the rest lower
  - `W3A.CAP` = `CHARS 'CC' BIND CC 0 GET UPPER CC 1 DROP JOIN LOWER 2 COLLECT JOIN` — non-empty String -> first letter upper case, rest lower case
  - `W3B.CAP` = `LOWER 'CW' BIND CW CHARS 0 GET UPPER CW CHARS 1 DROP JOIN 2 COLLECT JOIN` — non-empty word -> first letter upper, rest lower
  - `W3C.CAP` = `LOWER CHARS 'CC' BIND CC 0 GET UPPER CC 1 DROP JOIN 2 COLLECT JOIN` — One word: first letter upper case, the rest lower case.
  - `W3D.CAP` = `LOWER CHARS 'CC' BIND CC 1 TAKE JOIN UPPER CC 1 DROP JOIN 2 COLLECT JOIN` — word -> first letter upper case, the rest lower case
  - `F1A.CAP` = `LOWER CHARS 'CC' BIND CC 1 TAKE JOIN UPPER CC 1 DROP JOIN 2 COLLECT JOIN` — a word with first letter upper case, rest lower case
  - `F1B.CAP` = `LOWER CHARS 'CS' BIND CS 0 GET UPPER CS 1 DROP JOIN 2 COLLECT JOIN` — word -> word with first letter upper case, rest lower case
  - `F1C.CAP` = `LOWER CHARS 'C' BIND C 0 GET UPPER C 1 DROP JOIN 2 COLLECT JOIN` — word -> the word in lower case with its first letter in upper case
  - `F1D.CAP` = `CHARS 'CPC' BIND CPC 0 GET UPPER CPC 1 DROP JOIN LOWER 2 COLLECT JOIN` — a non-empty word with its first letter upper-cased and the rest lower-cased
  - `F2A.CAP` = `CHARS 'CP' BIND CP 0 GET UPPER 1 COLLECT CP 1 DROP CONCAT JOIN` — String with its first character upper-cased, rest unchanged
  - `F2B.CAP` = `LOWER 'CW' BIND CW CHARS 0 GET UPPER CW CHARS 1 DROP JOIN 2 COLLECT JOIN` — word -> first letter upper case, rest lower case
  - `F2C.CAP` = `LOWER 'CW' BIND CW CHARS 1 TAKE JOIN UPPER CW CHARS 1 DROP JOIN 2 COLLECT JOIN` — lower-cases a word, then upper-cases its first letter
  - `F2D.CAP` = `LOWER CHARS 'CC' BIND CC 0 GET UPPER CC 1 DROP JOIN 2 COLLECT JOIN` — non-empty word -> first letter upper case, the rest lower case
  - `F3A.CAP` = `'CW' BIND CW LOWER CHARS 1 TAKE JOIN UPPER CW LOWER CHARS 1 DROP JOIN 2 COLLECT JOIN` — non-empty word -> first letter upper case, rest lower case
  - `F3B.CAP` = `LOWER CHARS 'KC' BIND KC 0 GET UPPER KC 1 DROP JOIN 2 COLLECT JOIN` — word -> first letter upper case, rest lower case
  - `F3C.CAP` = `'W' BIND W LOWER CHARS 'C' BIND C 0 GET UPPER C 1 DROP JOIN 2 COLLECT JOIN` — String with its first letter upper case and the rest lower case
  - `F3D.CAP` = `LOWER CHARS 'CPC' BIND CPC 0 GET UPPER CPC 1 DROP JOIN 2 COLLECT JOIN` — non-empty String -> first letter upper case, the rest lower case
  - `U1A.CAP` = `'CW' BIND CW CHARS 1 TAKE JOIN UPPER CW CHARS 1 DROP JOIN LOWER 2 COLLECT JOIN` — non-empty word -> first letter upper case, rest lower case
  - `U1C.UP1` = `CHARS 'CH' BIND CH 0 GET UPPER CH 1 DROP JOIN 2 COLLECT JOIN` — String -> same String with its first character in upper case
  - `U1C.CAP` = `LOWER U1C.UP1` — word -> lower case with first letter upper case
  - `U1D.CAP` = `'W' BIND W CHARS 1 TAKE JOIN UPPER W CHARS 1 DROP JOIN 2 COLLECT JOIN` — non-empty String -> same String with its first character upper-cased, the rest kept
  - `U3A.CAP` = `LOWER CHARS 'TC' BIND TC 0 GET UPPER TC 1 DROP JOIN 2 COLLECT JOIN` — non-empty word -> first letter upper case, rest lower case
  - `U3B.CAP` = `CHARS 'CC' BIND CC 0 GET UPPER CC 1 DROP JOIN 2 COLLECT JOIN` — non-empty word -> first letter upper-cased, rest unchanged
  - `U3C.TITLE` = `LOWER 'W' BIND W CHARS 'C' BIND C 0 GET UPPER C 1 DROP JOIN 2 COLLECT JOIN` — String word -> lower case with its first letter in upper case
  - `U3D.CAP` = `'CW' BIND CW CHARS 0 GET UPPER CW CHARS 1 DROP JOIN LOWER 2 COLLECT JOIN` — non-empty String -> first character upper-cased, the rest lower-cased
- **31 agents** (W1A, G0B, U1A, G0A, G0C, W1B, W1C, W1D, W3A, W3B, W3C, W3D, F1A, F1B, F1C, F1D, F2A, F2B, F2C, F2D, F3A, F3B, F3C, F3D, U1B, U1C, U1D, U3A, U3B, U3C, U3D; models opus, sonnet, haiku, fable), joined at d0/d1/d0a:
  - `W1A.UNWORDS` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — join a Vector of space-free Strings with single spaces ('' for empty)
  - `G0B.SPACED` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of Strings joined with single spaces
  - `U1A.UNWORDS` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of space-free Strings -> one String joined by single spaces
  - `G0A.UNWORDS` = `[ ' ' 2 COLLECT ] MAP FLATTEN -1 DROP JOIN` — join a non-empty Vector of Strings with single spaces
  - `G0C.SPJOIN` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of texts joined by single spaces, no leading or trailing space.
  - `W1B.UNWORDS` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of Strings joined by single spaces
  - `W1C.JOINSP` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of Strings joined by single spaces with no space at either end.
  - `W1D.UNWORDS` = `[ 'W' BIND ' ' W 2 COLLECT JOIN ] MAP JOIN TRIM` — join a Vector of words with single spaces ('' for empty)
  - `W3A.UNWORDS` = `[ 'UW' BIND ' ' UW 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of space-free Strings -> one String joined by single spaces
  - `W3B.JOINSP` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of Strings -> one String joined by single spaces
  - `W3C.JOINSP` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of words joined into one String with single spaces, no leading or trailing space.
  - `W3D.UNWORDS` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of words -> one String joined by single spaces ('' for an empty Vector)
  - `F1A.UNWORDS` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — join a Vector of words with single spaces
  - `F1B.SPACEJOIN` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of Strings -> one String joined by single spaces
  - `F1C.SPJOIN` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of texts -> one text with the texts joined by single spaces
  - `F1D.SPACED` = `'' [ 'SPW' BIND ' ' SPW 3 COLLECT JOIN ] FOLD TRIM` — join a Vector of words (no whitespace inside) with single spaces
  - `F2A.UNWORDS` = `[ ' ' 2 COLLECT ] MAP FLATTEN -1 DROP JOIN` — join a non-empty Vector of Strings with single spaces
  - `F2B.JOINSP` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of Strings -> one String joined by single spaces
  - `F2C.JOINSP` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — joins a Vector of Strings with single spaces
  - `F2D.UNWORDS` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of words -> one String with the words joined by single spaces (inverse of G0A.WORDS)
  - `F3A.UNWORDS` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of words -> one String joined by single spaces
  - `F3B.SPJOIN` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of words -> one String joined by single spaces
  - `F3C.JOINSP` = `[ ' ' 2 COLLECT ] MAP FLATTEN JOIN TRIM` — Vector of Strings joined by single spaces, no space at either end
  - `F3D.UNWORDS` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of Strings -> one String, the pieces joined by single spaces
  - `O1D.LONGEST` = `'W' BIND W W [ CHARS LENGTH ] MAP G0A.ARGMAX GET` — longest String in a Vector of Strings, first on ties
  - `O2A.TITLE` = `[ G0B.CAP ] MAP G0B.SPACED` — Vector of lower-case words to one String of them capitalised and joined by single spaces
  - `O3D.LONGEST` = `'LW' BIND LW LW [ CHARS LENGTH ] MAP G0A.ARGMAX GET` — longest String in a non-empty Vector of Strings, first on ties
  - `O3D.INITIALS` = `[ CHARS 0 GET UPPER ] MAP JOIN` — Vector of non-empty Strings to one String of their upper-cased first letters ('' for empty)
  - `U1B.SPJOIN` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of Strings -> one String joined with single spaces
  - `U1C.SPJOIN` = `[ CHARS [ ' ' ] CONCAT JOIN ] MAP JOIN CHARS -1 DROP JOIN` — Vector of Strings -> one String joined by single spaces
  - `U1D.SPJOIN` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of Strings (no whitespace inside) -> one String of them joined by single spaces ('' for empty)
  - `U3A.UNWORDS` = `[ 'JW' BIND ' ' JW 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of space-free Strings -> one String joined by single spaces ('' for empty)
  - `U3B.SPJ` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of Strings -> one String joined by single spaces
  - `U3C.JOINSP` = `'' [ 'EL' BIND ' ' EL 3 COLLECT JOIN ] FOLD TRIM` — Vector of Strings -> one String joined by single spaces
  - `U3C.STK` = `[ 'Z' ] [ 'CH' BIND 'ST' BIND ST -1 TAKE 0 GET CH 2 COLLECT JOIN 'PR' BIND PR '()' EQ NOT PR '[]' EQ NOT AND NOT 'TR' BIND ST -1 DROP ST CH 1 COLLECT CONCAT 2 COLLECT 0 1 TR SELECT GET ] FOLD` — Vector of single bracket Strings -> final stack as a Vector (sentinel 'Z' at bottom); a close that does not match the top is pushed, so the length minus one is the unmatched count
  - `U3D.SPJOIN` = `[ 'JW' BIND ' ' JW 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of Strings -> one String joined with single spaces ('' for empty)
- **31 agents** (G0A, G0B, G0C, G0D, W1B, W1D, W2A, W2D, W3B, W3D, F1A, F1B, F1D, F2A, F2B, F2D, F3A, F3D, O1A, O1B, O1D, O2A, O2D, O3B, O3D, U1C, U2A, U2D, U3A, U3B, U3D; models opus, sonnet, haiku, fable), joined at d0a/d1/d0:
  - `G0A.ABS` = `'AV' BIND AV AV -1 MUL MAX` — absolute value, lifts over Vectors
  - `G0B.ABS` = `'AD' BIND AD AD -1 MUL MAX` — absolute value of a scalar or element-wise of a Vector
  - `G0C.ABS` = `'LD' BIND LD 0 LD SUB MAX` — Absolute value of an exact number.
  - `G0D.ABS` = `'X' BIND X -1 MUL X X 0 LT SELECT` — Absolute value of a scalar, or lane by lane over a Vector.
  - `W1B.ABS` = `'AV' BIND AV AV -1 MUL MAX` — absolute value, lifts over Vectors
  - `W1D.ABS` = `'X' BIND 0 X SUB X X 0 LT SELECT` — absolute value, lifts over Vectors
  - `W2A.ABS` = `'AX' BIND AX AX -1 MUL MAX` — absolute value, lifts over Vectors
  - `W2D.ABS` = `'AA' BIND AA AA -1 MUL MAX` — absolute value of a number, lifts lane by lane over a Vector
  - `W3B.ABS` = `'AX' BIND AX AX -1 MUL MAX` — absolute value, lifts over Vectors
  - `W3D.ABS` = `'AB' BIND AB 0 AB SUB MAX` — absolute value, lifts over Vectors
  - `F1A.ABS` = `'AX' BIND AX AX -1 MUL MAX` — absolute value, lifts over Vectors
  - `F1B.ABS` = `'AV' BIND AV AV -1 MUL MAX` — absolute value of a number or element-wise of a Vector
  - `F1D.ABS` = `'ABV' BIND ABV ABV -1 MUL MAX` — absolute value; lifts over Vectors
  - `F2A.ABS` = `'AX' BIND AX AX -1 MUL MAX` — absolute value, lifts over Vectors
  - `F2B.ABS` = `'AV' BIND AV AV -1 MUL MAX` — absolute value, lifts over Vectors
  - `F2D.ABS` = `'AX' BIND AX 0 AX SUB MAX` — absolute value, lifts over Vectors
  - `F3A.ABS` = `'AX' BIND AX AX -1 MUL MAX` — absolute value, lifts over Vectors
  - `F3D.ABS` = `'AV' BIND AV AV -1 MUL MAX` — absolute value, lifts over Vectors
  - `O1A.ABS` = `'AB' BIND AB AB -1 MUL MAX` — absolute value, lifts over Vectors
  - `O1B.ABS` = `'AX' BIND AX AX -1 MUL MAX` — absolute value of a number, lifts over Vectors
  - `O1D.ABS` = `'D' BIND D -1 MUL D D 0 LT SELECT` — absolute value, lifts over Vectors
  - `O2A.ABS` = `'AB' BIND AB 0 AB SUB MAX` — absolute value of a number, element-wise over a Vector
  - `O2D.ABS` = `'X' BIND X X -1 MUL MAX` — absolute value of a number, element-wise over a Vector
  - `O3B.ABS` = `'AX' BIND AX 0 AX SUB MAX` — absolute value of a number or element-wise of a Vector of numbers
  - `O3D.ABS` = `'AV' BIND AV AV -1 MUL MAX` — absolute value of a number; lifts over a Vector
  - `U1C.ABSV` = `'AV' BIND AV AV -1 MUL MAX` — elementwise absolute value of a Vector (MAX of x and -x)
  - `U2A.ABS` = `'AB' BIND AB AB -1 MUL MAX` — number or Vector of numbers -> absolute value, element-wise
  - `U2D.ABS` = `'X' BIND X X -1 MUL MAX` — number or Vector of numbers -> absolute value, lane by lane
  - `U3A.ABS` = `'AX' BIND AX AX -1 MUL MAX` — number or Vector of numbers -> absolute value, lane by lane
  - `U3B.ABS` = `'AD' BIND AD AD -1 MUL MAX` — number or Vector of numbers -> absolute value, element-wise
  - `U3D.ABS` = `'AX' BIND AX 0 AX SUB MAX` — absolute value of a number, element-wise over a Vector
- **25 agents** (G0A, G0B, G0D, W1B, W2A, W2B, W2D, F1A, F1B, F1C, F1D, F2A, F2B, F2C, F2D, F3A, F3B, F3D, U1B, U1D, U2A, U2B, U2D, U3A, U3D; models opus, sonnet, fable, haiku), joined at d0a/d1/d0:
  - `G0A.TOPKEY` = `'TK' BIND TK KEYS TK VALUES G0A.ARGMAX GET` — key of a Record whose numeric value is largest, first on ties
  - `G0B.TOPKEY` = `'TR' BIND TR KEYS TR VALUES G0B.ARGMAX GET` — Record of key->number: the key with the largest value, first on ties
  - `G0D.TOP-KEY` = `'T' BIND T KEYS T VALUES G0D.ARGMAX GET` — Key of a Record whose numeric value is largest; the first key on ties.
  - `W1B.TOPKEY` = `'RC' BIND RC KEYS RC VALUES -1 MUL ORDER 0 GET GET` — Record key -> number: key with the largest value, first on ties
  - `W2A.ARGMAXKEY` = `'AR' BIND AR KEYS AR VALUES W1A.ARGMAX GET` — Record -> key of the first largest numeric value
  - `W2B.TOPKEY` = `'TR' BIND TR KEYS TR VALUES W1A.ARGMAX GET` — Record of numbers -> the key with the largest value, first on ties
  - `W2D.TOPKEY` = `'TR' BIND TR KEYS TR VALUES W1A.ARGMAX GET` — key of a Record whose numeric value is largest; the first such key on ties
  - `F1A.TOPKEY` = `'TK' BIND TK KEYS TK VALUES F1A.ARGMAX GET` — key of a Record whose value is largest, first on ties
  - `F1B.TOPKEY` = `'TKR' BIND TKR KEYS TKR VALUES -1 MUL ORDER 0 GET GET` — Record of numbers -> key with the largest value, first key on ties
  - `F1C.ARGKEY` = `'R' BIND R VALUES [ -1 MUL ] MAP ORDER 0 GET 'I' BIND R KEYS I GET` — Record -> the key whose value is largest, the first such key on ties
  - `F1D.TOPKEY` = `'TKR' BIND TKR KEYS TKR VALUES F1D.ARGMAX GET` — key of a Record whose numeric value is largest; the earliest key when several tie
  - `F2A.TOPKEY` = `'TK' BIND TK KEYS TK VALUES F2A.ARGMAX GET` — key of a Record whose numeric value is largest, first key on a tie
  - `F2B.TOPKEY` = `'TR' BIND TR KEYS TR VALUES -1 MUL ORDER 0 GET GET` — Record of numbers -> key with the largest value, first key on ties
  - `F2C.ARGMAX` = `'AR' BIND AR KEYS AR VALUES -1 MUL ORDER 0 GET GET` — Record -> key with the largest value, first key on ties
  - `F2D.MAXKEY` = `'MR' BIND MR KEYS MR VALUES F2D.ARGMAX GET` — Record with numeric values -> the key holding the largest value, the earliest key when several tie
  - `F3A.TOPKEY` = `'TK' BIND TK KEYS TK VALUES F3A.ARGMAX GET` — Record of numbers -> key with the largest value (first in key order on ties)
  - `F3B.TOPKEY` = `'KR' BIND KR KEYS KR VALUES -1 MUL ORDER 0 GET GET` — Record of numbers -> key with the largest value, first on ties
  - `F3D.TOPKEY` = `'TR' BIND TR KEYS TR VALUES F3D.ARGMAX GET` — Record with number values -> the key with the largest value, the first on ties
  - `U1B.TOPKEY` = `'TR' BIND TR KEYS TR VALUES -1 MUL ORDER 0 GET GET` — Record of key to number -> the key with the largest value, first one on a tie
  - `U1D.TOPKEY` = `'R' BIND R KEYS R VALUES U1D.ARGMAX GET` — Record with numeric values -> the key holding the largest value, the first such key on a tie
  - `U2A.RARGMAX` = `'RA' BIND RA KEYS RA VALUES U1A.ARGMAX GET` — Record of numbers -> the key holding the largest value, first key on ties
  - `U2B.TOPKEY` = `'R' BIND R KEYS R VALUES U1A.ARGMAX GET` — Record of numbers -> key with largest value, first on ties
  - `U2D.TOPKEY` = `'R' BIND R KEYS R VALUES U1A.ARGMAX GET` — Record of numbers -> the key with the largest value, first on ties
  - `U3A.TOPKEY` = `'TR' BIND TR KEYS TR VALUES U3A.ARGMAX GET` — Record of numbers -> key with the largest value, the first in key order on ties
  - `U3D.TOPKEY` = `'TR' BIND TR KEYS TR VALUES U3D.ARGMAX GET` — Record with numeric values -> the key holding the largest value, the earliest key when several tie
- **25 agents** (O2A, G0B, G0C, G0D, W1B, W1C, W1D, W2A, W2B, W2D, W3B, F1C, F2A, F2D, F3B, O1A, O1C, O1D, O2C, U1B, U1C, U2A, U2C, U2D, U3C; models opus, sonnet, haiku, fable), joined at d1/d0a/d0:
  - `O2A.BYKEY` = `'BKP' BIND BKP [ 0 GET ] MAP BKP [ 1 GET ] MAP GROUP 'BKG' BIND BKG KEYS BKG VALUES [ G0A.SUM ] MAP RECORD` — Vector of [ key amount ] pairs to a Record from each key to the sum of its amounts, keys in order of first appearance
  - `G0B.BYNAME` = `'BL' BIND BL G0B.FIRSTS BL G0B.SECONDS G0B.GROUPSUM` — Vector of [key amount] rows -> Record key -> summed amount
  - `G0C.BYNAME` = `'LV' BIND LV [ 0 GET ] MAP LV [ 1 GET ] MAP G0C.GROUPSUM` — Entries [name amount] to a Record from each name to its total amount.
  - `G0D.LEDGER-SUMS` = `'L' BIND L [ 0 GET ] MAP L [ 1 GET ] MAP G0D.SUMS-BY` — Vector of [ name amount ] entries to a Record name -> summed amount.
  - `W1B.BYNAME` = `'LE' BIND LE [ 0 GET ] MAP LE [ 1 GET ] MAP GROUP 'GR' BIND GR KEYS GR VALUES [ G0A.SUM ] MAP RECORD` — Vector of [ name amount ] -> Record name to summed amount, first-appearance order
  - `W1C.BYNAME` = `'E' BIND E [ 0 GET ] MAP 'N' BIND E [ 1 GET ] MAP 'A' BIND N A GROUP 'R' BIND R KEYS R VALUES [ G0A.SUM ] MAP RECORD` — Vector of [name amount] entries to a Record from name to the sum of its amounts, names in first-appearance order.
  - `W1D.BYNAME` = `'L' BIND L W1D.NAMES L W1D.AMTS GROUP 'G' BIND G KEYS G VALUES [ G0A.SUM ] MAP RECORD` — Vector of [ name amount ] pairs to a Record name -> summed amount, names in first-appearance order
  - `W2A.LEDGER` = `'LE' BIND LE [ 0 GET ] MAP LE [ 1 GET ] MAP W2A.SUMBY` — Vector of [ name amount ] -> Record name to summed amount
  - `W2B.BYNAME` = `'LG' BIND LG [ 0 GET ] MAP LG [ 1 GET ] MAP W2B.GSUM` — Vector of [name amount] entries -> Record name to summed amounts
  - `W2D.SUMS` = `'SP' BIND SP [ 0 GET ] MAP SP [ 1 GET ] MAP GROUP 'SG' BIND SG KEYS SG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — Vector of [ key amount ] pairs to a Record key -> sum of its amounts, keys in order of first appearance
  - `W3B.BYNAME` = `'LE' BIND LE W3B.NAMES LE W3B.AMTS GROUP 'LG' BIND LG KEYS LG VALUES [ W3B.SUM ] MAP RECORD` — entries -> Record name -> sum of amounts, first-appearance order
  - `F1C.BYNAME` = `'E' BIND E [ 0 GET ] MAP E [ 1 GET ] MAP G0A.SUMBY` — ledger Vector of [name amount] -> Record from each name to the sum of its amounts
  - `F2A.BYNAME` = `'LB' BIND LB [ 0 GET ] MAP LB [ 1 GET ] MAP G0A.SUMBY` — Vector of [ name amount ] -> Record name to summed amount, first-appearance order
  - `F2D.LEDGER` = `'LG' BIND LG [ 0 GET ] MAP LG [ 1 GET ] MAP G0A.SUMBY` — Vector of [ name amount ] entries -> Record name to sum of its amounts, names in first-appearance order
  - `F3B.BYNAME` = `'KL' BIND KL [ 0 GET ] MAP KL [ 1 GET ] MAP G0A.SUMBY` — Vector of [name amount] entries -> Record name to summed amount
  - `O1A.BYNAME` = `'LG' BIND LG [ 0 GET ] MAP LG [ 1 GET ] MAP G0A.SUMBY` — Vector of [ name amount ] entries to Record name -> summed amount, first-appearance order
  - `O1C.BYNAME` = `'L' BIND L [ 0 GET ] MAP L [ 1 GET ] MAP G0A.SUMBY` — Vector of [name amount] entries to a Record from each name to the sum of its amounts, first-appearance order.
  - `O1D.LEDGER` = `'L' BIND L O1D.LNAMES L O1D.LAMTS G0A.SUMBY` — Vector of [name amount] pairs to a Record name -> sum of amounts, first-appearance order
  - `O2C.BYNAME` = `'L' BIND L [ 0 GET ] MAP L [ 1 GET ] MAP O2C.GROUPSUM` — Vector of [name amount] entries to a Record from each name to its summed amount, names in first-appearance order
  - `U1B.BYNAME` = `'BL' BIND BL [ 0 GET ] MAP BL [ 1 GET ] MAP G0A.SUMBY` — Vector of [name amount] entries -> Record name to sum of amounts, first-appearance order
  - `U1C.SUMREC` = `'L' BIND L [ 0 GET ] MAP L [ 1 GET ] MAP G0A.SUMBY` — ledger entries -> Record from each name to the sum of its amounts
  - `U2A.SUMBY` = `'SBP' BIND SBP [ 0 GET ] MAP SBP [ 1 GET ] MAP U2A.GSUM` — Vector of [key number] pairs -> Record from each key to the sum of its numbers, first-appearance order
  - `U2C.BYNAME` = `'ENT' BIND ENT [ 0 GET ] MAP ENT [ 1 GET ] MAP U2C.SUMBY` — ledger entries -> Record from each name to the sum of its amounts
  - `U2D.LEDGER` = `'L' BIND L [ 0 GET ] MAP L [ 1 GET ] MAP U2D.SUMBY` — Vector of [ name amount ] pairs -> Record name -> total amount, names in first-appearance order
  - `U3C.BYNAME` = `'E' BIND E [ 0 GET ] MAP E [ 1 GET ] MAP U2A.GSUM` — Vector of [name amount] entries -> Record from each name to the sum of its amounts
- **19 agents** (W1A, G0A, U1A, G0B, G0D, W1D, W3A, W3D, F1A, F1D, F2A, F2D, F3A, F3D, U1C, U1D, U3A, U3C, U3D; models opus, sonnet, fable, haiku), joined at d0/d1/d0a:
  - `W1A.ARGMAX` = `-1 MUL ORDER 0 GET` — index of the first largest number in a non-empty numeric Vector
  - `G0A.ARGMAX` = `-1 MUL ORDER 0 GET` — index of the largest number in a Vector, first on ties
  - `U1A.ARGMAX` = `-1 MUL ORDER 0 GET` — Vector of numbers -> index of its largest element, first on ties
  - `G0B.ARGMAX` = `-1 MUL ORDER 0 GET` — index of the first largest element of a Vector of numbers
  - `G0D.ARGMAX` = `-1 MUL ORDER 0 GET` — Index of the largest number in a Vector; the first one on ties.
  - `W1D.ARGMAX` = `'V' BIND 0 V SUB ORDER 0 GET` — index of the largest number in a Vector, the first one among ties
  - `W3A.ARGMAX` = `-1 MUL ORDER 0 GET` — numeric Vector -> index of its largest element, first on ties
  - `W3D.ARGMAX` = `'AV' BIND 0 AV SUB ORDER 0 GET` — numeric Vector -> index of its largest value, the first one on ties
  - `F1A.ARGMAX` = `-1 MUL ORDER 0 GET` — index of the first largest number in a Vector
  - `F1D.ARGMAX` = `-1 MUL ORDER 0 GET` — index of the largest number in a Vector; the first of them when several tie
  - `F2A.ARGMAX` = `-1 MUL ORDER 0 GET` — index of the largest number in a Vector, first one on a tie
  - `F2D.ARGMAX` = `'AV' BIND 0 AV SUB ORDER 0 GET` — Vector of numbers -> index of its largest element, the first one when several tie
  - `F3A.ARGMAX` = `-1 MUL ORDER 0 GET` — Vector of numbers -> index of its largest element (first on ties)
  - `F3D.ARGMAX` = `-1 MUL ORDER 0 GET` — Vector of numbers -> index of its largest element, the first one when several tie
  - `U1C.ARGMAX` = `-1 MUL ORDER 0 GET` — Vector of numbers -> index of its first largest element
  - `U1D.ARGMAX` = `-1 MUL ORDER 0 GET` — Vector of numbers -> index of its largest value, the first one when several tie
  - `U3A.ARGMAX` = `-1 MUL ORDER 0 GET` — Vector of numbers -> index of its largest element, the first one on ties
  - `U3C.FIRSTMAX` = `-1 MUL ORDER 0 GET` — Vector of numbers -> index of its first maximum
  - `U3D.ARGMAX` = `'AV' BIND 0 AV SUB ORDER 0 GET` — Vector of numbers -> index of the largest value, the first of them when several tie
- **15 agents** (W1B, O1B, U1B, W1D, W3B, W3C, W3D, O1C, O3B, O3D, U1A, U1C, U1D, U3B, U3D; models sonnet, fable, haiku, opus), joined at d0/d1:
  - `W1B.R1` = `'()' '' REPLACE '[]' '' REPLACE` — one pass deleting adjacent () then [] pairs from a String
  - `O1B.RED` = `'()' '' REPLACE '[]' '' REPLACE` — one pass deleting adjacent () and [] pairs from a bracket String
  - `O1B.REDALL` = `O1B.RED O1B.RED O1B.RED O1B.RED O1B.RED O1B.RED` — reduce a bracket String of at most 12 characters to its unmatched remainder
  - `U1B.RED1` = `'()' '' REPLACE '[]' '' REPLACE` — string -> string with every adjacent () and [] pair deleted once
  - `W1B.REDUCE` = `W1B.R1 W1B.R1 W1B.R1 W1B.R1 W1B.R1 W1B.R1` — bracket String (<=12 chars) with all matched pairs deleted repeatedly
  - `W1D.BPASS` = `'()' '' REPLACE '[]' '' REPLACE` — one pass deleting every adjacent () and [] pair from a String
  - `W1D.BSTRIP` = `W1D.BPASS W1D.BPASS W1D.BPASS W1D.BPASS W1D.BPASS W1D.BPASS` — delete adjacent matched bracket pairs until none remain (Strings up to 12 chars); '' iff balanced
  - `W3B.RED1` = `'()' '' REPLACE '[]' '' REPLACE` — one pass deleting adjacent () and [] pairs
  - `W3B.REDUCE` = `W3B.RED1 W3B.RED1 W3B.RED1 W3B.RED1 W3B.RED1 W3B.RED1` — bracket String of at most 12 chars with all matched pairs deleted
  - `W3C.REDUCE` = `'R' BIND [ '()' '[]' '()' '[]' '()' '[]' '()' '[]' '()' '[]' '()' '[]' ] R [ '' REPLACE ] FOLD` — Bracket String reduced to normal form by deleting adjacent () and [] pairs (12 REPLACE passes cover 12 characters).
  - `W3D.BRED` = `'()' '' REPLACE '[]' '' REPLACE` — bracket String -> same with every adjacent '()' and '[]' pair deleted once
  - `W3D.BRED6` = `W3D.BRED W3D.BRED W3D.BRED W3D.BRED W3D.BRED W3D.BRED` — bracket String (<=12 chars) -> fully reduced: all nested matched pairs deleted
  - `O1C.REDUCE` = `'S' BIND 0 5 RANGE S [ 'E' BIND '()' '' REPLACE '[]' '' REPLACE ] FOLD` — String to its reduced form after deleting adjacent () and [] pairs (six rounds cover twelve characters).
  - `O3B.PASS` = `'()' '' REPLACE '[]' '' REPLACE` — one pass deleting every adjacent () and [] pair from a String
  - `O3B.RED` = `O3B.PASS O3B.PASS O3B.PASS O3B.PASS O3B.PASS O3B.PASS` — bracket String of at most 12 characters reduced to what remains after deleting matched pairs until none is left
  - `O3D.REDUCE` = `'BS' BIND 1 8 RANGE BS [ 'IG' BIND '()' '' REPLACE '[]' '' REPLACE ] FOLD` — bracket String with adjacent matched pairs () and [] deleted repeatedly (8 passes, enough for 16 chars); '' when balanced
  - `U1A.BREDUCE` = `'()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE` — bracket String (up to 12 chars) -> what remains after repeatedly deleting adjacent () and [] pairs
  - `U1B.REDUCE` = `U1B.RED1 U1B.RED1 U1B.RED1 U1B.RED1 U1B.RED1 U1B.RED1` — bracket string (<=12 chars) -> string after deleting adjacent matched pairs until none is left
  - `U1C.REDUCE` = `'()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE` — bracket string -> its reduced form after deleting adjacent () and [] pairs (six rounds suffice for 12 characters)
  - `U1D.BRED` = `'()' '' REPLACE '[]' '' REPLACE` — String of brackets -> same with every adjacent '()' and '[]' pair deleted once (one reduction round)
  - `U1D.BREDUCE` = `U1D.BRED U1D.BRED U1D.BRED U1D.BRED U1D.BRED U1D.BRED` — String of at most 12 brackets -> its normal form after deleting adjacent matched pairs until none remain ('' iff balanced)
  - `U3B.RED1` = `'()' '' REPLACE '[]' '' REPLACE` — String of brackets -> one pass deleting all adjacent () and [] pairs
  - `U3B.RED` = `U3B.RED1 U3B.RED1 U3B.RED1 U3B.RED1 U3B.RED1 U3B.RED1` — String of up to 12 brackets -> what remains after deleting matched pairs until none is left
  - `U3D.BREDUCE` = `'()' '' REPLACE '[]' '' REPLACE` — String -> the String with every adjacent () and [] pair deleted once
  - `U3D.BNORM` = `U3D.BREDUCE U3D.BREDUCE U3D.BREDUCE U3D.BREDUCE U3D.BREDUCE U3D.BREDUCE` — bracket String of at most 12 chars -> its irreducible form after deleting all adjacent matched pairs ('' iff balanced)
- **15 agents** (W2B, G0A, O2D, U2D, G0B, G0C, G0D, W2A, W2C, W2D, O2A, O2C, U2A, U2B, U2C; models sonnet, opus, fable, haiku), joined at d0a/d0/d1:
  - `W2B.VAR` = `W2B.DEV 'VD' BIND VD VD MUL W2B.MEAN` — population variance of a numeric Vector
  - `G0A.VAR` = `G0A.DEVS 'VD' BIND VD VD MUL G0A.MEAN` — population variance
  - `O2D.VAR` = `O2D.DEVS 'D' BIND D D MUL G0A.MEAN` — population variance of a Vector of numbers
  - `U2D.VAR` = `U2D.DEVS 'D' BIND D D MUL G0A.MEAN` — Vector of numbers -> population variance
  - `G0B.VAR` = `G0B.DEV 'DD' BIND DD DD MUL G0B.MEAN` — population variance
  - `G0C.VAR` = `'LV' BIND LV G0C.MEAN 'LM' BIND LV [ LM SUB G0C.SQ ] MAP G0C.MEAN` — Population variance of a numeric Vector.
  - `G0D.VARIANCE` = `'V' BIND V V G0D.MEAN SUB 'D' BIND D D MUL G0D.MEAN` — Population variance of a Vector of numbers.
  - `W2A.VAR` = `'VV' BIND VV VV W2A.MEAN SUB 2 POW W2A.MEAN` — population variance of a numeric Vector
  - `W2C.VARIANCE` = `'V' BIND V W2C.MEAN 'M' BIND V M SUB 'D' BIND D D MUL W2C.MEAN` — Population variance: mean of squared deviations from the mean.
  - `W2D.VARIANCE` = `W2D.DEVS 'VD' BIND VD VD MUL W2D.MEAN` — population variance of a numeric Vector, as a bare scalar
  - `O2A.VAR` = `'VV' BIND VV VV G0A.MEAN SUB 'VD' BIND VD VD MUL G0A.MEAN` — population variance of a numeric Vector
  - `O2C.VAR` = `'V' BIND V G0A.MEAN 'M' BIND V [ M SUB 'X' BIND X X MUL ] MAP G0A.MEAN` — population variance of a Vector of numbers
  - `U2A.VAR` = `'VV' BIND VV VV G0A.MEAN SUB 'VD' BIND VD VD MUL G0A.MEAN` — Vector of numbers -> population variance
  - `U2B.VAR` = `'V' BIND V V G0A.MEAN SUB 'D' BIND D D MUL G0A.MEAN` — Vector of numbers -> population variance
  - `U2C.VAR` = `'V' BIND V G0A.MEAN 'M' BIND V M SUB 'D' BIND D D MUL G0A.MEAN` — population variance of a numeric vector
- **14 agents** (G0C, O2C, G0B, W2A, W2B, W2D, F1A, F2B, F2C, F3A, F3B, F3D, O2A, U2B; models haiku, sonnet, opus, fable), joined at d1/d0a/d0:
  - `G0C.LINE` = `'LR' BIND LR 'qty' GET LR 'price' GET MUL` — qty times price of one order Record.
  - `O2C.LINE` = `'O' BIND O 'qty' GET 0 O 'price' GET O 'price' GET NIL? SELECT MUL` — qty times price of one order Record; 0 when the order has no price
  - `G0B.ORDVAL` = `'OO' BIND OO 'price' GET 'OP' BIND 0 OP OP NIL? SELECT OO 'qty' GET MUL` — order Record -> qty times price, 0 when it has no price
  - `W2A.REV` = `'OD' BIND OD 'qty' GET OD 'price' GET MUL` — order Record -> qty times price
  - `W2B.LINE` = `'OO' BIND OO 'price' GET 'OP' BIND OO 'qty' GET 0 OP OP NIL? SELECT MUL` — order Record -> qty times price, 0 when it has no price
  - `W2D.LINE` = `'LO' BIND LO 'price' GET 'LP' BIND 0 LP LP NIL? SELECT LO 'qty' GET MUL` — order Record to qty times price, 0 when it has no price
  - `F1A.LINE` = `'OD' BIND OD 'qty' GET OD F1A.PRICE0 MUL` — order Record -> qty times price (0 when unpriced)
  - `F2B.LINE` = `'OD' BIND OD 'price' GET 'OP' BIND 0 OP OP NIL? SELECT OD 'qty' GET MUL` — order Record -> qty times price, 0 when it has no price
  - `F2C.LINE` = `'LR' BIND LR 'qty' GET LR 'price' GET MUL` — order Record -> qty times price
  - `F3A.LINEREV` = `'OD' BIND OD 'price' GET 'PR' BIND 0 PR PR NIL? SELECT OD 'qty' GET MUL` — order Record -> qty times price, 0 when price is missing
  - `F3B.LINE` = `'KO' BIND KO 'price' GET 'KP' BIND 0 KP KP NIL? SELECT KO 'qty' GET MUL` — order Record -> qty times price, 0 when no price
  - `F3D.LINE` = `'LO' BIND LO 'qty' GET LO F3D.PRICE0 MUL` — order Record -> qty times price (0 when unpriced)
  - `O2A.LINEREV` = `'OL' BIND OL 'price' GET 'OP' BIND OL 'qty' GET 0 OP OP NIL? SELECT MUL` — order Record to qty times price, or 0 when it has no price
  - `U2B.LINE` = `'O' BIND O 'price' GET 'P' BIND 0 P P NIL? SELECT O 'qty' GET MUL` — order Record -> qty times price, 0 when no price
- **14 agents** (G0C, W1A, W2A, W2B, W3B, F1D, F2A, F2B, F2C, F3C, O1D, O3D, U1B, U1C; models haiku, opus, sonnet, fable), joined at d1/d0a:
  - `G0C.MAD` = `'LV' BIND LV G0C.MEDIAN 'LM' BIND LV [ LM SUB G0C.ABS ] MAP G0C.MEDIAN` — Median absolute deviation of a numeric Vector.
  - `W1A.MAD` = `'V' BIND V V W1A.MEDIAN SUB 'D' BIND D D -1 MUL MAX W1A.MEDIAN` — median absolute deviation from the median
  - `W2A.MAD` = `'XV' BIND XV XV W2A.MEDIAN SUB W2A.ABS W2A.MEDIAN` — median absolute deviation of a numeric Vector
  - `W2B.MAD` = `'AX' BIND AX W2B.MEDIAN 'AM' BIND AX AM SUB 'AD' BIND AD AD -1 MUL MAX W2B.MEDIAN` — median absolute deviation of a numeric Vector
  - `W3B.MAD` = `'MD' BIND MD MD W3B.MEDIAN SUB W3B.ABS W3B.MEDIAN` — median absolute deviation
  - `F1D.MAD` = `'MDV' BIND MDV MDV F1D.MEDIAN SUB F1D.ABS F1D.MEDIAN` — median absolute deviation of a Vector of numbers
  - `F2A.MAD` = `'MDV' BIND MDV MDV F2A.MEDIAN SUB F2A.ABS F2A.MEDIAN` — median absolute deviation from the median
  - `F2B.MAD` = `'XS' BIND XS XS F2B.MEDIAN SUB F2B.ABS F2B.MEDIAN` — median absolute deviation of a Vector of numbers
  - `F2C.MAD` = `'MX' BIND MX F2C.MED 'MM' BIND MX MM SUB 'MD' BIND MD MD -1 MUL MAX F2C.MED` — median absolute deviation of a Vector of exact numbers
  - `F3C.MAD` = `'V' BIND V F3C.MEDIAN 'M' BIND V [ 'X' BIND M X SUB X M SUB MAX ] MAP F3C.MEDIAN` — median absolute deviation from the median of a Vector
  - `O1D.MAD` = `'V' BIND V V O1D.MEDIAN SUB O1D.ABS O1D.MEDIAN` — median absolute deviation of a Vector of numbers
  - `O3D.MAD` = `'MX' BIND MX MX O3D.MEDIAN SUB O3D.ABS O3D.MEDIAN` — median absolute deviation of a Vector of numbers
  - `U1B.MAD` = `'AX' BIND AX AX U1B.MEDIAN SUB 'AD' BIND AD AD -1 MUL MAX U1B.MEDIAN` — Vector of numbers -> median absolute deviation
  - `U1C.MAD` = `'V' BIND V V U1C.MED 'MD' BIND MD SUB U1C.ABSV U1C.MED` — median absolute deviation of a Vector of exact numbers
- **12 agents** (G0C, W2A, O2A, U2A, G0A, G0B, G0D, W2B, W2D, O2C, O2D, U2D; models haiku, opus, sonnet, fable), joined at d0/d1/d0a:
  - `G0C.ORDERS` = `JSON-DECODE 'orders' GET` — JSON text to the Vector of its order Records.
  - `W2A.ORDERS` = `JSON-DECODE 'orders' GET` — JSON String -> Vector of order Records under its orders key
  - `O2A.ORDERS` = `JSON-DECODE 'orders' GET` — JSON object String to the Vector of order Records under its "orders" key
  - `U2A.ORDERS` = `JSON-DECODE 'orders' GET` — JSON String -> the Vector of Records under its "orders" key
  - `G0A.REVENUE` = `JSON-DECODE 'orders' GET [ 'RO' BIND RO 'price' GET 'RP' BIND 0 RP RP NIL? SELECT RO 'qty' GET MUL ] MAP G0A.SUM` — orders JSON text -> total qty*price over priced orders
  - `G0A.QTYBY` = `JSON-DECODE 'orders' GET 'QO' BIND QO [ 'sku' GET ] MAP QO [ 'qty' GET ] MAP G0A.SUMBY` — orders JSON text -> Record sku to total qty
  - `G0B.REVENUE` = `JSON-DECODE 'orders' GET [ G0B.ORDVAL ] MAP G0B.SUM` — orders JSON String -> total qty*price over priced orders
  - `G0B.QTYBYSKU` = `JSON-DECODE 'orders' GET 'OL' BIND OL [ 'sku' GET ] MAP OL [ 'qty' GET ] MAP G0B.GROUPSUM` — orders JSON String -> Record sku -> summed qty
  - `G0C.REVENUE` = `G0C.ORDERS [ 'price' HAS? ] FILTER [ G0C.LINE ] MAP G0C.SUM` — Total of qty times price over the priced orders, from the JSON text.
  - `G0C.SKUQTY` = `G0C.ORDERS 'LO' BIND LO [ 'sku' GET ] MAP LO [ 'qty' GET ] MAP G0C.GROUPSUM` — From the JSON text, a Record from each sku to its total qty.
  - `G0C.TAGCOUNTS` = `G0C.ORDERS [ 'tags' HAS? ] FILTER [ 'tags' GET UNIQUE ] MAP FLATTEN TALLY` — From the JSON text, a Record from each tag to the number of orders carrying it.
  - `G0D.ORDERS` = `JSON-DECODE 'orders' GET` — The orders array of a JSON order document, as a Vector of Records.
  - `G0D.REVENUE` = `G0D.ORDERS [ 'price' HAS? ] FILTER [ 'O' BIND O 'qty' GET O 'price' GET MUL ] MAP 0 [ ADD ] FOLD` — Sum of qty*price over the priced orders of a JSON order document.
  - `W2A.REVENUE` = `W2A.ORDERS [ 'price' GET NIL? NOT ] FILTER [ W2A.REV ] MAP W2A.SUM` — orders JSON String -> total qty*price over priced orders
  - `W2A.QTYBYSKU` = `W2A.ORDERS 'OS' BIND OS [ 'sku' GET ] MAP OS [ 'qty' GET ] MAP W2A.SUMBY` — orders JSON String -> Record sku to summed qty
  - `W2A.TAGCOUNTS` = `W2A.ORDERS [ W2A.TAGS UNIQUE ] MAP [ ] [ CONCAT ] FOLD TALLY` — orders JSON String -> Record tag to number of orders carrying it
  - `W2B.REVENUE` = `JSON-DECODE 'orders' GET [ W2B.LINE ] MAP W2B.SUM` — orders JSON String -> total of qty times price over priced orders
  - `W2D.REVENUE` = `JSON-DECODE 'orders' GET [ W2D.LINE ] MAP 0 [ ADD ] FOLD` — orders JSON String to the total of qty times price over the priced orders
  - `F1A.REVENUE` = `G0C.ORDERS [ F1A.LINE ] MAP G0A.SUM` — orders JSON -> total qty times price over priced orders
  - `F1A.QBS` = `G0C.ORDERS 'OS' BIND OS [ 'sku' GET ] MAP OS [ 'qty' GET ] MAP G0A.SUMBY` — orders JSON -> Record sku to summed qty
  - `F1B.REVENUE` = `G0C.ORDERS [ 'price' HAS? ] FILTER 'ORS' BIND ORS [ 'OO' BIND OO 'qty' GET OO 'price' GET MUL ] MAP G0A.SUM` — orders JSON text -> total of qty times price over priced orders
  - `F1D.REVENUE` = `G0C.ORDERS [ 'price' GET NIL? NOT ] FILTER [ 'RVO' BIND RVO 'qty' GET RVO 'price' GET MUL ] MAP G0A.SUM` — orders JSON text -> total of qty times price over the priced orders
  - `F1D.QTY-BY-SKU` = `G0C.ORDERS 'QBO' BIND QBO [ 'sku' GET ] MAP QBO [ 'qty' GET ] MAP G0A.SUMBY` — orders JSON text -> Record sku to sum of qty, first-appearance order
  - `F2A.REVENUE` = `G0C.ORDERS [ 'price' HAS? ] FILTER [ 'RO' BIND RO 'qty' GET RO 'price' GET MUL ] MAP G0A.SUM` — orders JSON -> total qty*price over priced orders
  - `F2A.UNPRICED` = `G0C.ORDERS [ 'price' HAS? NOT ] FILTER [ 'sku' GET ] MAP UNIQUE` — orders JSON -> distinct skus of orders without a price
  - `F2A.QTYBYSKU` = `G0C.ORDERS 'OQ' BIND OQ [ 'sku' GET ] MAP OQ [ 'qty' GET ] MAP G0A.SUMBY` — orders JSON -> Record sku to summed qty, first-appearance order
  - `F2A.TAGCOUNTS` = `G0C.ORDERS [ 'OT' BIND [ ] OT 'tags' GET 2 COLLECT 1 0 OT 'tags' HAS? SELECT GET UNIQUE ] MAP FLATTEN TALLY` — orders JSON -> Record tag to number of orders carrying it
  - `F2B.QTYBYSKU` = `G0C.ORDERS 'OS' BIND OS [ 'sku' GET ] MAP OS [ 'qty' GET ] MAP G0A.SUMBY` — orders JSON text -> Record sku to sum of qty
  - `F2D.REVENUE` = `G0C.ORDERS [ 'price' GET NIL? NOT ] FILTER [ 'ORD' BIND ORD 'qty' GET ORD 'price' GET MUL ] MAP G0A.SUM` — orders JSON text -> total of qty times price over the orders that have a price
  - `F2D.QTYBYSKU` = `G0C.ORDERS 'OS' BIND OS [ 'sku' GET ] MAP OS [ 'qty' GET ] MAP G0A.SUMBY` — orders JSON text -> Record sku to total qty, skus in first-appearance order
  - `F3A.REVENUE` = `G0C.ORDERS [ F3A.LINEREV ] MAP G0A.SUM` — orders JSON text -> total qty*price over priced orders
  - `F3A.QTYBY` = `G0C.ORDERS 'OS' BIND OS [ 'sku' GET ] MAP OS [ 'qty' GET ] MAP G0A.SUMBY` — orders JSON text -> Record sku to summed qty, first-appearance order
  - `F3B.QBYSKU` = `G0C.ORDERS 'KQ' BIND KQ [ 'sku' GET ] MAP KQ [ 'qty' GET ] MAP G0A.SUMBY` — orders JSON text -> Record sku to summed qty
  - `O2A.REVENUE` = `O2A.ORDERS [ O2A.LINEREV ] MAP G0A.SUM` — orders JSON String to the total of qty times price over priced orders
  - `O2A.SKUQTY` = `O2A.ORDERS [ 'OQ' BIND OQ 'sku' GET OQ 'qty' GET 2 COLLECT ] MAP O2A.BYKEY` — orders JSON String to a Record from each sku to its summed qty, in order of first appearance
  - `O2C.ORDERS` = `JSON-DECODE 'orders' GET` — JSON text on the stack to the Vector of order Records under the orders key
  - `O2D.ORDERS` = `JSON-DECODE 'orders' GET` — JSON String with an "orders" array to its Vector of order Records
  - `O2D.PRICED` = `O2D.ORDERS [ 'price' HAS? ] FILTER` — JSON orders String to the order Records that have a price
  - `O2D.REVENUE` = `O2D.PRICED [ 'O' BIND O 'qty' GET O 'price' GET MUL ] MAP G0A.SUM` — JSON orders String to the total of qty times price over the priced orders
  - `U2A.REVENUE` = `U2A.ORDERS [ 'price' HAS? ] FILTER [ 'ORD' BIND ORD 'qty' GET ORD 'price' GET MUL ] MAP G0A.SUM` — orders JSON String -> total qty*price over orders that have a price
  - `U2D.ORDERS` = `JSON-DECODE 'orders' GET` — JSON String with an orders array -> Vector of order Records
  - `U2D.REVENUE` = `U2D.ORDERS [ 'price' HAS? ] FILTER [ 'O' BIND O 'qty' GET O 'price' GET MUL ] MAP G0A.SUM` — JSON orders String -> total of qty times price over the orders that have a price
- **12 agents** (G0A, O2C, U2A, G0B, G0C, G0D, W2A, W2B, O2B, U2B, U2C, U2D; models opus, haiku, sonnet, fable), joined at d0a/d1/d0:
  - `G0A.SUMBY` = `GROUP 'GR' BIND GR KEYS GR VALUES [ G0A.SUM ] MAP RECORD` — keys values -> Record key to sum of its values, first-appearance order
  - `O2C.GROUPSUM` = `GROUP 'G' BIND G KEYS G VALUES [ G0A.SUM ] MAP RECORD` — keys Vector and values Vector to a Record from each key to the sum of its values, keys in first-appearance order
  - `U2A.GSUM` = `GROUP 'GSR' BIND GSR KEYS GSR VALUES [ G0A.SUM ] MAP RECORD` — keys Vector, values Vector -> Record from each distinct key to the sum of its values, first-appearance order
  - `G0B.GROUPSUM` = `GROUP 'GG' BIND GG KEYS GG VALUES [ G0B.SUM ] MAP RECORD` — keys-vector values-vector -> Record key -> sum of its values, first-appearance order
  - `G0C.GROUPSUM` = `GROUP 'LG' BIND LG KEYS LG VALUES [ G0C.SUM ] MAP RECORD` — Keys Vector and values Vector to a Record from each key to the sum of its values, keys in first-appearance order.
  - `G0D.SUMS-BY` = `GROUP 'G' BIND G KEYS G VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — keys values G0D.SUMS-BY leaves a Record from each key to the sum of its values, keys in first-appearance order.
  - `W2A.SUMBY` = `GROUP 'SG' BIND SG KEYS SG VALUES [ W2A.SUM ] MAP RECORD` — keys values -> Record from each distinct key to the sum of its values, first-appearance order
  - `W2B.GSUM` = `GROUP 'GG' BIND GG KEYS GG VALUES [ W2B.SUM ] MAP RECORD` — keys-Vector values-Vector -> Record from each key to the sum of its values, first-appearance order
  - `O2B.GSUM` = `'GV' BIND 'GK' BIND GK GV GROUP 'GG' BIND GG KEYS GG VALUES [ G0A.SUM ] MAP RECORD` — Vector of keys, Vector of numbers to Record key -> sum of its numbers, keys in first-appearance order
  - `U2B.SUMGRP` = `GROUP 'G' BIND G KEYS G VALUES [ G0A.SUM ] MAP RECORD` — keys values -> Record key -> sum of its values, first-appearance order
  - `U2C.SUMBY` = `GROUP 'G' BIND G KEYS G VALUES [ G0A.SUM ] MAP RECORD` — keys vector and values vector -> Record from each key to the sum of its values, keys in first-appearance order
  - `U2D.SUMBY` = `GROUP 'G' BIND G KEYS G VALUES [ G0A.SUM ] MAP RECORD` — keys values -> Record from each distinct key (first-appearance order) to the sum of its values
- **12 agents** (G0D, O2D, O2C, U2A, W2A, F1A, F1B, F3B, F3C, F3D, U2C, U2D; models fable, haiku, opus, sonnet), joined at d1/d0a/d0:
  - `G0D.DIVISORS` = `'K' BIND 1 K RANGE [ 'D' BIND K K D DIV FLOOR D MUL SUB 0 EQ ] FILTER` — Ascending Vector of the positive divisors of a positive integer.
  - `O2D.DIVISORS` = `'N' BIND 1 N RANGE [ 'D' BIND N D O2D.MOD 0 EQ ] FILTER` — positive integer to its positive divisors, ascending
  - `O2C.DIVISORS` = `'N' BIND 1 N RANGE [ 'D' BIND N D DIV FLOOR D MUL N EQ ] FILTER` — Vector of the positive divisors of the integer on the stack, ascending
  - `U2A.DIVS` = `'DN' BIND 1 DN RANGE [ 'DX' BIND DN DX DIV FLOOR DX MUL DN EQ ] FILTER` — positive integer -> Vector of its positive divisors, ascending
  - `W2A.DIVISORS` = `'DN' BIND 1 DN RANGE [ -1 POW DN MUL RATIO 1 GET 1 EQ ] FILTER` — positive integer n -> ascending Vector of its positive divisors
  - `F1A.DIVS` = `'DN' BIND 1 DN RANGE [ 'DD' BIND DN DD G0A.MOD 0 EQ ] FILTER` — positive divisors of a positive integer, ascending
  - `F1B.DIVS` = `'DN' BIND 1 DN RANGE [ 'DD' BIND DN DD G0A.MOD 0 EQ ] FILTER` — positive integer -> Vector of its positive divisors, ascending
  - `F3B.DIVS` = `'KE' BIND 1 KE RANGE [ 'KF' BIND KE KF G0A.MOD 0 EQ ] FILTER` — positive integer -> Vector of its divisors ascending
  - `F3C.DIVS` = `'N' BIND 1 N RANGE [ 'D' BIND N D G0A.MOD 0 EQ ] FILTER` — positive divisors of a positive integer, ascending
  - `F3D.DIVISORS` = `'DN' BIND 1 DN RANGE [ 'DK' BIND DN DK G0A.MOD 0 EQ ] FILTER` — positive integer -> Vector of its positive divisors, ascending
  - `U2C.DIVISORS` = `'N' BIND 1 N RANGE [ 'D' BIND N D DIV FLOOR D MUL N EQ ] FILTER` — ascending positive divisors of a positive integer (divisibility via floor(n/d)*d = n)
  - `U2D.DIVISORS` = `'K' BIND 1 K RANGE [ 'D' BIND K D DIV FLOOR D MUL K EQ ] FILTER` — positive integer -> its positive divisors ascending
- **10 agents** (G0A, W1A, W3A, W3D, F1A, F2B, F3A, O1B, U1A, U3A; models opus, fable, sonnet), joined at d1/d0a/d0:
  - `G0A.LEDGER` = `ZIP 'LZ' BIND LZ 0 GET LZ 1 GET G0A.SUMBY` — Vector of [name amount] pairs -> Record name to total
  - `W1A.BYNAME` = `ZIP 'Z' BIND Z 0 GET Z 1 GET GROUP 'G' BIND G KEYS G VALUES [ G0A.SUM ] MAP RECORD` — Vector of [ name amount ] -> Record name to summed amount, first-appearance order
  - `W3A.LEDGER` = `ZIP 'LZ' BIND LZ 0 GET LZ 1 GET GROUP 'LG' BIND LG KEYS LG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — Vector of [ key number ] pairs -> Record key -> sum, keys in first-appearance order
  - `W3D.LSUMS` = `ZIP 'LZ' BIND LZ 0 GET LZ 1 GET GROUP 'LG' BIND LG KEYS LG VALUES [ W2B.SUM ] MAP RECORD` — Vector of [ key amount ] pairs -> Record key -> sum of its amounts, keys in first-appearance order
  - `F1A.LBY` = `ZIP 'LZ' BIND LZ 0 GET LZ 1 GET G0A.SUMBY` — Vector of [name amount] pairs -> Record name to sum of amounts
  - `F2B.BYNAME` = `'LE' BIND LE F2B.NAMES LE F2B.AMOUNTS G0A.SUMBY` — ledger entries -> Record name to sum of amounts
  - `F3A.LEDGER` = `ZIP 'LZ' BIND LZ 0 GET LZ 1 GET G0A.SUMBY` — Vector of [ key amount ] pairs -> Record key to summed amount, first-appearance order
  - `O1B.LEDGER` = `ZIP 'LZ' BIND LZ 0 GET LZ 1 GET G0A.SUMBY` — Vector of [name amount] entries to Record name to summed amount, first-appearance order
  - `U1A.LEDGER` = `ZIP 'LZ' BIND LZ 0 GET LZ 1 GET G0A.SUMBY` — Vector of [ key amount ] pairs -> Record key to summed amount, first-appearance order
  - `U3A.BYNAME` = `ZIP 'LZ' BIND LZ 0 GET LZ 1 GET U2A.GSUM` — Vector of [ key amount ] pairs -> Record from each key to the sum of its amounts, first-appearance order
- **9 agents** (O1A, W1A, W3A, W3D, O1D, O3D, U1D, U3A, U3D; models opus, fable), joined at d1:
  - `O1A.RLE` = `[ [ 1/2 0 ] ] [ O1A.RSTEP ] FOLD 1 DROP` — Vector of integers to its run-length encoding [ [ value count ] ... ]
  - `W1A.RLE` = `[ [ 1/2 0 ] ] [ W1A.RLESTEP ] FOLD 1 DROP` — run-length encoding of an integer Vector as [ value count ] pairs
  - `W3A.RLE` = `[ [ 'x' 0 ] ] [ W3A.RLESTEP ] FOLD 1 DROP` — Vector -> Vector of [ value count ] per maximal run of equal adjacent values
  - `W3D.RLE` = `[ [ '#' 0 ] ] [ W3D.RLESTEP ] FOLD 1 DROP` — Vector -> run-length encoding as Vector of [ value count ] pairs
  - `O1D.RLE` = `[ [ 'x' 0 ] ] [ O1D.RLESTEP ] FOLD 1 DROP` — run-length encoding of a Vector: [ value count ] per maximal run of equal adjacent values
  - `O3D.RLE` = `'RV' BIND RV O3D.RUNIDS RV GROUP VALUES [ 'RG' BIND RG 0 GET RG LENGTH 2 COLLECT ] MAP` — run-length encoding of a Vector of numbers: [ value count ] per maximal run of equal adjacent values, in order
  - `U1D.RLE` = `[ [ '' 0 ] ] [ 'X' BIND 'A' BIND A -1 GET [ 'LV' 'LC' ] BIND A -1 DROP LV LC 1 ADD 2 COLLECT 1 COLLECT CONCAT A X 1 2 COLLECT 1 COLLECT CONCAT 2 COLLECT 0 1 LV X EQ SELECT GET ] FOLD 1 DROP` — Vector -> run-length encoding: Vector of [ value count ] pairs, one per maximal run of equal adjacent elements (works for numbers, Strings, Booleans)
  - `U3A.RLE` = `'RV' BIND RV 1 DROP RV 0 GET 1 2 COLLECT 1 COLLECT [ U3A.RLESTEP ] FOLD` — non-empty Vector -> run-length encoding as Vector of [ value count ]
  - `U3D.RLE` = `[ [ '#' 0 ] ] [ 'X' BIND 'ACC' BIND ACC -1 GET [ 'LV' 'LC' ] BIND ACC -1 DROP X LC 1 ADD 2 COLLECT 1 COLLECT CONCAT ACC X 1 2 COLLECT 1 COLLECT CONCAT 2 COLLECT 0 1 LV X EQ SELECT GET ] FOLD 1 DROP` — Vector -> run-length encoding as a Vector of [ value count ] pairs, one per maximal run of equal adjacent values
- **8 agents** (G0A, W2A, W2B, G0B, G0C, G0D, W2C, W2D; models opus, sonnet, haiku, fable), joined at d0a/d0/d1:
  - `G0A.MEAN` = `'MV' BIND MV G0A.SUM MV LENGTH DIV` — arithmetic mean of a Vector
  - `W2A.MEAN` = `'MN' BIND MN W2A.SUM MN LENGTH DIV` — arithmetic mean of a non-empty numeric Vector
  - `W2B.MEAN` = `'SX' BIND SX W2B.SUM SX LENGTH DIV` — arithmetic mean of a numeric Vector
  - `G0B.MEAN` = `'MV' BIND MV G0B.SUM MV LENGTH DIV` — arithmetic mean of a Vector
  - `G0C.MEAN` = `'LV' BIND LV G0C.SUM LV LENGTH DIV` — Arithmetic mean of a non-empty numeric Vector.
  - `G0D.MEAN` = `'V' BIND V G0D.SUM V LENGTH DIV` — Arithmetic mean of a non-empty Vector of numbers.
  - `W2C.MEAN` = `'V' BIND V 0 [ ADD ] FOLD V LENGTH DIV` — Arithmetic mean of a numeric Vector.
  - `W2D.MEAN` = `'MV' BIND MV 0 [ ADD ] FOLD MV LENGTH DIV` — arithmetic mean of a non-empty numeric Vector, as a bare scalar
  - `W3B.MEAN` = `'SX' BIND SX W3B.SUM SX LENGTH DIV` — mean of a numeric Vector
- **8 agents** (G0C, G0B, W3C, F1D, F2A, F2D, F3C, F3D; models haiku, sonnet, fable, opus), joined at d0a/d0:
  - `G0C.MAXV` = `'LX' BIND LX LX 0 GET [ MAX ] FOLD` — Largest element of a non-empty numeric Vector.
  - `G0B.MAXOF` = `'XV' BIND XV XV 0 GET [ MAX ] FOLD` — largest element of a non-empty Vector
  - `W3C.MAXV` = `'MV' BIND MV MV 0 GET [ MAX ] FOLD` — Maximum of a non-empty numeric Vector, seeded with its first element so negatives work.
  - `F1D.VMAX` = `'VXV' BIND VXV VXV 0 GET [ MAX ] FOLD` — largest element of a non-empty Vector
  - `F2A.VMAX` = `'VX' BIND VX VX 0 GET [ MAX ] FOLD` — largest element of a non-empty Vector
  - `F2D.VMAX` = `'XV' BIND XV XV 0 GET [ MAX ] FOLD` — largest element of a non-empty Vector of numbers (or Strings)
  - `F3C.MAXV` = `'V' BIND V V 0 GET [ MAX ] FOLD` — largest element of a non-empty Vector of numbers
  - `F3D.VMAX` = `'VX' BIND VX VX 0 GET [ MAX ] FOLD` — non-empty Vector -> its largest element
- **8 agents** (W1D, O1D, W1A, W3B, W3D, U1B, U1D, U3D; models fable, opus, sonnet), joined at d0/d1/d0a:
  - `W1D.PRESENT` = `[ NIL? NOT ] FILTER` — the non-NIL elements of a Vector, in order
  - `O1D.PRESENT` = `[ NIL? NOT ] FILTER` — the non-NIL elements of a Vector, in order
  - `W1A.FILLED` = `'V' BIND V V [ NIL? NOT ] FILTER 0 GET [ 'X' BIND 'A' BIND A X X NIL? SELECT ] SCAN` — forward-fill NILs; leading NILs take the first present value
  - `W3B.PRESENT` = `[ NIL? NOT ] FILTER` — keep the non-NIL elements
  - `W3B.FILLED` = `'FV' BIND FV FV W3B.PRESENT 0 GET [ W3B.PICK ] SCAN` — forward-fill NIL readings, leading ones take the first present reading
  - `W3D.FILL` = `'FV' BIND FV FV [ NIL? NOT ] FILTER 0 GET [ 'FE' BIND 'FA' BIND FA FE FE NIL? SELECT ] SCAN` — Vector with NILs -> forward-filled; leading NILs take the first present value
  - `O1D.FILLED` = `'V' BIND V V O1D.PRESENT 0 GET [ 'E' BIND 'A' BIND A E E NIL? SELECT ] SCAN` — forward-fill NILs with the nearest earlier present value; leading NILs take the first present value
  - `O3D.DEDUPE` = `O3D.RLE [ 0 GET ] MAP` — Vector of numbers with each run of equal adjacent values collapsed to one value
  - `O3D.FILL` = `'SV' BIND SV SV O1D.PRESENT 0 GET [ 'SE' BIND 'SA' BIND SA SE SE NIL? SELECT ] SCAN` — forward fill: each NIL replaced by the nearest present element before it, leading NILs by the first present element
  - `U1B.FILLED` = `'SV' BIND SV SV [ NIL? NOT ] FILTER 0 GET [ U1B.FILL2 ] SCAN` — readings -> forward-filled readings, leading gaps take the first present reading
  - `U1D.PRESENT` = `[ NIL? NOT ] FILTER` — Vector -> only its non-NIL elements, in order
  - `U3D.PRESENT` = `[ NIL? NOT ] FILTER` — Vector -> its non-NIL elements, in order
- **8 agents** (G0A, W2A, W2D, F1A, F1B, F2C, F3A, F3C; models opus, fable, sonnet, haiku), joined at d0a/d1/d0:
  - `G0A.CSTEP` = `'CX' BIND CX 2 DIV CX 3 MUL 1 ADD CX 2 G0A.MOD 0 EQ SELECT 'CY' BIND 1 CY CX 1 EQ SELECT` — one Collatz step, with 1 fixed at 1
  - `W2A.CSTEP` = `'CV' BIND CV 2 DIV CV 3 MUL 1 ADD CV 2 G0A.MOD 0 EQ SELECT 'CW' BIND 1 CW CV 1 EQ SELECT` — one Collatz step, with 1 staying at 1
  - `W2D.CSTEP` = `'CX' BIND 1 CX 2 DIV CX 3 MUL 1 ADD CX 2 G0A.MOD 0 EQ SELECT CX 1 EQ SELECT` — one Collatz step: even halves, odd becomes 3x+1, 1 stays 1
  - `F1A.CNEXT` = `'CX' BIND CX 2 DIV CX 3 MUL 1 ADD CX 2 G0A.MOD 0 EQ SELECT 'CY' BIND 1 CY CX 1 EQ SELECT` — one Collatz step, 1 stays 1
  - `F1B.CSTEP` = `'CX' BIND CX 2 DIV CX 3 MUL 1 ADD CX 2 G0A.MOD 0 EQ SELECT 'CY' BIND 1 CY CX 1 EQ SELECT` — one Collatz step, with 1 staying at 1
  - `F2C.CF` = `'CV' BIND CV CV 2 DIV CV 3 MUL 1 ADD CV 2 G0A.MOD 0 EQ SELECT CV 1 EQ SELECT` — one Collatz step, with 1 fixed at 1
  - `F3A.CSTEP` = `'CV' BIND 1 CV 2 DIV CV 3 MUL 1 ADD CV 2 G0A.MOD 0 EQ SELECT CV 1 EQ SELECT` — one Collatz step: n even -> n/2, odd -> 3n+1, 1 stays 1
  - `F3C.STEP` = `'X' BIND X 2 DIV X 3 MUL 1 ADD X 2 G0A.MOD 0 EQ SELECT 'Y' BIND 1 Y X 1 EQ SELECT` — one Collatz step (x/2 if even, 3x+1 if odd), with 1 mapped to 1
- **8 agents** (G0D, W1C, W1D, W2A, W3B, F2A, F3C, F3D; models fable, haiku, opus, sonnet), joined at d0/d0a:
  - `G0D.PAIRS` = `'R' BIND R KEYS R VALUES 2 COLLECT ZIP` — A Record as a Vector of [ key value ] pairs in key order.
  - `W1C.RECPAIRS` = `'R' BIND R KEYS R VALUES 2 COLLECT ZIP` — Record to the Vector of [key value] pairs in key order.
  - `W1D.PAIRS` = `'R' BIND R KEYS R VALUES 2 COLLECT ZIP` — a Record as a Vector of [ key value ] pairs in key order
  - `W2A.PAIRS` = `'PR' BIND PR KEYS PR VALUES 2 COLLECT ZIP` — non-empty Record -> Vector of [ key value ] pairs in key order
  - `W3B.PAIRS` = `'PR' BIND PR KEYS PR VALUES 2 COLLECT ZIP` — Record -> Vector of [ key value ] pairs
  - `F2A.ENTRIES` = `'EN' BIND EN KEYS EN VALUES 2 COLLECT ZIP` — Record -> Vector of [ key value ] pairs in key order
  - `F3C.PAIRS` = `'R' BIND R KEYS R VALUES 2 COLLECT ZIP` — Record to a Vector of [key value] pairs in Record order
  - `F3D.PAIRS` = `'PR' BIND PR KEYS PR VALUES 2 COLLECT ZIP` — Record -> Vector of [ key value ] pairs in key order
- **8 agents** (W1A, W2A, W3B, F1D, F2A, O1D, O3D, U1B; models opus, sonnet, fable), joined at d0a/d1/d0:
  - `W1A.MOVING3` = `'V' BIND V -2 DROP V 1 DROP -1 DROP ADD V 2 DROP ADD 3 DIV` — means of every three consecutive numbers (length at least 3)
  - `W2A.MOVING3` = `'MV' BIND MV -2 DROP MV 1 DROP -1 DROP ADD MV 2 DROP ADD 3 DIV` — Vector of means of every three consecutive numbers
  - `W3B.MOVING3` = `'MV3' BIND MV3 -2 DROP MV3 1 DROP -1 DROP ADD MV3 2 DROP ADD 3 DIV` — means of every three consecutive numbers
  - `F1D.MOVING3` = `'M3V' BIND M3V -2 DROP M3V 1 DROP -1 DROP ADD M3V 2 DROP ADD 3 DIV` — means of every three consecutive numbers (input of at least three)
  - `F2A.MOVING3` = `'SW' BIND SW -2 DROP SW 1 DROP -1 DROP ADD SW 2 DROP ADD 3 DIV` — means of every three consecutive numbers
  - `O1D.MOVING3` = `'V' BIND V -2 DROP V 1 DROP -1 DROP V 2 DROP ADD ADD 3 DIV` — means of every three consecutive numbers (two fewer elements than the input)
  - `O3D.MOVING3` = `'MX' BIND MX -2 DROP MX 1 DROP -1 DROP ADD MX 2 DROP ADD 3 DIV` — means of every three consecutive numbers of a Vector of at least three (two fewer elements)
  - `U1B.MOVING3` = `'MX' BIND MX -2 DROP MX 1 DROP -1 DROP ADD MX 2 DROP ADD 3 DIV` — Vector of numbers -> means of every three consecutive numbers
- **7 agents** (G0A, O2B, G0B, G0C, G0D, O2D, U2B; models opus, sonnet, haiku, fable), joined at d0/d0a:
  - `G0A.MOD` = `'MB' BIND 'MA' BIND MA MA MB DIV FLOOR MB MUL SUB` — a b -> a mod b (floored), lifts over Vectors
  - `O2B.MOD` = `'MB' BIND 'MA' BIND MA MA MB DIV FLOOR MB MUL SUB` — a b to a mod b (a - b*floor(a/b)); lifts over a Vector a
  - `G0B.MOD` = `'MB' BIND 'MA' BIND MA MA MB DIV FLOOR MB MUL SUB` — a b -> a mod b (floor remainder)
  - `G0C.MOD` = `'LB' BIND 'LA' BIND LA LA LB DIV FLOOR LB MUL SUB` — Remainder a mod b for a stack pair [a b] with a below b.
  - `G0D.MOD` = `'B' BIND 'A' BIND A A B DIV FLOOR B MUL SUB` — a b G0D.MOD leaves a - b*floor(a/b), the remainder.
  - `O2D.MOD` = `'B' BIND 'A' BIND A A B DIV FLOOR B MUL SUB` — a b to the remainder a - b*floor(a/b)
  - `U2B.MOD` = `'B' BIND 'A' BIND A A B DIV FLOOR B MUL SUB` — a b -> a mod b (floor remainder), lifts over vectors
- **7 agents** (G0A, W2B, O2D, U2D, G0B, W2D, O2B; models opus, sonnet, fable), joined at d0a/d0/d1:
  - `G0A.DEVS` = `'DV' BIND DV DV G0A.MEAN SUB` — each number minus the mean
  - `W2B.DEV` = `'DX' BIND DX DX W2B.MEAN SUB` — numeric Vector -> each number minus the mean
  - `O2D.DEVS` = `'V' BIND V V G0A.MEAN SUB` — each number minus the mean of the Vector, in order
  - `U2D.DEVS` = `'V' BIND V V G0A.MEAN SUB` — Vector of numbers -> each number minus the mean, in order
  - `G0B.DEV` = `'DV' BIND DV DV G0B.MEAN SUB` — each number minus the mean
  - `W2D.DEVS` = `'DV' BIND DV DV W2D.MEAN SUB` — each number minus the mean of the Vector, in order
  - `O2B.DEV` = `'DV' BIND DV DV G0A.MEAN SUB` — Vector of numbers to each number minus the mean
- **7 agents** (G0D, O2C, O2D, W2C, F3D, U2A, U2D; models fable, haiku, opus), joined at d1/d0a:
  - `G0D.IS-PRIME` = `G0D.DIVISORS LENGTH 2 EQ` — Whether a positive integer is prime.
  - `O2C.ISPRIME` = `O2C.DIVISORS LENGTH 2 EQ` — TRUE when the integer on the stack has exactly two divisors
  - `O2D.ISPRIME` = `O2D.DIVISORS LENGTH 2 EQ` — TRUE when a positive integer is prime
  - `W2C.ISPRIME` = `'K' BIND K 1 K RANGE G0A.MOD [ 0 EQ ] FILTER LENGTH 2 EQ` — TRUE when a positive integer K has exactly two divisors in 1..K (K mod each d in 1..K, count zeros).
  - `F3D.ISPRIME` = `F3D.NDIV 2 EQ` — positive integer -> TRUE when it is prime
  - `U2A.ISPRIME` = `U2A.DIVS LENGTH 2 EQ` — positive integer -> TRUE when it is prime
  - `U2D.ISPRIME` = `U2D.DIVISORS LENGTH 2 EQ` — positive integer -> TRUE when prime (1 is not)
- **7 agents** (O1B, W1B, W3B, W3C, O3D, U1B, U3C; models sonnet, haiku, fable), joined at d1/d0a:
  - `O1B.MISSING` = `[ NIL? 'MX' BIND 1 0 MX SELECT ] MAP` — Vector with NILs to Vector of 1 for each NIL lane and 0 for each present lane
  - `W1B.MISSING` = `[ 'X' BIND 1 0 X NIL? SELECT ] MAP` — Vector of readings -> Vector of 1 (missing) / 0 (present)
  - `W3B.MISS01` = `[ NIL? ] MAP 'MB' BIND 1 0 MB SELECT` — Vector -> 1 where NIL, else 0
  - `W3C.GAPS` = `'X' BIND 0 X LENGTH 1 SUB RANGE 'I' BIND I -1 X [ NIL? NOT ] MAP SELECT -1 [ MAX ] SCAN 'CM' BIND I CM SUB` — Readings Vector to, per position, the distance back to the last present reading (1 at the start of each gap, 0 on present readings).
  - `O3D.MISS` = `[ 'ME' BIND 1 0 ME NIL? SELECT ] MAP` — Vector to a same-length Vector of 1 per NIL element and 0 per present element
  - `U1B.MISSMASK` = `[ U1B.MISS1 ] MAP` — Vector of readings -> Vector of 1 (missing) or 0 (present)
  - `U3C.MISS` = `[ NIL? ] MAP 'MT' BIND 1 0 MT SELECT` — Vector of readings -> numeric flags, 1 where the reading is missing and 0 where present
- **7 agents** (G0B, W1A, W1D, W3A, W3D, U1A, U3B; models sonnet, opus, fable), joined at d1/d0a/d0:
  - `G0B.MODEOF` = `TALLY G0B.TOPKEY` — most frequent element of a Vector, first-appearing on ties
  - `W1A.MOSTFREQ` = `TALLY 'T' BIND T KEYS T VALUES W1A.ARGMAX GET` — most frequent element of a non-empty Vector; ties go to the earliest first occurrence
  - `W1D.MODEFIRST` = `TALLY 'T' BIND T KEYS T VALUES W1D.ARGMAX GET` — most frequent element of a Vector; ties go to the one appearing first
  - `W3A.MODEFIRST` = `TALLY 'MT' BIND MT KEYS MT VALUES W3A.ARGMAX GET` — Vector -> most frequent element, earliest first appearance on ties
  - `W3D.TOPKEY` = `TALLY 'TK' BIND TK KEYS TK VALUES W3D.ARGMAX GET` — Vector -> most frequent element, the one appearing first on ties
  - `U1A.MODEFIRST` = `TALLY 'MT' BIND MT KEYS MT VALUES U1A.ARGMAX GET` — Vector -> its most frequent element, the first-appearing one on ties
  - `U3B.TOP` = `TALLY 'TT' BIND TT KEYS TT VALUES -1 MUL ORDER 0 GET GET` — Vector -> its most frequent element, the earliest-appearing on ties
- **7 agents** (W1D, O3B, O3D, U1A, U1D, U3A, U3D; models fable, sonnet, opus), joined at d1/d0a/d0:
  - `W1D.BDELTA` = `CHARS [ 'C' BIND 1 -1 '([' C SEARCH NIL? NOT SELECT ] MAP` — bracket String to a Vector of +1 per opener and -1 per closer
  - `O3B.BVALS` = `'[' '(' REPLACE ']' ')' REPLACE CHARS [ 'BC' BIND 1 -1 BC '(' EQ SELECT ] MAP` — bracket String to Vector of +1 for each opener and -1 for each closer
  - `O3D.SIGNS` = `CHARS [ 'BC' BIND -1 1 '([' BC SEARCH NIL? SELECT ] MAP` — bracket String to a Vector of +1 per opener ( [ and -1 per closer ) ]
  - `U1A.BSTEPS` = `CHARS [ 'BC' BIND 1 -1 BC '(' EQ NOT BC '[' EQ NOT AND NOT SELECT ] MAP` — bracket String -> Vector of +1 per opening and -1 per closing bracket
  - `U1D.BSIGNS` = `CHARS [ 'C' BIND -1 1 '([' C SEARCH NIL? SELECT ] MAP` — String of brackets ( ) [ ] -> Vector of 1 per opening and -1 per closing bracket
  - `U3A.DELTAS` = `CHARS [ 'BC' BIND -1 1 '([' BC SEARCH NIL? SELECT ] MAP` — bracket String -> Vector of +1 per opening and -1 per closing bracket
  - `U3D.BSIGNS` = `CHARS [ 'C' BIND 1 -1 '([' C SEARCH NIL? NOT SELECT ] MAP` — bracket String -> Vector with 1 for each opening bracket ( [ and -1 for each closing one
- **6 agents** (G0A, W2B, G0B, G0C, G0D, W2A; models opus, sonnet, haiku, fable), joined at d0:
  - `G0A.SUM` = `0 [ ADD ] FOLD` — sum of a Vector of numbers (0 for empty)
  - `W2B.SUM` = `0 [ ADD ] FOLD` — sum of a numeric Vector, bare scalar
  - `G0B.SUM` = `0 [ ADD ] FOLD` — sum of a Vector of numbers (0 when empty)
  - `G0C.SUM` = `0 [ ADD ] FOLD` — Sum of a numeric Vector; 0 for an empty one.
  - `G0D.SUM` = `0 [ ADD ] FOLD` — Sum of a Vector of numbers (0 for an empty Vector).
  - `W2A.SUM` = `0 [ ADD ] FOLD` — sum of a numeric Vector (0 for empty)
  - `W3B.SUM` = `0 [ ADD ] FOLD` — sum of a numeric Vector
- **6 agents** (G0C, W3C, F1D, F2A, F2D, F3D; models haiku, fable, opus), joined at d0a/d0:
  - `G0C.MINV` = `'LX' BIND LX LX 0 GET [ MIN ] FOLD` — Smallest element of a non-empty numeric Vector.
  - `W3C.MINV` = `'MV' BIND MV MV 0 GET [ MIN ] FOLD` — Minimum of a non-empty numeric Vector, seeded with its first element.
  - `F1D.VMIN` = `'VMV' BIND VMV VMV 0 GET [ MIN ] FOLD` — smallest element of a non-empty Vector
  - `F2A.VMIN` = `'VN' BIND VN VN 0 GET [ MIN ] FOLD` — smallest element of a non-empty Vector
  - `F2D.VMIN` = `'NV' BIND NV NV 0 GET [ MIN ] FOLD` — smallest element of a non-empty Vector of numbers (or Strings)
  - `F3D.VMIN` = `'VN' BIND VN VN 0 GET [ MIN ] FOLD` — non-empty Vector -> its smallest element
- **6 agents** (O1B, W1B, W3B, O3B, U1B, U3B; models sonnet), joined at d0:
  - `O1B.BAL` = `O1B.REDALL '' EQ` — TRUE when a bracket String (at most 12 chars) is balanced
  - `W1B.ISBAL` = `W1B.REDUCE '' EQ` — bracket String -> TRUE when properly balanced
  - `W3B.BAL` = `W3B.REDUCE '' EQ` — TRUE when a bracket String is balanced
  - `O3B.BAL` = `O3B.RED '' EQ` — TRUE when a bracket String is balanced (reduces to empty)
  - `U1B.ISBAL` = `U1B.REDUCE '' EQ` — bracket string -> TRUE when properly nested
  - `U3B.BAL` = `U3B.RED '' EQ` — String of brackets -> TRUE when properly balanced
- **6 agents** (O1D, W1A, W3B, W3D, O3D, U1B; models fable, opus, sonnet), joined at d1:
  - `O1D.GAPS` = `O1D.NILFLAGS 'F' BIND F FALSE 1 COLLECT F -1 DROP CONCAT NOT AND G0B.COUNT` — number of runs of consecutive NILs in a Vector
  - `W1A.GAPS` = `FALSE 0 2 COLLECT [ 'X' BIND 'A' BIND X NIL? 'N' BIND N A 1 GET 1 ADD A 1 GET N A 0 GET NOT AND SELECT 2 COLLECT ] FOLD 1 GET` — number of runs of consecutive NILs in a Vector
  - `W1A.LONGGAP` = `0 [ 'X' BIND 1 ADD 0 X NIL? SELECT ] SCAN 0 [ MAX ] FOLD` — length of the longest run of consecutive NILs (0 if none)
  - `W3B.GAPCOUNT` = `W3B.MISS01 'GM' BIND 0 1 COLLECT GM -1 DROP CONCAT 'GP' BIND GM GP SUB 0 GT 'GS' BIND 1 0 GS SELECT W3B.SUM` — number of runs of consecutive NIL readings
  - `W3B.LONGGAP` = `W3B.MISS01 0 [ W3B.RUNSTEP ] SCAN 0 [ MAX ] FOLD` — length of the longest run of NIL readings
  - `W3D.NTRUE` = `[ TRUE EQ ] FILTER LENGTH` — Vector of truths -> how many are TRUE
  - `O1D.LONGEST-GAP` = `O1D.NILFLAGS 0 [ 'T' BIND 'A' BIND A 1 ADD 0 T SELECT ] SCAN 0 1 COLLECT CONCAT G0D.MAXV` — length of the longest run of consecutive NILs (0 when none)
  - `O3D.GAPS` = `O3D.MISS 'MM' BIND MM 0 1 COLLECT MM -1 DROP CONCAT SUB [ 1 EQ ] FILTER LENGTH` — number of runs of consecutive NIL elements in a Vector
  - `O3D.LONGESTGAP` = `O3D.MISS 0 [ 'MF' BIND 1 ADD MF MUL ] SCAN G0D.MAXV` — length of the longest run of consecutive NIL elements in a non-empty Vector (0 when none is NIL)
  - `U1B.GAPCOUNT` = `U1B.MISSMASK 'GM' BIND GM [ 0 ] CONCAT [ 0 ] GM CONCAT SUB 0 MAX G0A.SUM` — readings -> number of runs of consecutive NILs
  - `U1B.LONGGAP` = `U1B.MISSMASK 0 [ U1B.RUNSTEP ] SCAN 0 [ MAX ] FOLD` — readings -> length of the longest run of consecutive NILs
- **6 agents** (O2C, F1C, F2C, U2A, U2C, U2D; models haiku, opus, fable), joined at d1/d0a/d0:
  - `O2C.REVENUE` = `[ O2C.LINE ] MAP G0A.SUM` — total of qty times price over a Vector of order Records, unpriced orders contributing 0
  - `F1C.QTYBYSKU` = `'O' BIND O [ 'sku' GET ] MAP O [ 'qty' GET ] MAP G0A.SUMBY` — orders Vector -> Record from each sku to the sum of its qty, in first-appearance order
  - `F2C.QTYREC` = `'OV' BIND OV [ 'sku' GET ] MAP OV [ 'qty' GET ] MAP G0A.SUMBY` — Vector of orders -> Record from sku to total qty, first-appearance order
  - `O2C.QTYSKU` = `'OS' BIND OS [ 'sku' GET ] MAP OS [ 'qty' GET ] MAP O2C.GROUPSUM` — Vector of order Records to a Record from sku to the sum of its qty
  - `U2A.QTYBY` = `'QO' BIND QO [ 'sku' GET ] MAP QO [ 'qty' GET ] MAP U2A.GSUM` — Vector of order Records -> Record from each sku to its summed qty
  - `U2C.QTYREC` = `'OO' BIND OO [ 'sku' GET ] MAP OO [ 'qty' GET ] MAP U2C.SUMBY` — orders vector -> Record from each sku to the sum of its qty
  - `U2D.QTYS` = `'OS' BIND OS [ 'sku' GET ] MAP OS [ 'qty' GET ] MAP U2D.SUMBY` — Vector of order Records -> Record sku -> summed qty, skus in first-appearance order
- **6 agents** (W3B, F1D, F2A, O1D, O3D, U1B; models sonnet, fable, opus), joined at d0a:
  - `W3B.RUNMAX` = `'RM' BIND RM RM 0 GET [ MAX ] SCAN` — running maximum
  - `F1D.RUNMAX` = `'RMV' BIND RMV RMV 0 GET [ MAX ] SCAN` — running maximum of a non-empty Vector, same length as the input
  - `F2A.RUNMAX` = `'RX' BIND RX RX 0 GET [ MAX ] SCAN` — running maximum of a non-empty Vector
  - `O1D.RUNMAX` = `'V' BIND V V 0 GET [ MAX ] SCAN` — running maximum of a non-empty Vector of numbers
  - `O3D.RUNMAX` = `'MX' BIND MX MX 0 GET [ MAX ] SCAN` — running maximum of a non-empty Vector of numbers, same length
  - `U1B.RUNMAX` = `'RX2' BIND RX2 RX2 0 GET [ MAX ] SCAN` — Vector of numbers -> running maximum
- **6 agents** (F1D, F2B, F3A, U1A, U2B, U2D; models fable, sonnet, opus), joined at d0:
  - `F1D.WHERE` = `2 COLLECT ZIP [ 1 GET ] FILTER [ 0 GET ] MAP` — values mask -> the elements of values whose aligned mask lane is TRUE, in order
  - `F2B.WHERE` = `2 COLLECT ZIP [ 1 GET ] FILTER [ 0 GET ] MAP` — vector mask -> the elements of vector whose mask lane is TRUE, in order
  - `F3A.WHERE` = `2 COLLECT ZIP [ 1 GET ] FILTER [ 0 GET ] MAP` — items mask -> the items whose mask lane is TRUE, in order (items non-empty)
  - `U1A.MASKED` = `2 COLLECT ZIP [ 1 GET ] FILTER [ 0 GET ] MAP` — values mask -> the values whose mask lane is TRUE, in order
  - `U2B.PICK` = `2 COLLECT ZIP [ 1 GET ] FILTER [ 0 GET ] MAP` — values truths -> the values whose truth is TRUE
  - `U2D.WHERE` = `2 COLLECT ZIP [ 1 GET ] FILTER [ 0 GET ] MAP` — values truths (equal length) -> the values at TRUE positions, in order
- **5 agents** (W1C, O1A, O1D, W3A, U1C; models haiku, opus, fable), joined at d1:
  - `W1C.CODE` = `CHARS 'V' BIND [ '(' '[' ')' ']' ] [ 1 2 -1 -2 ] RECORD V GET` — Bracket String to the Vector of codes: ( is 1, [ is 2, ) is -1, ] is -2.
  - `O1A.DELTAS` = `CHARS [ O1A.DELTA1 ] MAP` — bracket String to its Vector of +1/-1 steps (empty for '')
  - `O1D.BDELTAS` = `CHARS [ '(' '[' ')' ']' ] [ 1 1 -1 -1 ] RECORD 'BR' BIND [ 'BC' BIND BR BC GET ] MAP` — bracket String to a Vector of +1 per opener and -1 per closer
  - `W1C.DELTA` = `CHARS 'V' BIND [ '(' '[' ')' ']' ] [ 1 1 -1 -1 ] RECORD V GET` — Bracket String to the Vector of +1 for each opener and -1 for each closer.
  - `W3A.BRCODE` = `CHARS 'BC' BIND [ '(' '[' ')' ']' ] [ 1 2 -1 -2 ] RECORD 'BR' BIND BC [ 'CH' BIND BR CH GET ] MAP` — bracket String -> Vector of codes: ( 1, [ 2, ) -1, ] -2
  - `U1C.BDELTA` = `'INP' BIND [ '(' ')' '[' ']' ] [ 1 -1 1 -1 ] RECORD 'TBL' BIND INP CHARS [ 'CH' BIND TBL CH GET ] MAP` — bracket string -> vector of +1 for an opener and -1 for a closer, per character
- **5 agents** (G0D, O2D, F1A, F1D, F2A; models fable, opus), joined at d0/d1/d0a:
  - `G0D.MINV` = `SORT 0 GET` — Smallest element of a Vector of numbers or Strings.
  - `O2D.MINV` = `SORT 0 GET` — smallest element of a non-empty Vector of numbers or Strings
  - `F1A.MODE` = `TALLY 'MT' BIND MT KEYS 'MK' BIND MK ORDER 'MP' BIND MK MP GET MT VALUES MP GET F1A.ARGMAX GET` — most frequent element, smallest on ties
  - `F1D.MODE` = `SORT TALLY F1D.TOPKEY` — most frequent number of a Vector; the smallest when several tie
  - `F2A.MODE` = `SORT TALLY F2A.TOPKEY` — most frequent number, smallest on a tie
- **5 agents** (O1A, W1C, W3A, W3B, O3A; models opus, haiku, sonnet), joined at d0a/d1:
  - `O1A.FSTEP` = `'FX' BIND 'FA' BIND FA FX FX NIL? SELECT` — SCAN step for forward fill: the element, or the accumulator when the element is NIL
  - `W1C.FSTEP` = `'X' BIND 'A' BIND A X X NIL? SELECT` — SCAN step that carries the last present reading forward over a missing one.
  - `W3A.FFSTEP` = `'FE' BIND 'FA' BIND FE FA FE NIL? NOT SELECT` — SCAN step for forward fill: the element if present, else the accumulator
  - `W3B.PICK` = `'PE' BIND 'PA' BIND PA PE PE NIL? SELECT` — acc elem -> elem, or acc when elem is NIL
  - `O3A.FFSTEP` = `'FE' BIND 'FA' BIND FA FE FE NIL? SELECT` — SCAN step: the element, or the accumulator when the element is NIL (forward fill)
- **5 agents** (O1B, W3B, W3D, O3D, U1B; models sonnet, fable), joined at d1:
  - `O1B.BALPRE` = `'BS' BIND 0 BS CHARS LENGTH RANGE 'BR' BIND BR 0 BR [ 'BK' BIND BS CHARS BK TAKE JOIN O1B.BAL ] MAP SELECT 0 [ MAX ] FOLD` — length of the longest balanced prefix of a bracket String
  - `W3B.DEPTH` = `CHARS [ W3B.SGN ] MAP 0 [ ADD ] SCAN 0 [ MAX ] FOLD` — greatest nesting depth of a bracket String
  - `W3B.BPREFIX` = `CHARS 'PV' BIND 0 PV LENGTH RANGE [ 'PK' BIND PK 0 PV PK TAKE JOIN W3B.BAL SELECT ] MAP 0 [ MAX ] FOLD` — length of the longest balanced prefix of a bracket String
  - `W3D.BPREFIX` = `CHARS 'CS' BIND 0 CS LENGTH RANGE 'KS' BIND KS KS 0 MUL KS [ 'K' BIND CS K TAKE JOIN W3D.BRED6 '' EQ ] MAP SELECT 0 [ MAX ] FOLD` — bracket String -> length of its longest balanced prefix (0 when only the empty one)
  - `O3D.DEPTH` = `O3D.SIGNS 0 [ ADD ] SCAN 0 1 COLLECT CONCAT G0D.MAXV` — greatest nesting depth of a bracket String: max of the running open count and 0
  - `O3D.BALPREFIX` = `CHARS '' [ 2 COLLECT JOIN ] SCAN [ O3D.REDUCE '' EQ ] FILTER [ CHARS LENGTH ] MAP 0 1 COLLECT CONCAT G0D.MAXV` — length of the longest balanced prefix of a bracket String (0 when only the empty prefix is)
  - `U1B.DEPTH` = `CHARS [ U1B.DELTA ] MAP 0 [ ADD 0 MAX ] SCAN 0 [ MAX ] FOLD` — bracket string -> greatest number of brackets open at once
  - `U1B.BPREF` = `CHARS '' [ 2 COLLECT JOIN ] SCAN 'PS' BIND [ '' ] PS CONCAT 'PP' BIND PP [ CHARS LENGTH ] MAP 0 PP [ U1B.ISBAL ] MAP SELECT 0 [ MAX ] FOLD` — bracket string -> length of its longest balanced prefix
- **5 agents** (O2D, F1D, F3D, U1D, U3D; models fable), joined at d1/d0a:
  - `O2D.BYKEY` = `ZIP [ 'N' 'A' ] BIND N A GROUP 'G' BIND G KEYS G VALUES [ G0A.SUM ] MAP RECORD` — Vector of [ key amount ] pairs to a Record from each key to the sum of its amounts, keys in order of first appearance
  - `F1D.LEDGER-SUMS` = `ZIP [ 'LGN' 'LGA' ] BIND LGN LGA G0A.SUMBY` — Vector of [ name amount ] pairs -> Record name to sum of its amounts, first-appearance order
  - `F3D.LEDGER` = `ZIP [ 'LN' 'LA' ] BIND LN LA G0A.SUMBY` — Vector of [ name amount ] entries -> Record name to sum of its amounts, first-appearance order
  - `U1D.BYNAME` = `ZIP [ 'NMS' 'AMTS' ] BIND NMS AMTS G0A.SUMBY` — Vector of [ name amount ] pairs -> Record name to sum of its amounts, names in first-appearance order
  - `U3D.LBY` = `ZIP [ 'LN' 'LA' ] BIND LN LA U2A.GSUM` — Vector of [ name amount ] entries -> Record from each name to the sum of its amounts, first-appearance order
- **5 agents** (W1D, W3C, W3D, U2D, U3D; models fable, haiku), joined at d1/d0a:
  - `W1D.TRUERUN` = `0 [ 'T' BIND 'C' BIND C 1 ADD 0 T SELECT ] SCAN 0 [ MAX ] FOLD` — length of the longest run of adjacent TRUE lanes in a Vector of truths (0 if none)
  - `W1D.RUNSTARTS` = `'T' BIND T FALSE 1 COLLECT T -1 DROP CONCAT NOT AND` — Vector of truths marking the first lane of each run of TRUE in a Vector of truths
  - `W3C.STARTS` = `'F' BIND 0 F LENGTH 1 SUB RANGE 'IX' BIND IX -1 F SELECT [ 0 LT NOT ] FILTER` — Given a truth Vector, the indices at which it is TRUE, in order.
  - `W3D.LONGRUN` = `0 [ 'LB' BIND 'LC' BIND LC 1 ADD 0 LB SELECT ] SCAN 0 [ MAX ] FOLD` — Vector of truths -> length of the longest run of adjacent TRUE (0 when none)
  - `U2D.NTRUE` = `[ TRUE AND ] FILTER LENGTH` — Vector of truths -> how many are TRUE
  - `U3D.RUNMAX` = `0 [ 'RB' BIND 'RA' BIND RA 1 ADD 0 RB SELECT ] SCAN 0 [ MAX ] FOLD` — Vector of truth values -> length of the longest run of adjacent TRUEs (0 if none)
- **5 agents** (W3B, F2A, O1D, O3D, U1B; models sonnet, opus, fable), joined at d1/d0a:
  - `W3B.LARGEST` = `'LE' BIND LE W3B.AMTS W3B.MAXOF 'LM' BIND LE [ 1 GET LM EQ ] FILTER 0 GET` — entry with the largest amount, first on ties
  - `F2A.LARGEST` = `'LG' BIND LG LG [ 1 GET ] MAP F2A.ARGMAX GET` — the [ name amount ] entry with the largest amount, first on a tie
  - `O1D.LARGEST-ENTRY` = `'L' BIND L L O1D.LAMTS G0A.ARGMAX GET` — the [name amount] pair with the largest amount, first on ties
  - `O3D.MAXENTRY` = `'LE' BIND LE LE [ 1 GET ] MAP G0A.ARGMAX GET` — the [ name amount ] pair with the largest amount in a Vector of pairs, first on ties
  - `U1B.LARGEST` = `'LL' BIND LL LL [ 1 GET ] MAP -1 MUL ORDER 0 GET GET` — Vector of [name amount] entries -> the entry with the largest amount, first on a tie
- **4 agents** (G0A, G0B, G0C, G0D; models opus, sonnet, haiku, fable), joined at d1/d0:
  - `G0A.WORDS` = `' ' TOKENIZE [ '' EQ NOT ] FILTER` — split a String on spaces, dropping empty pieces
  - `G0B.WORDS` = `LOWER ' ' TOKENIZE [ CHARS LENGTH 0 GT ] FILTER` — String -> lower-case words split on spaces, empty pieces dropped
  - `G0C.WORDS` = `' ' TOKENIZE [ '' EQ NOT ] FILTER` — Text to the Vector of its non-empty space-separated words.
  - `G0D.WORDS` = `' ' TOKENIZE [ '' EQ NOT ] FILTER` — Split a String on spaces into its non-empty words (any spacing, leading/trailing spaces allowed).
  - `W3B.WORDS` = `' ' TOKENIZE [ '' EQ NOT ] FILTER` — split a String on spaces, dropping empty pieces
  - `W3B.LW` = `LOWER W3B.WORDS` — String -> Vector of its lower-case words
  - `O1A.LWORDS` = `G0A.WORDS [ LOWER ] MAP` — String to its space-separated words in lower case
  - `O2A.LWORDS` = `LOWER G0A.WORDS` — messy String to its Vector of lower-case words
- **4 agents** (G0A, F1C, F2C, F3A; models opus, haiku), joined at d1/d0a:
  - `G0A.DIVISORS` = `'DN' BIND 1 DN RANGE 'DR' BIND DR DN DR G0A.MOD 2 COLLECT ZIP [ 1 GET 0 EQ ] FILTER [ 0 GET ] MAP` — positive divisors of a positive integer, ascending
  - `F1C.DIVS` = `'FN' BIND 1 FN RANGE 'FD' BIND FN FD G0A.MOD 'FR' BIND FD FR 2 COLLECT ZIP [ 1 GET 0 EQ ] FILTER [ 0 GET ] MAP` — n -> ascending Vector of the positive divisors of n
  - `F2C.DIVS` = `'DN' BIND 1 DN RANGE 'DD' BIND DD DN DD G0A.MOD 2 COLLECT ZIP [ 1 GET 0 EQ ] FILTER [ 0 GET ] MAP` — positive integer -> ascending Vector of its divisors
  - `F3A.DIVISORS` = `'DN' BIND 1 DN RANGE DN 1 DN RANGE G0A.MOD [ 0 EQ ] MAP F3A.WHERE` — positive integer -> ascending Vector of its positive divisors
- **4 agents** (G0B, W2B, W2D, F2D; models sonnet, fable), joined at d1/d0a:
  - `G0B.DIVISORS` = `'DN' BIND 1 DN RANGE 'DK' BIND DN DK DIV 'DQ' BIND 0 DK DQ DQ FLOOR SUB 0 GT SELECT [ 0 GT ] FILTER` — positive divisors of a positive integer, ascending
  - `W2B.DIVISORS` = `'DN' BIND 1 DN RANGE 'DV' BIND DV 0 DN DV G0A.MOD 0 GT NOT SELECT [ 0 GT ] FILTER` — positive integer -> Vector of its positive divisors, ascending
  - `W2D.DIVISORS` = `'DP' BIND 1 DP RANGE 'DD' BIND DD 0 DP DD G0A.MOD 1 LT SELECT [ 0 GT ] FILTER` — ascending positive divisors of a positive integer
  - `F2D.DIVISORS` = `'DN' BIND 1 DN RANGE 'DS' BIND DS 0 DN DS G0A.MOD 1 LT SELECT [ 0 GT ] FILTER` — positive integer -> Vector of its positive divisors, ascending
- **4 agents** (O1D, W1A, W3D, U3A; models fable, opus), joined at d0a:
  - `O1D.RLESTEP` = `'E' BIND 'A' BIND A -1 GET 'P' BIND A -1 DROP P 0 GET P 1 GET 1 ADD 2 COLLECT 1 COLLECT CONCAT A E 1 2 COLLECT 1 COLLECT CONCAT 2 COLLECT 0 1 P 0 GET E EQ SELECT GET` — FOLD step [runs elem] over a non-empty Vector of [value count] pairs: bumps the last count when its value equals elem, else appends [elem 1]
  - `W1A.RLESTEP` = `'X' BIND 'A' BIND A -1 GET 'P' BIND A -1 DROP P 0 GET P 1 GET 1 ADD 2 COLLECT 1 COLLECT CONCAT A X 1 2 COLLECT 1 COLLECT CONCAT 2 COLLECT 0 1 P 0 GET X EQ SELECT GET` — run-length step: Vector of [ value count ] pairs and a value -> extend last run or start a new one
  - `W3D.RLESTEP` = `'RE' BIND 'RA' BIND RA -1 GET 'RL' BIND RA -1 DROP RL 0 GET RL 1 GET 1 ADD 2 COLLECT 1 COLLECT CONCAT RA RE 1 2 COLLECT 1 COLLECT CONCAT 2 COLLECT 0 1 RL 0 GET RE EQ SELECT GET` — FOLD step for run-length encoding: acc of [ value count ] pairs (non-empty) and next value -> new acc
  - `U3A.RLESTEP` = `'RX' BIND 'RA' BIND RA -1 GET 'RL' BIND RA -1 DROP RL 0 GET RL 1 GET 1 ADD 2 COLLECT 1 COLLECT CONCAT RA RX 1 2 COLLECT 1 COLLECT CONCAT 2 COLLECT 0 1 RL 0 GET RX EQ SELECT GET` — non-empty Vector of [ value count ] runs, next value -> runs with the value counted in
- **4 agents** (G0B, W1D, W3B, O1D; models sonnet, fable), joined at d0:
  - `G0B.FIRSTS` = `[ 0 GET ] MAP` — first element of each row of a Vector of Vectors
  - `W1D.NAMES` = `[ 0 GET ] MAP` — first element of each row of a Vector of pairs
  - `W3B.NAMES` = `[ 0 GET ] MAP` — Vector of [ name amount ] entries -> names
  - `O1D.LNAMES` = `[ 0 GET ] MAP` — first elements of a Vector of pairs (the names of ledger entries)
- **4 agents** (G0B, W1D, W3B, O1D; models sonnet, fable), joined at d0:
  - `G0B.SECONDS` = `[ 1 GET ] MAP` — second element of each row of a Vector of Vectors
  - `W1D.AMTS` = `[ 1 GET ] MAP` — second element of each row of a Vector of pairs
  - `W3B.AMTS` = `[ 1 GET ] MAP` — Vector of [ name amount ] entries -> amounts
  - `O1D.LAMTS` = `[ 1 GET ] MAP` — second elements of a Vector of pairs (the amounts of ledger entries)
- **4 agents** (W2A, F1A, F2B, F3A; models opus, sonnet), joined at d0/d1:
  - `W2A.TAGS` = `'tags' GET 'TG' BIND [ ] TG 2 COLLECT 0 1 TG NIL? SELECT GET` — order Record -> its tags Vector, or [ ] when it has none
  - `F1A.TAGS0` = `'tags' GET 'TG' BIND [ ] TG 2 COLLECT 0 1 TG NIL? SELECT GET` — order Record -> its tags Vector, or an empty Vector
  - `F2B.TAGS` = `'TG' BIND TG 'tags' GET 'TT' BIND [ ] TT 2 COLLECT 0 1 TT NIL? SELECT GET UNIQUE` — order Record -> its distinct tags as a Vector ([ ] when none)
  - `F3A.TAGS` = `'tags' GET 'TG' BIND [ ] TG 2 COLLECT 0 1 TG NIL? SELECT GET` — order Record -> its tags Vector, empty when missing
- **3 agents** (G0A, F3A, U2A; models opus), joined at d0a:
  - `G0A.SPAN` = `SORT 'SP' BIND SP -1 GET SP 0 GET SUB` — largest minus smallest
  - `F3A.SPREAD` = `SORT 'RG' BIND RG -1 GET RG 0 GET SUB` — largest minus smallest of a non-empty Vector
  - `U2A.RANGE` = `SORT 'RS' BIND RS -1 GET RS 0 GET SUB` — Vector of numbers -> largest minus smallest
- **3 agents** (G0D, W2A, F2A; models fable, opus), joined at d1:
  - `G0D.FIB-INDEX` = `'N' BIND [ 1 1 2 3 5 8 13 21 34 55 89 144 233 377 610 987 ] [ N LT ] FILTER LENGTH 1 ADD` — Smallest k with F(k) >= n (F(1)=F(2)=1), for n up to 987.
  - `W2A.FIBINDEX` = `'FN' BIND [ 1 1 2 3 5 8 13 21 34 55 89 144 233 ] [ FN LT ] FILTER LENGTH 1 ADD` — smallest k with F(k) >= n, for 1 <= n <= 233
  - `F2A.FIBINDEX` = `'FN' BIND 1 40 RANGE [ 0 1 ] [ 'FE' BIND 'FA' BIND FA 1 GET FA 0 GET FA 1 GET ADD 2 COLLECT ] SCAN [ 0 GET ] MAP [ FN LT ] FILTER LENGTH 1 ADD` — smallest k with Fibonacci F(k) >= n (F(1)=F(2)=1)
- **3 agents** (O1D, W3B, O3D; models fable, sonnet), joined at d1:
  - `O1D.SCALED` = `'V' BIND V O1D.PRESENT G0D.MAXV 'M' BIND V [ 'X' BIND NIL 0 X X NIL? SELECT M DIV X NIL? SELECT ] MAP` — each present element divided by the largest present element; NILs stay NIL
  - `W3B.SCALED` = `'SV' BIND SV W3B.PRESENT W3B.MAXOF 'SM' BIND SV [ SM W3B.RAT2 ] MAP` — present readings divided by the largest present one, NIL stays NIL
  - `O3D.SCALED` = `'SV' BIND SV O1D.PRESENT G0D.MAXV 'SM' BIND SV [ 'SE' BIND NIL SE O3D.OR1 SM DIV SE NIL? SELECT ] MAP` — each present element divided by the largest present element; NIL elements stay NIL
- **3 agents** (O2B, W2C, F1A; models sonnet, haiku, opus), joined at d0a/d1:
  - `O2B.LINE` = `'OL' BIND OL 'price' GET 'PP' BIND 0 OL 'qty' GET PP MUL PP NIL? SELECT` — order Record to qty times price, or 0 when it has no price
  - `W2C.LINE` = `'O' BIND O 'price' GET 'P' BIND 0 O 'qty' GET P MUL P NIL? SELECT` — Revenue of one order: qty times price, or 0 when the order has no price.
  - `F1A.PRICE0` = `'price' GET 'PP' BIND 0 PP PP NIL? SELECT` — order Record -> its price, or 0 when it has none
- **3 agents** (O2C, W2A, F1A; models haiku, opus), joined at d1:
  - `O2C.COLLATZ` = `'N' BIND N 1 COLLECT 0 199 RANGE N [ 'E' BIND 'X' BIND X X 2 DIV FLOOR X 3 MUL 1 ADD X X 2 DIV FLOOR 2 MUL SUB 0 EQ SELECT X 1 EQ SELECT ] SCAN CONCAT [ 1 EQ NOT ] FILTER LENGTH` — Collatz steps to reach 1, from a 200-step SCAN trajectory that holds at 1; counts the trajectory values not equal to 1
  - `W2A.COLLATZ` = `'CN' BIND CN 1 COLLECT 1 200 RANGE CN [ 0 MUL ADD W2A.CSTEP ] SCAN CONCAT [ 1 EQ NOT ] FILTER LENGTH` — number of Collatz steps from n down to 1 (up to 200 steps)
  - `F1A.COLLATZ` = `'CN' BIND CN 1 COLLECT 1 200 RANGE CN [ 'CE' BIND F1A.CNEXT ] SCAN CONCAT [ 1 EQ NOT ] FILTER LENGTH` — Collatz steps to reach 1 (up to 200)
- **3 agents** (W1A, W3B, U1B; models opus, sonnet), joined at d1:
  - `W1A.RATIOS` = `'V' BIND V 1 DROP V 0 GET [ ] 2 COLLECT [ 'X' BIND 'A' BIND A 0 GET 'P' BIND X A 1 GET 1 X X NIL? SELECT 1 P P NIL? SELECT DIV NIL 2 COLLECT 0 1 X NIL? NOT P NIL? NOT AND SELECT GET 1 COLLECT CONCAT 2 COLLECT ] FOLD 1 GET` — each element over the one before it, NIL where either is NIL (one shorter)
  - `W3B.RATIOS` = `'RV' BIND 0 RV LENGTH 1 SUB RANGE [ 'RI' BIND RV RI GET RV RI 1 SUB GET W3B.RAT2 ] MAP 1 DROP` — each reading divided by the one before it, NIL where either is missing
  - `U1B.RATIOS` = `'RX' BIND RX 1 DROP RX -1 DROP DIV` — readings -> each reading divided by the one before it (NIL lanes stay NIL)
- **3 agents** (W1B, W1D, O3A; models sonnet, fable, opus), joined at d1:
  - `W1B.RLE` = `'VV' BIND VV 0 GET 0 2 COLLECT 'SD' BIND VV SD [ 'X' BIND 'AC' BIND X 1 0 X AC 0 GET EQ NOT SELECT AC 1 GET ADD 2 COLLECT ] SCAN [ 1 GET ] MAP VV GROUP VALUES [ 'G' BIND G 0 GET G LENGTH 2 COLLECT ] MAP` — run-length encoding of a non-empty Vector: [ [ value count ] ... ]
  - `W1D.RLE` = `'V' BIND V V 0 GET 0 2 COLLECT 1 COLLECT [ 'X' BIND 'A' BIND A -1 GET 'P' BIND P 0 GET X EQ 'SM' BIND A -1 DROP A 2 COLLECT 0 1 SM SELECT GET X P 1 GET 1 ADD 1 SM SELECT 2 COLLECT 1 COLLECT CONCAT ] FOLD` — run-length encoding of a non-empty Vector: [ [ value count ] ... ] per maximal run of equal adjacent values
  - `O3A.RLE` = `'RLV' BIND RLV RLV 0 GET 0 2 COLLECT 1 COLLECT [ O3A.RLESTEP ] FOLD` — non-empty Vector to its run-length encoding, a Vector of [ value count ] pairs
- **3 agents** (W3B, F2A, O3D; models sonnet, opus, fable), joined at d0a/d0:
  - `W3B.TRIMMED` = `SORT 1 DROP -1 DROP W3B.MEAN` — mean after removing one smallest and one largest
  - `F2A.TRIMMED` = `SORT 1 DROP -1 DROP G0A.MEAN` — mean after removing one smallest and one largest
  - `O3D.TRIMMED` = `SORT 1 DROP -1 DROP G0A.MEAN` — mean of a Vector of at least three numbers after removing one smallest and one largest
- **3 agents** (F1C, F2C, U2C; models haiku), joined at d1:
  - `F1C.REV` = `[ 'price' HAS? ] FILTER 'P' BIND P [ 'qty' GET ] MAP P [ 'price' GET ] MAP MUL G0A.SUM` — orders Vector -> total of qty times price over the orders that have a price
  - `F2C.REV` = `[ 'price' HAS? ] FILTER [ F2C.LINE ] MAP G0A.SUM` — Vector of orders -> total of qty times price over priced orders
  - `U2C.REV` = `[ 'price' HAS? ] FILTER [ 'R' BIND R 'qty' GET R 'price' GET MUL ] MAP G0A.SUM` — orders vector -> total of qty times price over the priced orders (0 when none)
- **3 agents** (F2B, F3B, U2B; models sonnet), joined at d0a:
  - `F2B.DIVCOUNT` = `'CN' BIND 1 0 CN 1 CN RANGE G0A.MOD 1 LT SELECT G0A.SUM` — positive integer -> number of its positive divisors
  - `F3B.NDIV` = `'KD' BIND 1 0 KD 1 KD RANGE G0A.MOD 1 LT SELECT G0A.SUM` — positive integer -> number of its divisors
  - `U2B.NDIV` = `'K' BIND 1 0 K 1 K RANGE U2B.MOD 1 LT SELECT G0A.SUM` — positive integer -> number of its positive divisors
- **2 agents** (G0B, F1D; models sonnet, fable), joined at d1:
  - `G0B.COUNT` = `NOT [ NOT ] FILTER LENGTH` — number of TRUE lanes in a Vector of truths
  - `F1D.COUNT-TRUE` = `'CTV' BIND 1 0 CTV SELECT G0A.SUM` — how many lanes of a Vector of truths are TRUE
- **2 agents** (G0D, W3B; models fable, sonnet), joined at d1:
  - `G0D.MAXV` = `SORT -1 GET` — Largest element of a Vector of numbers or Strings.
  - `W3B.MAXOF` = `SORT REVERSE 0 GET` — largest number of a non-empty numeric Vector
- **2 agents** (G0B, W2B; models sonnet), joined at d1:
  - `G0B.BINARY` = `'BN' BIND [ 32 16 8 4 2 1 ] 'BP' BIND BN BP DIV FLOOR 'BQ' BIND BQ BQ 2 DIV FLOOR 2 MUL SUB STR BP BN GT G0B.COUNT DROP JOIN` — integer 1..63 as a base-2 String without leading zeros
  - `W2B.BINARY` = `'BN' BIND [ 32 16 8 4 2 1 ] 'BQ' BIND 0 BQ BQ BN GT SELECT [ 0 GT ] FILTER 'BR' BIND BN BR DIV FLOOR 2 G0A.MOD STR JOIN` — integer 1..63 -> base-2 String without leading zeros
- **2 agents** (G0C, F2A; models haiku, opus), joined at d1:
  - `G0C.ISPRIME` = `'LK' BIND 1 LK RANGE [ LK G0C.DIVIDES ] FILTER LENGTH 2 EQ` — TRUE when the integer k has exactly two divisors in 1..k.
  - `F2A.ISPRIME` = `F2A.DIVISORS LENGTH 2 EQ` — whether a positive integer is prime
- **2 agents** (G0C, W1C; models haiku), joined at d1:
  - `G0C.ARGMAXKEY` = `G0C.MAXKEYS 0 GET` — Record to the first key (in order) whose value is the largest.
  - `W1C.ARGMAX` = `W1C.RECPAIRS 'PR' BIND PR PR 0 GET [ W1C.PICK ] FOLD 0 GET` — Record to the key whose value is largest, the first key on ties.
- **2 agents** (G0D, W2A; models fable, opus), joined at d1:
  - `G0D.PRIMES` = `'N' BIND 1 N RANGE [ G0D.IS-PRIME ] FILTER` — Ascending Vector of the primes <= a positive integer (empty for 1).
  - `W2A.PRIMES` = `'PN' BIND 1 PN RANGE [ W2A.DIVISORS LENGTH 2 EQ ] FILTER` — positive integer n -> ascending Vector of primes <= n
- **2 agents** (G0D, U2D; models fable), joined at d1:
  - `G0D.COLLATZ` = `'N' BIND 1 130 RANGE N 0 2 COLLECT [ G0D.COLLATZ-STEP ] FOLD 1 GET` — Number of Collatz steps to reach 1 for an integer 1..60 (130 bounded iterations).
  - `U2D.COLLATZ` = `0 2 COLLECT 'A' BIND 0 129 RANGE A [ 'EL' BIND [ 'C' 'S' ] BIND C C 2 DIV C 3 MUL 1 ADD C 2 DIV FLOOR 2 MUL C EQ SELECT C 1 EQ SELECT S S 1 ADD C 1 EQ SELECT 2 COLLECT ] FOLD 1 GET` — positive integer (at most 130 steps needed) -> number of Collatz steps to reach 1
- **2 agents** (O1C, U3C; models haiku), joined at d1:
  - `O1C.PREV` = `'X' BIND X 0 GET X O1C.SHIFT` — Vector to the predecessor of each position; the first position is its own predecessor.
  - `U3C.PREV` = `'XV' BIND XV 0 GET 1 COLLECT XV XV LENGTH 1 SUB TAKE CONCAT` — Vector -> the vector shifted right by one place, its first element repeated in front
- **2 agents** (O1D, O3A; models fable, opus), joined at d1:
  - `O1D.BSTEP` = `'C' BIND 'S' BIND S CHARS 'A' BIND A -1 GET C 2 COLLECT JOIN 'P' BIND A -1 DROP JOIN S C 2 COLLECT JOIN 2 COLLECT 0 1 P '()' EQ NOT P '[]' EQ NOT AND NOT SELECT GET` — FOLD/SCAN step [stack char]: pops the stack String's last char when it pairs with the char ('()' or '[]'), else appends the char
  - `O3A.RSTEP` = `'RE' BIND 'RA' BIND RA CHARS -1 DROP JOIN RA RE 2 COLLECT JOIN RA CHARS -1 GET RE 2 COLLECT JOIN 'RP' BIND RP '()' EQ NOT RP '[]' EQ NOT AND NOT SELECT` — FOLD/SCAN step (stack-String char): pop the last char when it opens what char closes, else append char; seed with 'X'
- **2 agents** (O1D, W3D; models fable), joined at d1:
  - `O1D.NILFLAGS` = `[ NIL? ] MAP` — Vector of truths: TRUE where the element is NIL
  - `W3D.GAPSTARTS` = `[ NIL? ] MAP 'GM' BIND GM FALSE 1 COLLECT GM -1 DROP CONCAT NOT AND` — Vector with NILs -> truth per position: TRUE where a run of NILs begins
- **2 agents** (O1A, W1C; models opus, haiku), joined at d0a:
  - `O1A.RUNSTEP` = `'KT' BIND 'KA' BIND KA 1 ADD 0 KT SELECT` — SCAN step over truths: current run length of TRUE (acc+1 or 0)
  - `W1C.RUNSTEP` = `'M' BIND 'A' BIND A 1 ADD 0 M SELECT` — SCAN step over truths: length of the current run of TRUE values, reset to 0 on FALSE.
- **2 agents** (O1C, W3D; models haiku, fable), joined at d1:
  - `O1C.DELTA` = `'C' BIND 1 -1 '([' C SEARCH NIL? NOT SELECT` — Bracket character string to 1 when it opens a bracket, -1 when it closes one.
  - `W3D.BDELTA` = `'BC' BIND -1 1 '([' BC SEARCH NIL? SELECT` — one bracket character -> 1 for an opener ( [ and -1 for a closer ) ]
- **2 agents** (O1D, W1A; models fable, opus), joined at d1:
  - `O1D.BDEPTH` = `O1D.BDELTAS 0 [ ADD ] SCAN 0 1 COLLECT CONCAT G0D.MAXV` — greatest nesting depth of a bracket String (0 for empty)
  - `W1A.BRDEPTH` = `CHARS [ 'C' BIND [ '(' ')' '[' ']' ] [ 1 -1 1 -1 ] RECORD C GET ] MAP 0 [ ADD 0 MAX ] SCAN 0 [ MAX ] FOLD` — greatest bracket nesting depth of a bracket String (open count never below 0)
  - `W1A.BRPREFIX` = `CHARS [ '#' ] [ W1A.BRSTEP ] SCAN 0 0 2 COLLECT [ 'S' BIND 'A' BIND A 0 GET 1 ADD 'I' BIND I I A 1 GET S LENGTH 1 EQ SELECT 2 COLLECT ] FOLD 1 GET` — length of the longest balanced prefix of a bracket String
- **2 agents** (O1D, O3D; models fable), joined at d1:
  - `O1D.UPRUNS` = `'V' BIND V 1 DROP V -1 DROP GT 1 [ 'T' BIND 'A' BIND A 1 ADD 1 T SELECT ] SCAN 1 1 COLLECT CONCAT G0D.MAXV` — length of the longest strictly increasing stretch of adjacent values (1 for a single value)
  - `O3D.UPRUNS` = `'RV' BIND RV 1 DROP RV -1 DROP GT 0 [ 'RU' BIND 1 ADD 0 RU SELECT ] SCAN 0 1 COLLECT CONCAT G0D.MAXV 1 ADD` — length of the longest strictly increasing stretch of adjacent values in a non-empty Vector of numbers (1 for a single value)
- **2 agents** (O1D, O3D; models fable), joined at d1:
  - `O1D.RATIOS` = `'V' BIND V -1 DROP V 1 DROP 2 COLLECT ZIP [ O1D.RATIO ] MAP` — each element divided by the one before it (NIL where either is NIL); one shorter than the input
  - `O3D.RATIOS` = `'SV' BIND SV 1 DROP SV -1 DROP 2 COLLECT ZIP [ [ 'RA' 'RB' ] BIND RA O3D.OR1 RB O3D.OR1 DIV NIL RA NIL? NOT RB NIL? NOT AND SELECT ] MAP` — each element divided by the one before it (one shorter than the input); NIL where either is NIL
- **2 agents** (O2A, F2A; models opus), joined at d0:
  - `O2A.DIVS` = `'DN' BIND 1 DN RANGE [ 'DD' BIND DN DD DIV 'DQ' BIND DQ DQ FLOOR EQ ] FILTER` — positive integer to the ascending Vector of its positive divisors
  - `F2A.DIVISORS` = `'DN' BIND 1 DN RANGE [ 'DD' BIND DN DD DIV 'DQ' BIND DQ DQ FLOOR EQ ] FILTER` — positive divisors of a positive integer, ascending
- **2 agents** (U1C, W3B; models haiku, sonnet), joined at d0a:
  - `U1C.STARTIDX` = `'F' BIND 0 F LENGTH 1 SUB RANGE F 2 COLLECT ZIP [ 1 GET ] FILTER [ 0 GET ] MAP` — Vector of truths -> indices of its TRUE positions
  - `W3B.FSTARTS` = `'FF' BIND 0 FF LENGTH 1 SUB RANGE FF 2 COLLECT ZIP [ 1 GET ] FILTER [ 0 GET ] MAP` — boolean flag Vector -> indices of the TRUE flags
- **2 agents** (W1A, U1B; models opus, sonnet), joined at d1:
  - `W1A.MAJORITY` = `'V' BIND V TALLY 'T' BIND T VALUES W1A.ARGMAX 'I' BIND T KEYS I GET NIL 2 COLLECT 0 1 T VALUES I GET 2 MUL V LENGTH GT SELECT GET` — element occurring in more than half the positions, or NIL
  - `U1B.MAJORITY` = `TALLY 'MT' BIND MT U1B.TOPKEY 1 COLLECT 0 1 MT VALUES 0 [ MAX ] FOLD 2 MUL MT VALUES G0A.SUM GT SELECT GET` — Vector -> value held by more than half the positions, else NIL
- **2 agents** (W1A, U1B; models opus, sonnet), joined at d1:
  - `W1A.SCALED` = `'V' BIND V [ NIL? NOT ] FILTER G0C.MAXV 'M' BIND V [ 'X' BIND 1 X X NIL? SELECT M DIV NIL 2 COLLECT 0 1 X NIL? NOT SELECT GET ] MAP` — each present value divided by the largest present value; NILs stay NIL
  - `U1B.SCALED` = `'SX' BIND SX [ NIL? NOT ] FILTER 'SP' BIND SX SP SP 0 GET [ MAX ] FOLD DIV` — readings -> each present reading divided by the largest present reading
- **2 agents** (W1C, U1B; models haiku, sonnet), joined at d0a:
  - `W1C.RUNPAIR` = `'G' BIND G 0 GET G LENGTH 2 COLLECT` — Vector of equal values to the two-element Vector [value count].
  - `U1B.RUNPAIR` = `'RG' BIND RG 0 GET RG LENGTH 2 COLLECT` — a run (Vector) -> [ value count ]
- **2 agents** (W2D, F2D; models fable), joined at d0a:
  - `W2D.ISPRIME` = `W2D.DIVISORS LENGTH 2 EQ` — whether a positive integer is prime (1 is not)
  - `F2D.ISPRIME` = `F2D.DIVISORS LENGTH 2 EQ` — positive integer -> TRUE when it is prime
- **2 agents** (W3A, O3C; models opus, haiku), joined at d1:
  - `W3A.BRSTEP` = `'BE' BIND 'BA' BIND BA BE 1 COLLECT CONCAT BA -1 DROP 2 COLLECT 1 0 BA -1 GET 0 BE SUB EQ BE 0 LT AND SELECT GET` — FOLD/SCAN step over bracket codes: stack (seed [ 0 ]) pops a matching opener, else pushes the code
  - `O3C.BSTEP` = `'C' BIND 'S' BIND S -1 DROP S C 1 COLLECT CONCAT 2 COLLECT 0 1 C 0 LT S -1 GET C ADD 0 EQ AND SELECT GET` — FOLD/SCAN step on a bracket-code stack (sentinel 0 at the bottom): a closer cancels a matching top opener, otherwise the code is pushed.
- **2 agents** (W3B, U1B; models sonnet), joined at d0:
  - `W3B.SGN` = `'DC' BIND 1 -1 DC '(' EQ NOT DC '[' EQ NOT AND NOT SELECT` — bracket character -> 1 if opening, -1 if closing
  - `U1B.DELTA` = `'DC' BIND 1 -1 DC '(' EQ NOT DC '[' EQ NOT AND NOT SELECT` — one bracket char -> 1 for an opener, -1 for a closer
- **2 agents** (W3B, U1B; models sonnet), joined at d0:
  - `W3B.RUNSTEP` = `'RE' BIND 1 ADD RE MUL` — acc flag -> acc+1 when flag is 1, else 0
  - `U1B.RUNSTEP` = `'RE' BIND 1 ADD RE MUL` — acc flag -> (acc+1)*flag, a running count of consecutive 1s
- **2 agents** (W3B, U3B; models sonnet), joined at d1:
  - `W3B.RAT2` = `'RY' BIND 'RX' BIND RX W3B.NZ0 RY W3B.NZ0 DIV NIL RX NIL? NOT RY NIL? NOT AND SELECT` — x y -> x/y, or NIL when either is NIL
  - `U3B.RAT2` = `'QB' BIND 'QA' BIND NIL 1 QA QA NIL? SELECT 1 QB QB NIL? SELECT DIV QA NIL? NOT QB NIL? NOT AND NOT SELECT` — readings a b (either may be NIL) -> a divided by b, or NIL when either is missing
- **2 agents** (W3D, O3D; models fable), joined at d1:
  - `W3D.MAJORITY` = `'MV' BIND MV TALLY 'MT' BIND MT VALUES W3D.ARGMAX 'MI' BIND MT KEYS MI GET NIL MT VALUES MI GET 2 MUL MV LENGTH GT SELECT` — Vector -> the value in more than half the positions, or NIL
  - `O3D.MAJORITY` = `'JV' BIND JV TALLY 'JT' BIND JT G0A.TOPKEY 'JK' BIND JK NIL JT JK GET 2 MUL JV LENGTH GT SELECT` — the value occurring in more than half the positions of a Vector, or NIL when none does
- **2 agents** (F1A, F2A; models opus), joined at d1:
  - `F1A.BIN` = `'BN' BIND [ 1024 512 256 128 64 32 16 8 4 2 1 ] [ BN GT NOT ] FILTER 'BP' BIND BN BP DIV FLOOR 2 G0A.MOD STR JOIN` — positive integer below 2048 -> base-2 String
  - `F2A.BINARY` = `'BN' BIND 2 20 0 RANGE POW [ BN GT NOT ] FILTER 'BP' BIND BN BP DIV FLOOR 2 G0A.MOD STR JOIN` — positive integer (below 2^21) as a base-2 String
- **2 agents** (F1C, F2C; models haiku), joined at d1:
  - `F1C.NEXT` = `'I' BIND 'X' BIND 1 X 2 DIV X 3 MUL 1 ADD X 2 G0A.MOD 0 EQ SELECT X 1 EQ SELECT` — acc elem -> the Collatz successor of acc (1 stays 1); elem is ignored, for SCAN
  - `F2C.CSTEP` = `'CE' BIND 'CA' BIND CA F2C.CF` — SCAN block: ignores the element, applies one Collatz step to the accumulator
- **2 agents** (F1D, F2A; models fable, opus), joined at d0a:
  - `F1D.SPREAD` = `'SPV' BIND SPV F1D.VMAX SPV F1D.VMIN SUB` — largest minus smallest of a non-empty Vector
  - `F2A.RANGE` = `'RG' BIND RG F2A.VMAX RG F2A.VMIN SUB` — largest minus smallest of a non-empty Vector
- **2 agents** (F1D, F2B; models fable, sonnet), joined at d0a:
  - `F1D.DIVISORS` = `'DVN' BIND 1 DVN RANGE 'DVS' BIND DVS DVN DVS G0A.MOD 1 LT F1D.WHERE` — ascending positive divisors of a positive integer
  - `F2B.DIVISORS` = `'DN' BIND 1 DN RANGE 'DC' BIND DC DN DC G0A.MOD 1 LT F2B.WHERE` — n -> Vector of positive divisors ascending
- **2 agents** (F1D, F2D; models fable), joined at d1:
  - `F1D.BINARY` = `'BNN' BIND BNN 2 5 0 RANGE POW DIV FLOOR 2 G0A.MOD STR JOIN 'BNS' BIND BNS CHARS BNS '1' SEARCH DROP JOIN` — integer 1..63 written in base 2 as a String without leading zeros
  - `F2D.BINARY` = `'BN' BIND BN [ 32 16 8 4 2 1 ] DIV FLOOR 2 G0A.MOD STR JOIN 'BS' BIND BS CHARS BS '1' SEARCH DROP JOIN` — integer 1..63 -> its base-2 digits as a String without leading zeros
- **2 agents** (F1D, F2A; models fable, opus), joined at d0:
  - `F1D.DIGITAL-ROOT` = `1 SUB 9 G0A.MOD 1 ADD` — digital root of a positive integer
  - `F2A.DROOT` = `1 SUB 9 G0A.MOD 1 ADD` — digital root of a positive integer
- **2 agents** (F2A, O1D; models opus, fable), joined at d1:
  - `F2A.DEBTORS` = `F2A.BYNAME F2A.ENTRIES [ 1 GET 0 LT ] FILTER [ 0 GET ] MAP` — names whose summed amount is negative, first-appearance order
  - `O1D.DEBTORS` = `O1D.LEDGER 'R' BIND R KEYS R VALUES 2 COLLECT ZIP [ 1 GET 0 LT ] FILTER [ 0 GET ] MAP` — names whose summed amounts are negative, in first-appearance order
- **2 agents** (F2B, U3D; models sonnet, fable), joined at d0:
  - `F2B.NAMES` = `ZIP 0 GET` — Vector of [name amount] entries -> Vector of names
  - `U3D.NAMES` = `ZIP 0 GET` — Vector of [ name amount ] entries -> Vector of the names, in order
- **2 agents** (F2B, U3D; models sonnet, fable), joined at d0:
  - `F2B.AMOUNTS` = `ZIP 1 GET` — Vector of [name amount] entries -> Vector of amounts
  - `U3D.AMTS` = `ZIP 1 GET` — Vector of [ name amount ] entries -> Vector of the amounts, in order
- **2 agents** (F3C, F3D; models haiku, fable), joined at d1:
  - `F3C.NDIV` = `F3C.DIVS LENGTH` — number of positive divisors of a positive integer
  - `F3D.NDIV` = `'PK' BIND 1 PK RANGE 'PR' BIND PK PK PR DIV FLOOR PR MUL SUB [ 0 EQ ] FILTER LENGTH` — positive integer -> how many positive divisors it has
- **2 agents** (O3B, U1C; models sonnet, haiku), joined at d0a:
  - `O3B.LENS` = `'LN' BIND 'LS' BIND LS 1 DROP LN 1 COLLECT CONCAT LS SUB` — start-index Vector and total length n to the Vector of run lengths (next start, or n, minus start)
  - `U1C.SPANS` = `'N' BIND 'SS' BIND SS 1 DROP N 1 COLLECT CONCAT SS SUB` — start indices and total length n -> length of each segment
- **2 agents** (O3C, U3B; models haiku, sonnet), joined at d1:
  - `O3C.BCODE` = `'C' BIND [ 1 -1 2 -2 ] '()[]' C SEARCH GET` — One bracket character to its code: ( 1, ) -1, [ 2, ] -2.
  - `U3B.DELTA` = `'BC' BIND '()[]' BC SEARCH 'BP' BIND 1 BP 2 MUL SUB BP 2 DIV FLOOR 4 MUL ADD` — one bracket character -> +1 for an opener, -1 for a closer
- **2 agents** (U3B, U3C; models sonnet, haiku), joined at d1:
  - `U3B.RIDS` = `'RV' BIND RV RV 0 GET 0 2 COLLECT [ 'RX' BIND 'RA' BIND RX RA 1 GET RX RA 0 GET SUB 'RD' BIND RD RD MUL 1 MIN ADD 2 COLLECT ] SCAN [ 1 GET ] MAP` — Vector of numbers -> run index (0,1,2..) of each position, a new run starting wherever the value changes
  - `U3C.RUNIDS` = `'XV' BIND XV U3C.PREV 'XB' BIND XV XB LT NOT XV XB GT NOT AND NOT 'XT' BIND 1 0 XT SELECT 0 [ ADD ] SCAN` — Vector of numbers -> run id at each position, the id rising by one at every change of value

## Reuse of inherited Words

| condition | gen | correct solutions reaching one | inherited Words reached, per subject |
| --- | --- | --- | --- |
| breadth | 1 | 87/154 | W1A 4, W1B 4, W1C 6, W1D 5 |
| breadth | 2 | 85/176 | W2A 4, W2B 4, W2C 4, W2D 4 |
| breadth | 3 | 52/156 | W3A 3, W3B 0, W3C 3, W3D 3 |
| fixed | 1 | 137/175 | F1A 8, F1B 8, F1C 8, F1D 8 |
| fixed | 2 | 136/176 | F2A 8, F2B 8, F2C 8, F2D 8 |
| fixed | 3 | 135/172 | F3A 8, F3B 8, F3C 8, F3D 8 |
| open | 1 | 93/155 | O1A 10, O1B 8, O1C 11, O1D 13 |
| open | 2 | 123/176 | O2A 8, O2B 9, O2C 10, O2D 8 |
| open | 3 | 101/155 | O3A 10, O3B 11, O3C 10, O3D 10 |
| uses | 1 | 76/156 | U1A 4, U1B 4, U1C 4, U1D 4 |
| uses | 2 | 122/176 | U2A 6, U2B 6, U2C 7, U2D 6 |
| uses | 3 | 74/155 | U3A 4, U3B 4, U3C 4, U3D 4 |

## H2 — lexicon entries built from other entries

- breadth gen1: 2/8
- breadth gen2: 1/8
- breadth gen3: 4/8
- fixed gen1: 4/8
- fixed gen2: 4/8
- fixed gen3: 4/8
- open gen1: 18/39
- open gen2: 17/44
- open gen3: 18/37
- uses gen1: 4/8
- uses gen2: 1/8
- uses gen3: 4/8

## H4 — Core Words by number of correct solutions reaching them

`BIND` 1517 · `GET` 1249 · `MAP` 1042 · `FILTER` 1013 · `ADD` 918 · `EQ` 843 · `NOT` 839 · `FOLD` 808 · `LENGTH` 798 · `TOKENIZE` 650 · `MUL` 562 · `DIV` 558 · `COLLECT` 515 · `SUB` 462 · `SORT` 384 · `VALUES` 381 · `KEYS` 363 · `JOIN` 359 · `DROP` 334 · `CHARS` 329 · `SELECT` 314 · `LOWER` 292 · `UNIQUE` 283 · `ORDER` 270 · `MAX` 252 · `FLOOR` 245 · `GROUP` 236 · `TRIM` 236 · `RECORD` 228 · `TALLY` 206 · `NIL?` 197 · `SCAN` 195 · `JSON-DECODE` 167 · `ZIP` 163 · `GT` 157 · `UPPER` 154 · `RANGE` 147 · `CONCAT` 133 · `LT` 116 · `HAS?` 55 · `TAKE` 53 · `AND` 52 · `REPLACE` 48 · `REVERSE` 43 · `FLATTEN` 40 · `NIL` 39 · `SQRT` 29 · `FORMAT` 28 · `STR` 26 · `MIN` 25 · `SEARCH` 22 · `PUT` 14 · `POW` 13 · `FALSE` 10 · `TRUE` 5 · `RATIO` 2 · `FILL` 2 · `NUM` 1

Unused (19): `RECONCILE` `ROUND` `GCD` `SHAPE` `RESHAPE` `DEPTH` `BSEARCH` `WITHOUT` `MERGE` `JSON-ENCODE` `EXEC` `CONTRACT` `FAIL` `NIL-REASON` `ABSENT` `DEF` `DEL` `DIGEST` `PRINT`

## H5 — correct solutions whose Core-only expansion disagreed

None.
