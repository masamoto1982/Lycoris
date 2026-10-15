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
| 2 | 停止保証の穴 | 完了。D1 は (B)。仕様版・実装版とも 1.0.0-beta.3 へ | `spec/termination.json` の `stratification`、LANG.SOURCE.CODE、`rust/src/interpreter/execute_builtin.rs` の `in_block_frame` |
| 3 | `recursionLimitExceeded` を `resourceLimitExceeded{resource: callDepth}` に | 完了 | `rust/src/interpreter/interpreter_core.rs` の `MAX_CALL_DEPTH`、`tools/mcp-server/golden/limits.json` |
| 4 | 辞書族の名前の渡し方 | 完了。`CONTRACT` は String でも名指せる。`DIGEST` は String をテキストとして読み続ける（§3 D3） | `spec/words.json`、`rust/src/interpreter/reflection_ops.rs` 冒頭 |
| 5 | JSON/SQL 境界 | 完了 | Reference「JSONとSQLの境界」、適合コーパス `standard-json-round-trip-lexeme` |
| 6 | 値依存のコスト軸 | 完了。27 語に `valueDependent` | `scripts/check-value-dependent-cost.mjs`、`resourceUsage.algebraicTerms` |
| 7 | ゲートの盲点 | 完了。コーパスの分類を語の族に、全結果種別の到達を検査 | `scripts/check-conformance-coverage.mjs`、`docs/quality/TRACEABILITY_MATRIX.md` |
| 8 | MCP の `words` 引数 | 完了 | `tools/mcp-server/phrasebook.js`、`docs/phrasebook/phrasebook.ajisai` の `#:digest` |
| 9 | SKILL.md の二層化 | 完了。65 KB → 約 20 KB（§3 D4） | `scripts/generate-skill-md.mjs`、`skill/words.md` |
| 10 | 節ごとの日本語要約 | 完了 | `docs/i18n/clause-summaries.ja.json`、`scripts/generate-specification.mjs` |
| 11 | 語彙創発実験 Phase 2 | 道具のみ完了。実験そのものは未実施（§2.1） | `tools/lexicon-emergence/README.md` |
| 12 | 停止性の法則化 | 完了。逆向きの法則は、判定を外すと自己適用を見つけることを確かめた | `rust/tests/termination_laws.rs` |

---

## 2. 残る段

### 2.1 段 11 の残り — 語彙創発実験 Phase 2 を走らせる

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
| D1 | 停止保証をどう扱うか | **(B) に決定（作者）。** 当初案の「名前の参照グラフで閉路を検査」では塞がらない（自己複製はどの名前も名指さない）ので、ブロックを「書かれた」（走っているコードのリテラルかその入れ子に等しい、または語を名指さない）と「データから組み立てた」に分け、後者は同時に一つだけ走れるとした。測度は（残りのデータ製ブロック実行, 辞書の階数, コードの大きさ, 残り要素）。失ったのはコンビネータの入れ子と、別フレームで束縛したブロックの連鎖。`check` は実行前に予告しない（書かれたかどうかは値で決まる）。BIND の閉路検査は残したが、停止性はもうそれに依らない |
| D2 | beta.2 で実装版も上げるか | 既定どおり上げた（実装版・仕様版とも 1.0.0-beta.2） |
| D3 | `DIGEST` の String 被演算子 | **既定から外した。** 既定は「Word の名前として読む」だったが、そうすると `'ADD' DIGEST` のような文字列の値のダイジェストが、同じ綴りの Word を定義した時点で変わり、LANG.VALUES.DENOTATION に反する。String はテキストとして読み続け、Word の同一性は `[ X ] 0 GET DIGEST` と書くことを仕様・Reference・SKILL.md に明記した。`CONTRACT` は既定どおり String を名前として読む |
| D4 | `SKILL.md` のバイト予算 | **既定から外した。** 既定は 16,384 だったが、§9 の全文を `skill/words.md` へ移しても検証済みの §1〜§8 だけで約 19 KB ある。§7（実際のエラー出力と直し方）を削らないために 22,528 とした |
| D5 | `callDepth` の既定値 | 既定どおり 256 |
| D6 | 適合コーパスの分類軸 | 既定どおり語の族（語に依らない事例は `language`） |
| D7 | 正典の日本語化の範囲 | 既定どおり節ごとの要約（非正典、鮮度をゲートで検査） |

---

## 4. 完了の定義

- Phase 2 を走らせるかどうかが決まり、走らせたなら結果の観察ノートがある（§2.1）。
- 本書と INDEX.md の行が削除されている。
