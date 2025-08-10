# branchify

branchify は、標準入力からファイルパスのリストを受け取ってディレクトリツリー形式で出力するシンプルなCLIツールです。

```console
$ find . -type f | sed 's|^\./||'
src/main.rs
src/lib/tree_generator.rs
target/debug/branchify
Cargo.toml
README.md

$ find . -type f | sed 's|^\./||' | ./target/debug/branchify
├── Cargo.toml
├── README.md
├── src
│   ├── lib
│   │   └── tree_generator.rs
│   └── main.rs
└── target
    └── debug
        └── branchify
```

## 機能

標準入力からファイルパスのリストを読み込み、ディレクトリとファイルの階層構造を出力します。

## 使用方法

このツールは、他のコマンドの出力をパイプで受け取ることを想定して設計されています。

## 開発

- ビルド: `cargo build`
- 実行: `cargo run`
- テスト: `cargo test`
