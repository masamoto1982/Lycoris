# Lycoris

A Vector-based programming language inspired by FORTH and LISP, with perfect numerical precision through rational arithmetic.

## プロジェクト構成

```
lycoris/
├── src/
│   ├── lib.rs          # Rustコア実装（分数演算、インタープリタ）
│   ├── main.ts         # TypeScript UI（IndexedDB永続化）
│   └── worker.ts       # Web Worker（重い計算を別スレッドで実行）
├── www/
│   ├── index.html      # エントリーポイント
│   ├── styles.css      # UIスタイル
│   ├── js/            # (ビルド生成) TypeScriptコンパイル結果
│   │   ├── main.js
│   │   └── worker.js
│   └── pkg/           # (ビルド生成) WebAssemblyモジュール
├── .github/
│   └── workflows/
│       └── deploy.yml  # GitHub Pages自動デプロイ
├── Cargo.toml         # Rust設定
├── package.json       # Node.js設定
└── tsconfig.json      # TypeScript設定
```

## 主要機能

### 1. 完全精度演算
- すべての数値を内部的に分数（BigRational）として扱う
- 丸め誤差が一切発生しない
- 超巨大数のサポート（1e61など）

### 2. Web Worker統合
- 重い計算処理を別スレッドで実行
- UIのフリーズを防止
- 超巨大数演算でもレスポンシブなUI維持

### 3. IndexedDB永続化
- カスタムワード辞書をブラウザDBに保存
- セッション間でのデータ保持
- LocalStorageより大容量

### 4. レスポンシブUI
- デスクトップ：2x2グリッドレイアウト
- モバイル：画面切り替え式
- タッチ操作対応

## データ型

- **Number**: 任意精度の分数
- **Boolean**: true/false
- **String**: 文字列（"text"）
- **Nil**: null値
- **Vector**: 他の型を含む配列、ネスト可能

## 基本構文例

```lycoris
# 基本演算（完全精度）
1 3 DIVIDE              # 1/3（正確な分数）
1 3 DIVIDE 3 MULTIPLY   # 1（誤差なし）

# 超巨大数演算（Web Workerで実行）
1e61 1e61 MULTIPLY      # 1e122
100 100 POWER           # 100^100（巨大数）

# Vector操作
[1 2 3] 10 ADD         # [11 12 13]
[1 2 3] -1 GET         # 3（負インデックス）
[[1 2] [3 4]]          # ネストVector

# カスタムワード定義
DUPLICATE MULTIPLY 'SQUARE DEF
5 SQUARE               # 25

# 条件分岐（ガード節スタイル）
x 0 GREATER : x 2 MULTIPLY :
x 0 LESS : x -1 MULTIPLY :
0                      # デフォルト
```

## 組み込みワード

### 算術演算
- `ADD`, `SUBTRACT`, `MULTIPLY`, `DIVIDE`, `POWER`

### スタック操作
- `DUPLICATE` - 複製
- `DROP` - 削除
- `SWAP` - 交換

### Vector操作
- `GET` - 要素取得（負インデックス対応）
- `SET` - 要素設定
- `APPEND` - 追加
- `CONCAT` - 結合

### 辞書管理
- `DEF` - カスタムワード定義
- `DELETE` - 削除

### I/O
- `PRINT` - 出力
- `CLEAR` - クリア

## ビルドと実行

```bash
# 依存関係インストール
npm install

# Rustツールチェインインストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# wasm-packインストール
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# ビルド
npm run build

# 開発サーバー起動（http://localhost:8080）
npm run dev

# TypeScriptのウォッチモード
npm run watch
```

## 技術スタック

- **Rust**: WebAssemblyコンパイル、BigRational演算
- **WebAssembly**: 高速実行環境
- **TypeScript**: UI制御、型安全性
- **Web Worker**: 並列処理、UIブロック防止
- **IndexedDB**: 永続化ストレージ
- **GitHub Actions**: 自動デプロイ

## 特徴的な設計

1. **分数ベース演算**: 浮動小数点数を使わない完全精度
2. **トライ木辞書**: 効率的なワード管理
3. **後置記法**: FORTHスタイルの一貫した構文
4. **Vector中心**: LISPのリストに相当する汎用データ構造
5. **非同期処理**: Web Workerによる並列実行

## パフォーマンス考慮事項

- 超巨大数演算は自動的にWeb Workerで処理
- 最大60秒のタイムアウト設定
- POWER演算は指数10000まで制限
- UIは計算中でもレスポンシブ

## ライセンス

MIT License

## 作者

masamoto yamashiro
