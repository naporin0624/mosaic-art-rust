# GUI Usage Examples

This document provides practical examples and common usage patterns for the Mosaic Art Generator GUI application.

## Table of Contents

- [Basic Usage Workflow](#basic-usage-workflow)
- [Common Scenarios](#common-scenarios)
- [Advanced Configurations](#advanced-configurations)
- [Tips and Best Practices](#tips-and-best-practices)
- [Troubleshooting Examples](#troubleshooting-examples)

## Basic Usage Workflow

### Step-by-Step Guide

#### 1. Launch the GUI Application

```bash
# From release build
./target/release/mosaic-gui

# From source (development)
cargo run --bin mosaic-gui --release
```

The application will open with the default light theme and empty file paths.

#### 2. Select Your Target Image

1. In the **File Selection** section, find the **Target Image** field
2. Either:
   - Type the path directly: `/path/to/your/photo.jpg`
   - Click **"Browse"** to open the file dialog
3. Select an image file (PNG, JPG, JPEG supported)

**Example paths:**
- Windows: `C:\Users\YourName\Pictures\vacation.jpg`
- macOS: `/Users/YourName/Pictures/vacation.jpg`
- Linux: `/home/username/Pictures/vacation.jpg`

#### 3. Select Material Directory

1. In the **Material Directory** field
2. Click **"Browse"** to select the folder containing your material images
3. Choose a directory with multiple images (preferably 100+ for best results)

**Example directories:**
- A folder of personal photos
- Downloaded image collection
- VRChat screenshots collection
- Stock photo thumbnails

#### 4. Configure Grid Settings

**Option A: Auto-Calculate (Recommended)**
1. Ensure **"Auto-calculate grid from total tiles"** is checked ✅
2. Enter desired **Total tiles** (e.g., `1400`)
3. Click **"Calculate Grid"**
4. The Width and Height fields will auto-populate

**Option B: Manual Configuration**
1. Uncheck **"Auto-calculate grid from total tiles"** ❌
2. Manually enter **Grid Width** (e.g., `80`)
3. Manually enter **Grid Height** (e.g., `60`)

#### 5. Set Output Path

1. In the **Output Path** field
2. Click **"Browse"** to choose where to save your mosaic
3. Enter a filename with supported extension (`.png`, `.jpg`, `.jpeg`)

#### 6. Generate Your Mosaic

1. Click **"Generate Mosaic"**
2. Monitor the console output for progress (currently displayed in terminal)
3. Wait for completion message

## Common Scenarios

### Scenario 1: Quick Preview Mosaic

**Goal**: Generate a fast preview to test composition

**Configuration**:
- **Target Image**: Your photo
- **Material Directory**: Any collection of 100+ images
- **Grid Settings**: 
  - Auto-calculate: ✅ Enabled
  - Total tiles: `400` (creates ~20×20 grid)
- **Advanced Settings**:
  - Max materials: `200`
  - Color adjustment: `0.0` (disabled for speed)
  - Enable optimization: ❌ Disabled

**Expected Result**: Fast generation (~1-2 minutes) with basic quality

### Scenario 2: High-Quality Art Piece

**Goal**: Create a detailed, high-quality mosaic for printing

**Configuration**:
- **Target Image**: High-resolution source image
- **Material Directory**: Large collection (1000+ images)
- **Grid Settings**:
  - Auto-calculate: ✅ Enabled
  - Total tiles: `4000` (creates ~80×50 grid)
- **Advanced Settings**:
  - Max materials: `1500`
  - Color adjustment: `0.5`
  - Enable optimization: ✅ Enabled

**Expected Result**: High-quality output (15-30 minutes processing)

### Scenario 3: Portrait Orientation

**Goal**: Create a portrait-oriented mosaic (9:16 aspect ratio)

**Configuration**:
- **Target Image**: Portrait photo
- **Grid Settings**:
  - Auto-calculate: ❌ Disabled
  - Grid Width: `45`
  - Grid Height: `80`
- **Advanced Settings**:
  - Max materials: `800`
  - Color adjustment: `0.4`

**Expected Result**: Vertical mosaic with 3,600 tiles

### Scenario 4: Unique Tiles Only

**Goal**: Ensure no material image is repeated

**Configuration**:
- **Material Directory**: Collection with more images than grid cells
- **Grid Settings**: Standard configuration
- **Advanced Settings**:
  - Max materials: Match or exceed total tiles
  - **Note**: This requires manual CLI usage as GUI doesn't expose `max-usage-per-image`

**CLI Equivalent**:
```bash
./target/release/mosaic-rust \
  --target photo.jpg \
  --material-src ./materials \
  --output unique_mosaic.jpg \
  --grid-w 50 \
  --grid-h 40 \
  --max-usage-per-image 1 \
  --max-materials 2000
```

### Scenario 5: Theme Customization

**Goal**: Work in comfortable lighting conditions

**Steps**:
1. Click **"Toggle Theme"** to switch between light and dark modes
2. Dark theme: Better for low-light environments
3. Light theme: Better for bright workspaces

**Theme Features**:
- Persists during session
- Instant switching
- Affects all UI elements

## Advanced Configurations

### Memory-Constrained Systems

**Problem**: Limited RAM (< 8GB)

**Solution**:
- **Max materials**: `200-500`
- **Grid dimensions**: Keep total tiles under 2000
- **Color adjustment**: `0.0-0.2` (reduces memory usage)

**Example Configuration**:
- Grid: 40×30 (1,200 tiles)
- Max materials: 300
- Color adjustment: 0.1

### Ultra-High Quality Output

**Problem**: Need maximum possible quality

**Solution**:
- **Large grid**: 100×100+ tiles
- **Many materials**: 2000+ unique images
- **Strong color adjustment**: 0.6-0.8
- **Enable optimization**: Always

**Example Configuration**:
- Grid: 120×90 (10,800 tiles)
- Max materials: 2500
- Color adjustment: 0.7

### Batch Processing Simulation

**Problem**: Need to process multiple images with same settings

**Current Workaround**:
1. Configure settings for first image
2. Generate mosaic
3. Only change target image path for subsequent images
4. Click "Generate Mosaic" again

**Future Enhancement**: True batch processing interface planned

### Material Library Management

**Best Practices**:

1. **Organize by theme**:
   ```
   materials/
   ├── nature/
   ├── urban/
   ├── people/
   └── abstract/
   ```

2. **Use consistent sizing**: Pre-resize materials to similar dimensions

3. **Quality over quantity**: 500 high-quality images > 2000 low-quality

4. **Aspect ratio matching**: Include materials with similar aspect ratios to your target

## Tips and Best Practices

### File Management

**Recommended file naming**:
```
input_image.jpg          → Final name: input_image_mosaic_1400.png
portrait_photo.png       → Final name: portrait_photo_mosaic_3600.jpg
landscape_sunset.jpeg    → Final name: landscape_sunset_mosaic_2000.png
```

**Directory structure**:
```
mosaic_project/
├── originals/           # Source images
├── materials/           # Material image collections
├── outputs/             # Generated mosaics
└── configs/             # Saved settings (future feature)
```

### Performance Optimization

**For faster generation**:
1. Use smaller grids for testing (20×20 to 50×30)
2. Limit materials to 200-500 for previews
3. Disable color adjustment for speed tests
4. Keep optimization disabled during experimentation

**For better quality**:
1. Use larger grids (80×60 or higher)
2. Include 1000+ material images
3. Enable color adjustment (0.3-0.6)
4. Always enable optimization for final output

### Grid Calculation Guidelines

**Common aspect ratios and grid suggestions**:

| Target Aspect Ratio | Total Tiles | Suggested Grid | Actual Tiles |
|-------------------|------------|----------------|--------------|
| 16:9 (Landscape)  | 1400       | 50×28          | 1400         |
| 16:9 (Landscape)  | 2500       | 67×38          | 2546         |
| 4:3 (Standard)    | 1200       | 40×30          | 1200         |
| 1:1 (Square)      | 1600       | 40×40          | 1600         |
| 9:16 (Portrait)   | 1400       | 28×50          | 1400         |

### Color Adjustment Guidelines

| Strength | Use Case | Quality | Speed |
|----------|----------|---------|-------|
| 0.0      | Preview/Speed test | Basic | Fastest |
| 0.1-0.2  | Quick results | Good | Fast |
| 0.3-0.4  | Standard quality | Very Good | Medium |
| 0.5-0.6  | High quality | Excellent | Slow |
| 0.7-1.0  | Maximum quality | Outstanding | Slowest |

## Troubleshooting Examples

### Problem: No materials found

**Symptoms**: Error message about empty material directory

**Solution**:
1. Verify the material directory path is correct
2. Ensure the directory contains image files (PNG, JPG, JPEG)
3. Check file permissions (read access required)
4. Try a different directory with known good images

**Example fix**:
```
# Instead of empty directory
/path/to/empty_folder/

# Use directory with images
/path/to/photos_collection/
├── img001.jpg
├── img002.png
└── img003.jpeg
```

### Problem: Grid calculation produces unexpected results

**Symptoms**: Auto-calculated grid doesn't match expectations

**Debugging**:
1. Check the total tiles input (numbers only)
2. Remember the algorithm assumes 16:9 aspect ratio
3. For different aspect ratios, use manual grid entry

**Example**:
- Input: 1000 tiles
- Auto-calculated: 42×24 (1008 tiles) ✅
- Manual override: 32×32 (1024 tiles) if square needed

### Problem: Very slow generation

**Symptoms**: Processing takes over 30 minutes

**Immediate fixes**:
1. Reduce grid size: Try 30×20 instead of 100×75
2. Limit materials: Set max materials to 500
3. Disable optimization temporarily
4. Reduce color adjustment to 0.1

**Long-term optimization**:
1. Use SSD storage for material images
2. Pre-resize material images to similar dimensions
3. Ensure adequate RAM (8GB+ recommended for large mosaics)

### Problem: Poor color matching

**Symptoms**: Generated mosaic doesn't resemble the original

**Solutions**:
1. Increase color adjustment strength (0.5-0.7)
2. Use more diverse material images
3. Enable optimization for better tile placement
4. Check that material images have good color variety

**Color diversity check**:
- Materials should include: bright, dark, colored, and neutral images
- Avoid collections with similar color palettes
- Include images with various brightness levels

### Problem: File dialog doesn't open

**Platform-specific solutions**:

**Linux**:
```bash
# Install required dependencies
sudo apt-get install libgtk-3-dev

# For newer systems
sudo apt-get install libgtk-4-dev
```

**Windows**: Usually works out of the box, try running as administrator if issues persist

**macOS**: Ensure the app has file system permissions in System Preferences

### Problem: Theme doesn't change

**Solution**:
1. Click "Toggle Theme" button
2. Wait a moment for re-render
3. If stuck, restart the application
4. Check that your system supports the requested theme

---

These examples cover the most common usage patterns and problems. For additional help, refer to the [architecture documentation](architecture.md) or the main [README](../../README.md).