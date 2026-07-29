# タスクメモ

作業中の課題と判断ログ。着手したら `[ ]` を `[x]` にして、根拠になった commit / 調査結果を1行添える。

## 未着手

### バグ

- [ ] 素材画像の拡張子判定が CLI と GUI で食い違っている
  - `src/main.rs:181` … `"png" | "jpg" | "jpeg" | "webp"`
  - `src/gui/app_full.rs:1653` … `"png" | "jpg" | "jpeg"`（webp なし）
  - GUI からは `.webp` 素材が無言で無視される。同じ判定が2箇所にコピーされて drift した典型例なので、片方に寄せるより共有の判定関数へ切り出すのが筋
  - 参考: `.claude/skills/prefix-match-processor/SKILL.md`（この drift を発火条件に書いてある）

- [ ] キャスト前にオーバーフローしうる乗算
  - `src/lib.rs:127` … `let total_pixels = (width * height) as f64;`
  - `src/main.rs:502` … `let total_tiles = (grid_w * grid_h) as usize;`
  - どちらも u32 同士を掛けてから widen しているため、巨大入力ではキャストに到達する前に溢れる。`u64::from(width) * u64::from(height)` の形へ
  - 参考: `.claude/skills/explicit-primitive-conversion/SKILL.md`

- [ ] production パスの `expect` 除去
  - `src/gui/app_full.rs:322`, `src/gui/app_full.rs:757` … `.expect("Failed to load language catalog")`
  - GUI では panic がプロセスごとウィンドウを落とすため、`ProcessingState::Error` へ倒したい
  - `src/gui/app_full.rs:71-72` の `"en-US".parse().unwrap()` はリテラル由来で実質 infallible。優先度は低い
  - なお `src/main.rs` の unwrap/expect 約186件はすべて `#[cfg(test)]` 内で、対象外

### 実装中

- [ ] TileCache のタイル画像リサイズキャッシュ
  - `src/tile_cache.rs`（未追跡）、`src/lib.rs` / `src/main.rs` の呼び出し側改修が作業ツリーに残っている
  - TDD の GREEN までは到達済み。ベンチ計測とコミット分割が未了

## 完了

- [x] CLAUDE.md をこのリポジトリの運用ルールに差し替え — `d0141ae`
  - 自動生成のプロジェクト概要（ビルドコマンド、アーキテクチャ、CLI オプション約300行）は復元しない判断。README とコードで足りるため
  - 別プロジェクト（Spout/Syphon、webcam 回転）を指していた ref repository セクションは削除

- [x] Rust 規約 skills 7件を mosaic 文脈へ接地 — `e5a9322`
  - `.claude/skills/` … early-return-guards / branching-modeled-state-with-match / chaining-result-combinators / explicit-primitive-conversion / precise-type-modeling / prefix-match-processor / three-word-naming
  - 元は webcam→Syphon/Spout ツール向けに書かれていたため、例をこのクレートの実在 API（`UsageTracker`, `SimilarityDatabase`, `GridPosition`, `TileStatus`, `TileCache`, `OptimizationConfig`）へ置換
  - description は「発火条件のみ」に統一。処理内容を要約すると本文を読まずに description どおり動いてしまうため

## 判断ログ・保留

- [ ] `prefix-match-processor` の位置づけ再検討
  - このリポジトリに URI スキームのような prefix ファミリーが存在せず、拡張子トークン判定へ一般化して残した。上の webp drift を解消したら、skill 自体が役目を終える可能性がある

- [ ] `branching-modeled-state-with-match` の題材不足
  - 自前の多バリアント enum は `src/grid_visualizer.rs` の `TileStatus` のみ。他は `Option<PathBuf>` と `bool` で、exhaustive match の例として薄い。enum が増えた時点で例を足す

- GUI のフレームワークは **iced 0.12**（egui ではない）。`Cargo.toml:19` 参照。skill や資料を書く際に取り違えやすいので明記しておく
