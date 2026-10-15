# 観察結果の改修指示書（2026-10-15）

Status: non-canonical・`[作業指示書]`。本書は Ajisai の意味論も互換性方針も定義しない。
正典は `spec/` 配下の各ソースと、そこから生成される `SPECIFICATION.html` のみ。
本書が指示する正典の変更は、それぞれの段で仕様の版を動かす手順を含む。

出自: 2026-10-15 の観察。Ajisai を三つの角度から見た。(1) 想定ユーザーである AI 自身が
初見で MCP 経由に使い、つまずきを記録する。(2) 仕様が定理として掲げる主張（停止性）を
反例で攻める。(3) 外部への約束（仕様版・npm 公開）と、35 本のゲートが実際に守っている
ものを照合する。検出した問題点 P1〜P10 と可能性 1〜6 を、すべて改修の段に写した。

本書の処分: 各段が完了したら、現在を統べる部分を生きている文書（`spec/`、`docs/dev/` の
`[方針記録]`、`docs/quality/`）へ移し、その段の節を本書から削る。全段が終わったら本書を
削除し、INDEX.md の行も消す（INDEX.md 冒頭の規則どおり）。

---

## 0. 全体の規律

| 項目 | 規則 |
| --- | --- |
| PR の単位 | 一段一 PR。段の中の「副段」は同じ PR のコミットとして分けてよい |
| 順序 | §1 の表の順。段 1（仕様版の機構）は他のすべての正典変更の前提 |
| 毎 PR の受け入れ試験 | `cargo fmt --check`、`cargo clippy --all-targets -- -D warnings`、`cargo test --all-targets`（`rust/`）、`npm run check`、`npm run lint`、`npm test`、`.github/workflows/test.yml` の全 `npm run check:*` / `*:check`、`npm run test:mcp` |
| エンジンを変える段 | `npm run build:wasm` と `npm run build:mcp-wasm` で WASM バンドルを再生成して同じ PR に含める（既存の慣行「Rebuild the committed WASM bundles」） |
| 表面の同時更新 | 正典（`spec/`）を変える PR は、同じ PR で Reference 両言語（`npm run word:reference`）、`SKILL.md`（`npm run generate:skill`）、MCP の資産（`npm --prefix tools/mcp-server run sync:assets`）、適合コーパス（`tests/conformance/index.html`）、`SPECIFICATION.html`（`npm run specification:generate`）を更新する |
| 行数予算 | 新規・大幅改稿の Rust ファイルは 500 行以内（`check:file-size`）。`language-semantics.md` は `check-semantic-kernel.mjs` の行数予算内 |
| 速度 | 遅くなる変更は採らない（`rust-10k-reduction-plan.md` と同じ規律）。段 2 の検査は DEF/BIND 時に一度走るだけで、実行ループに入れない |

禁止事項:

- 仕様の版を動かさずに、語彙・プログラムの意味・ホストプロトコルを変えない（段 1 以後は
  ゲートが拒む）。
- テストや適合コーパスの事例を、通すために弱めたり消したりしない。削るのは「仕様にも実装
  にもない」と確定したものだけ（`spec-impl-alignment-methodology.md`）。
- 退役した名前（`spec/retired-words.json`）を戻さない。
- 文字列を Symbol に変える Word を作らない（`spec/termination.json` の不変条件
  `everyReachableWordIsNamedBySymbol` を壊す）。

停止条件: ある段の受け入れ条件が「作者の判断が必要なこと」（§3）の既定値では満たせないと
分かったら、その段で止めて §3 の該当項目に選択肢と実測を書き、作者に回す。他の段には進んでよい。

---

## 1. 段の一覧

| 段 | 内容 | 直す問題 / 追う可能性 | 依存 | 正典 | 仕様版 |
| --- | --- | --- | --- | --- | --- |
| 1 | 仕様版の機構: `spec/version-lock.json`、`spec/CHANGELOG.md`、`check:spec-version` ゲート、エンベロープの `specificationVersion`、beta.2 への繰り上げ | P2・P5 / 可能性 2・5 | なし | 変える | 1.0.0-beta.2 |
| 2 | 停止保証の穴を塞ぐ: 参照グラフを辞書と束縛で一つにし、DEF/BIND が同じ閉路検査を使う | P1 / 可能性 1 | 1 | 変える | 上げる |
| 3 | `recursionLimitExceeded` を `resourceLimitExceeded{resource: callDepth}` に畳む | 可能性 1 | 2 | 変える | 上げる |
| 4 | 辞書族の名前の渡し方を一本化: CONTRACT・DIGEST が String の名前を受ける | P3 / 可能性 4 | 1 | 変える | 上げる |
| 5 | JSON/SQL 境界を事例と Reference で明示する | P9 | 1 | 変えない（コーパスに事例追加） | 据え置き |
| 6 | コスト契約に値依存の軸 `algebraic` を加え、`resourceUsage` に `algebraicTermsPeak` を足す | P6 / 可能性 6 | 1 | 変える | 上げる |
| 7 | ゲートの盲点: 適合コーパスの分類、結果種別ごとの網羅、新ゲートの CI 登録 | P7 | 2・3 | 変えない | 据え置き |
| 8 | MCP に `words` 引数と `ajisai://phrasebook` を足し、phrasebook に内容ダイジェストを記録する | P4 / 可能性 3 | 4 | 変えない（ホスト側） | 据え置き |
| 9 | `SKILL.md` と MCP quickstart を二層にし、サイズ予算をゲートにする | P10 | 4・8 | 変えない | 据え置き |
| 10 | 正典の節ごとの日本語要約 `docs/i18n/clause-summaries.ja.json` と、その鮮度ゲート | P8 | なし | 変えない | 据え置き |
| 11 | 語彙創発実験 Phase 2 を MCP の `words` 引数で再開する | 可能性 3 | 8 | 変えない | 据え置き |
| 12 | 停止性の定理を機械化する: proptest の生成器と、`callDepth` 上限が閉路で発火しないことの法則 | 可能性 1 | 3 | 変えない | 据え置き |

段 2〜4・6 はそれぞれ破壊的変更で、段 1 の機構によって版が上がる。同じリリースにまとめる
場合は、`spec/CHANGELOG.md` の一つの版の節に複数の項目として書く（§2.1 の規則）。

---

## 2. 各段の詳細

### 段 1 — 仕様版の機構（P2・P5、可能性 2・5）

**現状の事実。**

| 事実 | 根拠 |
| --- | --- |
| README と `spec/README.md`、`language-semantics.md` の Status は「破壊的変更は仕様版を上げる」と約束している | `spec/language-semantics.md` 7〜8 行、README の Status 表 |
| 1.0.0-beta.1 のラベルは 2026-10-08 に付いた | `git log -S'1.0.0-beta.1' -- spec/language-semantics.md` |
| 以後、版は不変のまま破壊的変更が入った: 除算の全域化と三点（10-09）、Belnap 四値の採用（10-10）と三値への撤回（10-12）、分数の輪への移行（10-12）、上限超過の NIL から ERROR への変更、高階ブロックの一値規則、契約推論の縮小 | `git log -- spec/language-semantics.md`、`tools/mcp-server/CHANGELOG.md` 0.8.1〜0.9.0 |
| MCP の CHANGELOG 0.9.0 自身が "Breaking … no deprecation period" と書くが、エンベロープの `engineVersion` は 0.8.2 と同じ文字列 | `tools/mcp-server/CHANGELOG.md` |
| 35 本のゲートのどれもこの約束を検査しない。`check-version-sync.mjs` は実装の版の一致だけを見る | `scripts/check-version-sync.mjs` |

**目的。** 「版を上げる」を人の記憶ではなくゲートにする。npm の利用者が、答えが変わった
理由を版と日付で辿れるようにする。

**変更箇所と手順。**

1. `spec/version-lock.json` を新設する。

   ```json
   {
     "schemaVersion": 1,
     "specificationVersion": "1.0.0-beta.2",
     "definitionalDigest": "<sha256>",
     "normativeDigest": "<sha256>"
   }
   ```

   - `definitionalDigest`: 機械可読の正典の正規化ダイジェスト。対象は `grammar.json`、
     `words.json`（各 entry の `documentation` を除く）、`outcomes.json`、
     `termination.json`、`identity.json`、`retired-words.json`、`host-protocol.schema.json`。
     JSON はキー順で正規化してから連結する。
   - `normativeDigest`: `language-semantics.md` と `gui-semantics.md` の全文のダイジェスト。
2. `spec/CHANGELOG.md` を新設する。版ごとの節。各項目は次の形を守る。

   | 欄 | 内容 |
   | --- | --- |
   | 種別 | `breaking` / `compatible` / `editorial` のいずれか一つ |
   | 節 | 変えた LANG.* の節 ID |
   | 前後 | 一行のプログラムと、変更前・変更後の答え（`-5 0 DIV` → `-1/0` / `1/0` のように） |
   | 日付 | マージ日 |

   最初の節は 1.0.0-beta.2 とし、上の表の破壊的変更を日付付きで遡って記す（事後の記録で
   あることを節の冒頭に書く）。
3. `scripts/check-spec-version.mjs` を新設し、`npm run check:spec-version` に繋ぐ。検査:
   - `version-lock.json` の両ダイジェストが計算値と一致する。
   - `definitionalDigest` が git の `HEAD~`（PR では `origin/main`）から変わっていれば、
     `specificationVersion` も変わっていて、`CHANGELOG.md` の先頭の節がその版で、
     `breaking` の項目を一つ以上持つ。
   - `normativeDigest` だけが変わっていれば、`CHANGELOG.md` の先頭の節に項目が一つ以上
     増えている（種別は著者が選ぶ。`breaking` なら版も変わっていること）。
   - `specificationVersion` が `language-semantics.md` の Status、README の Status 表、
     `spec/README.md` の記述と一致する（`check-vocabulary-counts.mjs` と同じ「表面を数で
     縛る」やり方）。
   - 比較の基準となる ref は引数で受け、CI では `github.event.pull_request.base.sha` を渡す。
4. ホストプロトコルのエンベロープに任意フィールド `specificationVersion` を足す
   （`spec/host-protocol.schema.json`。任意フィールドの追加はプロトコル版を上げない、と
   `spec/README.md` が定めている）。値は `version-lock.json` から
   `scripts/generate-word-registry.mjs` と同じ経路で Rust の生成物に落とし、CLI の
   `agent compute` と WASM 境界の両方が返す。MCP の `mcp.engineVersion` の隣に
   `mcp.specificationVersion` として載せる。`result.schema.json` と `selftest.js` を更新する。
5. `tools/mcp-server/CHANGELOG.md` の各節の冒頭に、その版が話す仕様版の範囲を一行で書く
   規則を追加し、0.9.0 以前の節にも遡って書く。`tools/mcp-server/README.md` に
   「beta の間は仕様版が上がると答えが変わる。`mcp.specificationVersion` と
   `spec/CHANGELOG.md` で辿る」と書く。
6. `.github/workflows/test.yml` に `check:spec-version` を加える（base ref を渡す）。
7. 版を 1.0.0-beta.2 に上げる。実装の版（`package.json` ほか `check-version-sync.mjs` の
   四つ）も同じ PR で上げるかは §3 D2。

**受け入れ条件。**

- `npm run check:spec-version` が、`words.json` の `errorWhen` を一つ変えただけの作業木で
  失敗し、版と CHANGELOG を書き足すと通る（スクリプトの自己テストとして
  `scripts/check-spec-version.test.mjs` に固定する）。
- `documentation` だけを変えた作業木では失敗しない。
- `1 2 ADD` の MCP の結果に `mcp.specificationVersion` があり、`ajisai agent compute` の
  JSON にも同じ値がある。

### 段 2 — 停止保証の穴（P1、可能性 1）

**状態（2026-10-15 追記）: 作者の判断待ち（§3 D1）。** 着手時の追加調査で、下の既定案
（BIND の静的検査）では穴が塞がらないと分かった。束縛も User Word も使わずに、ブロックが
データから自分の複製を作って自分を実行できる。

```
[ 1 COLLECT [ 0 0 ] GET EXEC EXEC ] 1 COLLECT [ 0 0 ] GET EXEC EXEC
→ error:recursionLimitExceeded "recursion limit exceeded (256) in 'EXEC'"
```

`1 COLLECT [ 0 0 ] GET` は、添字が持ち上がるので、ブロックを一つ包んで二つ取り出す。
`FILL` は葉にしか持ち上がらないのでブロックを複製しないが、`GET` で足りる（ほかの語で複製できるかは未調査）。さらに `SELECT` で基底を
選べるので、終わる再帰も書ける（3 2 1 0 を出力して `'done'` で止まる）:

```
[ 'N' BIND N PRINT 1 COLLECT [ 0 0 ] GET EXEC N 1 SUB 3 COLLECT [ 0 2 1 ] GET
  [ EXEC EXEC ] [ 'done' ] N 0 GT SELECT EXEC ] 3 <同じブロック> EXEC
```

したがって `spec/termination.json` の `sites[EXEC].why`（「被演算子は EXEC の前から
あった有限の Vector で、入れ子が減る」）は偽で、現状の Ajisai は上限（256 段と
`executionSteps`）の下でチューリング完全に近い。README の「Ajisai has no recursion and
no unbounded loop」、LANG.DICTIONARY.ACYCLIC の「Ajisai is therefore total and not
Turing-complete」も同じく偽。正典はまだ直していない（どちらに直すかが D1）。

**再現。** いずれも現状 `error:recursionLimitExceeded`（256 段）。

```
[ 'F' BIND F F EXEC ] 'RUN' DEF [ RUN ] RUN
[ 'G' BIND G G EXEC ] 'RUN2' DEF [ 'F' BIND F RUN2 ] 'RUN' DEF [ RUN ] RUN
```

純束縛版 `[ 'F' BIND F F EXEC ] 'RUN' BIND [ RUN EXEC ] RUN EXEC` は
`selfReferentialDefinition` で拒まれる。差は、BIND の閉路検査が「他の束縛」は辿るが
User Word の本体は辿らないこと（`rust/src/interpreter/bindings.rs` 444 行付近）。
F = `[ RUN ]`、RUN の本体が F を名指しして閉路が閉じる。

`spec/termination.json` の claim「どの上限も終了を決めない」が、ここでは破れている。
`check` は note を出すだけで通す。適合コーパスに `recursionLimitExceeded` の事例はない。

**目的。** 参照グラフを一つにする。節点は User Word と可視の束縛、辺は本体または値が持つ
Symbol。DEF も BIND も同じ関数で閉路を検査する。

**変更箇所と手順。**

1. `rust/src/interpreter/` に閉路検査を一つにまとめたモジュールを置く（500 行以内。
   候補名 `reference_graph.rs`）。入力は「これから登録する名前」「その本体または値」「現在の
   辞書」「現在の可視束縛」。到達可能な Symbol を、User Word なら定義本体へ、可視束縛なら
   その値へ、入れ子の Vector と Record の中まで辿る。Core Word は葉。登録する名前に到達し
   たら `selfReferentialDefinition`。
2. `execute_def.rs` の `op_def_inner`（285 行付近の検査）と `bindings.rs` の検査を、その
   関数の呼び出しに置き換える。既存のメッセージ文（`the body names itself (B -> A -> B)` と
   `the value names 'F' itself, directly or through other bindings`）は、経路に Word と束縛
   が混ざることを言える形に直す。
3. `spec/language-semantics.md` の LANG.DICTIONARY.ACYCLIC を「辞書と束縛を合わせた
   一つの参照グラフが非循環」と書き直す。`spec/termination.json` の `acyclicity.statement`
   と `sites[boundValue].why` を同じ内容に直し、`witnesses.refused` に上の再現二つと、DEF が
   束縛を経て閉じる形

   ```
   [ W ] 'F' BIND [ F EXEC ] 'W' DEF
   ```

   を加える。`witnesses.accepted` にブロック渡しが生きていることを示す

   ```
   [ 'F' BIND F EXEC ] 'APPLY' DEF [ 1 2 ADD ] APPLY
   ```

   を加える。
4. `rust/tests/termination_laws.rs` は witnesses を実行するので、追加した事例が自動で走る。
   加えて `rust/tests/standard_operational_laws.rs` か新規の `reference_graph_laws.rs` に、
   「DEF と BIND が同じ答えを出す」法則を置く: 同じ本体を DEF しても BIND しても、閉路の
   有無の判定が一致する。
5. 適合コーパスに事例を 3 件足す（再現二つは `ajisai-expect-error` に
   `selfReferentialDefinition`、ブロック渡しの accepted は結果 `3/1`）。
6. `check` が同じ検査を実行前に走らせることを確認する（現状 DEF 時の検査は `check` でも
   走るはず。走らなければ `contractDecls.findings` ではなく実行と同じ ERROR として返す）。
7. 失う完全性を文書に明記する。ブロックを受けて EXEC する Word に、その Word 自身を名指す
   ブロックを渡せなくなる:

   ```
   [ 'F' BIND F EXEC ] 'APPLY' DEF [ [ 1 ] APPLY ] APPLY
   ```

   は現状 `1/1` を返して止まるが、改修後は BIND で拒まれる。この判断は §3 D1。

**受け入れ条件。**

- 再現二つが `selfReferentialDefinition` で止まり、`executionSteps` が一桁。
- `cargo test --all-targets` で `termination_laws` が通り、新しい refused/accepted 事例を含む。
- `rust/examples/speed_bench.rs` の合計が main と同等（DEF/BIND の検査は実行ループ外）。
- `spec/CHANGELOG.md` の beta.2（または次）の節に `breaking` として、前後の例
  （上の再現と、失う完全性の例）を書く。

### 段 3 — `recursionLimitExceeded` を資源上限に畳む（可能性 1）

**事実。** `spec/outcomes.json` はこの分類を「非循環の長い呼び出し鎖が入れ子上限に達した」
と説明する。段 2 の後、それが唯一の到達経路になる（W1→W2→…→W300 のような鎖）。それは
ホストの上限であり、0.9.0 の規則「ホストの上限は値ではなく、上限の名前を持つ ERROR」に
揃えるべきもの。

**手順。**

1. `rust/src/interpreter/interpreter_core.rs` の `MAX_USER_WORD_DEPTH` を
   `runtime_limits.rs` の上限の一つ `max_call_depth`（既定 256）にし、超過は
   `resourceLimitExceeded` で `resource: "callDepth"` を名指す。`execute_builtin.rs` の
   二か所（98 行・171 行付近）の発火点を置き換える。
2. `spec/outcomes.json` から `recursionLimitExceeded` を外す（`outcome-bijection:check` と
   `outcome-registry:check` が整合を要求する）。`ResourceLimit` の列挙に `CallDepth` を足し、
   `ajisai://limits` のプロファイル表（`mcp-host-profiles.md`）に行を足す。
3. `spec/termination.json` の `claim` はそのまま（「上限は費用を縛り、終了を決めない」）。
   `recursionSites[userWordCall].why` に「鎖の長さは辞書の語数で縛られ、`callDepth` は
   その費用の上限」と足す。
4. `rust/src/runtime_limits_tests.rs` の該当テスト（371 行・401 行付近）を新しい分類に直す。
   適合コーパスに「長い非循環鎖が `callDepth` で止まる」事例は入れない（上限値はホストの
   プロファイルで、現象ではない。`PORTABILITY.md` 11）。
5. 段 1 の CHANGELOG に `breaking`（エラー分類の名前が変わる）として書く。

**受け入れ条件。** `W1→…→W300` の鎖が `error:resourceLimitExceeded` と
`error.resource == "callDepth"` を返し、段 2 の再現二つは `callDepth` に触れずに止まる。

### 段 4 — 名前の渡し方を一本化（P3、可能性 4）

**事実。**

| Word | 名前の受け取り | 根拠 |
| --- | --- | --- |
| DEF / BIND / DEL | String `'INC'` | `spec/words.json` の `stack.domains.operands` |
| CONTRACT | Symbol か block のみ。String は `notASymbol` ERROR | 同上 |
| DIGEST | 何でも。`'INC' DIGEST` は文字列 `'INC'` の表示のダイジェストを黙って返す | 同上 |

初見の AI は `'INC' CONTRACT`（ERROR）、`INC CONTRACT`（INC が実行されて underflow）、
`'INC' DIGEST 'INC2' DIGEST EQ`（同じ本体でも FALSE）と三回外す。Symbol のリテラル構文は
なく、`[ INC ] 0 GET` の慣用句を知らないと届かない。「等しいダイジェストは一つのものを
意味する」という約束の下で、三つ目は誤読を誘う。

**既定の設計（§3 D3）。** 辞書族の五語で「String は名前」に揃える。

| Word | 改修後 |
| --- | --- |
| CONTRACT | String は Word の名前。名指す Word がなければ `notFound` を射影（Symbol と同じ規則）。Symbol と block は従来どおり。数など名前にも code にもならない値は従来の ERROR（分類名は `notASymbol` から `notAName` に改め、DEF/DEL の `nonText` と同じ扱いにするかは §3 D3） |
| DIGEST | String は Word の名前。名指す Word がなければ `notFound` を射影。それ以外の値は従来どおり表示のダイジェスト |

採らない案: Symbol のリテラル字句を足す。`ajisai-authoring-style.md` §11 の文字配分の
規準（空いている枠を消費する判断）に掛かり、`[ X ] 0 GET` の慣用句と二重になる。

**手順。**

1. `spec/words.json` の CONTRACT・DIGEST の `stack.domains.operands`、`nilPolicy`、
   `projection`、`errorWhen`、`documentation` を直す。`npm run word-registry:generate`。
2. Rust の実装（`executorKey` が `Contract`・`Digest` の実行関数）を直す。
3. `spec/language-semantics.md` の LANG.CONTRACT.REGISTRY と LANG.DICTIONARY.MUTATION の
   記述（「DIGEST answers that identity for a Symbol naming a Word」）を直す。
4. 適合コーパスに 4 件: `'INC' CONTRACT 'inputs' GET` → `1/1`、
   `[ 1 ADD ] 'INC' DEF [ 1 ADD ] 'INC2' DEF 'INC' DIGEST 'INC2' DIGEST EQ` → `TRUE`、
   `'NOPE' DIGEST NIL-REASON` → `'notFound'`、`'abc' 'abc' DIGEST`（文字列値のダイジェスト
   が必要な場合の書き方を決めたうえで。既定: 必要ないと判断し、事例は入れない）。
5. `rust/tests/identity_laws.rs`（`rust/src/identity_laws.rs`）に「同じ本体の二つの
   User Word は同じ内容識別を持つ」法則が String 経由でも成り立つことを足す。
6. SKILL.md 生成（§9 の「落とし穴」節）と MCP の `word_contract` の説明、quickstart の
   辞書の節に、三つの書き方（String・`[ X ] 0 GET`・block）を一つの表で示す。
7. CHANGELOG に `breaking`（`'INC' CONTRACT` が ERROR から値に、`'INC' DIGEST` の答えが
   変わる）として書く。

**受け入れ条件。** 上の 3 事例が通り、`[ INC ] 0 GET CONTRACT` と `'INC' CONTRACT` が
同じ Record を返す。

### 段 5 — JSON/SQL 境界を明示する（P9）

**事実。** `1/0 JSON-ENCODE JSON-DECODE` は String `'1/0'`。`[ 1 0/0 ] JSON-ENCODE` は
`'[1,"0/0"]'`。仕様（JSON-ENCODE の summary）は正しく書いているが、適合コーパスに事例が
なく、Reference と quickstart は SQL 親和性を掲げながらこの型変化に触れない。

**手順。**

1. 適合コーパスに 3 件: 上の二つと `'1/0' NUM 1/0 EQ` → `TRUE`。
2. Reference（`public/docs/ja/index.html` を先に、英語は日本語から）に「JSON と SQL の境界」
   の節を足す。内容は `sql-affinity.md` の対応表を要約し、「非十進分数と二つの点は文字列で
   往復し、受け手が `NUM` を掛ける」を一行で言う。両言語の節 ID は同じにする。
3. quickstart（`tools/mcp-server/assets/quickstart.md` の手書き半分）の JSON の節に同じ
   一行と例を足す。`SKILL.md` の落とし穴節にも一行。
4. 正典は変えない。CHANGELOG は `editorial`（コーパスへの追加は現象の記録で意味の変更
   ではないが、段 1 の `definitionalDigest` には入らない。コーパスは `spec/` の外）。

### 段 6 — コスト契約に値依存の軸を足す（P6、可能性 6）

**事実。**

```
[ 2 3 5 7 11 13 17 19 23 29 31 37 41 43 ] SQRT [ 1 ADD ] MAP [ 1 ] [ MUL ] FOLD
```

MUL の `cost.numeric.class` は `const`、実測 `numericWork` は 2,123,992、
`algebraicTerms` 上限 512 で `resourceLimitExceeded`。級は入力長の関数で、値の次数
（素な根号 k 個で 2^k 項）を見ない。`word_contract` の説明は 10^2〜10^3 倍と書くが、
機械可読の契約には現れない。

**手順。**

1. `spec/words.schema.json` の `costAxis` に任意の `valueDependent` を足す。値は
   `"algebraicTerms"` の一種類で、「この軸の費用は、入力長のほかに被演算子の `exactTerms`
   の項数にも比例して増え、上限は `algebraicTerms`」を意味する。
2. `spec/words.json` で、無理数を演算・比較・整列・ダイジェストする Word の
   `cost.numeric` に `valueDependent: "algebraicTerms"` を付ける（knowledgeOrder 全語、
   EQ・LT・GT・MIN・MAX、SORT・ORDER・UNIQUE・BSEARCH・TALLY・GROUP、STR・FORMAT・DIGEST、
   RECONCILE）。付けるべき語の列は実装から機械的に出す: `types/exact` を呼ぶ実行関数を
   持つ Word（`check-unreachable-contract.mjs` と同じ到達性の読みで、スクリプトに固定する）。
3. `resourceUsage` に `algebraicTermsPeak`（その実行で最大だった項数）を足す
   （`host-protocol.schema.json` の任意フィールド、プロトコル版は上げない）。
4. `word_contract` と quickstart の予算の文を、この軸を読む書き方に直す。「級は増え方で、
   大きさではない。`valueDependent` の語は小さな実行で `algebraicTermsPeak` を読んでから
   見積もる」。
5. `check-semantic-kernel.mjs`・`check-termination.mjs` は `cost.steps.class` だけを読むので
   影響なし。`check-formalization-coverage.mjs` が `cost` を検査していれば新フィールドを
   通す。
6. CHANGELOG は `compatible`（契約に欄が増える。既存欄の意味は変わらない）。ただし
   `definitionalDigest` は動くので版は上がる（段 2〜4 と同じリリースにまとめる）。

**受け入れ条件。** 上の再現の結果に `algebraicTermsPeak` があり、`word_contract MUL` の
`cost.numeric.valueDependent` が `algebraicTerms`。

### 段 7 — ゲートの盲点（P7）

**事実。**

| 層 | 規模 |
| --- | ---: |
| Rust 本体（計画書の数え方） | 約 21,000 行 |
| Rust テスト | 約 33,500 行 |
| ゲートスクリプト 35 本 | 4,098 行 |
| 適合コーパス | 405 件、`data-category` は全件 `core` |

ゲートは数の一致・生成物の drift（安い検査）を守り、P1（停止性）と P2（版）を守って
いなかった。段 1〜3 で二つを塞ぐ。本段はコーパスの分類と網羅を直す。

**手順。**

1. `data-category` を語の族（`spec/words.json` の `family`）か節 ID にする（§3 D6、
   既定は族。語に依らない事例は節 ID の先頭 `LANG.VALUES` などを使う）。405 件を機械的に
   付け直すスクリプトを `scripts/` に一度だけ置き、使ったら消す（`#:` コメントの LANG.*
   参照と `ajisai-source` に現れる語から決める）。
2. `check-conformance-coverage.mjs` を拡張する: (a) 全 Core 語が現れる（現状）、(b)
   `spec/outcomes.json` の全 `nilReasons` と、`attribution` が `machine` でない全
   `errorCategories` に少なくとも一つ `ajisai-expect-error` / 結果の事例がある、(c) 族ごとの
   件数を表で出す。(b) で欠けている分類は本段で事例を足す。
3. `.github/workflows/test.yml` に段 1 の `check:spec-version` を加え（段 1 で済んで
   いれば確認のみ）、段 2 の法則が `cargo test` に含まれていることを `test.yml` のコメントに
   書く。
4. `docs/quality/TRACEABILITY_MATRIX.md` と `check-traceability-matrix.mjs` に、
   LANG.DICTIONARY.ACYCLIC → `termination_laws.rs` と適合事例、LANG.AUTHORITY.* → 
   `check-spec-version.mjs` の行を足す。

### 段 8 — MCP に語彙を渡す（P4、可能性 3）

**事実。** MCP は毎回新規セッションで User Word は持続しない（`tools/mcp-server/index.js`
237 行の説明）。phrasebook の 17 語は Playground（IndexedDB）でだけ生きる。「真のユーザー」
と Reference が呼ぶ AI は、概念 6（内容アドレス辞書）に MCP から触れられない。

**設計。** セッションの持続は入れない（再現性と「一呼び出し一ソース」を守る）。代わりに
呼び出しを自己完結のまま太らせる。

| 追加 | 内容 |
| --- | --- |
| 引数 `words`（compute / check / infer_contracts） | phrasebook の語名の配列。サーバーが phrasebook から、その語と依存する語の定義を元の順で `source` の前に付けて実行する。知らない名前は `invalidRequest` で候補を返す |
| 結果の `prelude` | 付けた語の `[{ name, digest }]`。`digest` は語の内容識別（DIGEST の答え） |
| 資源 `ajisai://phrasebook` | `assets/phrasebook.ajisai` をそのまま返す。`sync-assets.js` が `docs/phrasebook/phrasebook.ajisai` との一致を検査する |
| phrasebook の `#:digest` 行 | 各 `DEF` の直後に `#:digest <hex>` を置く。`rust/tests/phrasebook_laws.rs` が、ファイルを頭から実行した時点の `[ NAME ] 0 GET DIGEST` と一致することを検査する。段 4 の後は `'NAME' DIGEST` と書ける |

`words` で付く定義は `source` と合わせて 65,536 バイトの上限に数える。

**手順。**

1. `docs/phrasebook/phrasebook.ajisai` に `#:digest` を付け、`phrasebook_laws.rs` に検査を
   足す。digest の値は実装が出したものを記録する（手で書かない）。
2. `tools/mcp-server/index.js` の `sourceSchema` に `words` を足し、三つのツールの説明に
   一文足す。`result.schema.json` に `prelude` を足す。`selftest.js` と `golden/cases.json`
   に `words: ["MEAN"]` の往復を一件、未知の名前の拒否を一件。
3. `eval/cases.json` に、phrasebook の語を使えば一語で済む課題を 3 件足す（二言語）。
4. `tools/mcp-server/CHANGELOG.md` に minor として書く（ツールの引数が増える）。

**受け入れ条件。** `compute { source: "[ 1 2 3 ] MEAN", words: ["MEAN"] }` が `2/1` と
`prelude: [{ name: "MEAN", digest: … }]` を返し、その digest が phrasebook の `#:digest` と
一致する。

### 段 9 — `SKILL.md` と quickstart の二層化（P10）

**事実。** `SKILL.md` 63.5 KB（約 16k トークン）、`tools/mcp-server/assets/quickstart.md`
79 KB。77 語に対し 1 語あたり約 2.9 KB の説明。初見の AI は毎回これを読む。

**手順。**

1. `scripts/generate-skill-md.mjs` の出力を二つにする。`SKILL.md` は規約・十の規則・落とし穴
   （段 4 の名前の表、段 5 の JSON 境界、段 2 のブロック渡しの制限、FOLD の種の形、
   `[ X ] 0 GET`）・予算（段 6 の軸を含む）・語の一覧表（名前と一行の `hover` だけ）。
   `skill/words.md`（新設）に各語の全文 summary と例。SKILL.md から相対リンクで参照する。
2. `SKILL.md` にバイト数の予算をゲートで持たせる（§3 D4、既定 16,384 バイト）。
   `--check` が予算超過で失敗する。
3. quickstart も同じ二層にする: `ajisai://guide/quickstart` を規約と落とし穴に絞り、語の
   全文は `ajisai://contracts`（既存）に任せる。`eval/performance.json` の応答バイト予算を
   それに合わせて下げる。
4. 段 8 の `words` 引数を quickstart の最初の節で紹介する。

**受け入れ条件。** `npm run check:skill` が予算を検査し、`SKILL.md` の全サンプルが引き続き
`check-reference-samples.mjs` と同じ方式で実行されて一致する（現状どおり）。

### 段 10 — 正典の節ごとの日本語要約（P8）

**事実。** 仕様は英語だけが正典。設計根拠（`docs/dev/`）は日本語。Reference は日本語から
英語を生成する（50c66f2）。著者の意図から正典への翻訳 drift を検査する手段がない。
語には `docs/i18n/word-summaries.ja.json` と鮮度ダイジェストの仕組みがある
（`generate-word-reference.mjs` 177〜187 行）。

**手順。**

1. `docs/i18n/clause-summaries.ja.json` を新設する。`language-semantics.md` の全 LANG.* 節に
   ついて `{ clause, source: <英文節のダイジェスト>, summary: <日本語 3〜6 行> }`。
2. `generate-specification.mjs` が、節ごとに日本語要約を折りたたみ（`<details>`）で
   `SPECIFICATION.html` に埋め、`--check` で (a) 全節に要約がある、(b) `source` が現在の
   英文と一致する（語の要約と同じ「英文が変わったら日本語を見直す」規則）を検査する。
3. `check-docs-dev-drift.mjs` に、`[方針記録]` 文書が本文で名指す LANG.* 節 ID がすべて
   `language-semantics.md` に存在することの検査を足す（文字列一致。既存の規則と同じ
   ヒューリスティック）。
4. 全文の日本語仕様は作らない（§3 D7）。

**受け入れ条件。** `npm run specification:check` が節要約の欠落と鮮度を検査し、
`SPECIFICATION.html` に全節の日本語要約が入る。

### 段 11 — 語彙創発実験 Phase 2（可能性 3）

**事実。** `lexicon-emergence-pilot-results-2026-09-23.md` の停止条件に該当して Phase 1 で
止まっている。反例の原因だった `KEEP` は退役済み。Phase 2 の前提（同書 §5）は、系統の
異なるモデルの混在、辞書ありなしで差が出る難度の課題、辞書の配布経路。

**手順。**

1. `tools/lexicon-emergence/` の配布経路を SKILL.md 貼り付けから、段 8 の `words` 引数に
   変える（辞書 = phrasebook 形式のファイル、`#:digest` 付き）。H5（監査可能性）の計測は
   `prelude` の digest と、`words` を外して本体を展開した実行の結果の一致で取る。
2. 課題は正答率 100% にならない難度にし（パイロットの記録）、系統の異なるモデルを 2 つ
   以上混ぜる。
3. 結果は `docs/dev/lexicon-emergence-phase2-results-<date>.md` に `[観察ノート]` として
   記録し、生データは読むコードがなければ削除する（パイロットと同じ扱い）。

### 段 12 — 停止性の定理を機械化する（可能性 1）

**手順。**

1. `rust/tests/test_support/generators.rs` に、非循環の User 辞書（段 2 の規則を満たす
   DEF 列）と、ブロック渡しを含むプログラム（`[ … ] APPLY` の形）の proptest 生成器を足す。
2. 法則「生成した任意のプログラムは `callDepth` に触れずに終わる」を `termination_laws.rs`
   に足す。到達上限は `callDepth` 以外（`executionSteps` など）なら許す。
3. 逆向きの法則「段 2 の検査を外した実装（テスト専用フラグ）では再現プログラムが
   `callDepth` に触れる」を置き、検査が仕事をしていることを示す。
4. Lean/Coq による形式化は、上の二法則が一か月安定してから検討する（本書の範囲外）。

---

## 3. 作者の判断が必要なこと（既定値付き）

| ID | 判断 | 既定値 | 代案と費用 |
| --- | --- | --- | --- |
| D1 | 停止保証をどう扱うか（段 2 の着手時に前提が変わった。上の「状態」を参照） | なし（作者が選ぶ）。推奨は (B) | (A) 認める: 入れ子の上限 `callDepth` を正典の上限にし（段 3 がその実装）、主張を「入れ子は `callDepth`、仕事は `executionSteps` で縛られるので必ず止まる」に改める。README・LANG.DICTIONARY.ACYCLIC・`termination.json` の「全域・非チューリング完全」を撤回。基底つきの再帰が書ける言語として説明し直す。実装の追加はほぼない。(B) 全域性を取り戻す: 「動的な実行」を、コードを走らせる語（EXEC・MAP・FILTER・FOLD・SCAN）のうち、直前に書かれたリテラルのブロックでも直前に読んだ束縛名でもない被演算子を走らせるものと定義し、動的に走らせるブロックは、それ自身と、そこから User Word の本体を辿って届くコードに、動的な実行を一つも含んではならない（含めば実行時に新しい ERROR で拒む。判定はブロックの内容だけで決まるのでダイジェストで記憶できる）。動的な実行は入れ子にならず、その内側はリテラルの入れ子と非循環の辞書で減るので、全域性が証明できる。上の二例はどちらも拒まれる。適合コーパスの既存事例は束縛名経由なので通る見込み。費用は、ブロックを受け取りさらにスタック上のブロックを走らせる高階の組み立て（コンビネータ）を書けなくなることと、実装と仕様の追加 |
| D2 | beta.2 で実装の版も上げるか | 上げる（`check-version-sync.mjs` の四つと `version-lock.json` を同じ値にし、一つの番号で語る） | 仕様版と実装版を別々に動かす（`spec/README.md` の現行の建前。利用者が二つの番号を追う） |
| D3 | DIGEST の String 被演算子 | Word の名前。名指す Word がなければ `notFound` | 現状維持（表示のダイジェスト）＋ `check` の note。黙って別の物を測る罠は残る |
| D4 | `SKILL.md` のバイト予算 | 16,384 | 8,192（語の一覧表も外へ出す） |
| D5 | `callDepth` の既定値 | 256（現行） | 1,024。Rust の既定スタックで安全か `runtime_limits_tests.rs` で確かめてから |
| D6 | 適合コーパスの分類軸 | 語の族 | 節 ID（細かいが 405 件の付け直しが人手になる） |
| D7 | 正典の日本語化の範囲 | 節ごとの要約（段 10） | 全文の日本語仕様を第二の正典にする。二重保守になる |

---

## 4. 完了の定義

- §1 の全段が main にマージされ、`spec/CHANGELOG.md` に beta.2 以後の全変更が種別付きで
  ある。
- 2026-10-15 の観察の再現プログラム（段 2・4・5・6 に記載）が、すべて本書の受け入れ条件の
  答えを返す。
- 本書の各節は生きている文書へ移され、本書と INDEX.md の行が削除されている。
