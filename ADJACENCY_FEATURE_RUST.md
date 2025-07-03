# Rust実装: 隣接タイル類似度回避機能 + 色調整機能

Rust版のモザイクアート生成ツールに、以下の高度な機能を実装しました：
1. 隣接するタイルに似たような画像が来ないようにする機能
2. タイルの色を元画像に合わせて調整する色調整機能

## 新機能の概要

### 1. 類似度データベース (JSON形式)
- Lab色空間での画像間の類似度を事前計算
- JSON形式で保存・読み込み
- 初回実行時に自動構築

### 2. 隣接ペナルティシステム
- 隣接タイルの類似度に基づくペナルティ計算
- 色マッチングと隣接ペナルティの総合スコアでタイル選択
- 重み調整可能なパラメータ

### 3. シミュレーテッドアニーリング最適化
- 配置後のスワッピングによる改善
- 温度スケジュールによる局所最適解の回避
- 設定可能な反復回数

### 4. 高度な色調整機能
- **輝度調整**: タイルの明るさを元画像の各領域に合わせて自動調整
- **色相調整**: HSV色空間でのインテリジェントな色相シフト
- **彩度調整**: 元画像の彩度に応じた適応的な彩度調整
- **調整強度制御**: 0.0-1.0の範囲で調整の強さを制御可能

## 使用方法

### 基本的な使用法（色調整 + 隣接ペナルティ）
```bash
# 色調整と隣接ペナルティを両方有効にした高品質生成
cargo run --release -- \
  --target ../yoko.png \
  --material-src ../material/euph_part_icon \
  --output ../product/rust_mosaic_enhanced.png \
  --grid-w 100 \
  --grid-h 56 \
  --adjacency-penalty-weight 0.3 \
  --color-adjustment-strength 0.3
```

### 色調整機能のみ使用
```bash
# 隣接ペナルティなしで色調整のみ
cargo run --release -- \
  --target ../yoko.png \
  --material-src ../material/euph_part_icon \
  --output ../product/rust_mosaic_color_only.png \
  --grid-w 100 \
  --grid-h 56 \
  --adjacency-penalty-weight 0.0 \
  --color-adjustment-strength 0.4
```

### 最適化を無効にする場合
```bash
cargo run --release -- \
  --target ../yoko.png \
  --material-src ../material/euph_part_icon \
  --output ../product/rust_mosaic_no_opt.png \
  --grid-w 50 \
  --grid-h 28 \
  --enable-optimization false
```

### パラメータの詳細調整
```bash
cargo run --release -- \
  --target ../tate.png \
  --material-src ../material/euph_part_icon \
  --output ../product/rust_mosaic_custom.png \
  --grid-w 60 \
  --grid-h 107 \
  --adjacency-penalty-weight 0.5 \
  --optimization-iterations 2000 \
  --similarity-db custom_similarity.json
```

## 新しいコマンドライン引数

| 引数 | デフォルト値 | 説明 |
|------|-------------|------|
| `--adjacency-penalty-weight` | 0.3 | 隣接類似度ペナルティの重み（0.0で無効） |
| `--enable-optimization` | true | 最適化フェーズを有効化 |
| `--optimization-iterations` | 1000 | 最適化の反復回数 |
| `--similarity-db` | similarity_db.json | 類似度データベースのパス |
| `--rebuild-similarity-db` | false | 類似度データベースを強制的に再構築 |
| `--color-adjustment-strength` | 0.3 | 色調整の強度（0.0で無効、1.0で最大） |

## パフォーマンス比較

Python実装と比較したRust実装の利点:
- **処理速度**: 約5-10倍高速
- **メモリ効率**: 並列処理での効率的なメモリ使用
- **スケーラビリティ**: 大規模グリッド（200x200以上）でも実用的な速度

### ベンチマーク例（100x56グリッド）
- Python実装: 約60-90秒
- Rust実装: 約10-15秒

## 技術的詳細

### アーキテクチャ
1. **モジュール構成**:
   - `similarity.rs`: 類似度データベース管理
   - `adjacency.rs`: 隣接ペナルティ計算
   - `optimizer.rs`: スワッピング最適化

2. **並列処理**:
   - Rayonによる並列タイル読み込み
   - 最適化フェーズは順次処理（状態の一貫性保持）

3. **メモリ最適化**:
   - Arc<Tile>による共有参照
   - 上三角行列による類似度データの効率的格納

### アルゴリズムの特徴
- **タイル選択**: k-d treeによる高速近傍探索 + 隣接ペナルティ
- **最適化**: シミュレーテッドアニーリングによる大域的最適化
- **制約処理**: 使用回数制限と隣接制約の同時考慮

## トラブルシューティング

### よくある問題
1. **メモリ不足**: `--max-materials`を減らす
2. **処理時間が長い**: グリッドサイズを小さくする、最適化を無効化
3. **類似度データベースエラー**: `--rebuild-similarity-db`で再構築

### デバッグ方法
```bash
# 隣接ペナルティなしで比較
cargo run -- ... --adjacency-penalty-weight 0.0

# 最適化前後の比較
cargo run -- ... --enable-optimization false
cargo run -- ... --enable-optimization true
```

## ビルド方法

```bash
# デバッグビルド
cargo build

# リリースビルド（推奨）
cargo build --release

# テスト実行
cargo test
```

## 今後の改善案

1. **GPU加速**: CUDAやWebGPUによる並列化
2. **より高度な類似度メトリクス**: SSIM、perceptual hash
3. **インタラクティブモード**: リアルタイムプレビュー
4. **プログレッシブ生成**: 段階的解像度での生成