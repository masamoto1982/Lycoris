# Lexicon-emergence run report

## Per generation

| condition | gen | agents | lexicon | correct | mean tokens | tokens ÷ expanded | Core-equivalent (H5) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| bottleneck | 0 | 4 | 0 | 132/132 | 19.4 | 0.541 | 132/132 |
| runs | 0 | 4 | 0 | 132/132 | 13.7 | 0.383 | 132/132 |
| solo | 0 | 4 | 0 | 130/132 | 10.9 | 0.304 | 130/130 |

## Correct solutions by generation and model

| condition | gen | opus | sonnet | haiku | fable | mean runs |
| --- | --- | --- | --- | --- | --- | --- |
| bottleneck | 0 | 33/33 | 33/33 | 33/33 | 33/33 | — |
| runs | 0 | 33/33 | 33/33 | 33/33 | 33/33 | 221.3 |
| solo | 0 | 33/33 | 33/33 | 32/33 | 32/33 | — |

## Correct solutions identical to the reference

- G0B: 1 (orders.unpriced)
- G0D: 2 (sensor.present_mean, sensor.filled)
- E0A: 1 (sensor.ratios)
- E0B: 2 (sensor.filled, sensor.ratios)
- E0C: 3 (number.divisors, sensor.present_mean, sensor.ratios)
- S0B: 1 (orders.unpriced)
- S0C: 1 (sensor.ratios)

## Accuracy per model

| model | agents | correct |
| --- | --- | --- |
| opus | 3 | 99/99 |
| sonnet | 3 | 99/99 |
| haiku | 3 | 98/99 |
| fable | 3 | 98/99 |

## H1 — classes defined independently by two or more agents (28 of 92 classes, 195 Words; 24 reached from two or more models)

Baseline over subjects that inherited nothing — mean Jaccard of defined classes: same-model pairs 0.188, cross-model pairs 0.158.

- **12 agents** (G0A, G0B, G0C, G0D, E0A, E0B, E0C, E0D, S0A, S0B, S0C, S0D; models opus, sonnet, haiku, fable), joined at d1/d0:
  - `G0A.WORDS` = `' ' TOKENIZE [ CHARS LENGTH 0 GT ] FILTER` — space-separated String -> Vector of its non-empty words
  - `G0A.LWORDS` = `G0A.WORDS [ LOWER ] MAP` — space-separated String -> Vector of its words in lower case
  - `G0B.WORDS` = `LOWER ' ' TOKENIZE [ '' EQ NOT ] FILTER` — String in, Vector of its lower-case words (empty pieces dropped).
  - `G0C.WORDS` = `' ' TOKENIZE [ 'MW' BIND MW '' EQ NOT ] FILTER` — Words of a space-separated String as a Vector, with empty pieces dropped.
  - `G0D.WORDS` = `' ' TOKENIZE [ '' EQ NOT ] FILTER` — space-separated String -> Vector of its non-empty words
  - `G0D.LOWER-WORDS` = `G0D.WORDS [ LOWER ] MAP` — space-separated String -> Vector of its words in lower case
  - `E0A.WORDS` = `' ' TOKENIZE [ '' EQ NOT ] FILTER` — String -> Vector of its space-separated words, empty pieces dropped
  - `E0B.WORDS` = `LOWER ' ' TOKENIZE [ '' EQ NOT ] FILTER` — String -> Vector of its words in lower case, ignoring extra spaces
  - `E0C.WORDS` = `' ' TOKENIZE [ '' EQ NOT ] FILTER` — Splits a String on spaces and drops the empty pieces: the words in order.
  - `E0D.WORDS` = `' ' TOKENIZE [ '' EQ NOT ] FILTER` — String -> its space-separated words (empty tokens dropped)
  - `S0A.WORDS` = `' ' TOKENIZE [ '' EQ NOT ] FILTER` — String -> Vector of its space-separated non-empty words
  - `S0B.WORDS` = `' ' TOKENIZE [ CHARS LENGTH 0 GT ] FILTER` — String -> Vector of its non-empty space-separated words.
  - `S0C.WORDS` = `' ' TOKENIZE [ '' EQ NOT ] FILTER` — Input String; answers its nonempty space-separated words as a Vector of Strings.
  - `S0D.WORDS` = `' ' TOKENIZE [ '' EQ NOT ] FILTER` — split a String on spaces, dropping empty tokens
  - `S0D.LWORDS` = `S0D.WORDS [ LOWER ] MAP` — the words of a String, lower-cased
- **10 agents** (G0A, G0B, E0A, E0B, E0C, E0D, S0A, S0B, S0C, S0D; models opus, sonnet, haiku, fable), joined at d0/d1/d0a:
  - `G0A.UNWORDS` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of space-free words -> one String joined by single spaces
  - `G0B.SPACEJOIN` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of Strings in, one String with single spaces between them.
  - `E0A.UNWORDS` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of words -> one String joined by single spaces
  - `E0B.JOINSP` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of Strings -> one String joined with single spaces
  - `E0C.SPJOIN` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Joins a Vector of Strings with single spaces, no leading or trailing space.
  - `E0D.UNWORDS` = `[ 'X' BIND ' ' X 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of Strings -> joined with single spaces
  - `S0A.JOINSP` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of space-free Strings -> joined by single spaces
  - `S0B.JOINSP` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — Vector of Strings -> one String joined by single spaces.
  - `S0C.JOINSP` = `[ 'WW' BIND ' ' WW 2 COLLECT JOIN ] MAP JOIN TRIM` — Input Vector of Strings; answers them joined by single spaces with no leading or trailing space.
  - `S0D.SPACEJOIN` = `[ ' ' 2 COLLECT JOIN ] MAP JOIN TRIM` — join a Vector of Strings with single spaces
- **9 agents** (G0A, G0C, G0D, E0A, E0B, E0C, E0D, S0A, S0D; models opus, haiku, fable, sonnet), joined at d1/d0/d0a:
  - `G0A.REVENUE` = `JSON-DECODE 'orders' GET [ 'G0A.RO' BIND G0A.RO 'price' GET 0 2 COLLECT 0 1 G0A.RO 'price' HAS? SELECT GET G0A.RO 'qty' GET MUL ] MAP 0 [ ADD ] FOLD` — orders JSON String -> total qty*price over priced orders
  - `G0A.QTYBYSKU` = `JSON-DECODE 'orders' GET 'G0A.KO' BIND G0A.KO [ 'sku' GET ] MAP G0A.KO [ 'qty' GET ] MAP GROUP 'G0A.KG' BIND G0A.KG KEYS G0A.KG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — orders JSON String -> Record sku -> summed qty, first-appearance order
  - `G0C.ORDERS` = `JSON-DECODE 'orders' GET` — Parse the orders JSON String into the Vector of order Records.
  - `G0D.ORDERS` = `JSON-DECODE 'orders' GET` — orders JSON String -> Vector of order Records
  - `G0D.REVENUE` = `G0D.ORDERS [ 'price' HAS? ] FILTER [ 'O' BIND O 'qty' GET O 'price' GET MUL ] MAP 0 [ ADD ] FOLD` — orders JSON String -> total qty*price over priced orders
  - `G0D.QTY-BY-SKU` = `G0D.ORDERS 'QO' BIND QO [ 'sku' GET ] MAP QO [ 'qty' GET ] MAP GROUP 'QG' BIND QG KEYS QG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — orders JSON String -> Record sku -> summed qty, skus in first-appearance order
  - `E0A.ORDERS` = `JSON-DECODE 'orders' GET` — JSON String -> Vector of the order Records under its 'orders' key
  - `E0A.REVENUE` = `E0A.ORDERS [ E0A.LINE ] MAP E0A.SUM` — orders JSON String -> total qty*price over priced orders
  - `E0A.QTYBYSKU` = `E0A.ORDERS 'O' BIND O [ 'sku' GET ] MAP O [ 'qty' GET ] MAP GROUP 'G' BIND G KEYS G VALUES [ E0A.SUM ] MAP RECORD` — orders JSON String -> Record sku -> summed qty, skus in first-appearance order
  - `E0B.QTYSKU` = `JSON-DECODE 'orders' GET 'OS' BIND OS [ 'sku' GET ] MAP OS [ 'qty' GET ] MAP GROUP 'G' BIND G KEYS G VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — Orders JSON String -> Record sku -> summed qty, in first-appearance order
  - `E0C.ORDERS` = `JSON-DECODE 'orders' GET` — Parses the JSON text and answers its orders Vector.
  - `E0D.ORDERS` = `JSON-DECODE 'orders' GET` — JSON String with an 'orders' array -> the Vector of order Records
  - `E0D.REVENUE` = `E0D.ORDERS [ 'O' BIND O 'qty' GET O 'price' GET MUL 0 O 'price' HAS? SELECT ] MAP E0D.SUM` — orders JSON String -> total qty*price over orders that have a price
  - `E0D.QTY-BY-SKU` = `E0D.ORDERS 'O' BIND O [ 'sku' GET ] MAP O [ 'qty' GET ] MAP GROUP 'G' BIND G KEYS G VALUES [ E0D.SUM ] MAP RECORD` — orders JSON String -> Record sku -> summed qty, skus in order of first appearance
  - `S0A.ORDERS` = `JSON-DECODE 'orders' GET` — JSON String -> Vector of order Records
  - `S0A.REVENUE` = `S0A.ORDERS [ S0A.LINEREV ] MAP 0 [ ADD ] FOLD` — orders JSON String -> total qty*price over priced orders
  - `S0A.QTYBYSKU` = `S0A.ORDERS 'QO' BIND QO [ 'sku' GET ] MAP QO [ 'qty' GET ] MAP GROUP 'QG' BIND QG KEYS QG VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — orders JSON String -> Record sku -> summed qty
  - `S0A.TAGCOUNTS` = `S0A.ORDERS [ 'tags' GET NIL? NOT ] FILTER [ 'tags' GET UNIQUE ] MAP FLATTEN TALLY` — orders JSON String -> Record tag -> number of orders carrying it
  - `S0D.ORDERS` = `JSON-DECODE 'orders' GET` — JSON String -> Vector of order Records
  - `S0D.REVENUE` = `S0D.ORDERS [ 'O' BIND O 'qty' GET 0 O 'price' GET 'P' BIND P P NIL? SELECT MUL ] MAP 0 [ ADD ] FOLD` — sum of qty*price over priced orders
  - `S0D.QTYBYSKU` = `S0D.ORDERS 'O' BIND O [ 'sku' GET ] MAP O [ 'qty' GET ] MAP GROUP 'G' BIND G KEYS G VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — Record sku -> total qty, skus in order of first appearance
  - `S0D.TAGCOUNTS` = `S0D.ORDERS [ 'O' BIND [ ] O 'tags' GET 2 COLLECT 1 0 O 'tags' HAS? SELECT GET UNIQUE ] MAP FLATTEN TALLY` — Record tag -> number of orders carrying it
- **7 agents** (G0A, G0D, E0A, E0D, S0A, S0C, S0D; models opus, fable, haiku), joined at d1/d0/d0a:
  - `G0A.DIVISORS` = `'G0A.DN' BIND 1 G0A.DN RANGE [ 'G0A.DD' BIND G0A.DN G0A.DN G0A.DD DIV FLOOR G0A.DD MUL SUB 0 EQ ] FILTER` — positive integer -> Vector of its positive divisors, ascending
  - `G0D.DIVISORS` = `'DK' BIND 1 DK RANGE [ 'DD' BIND DK DD DIV FLOOR DD MUL DK EQ ] FILTER` — positive integer -> ascending Vector of its positive divisors
  - `E0A.DIVISORS` = `'N' BIND 1 N RANGE [ 'D' BIND N D E0A.MOD 0 EQ ] FILTER` — positive integer -> Vector of its positive divisors, ascending
  - `E0D.DIVISORS` = `'N' BIND 1 N RANGE [ 'D' BIND N D E0D.MOD 0 EQ ] FILTER` — positive integer -> its positive divisors ascending
  - `S0A.DIVISORS` = `'DN' BIND 1 DN RANGE [ 'DD' BIND DN DD S0A.MOD 0 EQ ] FILTER` — positive integer -> its positive divisors ascending
  - `S0C.DIVS` = `'DV' BIND 1 DV RANGE [ 'DD' BIND DV DD DIV FLOOR DD MUL DV EQ ] FILTER` — Input positive integer; answers its ascending positive divisors as a Vector.
  - `S0D.DIVISORS` = `'K' BIND 1 K RANGE [ 'D' BIND K D S0D.MOD 0 EQ ] FILTER` — ascending positive divisors of a positive integer
- **7 agents** (G0B, E0A, E0B, S0A, S0B, S0C, S0D; models sonnet, opus, haiku, fable), joined at d1/d0:
  - `G0B.REDUCE` = `'()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE` — String of brackets in, the String left after deleting adjacent () and [] pairs six times (enough for 12 chars).
  - `E0A.REDUCE` = `'S' BIND 0 S CHARS LENGTH RANGE S [ 'I' BIND '()' '' REPLACE '[]' '' REPLACE ] FOLD` — bracket String -> what remains after repeatedly deleting adjacent '()' and '[]' pairs
  - `E0B.REDUCE` = `'S' BIND 0 6 RANGE S [ 'E' BIND '()' '' REPLACE '[]' '' REPLACE ] FOLD` — Bracket String -> String left after repeatedly deleting adjacent () and [] pairs
  - `S0A.BREDUCE` = `'()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE` — bracket String (<= 12 chars) -> what remains after deleting matched adjacent pairs repeatedly
  - `S0B.REDUCED` = `'()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE '()' '' REPLACE '[]' '' REPLACE` — String of ()[] -> String after 6 passes deleting adjacent () and [] pairs.
  - `S0C.REDUCE` = `'RS' BIND 1 6 RANGE RS [ 'IG' BIND '()' '' REPLACE '[]' '' REPLACE ] FOLD` — Input String of brackets; six rounds of deleting adjacent matched round and square pairs, answering what remains (the reduced form is unique).
  - `S0D.BREDUCE1` = `'()' '' REPLACE '[]' '' REPLACE` — one pass deleting adjacent matched pairs () and []
  - `S0D.BREDUCE` = `S0D.BREDUCE1 S0D.BREDUCE1 S0D.BREDUCE1 S0D.BREDUCE1 S0D.BREDUCE1 S0D.BREDUCE1` — delete adjacent matched pairs until none remain (up to 12 chars)
- **5 agents** (G0D, E0A, E0D, S0A, S0D; models fable, opus), joined at d0a/d0:
  - `G0D.MOD` = `'MB' BIND 'MA' BIND MA MA MB DIV FLOOR MB MUL SUB` — a b G0D.MOD -> a modulo b (a - b*floor(a/b))
  - `E0A.MOD` = `'B' BIND 'A' BIND A A B DIV FLOOR B MUL SUB` — a b -> a mod b (floored remainder); a may be a Vector
  - `E0D.MOD` = `'B' BIND 'A' BIND A A B DIV FLOOR B MUL SUB` — a b -> a modulo b (floor remainder)
  - `S0A.MOD` = `'MB' BIND 'MA' BIND MA MA MB DIV FLOOR MB MUL SUB` — a b -> a mod b (floor remainder), lifts over vectors
  - `S0D.MOD` = `'B' BIND 'A' BIND A A B DIV FLOOR B MUL SUB` — a b -> a mod b (lifts over a Vector a)
- **4 agents** (G0B, E0D, S0A, S0B; models sonnet, fable, opus), joined at d1/d0a:
  - `G0B.TITLECASE` = `'TW' BIND TW CHARS 0 GET UPPER TW CHARS 1 DROP JOIN 2 COLLECT JOIN` — Lower-case word in, same word with first letter upper-cased.
  - `E0D.CAPITALIZE` = `CHARS 'CS' BIND CS 1 TAKE JOIN UPPER CS 1 DROP JOIN LOWER 2 COLLECT JOIN` — non-empty word -> first letter upper case, rest lower case
  - `S0A.CAP` = `CHARS 'TC' BIND TC 1 TAKE JOIN UPPER TC 1 DROP JOIN LOWER 2 COLLECT JOIN` — non-empty word -> first letter upper, rest lower
  - `S0B.CAP` = `LOWER CHARS 'CC' BIND CC 0 CC 0 GET UPPER PUT JOIN` — Word -> first letter upper, rest lower.
- **4 agents** (G0D, E0D, S0A, S0D; models fable, opus), joined at d1/d0a:
  - `G0D.PRIME` = `G0D.DIVISORS LENGTH 2 EQ` — positive integer -> TRUE when prime
  - `E0D.PRIME?` = `E0D.DIVISORS LENGTH 2 EQ` — positive integer -> TRUE when it is prime
  - `S0A.ISPRIME` = `'PK' BIND PK 1 PK RANGE S0A.MOD [ 0 EQ ] FILTER LENGTH 2 EQ` — positive integer -> TRUE when prime
  - `S0D.ISPRIME` = `S0D.DIVISORS LENGTH 2 EQ` — TRUE when a positive integer is prime
- **4 agents** (E0A, E0D, S0A, S0D; models opus, fable), joined at d0/d1/d0a:
  - `E0A.PRESENT` = `[ NIL? NOT ] FILTER` — Vector -> the elements that are not NIL, in order
  - `E0D.PRESENT` = `[ NIL? NOT ] FILTER` — vector -> only its non-NIL elements, in order
  - `E0D.FILLED` = `'V' BIND V V E0D.PRESENT 0 GET [ 'X' BIND 'A' BIND A X X NIL? SELECT ] SCAN` — vector with NILs (some present) -> NILs replaced by the nearest earlier present value (first present for leading NILs)
  - `S0A.PRESENT` = `[ NIL? NOT ] FILTER` — vector -> its non-NIL elements
  - `S0A.FILLED` = `'FV' BIND FV FV S0A.PRESENT 0 GET [ 'FE' BIND 'FA' BIND FA FE FE NIL? SELECT ] SCAN` — readings with NILs -> forward-filled, leading NILs take first present
  - `S0D.DEDUPE` = `'V' BIND V V S0D.STARTS GET` — collapse each run of equal adjacent values to one value
  - `S0D.PRESENT` = `[ NIL? NOT ] FILTER` — the non-NIL elements of a Vector
  - `S0D.FILLED` = `'V' BIND V V S0D.PRESENT 0 GET [ 'X' BIND 'A' BIND A X X NIL? SELECT ] SCAN` — forward-fill NILs with the nearest earlier present value (first present value before it)
- **3 agents** (G0A, G0D, E0C; models opus, fable, haiku), joined at d1:
  - `G0A.BSCAN` = `CHARS '#' [ G0A.BSTEP ] SCAN` — bracket String -> Vector of matching stacks after each char; '#' marks a balanced prefix
  - `G0D.BRACKET-DELTAS` = `CHARS [ 'DC' BIND [ '(' '[' ')' ']' ] [ 1 1 -1 -1 ] RECORD DC GET ] MAP` — bracket String -> Vector of +1 per opener and -1 per closer
  - `E0C.CODES` = `CHARS [ 'C' BIND [ 1 -1 2 -2 ] '()[]' C SEARCH GET ] MAP` — Bracket String to a Vector of codes: ( = 1, ) = -1, [ = 2, ] = -2.
  - `E0C.STEPS` = `E0C.CODES [ 'C' BIND 1 -1 C 0 GT SELECT ] MAP` — Bracket String to a Vector of +1 (open) and -1 (close) per character.
- **3 agents** (E0A, E0D, S0B; models opus, fable, sonnet), joined at d0:
  - `E0A.SUM` = `0 [ ADD ] FOLD` — Vector of numbers -> their sum (0 for empty)
  - `E0D.SUM` = `0 [ ADD ] FOLD` — vector -> sum of its numbers (0 for empty)
  - `S0B.SUM` = `0 [ ADD ] FOLD` — Sum of a Vector of numbers (0 when empty).
- **3 agents** (E0A, S0A, S0B; models opus, sonnet), joined at d1:
  - `E0A.BALANCED` = `E0A.REDUCE '' EQ` — bracket String -> TRUE when every ( ) [ ] is properly matched and nested
  - `S0A.BALANCED` = `S0A.BREDUCE '' EQ` — bracket String -> TRUE when properly nested and matched
  - `S0B.ISBAL` = `S0B.REDUCED CHARS LENGTH 0 EQ` — String of ()[] -> TRUE when properly nested.
- **3 agents** (E0D, S0A, S0D; models fable, opus), joined at d1:
  - `E0D.DEPTH` = `CHARS [ 'C' BIND 1 -1 C E0D.OPEN? SELECT ] MAP 0 [ ADD ] SCAN 0 [ MAX ] FOLD` — bracket String -> greatest nesting depth (max running open-minus-close count, at least 0)
  - `S0A.BDEPTH` = `CHARS [ S0A.BDELTA ] MAP 0 [ ADD 0 MAX ] SCAN 0 [ MAX ] FOLD` — bracket String -> greatest number of brackets open at once
  - `S0A.BPREFIX` = `CHARS 'PC' BIND 0 PC LENGTH RANGE [ 'PL' BIND PC PL TAKE JOIN S0A.BALANCED ] FILTER 0 [ MAX ] FOLD` — bracket String -> length of the longest balanced prefix
  - `S0D.BALPREFIX` = `CHARS 'C' BIND 0 C LENGTH RANGE [ 'K' BIND C K TAKE JOIN S0D.BREDUCE CHARS LENGTH 0 EQ ] FILTER -1 GET` — length of the longest balanced prefix of a bracket String
- **3 agents** (E0D, S0A, S0D; models fable, opus), joined at d1/d0a:
  - `E0D.SCALED` = `'V' BIND V E0D.PRESENT 'P' BIND V P P 0 GET [ MAX ] FOLD DIV` — vector with NILs (some present) -> each present value divided by the largest present value; NILs stay NIL
  - `S0A.SCALED` = `'CV' BIND CV S0A.PRESENT 'CP' BIND CP CP 0 GET [ MAX ] FOLD 'MX' BIND CV [ 'SE' BIND NIL 0 SE SE NIL? SELECT MX DIV SE NIL? SELECT ] MAP` — readings with NILs -> each present reading over the largest present, NIL kept
  - `S0D.SCALED` = `'V' BIND V S0D.PRESENT 'P' BIND P P 0 GET [ MAX ] FOLD 'M' BIND V [ 'X' BIND NIL 0 X X NIL? SELECT M DIV X NIL? SELECT ] MAP` — each present element divided by the largest present element; NILs stay NIL
- **2 agents** (G0A, E0C; models opus, haiku), joined at d1:
  - `G0A.RUNS` = `'G0A.RV' BIND G0A.RV G0A.RV 1 TAKE G0A.RV -1 DROP CONCAT SUB 'G0A.RD' BIND G0A.RD G0A.RD MUL 1 MIN 0 [ ADD ] SCAN G0A.RV GROUP VALUES` — integer Vector -> Vector of its maximal runs of equal adjacent values, each a Vector
  - `E0C.RUNS` = `'V' BIND V 1 DROP V -1 DROP SUB 'D' BIND [ TRUE ] D D MUL 0 GT CONCAT 'S' BIND 1 0 S SELECT 0 [ ADD ] SCAN 'ID' BIND ID V GROUP VALUES` — Run-length groups of a Vector: one Vector per maximal run of equal adjacent values, in order.
- **2 agents** (G0B, S0B; models sonnet), joined at d1:
  - `G0B.FILLSTEP` = `'FE' BIND 'FA' BIND FA FE FE NIL? SELECT` — acc elem in: elem if present, else acc (for SCAN forward fill).
  - `S0B.FILL1` = `'FE' BIND FE FE NIL? SELECT` — acc and element -> acc if element is NIL else element (SCAN block for forward fill).
- **2 agents** (G0D, S0A; models fable, opus), joined at d1:
  - `G0D.GAPS` = `[ 'GE' BIND 1 0 GE NIL? SELECT ] MAP G0D.RUNS [ 0 GET 1 EQ ] FILTER` — Vector with NILs -> Vector of the runs of consecutive NILs (each run a Vector of 1s)
  - `S0A.GAPS` = `[ 'GE' BIND 'x' 'o' GE NIL? SELECT ] MAP JOIN 'o' TOKENIZE [ '' EQ NOT ] FILTER` — readings with NILs -> one String of x's per run of missing readings
- **2 agents** (E0A, S0D; models opus, fable), joined at d0:
  - `E0A.ARGMAX` = `-1 MUL ORDER 0 GET` — non-empty Vector of numbers -> index of the first occurrence of its largest value
  - `S0D.ARGMAX` = `-1 MUL ORDER 0 GET` — index of the first largest element of a numeric Vector
- **2 agents** (E0C, S0B; models haiku, sonnet), joined at d1:
  - `E0C.SKUQTY` = `'OS' BIND OS [ 'sku' GET ] MAP OS [ 'qty' GET ] MAP GROUP 'G' BIND G KEYS G VALUES [ 0 [ ADD ] FOLD ] MAP RECORD` — From an orders Vector: a Record from each sku to its total qty, in first-appearance order.
  - `S0B.REVENUE` = `[ S0B.LINE ] MAP S0B.SUM` — Vector of order Records -> total revenue.
  - `S0B.BYSKU` = `'BO' BIND BO [ 'sku' GET ] MAP BO [ 'qty' GET ] MAP GROUP 'BG' BIND BG KEYS BG VALUES [ S0B.SUM ] MAP RECORD` — Vector of order Records -> Record sku -> summed qty.
- **2 agents** (E0D, S0A; models fable, opus), joined at d1:
  - `E0D.MEAN` = `'P' BIND P E0D.SUM P LENGTH DIV` — vector of numbers -> arithmetic mean
  - `S0A.MEAN` = `'MP' BIND MP 0 [ ADD ] FOLD MP LENGTH DIV` — non-empty numeric vector -> arithmetic mean
- **2 agents** (E0D, S0D; models fable), joined at d1:
  - `E0D.IOTA` = `[ 'X' BIND 1 ] MAP 0 [ ADD ] SCAN 1 SUB` — vector -> [ 0 .. n-1 ], its indices (empty for empty)
  - `S0D.STARTS` = `'V' BIND V LENGTH 'N' BIND [ 1 ] V 1 DROP V -1 DROP SUB CONCAT [ 0 EQ NOT ] MAP 'F' BIND 0 N 1 SUB RANGE -1 F SELECT [ -1 GT ] FILTER` — indices where a maximal run of equal adjacent values begins
- **2 agents** (E0D, S0D; models fable), joined at d1:
  - `E0D.RLE` = `'V' BIND V E0D.DEDUPE V E0D.RUNLENS 2 COLLECT ZIP` — vector (n>=1) -> run-length encoding as [ [ value count ] ... ]
  - `S0D.RLE` = `'V' BIND V S0D.STARTS 'S' BIND V S GET S 1 DROP V LENGTH 1 COLLECT CONCAT S SUB 2 COLLECT ZIP` — run-length encoding: Vector of [ value count ] pairs
- **2 agents** (E0D, S0B; models fable, sonnet), joined at d1:
  - `E0D.LONGEST-TRUE` = `0 [ 'T' BIND 'A' BIND A 1 ADD 0 T SELECT ] SCAN 0 [ MAX ] FOLD` — vector of truths -> length of the longest run of consecutive TRUE (0 if none)
  - `E0D.TRUE-RUNS` = `[ 0 FALSE ] [ 'T' BIND [ 'C' 'P' ] BIND C 1 ADD C T P NOT AND SELECT T 2 COLLECT ] FOLD 0 GET` — vector of truths -> number of maximal runs of consecutive TRUE
  - `S0B.WHERE` = `'WF' BIND 0 WF LENGTH 1 SUB RANGE -1 WF SELECT [ -1 GT ] FILTER` — Vector of booleans (length>=1) -> 0-based indices of the TRUE lanes.
- **2 agents** (E0D, S0D; models fable), joined at d1:
  - `E0D.FIB-INDEX` = `'N' BIND 1 12 RANGE [ 0 1 ] [ 'X' BIND [ 'A' 'B' ] BIND B A B ADD 2 COLLECT ] SCAN [ 0 GET ] MAP [ N LT ] FILTER LENGTH 1 ADD` — positive integer n <= 144 -> smallest k with F(k) >= n (F(1)=F(2)=1)
  - `S0D.FIBINDEX` = `'N' BIND [ 1 1 2 3 5 8 13 21 34 55 89 ] [ N LT ] FILTER LENGTH 1 ADD` — smallest k with F(k) >= n, for n <= 89
- **2 agents** (S0A, S0D; models opus, fable), joined at d1:
  - `S0A.UPRUNS` = `'UV' BIND 0 UV LENGTH 1 SUB RANGE 'UI' BIND UV UV UI 1 SUB GET GT 0 FALSE PUT 0 [ 'UF' BIND 'UA' BIND UA 1 ADD 1 UF SELECT ] SCAN 0 [ MAX ] FOLD` — numeric vector -> length of longest strictly increasing adjacent stretch
  - `S0D.UPRUNS` = `'V' BIND 1 0 V 1 DROP V -1 DROP SUB 0 GT SELECT 1 [ MUL 1 ADD ] SCAN 1 [ MAX ] FOLD` — length of the longest strictly increasing stretch of adjacent values
- **2 agents** (S0A, S0D; models opus, fable), joined at d1:
  - `S0A.MAJORITY` = `'MV' BIND MV TALLY 'MT' BIND MT VALUES ORDER -1 GET 'MI' BIND MT KEYS MI GET NIL MT VALUES MI GET 2 MUL MV LENGTH GT SELECT` — vector -> value occurring in more than half the positions, or NIL
  - `S0D.MAJORITY` = `'V' BIND V TALLY 'T' BIND T VALUES 'C' BIND C ORDER -1 GET 'I' BIND T KEYS I GET NIL C I GET 2 MUL V LENGTH GT SELECT` — value occurring in more than half the positions, else NIL
- **2 agents** (S0A, S0C; models opus, haiku), joined at d0a:
  - `S0A.BDELTA` = `'BC' BIND 1 -1 BC '(' EQ NOT BC '[' EQ NOT AND NOT SELECT` — bracket character -> 1 for an opener, -1 for a closer
  - `S0C.DELTA` = `'DC' BIND 1 -1 DC '(' EQ NOT DC '[' EQ NOT AND NOT SELECT` — Input one bracket character; answers 1 for an opening bracket and -1 for a closing one.
- **2 agents** (S0A, S0B; models opus, sonnet), joined at d1:
  - `S0A.LINEREV` = `'LO' BIND LO 'price' GET 'LP' BIND 0 LP LP NIL? SELECT LO 'qty' GET MUL` — order Record -> qty times price, 0 when unpriced
  - `S0B.LINE` = `'LO' BIND LO 'qty' GET LO 'price' GET 0 S0B.NILOR MUL` — Order Record -> qty*price, 0 when no price.

## What subjects ran (cli route)

| subject | compute calls | naming inherited Words | prelude not the lexicon |
| --- | --- | --- | --- |
| E0A | 223 | 0 | 0 |
| E0B | 202 | 0 | 0 |
| E0C | 337 | 0 | 0 |
| E0D | 123 | 0 | 0 |

## H2 — lexicon entries built from other entries


## H4 — Core Words by number of correct solutions reaching them

`BIND` 323 · `GET` 216 · `FILTER` 214 · `MAP` 198 · `ADD` 164 · `FOLD` 161 · `LENGTH` 151 · `SELECT` 148 · `NOT` 146 · `EQ` 145 · `MUL` 131 · `COLLECT` 124 · `SUB` 99 · `DIV` 95 · `DROP` 87 · `NIL?` 84 · `RANGE` 76 · `TOKENIZE` 75 · `CHARS` 71 · `JSON-DECODE` 71 · `GT` 68 · `SCAN` 68 · `CONCAT` 65 · `JOIN` 65 · `VALUES` 63 · `MAX` 62 · `LOWER` 62 · `FLOOR` 59 · `UNIQUE` 57 · `KEYS` 47 · `TALLY` 38 · `GROUP` 37 · `TRIM` 34 · `HAS?` 33 · `ZIP` 30 · `RECORD` 30 · `TAKE` 29 · `SORT` 27 · `ORDER` 27 · `REPLACE` 25 · `LT` 24 · `AND` 22 · `NIL` 18 · `UPPER` 12 · `FORMAT` 12 · `FLATTEN` 11 · `STR` 10 · `SEARCH` 9 · `POW` 8 · `TRUE` 7 · `FALSE` 6 · `PUT` 6 · `MIN` 5 · `REVERSE` 5 · `NUM` 2

Unused (22): `RECONCILE` `ROUND` `SQRT` `GCD` `RATIO` `FILL` `SHAPE` `RESHAPE` `DEPTH` `BSEARCH` `WITHOUT` `MERGE` `JSON-ENCODE` `EXEC` `CONTRACT` `FAIL` `NIL-REASON` `ABSENT` `DEF` `DEL` `DIGEST` `PRINT`

## H5 — correct solutions whose Core-only expansion disagreed

None.
