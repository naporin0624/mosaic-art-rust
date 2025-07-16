# Application Title
app-title = モザイクアート生成ツール
app-subtitle = 画像から美しいモザイクアートを作成

# Language Selection
language-label = Language
language-english = English
language-japanese = Japanese

# File Selection Section
file-selection-title = ファイル選択
file-selection-description = 入力画像、素材画像フォルダ、出力場所を選択してください

target-image-label = ターゲット画像
target-image-description = モザイクアートに変換するメイン画像
target-image-placeholder = ターゲット画像ファイルを選択
target-image-browse = 参照
target-image-tooltip = 最高の結果を得るには高解像度の画像を選択してください。対応形式：PNG、JPG、JPEG

material-directory-label = 素材ディレクトリ
material-directory-description = モザイクタイルとして使用する画像を含むフォルダ
material-directory-placeholder = 素材画像フォルダを選択
material-directory-browse = 参照
material-directory-tooltip = 最適な多様性を得るには100〜1000+の多様な画像を含むフォルダを選択してください

output-path-label = 出力パス
output-path-description = 最終的なモザイクが保存される場所
output-path-placeholder = 出力ファイルの場所を選択
output-path-browse = 参照
output-path-tooltip = モザイクを保存する場所を選択してください。ロスレス品質にはPNGを使用

# Grid Settings Section
grid-settings-title = グリッド設定
grid-settings-description = 画像をタイルに分割する方法を設定

auto-calculate-label = Auto-calculate grid from total tiles
auto-calculate-description = タイル総数に基づいて最適なグリッド寸法を自動決定
auto-calculate-tooltip = これを有効にすると、タイル総数からグリッドの幅と高さを自動計算します

total-tiles-label = タイル総数
total-tiles-description = モザイクで使用するタイルの数
total-tiles-placeholder = 例：1400
total-tiles-tooltip = タイル数が多い = 詳細度が高いが処理時間が長い。推奨：1000-2000

calculate-grid-button = グリッド計算
calculate-grid-tooltip = 指定されたタイル数に対する最適なグリッド寸法を計算

grid-width-label = グリッド幅
grid-width-description = 水平方向のタイル数
grid-width-placeholder = 50
grid-width-tooltip = 列数が多い = 水平方向の詳細度が高い

grid-height-label = グリッド高さ
grid-height-description = 垂直方向のタイル数
grid-height-placeholder = 28
grid-height-tooltip = 行数が多い = 垂直方向の詳細度が高い

# Advanced Settings Section
advanced-settings-title = 高度な設定
advanced-settings-description = 細かい調整のためのエキスパートレベル設定オプション

# Configuration Subsection
configuration-title = 設定
configuration-description = タイルの選択と処理に影響する設定

max-materials-label = 最大素材数
max-materials-description = 読み込む素材画像の数を制限
max-materials-placeholder = 500
max-materials-tooltip = 大きい値 = より多様だが読み込み時間が長い。コレクションサイズに合わせて調整

color-adjustment-label = 色調整 (0.0-1.0)
color-adjustment-description = ターゲットとタイル間の色マッチングを微調整
color-adjustment-placeholder = 0.3
color-adjustment-tooltip = 0.0 = 調整なし、0.3 = バランス（推奨）、1.0 = 最大調整

max-usage-per-image-label = 画像あたりの最大使用回数
max-usage-per-image-description = 個々のタイル画像の過度の使用を防ぐ（0で自動計算）
max-usage-per-image-placeholder = 0 (自動)
max-usage-per-image-tooltip = 0 = 自動計算（総タイル数÷最大素材数）、1 = 最大多様性、3 = バランス、10+ = 頻繁な再利用を許可

auto-calculate-max-usage-label = 画像使用回数を自動計算
auto-calculate-max-usage-description = 総タイル数÷最大素材数に基づいて画像あたりの最大使用回数を自動計算
auto-calculate-max-usage-tooltip = 有効にすると、総タイル数÷最大素材数から画像あたりの最大使用回数を自動計算します

adjacency-penalty-weight-label = 隣接ペナルティ重み (0.0-1.0)
adjacency-penalty-weight-description = 類似タイルが隣り合って配置されるのを防ぐ
adjacency-penalty-weight-placeholder = 0.3
adjacency-penalty-weight-tooltip = 0.0 = ペナルティなし、0.3 = バランス（推奨）、1.0 = 最大ペナルティ

similarity-db-path-label = 類似度データベースパス
similarity-db-path-description = 類似度データベースファイルのパス
similarity-db-path-placeholder = similarity_db.json
similarity-db-path-tooltip = タイル間の類似度計算をキャッシュするデータベースファイル

rebuild-similarity-db-label = Rebuild similarity database
rebuild-similarity-db-description = 次回生成時に類似度データベースを強制的に再構築
rebuild-similarity-db-tooltip = これを有効にすると、すべてのタイルの類似度を再計算します。素材画像が変更された場合に便利です

# Optimization Subsection
optimization-title = 最適化
optimization-description = シミュレーテッドアニーリングを使用した配置後最適化の設定

enable-optimization-label = Enable optimization
enable-optimization-description = シミュレーテッドアニーリングを使用してタイル配置を改善
enable-optimization-tooltip = 処理時間を犠牲にして品質を向上させます。推奨：有効

optimization-iterations-label = 最適化反復回数
optimization-iterations-description = 実行する最適化ステップの数
optimization-iterations-placeholder = 1000
optimization-iterations-tooltip = 反復回数が多い = 品質が向上するが処理時間が長い

# Debugging Subsection
debugging-title = デバッグ
debugging-description = トラブルシューティングと詳細分析のオプション

verbose-logging-label = Verbose logging (debug output)
verbose-logging-description = トラブルシューティング用の詳細なデバッグ出力を有効にする
verbose-logging-tooltip = 詳細な処理情報を表示します。問題のトラブルシューティングに便利

# Action Buttons
generate-button = モザイク生成
generate-button-processing = 処理中...
generate-button-tooltip = モザイクアートの作成を開始

toggle-theme-button = テーマ切り替え
toggle-theme-tooltip = ライトテーマとダークテーマを切り替え

# Progress and Status
progress-initializing = 初期化中...
progress-loading-target = ターゲット画像を読み込み中...
progress-loading-materials = 素材画像を読み込み中...
progress-analyzing-materials = 素材画像を分析中...
progress-building-database = 類似度データベースを構築中...
progress-processing-grid = グリッドセルを処理中...
progress-optimization = タイル配置を最適化中...
progress-saving = 出力画像を保存中...
progress-completed = 完了

# Status Messages
status-ready = モザイク生成の準備完了
status-processing = 処理中...
status-completed = ✅ 完了
status-error = ❌ エラー: { $error }

# Generation Log
generation-log-title = 生成ログ
generation-log-description = モザイク生成中の全操作の詳細ログ

# Success Messages
success-completed = ✅ モザイク生成が完了しました
success-completed-with-time = ✅ モザイク生成が{ $time }秒で完了しました
success-saved-to = 💾 保存先: { $path }
success-optimization-improved = ✅ 最適化によりコストが{ $percentage }%改善されました

# Error Messages
error-no-target = ❌ エラー: ターゲット画像が選択されていません
error-no-material = ❌ エラー: 素材ディレクトリが選択されていません
error-no-output = ❌ エラー: 出力パスが指定されていません
error-target-not-found = ターゲット画像ファイルが存在しません
error-material-not-found = 素材ディレクトリが存在しないか、ディレクトリではありません
error-no-materials-found = 指定されたディレクトリに素材画像が見つかりません
error-failed-to-load-target = ターゲット画像の読み込みに失敗しました: { $error }
error-failed-to-save = 出力画像の保存に失敗しました: { $error }
error-processing = 処理エラー: { $error }

# Info Messages
info-starting = 🚀 モザイク生成を開始しています...
info-target-loaded = 📸 ターゲット画像を読み込みました: { $width }x{ $height }
info-materials-found = 🎨 { $count }個の素材画像を発見しました
info-materials-loaded = ✅ { $count }個のタイルを読み込みました
info-grid-config = 🔧 グリッド: { $width }x{ $height } ({ $total }タイル)
info-tile-size = 🔧 タイルサイズ: タイルあたり{ $width }x{ $height }ピクセル
info-optimization-enabled = 🔧 最適化: 有効
info-optimization-disabled = 🔧 最適化: 無効

# Log Prefixes
log-status = 🚀
log-file = 📁
log-config = 🔧
log-processing = ⚙️
log-success = ✅
log-error = ❌
log-debug = 🔍
log-warning = ⚠️

# Robustness Features
fallback-primary-failed = ⚠️ 位置({ $x }, { $y })のプライマリ選択が失敗しました、フォールバックを試行中...
fallback-selection-success = ✅ 位置({ $x }, { $y })のフォールバック選択が成功しました
fallback-final-attempt = ⚠️ 最終フォールバックを使用 - 隣接制約なしの最適色マッチング...
fallback-final-success = ✅ 位置({ $x }, { $y })の最終フォールバックが成功しました
fallback-critical-failure = ❌ 重要: 位置({ $x }, { $y })の全フォールバック方法が失敗しました

# Validation Messages
validation-grid-dimensions = グリッド寸法は正の数でなければなりません
validation-total-tiles = タイル総数は正の数でなければなりません
validation-color-adjustment = 色調整は0.0から1.0の間でなければなりません
validation-max-materials = 最大素材数は正の数でなければなりません
validation-max-usage = 画像あたりの最大使用回数は最低1でなければなりません
validation-adjacency-weight = 隣接ペナルティ重みは0.0から1.0の間でなければなりません
validation-optimization-iterations = 最適化反復回数は最低1でなければなりません

# Tooltips and Help
help-grid-calculation = グリッド計算は16:9アスペクト比の仮定を使用します
help-file-formats = 対応形式：PNG、JPG、JPEG
help-performance-tip = より良いパフォーマンスのために、リリースビルドを使用してください：cargo build --release
help-memory-usage = メモリ使用量はタイル数と画像サイズに応じて増加します
help-processing-time = 処理時間はグリッドサイズと最適化設定に依存します