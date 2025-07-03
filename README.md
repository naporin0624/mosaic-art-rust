# Mosaic Art Generator - Rust Edition

高速なRust実装のモザイクアート生成ツール。Pythonバージョンよりも大幅に高速化されています。

## 特徴

- **高速処理**: Lab色空間とk-d treeによる高速マッチング
- **並列処理**: Rayonによる自動並列化
- **メモリ効率**: Arcによる効率的なメモリ管理
- **アスペクト比フィルタリング**: 対象画像と同じアスペクト比の素材のみ使用
- **SIMD最適化**: fast_image_resizeによる高速リサイズ

## 技術的な最適化

- **Lab色空間**: 人間の知覚に近い色空間での色マッチング
- **k-d tree**: O(log n)の高速最近傍探索
- **並列画像処理**: タイルの処理を並列化
- **キャッシュ効率**: 素材画像のメタデータを事前計算

## インストール

```bash
# mise を使用
cd mosaic-rust
mise install
mise trust

# ビルド（デバッグ版）
cargo build

# ビルド（リリース版・最適化有効）
cargo build --release
```

## 使用方法

```bash
# 基本的な使用
cargo run --release -- \
  --target ../yoko.png \
  --material-src ../sozai \
  --output ../product/mosaic_rust.png

# 詳細なオプション指定
cargo run --release -- \
  --target ../yoko.png \
  --material-src ../sozai \
  --output ../product/mosaic_rust.png \
  --grid-w 64 \
  --grid-h 36 \
  --max-materials 1000 \
  --aspect-tolerance 0.05
```

## コマンドラインオプション

| オプション | 説明 | デフォルト |
|-----------|------|-----------|
| `--target` | 対象画像パス | 必須 |
| `--material-src` | 素材画像ディレクトリ | 必須 |
| `--output` | 出力ファイルパス | 必須 |
| `--grid-w` | 横方向のタイル数 | 50 |
| `--grid-h` | 縦方向のタイル数 | 28 |
| `--max-materials` | 使用する最大素材数 | 500 |
| `--aspect-tolerance` | アスペクト比許容誤差 | 0.1 |

## パフォーマンス比較

Python版と比較して期待される改善:

- **素材読み込み**: 5-10倍高速（並列処理）
- **色計算**: 3-5倍高速（Lab色空間の効率的な実装）
- **最近傍探索**: 100倍以上高速（k-d tree vs 線形探索）
- **画像リサイズ**: 5-10倍高速（SIMD最適化）
- **全体処理時間**: 10-20倍高速

## ビルド最適化

リリースビルドでは以下の最適化が有効:

```toml
[profile.release]
lto = true          # Link Time Optimization
opt-level = 3       # 最大最適化
codegen-units = 1   # 単一コード生成ユニット
```

## 依存関係

- `image`: 画像の読み込み・保存
- `fast_image_resize`: SIMD最適化されたリサイズ
- `palette`: Lab色空間変換
- `kiddo`: k-d tree実装
- `rayon`: データ並列処理
- `clap`: CLIパーサー
- `indicatif`: プログレスバー表示