# Release workflow

1. `release-pr` が作成した release PR を確認してマージする
2. `master` ブランチから `Release` GitHub Actions workflow を手動実行する

3 crate は同じバージョンで crates.io に publish され、それぞれに Git tag と
GitHub Release が作成されます。`lmml-cli` の各プラットフォーム向けバイナリは、
`lmml-cli` の Release にだけ添付されます。
