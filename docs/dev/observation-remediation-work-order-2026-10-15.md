# 観察結果の改修指示書（2026-10-15）

Status: non-canonical・`[作業指示書]`。本書は Ajisai の意味論も互換性方針も定義しない。
正典は `spec/` 配下の各ソースと、そこから生成される `SPECIFICATION.html` のみ。

出自: 2026-10-15 の観察。Ajisai を三つの角度から見た。(1) 想定ユーザーである AI 自身が
初見で MCP 経由に使い、つまずきを記録する。(2) 仕様が定理として掲げる主張（停止性）を
反例で攻める。(3) 外部への約束（仕様版・npm 公開）と、ゲートが実際に守っているものを
照合する。検出した問題点 P1〜P10 と可能性 1〜6 を 12 段の改修に写した。

本書の処分: 完了した段は、現在を統べる部分を生きている文書（`spec/`、`spec/CHANGELOG.md`、
各ゲートの冒頭コメント、`docs/quality/`）へ移してから本書から削った（INDEX.md 冒頭の規則）。
残る段が終わったら本書を削除し、INDEX.md の行も消す。

---

## 1. 進捗（2026-10-15 時点）

| 段 | 内容 | 状態 | 現在を統べる場所 |
| --- | --- | --- | --- |
| 1 | 仕様版の機構 | 完了。仕様版と実装版を 1.0.0-beta.2 へ | `spec/version-lock.json`、`spec/CHANGELOG.md`、`scripts/check-spec-version.mjs`、`spec/README.md` |
| 2 | 停止保証の穴 | **作者の判断待ち（§3 D1）** | 本書 §2 |
| 3 | `recursionLimitExceeded` を `resourceLimitExceeded{resource: callDepth}` に | 完了 | `rust/src/interpreter/interpreter_core.rs` の `MAX_CALL_DEPTH`、`tools/mcp-server/golden/limits.json` |
| 4 | 辞書族の名前の渡し方 | 完了。`CONTRACT` は String でも名指せる。`DIGEST` は String をテキストとして読み続ける（§3 D3） | `spec/words.json`、`rust/src/interpreter/reflection_ops.rs` 冒頭 |
| 5 | JSON/SQL 境界 | 完了 | Reference「JSONとSQLの境界」、適合コーパス `standard-json-round-trip-lexeme` |
| 6 | 値依存のコスト軸 | 完了。27 語に `valueDependent` | `scripts/check-value-dependent-cost.mjs`、`resourceUsage.algebraicTerms` |
| 7 | ゲートの盲点 | 完了。コーパスの分類を語の族に、全結果種別の到達を検査 | `scripts/check-conformance-coverage.mjs`、`docs/quality/TRACEABILITY_MATRIX.md` |
| 8 | MCP の `words` 引数 | 完了 | `tools/mcp-server/phrasebook.js`、`docs/phrasebook/phrasebook.ajisai` の `#:digest` |
| 9 | SKILL.md の二層化 | 完了。65 KB → 約 20 KB（§3 D4） | `scripts/generate-skill-md.mjs`、`skill/words.md` |
| 10 | 節ごとの日本語要約 | 完了 | `docs/i18n/clause-summaries.ja.json`、`scripts/generate-specification.mjs` |
| 11 | 語彙創発実験 Phase 2 | 道具のみ完了。実験そのものは未実施（§2.2） | `tools/lexicon-emergence/README.md` |
| 12 | 停止性の法則化 | 一部完了。逆向きの法則は D1 待ち | `rust/tests/termination_laws.rs` |

---

## 2. 残る段

### 2.1 段 2 — 停止保証の穴（P1、可能性 1）

**事実。** ブロックは、束縛も User Word も使わずに、データから自分の複製を作って
自分を実行できる。止めているのは 256 段の `callDepth` 上限だけである。

```
[ 1 COLLECT [ 0 0 ] GET EXEC EXEC ] 1 COLLECT [ 0 0 ] GET EXEC EXEC
→ error:resourceLimitExceeded（resource: callDepth）"callDepth of 257 exceeds the limit of 256"
```

`1 COLLECT [ 0 0 ] GET` は、添字が持ち上がるので、一つのブロックから二つを取り出す。
`SELECT` で基底を選べば、上限に触れずに終わる再帰も書ける（3 2 1 0 を出力して止まる）。

```
[ 'N' BIND N PRINT 1 COLLECT [ 0 0 ] GET EXEC N 1 SUB 3 COLLECT [ 0 2 1 ] GET
  [ EXEC EXEC ] [ 'done' ] N 0 GT SELECT EXEC ] 3 <同じブロック> EXEC
```

User Word を経る形も同じ上限で止まる。

```
[ 'F' BIND F F EXEC ] 'RUN' DEF [ RUN ] RUN
[ 1 COLLECT [ 0 0 ] GET EXEC EXEC ] 'RUN' DEF [ RUN ] RUN
```

したがって次の主張は現状では偽である。どちらに直すかが D1。

| 場所 | 主張 |
| --- | --- |
| `spec/termination.json` `sites[EXEC].why` | 被演算子は EXEC の前からあった有限の Vector で、入れ子が減る |
| `spec/termination.json` `claim` | どの上限も終了を決めない |
| `spec/language-semantics.md` LANG.DICTIONARY.ACYCLIC | Ajisai is therefore total and not Turing-complete |
| `spec/language-semantics.md` LANG.MACHINE.LIMITS | termination already follows from LANG.DICTIONARY.ACYCLIC |
| README | Ajisai has no recursion and no unbounded loop |
| SKILL.md §3 | Repetition is expressed only through MAP / FILTER / FOLD / SCAN |

`rust/tests/termination_laws.rs` の `self_application_is_stopped_by_a_ceiling_today` が
現状を固定しているので、どちらを選んでもこの挙動の変化は意図したものになる。

**着手時に分かったこと。** 当初の既定案（DEF と BIND が一つの参照グラフで閉路を検査する）
では塞がらない。上の第一例はどの名前も名指さないので、名前のグラフには現れない。

**(A) 認める場合の手順。**

1. LANG.MACHINE.LIMITS の `callDepth` 行を、ホストの裁量でなく正典の上限として書き直す。
   値 256 を固定するか、「少なくとも N」とするかを決める。
2. `spec/termination.json` の `claim` を「入れ子は `callDepth`、仕事は `executionSteps` で
   縛られるので必ず止まる」に、`sites[EXEC].why` を事実どおりに改める。`ceilings.notLoadBearing`
   から `resourceLimitExceeded` を外す。
3. 上の表の各主張を撤回し、基底つき再帰が書けることを README・Reference 両言語・SKILL.md に
   書く。`SELECT` が何も評価しないことの意味（両方の腕がすでに計算済み）は変わらない。
4. 適合コーパスに第一例（`callDepth` で止まる）と countdown（`3 2 1 0` を出力）を足す。
5. `spec/CHANGELOG.md` に `breaking`（全域性の撤回）として書き、仕様版を上げる。

**(B) 全域性を取り戻す場合の手順。**

1. 「動的な実行」を定義する: コードを走らせる語（EXEC・MAP・FILTER・FOLD・SCAN）のうち、
   被演算子が直前に書かれたリテラルのブロックでも、直前に読んだ束縛名でもないもの。
2. 動的に走らせるブロックは、それ自身と、User Word の本体を辿って届くコードに、動的な実行を
   一つも含んではならない。含めば実行時に新しい ERROR（例 `nonStratifiedCode`）で拒む。
   判定はブロックの内容だけで決まるので、値ダイジェストで記憶できる。
3. 証明の筋: 動的な実行は入れ子にならない。その内側はリテラルの入れ子と非循環の辞書で
   測度が減る。`spec/termination.json` に新しい site と measure を足す。
4. 上の二例は拒まれる。適合コーパスの既存の EXEC 事例は束縛名経由かリテラルなので通る
   見込みだが、`check:conformance-coverage` と `cargo test` で確かめる。
5. 失うもの: ブロックを受け取り、さらにスタック上のブロックを走らせる組み立て
   （コンビネータ）。`spec/CHANGELOG.md` に `breaking` として書く。

**どちらでも。** `check` がこの形を実行前に知らせられるか（B なら同じ判定を静的に、A なら
`callDepth` に届きうることを note で）を決める。段 12 の逆向きの法則（B なら「判定を外すと
再現が上限に触れる」）もここで足す。

### 2.2 段 11 の残り — 語彙創発実験 Phase 2 を走らせる

道具は揃った。route B は各世代の語彙を `phrasebook.ajisai`（各語の `#:digest` 付き）に書き、
被験体のサーバーを `AJISAI_PHRASEBOOK` でそれに向け、被験体には `words` で呼ばせる。結果の
`prelude` のダイジェストを `lexicon-digests.json` と照合すれば、H5 を内容で測れる。

走らせるには次が要る（パイロット結果 §5 の前提）。

- 系統の異なるモデル。route B の harness は Claude API しか話さないので、別系統用の
  harness が要る。
- 正答率が 100% にならない難度の課題族。
- API の予算と、実行の承認。

結果は `docs/dev/lexicon-emergence-phase2-results-<date>.md` に `[観察ノート]` として残す。

---

## 3. 判断（既定値で進めたものを含む）

| ID | 判断 | 状態 |
| --- | --- | --- |
| D1 | 停止保証をどう扱うか | **未決。** §2.1 の (A) と (B)。推奨は (B)（「構成による全域性」と、`SELECT` が何も評価しないことの安さを保てる） |
| D2 | beta.2 で実装版も上げるか | 既定どおり上げた（実装版・仕様版とも 1.0.0-beta.2） |
| D3 | `DIGEST` の String 被演算子 | **既定から外した。** 既定は「Word の名前として読む」だったが、そうすると `'ADD' DIGEST` のような文字列の値のダイジェストが、同じ綴りの Word を定義した時点で変わり、LANG.VALUES.DENOTATION に反する。String はテキストとして読み続け、Word の同一性は `[ X ] 0 GET DIGEST` と書くことを仕様・Reference・SKILL.md に明記した。`CONTRACT` は既定どおり String を名前として読む |
| D4 | `SKILL.md` のバイト予算 | **既定から外した。** 既定は 16,384 だったが、§9 の全文を `skill/words.md` へ移しても検証済みの §1〜§8 だけで約 19 KB ある。§7（実際のエラー出力と直し方）を削らないために 22,528 とした |
| D5 | `callDepth` の既定値 | 既定どおり 256 |
| D6 | 適合コーパスの分類軸 | 既定どおり語の族（語に依らない事例は `language`） |
| D7 | 正典の日本語化の範囲 | 既定どおり節ごとの要約（非正典、鮮度をゲートで検査） |

---

## 4. 完了の定義

- D1 が決まり、§2.1 の選んだ手順が main にマージされている。
- Phase 2 を走らせるかどうかが決まり、走らせたなら結果の観察ノートがある。
- 本書と INDEX.md の行が削除されている。
