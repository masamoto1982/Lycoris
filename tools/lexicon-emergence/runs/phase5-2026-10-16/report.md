# Lexicon-emergence run report

## Per generation

| condition | gen | agents | lexicon | own Words / subject | correct | mean tokens | tokens ÷ expanded | Core-equivalent (H5) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bare | 1 | 4 | 0 | 0 | 156/156 | 26.3 | 1 | 156/156 |
| bare | 2 | 4 | 0 | 0 | 176/176 | 22.4 | 1 | 176/176 |
| base | 0 | 4 | 0 | 24.3 | 175/176 | 6.5 | 0.21 | 175/175 |
| families | 1 | 4 | 8 | 18 | 155/156 | 7.9 | 0.245 | 155/155 |
| families | 2 | 4 | 8 | 18.3 | 175/176 | 7.3 | 0.243 | 175/175 |
| families | 3 | 4 | 8 | 15.5 | 155/156 | 8.9 | 0.233 | 155/155 |
| probe | 1 | 4 | 8 | 0 | 155/156 | 24.8 | 0.905 | 155/155 |
| probe | 2 | 4 | 8 | 0 | 176/176 | 19.8 | 0.814 | 176/176 |
| probe | 3 | 4 | 8 | 0 | 154/156 | 23.4 | 0.732 | 154/154 |

## Training and held-out tasks

Held-out results never rank a Word for the next lexicon. "reaching" is correct solutions that call an inherited Word; "tasks" is how many distinct tasks those are.

| condition | gen | train correct | train ÷ expanded | train reaching | held-out correct | held-out ÷ expanded | held-out reaching | held-out tasks |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bare | 1 | 84/84 | 1 | — | 72/72 | 1 | — | — |
| bare | 2 | 104/104 | 1 | — | 72/72 | 1 | — | — |
| base | 0 | 103/104 | 0.218 | — | 72/72 | 0.199 | — | — |
| families | 1 | 84/84 | 0.245 | 39/84 | 71/72 | 0.245 | 45/71 | 12 |
| families | 2 | 103/104 | 0.258 | 39/103 | 72/72 | 0.221 | 36/72 | 9 |
| families | 3 | 83/84 | 0.255 | 49/83 | 72/72 | 0.198 | 50/72 | 15 |
| probe | 1 | 83/84 | 0.948 | 12/83 | 72/72 | 0.847 | 26/72 | 15 |
| probe | 2 | 104/104 | 0.869 | 27/104 | 72/72 | 0.738 | 36/72 | 9 |
| probe | 3 | 82/84 | 0.774 | 40/82 | 72/72 | 0.675 | 46/72 | 15 |

## Held-out families: shared with training, and inherited

Of each family's correct held-out solutions: "shared" call a Word their own subject also reached from a correct training solution, "inherited" call an inherited Word. Where subjects may define Words, shared all but bounds inherited from above.

| condition | gen | ledger correct · shared · inherited | messy correct · shared · inherited | series correct · shared · inherited |
| --- | --- | --- | --- | --- |
| bare | 1 | 24/24 · 0 · — | 24/24 · 0 · — | 24/24 · 0 · — |
| bare | 2 | 24/24 · 0 · — | 24/24 · 0 · — | 24/24 · 0 · — |
| base | 0 | 24/24 · 21 · — | 24/24 · 22 · — | 24/24 · 11 · — |
| families | 1 | 24/24 · 19 · 17 | 24/24 · 24 · 24 | 23/24 · 7 · 4 |
| families | 2 | 24/24 · 19 · 8 | 24/24 · 24 · 24 | 24/24 · 9 · 4 |
| families | 3 | 24/24 · 20 · 18 | 24/24 · 24 · 24 | 24/24 · 10 · 8 |
| probe | 1 | 24/24 · 9 · 9 | 24/24 · 6 · 12 | 24/24 · 5 · 5 |
| probe | 2 | 24/24 · 8 · 8 | 24/24 · 19 · 24 | 24/24 · 4 · 4 |
| probe | 3 | 24/24 · 14 · 14 | 24/24 · 19 · 24 | 24/24 · 8 · 8 |

## Correct solutions by generation and model

| condition | gen | opus | sonnet | haiku | fable | mean runs |
| --- | --- | --- | --- | --- | --- | --- |
| bare | 1 | 39/39 | 39/39 | 39/39 | 39/39 | — |
| bare | 2 | 44/44 | 44/44 | 44/44 | 44/44 | — |
| base | 0 | 44/44 | 44/44 | 43/44 | 44/44 | — |
| families | 1 | 39/39 | 39/39 | 38/39 | 39/39 | — |
| families | 2 | 44/44 | 44/44 | 44/44 | 43/44 | — |
| families | 3 | 39/39 | 39/39 | 39/39 | 38/39 | — |
| probe | 1 | 39/39 | 39/39 | 39/39 | 38/39 | — |
| probe | 2 | 44/44 | 44/44 | 44/44 | 44/44 | — |
| probe | 3 | 39/39 | 39/39 | 39/39 | 37/39 | — |

## Correct solutions identical to the reference

- N1A: 10 (messy.count, messy.distinct, messy.top_word, prose.longest, prose.freq_top, sensor.present_mean, sensor.ratios, sensor.scaled, series.moving3, series.runmax)
- N1B: 5 (messy.distinct, messy.top_word, prose.longest, prose.freq_top, sensor.present_mean)
- N1C: 2 (series.moving3, series.runmax)
- N1D: 9 (ledger.total, ledger.count_names, messy.count, messy.distinct, sensor.present_mean, sensor.filled, series.median, series.moving3, series.runmax)
- N2A: 16 (messy.count, messy.distinct, messy.top_word, orders.qty_by_sku, orders.top_sku, series.moving3, series.runmax, stats.sum, stats.mean, stats.deviations, stats.variance, stats.stddev, text.count, text.frequency, text.distinct, text.long-words)
- N2B: 7 (orders.qty_by_sku, stats.sum, text.count, text.frequency, text.distinct, text.long-words, text.acronym)
- N2C: 12 (messy.count, orders.unpriced, orders.qty_by_sku, orders.top_sku, series.runmax, stats.sum, stats.mean, text.count, text.frequency, text.distinct, text.long-words, text.acronym)
- N2D: 17 (messy.count, messy.distinct, messy.top_word, orders.qty_by_sku, orders.top_sku, series.runmax, stats.sum, stats.mean, stats.deviations, stats.variance, stats.stddev, stats.range, text.count, text.frequency, text.distinct, text.longest, text.long-words)
- G0A: 1 (orders.unpriced)
- G0B: 6 (orders.unpriced, text.count, text.frequency, text.distinct, text.long-words, text.acronym)
- G0D: 1 (series.runmax)
- L1A: 1 (ledger.count_names)
- L2A: 1 (stats.sum)
- L2C: 1 (series.runmax)
- L3B: 1 (ledger.count_names)
- L3D: 1 (ledger.count_names)
- P1A: 4 (messy.count, messy.distinct, messy.top_word, prose.freq_top)
- P1B: 4 (ledger.total, ledger.count_names, messy.count, messy.distinct)
- P1C: 4 (sensor.filled, sensor.ratios, series.moving3, series.runmax)
- P1D: 5 (ledger.count_names, prose.freq_top, prose.initials_of_long, series.median, series.runmax)
- P2A: 8 (series.moving3, series.runmax, stats.sum, stats.mean, stats.deviations, stats.variance, stats.stddev, stats.range)
- P2B: 1 (stats.sum)
- P2C: 5 (number.divisors, series.moving3, series.runmax, stats.sum, stats.mean)
- P2D: 12 (series.runmax, stats.sum, stats.mean, stats.deviations, stats.variance, stats.stddev, stats.range, text.count, text.frequency, text.distinct, text.longest, text.long-words)
- P3D: 7 (ledger.total, ledger.by_name, ledger.top_name, ledger.count_names, series.median, series.moving3, series.runmax)

## Accuracy per model

| model | agents | correct |
| --- | --- | --- |
| opus | 9 | 371/371 |
| sonnet | 9 | 371/371 |
| haiku | 9 | 369/371 |
| fable | 9 | 366/371 |

## H1 — classes defined independently by two or more agents (39 of 118 classes, 304 Words; 32 reached from two or more models)

Baseline over subjects that inherited nothing — mean Jaccard of defined classes: same-model pairs 0, cross-model pairs 0.042.

- **16 agents** (G0A, G0B, G0C, G0D, L1A, L1B, L1C, L1D, L2A, L2B, L2C, L2D, L3A, L3B, L3C, L3D; models opus, sonnet, haiku, fable), joined at d1/d0a/d0:
  - `G0A.CAP` = `CHARS 'CC' BIND CC 0 GET UPPER CC 1 DROP JOIN LOWER 2 COLLECT JOIN` — first letter upper case, rest lower case
  - `G0B.CAP` = `CHARS 'CC' BIND CC 0 GET UPPER CC 1 DROP JOIN 2 COLLECT JOIN` — upper-case the first letter of a String
  - `G0C.CAP` = `LOWER CHARS 'LC' BIND LC 0 GET UPPER LC 1 DROP JOIN 2 COLLECT JOIN` — Word to its first letter upper-cased and the rest lower-cased.
  - `G0D.CAP` = `'W' BIND W CHARS 1 TAKE JOIN UPPER W CHARS 1 DROP JOIN LOWER 2 COLLECT JOIN` — A word with its first letter upper-cased and the rest lower-cased.
  - `L1A.CAP` = `LOWER CHARS 'CW' BIND CW 0 GET UPPER CW 1 DROP JOIN 2 COLLECT JOIN` — non-empty word with first letter upper, rest lower
  - `L1B.CAP` = `LOWER CHARS 'CC' BIND CC 0 GET UPPER CC 1 DROP JOIN 2 COLLECT JOIN` — non-empty word to first letter upper, rest lower
  - `L1C.UP1` = `CHARS 'C' BIND C 0 GET UPPER C 1 DROP JOIN 2 COLLECT JOIN` — Upper-case the first letter of a String, leave the rest as it is.
  - `L1C.CAP` = `LOWER L1C.UP1` — Title-case one word: lower-case it, then upper-case its first letter.
  - `L1D.CAP` = `LOWER CHARS 'CC' BIND CC 0 GET UPPER CC 1 DROP JOIN 2 COLLECT JOIN` — one word -> first letter upper case, the rest lower case
  - `L2A.CAP` = `CHARS 'CC' BIND CC 0 GET UPPER CC 1 DROP JOIN LOWER 2 COLLECT JOIN` — non-empty String -> first character upper case, the rest lower case
  - `L2B.CAP` = `LOWER CHARS 'ZC' BIND ZC 0 GET UPPER 1 COLLECT ZC 1 DROP CONCAT JOIN` — A word to first letter upper case, rest lower case.
  - `L2C.CAP` = `'ZW' BIND ZW CHARS 0 GET UPPER ZW CHARS 1 DROP JOIN 2 COLLECT JOIN` — Upper-cases the first character of a text and leaves the rest unchanged.
  - `L2D.CAP` = `LOWER CHARS 'CS' BIND CS 0 CS 0 GET UPPER PUT JOIN` — Non-empty String -> first letter upper case, the rest lower case.
  - `L3A.CAP` = `LOWER CHARS 'KC' BIND KC 1 TAKE JOIN UPPER KC 1 DROP JOIN 2 COLLECT JOIN` — non-empty word -> first letter upper case, rest lower case
  - `L3B.CAP` = `LOWER CHARS 'CC' BIND CC 0 GET UPPER CC 1 DROP JOIN 2 COLLECT JOIN` — Word to first letter upper case, rest lower case.
  - `L3C.CAP` = `'CW' BIND CW CHARS 1 TAKE JOIN UPPER CW CHARS 1 DROP JOIN 2 COLLECT JOIN` — Non-empty word -> first letter upper case, the rest kept as they are.
  - `L3D.CAP` = `LOWER CHARS 'XC' BIND XC 0 GET UPPER XC 1 DROP JOIN 2 COLLECT JOIN` — Non-empty word -> first letter upper case, the rest lower case.
- **16 agents** (G0A, G0B, G0C, G0D, L1A, L1B, L1C, L1D, L2A, L2B, L2C, L2D, L3A, L3B, L3C, L3D; models opus, sonnet, haiku, fable), joined at d0a/d1/d0:
  - `G0A.MEDIAN` = `SORT 'MS' BIND MS LENGTH 'ML' BIND MS ML 1 SUB 2 DIV FLOOR GET MS ML 2 DIV FLOOR GET ADD 2 DIV` — median of a non-empty Vector of numbers
  - `G0B.MEDIAN` = `SORT 'MS' BIND MS LENGTH 'MN' BIND MS MN 1 SUB 2 DIV FLOOR GET MS MN 2 DIV FLOOR GET ADD 2 DIV` — median of a Vector of numbers
  - `G0C.MEDIAN` = `SORT 'LS' BIND LS LENGTH 'LN' BIND LS LN 1 SUB 2 DIV FLOOR LN 2 DIV FLOOR 2 COLLECT GET G0C.SUM 2 DIV` — Median of a numeric Vector (mean of the two middle values when the count is even).
  - `G0D.MEDIAN` = `SORT 'S' BIND S LENGTH 'N' BIND S N 1 SUB 2 DIV FLOOR GET S N 2 DIV FLOOR GET ADD 2 DIV` — Median of a non-empty Vector of numbers (mean of the two middle values when even).
  - `L1A.MEDIAN` = `SORT 'QT' BIND QT LENGTH 'QN' BIND QT QN 1 SUB 2 DIV FLOOR GET QT QN 2 DIV FLOOR GET ADD 2 DIV` — median of a non-empty numeric Vector
  - `L1B.MEDIAN` = `SORT 'MDV' BIND MDV LENGTH 'MLN' BIND MDV MLN 1 SUB 2 DIV FLOOR GET MDV MLN 2 DIV FLOOR GET ADD 2 DIV` — median of a non-empty Vector of numbers
  - `L1C.MED` = `SORT 'S' BIND S LENGTH 'N' BIND S N 1 SUB 2 DIV FLOOR GET S N 2 DIV FLOOR GET ADD 2 DIV` — Median of a non-empty Vector of exact numbers (mean of the two middle values when the count is even).
  - `L1D.MEDIAN` = `SORT 'SS' BIND SS LENGTH 'SN' BIND SS SN 1 SUB 2 DIV FLOOR GET SS SN 2 DIV FLOOR GET ADD 2 DIV` — median of a non-empty numeric Vector (mean of the two middle values when the count is even)
  - `L2A.MEDIAN` = `SORT 'MS' BIND MS LENGTH 'ML' BIND MS ML 1 SUB 2 DIV FLOOR GET MS ML 2 DIV FLOOR GET ADD 2 DIV` — non-empty numeric Vector -> median (mean of the two middle values when even)
  - `L2B.MEDIAN` = `SORT 'ZS' BIND ZS LENGTH 'ZL' BIND ZS ZL 1 SUB 2 DIV FLOOR GET ZS ZL 2 DIV FLOOR GET ADD 2 DIV` — Median of a non-empty numeric Vector.
  - `L2C.MED` = `'ZV' BIND ZV SORT 'ZS' BIND ZV LENGTH 'ZN' BIND ZS ZN 1 SUB 2 DIV FLOOR GET ZS ZN 2 DIV FLOOR GET ADD 2 DIV` — Median of a non-empty Vector of numbers (mean of the two middle values when even).
  - `L2D.MEDIAN` = `SORT 'SV' BIND SV LENGTH 'LN' BIND SV LN 1 SUB 2 DIV FLOOR GET SV LN 2 DIV FLOOR GET ADD 2 DIV` — Median of a non-empty numeric Vector (mean of the two middle values when the count is even).
  - `L3A.MEDIAN` = `'MS' BIND MS SORT 'MZ' BIND MZ MS LENGTH 1 SUB 2 DIV FLOOR GET MZ MS LENGTH 2 DIV FLOOR GET ADD 2 DIV` — non-empty numeric Vector -> median
  - `L3B.MEDIAN` = `SORT 'MS' BIND MS LENGTH 'MN' BIND MS MN 1 SUB 2 DIV FLOOR GET MS MN 2 DIV FLOOR GET ADD 2 DIV` — Median of a non-empty numeric Vector.
  - `L3C.MED` = `'MX' BIND MX SORT 'MS' BIND MS MX LENGTH 1 SUB 2 DIV FLOOR MX LENGTH 2 DIV FLOOR 2 COLLECT GET L2B.MEAN` — Non-empty numeric Vector -> median (mean of the two middle values when even).
  - `L3D.MEDIAN` = `SORT 'XS' BIND XS LENGTH 'XN' BIND XS XN 1 SUB 2 DIV FLOOR GET XS XN 2 DIV FLOOR GET ADD 2 DIV` — Median of a non-empty numeric Vector (mean of the two middle values when the count is even).
- **14 agents** (G0A, G0B, G0C, L1A, L1B, L1C, L1D, L2B, L2C, L2D, L3A, L3B, L3C, L3D; models opus, sonnet, haiku, fable), joined at d1/d0:
  - `G0A.UNWORDS` = `[ ' ' 2 COLLECT ] MAP FLATTEN -1 DROP JOIN` — join a non-empty Vector of Strings with single spaces
  - `G0B.SPACED` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of Strings joined with single spaces
  - `G0C.SPJOIN` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of texts joined by single spaces, no leading or trailing space.
  - `L1A.JOINSP` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — join a Vector of space-free Strings with single spaces ('' for empty)
  - `L1B.SJOIN` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of Strings joined with single spaces
  - `L1C.SPJOIN` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Join a Vector of Strings with single spaces, no space at either end.
  - `L1D.UNWORDS` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of words -> one String joined by single spaces
  - `L2B.JOINSP` = `[ ' ' 2 COLLECT ] MAP FLATTEN JOIN TRIM` — Vector of Strings joined with single spaces.
  - `L2C.JOINSP` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Joins a Vector of texts with single spaces.
  - `L2D.SPJOIN` = `'' [ 'W' BIND ' ' W 3 COLLECT JOIN ] FOLD TRIM` — Vector of words -> one String joined by single spaces, no space at either end.
  - `L3A.UNWORDS` = `[ 'UW' BIND ' ' UW 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of space-free words -> one String joined by single spaces
  - `L3B.SPJOIN` = `[ 1 COLLECT ' ' 1 COLLECT CONCAT JOIN ] MAP JOIN TRIM` — Vector of Strings joined with single spaces.
  - `L3C.SJ` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN CHARS -1 DROP JOIN` — Non-empty Vector of texts -> one text with single spaces between them.
  - `L3D.UNWORDS` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of Strings -> one String joined by single spaces ('' for an empty Vector).
- **12 agents** (G0A, G0B, G0C, G0D, L1A, L1D, L2A, L2B, L2D, L3A, L3C, L3D; models opus, sonnet, haiku, fable), joined at d0a/d1/d0:
  - `G0A.ABS` = `'AV' BIND AV AV -1 MUL MAX` — absolute value, lifts over Vectors
  - `G0B.ABS` = `'AD' BIND AD AD -1 MUL MAX` — absolute value of a scalar or element-wise of a Vector
  - `G0C.ABS` = `'LD' BIND LD 0 LD SUB MAX` — Absolute value of an exact number.
  - `G0D.ABS` = `'X' BIND X -1 MUL X X 0 LT SELECT` — Absolute value of a scalar, or lane by lane over a Vector.
  - `L1A.ABS` = `'AB' BIND AB AB -1 MUL MAX` — absolute value, lifts over Vectors
  - `L1D.ABS` = `'AX' BIND AX AX -1 MUL MAX` — absolute value of a number, lifts over Vectors
  - `L2A.ABS` = `'AB' BIND AB AB -1 MUL MAX` — number or numeric Vector -> absolute value, lane by lane
  - `L2B.ABS` = `'ZA' BIND ZA ZA -1 MUL MAX` — Absolute value of a number or Vector (lane-wise).
  - `L2D.ABS` = `'AV' BIND AV -1 MUL AV MAX` — Absolute value of a number or of each element of a Vector.
  - `L3A.ABS` = `'AB' BIND AB AB -1 MUL MAX` — absolute value; lifts over a Vector
  - `L3C.ABS` = `'AB' BIND AB 0 AB SUB MAX` — Number or Vector -> absolute value, as max of x and minus x.
  - `L3D.ABS` = `'XA' BIND XA XA -1 MUL MAX` — Absolute value of a number, lifting over a Vector lane by lane.
- **10 agents** (G0B, G0C, G0D, L1B, L1C, L2A, L2B, L2C, L3A, L3C; models sonnet, haiku, fable, opus), joined at d1/d0a:
  - `G0B.BYNAME` = `'BL' BIND BL G0B.FIRSTS BL G0B.SECONDS G0B.GROUPSUM` — Vector of [key amount] rows -> Record key -> summed amount
  - `G0C.BYNAME` = `'LV' BIND LV [ 0 GET ] MAP LV [ 1 GET ] MAP G0C.GROUPSUM` — Entries [name amount] to a Record from each name to its total amount.
  - `G0D.LEDGER-SUMS` = `'L' BIND L [ 0 GET ] MAP L [ 1 GET ] MAP G0D.SUMS-BY` — Vector of [ name amount ] entries to a Record name -> summed amount.
  - `L1B.NAMESUMS` = `'EN' BIND EN [ 0 GET ] MAP EN [ 1 GET ] MAP GROUP 'GR' BIND GR KEYS GR VALUES [ G0A.SUM ] MAP RECORD` — Vector of [name amount] to Record name -> sum, first-appearance order
  - `L1C.BYNAME` = `'ENTS' BIND ENTS [ 0 GET ] MAP ENTS [ 1 GET ] MAP GROUP 'G' BIND G KEYS G VALUES [ G0A.SUM ] MAP RECORD` — Ledger entries to a Record from each name to the sum of its amounts, names in first-appearance order.
  - `L2A.BYNAME` = `'LBV' BIND LBV [ 0 GET ] MAP LBV [ 1 GET ] MAP L2A.SUMBY` — Vector of [ name amount ] pairs -> Record from name to summed amount, first-appearance order
  - `L2B.LEDGER` = `'ZE' BIND ZE [ 0 GET ] MAP ZE [ 1 GET ] MAP L2B.GSUM` — Vector of [name amount] entries to a Record name -> sum of amounts.
  - `L2C.BYNAME` = `'ZL' BIND ZL [ 0 GET ] MAP ZL [ 1 GET ] MAP L2C.SUMBY` — Ledger entries to a Record from each name to the sum of its amounts.
  - `L3A.BYNAME` = `'LE' BIND LE [ 0 GET ] MAP LE [ 1 GET ] MAP GROUP 'LG' BIND LG KEYS LG VALUES [ L2B.SUM ] MAP RECORD` — Vector of [ key amount ] pairs -> Record key to summed amount, first-appearance order
  - `L3C.BYNAME` = `'BN' BIND BN [ 0 GET ] MAP BN [ 1 GET ] MAP GROUP 'BG' BIND BG KEYS BG VALUES [ L2B.SUM ] MAP RECORD` — Vector of [ name amount ] entries -> Record from name to summed amount, names in first-appearance order.
- **9 agents** (L1A, G0A, G0B, G0D, L1D, L3A, L3B, L3C, L3D; models opus, sonnet, fable, haiku), joined at d0a/d0/d1:
  - `L1A.TOPKEY` = `'TR' BIND TR KEYS TR VALUES L1A.ARGMAX GET` — key of a Record whose (numeric) value is largest, first key on ties
  - `G0A.TOPKEY` = `'TK' BIND TK KEYS TK VALUES G0A.ARGMAX GET` — key of a Record whose numeric value is largest, first on ties
  - `G0B.TOPKEY` = `'TR' BIND TR KEYS TR VALUES G0B.ARGMAX GET` — Record of key->number: the key with the largest value, first on ties
  - `G0D.TOP-KEY` = `'T' BIND T KEYS T VALUES G0D.ARGMAX GET` — Key of a Record whose numeric value is largest; the first key on ties.
  - `L1D.TOPKEY` = `'TR' BIND TR KEYS TR VALUES L1D.ARGMAX GET` — Record with numeric values -> the key with the largest value, the earliest key when tied
  - `L3A.ARGMAXKEY` = `'AK' BIND AK KEYS AK VALUES L3A.ARGMAX GET` — Record of numbers -> key holding the largest value, first on ties
  - `L3B.TOPKEY` = `'TR' BIND TR KEYS TR VALUES 'TV' BIND TV TV G0C.MAXV LT NOT L1D.WHERE 0 GET GET` — Record of key -> number to the first key holding the largest value.
  - `L3C.ARGMAX` = `'AR' BIND AR KEYS 'AK' BIND AR VALUES 'AS' BIND AS G0C.MAXV 'AM' BIND AK AS [ AM EQ ] MAP L1D.WHERE 0 GET GET` — Record of numbers -> the key with the largest value, the first such key on ties.
  - `L3D.TOPKEY` = `'XR' BIND XR KEYS XR VALUES L3D.ARGMAX GET` — Record with numeric values -> the key holding the largest value, the first such key on a tie.
- **8 agents** (G0C, L2A, G0A, G0B, G0D, L2B, L2C, L2D; models haiku, opus, sonnet, fable), joined at d0/d1/d0a:
  - `G0C.ORDERS` = `JSON-DECODE 'orders' GET` — JSON text to the Vector of its order Records.
  - `L2A.ORDERS` = `JSON-DECODE 'orders' GET` — JSON String with an orders array -> Vector of order Records
  - `G0A.REVENUE` = `JSON-DECODE 'orders' GET [ 'RO' BIND RO 'price' GET 'RP' BIND 0 RP RP NIL? SELECT RO 'qty' GET MUL ] MAP G0A.SUM` — orders JSON text -> total qty*price over priced orders
  - `G0A.QTYBY` = `JSON-DECODE 'orders' GET 'QO' BIND QO [ 'sku' GET ] MAP QO [ 'qty' GET ] MAP G0A.SUMBY` — orders JSON text -> Record sku to total qty
  - `G0B.REVENUE` = `JSON-DECODE 'orders' GET [ G0B.ORDVAL ] MAP G0B.SUM` — orders JSON String -> total qty*price over priced orders
  - `G0B.QTYBYSKU` = `JSON-DECODE 'orders' GET 'OL' BIND OL [ 'sku' GET ] MAP OL [ 'qty' GET ] MAP G0B.GROUPSUM` — orders JSON String -> Record sku -> summed qty
  - `G0C.REVENUE` = `G0C.ORDERS [ 'price' HAS? ] FILTER [ G0C.LINE ] MAP G0C.SUM` — Total of qty times price over the priced orders, from the JSON text.
  - `G0C.SKUQTY` = `G0C.ORDERS 'LO' BIND LO [ 'sku' GET ] MAP LO [ 'qty' GET ] MAP G0C.GROUPSUM` — From the JSON text, a Record from each sku to its total qty.
  - `G0C.TAGCOUNTS` = `G0C.ORDERS [ 'tags' HAS? ] FILTER [ 'tags' GET UNIQUE ] MAP FLATTEN TALLY` — From the JSON text, a Record from each tag to the number of orders carrying it.
  - `G0D.ORDERS` = `JSON-DECODE 'orders' GET` — The orders array of a JSON order document, as a Vector of Records.
  - `G0D.REVENUE` = `G0D.ORDERS [ 'price' HAS? ] FILTER [ 'O' BIND O 'qty' GET O 'price' GET MUL ] MAP 0 [ ADD ] FOLD` — Sum of qty*price over the priced orders of a JSON order document.
  - `L2A.REVENUE` = `L2A.ORDERS [ 'price' GET NIL? NOT ] FILTER [ 'OQ' BIND OQ 'qty' GET OQ 'price' GET MUL ] MAP 0 [ ADD ] FOLD` — orders JSON String -> total qty*price over orders that have a price
  - `L2A.QTYBYSKU` = `L2A.ORDERS 'OO' BIND OO [ 'sku' GET ] MAP OO [ 'qty' GET ] MAP L2A.SUMBY` — orders JSON String -> Record from sku to summed qty, first-appearance order
  - `L2A.TAGCOUNTS` = `L2A.ORDERS [ 'tags' HAS? ] FILTER [ 'tags' GET UNIQUE ] MAP [ ] [ CONCAT ] FOLD TALLY` — orders JSON String -> Record from tag to number of orders carrying it
  - `L2B.ORDERS` = `JSON-DECODE 'orders' GET` — JSON String to the Vector of order Records.
  - `L2B.REVENUE` = `L2B.ORDERS [ L2B.LINE ] MAP L2B.SUM` — JSON orders String to total revenue.
  - `L2C.ORDERS` = `JSON-DECODE 'orders' GET` — JSON text with an orders key to the Vector of order Records.
  - `L2D.ORDERS` = `JSON-DECODE 'orders' GET` — JSON String with an "orders" array -> Vector of the order Records.
- **8 agents** (G0A, L2B, G0B, G0C, G0D, L2A, L2C, L2D; models opus, sonnet, haiku, fable), joined at d0a/d0/d1:
  - `G0A.MEAN` = `'MV' BIND MV G0A.SUM MV LENGTH DIV` — arithmetic mean of a Vector
  - `L2B.MEAN` = `'ZV' BIND ZV L2B.SUM ZV LENGTH DIV` — Arithmetic mean of a non-empty numeric Vector, a bare scalar.
  - `G0B.MEAN` = `'MV' BIND MV G0B.SUM MV LENGTH DIV` — arithmetic mean of a Vector
  - `G0C.MEAN` = `'LV' BIND LV G0C.SUM LV LENGTH DIV` — Arithmetic mean of a non-empty numeric Vector.
  - `G0D.MEAN` = `'V' BIND V G0D.SUM V LENGTH DIV` — Arithmetic mean of a non-empty Vector of numbers.
  - `L2A.MEAN` = `'AV' BIND AV 0 [ ADD ] FOLD AV LENGTH DIV` — non-empty numeric Vector -> arithmetic mean
  - `L2C.MEAN` = `'ZV' BIND ZV L2C.SUM ZV LENGTH DIV` — Arithmetic mean of a non-empty Vector of numbers.
  - `L2D.MEAN` = `'MV' BIND MV 0 [ ADD ] FOLD MV LENGTH DIV` — Arithmetic mean of a non-empty numeric Vector.
- **8 agents** (G0A, G0B, G0C, G0D, L2A, L2B, L2C, L2D; models opus, sonnet, haiku, fable), joined at d0a/d1:
  - `G0A.SUMBY` = `GROUP 'GR' BIND GR KEYS GR VALUES [ G0A.SUM ] MAP RECORD` — keys values -> Record key to sum of its values, first-appearance order
  - `G0B.GROUPSUM` = `GROUP 'GG' BIND GG KEYS GG VALUES [ G0B.SUM ] MAP RECORD` — keys-vector values-vector -> Record key -> sum of its values, first-appearance order
  - `G0C.GROUPSUM` = `GROUP 'LG' BIND LG KEYS LG VALUES [ G0C.SUM ] MAP RECORD` — Keys Vector and values Vector to a Record from each key to the sum of its values, keys in first-appearance order.
  - `G0D.SUMS-BY` = `GROUP 'G' BIND G KEYS G VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — keys values G0D.SUMS-BY leaves a Record from each key to the sum of its values, keys in first-appearance order.
  - `L2A.SUMBY` = `'SBV' BIND 'SBK' BIND SBK SBV GROUP 'SBG' BIND SBG KEYS SBG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — keys Vector, numbers Vector -> Record from each key to the sum of its numbers, keys in first-appearance order
  - `L2B.GSUM` = `GROUP 'ZG' BIND ZG KEYS ZG VALUES [ L2B.SUM ] MAP RECORD` — keys-Vector values-Vector to a Record from each key to the sum of its values, keys in first-appearance order.
  - `L2C.SUMBY` = `GROUP 'ZR' BIND ZR KEYS ZR VALUES [ L2C.SUM ] MAP RECORD` — Keys Vector and values Vector to a Record from each key to the sum of its values, keys in first-appearance order.
  - `L2D.SUMBY` = `GROUP 'G' BIND G KEYS G VALUES [ L2D.SUM ] MAP RECORD` — keys Vector, values Vector -> Record from each key to the sum of its values, keys in order of first appearance.
- **8 agents** (G0A, G0B, G0C, G0D, L2A, L2B, L2C, L2D; models opus, sonnet, haiku, fable), joined at d0a/d1:
  - `G0A.VAR` = `G0A.DEVS 'VD' BIND VD VD MUL G0A.MEAN` — population variance
  - `G0B.VAR` = `G0B.DEV 'DD' BIND DD DD MUL G0B.MEAN` — population variance
  - `G0C.VAR` = `'LV' BIND LV G0C.MEAN 'LM' BIND LV [ LM SUB G0C.SQ ] MAP G0C.MEAN` — Population variance of a numeric Vector.
  - `G0D.VARIANCE` = `'V' BIND V V G0D.MEAN SUB 'D' BIND D D MUL G0D.MEAN` — Population variance of a Vector of numbers.
  - `L2A.VAR` = `'VV' BIND VV VV L2A.MEAN SUB 'VD' BIND VD VD MUL L2A.MEAN` — non-empty numeric Vector -> population variance
  - `L2B.VAR` = `L2B.DEV 'ZD' BIND ZD ZD MUL L2B.MEAN` — Population variance of a numeric Vector.
  - `L2C.VAR` = `'ZS' BIND ZS L2C.MEAN 'ZM' BIND ZS ZM SUB 'ZD' BIND ZD ZD MUL L2C.MEAN` — Population variance of a non-empty Vector of numbers.
  - `L2D.VAR` = `'VX' BIND VX VX L2D.MEAN SUB 'VD' BIND VD VD MUL L2D.MEAN` — Population variance of a numeric Vector (mean of squared deviations).
- **7 agents** (G0A, L2B, G0B, G0C, G0D, L2C, L2D; models opus, sonnet, haiku, fable), joined at d0:
  - `G0A.SUM` = `0 [ ADD ] FOLD` — sum of a Vector of numbers (0 for empty)
  - `L2B.SUM` = `0 [ ADD ] FOLD` — Vector of numbers to its sum (0 for empty).
  - `G0B.SUM` = `0 [ ADD ] FOLD` — sum of a Vector of numbers (0 when empty)
  - `G0C.SUM` = `0 [ ADD ] FOLD` — Sum of a numeric Vector; 0 for an empty one.
  - `G0D.SUM` = `0 [ ADD ] FOLD` — Sum of a Vector of numbers (0 for an empty Vector).
  - `L2C.SUM` = `0 [ ADD ] FOLD` — Sum of a Vector of numbers (0 for an empty Vector).
  - `L2D.SUM` = `0 [ ADD ] FOLD` — Sum of a numeric Vector (0 for an empty one).
- **7 agents** (L1A, G0A, G0B, G0D, L1D, L3A, L3D; models opus, sonnet, fable), joined at d0:
  - `L1A.ARGMAX` = `-1 MUL ORDER 0 GET` — index of the largest number in a Vector, first on ties
  - `G0A.ARGMAX` = `-1 MUL ORDER 0 GET` — index of the largest number in a Vector, first on ties
  - `G0B.ARGMAX` = `-1 MUL ORDER 0 GET` — index of the first largest element of a Vector of numbers
  - `G0D.ARGMAX` = `-1 MUL ORDER 0 GET` — Index of the largest number in a Vector; the first one on ties.
  - `L1D.ARGMAX` = `-1 MUL ORDER 0 GET` — index of the largest number in a Vector, the first one when tied
  - `L3A.ARGMAX` = `-1 MUL ORDER 0 GET` — non-empty numeric Vector -> index of its largest element, first on ties
  - `L3D.ARGMAX` = `-1 MUL ORDER 0 GET` — Index of the first largest element of a non-empty numeric Vector (ties go to the earliest).
- **6 agents** (G0A, L2A, G0B, G0C, G0D, L2D; models opus, sonnet, haiku, fable), joined at d0/d0a:
  - `G0A.MOD` = `'MB' BIND 'MA' BIND MA MA MB DIV FLOOR MB MUL SUB` — a b -> a mod b (floored), lifts over Vectors
  - `L2A.MOD` = `'MB' BIND 'MA' BIND MA MA MB DIV FLOOR MB MUL SUB` — a b -> a - b * floor(a/b); lifts over a Vector in either operand
  - `G0B.MOD` = `'MB' BIND 'MA' BIND MA MA MB DIV FLOOR MB MUL SUB` — a b -> a mod b (floor remainder)
  - `G0C.MOD` = `'LB' BIND 'LA' BIND LA LA LB DIV FLOOR LB MUL SUB` — Remainder a mod b for a stack pair [a b] with a below b.
  - `G0D.MOD` = `'B' BIND 'A' BIND A A B DIV FLOOR B MUL SUB` — a b G0D.MOD leaves a - b*floor(a/b), the remainder.
  - `L2D.MOD` = `'MB' BIND 'MA' BIND MA MA MB DIV FLOOR MB MUL SUB` — a b -> a modulo b (a - b*floor(a/b)); lifts over Vectors in either operand.
- **5 agents** (G0A, L1A, L1D, L3B, L3D; models opus, fable, sonnet), joined at d1/d0a:
  - `G0A.LEDGER` = `ZIP 'LZ' BIND LZ 0 GET LZ 1 GET G0A.SUMBY` — Vector of [name amount] pairs -> Record name to total
  - `L1A.BYNAME` = `ZIP 'LZ' BIND LZ 0 GET LZ 1 GET GROUP 'LG' BIND LG KEYS LG VALUES [ G0A.SUM ] MAP RECORD` — Vector of [key amount] pairs to Record key -> summed amount, first-appearance order
  - `L1D.BY-NAME` = `'LE' BIND LE L1D.NAMES LE L1D.AMOUNTS GROUP 'LG' BIND LG KEYS LG VALUES [ G0A.SUM ] MAP RECORD` — Vector of [ name amount ] pairs -> Record name -> sum of its amounts, names in first-appearance order
  - `L3B.BYNAME` = `ZIP 'ZZ' BIND ZZ 0 GET ZZ 1 GET GROUP 'GG' BIND GG KEYS GG VALUES [ L2B.SUM ] MAP RECORD` — Vector of [name amount] entries to Record name -> sum of amounts, first-appearance order.
  - `L3D.SUMS` = `ZIP 'XT' BIND XT 0 GET XT 1 GET GROUP 'XG' BIND XG KEYS XG VALUES [ L2B.SUM ] MAP RECORD` — Vector of [ name amount ] pairs -> Record name -> sum of its amounts, names in order of first appearance.
- **5 agents** (L1A, L1B, L1D, L3A, L3D; models opus, sonnet, fable), joined at d1/d0a:
  - `L1A.BRVAL` = `CHARS [ 'BX' BIND 1 -1 BX '(' EQ NOT BX '[' EQ NOT AND NOT SELECT ] MAP` — bracket String to Vector of +1 (opener) / -1 (closer)
  - `L1B.DELTAS` = `'[' '(' REPLACE ']' ')' REPLACE CHARS [ 'CH' BIND 1 -1 CH '(' EQ SELECT ] MAP` — bracket String to Vector of +1 (opener) / -1 (closer)
  - `L1D.BSIGNS` = `CHARS [ 'C' BIND 1 -1 '([' C SEARCH NIL? NOT SELECT ] MAP` — bracket String -> Vector of +1 per opening bracket and -1 per closing bracket
  - `L3A.STEPS` = `'[' '(' REPLACE ']' ')' REPLACE CHARS [ 'BC' BIND 1 -1 BC '(' EQ SELECT ] MAP` — bracket String -> Vector of +1 per opening and -1 per closing bracket
  - `L3D.DELTAS` = `CHARS [ 'XC' BIND 1 -1 '([' XC SEARCH NIL? NOT SELECT ] MAP` — Bracket String -> Vector of +1 for each opener ( [ and -1 for each closer ) ].
- **5 agents** (L1B, L1D, L3A, L3B, L3D; models sonnet, fable, opus), joined at d1/d0:
  - `L1B.REDUCE` = `'RS' BIND 1 6 RANGE RS [ 'IGN' BIND '()' '' REPLACE '[]' '' REPLACE ] FOLD` — delete adjacent () and [] pairs six times; leaves the reduced String
  - `L1D.BREDUCE` = `'S' BIND [ '()' '()' '()' '()' '()' '()' ] S [ '' REPLACE '[]' '' REPLACE ] FOLD` — bracket String (up to 12 chars) with every adjacent matched pair () or [] repeatedly deleted
  - `L3A.RED1` = `'()' '' REPLACE '[]' '' REPLACE` — one round of deleting adjacent matched pairs () and []
  - `L3A.REDUCE` = `L3A.RED1 L3A.RED1 L3A.RED1 L3A.RED1 L3A.RED1 L3A.RED1` — bracket String (<=12 chars) -> what remains after deleting matched pairs until none is left
  - `L3B.RED1` = `'()' '' REPLACE '[]' '' REPLACE` — String of brackets with every adjacent () and [] pair deleted once.
  - `L3B.RED6` = `L3B.RED1 L3B.RED1 L3B.RED1 L3B.RED1 L3B.RED1 L3B.RED1` — Bracket String fully reduced by deleting matched pairs (enough rounds for up to 12 characters).
  - `L3D.RED1` = `'()' '' REPLACE '[]' '' REPLACE` — One round of deleting every adjacent '()' and '[]' pair from a bracket String.
  - `L3D.REDUCE` = `L3D.RED1 L3D.RED1 L3D.RED1 L3D.RED1 L3D.RED1 L3D.RED1` — Bracket String of at most 12 characters with every adjacent matched pair deleted repeatedly (6 rounds); '' when balanced.
- **4 agents** (G0A, G0B, G0C, G0D; models opus, sonnet, haiku, fable), joined at d1/d0:
  - `G0A.WORDS` = `' ' TOKENIZE [ '' EQ NOT ] FILTER` — split a String on spaces, dropping empty pieces
  - `G0B.WORDS` = `LOWER ' ' TOKENIZE [ CHARS LENGTH 0 GT ] FILTER` — String -> lower-case words split on spaces, empty pieces dropped
  - `G0C.WORDS` = `' ' TOKENIZE [ '' EQ NOT ] FILTER` — Text to the Vector of its non-empty space-separated words.
  - `G0D.WORDS` = `' ' TOKENIZE [ '' EQ NOT ] FILTER` — Split a String on spaces into its non-empty words (any spacing, leading/trailing spaces allowed).
- **4 agents** (G0C, L1A, L2A, L3A; models haiku, opus), joined at d1/d0a:
  - `G0C.MAD` = `'LV' BIND LV G0C.MEDIAN 'LM' BIND LV [ LM SUB G0C.ABS ] MAP G0C.MEDIAN` — Median absolute deviation of a numeric Vector.
  - `L1A.MAD` = `'QV' BIND QV QV L1A.MEDIAN SUB L1A.ABS L1A.MEDIAN` — median absolute deviation of a numeric Vector
  - `L2A.MAD` = `'MV' BIND MV MV L2A.MEDIAN SUB L2A.ABS L2A.MEDIAN` — numeric Vector -> median absolute deviation from the median
  - `L3A.MAD` = `'DV' BIND DV DV L3A.MEDIAN SUB L3A.ABS L3A.MEDIAN` — non-empty numeric Vector -> median absolute deviation from the median
- **4 agents** (G0D, L2A, L2C, L2D; models fable, opus, haiku), joined at d1/d0a:
  - `G0D.DIVISORS` = `'K' BIND 1 K RANGE [ 'D' BIND K K D DIV FLOOR D MUL SUB 0 EQ ] FILTER` — Ascending Vector of the positive divisors of a positive integer.
  - `L2A.DIVISORS` = `'DN' BIND 1 DN RANGE [ 'DD' BIND DN DD L2A.MOD 0 EQ ] FILTER` — positive integer -> ascending Vector of its positive divisors
  - `L2C.DIVS` = `'ZN' BIND 1 ZN RANGE [ 'ZD' BIND ZN ZD DIV FLOOR ZD MUL ZN EQ ] FILTER` — Ascending positive divisors of a positive integer N.
  - `L2D.DIVISORS` = `'DN' BIND 1 DN RANGE [ 'D' BIND DN D L2D.MOD 0 EQ ] FILTER` — Positive integer -> Vector of its positive divisors, ascending.
- **3 agents** (G0A, G0B, L2B; models opus, sonnet), joined at d0/d0a:
  - `G0A.DEVS` = `'DV' BIND DV DV G0A.MEAN SUB` — each number minus the mean
  - `G0B.DEV` = `'DV' BIND DV DV G0B.MEAN SUB` — each number minus the mean
  - `L2B.DEV` = `'ZW' BIND ZW ZW L2B.MEAN SUB` — Vector of each number minus the mean.
- **3 agents** (G0B, G0C, L2B; models sonnet, haiku), joined at d1:
  - `G0B.ORDVAL` = `'OO' BIND OO 'price' GET 'OP' BIND 0 OP OP NIL? SELECT OO 'qty' GET MUL` — order Record -> qty times price, 0 when it has no price
  - `G0C.LINE` = `'LR' BIND LR 'qty' GET LR 'price' GET MUL` — qty times price of one order Record.
  - `L2B.LINE` = `'ZO' BIND ZO 'qty' GET ZO L2B.PRICE MUL` — Order Record to qty times price (0 when unpriced).
- **3 agents** (L1A, L1B, L3A; models opus, sonnet), joined at d1:
  - `L1A.BALPREFIX` = `CHARS [ '#' ] [ L1A.BRSTEP ] SCAN 'BP' BIND BP [ LENGTH 0 MUL 1 ADD ] MAP 0 [ ADD ] SCAN [ 'BI' BIND BI 0 BP BI 1 SUB GET LENGTH 1 EQ SELECT ] MAP 0 [ MAX ] FOLD` — length of the longest balanced prefix of a bracket String
  - `L1B.BPREFIX` = `CHARS 'CV' BIND 0 CV LENGTH RANGE [ 'KK' BIND KK 0 CV KK TAKE JOIN L1B.BALANCED SELECT ] MAP 0 [ MAX ] FOLD` — length of longest balanced prefix of a bracket String
  - `L3A.BPREFIX` = `CHARS 'PC' BIND 0 PC LENGTH RANGE [ 'PK' BIND PC PK TAKE JOIN L3A.REDUCE '' EQ ] FILTER 0 [ MAX ] FOLD` — bracket String -> length of its longest balanced prefix
- **3 agents** (L1A, L1D, L3A; models opus, fable), joined at d1/d0:
  - `L1A.PRESENT` = `[ NIL? NOT ] FILTER` — drop NIL elements of a Vector
  - `L1A.FFILL` = `'FV' BIND FV FV L1A.PRESENT 0 GET [ 'FX' BIND 'FA' BIND FA FX FX NIL? SELECT ] SCAN` — forward-fill NILs; leading NILs take the first present value
  - `L1D.PRESENT` = `[ NIL? NOT ] FILTER` — Vector -> the same Vector without its NIL elements
  - `L3A.PRESENT` = `[ NIL? NOT ] FILTER` — Vector -> its non-NIL elements
  - `L3A.FILLED` = `'FV' BIND FV FV L3A.PRESENT 0 GET [ 'FX' BIND 'FA' BIND FA FX FX NIL? SELECT ] SCAN` — readings with NILs -> forward-filled, leading NILs take the first present reading
- **3 agents** (L1A, L3A, L3D; models opus, fable), joined at d1:
  - `L1A.GAPS` = `[ NIL? ] MAP FALSE 0 2 COLLECT [ L1A.GAPSTEP ] FOLD 1 GET` — number of runs of consecutive NILs
  - `L1A.LONGGAP` = `[ NIL? ] MAP 0 0 2 COLLECT [ L1A.LGSTEP ] FOLD 1 GET` — length of the longest run of consecutive NILs (0 if none)
  - `L3A.GAPS` = `L3A.MISSMASK 'GK' BIND GK 1 [ 0 ] GK CONCAT -1 DROP SUB MUL L2B.SUM` — Vector with NILs -> number of runs of consecutive NILs
  - `L3D.COUNT` = `[ TRUE EQ ] FILTER LENGTH` — Vector of truths -> how many are TRUE.
- **3 agents** (L1B, L3A, L3B; models sonnet, opus), joined at d1/d0a:
  - `L1B.MISS` = `[ NIL? ] MAP 'MF' BIND 1 0 MF SELECT` — readings to 1 where missing (NIL), 0 where present
  - `L3A.MISSMASK` = `[ NIL? 'GM' BIND 1 0 GM SELECT ] MAP` — Vector -> 1 where the element is NIL, 0 elsewhere
  - `L3B.MISS` = `[ NIL? 'MQ' BIND 1 0 MQ SELECT ] MAP` — Vector with NILs to 1 where missing and 0 where present.
- **2 agents** (G0C, G0B; models haiku, sonnet), joined at d0a:
  - `G0C.MAXV` = `'LX' BIND LX LX 0 GET [ MAX ] FOLD` — Largest element of a non-empty numeric Vector.
  - `G0B.MAXOF` = `'XV' BIND XV XV 0 GET [ MAX ] FOLD` — largest element of a non-empty Vector
- **2 agents** (L2C, L2D; models haiku, fable), joined at d1:
  - `L2C.REV` = `[ 'price' HAS? ] FILTER [ 'ORD' BIND ORD 'qty' GET ORD 'price' GET MUL ] MAP L2C.SUM` — Total of qty times price over the orders that have a price.
  - `L2C.TAGS` = `[ 'tags' HAS? ] FILTER [ 'tags' GET ] MAP [ UNIQUE ] MAP FLATTEN TALLY` — Orders Vector to a Record from each tag to the number of orders carrying it.
  - `L2D.REVENUE` = `[ 'price' HAS? ] FILTER [ 'O' BIND O 'qty' GET O 'price' GET MUL ] MAP 0 [ ADD ] FOLD` — Vector of order Records -> sum of qty*price over the orders that have a price.
- **2 agents** (G0A, L2A; models opus), joined at d0a:
  - `G0A.SPAN` = `SORT 'SP' BIND SP -1 GET SP 0 GET SUB` — largest minus smallest
  - `L2A.RANGEV` = `SORT 'RS' BIND RS -1 GET RS 0 GET SUB` — non-empty numeric Vector -> largest minus smallest
- **2 agents** (G0B, L1B; models sonnet), joined at d1:
  - `G0B.MODEOF` = `TALLY G0B.TOPKEY` — most frequent element of a Vector, first-appearing on ties
  - `L1B.TOPW` = `TALLY 'TR' BIND TR KEYS TR VALUES -1 MUL ORDER 0 GET GET` — most frequent element of a Vector; ties go to the earliest first occurrence
- **2 agents** (G0B, L2D; models sonnet, fable), joined at d0:
  - `G0B.FIRSTS` = `[ 0 GET ] MAP` — first element of each row of a Vector of Vectors
  - `L2D.NAMES` = `[ 0 GET ] MAP` — Vector of [name amount] entries -> Vector of the names (first elements).
- **2 agents** (G0B, L2D; models sonnet, fable), joined at d0:
  - `G0B.SECONDS` = `[ 1 GET ] MAP` — second element of each row of a Vector of Vectors
  - `L2D.AMTS` = `[ 1 GET ] MAP` — Vector of [name amount] entries -> Vector of the amounts (second elements).
- **2 agents** (G0B, L2B; models sonnet), joined at d1:
  - `G0B.CSTEP` = `'CX' BIND 3 CX MUL 1 ADD CX 2 DIV CX 2 G0B.MOD 0 GT SELECT 'CY' BIND 1 CY CX 1 EQ SELECT` — one Collatz step, with 1 staying at 1
  - `L2B.CSTEP` = `'ZX' BIND 1 ZX 3 MUL 1 ADD ZX 2 DIV ZX ZX 2 DIV FLOOR 2 MUL SUB 0 GT SELECT ZX 1 EQ SELECT` — One Collatz step; 1 stays 1.
- **2 agents** (G0C, L2D; models haiku, fable), joined at d0a:
  - `G0C.MINV` = `'LX' BIND LX LX 0 GET [ MIN ] FOLD` — Smallest element of a non-empty numeric Vector.
  - `L2D.MINV` = `'MNV' BIND MNV MNV 0 GET [ MIN ] FOLD` — Smallest element of a non-empty numeric Vector.
- **2 agents** (G0D, L2A; models fable, opus), joined at d1:
  - `G0D.FIB-INDEX` = `'N' BIND [ 1 1 2 3 5 8 13 21 34 55 89 144 233 377 610 987 ] [ N LT ] FILTER LENGTH 1 ADD` — Smallest k with F(k) >= n (F(1)=F(2)=1), for n up to 987.
  - `L2A.FIBINDEX` = `'FN' BIND 30 L2A.FIBS [ FN LT ] FILTER LENGTH 1 ADD` — positive integer n (up to F(30)) -> smallest k with F(k) >= n
- **2 agents** (L1A, L3A; models opus), joined at d1:
  - `L1A.RLE` = `'RV' BIND RV RV 0 GET 0 2 COLLECT 1 COLLECT [ L1A.RLESTEP ] FOLD` — run-length encoding [ [ value count ] ... ] of a non-empty Vector
  - `L3A.RLE` = `'RV' BIND RV RV 0 GET 0 2 COLLECT 1 COLLECT [ 'RX' BIND 'RA' BIND RA -1 GET 'RL' BIND RA -1 DROP RX RL 1 GET 1 ADD 2 COLLECT 1 COLLECT CONCAT RA RX 1 2 COLLECT 1 COLLECT CONCAT 2 COLLECT 0 1 RL 0 GET RX EQ SELECT GET ] FOLD` — non-empty Vector -> run-length encoding as [ value count ] pairs
- **2 agents** (L1A, L3A; models opus), joined at d1:
  - `L1A.UPRUNS` = `'UV' BIND UV UV 0 GET 0 0 3 COLLECT [ L1A.UPSTEP ] FOLD 2 GET` — length of the longest strictly increasing adjacent stretch
  - `L3A.UPRUNS` = `'UV' BIND UV UV 0 GET 0 2 COLLECT [ 'UX' BIND 'UA' BIND UX UA 1 GET 1 ADD 1 UX UA 0 GET GT SELECT 2 COLLECT ] SCAN [ 1 GET ] MAP G0C.MAXV` — non-empty numeric Vector -> length of longest strictly increasing adjacent stretch
- **2 agents** (L1B, L3B; models sonnet), joined at d1:
  - `L1B.BALANCED` = `L1B.REDUCE '' EQ` — bracket String to TRUE when fully balanced
  - `L3B.BAL` = `L3B.RED6 '' EQ` — Bracket String to TRUE when properly nested and matched.
- **2 agents** (L1B, L3A; models sonnet, opus), joined at d1:
  - `L1B.MAJ` = `'MJ' BIND MJ TALLY 'MT' BIND MT KEYS 'MK' BIND MT VALUES 'MC' BIND MC -1 MUL ORDER 0 GET 'MI' BIND MK MI MK LENGTH MC MI GET 2 MUL MJ LENGTH GT SELECT GET` — value in more than half the positions, else NIL
  - `L3A.MAJ` = `'MV' BIND MV TALLY 'MT' BIND MT KEYS MT VALUES L3A.ARGMAX GET NIL 2 COLLECT 0 1 MT VALUES G0C.MAXV 2 MUL MV LENGTH GT SELECT GET` — non-empty Vector -> value filling more than half the positions, or NIL
- **2 agents** (L2A, L3A; models opus), joined at d0a:
  - `L2A.MOVING3` = `'MVT' BIND MVT -2 DROP MVT 1 DROP -1 DROP ADD MVT 2 DROP ADD 3 DIV` — numeric Vector (3+) -> means of every three consecutive numbers
  - `L3A.MOVING3` = `'WV' BIND WV -2 DROP WV 1 DROP -1 DROP ADD WV 2 DROP ADD 3 DIV` — numeric Vector of length >= 3 -> means of each three consecutive numbers

## Reuse of inherited Words

| condition | gen | correct solutions reaching one | inherited Words reached, per subject |
| --- | --- | --- | --- |
| families | 1 | 84/155 | L1A 4, L1B 4, L1C 4, L1D 4 |
| families | 2 | 75/175 | L2A 4, L2B 4, L2C 5, L2D 5 |
| families | 3 | 99/155 | L3A 4, L3B 5, L3C 5, L3D 5 |
| probe | 1 | 38/155 | P1A 0, P1B 0, P1C 4, P1D 3 |
| probe | 2 | 63/176 | P2A 4, P2B 3, P2C 4, P2D 3 |
| probe | 3 | 86/154 | P3A 4, P3B 5, P3C 5, P3D 3 |

## H2 — lexicon entries built from other entries

- families gen1: 2/8
- families gen2: 2/8
- families gen3: 2/8
- probe gen1: 2/8
- probe gen2: 2/8
- probe gen3: 2/8

## H4 — Core Words by number of correct solutions reaching them

`BIND` 1084 · `GET` 885 · `MAP` 715 · `FILTER` 646 · `ADD` 621 · `FOLD` 591 · `LENGTH` 569 · `EQ` 504 · `NOT` 491 · `TOKENIZE` 449 · `COLLECT` 361 · `DIV` 355 · `MUL` 352 · `SUB` 316 · `DROP` 265 · `VALUES` 262 · `SORT` 255 · `JOIN` 253 · `CHARS` 252 · `KEYS` 251 · `MAX` 232 · `SELECT` 200 · `UNIQUE` 197 · `LOWER` 195 · `ORDER` 171 · `TRIM` 160 · `GROUP` 153 · `TALLY` 146 · `SCAN` 146 · `FLOOR` 145 · `RECORD` 129 · `RANGE` 124 · `NIL?` 124 · `GT` 112 · `CONCAT` 109 · `ZIP` 109 · `UPPER` 108 · `JSON-DECODE` 96 · `LT` 90 · `REPLACE` 43 · `HAS?` 41 · `AND` 37 · `TAKE` 36 · `REVERSE` 30 · `FLATTEN` 27 · `MIN` 22 · `NIL` 22 · `SQRT` 16 · `FORMAT` 16 · `STR` 15 · `TRUE` 13 · `FALSE` 11 · `SEARCH` 11 · `POW` 7 · `PUT` 5 · `NUM` 5 · `MERGE` 4 · `RATIO` 3 · `GCD` 2

Unused (18): `RECONCILE` `ROUND` `FILL` `SHAPE` `RESHAPE` `DEPTH` `BSEARCH` `WITHOUT` `JSON-ENCODE` `EXEC` `CONTRACT` `FAIL` `NIL-REASON` `ABSENT` `DEF` `DEL` `DIGEST` `PRINT`

## H5 — correct solutions whose Core-only expansion disagreed

None.
