![Rust](docs/assets/badges/rust.svg) ![WebAssembly](docs/assets/badges/webassembly.svg) ![TypeScript](docs/assets/badges/typescript.svg) ![Tauri](docs/assets/badges/tauri.svg)

# Lycoris

Rust で書いたコアを WebAssembly にして、TypeScript の Playground から動かすための雛形です。プログラミング言語 Ajisai の技術スタックと Playground の GUI をそのまま流用できるように切り出しています。

同梱のRustコアは、Playground のすべての機能を動かすためだけの小さなスタック言語（サンプル）です。新しいプロジェクトでは `rust/src/` を自分のコアに差し替えて使います。

```
[ DUP MUL ] 'SQUARE' DEF
[ 1 2 3 ] [ SQUARE ] MAP     # [ 1/1 4/1 9/1 ]
```

## 構成

| 場所 | 役割 |
|---|---|
| `rust/` | コア。`src/interpreter.rs` などがサンプル言語、`src/wasm.rs` が Playground とのやり取り（`wasm` feature で有効） |
| `src/wasm/generated/` | `npm run build:wasm` が書き出す wasm バンドル（コミット対象） |
| `src/wasm-interpreter-types.ts` | Rust と TypeScript の間の約束（インタープリタのメソッドと値の形） |
| `src/workers/` | wasm を Web Worker で実行する仕組み。重い処理や無限ループでも GUI が固まらず、タイムアウトと中断ができる |
| `src/gui/` | Playground の GUI（エディタ、出力、スタック、辞書、ステップ実行、保存と復元、レイアウト） |
| `src/platform/` | Web と Tauri（デスクトップ）の違いの吸収、cross-origin isolation |
| `src/styles/playground.css` | 配色とレイアウト。`npm run check:contrast` が WCAG のコントラストを検査 |
| `public/docs/` | Playground から開くリファレンス（日英）。例は「Playground で開く」でエディタに読み込める |
| `src-tauri/` | デスクトップアプリ（Tauri） |
| `docs/dev/gui-current-design-memory.md` | GUI の設計メモ |

## コアを差し替えるとき

1. `rust/src/` を自分のコアに書き換える。`src/wasm.rs` は `src/wasm-interpreter-types.ts` の `AjisaiInterpreter` と同じメソッドと戻り値の形を保つ
2. `npm run build:wasm` で `src/wasm/generated/` を作り直してコミットする
3. `public/docs/` のリファレンスと `src/gui/interpreter-state-persistence.ts` のサンプルワード（`EXAMPLE_USER_WORDS`）を新しい言語に合わせる

## ビルドと実行

| 作業 | コマンド |
|---|---|
| 依存関係のインストール | `npm ci` |
| 開発サーバー | `npm run dev` |
| wasm のビルド | `npm run build:wasm`（初回は `bash ./scripts/install-wasm-pack.sh`） |
| ブラウザ向けビルド | `npm run build` |
| デスクトップアプリのビルド | `npm run tauri:build` |
| TypeScript のテスト | `npm test` |
| Rust のテスト | `cargo test`（`rust/` で実行） |

`main` への push で GitHub Pages にデプロイされます（`.github/workflows/build.yml`）。

## ライセンス

MIT（[`LICENSE`](LICENSE)）。同梱しているサードパーティ製品は [`THIRD-PARTY-LICENSES.md`](THIRD-PARTY-LICENSES.md) を参照してください。
