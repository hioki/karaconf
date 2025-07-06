# Karaconf

## プロジェクトの概要と目的 (何を作っているか、なぜ作っているか)

[Karabiner-Elements](https://karabiner-elements.pqrs.org/) の設定ファイル (`$HOME/.config/karabiner/karabiner.json`) を Rust で書くためのツールです。

大量に存在する karabiner の設定を `karabiner.json` を直接編集して管理するのは大変なため、Rust で設定を生成・更新するツールを作成しました。

## 技術スタック (使用している言語、フレームワーク、ライブラリ)

- Rust

## ファイル構造の説明 (主要なディレクトリ・ファイルの役割)

- `src/main.rs`: メインの実行ファイル。Karabiner-Elements の設定を生成します。
- `src/rule_sets/`: Karabiner-Elements のルールセットを定義するモジュール。各ルールセットは個別のファイルに分かれています。

## 実装上の重要な決定事項 (なぜその設計にしたか、制約事項など)

- 特になし。

## 既知の問題・TODO (未解決の課題や今後の改善点)

- 他の人が使うことを想定していない。

## セットアップ・実行方法 (環境構築や動作確認の手順)

```console
$ cargo run
```
