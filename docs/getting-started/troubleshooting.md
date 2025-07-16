# Troubleshooting

This guide covers common issues you might encounter when using the Mosaic Art Generator and how to resolve them.

## Installation Issues

### Rust Version Too Old

**Error Message:**
```
error: package `mosaic-rust` cannot be compiled due to multiple errors
```

**Solution:**
```bash
# Update Rust to the latest version
rustup update

# Verify you have Rust 1.88.0 or later
rustc --version
```

### Missing Build Tools

**Error Message:**
```
error: linker `cc` not found
```

**Solution:**

**Linux (Ubuntu/Debian):**
```bash
sudo apt update
sudo apt install build-essential pkg-config
```

**macOS:**
```bash
xcode-select --install
```

**Windows:**
Install Visual Studio Build Tools or Visual Studio Community.

### Compilation Errors

**Error Message:**
```
error: failed to compile `mosaic-rust`
```

**Solution:**
```bash
# Clean and rebuild
cargo clean
cargo build --release

# If still failing, check Rust version
rustc --version
```

## Runtime Issues

### No Materials Found

**Error Message:**
```
Error: No materials found in directory 'materials'
```

**Causes & Solutions:**

1. **Directory doesn't exist:**
   ```bash
   # Create the directory
   mkdir materials
   # Add images to it
   ```

2. **No supported image files:**
   ```bash
   # Check what's in the directory
   ls -la materials/
   
   # Look for supported formats (PNG, JPG, JPEG)
   ls materials/*.{png,jpg,jpeg,PNG,JPG,JPEG}
   ```

3. **Wrong directory path:**
   ```bash
   # Use absolute path
   ./target/release/mosaic-rust \
     --material-src /full/path/to/materials \
     --target photo.jpg \
     --output mosaic.jpg
   ```

### Aspect Ratio Warnings

**Warning Message:**
```
Warning: No materials match the target aspect ratio (tolerance: 0.10)
Falling back to using all available materials
```

**Solutions:**

1. **Increase tolerance:**
   ```bash
   --aspect-tolerance 0.2  # Allow ±20% difference
   ```

2. **Check material aspect ratios:**
   ```bash
   # Use ImageMagick to check aspect ratios
   identify -format "%f: %w/%h = %[fx:w/h]\n" materials/*
   ```

3. **Prepare materials with consistent aspect ratios:**
   ```bash
   # Resize materials to consistent aspect ratio
   for img in materials/*; do
     convert "$img" -resize 400x400^ -gravity center -crop 400x400+0+0 "resized_$img"
   done
   ```

### Memory Issues

**Error Message:**
```
thread 'main' panicked at 'out of memory'
```

**Solutions:**

1. **Reduce memory usage:**
   ```bash
   ./target/release/mosaic-rust \
     --target photo.jpg \
     --material-src materials \
     --output mosaic.jpg \
     --max-materials 100 \
     --grid-w 40 \
     --grid-h 30
   ```

2. **Check system memory:**
   ```bash
   # Linux/macOS
   free -h
   
   # Monitor memory usage during execution
   top -p $(pgrep mosaic-rust)
   ```

3. **Use smaller target image:**
   ```bash
   # Resize target image
   convert large-target.jpg -resize 1920x1080 target.jpg
   ```

### Performance Issues

**Problem: Very slow processing**

**Solutions:**

1. **Ensure using release build:**
   ```bash
   # Wrong - debug build is slow
   ./target/debug/mosaic-rust  # DON'T USE THIS
   
   # Correct - release build is fast
   ./target/release/mosaic-rust  # USE THIS
   ```

2. **Reduce computational load:**
   ```bash
   # Fast settings
   ./target/release/mosaic-rust \
     --target photo.jpg \
     --material-src materials \
     --output mosaic.jpg \
     --grid-w 30 \
     --grid-h 20 \
     --max-materials 200 \
     --enable-optimization false \
     --adjacency-penalty-weight 0.0
   ```

3. **Check system resources:**
   ```bash
   # Monitor CPU usage
   htop
   
   # Check if disk I/O is limiting
   iotop
   ```

### File Permission Issues

**Error Message:**
```
Permission denied (os error 13)
```

**Solutions:**

1. **Check file permissions:**
   ```bash
   # Make sure binary is executable
   chmod +x target/release/mosaic-rust
   
   # Check target image permissions
   ls -la photo.jpg
   ```

2. **Check directory permissions:**
   ```bash
   # Ensure materials directory is readable
   ls -la materials/
   
   # Ensure output directory is writable
   touch test-output.jpg && rm test-output.jpg
   ```

### Similarity Database Issues

**Error Message:**
```
Error: Failed to load similarity database
```

**Solutions:**

1. **Rebuild similarity database:**
   ```bash
   ./target/release/mosaic-rust \
     --target photo.jpg \
     --material-src materials \
     --output mosaic.jpg \
     --rebuild-similarity-db
   ```

2. **Delete corrupted database:**
   ```bash
   # Remove the database file
   rm similarity_db.json
   
   # Run again to rebuild
   ./target/release/mosaic-rust ...
   ```

## Quality Issues

### Poor Color Matching

**Problem: Colors don't match the target well**

**Solutions:**

1. **Increase color adjustment:**
   ```bash
   --color-adjustment-strength 0.6
   ```

2. **Use more diverse materials:**
   ```bash
   --max-materials 1500
   ```

3. **Check material color distribution:**
   ```bash
   # Ensure materials cover all major colors
   # Add materials with missing colors
   ```

### Repetitive Patterns

**Problem: Same materials appear in clusters**

**Solutions:**

1. **Increase adjacency penalty:**
   ```bash
   --adjacency-penalty-weight 0.4
   ```

2. **Limit material usage:**
   ```bash
   --max-usage-per-image 2
   ```

3. **Use more materials:**
   ```bash
   --max-materials 2000
   ```

### Blurry or Poor Quality Output

**Problem: Final mosaic lacks detail**

**Solutions:**

1. **Increase grid resolution:**
   ```bash
   --grid-w 120
   --grid-h 80
   ```

2. **Use higher resolution materials:**
   ```bash
   # Ensure materials are at least 512x512
   identify -format "%f: %wx%h\n" materials/* | head -10
   ```

3. **Use higher resolution target:**
   ```bash
   # Resize target to higher resolution
   convert target.jpg -resize 2560x1440 target-hd.jpg
   ```

## Debug Information

### Enable Verbose Output

```bash
# Set environment variable for detailed logging
RUST_LOG=debug ./target/release/mosaic-rust \
  --target photo.jpg \
  --material-src materials \
  --output mosaic.jpg
```

### Check System Information

```bash
# Check available memory
free -h

# Check CPU information
lscpu

# Check disk space
df -h

# Check Rust version
rustc --version
cargo --version
```

### Benchmark Performance

```bash
# Time the execution
time ./target/release/mosaic-rust \
  --target photo.jpg \
  --material-src materials \
  --output mosaic.jpg
```

## Common Parameter Mistakes

### Using Debug Build

**Wrong:**
```bash
./target/debug/mosaic-rust  # Very slow!
```

**Correct:**
```bash
./target/release/mosaic-rust  # Fast and optimized
```

### Unrealistic Grid Sizes

**Wrong:**
```bash
--grid-w 500 --grid-h 500  # 250,000 tiles - too many!
```

**Correct:**
```bash
--grid-w 100 --grid-h 100  # 10,000 tiles - reasonable
```

### Conflicting Parameters

**Wrong:**
```bash
--max-materials 10 --grid-w 100 --grid-h 100  # Not enough materials for 10,000 tiles
```

**Correct:**
```bash
--max-materials 1000 --grid-w 100 --grid-h 100  # Sufficient materials
```

## Getting Help

If you're still experiencing issues:

### 1. Check the Documentation
- [CLI Reference](/guide/cli-reference) - Complete parameter guide
- [Performance Tuning](/guide/performance-tuning) - Optimization tips
- [Examples](/guide/examples) - Working configurations

### 2. Search Existing Issues
Visit [GitHub Issues](https://github.com/naporin0624/mosaic-art-rust/issues) to see if others have encountered similar problems.

### 3. Create a Bug Report

When reporting issues, include:

```bash
# System information
uname -a
rustc --version
cargo --version

# Error message
./target/release/mosaic-rust --target photo.jpg --material-src materials --output mosaic.jpg

# Directory structure
ls -la materials/ | head -10
ls -la *.jpg
```

### 4. Common Information to Include

- **Operating System**: Linux, macOS, Windows
- **Rust Version**: Output of `rustc --version`
- **Command Used**: Full command line with parameters
- **Error Message**: Complete error output
- **File Information**: 
  - Target image size and format
  - Number of materials
  - Material image sizes
- **System Resources**: Available RAM, CPU cores

## Prevention Tips

### 1. Start Small
Always test with smaller configurations first:
```bash
# Test configuration
--grid-w 20 --grid-h 15 --max-materials 50
```

### 2. Use Release Build
Always use the optimized release build:
```bash
cargo build --release
./target/release/mosaic-rust
```

### 3. Monitor Resources
Keep an eye on memory and CPU usage during processing.

### 4. Validate Inputs
- Check that target image exists and is readable
- Verify materials directory contains supported images
- Ensure sufficient disk space for output

### 5. Keep Settings Reasonable
- Grid sizes under 200×200
- Material counts under 5,000
- Optimization iterations under 10,000

Following these guidelines will help you avoid most common issues. If you continue to experience problems, don't hesitate to seek help through the [GitHub Issues](https://github.com/naporin0624/mosaic-art-rust/issues) page.