# Advanced Settings

The Advanced Settings panel contains expert-level configuration options that allow you to fine-tune every aspect of the mosaic generation process. This section is **collapsible** by default to keep the interface clean for beginners while providing powerful customization for advanced users.

## Accessing Advanced Settings

### Panel Location
The Advanced Settings panel is located below the Grid Settings panel in the main interface.

### Expanding the Panel
- **Click the ► arrow** next to "Advanced Settings" to expand
- **Click the ▼ arrow** to collapse the panel
- **Default state**: Collapsed to reduce interface complexity

### Panel Organization
The advanced settings are organized into three logical sections:
1. **Configuration** - Core processing parameters
2. **Optimization** - Post-placement enhancement options  
3. **Debugging** - Troubleshooting and analysis tools

## Configuration Section

This section contains settings that directly affect how tiles are selected, processed, and matched to the target image.

### Max Materials

**Purpose**: Limits the number of material images loaded from the selected directory.

**Default**: 500 images

**Range**: 10 to 10,000+ (practical limits depend on system resources)

**Impact**:
- **Lower values (100-300)**:
  - Faster loading and processing
  - Reduced memory usage
  - Limited tile variety
  - Good for testing and quick previews

- **Higher values (500-2000)**:
  - Longer loading time
  - Increased memory usage
  - Better tile variety and matching
  - Recommended for final high-quality mosaics

**Recommendations**:
- Match this setting to your material collection size
- Start with 500 and adjust based on results
- Consider system RAM when using high values
- For collections with >1000 images, consider organizing by theme

### Color Adjustment (0.0-1.0)

**Purpose**: Controls how much the tile colors are adjusted to match the target image regions.

**Default**: 0.3 (30% adjustment)

**Range**: 0.0 (no adjustment) to 1.0 (maximum adjustment)

**Technical Details**:
- Uses HSV color space for natural-looking adjustments
- Preserves image details while improving color accuracy
- Applied during the final compositing phase

**Impact**:
- **0.0 (No Adjustment)**:
  - Pure color matching based on original tile colors
  - May look artificial if tiles don't match well
  - Preserves original tile appearance
  - Good for artistic effect or when tiles match well naturally

- **0.3 (Balanced - Recommended)**:
  - Moderate color adjustment for better blending
  - Maintains tile character while improving accuracy
  - Good balance between realism and artistic effect
  - Works well with diverse material collections

- **1.0 (Maximum Adjustment)**:
  - Heavy color adjustment for close target matching
  - May lose original tile characteristics
  - Can create very realistic results
  - May introduce color artifacts with extreme adjustments

**Recommendations**:
- Start with 0.3 for most projects
- Use 0.0 for artistic or stylized effects
- Increase to 0.5-0.8 for photorealistic results
- Use 1.0 only when tile colors are very different from target

### Max Usage Per Image

**Purpose**: Prevents overuse of individual tile images by limiting how many times each can be used.

**Default**: 3 uses per image

**Range**: 1 to 100+ (practical limits depend on grid size and material count)

**Impact**:
- **1 (Maximum Variety)**:
  - Each tile used only once
  - Requires materials ≥ grid cells
  - Creates maximum visual variety
  - May sacrifice color accuracy for variety

- **3 (Balanced - Recommended)**:
  - Allows moderate repetition
  - Good balance between variety and color accuracy
  - Works well with moderate material collections
  - Prevents obvious repetition patterns

- **10+ (Flexible Matching)**:
  - Allows frequent reuse of good matches
  - Prioritizes color accuracy over variety
  - Good for small material collections
  - May create noticeable repetition patterns

**Recommendations**:
- Use 1 when you have materials ≥ total tiles
- Use 3 for balanced results with moderate collections
- Increase for small material collections or when color accuracy is critical
- Monitor the generation log for usage tracking information

#### Interaction with Fallback System

The Max Usage Per Image setting works seamlessly with the GUI's three-stage fallback system (added 2025-01-11) to ensure no grid cells remain empty:

**Fallback Behavior**:
1. **Normal Operation**: Tiles are used up to the specified limit
2. **When Limit Reached**: If all suitable tiles have reached their usage limit:
   - The system automatically resets the usage tracker
   - Previously used tiles become available again
   - Adjacency constraints are still respected to maintain quality
3. **Final Fallback**: If constraints still prevent placement:
   - The system selects the best color match regardless of usage or adjacency
   - This guarantees every cell is filled

**Example Scenario**:
- Grid: 100×100 (10,000 cells)
- Materials: 500 images
- Max Usage: 3 per image
- Maximum possible placements: 1,500

In this case, the fallback system will:
1. Use each image up to 3 times (1,500 placements)
2. Reset usage tracking for remaining 8,500 cells
3. Continue placing tiles with best color matches
4. Ensure all 10,000 cells are filled

**Monitoring Fallback Activity**:
Enable "Verbose logging" to see when fallback mechanisms activate:
- "⚠️ Using fallback tile selection with reset usage tracker..."
- "⚠️ Using final fallback - best color match without adjacency constraints..."

### Adjacency Penalty Weight (0.0-1.0)

**Purpose**: Prevents similar tiles from being placed adjacent to each other, creating more natural-looking patterns.

**Default**: 0.3 (30% penalty)

**Range**: 0.0 (no penalty) to 1.0 (maximum penalty)

**Technical Details**:
- Uses precomputed similarity database for efficient lookup
- Applies penalty during tile selection phase
- Considers all four adjacent positions (up, down, left, right)

**Impact**:
- **0.0 (No Penalty)**:
  - Pure color-based matching
  - May create clusters of similar tiles
  - Fastest processing
  - Good when material diversity is high

- **0.3 (Balanced - Recommended)**:
  - Moderate penalty for adjacent similarity
  - Creates natural-looking patterns
  - Good balance between color accuracy and variety
  - Works well with most material collections

- **1.0 (Maximum Penalty)**:
  - Strong penalty for adjacent similarity
  - May sacrifice color accuracy for pattern variety
  - Creates very diverse patterns
  - May cause suboptimal color matching

**Recommendations**:
- Use 0.0 for maximum color accuracy
- Use 0.3 for balanced, natural-looking results
- Increase to 0.5-0.8 for more varied patterns
- Use 1.0 only when avoiding repetition is critical

## Optimization Section

This section controls post-placement optimization using simulated annealing algorithms.

### Enable Optimization

**Purpose**: Toggles post-placement optimization to improve tile placement quality.

**Default**: Enabled

**Technical Details**:
- Uses simulated annealing algorithm
- Swaps tiles to minimize total adjacency penalty
- Runs after initial placement is complete
- Preserves color accuracy while improving patterns

**Impact**:
- **Enabled (Recommended)**:
  - Improves tile placement quality
  - Reduces repetition patterns
  - Adds 10-50% to processing time
  - Creates more professional-looking results

- **Disabled**:
  - Faster processing
  - Uses initial placement without refinement
  - May have more noticeable patterns
  - Good for quick previews or testing

**Recommendations**:
- Keep enabled for final mosaics
- Disable for quick testing or previews
- Required for best results when adjacency penalty weight > 0.0

### Optimization Iterations

**Purpose**: Controls how many optimization steps to perform during the simulated annealing process.

**Default**: 1000 iterations

**Range**: 1 to 10,000+ (practical limits depend on patience and quality requirements)

**Technical Details**:
- Each iteration attempts to swap two tiles
- Uses temperature decay to gradually reduce acceptance of worse swaps
- Convergence typically occurs within 500-2000 iterations
- Diminishing returns beyond 2000 iterations for most images

**Impact**:
- **100-500 (Quick Optimization)**:
  - Fast optimization with modest improvement
  - Good for testing optimization effects
  - May not reach full convergence
  - Adds 5-15% to processing time

- **1000 (Balanced - Recommended)**:
  - Good balance between quality and time
  - Typically achieves 80-90% of possible improvement
  - Adds 15-30% to processing time
  - Suitable for most mosaics

- **2000+ (Thorough Optimization)**:
  - Maximum quality improvement
  - Diminishing returns beyond 2000
  - Adds 30-60% to processing time
  - Recommended for final, high-quality mosaics

**Recommendations**:
- Start with 1000 for most projects
- Use 500 for quick testing
- Increase to 2000 for final mosaics
- Monitor improvement percentage in logs to gauge effectiveness

## Debugging Section

This section provides tools for troubleshooting and detailed analysis of the mosaic generation process.

### Verbose Logging

**Purpose**: Enables detailed debug output for troubleshooting and analysis.

**Default**: Disabled

**Impact**:
- **Enabled**:
  - Provides comprehensive processing information
  - Shows detailed tile selection decisions
  - Includes performance metrics and timing
  - Helpful for troubleshooting issues
  - Generates extensive log output

- **Disabled**:
  - Shows only essential progress information
  - Cleaner, more focused log output
  - Faster processing (minimal performance impact)
  - Suitable for normal operation

**Log Information When Enabled**:
- Individual tile loading and Lab color calculation
- Detailed tile selection process for each grid cell
- Usage tracker status and resets
- Similarity database operations
- Optimization step details
- Performance timing information

**Recommendations**:
- Enable when troubleshooting generation issues
- Use for understanding algorithm behavior
- Enable for performance analysis
- Required when reporting bugs or issues
- Keep disabled for normal operation

## Advanced Usage Patterns

### High-Quality Mosaics
For maximum quality results:
```
Max Materials: 1000+
Color Adjustment: 0.5
Max Usage Per Image: 1-2
Adjacency Penalty Weight: 0.4
Enable Optimization: Yes
Optimization Iterations: 2000
```

### Quick Previews
For fast testing and iteration:
```
Max Materials: 200
Color Adjustment: 0.3
Max Usage Per Image: 5
Adjacency Penalty Weight: 0.2
Enable Optimization: No
Optimization Iterations: N/A
```

### Artistic Effects
For stylized, artistic results:
```
Max Materials: 300
Color Adjustment: 0.0
Max Usage Per Image: 3
Adjacency Penalty Weight: 0.8
Enable Optimization: Yes
Optimization Iterations: 1500
```

### Large-Scale Mosaics
For very large mosaics (5000+ tiles):
```
Max Materials: 2000
Color Adjustment: 0.4
Max Usage Per Image: 2
Adjacency Penalty Weight: 0.5
Enable Optimization: Yes
Optimization Iterations: 1000
```

## Performance Considerations

### Memory Usage
- **Max Materials**: Primary factor in memory usage
- **Grid Size**: Secondary factor (width × height)
- **Material Image Size**: Affects loading time and memory
- **Optimization**: Minimal additional memory usage

### Processing Time
- **Loading Phase**: Depends on Max Materials and image sizes
- **Placement Phase**: Depends on grid size and adjacency penalty
- **Optimization Phase**: Depends on optimization iterations
- **Saving Phase**: Depends on output image size

### System Resources
- **CPU**: Intensive during placement and optimization
- **Memory**: Scales with material count and grid size
- **Disk**: Minimal usage except for output file
- **Network**: No network access required

## Troubleshooting Advanced Settings

### Common Issues

**Out of Memory Errors**:
- Reduce Max Materials
- Use smaller material images
- Reduce grid size
- Close other applications

**Slow Processing**:
- Reduce Optimization Iterations
- Lower Max Materials
- Disable Optimization for testing
- Use release builds

**Poor Results**:
- Increase Max Materials for better variety
- Adjust Color Adjustment for better blending
- Enable Optimization for better patterns
- Check material image quality and diversity

**Repetitive Patterns**:
- Increase Adjacency Penalty Weight
- Enable Optimization
- Reduce Max Usage Per Image
- Increase Max Materials

### Debug Process
1. Enable Verbose Logging
2. Start with simplified settings
3. Gradually increase complexity
4. Monitor log output for issues
5. Report bugs with detailed logs

The Advanced Settings panel provides fine-grained control over every aspect of mosaic generation. While the defaults work well for most scenarios, these settings allow you to optimize for specific use cases, material collections, and quality requirements.