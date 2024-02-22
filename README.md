# Chinese Trainer

[![codecov](https://codecov.io/gh/0918nobita/chinese-trainer/branch/main/graph/badge.svg?token=MJ94QXFAZ1)](https://codecov.io/gh/0918nobita/chinese-trainer)

中国語の学習を支援するアプリを作ろうとしています

## 実装したい機能

- 単語 (簡体字 + ピンイン + 意味) を登録できる
- 登録済みの単語の閲覧・タグ付け
  - リスト表示
  - 単語カード
- 単語テストの問題を生成できる
  - 出題範囲 (登録日の期間とタグで絞り込み)
  - 出題順序 (登録順 or ランダム)
  - 出題形式
    - 意味を見て簡体字で答える
    - 意味を見てピンインで答える
    - 簡体字を見て意味を答える
    - 簡体字を見てピンインを答える
  - 解答形式
    - 直接入力
    - (ピンインを答える問題のみ) 正解のピンインと、子音や声調が異なる複数の自動生成されたピンインからの択一式
  - 採点方法
    - 全ての出題形式で手動採点ができる
    - 意味を答える問題以外は自動採点ができる
  - 採点結果表示
    - 単語を手動で複数選択してタグをまとめて追加・削除できる
    - 不正解だった単語すべてを自動選択できる

文法・用法に関する問題も作成できるようにしたいが、後回し

## データベースの作成

```bash
$ sqlx database create --database-url "sqlite:./database.sqlite3"
```
