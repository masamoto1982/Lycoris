# 語彙創発実験 Phase 4 の設計（2026-10-16）

Status: 非正典・`[作業指示書]`。本書は Ajisai の意味論も互換性方針も定義しない。
正典は `spec/` 配下の各ソースと、そこから生成される `SPECIFICATION.html` のみ。

前提文書: `lexicon-emergence-phase3-results-2026-10-15.md`（以下「Phase 3」）。
道具: `tools/lexicon-emergence/`（使い方は同ディレクトリの `README.md`）。

---

## 0. 要約

- Phase 3 の §7 が挙げた次の論点 4 つのうち、3 つ（選定規則・課題の入れ替え・課題の難しさ）を
  本書で扱う。残る 1 つ（`SELECT` のエラーに書き方を添える）は、本書と同じ変更でエンジンに入れた（§9）。
- Phase 3 の容量なし（C-open）の辞書は、課題 1 問を丸ごと解く語で埋まった。同じ課題を毎世代
  解かせる限り、それを「良い言語」と区別できない。Phase 4 は**選定に使わない課題（held-out）**を
  毎世代解かせ、**辞書が選ばれなかった課題にも効くか**を測る。
- 学習に使う課題族は世代ごとに 2 組（A・B）を入れ替える。前の世代の課題にしか効かない語は、
  次の世代で使われずに辞書から落ちるはずである。
- 選定規則に `breadth`（その語に届いた正答の、異なる課題の数で並べる）を足す。課題を丸ごと解く語は
  何体が呼んでも 1 課題にしか届かないので、下位に回る。
- 正答率は主な指標にしない。Phase 3 で無実行でも 99% に達し、上げる余地がなかった。課題を長くして
  誤りを増やす案は、測るものが「言語」から「注意の持続」に寄るので採らない（§1 の補）。
- 規模は 52 体（世代 0 の 4 体と、4 条件 × 世代 1〜3 × 4 体）、無実行、1 体 39〜44 問。

## 1. 問い

| 番号 | 問い | 主に見る比較 |
| --- | --- | --- |
| P1（汎化） | 継承した辞書は、選定に使わなかった課題（held-out）でも再利用され、解答を短くするか | 各条件の世代 1〜3 の held-out と、世代 0（辞書なし）の held-out |
| P2（入れ替え） | 学習課題を世代ごとに入れ替えると、辞書は課題を丸ごと解く語で埋まらなくなるか | `open`（入れ替えあり）の辞書と Phase 3 の C-open の辞書。`uses` と `fixed` の held-out |
| P3（選定規則） | `breadth` は、`uses` の早い固定と `reinvention` の振動を避けつつ、held-out の再利用を上げるか | `breadth` と `uses`（と Phase 3 の `reinvention`） |
| P4（忘却） | 一方の学習組にしかない系統の語は、もう一方の組を解く世代を越えて辞書に残るか | held-out の `ledger` 族（キーごとの集計。学習では A の `orders` 族にしかない）の再利用 |
| 補 | 正答率 | 記録だけする。天井は止める理由にしない |

課題の難しさ（Phase 3 §7 の 1）は問いに置かない。Phase 3 で残った誤りは引数の順・スタックの消費・
`SELECT` の形で、課題を長くすれば増やせるが、それは言語の習得ではなく注意の持続を測る。辞書の
効果を正答率で見る実験は、無実行で誤りが出る課題が別に見つかるまで保留する（§8 の 2）。

## 2. 課題族の割り当て

11 族を、共有する部分計算で 4 系統に分ける。held-out は 3 系統から 1 族ずつ選び、学習の 2 組は
4 系統をなるべく両方に置く。

| 系統 | 学習 A | 学習 B | held-out |
| --- | --- | --- | --- |
| 語の処理（分割・大文字化・頻度） | `text`（7） | `prose`（6） | `messy`（6） |
| 数の列（平均・最大・連） | `stats`（7） | `sensor`（6）、`runs`（5） | `series`（6） |
| キーごとの集計 | `orders`（6） | — | `ledger`（6） |
| アルゴリズム | `number`（6） | `brackets`（4） | — |
| 計 | 26 問 | 21 問 | 18 問 |

キーごとの集計は 2 族しかないので、学習 B には置けない。これを P4 に使う。B を解く世代では
`orders` 由来の語が使われず、使用回数で選ぶ規則なら辞書から落ちる。そのとき held-out の
`ledger` の再利用が下がるかを見る。

被験体は学習と held-out の区別を知らない。課題は族の名前順に並べ、held-out が常に最後にならない
ようにする（`lib/rotation.mjs`）。

## 3. 固定する設定

| 項目 | 値 |
| --- | --- |
| 経路 | cli 経路。全条件で無実行（`callBudget: 0`） |
| モデル | A Opus・B Sonnet・C Haiku・D Fable（被験体名の末尾の文字） |
| SKILL.md | 本書を入れた変更の時点のもの |
| 容量 K | 8（`open` は無制限） |

`design.json`:

```json
{
  "rotation": {
    "train": [["text", "stats", "orders", "number"], ["prose", "sensor", "runs", "brackets"]],
    "heldOut": ["messy", "series", "ledger"],
    "fixed": ["fixed"]
  },
  "models": { "A": "opus", "B": "sonnet", "C": "haiku", "D": "fable" },
  "callBudget": 0
}
```

## 4. 条件と世代

| 条件 | 世代 | 被験体 | 辞書 | 学習課題 |
| --- | --- | --- | --- | --- |
| `base` | 0 | G0A〜G0D | なし（世代 1 の 4 条件の辞書の出所、held-out の基準線） | A |
| `uses` | 1〜3 | U*g*A〜D | 使用回数順、K = 8 | B・A・B |
| `breadth` | 1〜3 | W*g*A〜D | 届いた課題の数の順、K = 8 | B・A・B |
| `open` | 1〜3 | O*g*A〜D | 容量なし（使用回数順） | B・A・B |
| `fixed` | 1〜3 | F*g*A〜D | 使用回数順、K = 8 | A・A・A |

- 4 条件の世代 1 は同じ `base` 世代 0 から作る。`uses` と `fixed` の世代 1 の辞書は同じで、
  違いは解く課題だけになる。
- `fixed` は入れ替えの対照である（Phase 3 の `uses` に held-out を足したもの）。
- 被験体の名前の頭の文字が条件を表す（breadth は Sonnet の `B` と紛れないよう `W`）。
- held-out の結果は採点し報告するが、次の辞書の選定には入らない（`grade` が `heldOut` の印を付け、
  `evolve` がそれを除く）。

## 5. 手順

`<run>` は `phase4-<実施日>`。Phase 3 と同じく、各段の後に `grade` → `evolve` → `seal` の順で進める。

1. 準備: `runs/<run>/design.json` を §3 のとおり置く。共有領域（scratchpad）に解答を含むファイルが
   ないことを確かめる。
2. 世代 0（4 体）: `base`。プロンプトは `node cli.mjs prompt <run> base 0 <agent> --route cli` の
   出力をそのまま渡す。
3. 世代 0 の採点と §6 の判定。続けるなら:

   ```sh
   node cli.mjs evolve <run> base 0 8 --into uses
   node cli.mjs evolve <run> base 0 8 --rule breadth --into breadth
   node cli.mjs evolve <run> base 0 Infinity --into open
   node cli.mjs evolve <run> base 0 8 --into fixed
   node cli.mjs seal <run> base 0
   ```

4. 世代 1・2（各 16 体）: 4 条件の各 4 体。採点の後:

   ```sh
   node cli.mjs evolve <run> uses <g> 8
   node cli.mjs evolve <run> breadth <g> 8 --rule breadth
   node cli.mjs evolve <run> open <g> Infinity
   node cli.mjs evolve <run> fixed <g> 8
   node cli.mjs seal <run> <cond> <g>   # 4 条件それぞれ
   ```

5. 世代 3（16 体）: 採点の後に seal。
6. `node cli.mjs unseal <run>` で全世代を戻し、`node cli.mjs analyze <run>`。
7. 生データを丸ごとコミットし、次のコミットで作業木から消す。

各被験体の使用量（トークン・道具の呼び出し・所要時間）は、subagent の完了通知から記録する。
API の利用上限で止まった被験体は、Phase 3 と同じく、解答を書き終えていればそのまま採点し、
そうでなければ「一時的な上限で中断した。続けてほしい」とだけ送って再開させる。

## 6. 停止条件

- 世代 0 の 4 体の正答が 30% 未満（176 問中 53 問未満）: 止めて誤答を読む。
- 天井では止めない。正答率は主な指標ではない。
- 世代 1 以降で、ある条件の 4 体のうち 2 体以上が解答を書けなかったときは、その条件を止めて
  原因を報告する。

## 7. 指標

| 指標 | 出所 | 問い |
| --- | --- | --- |
| held-out の正答のうち継承語に届いた割合、届いた課題の数 | `report.md`「Training and held-out tasks」 | P1・P2・P3 |
| held-out の tokens ÷ expanded（世代 0 との比） | 同上 | P1 |
| 辞書の語ごとの、前の世代で届いた課題の数（`lexicon.json` の `tasks`） | 各世代の `lexicon.json` | P2・P3 |
| 辞書の入れ替わり（前の世代から残った語の数） | 各世代の `lexicon.json` | P3 |
| `ledger` 族の正答のうち継承語に届いた数 | `graded/` を族で数える | P4 |
| 正答（学習・held-out 別、モデル別） | `report.md` | 補 |
| H1・H2・H5、参照解と字句まで同じ正答 | `report.md` | 従来どおり |

「課題を丸ごと解く語」は、辞書の語のうち `tasks` が 1 のものと数える。

## 8. 規模と所有者が決めること

Phase 3 の無実行の被験体は 33 問で 1 体 10.5 万〜20 万トークンだった。Phase 4 は 39〜44 問なので
1 体 12.5 万〜27 万と見込み、52 体で 650 万〜1,400 万トークンになる。

1. 規模: 52 体で進めるか。縮める場合の順は、世代 3（16 体）を外す → `fixed` を外す（P2 の
   `uses` との比較を失う）。
2. 課題の難しさ: 正答率を主な指標から外すことを了承するか。了承しない場合は、無実行で誤りが出る
   課題を先に作る較正が要る（較正 1 回あたり 4 体）。
3. 実施の時期。

## 9. 本書と同じ変更で入れたもの

| 変更 | 場所 |
| --- | --- |
| `SELECT` が `shapeMismatch` で拒んだときのエラー文に、丸ごと選ぶ書き方（`2 COLLECT … GET`）を添える。例は実行して確かめる | `rust/src/diagnostic_hint.rs`（`MISTAKEN_SHAPE`）、WASM 2 か所の再生成 |
| `design.json` の `rotation`（学習組の入れ替え・held-out・入れ替えない条件） | `lib/rotation.mjs`・`cli.mjs` |
| 採点が held-out の結果に `heldOut` の印を付ける | `lib/grade.mjs` |
| 選定規則 `breadth`。どの規則も held-out の結果を数えない。辞書の語に `tasks`（届いた課題の数）を記録 | `lib/evolve.mjs` |
| 報告の学習・held-out 別の表。正答の分母を世代ごとの出題数にする | `lib/analyze.mjs` |
| 上の道具のテスト | `test/phase4-tooling.test.mjs` |

`SELECT` のヒントは、Phase 3 の 2 件の誤り（どちらも無実行の Haiku）を防がない。無実行の被験体は
エラー文を見ないからである。効くのは、走らせて確かめる利用者と、実行を許した条件の被験体である。
Phase 3 で実行を許した 12 体の記録には、`SELECT` の `shapeMismatch` は 1 件もなかった。

## 10. 完了の定義

- 結果を `docs/dev/lexicon-emergence-phase4-results-<実施日>.md`（`[観察ノート]`）にまとめ、
  P1〜P4 と補のそれぞれに、答え・答えられなかった理由のどちらかを書く。
- `tools/lexicon-emergence/README.md` に Phase 4 の結果ノートを足す。
- 本書は、結果ノートに設計の要点を移したうえで削除する。
