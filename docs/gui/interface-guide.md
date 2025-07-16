# Interface Guide

This comprehensive guide covers every aspect of the GUI interface, explaining each section, control, and feature in detail.

## Application Layout

The GUI is organized into logical sections that flow from top to bottom:

1. **Application Title** - Mosaic Art Generator
2. **File Selection Panel** - Choose input and output files
3. **Grid Settings Panel** - Configure mosaic dimensions
4. **Advanced Settings Panel** - Expert configuration options
5. **Status and Progress** - Real-time generation feedback
6. **Action Buttons** - Generate mosaic and toggle theme
7. **Generation Log** - Detailed operation logs

## File Selection Panel

### Target Image Selection
The target image is the main image that will be converted into a mosaic.

- **Input Field**: Shows the currently selected file path
- **Browse Button**: Opens native file dialog
- **Supported Formats**: PNG, JPG, JPEG
- **Recommendations**: 
  - Use high-resolution images for best results
  - Ensure good contrast and clear details
  - Square or rectangular images work well

### Material Directory Selection
The material directory contains all images that will be used as mosaic tiles.

- **Input Field**: Shows the currently selected directory path
- **Browse Button**: Opens native folder dialog
- **Requirements**: Must be a valid directory containing image files
- **Recommendations**:
  - Use 100-1000+ images for best variety
  - Ensure images have diverse colors and textures
  - Consistent image quality across all tiles

### Output Path Selection
Specifies where the final mosaic will be saved.

- **Input Field**: Shows the complete output file path
- **Browse Button**: Opens native save dialog
- **Supported Formats**: PNG, JPG, JPEG
- **Recommendations**:
  - Use PNG for lossless quality
  - Ensure sufficient disk space
  - Choose descriptive filenames

## Grid Settings Panel

### Auto-Calculate Mode
When enabled, the application automatically calculates optimal grid dimensions based on the total number of tiles.

#### Auto-Calculate Checkbox
- **Checked**: Enables automatic grid calculation
- **Unchecked**: Allows manual grid dimension entry
- **Default**: Enabled for user convenience

#### Total Tiles Input
- **Purpose**: Specifies how many tiles to use in the mosaic
- **Range**: 100-10000+ (practical limits depend on system resources)
- **Recommendations**:
  - **500-1000**: Good for testing and preview
  - **1000-2000**: Balanced detail and performance
  - **2000-5000**: High detail, longer processing time
  - **5000+**: Maximum detail, requires powerful hardware

#### Calculate Grid Button
- **Function**: Computes grid dimensions based on total tiles
- **Algorithm**: Uses 16:9 aspect ratio assumption
- **Formula**: Width = √(total_tiles × 16/9), Height = total_tiles ÷ width

### Manual Grid Mode
When auto-calculate is disabled, you can set exact grid dimensions.

#### Grid Width
- **Purpose**: Number of tiles horizontally
- **Range**: 1-1000+ (practical limits apply)
- **Impact**: More columns = finer horizontal detail

#### Grid Height
- **Purpose**: Number of tiles vertically
- **Range**: 1-1000+ (practical limits apply)
- **Impact**: More rows = finer vertical detail

## Advanced Settings Panel

The advanced settings panel is **collapsible** and contains expert-level configuration options organized into logical sections.

### Panel Header
- **Expand/Collapse Button**: Click the ► (collapsed) or ▼ (expanded) arrow to toggle visibility
- **Default State**: Collapsed to reduce interface complexity
- **Purpose**: Hides advanced options that most users don't need to modify

### Configuration Section
Settings that affect how tiles are selected and processed.

#### Max Materials
- **Purpose**: Limits the number of material images to load
- **Default**: 500
- **Range**: 10-10000+
- **Impact**: 
  - **Higher values**: More tile variety, longer loading time
  - **Lower values**: Faster loading, less variety
- **Recommendation**: Match to your material collection size

#### Color Adjustment (0.0-1.0)
- **Purpose**: Fine-tunes color matching between target and tiles
- **Default**: 0.3
- **Range**: 0.0 (no adjustment) to 1.0 (maximum adjustment)
- **Impact**:
  - **0.0**: Pure color matching, may look artificial
  - **0.3**: Balanced adjustment (recommended)
  - **1.0**: Heavy adjustment, may lose color accuracy

#### Max Usage Per Image
- **Purpose**: Prevents overuse of individual tile images
- **Default**: 3
- **Range**: 1-100+
- **Impact**:
  - **1**: Each tile used only once (maximum variety)
  - **3**: Balanced repetition (recommended)
  - **10+**: Allows frequent reuse of good matches

#### Adjacency Penalty Weight (0.0-1.0)
- **Purpose**: Prevents similar tiles from being placed next to each other
- **Default**: 0.3
- **Range**: 0.0 (no penalty) to 1.0 (maximum penalty)
- **Impact**:
  - **0.0**: No adjacency checking (may create clusters)
  - **0.3**: Balanced penalty (recommended)
  - **1.0**: Strong penalty (may sacrifice color accuracy)

### Optimization Section
Settings for post-placement optimization using simulated annealing.

#### Enable Optimization Checkbox
- **Purpose**: Toggles post-placement optimization
- **Default**: Enabled
- **Impact**: Improves tile placement quality at the cost of processing time
- **Recommendation**: Keep enabled unless time is critical

#### Optimization Iterations
- **Purpose**: Number of optimization steps to perform
- **Default**: 1000
- **Range**: 1-10000+
- **Impact**:
  - **100-500**: Fast optimization, modest improvement
  - **1000**: Balanced optimization (recommended)
  - **2000+**: Thorough optimization, longer processing time

### Debugging Section
Options for troubleshooting and detailed analysis.

#### Verbose Logging Checkbox
- **Purpose**: Enables detailed debug output
- **Default**: Disabled
- **Impact**: Provides comprehensive processing information
- **Use Cases**:
  - Troubleshooting generation issues
  - Understanding algorithm behavior
  - Performance analysis
  - Bug reporting

## Status and Progress Section

### Processing States
The interface shows different states during mosaic generation:

#### Idle State
- **Appearance**: No progress indicators visible
- **Status**: Ready to generate mosaic
- **Actions**: All controls enabled

#### Processing State
- **Progress Bar**: Shows completion percentage (0-100%)
- **Step Description**: Current processing step
- **Status Text**: Detailed operation information
- **Actions**: Generate button disabled during processing

#### Completed State
- **Appearance**: "✅ Completed" message
- **Status**: Mosaic generation successful
- **Actions**: All controls re-enabled

#### Error State
- **Appearance**: "❌ Error: [description]" message
- **Status**: Generation failed
- **Actions**: All controls enabled, check logs for details

### Progress Indicators
- **Progress Bar**: Visual representation of completion
- **Percentage**: Numeric completion percentage
- **Step Messages**: Current processing operation
- **Estimated Time**: Time remaining (when available)

## Action Buttons

### Generate Mosaic Button
- **Primary Function**: Starts mosaic generation process
- **States**:
  - **"Generate Mosaic"**: Ready to start
  - **"Processing..."**: Generation in progress (disabled)
- **Requirements**: All file paths must be valid
- **Validation**: Automatic input validation before starting

### Toggle Theme Button
- **Function**: Switches between light and dark themes
- **States**: Light Theme ↔ Dark Theme
- **Persistence**: Theme preference remembered during session
- **Accessibility**: Improves usability in different lighting conditions

## Generation Log

### Log Display
- **Location**: Bottom of the interface
- **Visibility**: Appears automatically when generation starts
- **Capacity**: Shows last 50 log messages
- **Scrolling**: Automatic scrolling to newest messages

### Log Types
- **🚀 Status**: General progress updates
- **📁 File**: File operations and paths
- **🔧 Configuration**: Settings and parameters
- **⚙️ Processing**: Detailed operation steps
- **✅ Success**: Completion messages
- **❌ Error**: Error messages and warnings
- **🔍 Debug**: Detailed debug information (when verbose logging enabled)

### Log Navigation
- **Auto-scroll**: Newest messages appear at the bottom
- **Manual scroll**: Use scrollbar to review previous messages
- **Text selection**: Click and drag to select log text
- **Copy support**: Selected text can be copied to clipboard

## Keyboard Shortcuts

### Navigation
- **Tab**: Move between input fields
- **Shift+Tab**: Move backwards between fields
- **Enter**: Activate focused button
- **Space**: Toggle focused checkbox

### File Operations
- **Ctrl+O**: Open target file (Windows/Linux)
- **Cmd+O**: Open target file (macOS)
- **Ctrl+S**: Save output file (Windows/Linux)
- **Cmd+S**: Save output file (macOS)

### Application
- **Ctrl+T**: Toggle theme (Windows/Linux)
- **Cmd+T**: Toggle theme (macOS)
- **F1**: Show help (planned)
- **Ctrl+Q**: Quit application (Windows/Linux)
- **Cmd+Q**: Quit application (macOS)

## Accessibility Features

### Visual Accessibility
- **High Contrast**: Dark theme for better visibility
- **Clear Typography**: Readable font sizes and spacing
- **Color Coding**: Consistent color scheme for different message types
- **Visual Feedback**: Clear indication of interactive elements

### Keyboard Accessibility
- **Tab Navigation**: All controls accessible via keyboard
- **Focus Indicators**: Clear visual focus indicators
- **Keyboard Shortcuts**: Common operations accessible via keyboard
- **Screen Reader Support**: Proper labeling for assistive technologies

## Performance Monitoring

### Real-time Feedback
- **Progress Updates**: Regular progress percentage updates
- **Step Information**: Current processing operation
- **Time Tracking**: Elapsed time display
- **Memory Usage**: Visual cues for resource usage

### System Resources
- **CPU Usage**: Processing intensity varies by operation
- **Memory Usage**: Increases with tile count and image size
- **Disk Usage**: Output file size depends on mosaic dimensions
- **Network**: No network activity required

## Robustness Features

### Three-Stage Fallback System

The GUI application includes a comprehensive fallback mechanism (added 2025-01-11) that ensures every grid cell in your mosaic is filled, preventing any black or empty cells in the final output. This system works automatically and provides complete feature parity with the CLI version's robustness.

#### How It Works

1. **Primary Selection Stage**:
   - Uses the k-d tree search with all constraints enabled
   - Respects usage limits and adjacency penalties
   - Finds the best color-matching tile within all constraints

2. **Fallback Selection Stage**:
   - Activates when primary selection fails due to usage limits
   - Resets the usage tracker to allow tiles to be reused
   - Maintains adjacency constraints to prevent clustering
   - Ensures variety while filling difficult cells

3. **Final Fallback Stage**:
   - Used as a last resort when all constraints prevent placement
   - Ignores adjacency penalties completely
   - Always selects the best color-matching tile
   - Guarantees that every cell will be filled

#### Monitoring Fallback Activity

When **Verbose logging** is enabled in Advanced Settings, you can monitor fallback activity in the Generation Log:

- **Primary selection**: No special log entries (normal operation)
- **Fallback selection**: "⚠️ Using fallback tile selection with reset usage tracker..."
- **Final fallback**: "⚠️ Using final fallback - best color match without adjacency constraints..."

These log entries help you understand when and why the fallback system is being used, which can guide you in adjusting your settings for better results.

#### Impact on Results

The fallback system ensures:
- **No empty cells**: Every position in the mosaic will have a tile
- **Graceful degradation**: Quality is maintained as much as possible
- **Predictable output**: No surprises with black squares in your mosaic
- **Better material utilization**: Tiles can be reused when necessary

## Tips for Efficient Usage

### Interface Organization
1. **Start with file selection**: Always begin with proper file selection
2. **Use auto-calculate**: Simplifies grid configuration for beginners
3. **Expand advanced settings**: Only when default settings aren't sufficient
4. **Monitor logs**: Check for warnings or errors during processing

### Workflow Optimization
1. **Prepare materials**: Organize material images before starting
2. **Test with small settings**: Use fewer tiles for initial testing
3. **Save presets**: Remember successful settings for future use
4. **Batch similar images**: Process similar target images with same settings

The GUI interface provides a comprehensive yet approachable way to create mosaic art. Each section is designed to guide you through the process while providing access to advanced features when needed.