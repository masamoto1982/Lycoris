# 語彙創発実験 Phase 5 の設計（2026-10-16）

Status: 非正典・`[作業指示書]`。本書は Ajisai の意味論も互換性方針も定義しない。
正典は `spec/` 配下の各ソースと、そこから生成される `SPECIFICATION.html` のみ。

前提文書: `lexicon-emergence-phase4-results-2026-10-16.md`（以下「Phase 4」）。
道具: `tools/lexicon-emergence/`（使い方は同ディレクトリの `README.md`）。

---

## 0. 要約

- Phase 4 §8 の論点は 4 つ。そのうち 3（`series` に一般語が届かないのは汎化の失敗か、系統の欠落か）には、
  Phase 4 の生データで答えが出た（§1）。**欠落である**。辞書なしの世代 0 でも、`series` の正答のうち、
  同じ被験体が学習課題で使った語を呼んだものは 11/24 しかない。継承語はこの上限をほぼ越えない
  （辞書のある 858 件の held-out 正答のうち、例外は 1 件）。held-out を系統で選び直す必要はなく、
  「継承した ÷ 共有した」を汎化の指標にすればよい。
- 残る 2 つ（1. 辞書の価値を、自前の定義を禁じて測る。2. 族の数で並べる規則と、1 類 1 枠の決まり）を
  Phase 5 で扱う。4（正答率は尽きた）は、定義を禁じた条件で正答率に差が出るかを記録するに留める。
- 条件は 3 つ。`families`（族の数で選ぶ辞書。自前の定義は許す）、`probe`（`families` と同じ世代・
  同じ辞書・同じ課題で、自前の定義を禁じる）、`bare`（辞書なし、自前の定義も禁じる）。
  世代 0 は Phase 4 の `base` の 4 体の提出をそのまま使う。
- 規模は 32 体（`families` と `probe` が世代 1〜3 で各 12 体、`bare` が世代 1・2 で 8 体）、無実行、
  1 体 39〜44 問。見込みは 330 万〜690 万トークン。

## 1. Phase 4 のデータで答えた論点（§8 の 3）

held-out の正答を、族ごとに次の 2 つで数えた（`report.md` の新しい表「Held-out families: shared
with training, and inherited」。Phase 4 の生データに今回の道具をかけて再計算した）。

- 共有: その正答が呼ぶ語のどれかを、**同じ被験体が**学習課題の正答でも呼んだ（自前でも継承でも）。
- 継承: その正答が継承語を 1 つ以上呼んだ。

世代 1〜3 の合計（`base` は世代 0）。各欄は「正答 · 共有 · 継承」:

| 条件 | `ledger` | `messy` | `series` |
| --- | --- | --- | --- |
| base（辞書なし） | 24 · 21 · — | 24 · 22 · — | 24 · 11 · — |
| uses | 72 · 59 · 53 | 72 · 72 · 72 | 72 · 26 · 18 |
| breadth | 72 · 54 · 34 | 71 · 71 · 65 | 72 · 26 · 15 |
| fixed | 71 · 54 · 47 | 71 · 71 · 71 | 70 · 27 · 16 |
| open | 72 · 61 · 61 | 72 · 72 · 72 | 71 · 24 · 25 |

1. `series` は、被験体が自分の語を使えるときでも、学習課題と語を共有しない（共有 11/24、
   入れ替え条件で 24〜27/72）。中央値・移動平均・最頻値は、`stats` などの合計・平均・最大と同じ
   系統に括ったが、部分計算を共有していなかった。系統の割り当ては手で決めたもので、誤っていた。
2. 継承は共有をほぼ越えない。辞書のある条件の held-out 正答 858 件のうち、被験体が学習課題で
   使わなかった継承語を held-out で呼んだのは 1 件（`open` の `series`）だけだった。被験体は、
   学習課題で使った継承語だけを held-out にも持ち込む。
3. したがって「継承 ÷ 共有」が、辞書の汎化の効率になる。容量なしの `open` はどの族でも 1.0 前後、
   `uses` は 0.69〜1.0、`breadth` は 0.58〜0.92 だった。Phase 4 §2.1 の族ごとの差（`series` が低い）は、
   大半がこの上限の差である。

道具はこの表を `report.md` に出す（`lib/analyze.mjs` の `heldOutFamilies`）。Phase 5 は課題の割り当てを
Phase 4 のまま変えず、汎化をこの比で読む。

## 2. 問い

| 番号 | 問い | 主に見る比較 |
| --- | --- | --- |
| Q1（辞書の価値） | 自前の定義を書けないとき、継承した辞書は解答を短くし、労力を減らし、誤りを減らすか | `probe` と `bare`（同じ世代・同じ課題、辞書の有無だけが違う） |
| Q2（置き換え） | 自前の定義を許すと、継承語はどれだけ使われなくなるか | `probe` と `families`（同じ辞書・同じ課題、定義の可否だけが違う） |
| Q3（`families` 規則） | 族の数で選ぶと、族をまたぐ語が辞書に残り、held-out の「継承 ÷ 共有」が上がるか | `families` と Phase 4 の `uses`・`breadth` |
| 補 正答率 | 定義を禁じると、正答率に差が出るか | `bare`・`probe` と、`families`・Phase 4 の世代 0 |

Q1 が Phase 4 §8 の 1 である。Phase 4 では、辞書の有無で解答の長さも労力も変わらなかった。
被験体は辞書がなくても補助語を自分で書き、辞書があるとその一部を継承語に置き換えるだけだったからである。
自前の定義を禁じれば、継承語は部分計算に名前を付ける唯一の手段になり、辞書の価値がそのまま
解答の長さと誤りに出るはずである。

Q3 は Phase 4 の比較と条件をまたぐ（実施日もモデルの割り当ても同じだが、被験体は別）。世代 1 の
辞書は Phase 4 と同じ世代 0 から作るので、その差は選定規則だけによる（§5）。

## 3. 課題族

Phase 4 と同じ割り当てである（Phase 4 付録 B.2）。§1 で答えが出たので、held-out は変えない。

| 系統（Phase 4 の割り当て） | 学習 A | 学習 B | held-out |
| --- | --- | --- | --- |
| 語の処理 | `text`（7） | `prose`（6） | `messy`（6） |
| 数の列 | `stats`（7） | `sensor`（6）、`runs`（5） | `series`（6） |
| キーごとの集計 | `orders`（6） | — | `ledger`（6） |
| アルゴリズム | `number`（6） | `brackets`（4） | — |

## 4. 固定する設定

| 項目 | 値 |
| --- | --- |
| 経路 | cli 経路。全条件で無実行（`callBudget: 0`） |
| モデル | A Opus・B Sonnet・C Haiku・D Fable（被験体名の末尾の文字） |
| SKILL.md・`skill/words.md`・課題族 | Phase 4 の実施時から変わっていない |
| 容量 K | 8 |
| 自前の定義 | `probe`・`bare` は 0（`definitionBudget`）、`families` は制限なし |

`design.json`:

```json
{
  "rotation": {
    "train": [["text", "stats", "orders", "number"], ["prose", "sensor", "runs", "brackets"]],
    "heldOut": ["messy", "series", "ledger"]
  },
  "models": { "A": "opus", "B": "sonnet", "C": "haiku", "D": "fable" },
  "callBudget": 0,
  "definitionBudget": { "default": null, "probe": 0, "bare": 0 }
}
```

定義を禁じた被験体へのプロンプトは、「自前の Word を定義してはならない。Core と継承語だけで書き、
`definitions` は空にする。提出した定義は採点で無視されるので、それを呼ぶ解答は誤りになる」と伝え、
提出の雛形の `definitions` を `[]` にする。採点は予算を越えた定義を落として採点し、落とした名前を
`graded/*.json` の `definitionsDropped` に残す（違反の記録）。

解答の中で `DEF` を書く、またはブロックを `BIND` して `EXEC` で呼ぶことは止めない。採点は解答を 1 つずつ
走らせるので、そうして名付けた計算は、その解答の中でしか使えず、トークン数もその解答に数えられる。
禁じたいのは課題をまたぐ補助語であり、それは提出の `definitions` を通してしか作れない。結果ノートでは、
解答の中の `DEF` を数えて報告する。

## 5. 条件と世代

| 条件 | 世代 | 被験体 | 辞書 | 自前の定義 | 学習課題 |
| --- | --- | --- | --- | --- | --- |
| `base` | 0 | G0A〜G0D | なし。**Phase 4 の世代 0 の提出を再利用** | 可 | A |
| `families` | 1〜3 | L*g*A〜D | 前の世代から、族の数の順、K = 8 | 可 | B・A・B |
| `probe` | 1〜3 | P*g*A〜D | `families` の同じ世代と同じ | 不可 | B・A・B |
| `bare` | 1・2 | N*g*A〜D | なし | 不可 | B・A |

- `probe` は辞書を作らない（自前の定義がないので、選ぶ語がない）。毎世代、`families` の辞書を
  `--into probe` で受け取る。`probe` は `families` の各世代の辞書の「価値の測定器」である。
- `bare` は世代 3 を置かない。世代 3 の課題は世代 1 と同じ学習 B なので、世代 1 の `bare` が基準線になる。
- 世代 0 の再利用: Phase 4 の実施時から SKILL.md・`skill/words.md`・課題族・プロンプトの道具は
  変わっていない。今回の道具で出した世代 0 のプロンプトは、Phase 4 で渡したものと run の名前だけが
  違うことを確かめた。提出は Phase 4 の生データ（git 履歴）から写し、今回の道具で採点し直す。
- 世代 0 から `families` 規則で作る世代 1 の辞書（今回の道具で計算済み）:
  `G0A.SUM` `G0C.MAXV` `G0D.MAXV` `G0A.WORDS` `G0C.ORDERS` `G0A.MOD` `G0A.MEAN` `G0A.DEVS`。
  Phase 4 の `uses` の世代 1 との違いは、`G0A.SUMBY`・`G0A.VAR` の代わりに、3 族と 2 族に届く
  最大値の語が 2 つ入ったことである。2 つの最大値は書き方が違い（`[ MAX ] FOLD` と `SORT -1 GET`）、
  プローブ（文字列・入れ子）での答えが違うので、別の類として数えられる。

## 6. 手順

`<run>` は `phase5-<実施日>`。各段の後に `grade` → `evolve` → `seal` の順で進める。

1. 準備: `runs/<run>/design.json` を §4 のとおり置く。Phase 4 の `base/gen0/submissions/` を git 履歴
   （`17790710`）から `runs/<run>/base/gen0/submissions/` へ写す。共有領域（scratchpad）に解答を含む
   ファイルがないことを確かめる。
2. 世代 0: `node cli.mjs grade <run> base 0` の後、

   ```sh
   node cli.mjs evolve <run> base 0 8 --rule families --into families
   node cli.mjs evolve <run> base 0 8 --rule families --into probe
   node cli.mjs seal <run> base 0
   ```

3. 世代 1・2（各 12 体）: `families`・`probe`・`bare` の各 4 体。プロンプトは
   `node cli.mjs prompt <run> <cond> <g> <agent> --route cli` の出力をそのまま渡す。採点の後:

   ```sh
   node cli.mjs evolve <run> families <g> 8 --rule families
   node cli.mjs evolve <run> families <g> 8 --rule families --into probe
   node cli.mjs seal <run> <cond> <g>   # 3 条件それぞれ
   ```

4. 世代 3（8 体）: `families`・`probe` の各 4 体。採点の後に seal。
5. `node cli.mjs unseal <run>` で全世代を戻し、`node cli.mjs analyze <run>`。
6. 生データを丸ごとコミットし、次のコミットで作業木から消す。

各被験体の使用量（トークン・道具の呼び出し・所要時間）は、subagent の完了通知から記録する。
被験体の最初のメッセージが `prompt` の出力と一字一句一致することを、記録から機械的に確かめる。

## 7. 停止条件

- 世代 1 の `bare` または `probe` の 4 体のうち 2 体以上が、解答を書けない、または定義を禁じられた
  ことを理由に課題を放棄した: その条件を止めて原因を報告する。
- 定義の禁止を破った被験体（`definitionsDropped` がある）は、そのまま採点して数える。
  ある条件で 4 体中 2 体以上が破ったときは、プロンプトの書き方を疑い、止めて報告する。
- 天井では止めない。

## 8. 指標

| 指標 | 出所 | 問い |
| --- | --- | --- |
| 正答 1 件あたりのトークン数（学習・held-out 別） | `report.md` の Per generation、`graded/` | Q1 |
| 1 体の使用量（トークン）・所要時間 | subagent の完了通知 | Q1 |
| 正答（条件・モデル別）と誤答の原因 | `report.md`、`graded/` | Q1・補 |
| 正答のうち継承語を呼んだ割合（学習・held-out 別） | `report.md`「Training and held-out tasks」 | Q2 |
| 1 体が自分で定義した語の数 | `report.md` の `own Words / subject` | Q2 |
| 辞書の語ごとの、届いた族の数・課題の数（`lexicon.json` の `families`・`tasks`） | 各世代の `lexicon.json` | Q3 |
| held-out の族ごとの「継承 ÷ 共有」 | `report.md`「Held-out families」 | Q3 |
| 定義の禁止を破った数 | `graded/*.json` の `definitionsDropped` | 統制 |

`probe` の解答には自前の語がないので、「tokens ÷ expanded」は継承語の展開だけを表す。Q1 は
展開前のトークン数そのもの（`mean tokens`）を `bare` と比べる。

## 9. 規模と所有者が決めること

Phase 4 の被験体は 1 体 10.4 万〜21.4 万トークン（平均 13.1 万）だった。自前の定義を禁じると解答は
長くなるが、定義を書く手間はなくなる。同じ幅と見込み、32 体で 330 万〜690 万トークンとする。

1. 規模: 32 体で進めるか。縮める場合の順は、世代 3（8 体）を外す（Q3 で辞書が世代をまたいで
   どう動くかを失う）→ `bare` の世代 2（4 体）を外す（学習 A の基準線を失う）。
2. 世代 0 の再利用: Phase 4 の `base` の 4 体の提出を使うか（使わなければ 4 体増える）。
3. 実施の時期。

## 10. 本書と同じ変更で入れたもの

| 変更 | 場所 |
| --- | --- |
| 選定規則 `families`（届いた族の数、次に課題の数、次に使用回数）。辞書の語に `families` を記録 | `lib/evolve.mjs` |
| どの規則でも、1 つの類は辞書に 1 枠まで。依存先が、すでに選ばれた類の別の成員を運び込む候補は、同じ類の次の成員に譲る。どの成員でもそうなる類は、すべての類を試した後に、残った枠にだけ重複ごと入る（重複が別の類を押し出すことはなく、容量なしの辞書から類が落ちることもない） | `lib/evolve.mjs` |
| `design.json` の `definitionBudget`（`callBudget` と同じ形）。プロンプトが禁止・上限を伝え、採点が越えた定義を落として `definitionsDropped` に残す | `cli.mjs`・`lib/prompt.mjs`・`lib/grade.mjs` |
| 報告の「Held-out families」の表（族ごとの正答・共有・継承）と、`own Words / subject` の列 | `lib/analyze.mjs` |
| 上の道具のテスト | `test/phase5-tooling.test.mjs` |

1 類 1 枠の決まりを Phase 4 の生データにかけ直すと、Phase 4 で実際に作られた 12 の辞書のうち変わるのは
3 つである。`breadth` の世代 3 は平均の重複が消え、`W2B.MEAN`・`W2B.DEV`・`W2B.VAR` の代わりに
`W2A.VAR`・`W2A.REV`・`W1A.ARGMAX` が入る。`open` の世代 3 は、プローブ（D1）でだけ一致した約数と
素数判定の 2 語目（`O2C.DIVISORS`・`O2C.ISPRIME`）が消える。`open` の世代 2 は語が同じで並びだけが
変わる。`uses`・`fixed` の辞書は変わらない。

## 11. 完了の定義

- 結果を `docs/dev/lexicon-emergence-phase5-results-<実施日>.md`（`[観察ノート]`）にまとめ、
  Q1〜Q3 と補のそれぞれに、答え・答えられなかった理由のどちらかを書く。§1 の答えも結果ノートに移す。
- `tools/lexicon-emergence/README.md` に Phase 5 の結果ノートを足す。
- 本書は、結果ノートに設計の要点を移したうえで削除する。
