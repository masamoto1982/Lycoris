# 語彙創発実験 Phase 3 の設計（2026-10-15）

Status: 非正典・`[作業指示書]`。本書は Ajisai の意味論も互換性方針も定義しない。
正典は `spec/` 配下の各ソースと、そこから生成される `SPECIFICATION.html` のみ。

前提文書: `lexicon-emergence-phase2-results-2026-10-15.md`（以下「Phase 2」）、
`lexicon-emergence-difficulty-calibration-2026-10-15.md`（以下「較正」）。
道具: `tools/lexicon-emergence/`（使い方は同ディレクトリの `README.md`）。

**2026-10-15 に実施し、世代 0 で §6 の停止条件（天井）に該当して止めた。** 無実行の 8 体の正答は
262/264 だった。結果は `lexicon-emergence-phase3-results-2026-10-15.md`。世代 1 以降（Q1〜Q5）を
課題を変えて続けるか、このまま走らせるかは所有者が決める。それまで本書は `[作業指示書]` として残す。

---

## 0. 要約

- Phase 2 と較正で、正答率に分散が出るのはコードを走らせない条件だけだとわかった。Phase 3 は
  その条件（無実行、`callBudget: 0`）を主に置き、**継承した辞書が初手の正答率を上げるか**を問う。
- 辞書の選定規則を 2 つ（使用回数順・再発明順）と、容量の制限なし（C-open）を並べ、Phase 2 の
  「1 世代で固定点」が容量のせいか選定規則のせいかを分ける。
- 被験体は Opus・Sonnet・Haiku・Fable の 4 モデル。課題は較正で作った 6 族 33 問。
- 規模は 68 体（無実行 56 体・実行あり 12 体）。世代 0 の 12 体を先に走らせ、§6 の停止条件を
  そこで判定する。
- 基準線として、SKILL.md に `SELECT` の一行（値を丸ごと選ぶには `2 COLLECT` して `GET`）を
  本書と同じ変更で足した。較正の唯一の誤答の原因である。Phase 3 の数値は、Phase 2・較正と
  被験体の読む文書が違う。

## 1. 問い

| 番号 | 問い | 主に見る比較 |
| --- | --- | --- |
| Q1（H6、新設） | 継承した辞書は、無実行の初手の正答率を上げるか | 世代 0（辞書なし）と、uses の世代 1〜3 |
| Q2（H1） | 4 モデルで、辞書を継承しない体の類はどこまで重なるか。同一モデル対と異モデル対で違うか | 世代 0 の 8 体の類の Jaccard |
| Q3 | 再発明順の選定は、使用回数順が取りこぼした語（Phase 2 の CAP・ABS）を辞書に入れ、次の世代の作り直しを減らすか | uses と reinvention |
| Q4 | 容量を外すと、辞書は固定点に達するか。Phase 2 の固定点は容量の結果か | uses と open |
| Q5 | 名前と注記のない辞書でも、無実行の初手で再利用が起き、正答率が保たれるか | uses と shuffled |
| Q6 | SKILL.md に足した `SELECT` の一行で、較正の誤り（レーン選択による `shapeMismatch`）は消えるか | 全条件の誤答の原因 |
| 補 | 実行できる条件では、辞書は正答までの実行回数を減らすか | runs の世代 0〜2 |

H1〜H5 の定義はパイロットの結果の付録 B にある。H6 は本書で置く。

## 2. 固定する設定

| 項目 | 値 |
| --- | --- |
| 経路 | cli 経路（Claude Code の subagent が `cli.mjs compute` で走らせる。無実行の条件では `compute` が拒む） |
| モデル | A Opus・B Sonnet・C Haiku・D Fable（被験体名の末尾の文字で決まる） |
| 課題族 | `runs`・`number`・`brackets`・`sensor`・`orders`・`messy`（33 問） |
| SKILL.md | 本書を入れた変更の時点のもの（21,197 バイト） |
| 容量 K | 8（open は無制限） |
| 実行の上限 | 既定 0。`runs` 条件だけ無制限 |

`design.json`:

```json
{
  "families": ["runs", "number", "brackets", "sensor", "orders", "messy"],
  "models": { "A": "opus", "B": "sonnet", "C": "haiku", "D": "fable" },
  "callBudget": { "default": 0, "runs": null }
}
```

## 3. 条件と世代

| 条件 | 世代 | 被験体 | 辞書 | 実行 |
| --- | --- | --- | --- | --- |
| `solo` | 0 | S0A〜S0D | なし | 不可 |
| `bottleneck` | 0 | G0A〜G0D | なし（世代 1 の 4 条件の辞書の出所） | 不可 |
| `uses` | 1〜3 | U*g*A〜D | 使用回数順、K = 8 | 不可 |
| `reinvention` | 1〜3 | R*g*A〜D | 再発明順、K = 8 | 不可 |
| `open` | 1〜3 | O*g*A〜D | 容量なし | 不可 |
| `shuffled` | 1〜3 | Z*g*A〜D | 使用回数順、K = 8、各世代で全語を改名（`X<g>.`） | 不可 |
| `runs` | 0〜2 | E*g*A〜D | 世代 *g* の `uses` と同じ辞書（世代 0 はなし） | 無制限 |

- 無実行 56 体（4 + 4 + 4 × 3 × 4）、実行あり 12 体、計 68 体。
- 世代 0 の辞書なしの 8 体（solo と bottleneck）が Q1 の基準線であり、Q2 の対象である。
- 4 条件の世代 1 は、同じ bottleneck 世代 0 から `evolve --into` で作る。条件間の差は、選定規則と
  容量と名前だけになる。
- 再発明順（`--rule reinvention`）は、その語の類を自分で定義して正答に使った体の数で先に並べ、
  同数なら使用回数で並べる。使用回数順は Phase 2 と同じ規則である。

## 4. 手順

`<run>` は `phase3-<実施日>`。各段の後に `grade` → `evolve`（・`shuffle`）→ `seal` の順で進める。
`seal` は採点済みの解答・実行記録・作業ファイルを作業木の外（`$LEXICON_STASH`、既定は
`~/.lexicon-stash`）へ移し、辞書だけを残す。後の世代の被験体は前の解答を読めない。

1. 準備: `runs/<run>/design.json` を §2 のとおり置く。共有領域（scratchpad）に解答を含むファイルが
   ないことを確かめる。
2. 世代 0（12 体）: solo・bottleneck・runs の各 4 体。プロンプトは
   `node cli.mjs prompt <run> <cond> 0 <agent> --route cli` の出力をそのまま渡す。
3. 世代 0 の採点と §6 の判定。続けるなら:

   ```sh
   node cli.mjs evolve <run> bottleneck 0 8 --into uses
   node cli.mjs evolve <run> bottleneck 0 8 --rule reinvention --into reinvention
   node cli.mjs evolve <run> bottleneck 0 Infinity --into open
   node cli.mjs shuffle <run> uses 1 shuffled 1
   cp runs/<run>/uses/gen1/lexicon.json runs/<run>/runs/gen1/   # runs 条件は uses の辞書を使う
   node cli.mjs seal <run> solo 0; node cli.mjs seal <run> bottleneck 0; node cli.mjs seal <run> runs 0
   ```

4. 世代 1・2（各 20 体）: uses・reinvention・open・shuffled・runs の各 4 体。採点の後:

   ```sh
   node cli.mjs evolve <run> uses <g> 8
   node cli.mjs evolve <run> reinvention <g> 8 --rule reinvention
   node cli.mjs evolve <run> open <g> Infinity
   node cli.mjs evolve <run> shuffled <g> 8 && node cli.mjs shuffle <run> shuffled <g+1> shuffled <g+1>
   cp runs/<run>/uses/gen<g+1>/lexicon.json runs/<run>/runs/gen<g+1>/   # g = 1 のときだけ
   node cli.mjs seal <run> <cond> <g>   # 5 条件それぞれ
   ```

5. 世代 3（16 体）: uses・reinvention・open・shuffled の各 4 体。採点の後に seal。
6. `node cli.mjs unseal <run>` で全世代を戻し、`node cli.mjs analyze <run>`。
7. 生データを丸ごとコミットし、次のコミットで作業木から消す（Phase 2・較正と同じ）。

各被験体の使用量（トークン・道具の呼び出し・所要時間）は、subagent の完了通知から記録する。

## 5. 指標

| 指標 | 出所 | 問い |
| --- | --- | --- |
| 世代・条件・モデル別の正答数 | `report.md`「Correct solutions by generation and model」 | Q1・Q3〜Q5 |
| 正答までの実行回数の平均 | 同じ表の mean runs（`calls/<agent>.jsonl` の行数） | 補 |
| 類の Jaccard（同一モデル対・異モデル対） | H1 | Q2 |
| 辞書の語の入れ替わり、辞書外の類を自分で定義した体の数 | 各世代の `lexicon.json`、H1 の類 | Q3・Q4 |
| 継承語に届いた正答の割合 | H4 | Q1・Q5 |
| 参照解と字句まで同じ正答 | 「Correct solutions identical to the reference」 | 汚染の検出 |
| H2・H5 | `report.md` | 従来どおり |
| 誤答の原因 | 採点結果を手で読む。`SELECT` の `shapeMismatch` は数える | Q6 |

無実行の条件では被験体が `compute` を呼ばないので、prelude のダイジェスト照合は `runs` 条件だけで
意味を持つ。無実行の条件の H5 は、採点器の Core のみへの展開で見る。

## 6. 停止条件

世代 0 の無実行 8 体で判定する（`runs` 条件の 4 体は含めない）。

- 正答が 95% 以上（264 問中 251 問以上）: 無実行でも天井に近く、Q1 の差は測れない。世代 1 へ
  進まずに止め、世代 0 の結果を観察ノートにして報告する。`SELECT` の一行の効果（Q6）は
  この場合にも読める。
- 正答が 30% 未満（80 問未満）: 課題が言語の習得ではなく問題文の読解を測っている恐れがある。
  止めて、誤答を読んで報告する。
- それ以外: 世代 1 へ進む。

世代 1 以降で、ある条件の 4 体のうち 2 体以上が解答を書けなかったとき（書式の誤り、時間切れ）は、
その条件を止めて原因を報告する。

## 7. 汚染の統制

- 各世代の採点・選定の直後に `seal` する。作業木に残るのは辞書と `design.json` だけになる。
- 共有領域（scratchpad）に被験体が書いたものは、各段の後に作業木と共有領域の外へ移す。参照解の
  下書きは作業木と共有領域の外に置く。
- Phase 2・較正の生データは git 履歴にしかなく、作業木にはない。被験体が git を読むことは禁止
  するが、技術的には防いでいない。各被験体の道具の呼び出し回数を記録し、多いものは記録を読む。
- 参照解と字句まで同じ正答は、汚染の兆候として個別に読む（同じ解に独立に至ることもあるので、
  それだけで汚染とはしない）。
- 被験体が `ajisai` バイナリや MCP ツールで走らせることは禁止する。無実行の条件で `compute` が
  拒むのは cli 経路だけである（較正 §3 と同じ限界）。

## 8. 規模と所有者が決めること

Phase 2 と較正の subagent は、1 体あたり 9.5 万〜21 万トークンを使った（較正の無実行の 4 体は
11 万〜15 万、課題は 15 問）。Phase 3 は 1 体 33 問なので 15 万〜25 万と見込み、68 体で
1,000 万〜1,700 万トークンになる。世代 0 で止まれば 12 体で済む。

1. 規模: 全 68 体で進めるか。縮める場合の順は、`runs` 条件（12 体）を外す → 世代 3（16 体）を
   外す → `open` を外す。
2. 課題族: 6 族 33 問とするか。較正で無実行の差が出たのは `runs`・`number`・`brackets` の 3 族で、
   残る 3 族は無実行では未確認である。3 族（15 問）に絞ると 1 体の負荷は半分以下になるが、
   天井に当たる恐れは増す。
3. 実施の時期。

## 9. 完了の定義

- 結果を `docs/dev/lexicon-emergence-phase3-results-<実施日>.md`（`[観察ノート]`）にまとめ、
  Q1〜Q6 と補のそれぞれに、答え・答えられなかった理由のどちらかを書く。
- `tools/lexicon-emergence/README.md` に Phase 3 の結果ノートを足す。
- 本書は、結果ノートに設計の要点を移したうえで削除する。

## 10. 本書と同じ変更で入れた道具

| 変更 | 場所 |
| --- | --- |
| SKILL.md §3 に、`SELECT` で 2 つの Vector を丸ごと選べないことと、`2 COLLECT … GET` の書き方（実行して確かめる canonical example `select-whole`） | `scripts/generate-skill-md.mjs` |
| 選定規則 `uses`・`reinvention`（`evolve --rule`、`design.json` の `selectionRule`） | `lib/evolve.mjs`・`cli.mjs` |
| 別の条件の次の世代へ辞書を書く `evolve --into`。K に `Infinity` | `cli.mjs` |
| 条件ごとの実行の上限（`callBudget` にオブジェクト、`null` は無制限） | `cli.mjs` |
| `seal`・`unseal` | `cli.mjs` |
| 世代・モデル別の正答と平均実行回数の表、参照解と字句まで同じ正答の表 | `lib/analyze.mjs` |
| 上の道具のテスト | `test/phase3-tooling.test.mjs` |
