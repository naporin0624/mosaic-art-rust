# Examples and Tutorials

This section provides step-by-step examples for common use cases and creative applications of the GUI application.

## Basic Examples

### Example 1: Your First Mosaic

**Goal**: Create a simple mosaic using default settings

**Materials Needed**:

- 1 target image (photo or artwork)
- 100+ material images (photos, screenshots, artwork)

**Steps**:

1. **Launch the GUI**

   ```bash
   ./target/release/mosaic-gui
   ```

2. **Select Files**

   - Click "Browse" next to "Target Image"
   - Select your main image (e.g., `portrait.jpg`)
   - Click "Browse" next to "Material Directory"
   - Select folder containing your tile images
   - Click "Browse" next to "Output Path"
   - Choose save location (e.g., `my-first-mosaic.png`)

3. **Basic Settings**

   - Keep "Auto-calculate grid from total tiles" checked
   - Set "Total tiles" to 1000
   - Click "Calculate Grid"
   - Result: Approximately 42×24 grid

4. **Generate**
   - Click "Generate Mosaic"
   - Wait for completion (1-5 minutes)
   - View your result!

**Expected Results**:

- Processing time: 1-5 minutes
- Output size: Similar to target image dimensions
- Quality: Good balance of detail and recognizability

### Example 2: High-Quality Portrait

**Goal**: Create a detailed portrait mosaic with optimal settings

**Materials Needed**:

- High-resolution portrait (1920×1080 or higher)
- 500+ diverse material images
- Powerful computer (8GB+ RAM)

**Steps**:

1. **File Selection**

   - Target: High-resolution portrait
   - Materials: Diverse collection with good color variety
   - Output: PNG format for best quality

2. **Grid Settings**

   - Enable auto-calculate
   - Set total tiles to 2500
   - Calculated grid: ~67×37

3. **Advanced Settings** (Click ► to expand)

   - Max materials: 800
   - Color adjustment: 0.4
   - Max usage per image: 2
   - Adjacency penalty weight: 0.4
   - Enable optimization: Yes
   - Optimization iterations: 1500

4. **Generate and Review**
   - Enable verbose logging for detailed progress
   - Processing time: 5-15 minutes
   - Final result: High-detail portrait with minimal repetition

**Expected Results**:

- Very detailed facial features
- Smooth color transitions
- Minimal tile repetition
- Professional-looking result

### Example 3: Landscape Mosaic

**Goal**: Create a landscape mosaic emphasizing natural patterns

**Materials Needed**:

- Landscape photograph (mountain, ocean, forest)
- Nature-themed material images
- Medium-powered computer

**Steps**:

1. **File Selection**

   - Target: Landscape with clear horizon and sky
   - Materials: Nature photos (trees, clouds, water, rocks)
   - Output: JPG format for smaller file size

2. **Grid Settings**

   - Enable auto-calculate
   - Set total tiles to 1800
   - Calculated grid: ~57×32

3. **Advanced Settings**

   - Max materials: 600
   - Color adjustment: 0.5 (higher for natural blending)
   - Max usage per image: 3
   - Adjacency penalty weight: 0.2 (lower for natural clustering)
   - Enable optimization: Yes
   - Optimization iterations: 1200

4. **Review Results**
   - Sky should blend smoothly
   - Natural textures in terrain
   - Organic-looking patterns

**Expected Results**:

- Natural-looking texture variations
- Smooth sky gradients
- Realistic terrain representation
- 3-8 minutes processing time

## Creative Applications

### Example 4: Artistic Style Transfer

**Goal**: Create an artistic mosaic that mimics a specific art style

**Materials Needed**:

- Target image (any subject)
- Material images from specific art movement (e.g., impressionist paintings)
- Creative eye for artistic effect

**Steps**:

1. **Curate Materials**

   - Collect 200-400 images from chosen art style
   - Ensure good color and texture variety
   - Resize to consistent dimensions if needed

2. **Configure for Artistic Effect**

   - Total tiles: 1200-1600
   - Max materials: 300-400
   - Color adjustment: 0.1 (preserve original art colors)
   - Max usage per image: 1 (maximum variety)
   - Adjacency penalty weight: 0.6 (avoid repetition)
   - Enable optimization: Yes
   - Optimization iterations: 2000

3. **Generate and Refine**
   - First attempt with settings above
   - Adjust color adjustment if needed
   - Try different tile counts for different effects

**Expected Results**:

- Target image rendered in chosen art style
- Unique artistic interpretation
- Each tile maintains original artwork character
- Cohesive artistic aesthetic

### Example 5: Pixel Art Style

**Goal**: Create a retro pixel art style mosaic

**Materials Needed**:

- Target image with clear, bold colors
- Pixel art sprites or simple geometric shapes
- Retro color palette

**Steps**:

1. **Material Preparation**

   - Collect pixel art sprites, icons, or simple shapes
   - Ensure consistent pixel dimensions
   - Focus on bold, saturated colors

2. **Settings for Pixel Art Style**

   - Total tiles: 400-800 (fewer for chunky pixel effect)
   - Max materials: 100-200
   - Color adjustment: 0.0 (preserve pixel art colors)
   - Max usage per image: 5+ (allow repetition)
   - Adjacency penalty weight: 0.1 (allow clustering)
   - Enable optimization: No (preserve blocky appearance)

3. **Generate Pixel Art**
   - Lower tile count creates chunkier effect
   - Monitor results for desired pixelated look
   - Adjust tile count as needed

**Expected Results**:

- Retro pixel art aesthetic
- Chunky, blocky appearance
- Bold color contrasts
- Nostalgic gaming feel

### Example 6: Collage Effect

**Goal**: Create a photo collage mosaic using personal photos

**Materials Needed**:

- Target image (family photo, pet, etc.)
- Collection of personal photos
- Sentimental value for creative project

**Steps**:

1. **Personal Photo Collection**

   - Gather 200-500 personal photos
   - Include variety: people, places, objects
   - Mix color and black & white photos

2. **Collage Settings**

   - Total tiles: 1500-2000
   - Max materials: 400-500
   - Color adjustment: 0.6 (help photos blend)
   - Max usage per image: 1-2 (see many different photos)
   - Adjacency penalty weight: 0.5 (avoid photo clustering)
   - Enable optimization: Yes
   - Optimization iterations: 1500

3. **Create Memory Collage**
   - Each tile will be a recognizable photo
   - Resulting mosaic tells a story through images
   - Perfect for gifts or memory preservation

**Expected Results**:

- Personal photo collage
- Each tile is a distinct memory
- Meaningful artistic creation
- Unique family artwork

## Advanced Techniques

### Example 7: Multi-Pass Processing

**Goal**: Create extremely high-quality mosaics through multiple processing passes

**Materials Needed**:

- High-resolution target image
- Large, diverse material collection (1000+ images)
- Powerful computer with 16GB+ RAM

**Steps**:

1. **First Pass - Structure**

   - Total tiles: 3000
   - Max materials: 1000
   - Color adjustment: 0.3
   - Max usage per image: 3
   - Adjacency penalty weight: 0.5
   - Enable optimization: Yes
   - Optimization iterations: 2000

2. **Analyze Results**

   - Enable verbose logging
   - Note areas with poor tile matching
   - Identify repetitive patterns
   - Check color accuracy

3. **Second Pass - Refinement**

   - Adjust settings based on first pass
   - Increase color adjustment if needed
   - Add more materials if available
   - Focus on problem areas

4. **Final Pass - Polish**
   - Fine-tune all settings
   - Maximum optimization iterations
   - Highest quality output format
   - Final quality check

**Expected Results**:

- Museum-quality mosaic
- Exceptional detail and color accuracy
- Minimal artifacts or repetition
- Professional presentation quality

### Example 8: Batch Processing Workflow

**Goal**: Process multiple related images with consistent settings

**Materials Needed**:

- Multiple target images (portrait series, landscape collection)
- Consistent material collection
- Organized file structure

**Steps**:

1. **Prepare File Structure**

   ```
   project/
   ├── targets/
   │   ├── image1.jpg
   │   ├── image2.jpg
   │   └── image3.jpg
   ├── materials/
   │   ├── tile1.jpg
   │   └── ...
   └── outputs/
       ├── mosaic1.png
       ├── mosaic2.png
       └── mosaic3.png
   ```

2. **Establish Baseline Settings**

   - Process first image with various settings
   - Document successful configuration
   - Note processing time and resource usage

3. **Process Series**

   - Use identical settings for all images
   - Process one at a time
   - Monitor system resources
   - Save with consistent naming

4. **Quality Control**
   - Review all results
   - Ensure consistent quality
   - Note any images needing adjustment
   - Reprocess if necessary

**Expected Results**:

- Consistent visual style across series
- Predictable processing times
- Organized output collection
- Professional workflow

## Troubleshooting Examples

### Example 9: Fixing Poor Results

**Problem**: Mosaic looks blurry or colors don't match

**Diagnostic Steps**:

1. **Enable Verbose Logging**

   - Check advanced settings
   - Enable verbose logging
   - Regenerate mosaic
   - Review log messages

2. **Analyze Issues**

   - Check material loading messages
   - Look for color matching warnings
   - Note any error messages
   - Review processing statistics

3. **Adjust Settings**

   - Increase color adjustment strength
   - Add more material images
   - Increase total tiles for more detail
   - Enable optimization if disabled

4. **Iterative Improvement**
   - Make one change at a time
   - Test results
   - Document effective changes
   - Build optimal configuration

**Expected Results**:

- Systematic problem identification
- Targeted setting adjustments
- Improved mosaic quality
- Repeatable success

### Example 10: Performance Optimization

**Problem**: Mosaic generation takes too long or uses too much memory

**Optimization Steps**:

1. **Baseline Measurement**

   - Record current processing time
   - Monitor memory usage
   - Note system specifications
   - Document current settings

2. **Systematic Reduction**

   - Reduce total tiles by 25%
   - Reduce max materials by 25%
   - Disable optimization temporarily
   - Test processing time

3. **Find Balance Point**

   - Gradually increase settings
   - Monitor performance impact
   - Find optimal balance
   - Document final configuration

4. **System Optimization**
   - Close unnecessary applications
   - Ensure adequate free memory
   - Use release builds
   - Consider hardware upgrades

**Expected Results**:

- Acceptable processing time
- Stable memory usage
- Maintained quality
- Optimized workflow

## Tips for Success

### Material Collection Tips

1. **Diversity is Key**

   - Wide range of colors
   - Various textures and patterns
   - Different subjects and themes
   - Mix of bright and dark images

2. **Quality Matters**

   - High resolution when possible
   - Good exposure and contrast
   - Minimal noise or artifacts
   - Consistent format (PNG or JPG)

3. **Quantity Guidelines**
   - Minimum 100 images for basic mosaics
   - 300-500 for good variety
   - 1000+ for professional results
   - Match to your tile count needs

### Settings Optimization

1. **Start Conservative**

   - Begin with lower tile counts
   - Use moderate material limits
   - Enable optimization
   - Adjust based on results

2. **Iterative Improvement**

   - Make one change at a time
   - Test results thoroughly
   - Document successful settings
   - Build configuration library

3. **System Awareness**
   - Monitor resource usage
   - Stay within system limits
   - Use release builds
   - Plan for processing time

### Creative Exploration

1. **Experiment with Styles**

   - Try different material themes
   - Vary color adjustment strengths
   - Explore different tile counts
   - Mix artistic and photographic materials

2. **Document Everything**

   - Keep notes on successful settings
   - Record processing times
   - Save example outputs
   - Build personal style guide

3. **Share and Learn**
   - Share results with community
   - Learn from others' techniques
   - Contribute to documentation
   - Explore new applications

These examples provide a foundation for creating stunning mosaics with the GUI application. Each example builds on previous concepts while introducing new techniques and creative possibilities.
