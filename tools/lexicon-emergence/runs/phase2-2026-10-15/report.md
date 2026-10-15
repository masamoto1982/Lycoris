# Lexicon-emergence run report

## Per generation

| condition | gen | agents | lexicon | correct | mean tokens | tokens ÷ expanded | Core-equivalent (H5) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| bottleneck | 0 | 3 | 0 | 54/54 | 9.4 | 0.336 | 54/54 |
| bottleneck | 1 | 3 | 8 | 54/54 | 4.8 | 0.151 | 54/54 |
| bottleneck | 2 | 3 | 8 | 54/54 | 6.7 | 0.222 | 54/54 |
| shuffled | 1 | 3 | 8 | 54/54 | 6 | 0.2 | 54/54 |
| shuffled | 2 | 3 | 8 | 54/54 | 7 | 0.233 | 54/54 |
| solo | 0 | 3 | 0 | 54/54 | 10.4 | 0.35 | 54/54 |

## Accuracy per model

| model | agents | correct |
| --- | --- | --- |
| opus | 6 | 108/108 |
| sonnet | 6 | 108/108 |
| haiku | 6 | 108/108 |

## H1 — classes defined independently by two or more agents (19 of 34 classes, 129 Words; 16 reached from two or more models)

Baseline over subjects that inherited nothing — mean Jaccard of defined classes: same-model pairs 0.37, cross-model pairs 0.243.

- **14 agents** (G0A, G0B, G1A, G1B, G2A, G2B, R1A, R1B, R1C, R2A, R2C, S0A, S0B, S0C; models opus, sonnet, haiku), joined at d1/d0/d0a:
  - `G0A.CAP` = `CHARS 'C' BIND C 0 GET UPPER C 1 DROP JOIN 2 COLLECT JOIN` — upper-case the first letter of a non-empty String
  - `G0B.CAP` = `CHARS 'C' BIND C 0 GET UPPER 1 COLLECT C 1 DROP CONCAT JOIN` — capitalize one word
  - `G1A.CAPWORD` = `CHARS 'C' BIND C 0 GET UPPER C 1 DROP JOIN 2 COLLECT JOIN` — a non-empty String with its first character upper-cased
  - `G1B.CAP` = `CHARS 'C' BIND C 0 GET UPPER 1 COLLECT C 1 DROP CONCAT JOIN` — capitalize the first letter of one word
  - `G2A.CAP` = `CHARS 'C' BIND C 0 GET UPPER C 1 DROP JOIN 2 COLLECT JOIN` — upper-case the first letter of a non-empty String
  - `G2B.CAP` = `'W' BIND W CHARS 'C' BIND C 0 C 0 GET UPPER PUT JOIN` — upper-case the first letter of a non-empty word String
  - `R1A.CAPWORD` = `CHARS 'C' BIND C 0 GET UPPER C 1 DROP JOIN 2 COLLECT JOIN` — a word with its first letter upper-cased
  - `R1B.CAP` = `'W' BIND W CHARS 0 GET UPPER W CHARS 1 DROP JOIN 2 COLLECT JOIN` — Capitalize the first letter of a word (String).
  - `R1C.CAPWORD` = `CHARS 'C' BIND C 0 GET UPPER 1 COLLECT C 1 DROP CONCAT JOIN` — Upper-cases the first character of a String and keeps the rest unchanged.
  - `R2A.CAP` = `CHARS 'C' BIND C 0 GET UPPER C 1 DROP JOIN 2 COLLECT JOIN` — String with its first letter upper-cased
  - `R2C.CAP` = `'W' BIND W CHARS 'C' BIND C 0 GET UPPER 1 COLLECT C 1 DROP CONCAT JOIN` — Upper-cases the first letter of one word and keeps the rest.
  - `S0A.CAP` = `CHARS 'C' BIND C 0 GET UPPER C 1 DROP JOIN 2 COLLECT JOIN` — upper-case the first letter of a non-empty String
  - `S0B.CAP1` = `CHARS 'CS' BIND CS 0 GET UPPER CS 1 DROP JOIN 2 COLLECT JOIN` — Upper-case the first letter of a word.
  - `S0C.CAPW` = `'W' BIND W CHARS 'C' BIND C 0 GET UPPER C 1 DROP JOIN 2 COLLECT JOIN` — Upper-cases the first character of one String and keeps the rest.
- **13 agents** (G0A, G0B, G1A, G1B, G1C, G2A, G2C, R1A, R1C, R2A, R2C, S0A, S0C; models opus, sonnet, haiku), joined at d1/d0/d0a:
  - `G0A.ABS` = `'X' BIND X 0 X SUB MAX` — absolute value, lifts over Vectors
  - `G0B.ABS` = `'D' BIND D D -1 MUL MAX` — absolute value, lifts over vectors
  - `G1A.ABS` = `'X' BIND X 0 X SUB MAX` — absolute value, element-wise over a Vector
  - `G1B.ABSV` = `'D' BIND D D -1 MUL MAX` — elementwise absolute value of a numeric Vector
  - `G1C.ABS` = `'D' BIND D D -1 MUL MAX` — absolute value of each element of a numeric Vector, element-wise
  - `G2A.ABS` = `'X' BIND X 0 X SUB MAX` — absolute value, element-wise
  - `G2C.ABS` = `'X' BIND X 0 X SUB MAX` — absolute value of a number, or element-wise over a numeric Vector
  - `R1A.ABS` = `'X' BIND X 0 X SUB MAX` — absolute value, element-wise over a Vector
  - `R1C.ABS` = `'D' BIND D D -1 MUL MAX` — Element-wise absolute value of a Vector, as the larger of x and -x.
  - `R2A.ABS` = `'X' BIND X 0 X SUB MAX` — absolute value of a number (lifts over a Vector)
  - `R2C.ABS` = `'X' BIND X 0 X SUB MAX` — Element-wise absolute value: the larger of X and its negation.
  - `S0A.ABS` = `'X' BIND X X -1 MUL MAX` — absolute value, lane by lane over a number or Vector
  - `S0C.ABS` = `2 POW SQRT` — Absolute value of an exact number, via its square root.
- **7 agents** (G0A, G1A, G1B, G2A, R1A, R2A, S0A; models opus, sonnet), joined at d0a/d1/d0:
  - `G0A.MAD` = `'X' BIND X X G0A.MEDIAN SUB G0A.ABS G0A.MEDIAN` — median absolute deviation from the median
  - `G1A.MAD` = `'V' BIND V V G0A.MEDIAN SUB G1A.ABS G0A.MEDIAN` — median absolute deviation of a non-empty numeric Vector
  - `G1B.MAD` = `'V' BIND V G0A.MEDIAN 'M' BIND V M SUB G1B.ABSV G0A.MEDIAN` — median absolute deviation of a numeric Vector
  - `G2A.MAD` = `'V' BIND V V G0A.MEDIAN SUB G2A.ABS G0A.MEDIAN` — median absolute deviation of a non-empty numeric Vector
  - `R1A.MAD` = `'V' BIND V V X1.FIROSO SUB R1A.ABS X1.FIROSO` — median absolute deviation of a numeric Vector
  - `R2A.MAD` = `'V' BIND V V X2.ZILOLE SUB R2A.ABS X2.ZILOLE` — median absolute deviation of a numeric Vector
  - `S0A.MAD` = `'V' BIND V V S0A.MEDIAN SUB S0A.ABS S0A.MEDIAN` — median absolute deviation from the median
- **7 agents** (G0A, G1A, G1B, G2A, R1A, R2A, S0A; models opus, sonnet), joined at d0/d1/d0a:
  - `G0A.MOVING3` = `'V' BIND V -2 DROP V 1 DROP -1 DROP ADD V 2 DROP ADD 3 DIV` — means of every three consecutive numbers
  - `G1A.MOVING3` = `'V' BIND V -2 DROP V 1 DROP -1 DROP ADD V 2 DROP ADD 3 DIV` — means of every three consecutive numbers of a Vector (length >= 3)
  - `G1B.MOVING3` = `'V' BIND V LENGTH 2 SUB 'K' BIND V K TAKE V 1 DROP K TAKE ADD V 2 DROP ADD 3 DIV` — means of every three consecutive numbers
  - `G2A.MOVING3` = `'V' BIND V LENGTH 2 SUB 'N' BIND V N TAKE V 1 DROP N TAKE ADD V 2 DROP ADD 3 DIV` — means of every three consecutive numbers (length >= 3)
  - `R1A.MOVING3` = `'V' BIND V -2 DROP V 1 DROP -1 DROP ADD V 2 DROP ADD 3 DIV` — means of every three consecutive numbers
  - `R2A.MOVING3` = `'V' BIND V -2 DROP V 1 DROP -1 DROP ADD V 2 DROP ADD 3 DIV` — means of every three consecutive numbers
  - `S0A.MOVING3` = `'V' BIND V LENGTH 2 SUB 'N' BIND V N TAKE V 1 DROP N TAKE ADD V 2 DROP ADD 3 DIV` — means of every three consecutive numbers
- **7 agents** (G0A, G1A, G1B, G2A, R1A, R2A, S0A; models opus, sonnet), joined at d0:
  - `G0A.RUNMAX` = `'V' BIND V V 0 GET [ MAX ] SCAN` — running maximum of a non-empty numeric Vector
  - `G1A.RUNMAX` = `'V' BIND V V 0 GET [ MAX ] SCAN` — running maximum of a non-empty numeric Vector
  - `G1B.RUNMAX` = `'V' BIND V V 0 GET [ MAX ] SCAN` — running maximum of a numeric Vector
  - `G2A.RUNMAX` = `'V' BIND V V 0 GET [ MAX ] SCAN` — running maximum of a non-empty numeric Vector
  - `R1A.RUNMAX` = `'V' BIND V V 0 GET [ MAX ] SCAN` — running maximum of a numeric Vector
  - `R2A.RUNMAX` = `'V' BIND V V 0 GET [ MAX ] SCAN` — running maximum of a numeric Vector
  - `S0A.RUNMAX` = `'V' BIND V V 0 GET [ MAX ] SCAN` — running maximum, one lane per input lane
- **7 agents** (G0A, G1A, G1B, G2A, R1A, R2A, S0A; models opus, sonnet), joined at d1/d0:
  - `G0A.MODE` = `SORT TALLY 'T' BIND T KEYS T VALUES G0A.ARGMAX GET` — most frequent number, smallest on ties
  - `G1A.MODE` = `SORT TALLY G0B.TOPKEY` — most frequent element of a Vector; smallest on ties
  - `G1B.MODE` = `SORT TALLY G0B.TOPKEY` — most frequent number, smallest on ties
  - `G2A.MODE` = `SORT TALLY G0B.TOPKEY` — most frequent element; smallest on ties
  - `R1A.MODE` = `SORT TALLY X1.NILUGA` — most frequent number, smallest on ties
  - `R2A.MODE` = `SORT TALLY X2.PIDOZE` — most frequent element, smallest on ties
  - `S0A.MODE` = `SORT S0A.MOST-FREQ` — most frequent number; ties go to the smallest
- **7 agents** (G0B, G1A, G2A, R1A, S0A, S0B, S0C; models sonnet, opus, haiku), joined at d1/d0:
  - `G0B.JOINSP` = `[ 'X' BIND ' ' X 2 COLLECT JOIN ] MAP JOIN TRIM` — join a Vector of words with single spaces
  - `G1A.LONGEST` = `'W' BIND W W [ CHARS LENGTH ] MAP G0A.ARGMAX GET` — longest String in a Vector of Strings; first on ties
  - `G2A.LONGEST` = `'W' BIND W W [ CHARS LENGTH ] MAP G0A.ARGMAX GET` — longest String in a Vector; first on ties
  - `R1A.ARGMAX-BY-LEN` = `'W' BIND W W [ CHARS LENGTH ] MAP X1.PIMAPA GET` — longest String in a Vector, first on ties
  - `S0A.JOIN-SP` = `[ ' ' 2 COLLECT ] MAP FLATTEN -1 DROP JOIN` — join a non-empty Vector of Strings with single spaces
  - `S0B.SPJOIN` = `[ ' ' 2 COLLECT ] MAP FLATTEN JOIN TRIM` — Join a Vector of Strings with single spaces.
  - `S0C.SPJOIN` = `[ ' ' 2 COLLECT ] MAP FLATTEN JOIN TRIM` — Joins a Vector of Strings into one String separated by single spaces.
- **6 agents** (G0A, G0B, G0C, S0A, S0B, S0C; models opus, sonnet, haiku), joined at d0/d0a/d1:
  - `G0A.MEDIAN` = `SORT 'V' BIND V LENGTH 'N' BIND V N 1 SUB 2 DIV FLOOR N 2 DIV FLOOR 2 COLLECT GET G0A.MEAN` — median of a non-empty numeric Vector
  - `X1.FIROSO` = `SORT 'V' BIND V LENGTH 'N' BIND V N 1 SUB 2 DIV FLOOR N 2 DIV FLOOR 2 COLLECT GET X1.GISUZU`
  - `X2.ZILOLE` = `SORT 'V' BIND V LENGTH 'N' BIND V N 1 SUB 2 DIV FLOOR N 2 DIV FLOOR 2 COLLECT GET X2.NEGEKE`
  - `G0B.MEDIAN` = `SORT 'S' BIND S LENGTH 'N' BIND S N 1 SUB 2 DIV FLOOR N 2 DIV FLOOR 2 COLLECT GET G0B.MEAN` — median of a Vector
  - `G0C.MEDIAN` = `'MDV' BIND MDV SORT 'MDS' BIND MDV LENGTH 'MDN' BIND MDS MDN 1 SUB 2 DIV FLOOR GET MDS MDN 2 DIV FLOOR GET ADD 2 DIV` — Median of a numeric Vector: mean of the two middle sorted values (one value twice when the count is odd).
  - `S0A.MEDIAN` = `SORT 'S' BIND S LENGTH 'N' BIND S N 1 SUB 2 DIV FLOOR N 2 DIV FLOOR 2 COLLECT GET S0A.MEAN` — median of a non-empty Vector of numbers
  - `S0B.MEDIAN` = `SORT 'XS' BIND XS LENGTH 'LN' BIND XS LN 1 SUB 2 DIV FLOOR GET XS LN 2 DIV FLOOR GET ADD 2 DIV` — Median of a Vector of numbers.
  - `S0C.MEDIAN` = `'V' BIND V SORT 'S' BIND V LENGTH 'N' BIND S N 1 SUB 2 DIV FLOOR GET S N 2 DIV FLOOR GET ADD 2 DIV` — Median of a Vector of exact numbers (mean of the two middle values when the count is even).
- **6 agents** (G0A, G1A, G1B, G2A, R1A, S0A; models opus, sonnet), joined at d0:
  - `G0A.TRIMMED` = `SORT 1 DROP -1 DROP G0A.MEAN` — mean after dropping one smallest and one largest
  - `G1A.TRIMMED` = `SORT 1 DROP -1 DROP G0A.MEAN` — mean after removing one smallest and one largest number
  - `G1B.TRIMMED` = `SORT 1 DROP -1 DROP G0A.MEAN` — mean after dropping one min and one max
  - `G2A.TRIMMED` = `SORT 1 DROP -1 DROP G0A.MEAN` — mean after dropping one smallest and one largest
  - `R1A.TRIMMED` = `SORT 1 DROP -1 DROP X1.GISUZU` — mean after dropping one smallest and one largest
  - `S0A.TRIMMED` = `SORT 1 DROP -1 DROP S0A.MEAN` — mean after dropping one smallest and one largest
- **4 agents** (G0A, G0B, S0A, S0B; models opus, sonnet), joined at d0:
  - `G0A.SUM` = `0 [ ADD ] FOLD` — sum of a numeric Vector (0 for empty)
  - `X1.RUKUTA` = `0 [ ADD ] FOLD`
  - `X2.TUZIGA` = `0 [ ADD ] FOLD`
  - `G0B.SUM` = `0 [ ADD ] FOLD` — sum of a Vector
  - `S0A.SUM` = `0 [ ADD ] FOLD` — sum of a Vector of numbers (0 for empty)
  - `S0B.SUM` = `0 [ ADD ] FOLD` — Sum of a Vector.
- **3 agents** (G0A, G0B, S0A; models opus, sonnet), joined at d0:
  - `G0A.MEAN` = `'V' BIND V G0A.SUM V LENGTH DIV` — arithmetic mean of a numeric Vector
  - `X1.GISUZU` = `'V' BIND V X1.RUKUTA V LENGTH DIV`
  - `X2.NEGEKE` = `'V' BIND V X2.TUZIGA V LENGTH DIV`
  - `G0B.MEAN` = `'V' BIND V G0B.SUM V LENGTH DIV` — mean of a Vector
  - `S0A.MEAN` = `'V' BIND V S0A.SUM V LENGTH DIV` — arithmetic mean of a non-empty Vector of numbers
- **3 agents** (G1B, G2A, R1A; models sonnet, opus), joined at d1:
  - `G1B.DEBTORS` = `G0A.BYNAME 'R' BIND R KEYS R VALUES 2 COLLECT ZIP [ 1 GET 0 LT ] FILTER [ 0 GET ] MAP` — names whose summed amounts are negative
  - `G2A.DEBTORS` = `G0A.BYNAME G2A.PAIRS [ 1 GET 0 LT ] FILTER [ 0 GET ] MAP` — names whose ledger amounts sum below zero, first-appearance order
  - `R1A.DEBTORS` = `X1.RIVINO 'R' BIND R KEYS [ 'K' BIND R K GET 0 LT ] FILTER` — names whose ledger amounts sum below zero, in first-appearance order
- **2 agents** (G0A, S0A; models opus), joined at d0:
  - `G0A.WORDS` = `' ' TOKENIZE` — split a String on single spaces into a Vector of words
  - `X1.VIDIKO` = `' ' TOKENIZE`
  - `X2.VIVIKA` = `' ' TOKENIZE`
  - `S0A.WORDS` = `' ' TOKENIZE` — split a String at single spaces into words
- **2 agents** (G0B, S0A; models sonnet, opus), joined at d1:
  - `G0B.BYNAME` = `ZIP 'Z' BIND Z 0 GET Z 1 GET GROUP 'G' BIND G KEYS G VALUES [ G0B.SUM ] MAP RECORD` — ledger entries to Record name->sum
  - `S0A.BY-NAME` = `S0A.LEDGER-SPLIT GROUP 'G' BIND G KEYS G VALUES [ S0A.SUM ] MAP RECORD` — ledger -> Record name -> summed amount, first-appearance order
- **2 agents** (G0C, S0B; models haiku, sonnet), joined at d1:
  - `G0C.SUMS` = `'LE' BIND LE [ 0 GET ] MAP 'LN' BIND LE [ 1 GET ] MAP 'LA' BIND LN LA GROUP 'LG' BIND LG KEYS LG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — Ledger entries [name amount] to a Record from name to its summed amount, names in first-appearance order.
  - `S0B.SUMS` = `'E' BIND E [ 0 GET ] MAP E [ 1 GET ] MAP GROUP 'R' BIND R KEYS R VALUES [ S0B.SUM ] MAP RECORD` — Record name -> sum of amounts from a Vector of [name amount] entries.
- **2 agents** (G1A, R1A; models opus), joined at d0:
  - `G1A.INITIALS` = `G0A.WORDS [ CHARS LENGTH 3 GT ] FILTER [ CHARS 0 GET UPPER ] MAP JOIN` — upper-case initials of words longer than three letters ('' if none)
  - `R1A.INITIALS` = `X1.VIDIKO [ CHARS LENGTH 3 GT ] FILTER [ CHARS 0 GET UPPER ] MAP JOIN` — upper-case initials of the words longer than three letters in a text
- **2 agents** (G1A, R2A; models opus), joined at d0:
  - `G1A.KEYSWHERE` = `'R' BIND R KEYS R VALUES 2 COLLECT ZIP [ 1 GET 0 LT ] FILTER [ 0 GET ] MAP` — keys of a Record whose numeric value is below zero, in key order
  - `R2A.NEGKEYS` = `'R' BIND R KEYS R VALUES 2 COLLECT ZIP [ 1 GET 0 LT ] FILTER [ 0 GET ] MAP` — keys of a Record whose values are below zero, in key order
- **2 agents** (G1A, G1B; models opus, sonnet), joined at d1:
  - `G1A.LARGEST` = `'E' BIND E E G1A.AMOUNTS G0A.ARGMAX GET` — [ name amount ] entry with the largest amount; first on ties
  - `G1B.LARGEST` = `'V' BIND V V ZIP 1 GET G0A.ARGMAX GET` — entry with the largest amount, first on ties
- **2 agents** (G1B, S0A; models sonnet, opus), joined at d1:
  - `G1B.LONGEST` = `G0A.WORDS 'W' BIND W W [ CHARS LENGTH ] MAP G0A.ARGMAX GET` — longest word of a String, first on ties
  - `S0A.LONGEST` = `S0A.WORDS 'W' BIND W W [ CHARS LENGTH ] MAP S0A.ARGMAX` — longest word of a text, first of ties

## Reuse of inherited Words

| condition | gen | correct solutions reaching one | inherited Words reached, per subject |
| --- | --- | --- | --- |
| bottleneck | 1 | 46/54 | G1A 8, G1B 8, G1C 8 |
| bottleneck | 2 | 45/54 | G2A 8, G2B 8, G2C 8 |
| shuffled | 1 | 44/54 | R1A 8, R1B 7, R1C 8 |
| shuffled | 2 | 45/54 | R2A 8, R2B 8, R2C 8 |

## What subjects ran (cli route)

| subject | compute calls | naming inherited Words | prelude not the lexicon |
| --- | --- | --- | --- |
| G0A | 74 | 0 | 0 |
| G0B | 52 | 0 | 0 |
| G0C | 73 | 0 | 0 |
| G1A | 38 | 38 | 0 |
| G1B | 54 | 54 | 0 |
| G1C | 35 | 35 | 0 |
| G2A | 60 | 60 | 0 |
| G2B | 55 | 55 | 0 |
| G2C | 61 | 61 | 0 |
| R1A | 37 | 37 | 0 |
| R1B | 37 | 37 | 0 |
| R1C | 49 | 49 | 0 |
| R2A | 110 | 110 | 0 |
| R2B | 22 | 22 | 0 |
| R2C | 44 | 44 | 0 |
| S0A | 136 | 0 | 0 |
| S0B | 41 | 0 | 0 |
| S0C | 34 | 0 | 0 |

## H2 — lexicon entries built from other entries

- bottleneck gen1: 3/8
- bottleneck gen2: 3/8
- shuffled gen1: 3/8
- shuffled gen2: 3/8

## H4 — Core Words by number of correct solutions reaching them

`GET` 264 · `BIND` 263 · `FOLD` 164 · `ADD` 145 · `MAP` 141 · `LENGTH` 113 · `TOKENIZE` 108 · `COLLECT` 103 · `KEYS` 91 · `SORT` 90 · `VALUES` 90 · `ZIP` 87 · `ORDER` 85 · `DROP` 82 · `DIV` 72 · `JOIN` 72 · `SUB` 71 · `MUL` 63 · `GROUP` 55 · `RECORD` 54 · `CHARS` 54 · `FILTER` 41 · `MAX` 40 · `FLOOR` 36 · `TALLY` 36 · `UPPER` 36 · `UNIQUE` 35 · `LT` 18 · `GT` 18 · `REVERSE` 18 · `SCAN` 18 · `TRIM` 12 · `FLATTEN` 9 · `EQ` 5 · `TAKE` 5 · `CONCAT` 4 · `SQRT` 1 · `POW` 1 · `PUT` 1

Unused (38): `TRUE` `FALSE` `AND` `NOT` `SELECT` `RECONCILE` `ROUND` `MIN` `GCD` `RATIO` `RANGE` `FILL` `SHAPE` `RESHAPE` `DEPTH` `BSEARCH` `WITHOUT` `HAS?` `MERGE` `LOWER` `SEARCH` `REPLACE` `NUM` `STR` `FORMAT` `JSON-DECODE` `JSON-ENCODE` `EXEC` `CONTRACT` `FAIL` `NIL` `NIL?` `NIL-REASON` `ABSENT` `DEF` `DEL` `DIGEST` `PRINT`

## H5 — correct solutions whose Core-only expansion disagreed

None.
