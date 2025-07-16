# Application Title
app-title = Mosaic Art Generator
app-subtitle = Create beautiful mosaic art from your images

# Language Selection
language-label = Language
language-english = English
language-japanese = 日本語

# File Selection Section
file-selection-title = File Selection
file-selection-description = Select your input image, material images folder, and output location

target-image-label = Target Image
target-image-description = The main image that will be converted into a mosaic art
target-image-placeholder = Select target image file
target-image-browse = Browse
target-image-tooltip = Choose a high-resolution image for best results. Supported formats: PNG, JPG, JPEG

material-directory-label = Material Directory
material-directory-description = Folder containing images to use as mosaic tiles
material-directory-placeholder = Select material images folder
material-directory-browse = Browse
material-directory-tooltip = Select a folder with 100-1000+ diverse images for best variety

output-path-label = Output Path
output-path-description = Where the final mosaic will be saved
output-path-placeholder = Choose output file location
output-path-browse = Browse
output-path-tooltip = Choose location to save your mosaic. Use PNG for lossless quality

# Grid Settings Section
grid-settings-title = Grid Settings
grid-settings-description = Configure how your image will be divided into tiles

auto-calculate-label = Auto-calculate grid from total tiles
auto-calculate-description = Automatically determine optimal grid dimensions based on total tiles
auto-calculate-tooltip = Enable this to automatically calculate grid width and height from total tiles

total-tiles-label = Total tiles
total-tiles-description = Number of tiles to use in the mosaic
total-tiles-placeholder = e.g., 1400
total-tiles-tooltip = More tiles = higher detail but longer processing time. Recommended: 1000-2000

calculate-grid-button = Calculate Grid
calculate-grid-tooltip = Calculate optimal grid dimensions for the specified number of tiles

grid-width-label = Grid Width
grid-width-description = Number of tiles horizontally
grid-width-placeholder = 50
grid-width-tooltip = More columns = finer horizontal detail

grid-height-label = Grid Height
grid-height-description = Number of tiles vertically
grid-height-placeholder = 28
grid-height-tooltip = More rows = finer vertical detail

# Advanced Settings Section
advanced-settings-title = Advanced Settings
advanced-settings-description = Expert-level configuration options for fine-tuning

# Configuration Subsection
configuration-title = Configuration
configuration-description = Settings that affect how tiles are selected and processed

max-materials-label = Max materials
max-materials-description = Limit the number of material images to load
max-materials-placeholder = 500
max-materials-tooltip = Higher values = more variety but longer loading time. Match to your collection size

color-adjustment-label = Color adjustment (0.0-1.0)
color-adjustment-description = Fine-tune color matching between target and tiles
color-adjustment-placeholder = 0.3
color-adjustment-tooltip = 0.0 = no adjustment, 0.3 = balanced (recommended), 1.0 = maximum adjustment

max-usage-per-image-label = Max usage per image
max-usage-per-image-description = Prevent overuse of individual tile images (set to 0 for auto-calculation)
max-usage-per-image-placeholder = 0 (auto)
max-usage-per-image-tooltip = 0 = auto-calculate (total tiles ÷ max materials), 1 = maximum variety, 3 = balanced, 10+ = allow frequent reuse

auto-calculate-max-usage-label = Auto-calculate max usage per image
auto-calculate-max-usage-description = Automatically calculate max usage per image based on total tiles ÷ max materials
auto-calculate-max-usage-tooltip = When enabled, max usage per image is automatically calculated from total tiles ÷ max materials

adjacency-penalty-weight-label = Adjacency penalty weight (0.0-1.0)
adjacency-penalty-weight-description = Prevent similar tiles from being placed next to each other
adjacency-penalty-weight-placeholder = 0.3
adjacency-penalty-weight-tooltip = 0.0 = no penalty, 0.3 = balanced (recommended), 1.0 = maximum penalty

similarity-db-path-label = Similarity database path
similarity-db-path-description = Path to the similarity database file
similarity-db-path-placeholder = similarity_db.json
similarity-db-path-tooltip = Database file for caching similarity calculations between tiles

rebuild-similarity-db-label = Rebuild similarity database
rebuild-similarity-db-description = Force rebuild the similarity database on next generation
rebuild-similarity-db-tooltip = Enable this to recalculate all tile similarities. Useful when material images have changed

# Optimization Subsection
optimization-title = Optimization
optimization-description = Settings for post-placement optimization using simulated annealing

enable-optimization-label = Enable optimization
enable-optimization-description = Use simulated annealing to improve tile placement
enable-optimization-tooltip = Improves quality at the cost of processing time. Recommended: enabled

optimization-iterations-label = Optimization iterations
optimization-iterations-description = Number of optimization steps to perform
optimization-iterations-placeholder = 1000
optimization-iterations-tooltip = More iterations = better quality but longer processing time

# Debugging Subsection
debugging-title = Debugging
debugging-description = Options for troubleshooting and detailed analysis

verbose-logging-label = Verbose logging (debug output)
verbose-logging-description = Enable detailed debug output for troubleshooting
verbose-logging-tooltip = Shows detailed processing information. Useful for troubleshooting issues

# Action Buttons
generate-button = Generate Mosaic
generate-button-processing = Processing...
generate-button-tooltip = Start creating your mosaic art

toggle-theme-button = Toggle Theme
toggle-theme-tooltip = Switch between light and dark themes

# Progress and Status
progress-initializing = Initializing...
progress-loading-target = Loading target image...
progress-loading-materials = Loading material images...
progress-analyzing-materials = Analyzing material images...
progress-building-database = Building similarity database...
progress-processing-grid = Processing grid cells...
progress-optimization = Optimizing tile placement...
progress-saving = Saving output image...
progress-completed = Completed

# Status Messages
status-ready = Ready to generate mosaic
status-processing = Processing...
status-completed = ✅ Completed
status-error = ❌ Error: { $error }

# Generation Log
generation-log-title = Generation Log
generation-log-description = Detailed log of all operations during mosaic generation

# Success Messages
success-completed = ✅ Mosaic generation completed
success-completed-with-time = ✅ Mosaic generation completed in { $time }s
success-saved-to = 💾 Saved to: { $path }
success-optimization-improved = ✅ Optimization improved cost by { $percentage }%

# Error Messages
error-no-target = ❌ Error: No target image selected
error-no-material = ❌ Error: No material directory selected
error-no-output = ❌ Error: No output path specified
error-target-not-found = Target image file does not exist
error-material-not-found = Material directory does not exist or is not a directory
error-no-materials-found = No material images found in the specified directory
error-failed-to-load-target = Failed to load target image: { $error }
error-failed-to-save = Failed to save output image: { $error }
error-processing = Processing error: { $error }

# Info Messages
info-starting = 🚀 Starting mosaic generation...
info-target-loaded = 📸 Loaded target image: { $width }x{ $height }
info-materials-found = 🎨 Found { $count } material images
info-materials-loaded = ✅ Loaded { $count } tiles
info-grid-config = 🔧 Grid: { $width }x{ $height } ({ $total } tiles)
info-tile-size = 🔧 Tile size: { $width }x{ $height } pixels per tile
info-optimization-enabled = 🔧 Optimization: enabled
info-optimization-disabled = 🔧 Optimization: disabled

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
fallback-primary-failed = ⚠️ Primary selection failed for position ({ $x }, { $y }), trying fallback...
fallback-selection-success = ✅ Fallback selection success for position ({ $x }, { $y })
fallback-final-attempt = ⚠️ Using final fallback - best color match without adjacency constraints...
fallback-final-success = ✅ Final fallback success for position ({ $x }, { $y })
fallback-critical-failure = ❌ CRITICAL: All fallback methods failed for position ({ $x }, { $y })

# Validation Messages
validation-grid-dimensions = Grid dimensions must be positive numbers
validation-total-tiles = Total tiles must be a positive number
validation-color-adjustment = Color adjustment must be between 0.0 and 1.0
validation-max-materials = Max materials must be a positive number
validation-max-usage = Max usage per image must be at least 1
validation-adjacency-weight = Adjacency penalty weight must be between 0.0 and 1.0
validation-optimization-iterations = Optimization iterations must be at least 1

# Tooltips and Help
help-grid-calculation = Grid calculation uses 16:9 aspect ratio assumption
help-file-formats = Supported formats: PNG, JPG, JPEG
help-performance-tip = For better performance, use release build: cargo build --release
help-memory-usage = Memory usage increases with tile count and image size
help-processing-time = Processing time depends on grid size and optimization settings