# SwitchBot API仕様調査

## このリポジトリについて

SwitchBot APIの仕様を確認しつつ、自作WebアプリからAPIを叩く練習としたい。

## ファイルとその目的

- test.js
`$ node test.js` で実行してAPIを簡単に叩けるもの。
APIを叩くために最初に作った部分なのでテスト用途に

- backend
rust(axum)のwebサーバ。secretやらtokenやらを隠すために噛ませている。
最終的にSwitchBot APIを叩くだけのAPIとして使い、Reactのバックエンドにしたい。

- React (未実装)
最終的にReact->backend->SwitchBotと通信できるようにしたい。
