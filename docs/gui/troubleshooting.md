# Troubleshooting

This guide helps you diagnose and resolve common issues when using the GUI application.

## Quick Diagnosis

### Application Won't Start
1. **Check system requirements**: Ensure your platform is supported
2. **Verify dependencies**: Install required system libraries
3. **Try debug mode**: Run with `RUST_LOG=debug` for detailed output
4. **Check permissions**: Ensure the executable has proper permissions

### Generation Fails
1. **Verify file paths**: Ensure all selected files and directories exist
2. **Check material directory**: Confirm it contains supported image files
3. **Enable verbose logging**: Get detailed error information
4. **Try simpler settings**: Reduce complexity to isolate issues

### Poor Performance
1. **Use release builds**: Debug builds are 3-5x slower
2. **Reduce settings**: Lower tile counts and material limits
3. **Free system resources**: Close other applications
4. **Check system specifications**: Ensure adequate RAM and CPU

## Common Issues and Solutions

### Installation and Setup Issues

#### GUI Application Won't Launch

**Symptoms**:
- Application fails to start
- Error message about missing libraries
- Immediate crash on startup

**Solutions**:

**Linux**:
```bash
# Install required GTK dependencies
sudo apt-get install libgtk-3-dev libxdo-dev

# For Fedora/RHEL
sudo dnf install gtk3-devel libxdo-devel

# For Arch Linux
sudo pacman -S gtk3 xdotool
```

**macOS**:
```bash
# Install Xcode command line tools
xcode-select --install

# Verify macOS version (10.15+ required)
sw_vers
```

**Windows**:
- Ensure Windows 10 or later
- Try running as administrator
- Check Windows Defender/antivirus settings

#### Build Errors

**Symptoms**:
- `cargo build` fails with compilation errors
- Missing dependencies during build
- Platform-specific build issues

**Solutions**:
```bash
# Update Rust toolchain
rustup update

# Clean build cache
cargo clean

# Rebuild with verbose output
cargo build --bin mosaic-gui --verbose

# Check for specific error messages
cargo check --bin mosaic-gui
```

### File Selection Issues

#### File Dialog Doesn't Appear

**Symptoms**:
- Clicking "Browse" buttons has no effect
- File dialogs appear but are empty
- Cannot select files or directories

**Solutions**:
1. **Check permissions**: Ensure the application has file system access
2. **Try different directories**: Test with directories you own
3. **Restart application**: Sometimes helps with dialog issues
4. **Check file paths**: Ensure no special characters in paths

#### Invalid File Path Errors

**Symptoms**:
- Error messages about invalid file paths
- Files selected but not recognized
- Directory selection fails

**Solutions**:
1. **Verify file existence**: Ensure files haven't been moved or deleted
2. **Check file permissions**: Ensure files are readable
3. **Avoid special characters**: Use simple filenames without spaces or symbols
4. **Try different locations**: Test with files in different directories

### Material Directory Issues

#### No Material Images Found

**Symptoms**:
- Error: "No material images found in the specified directory"
- Generation fails immediately
- Empty material directory warnings

**Solutions**:
1. **Verify file formats**: Ensure images are PNG, JPG, or JPEG
2. **Check directory contents**: Confirm directory isn't empty
3. **File extensions**: Ensure proper file extensions (.png, .jpg, .jpeg)
4. **Subdirectories**: Materials must be in the root of selected directory

#### Material Loading Errors

**Symptoms**:
- Some materials fail to load
- Warnings about corrupted images
- Inconsistent material counts

**Solutions**:
1. **Check image integrity**: Verify images aren't corrupted
2. **Consistent formats**: Use consistent image formats
3. **File size limits**: Very large images may cause issues
4. **Enable verbose logging**: Get detailed error information

### Generation Process Issues

#### Out of Memory Errors

**Symptoms**:
- Application crashes during generation
- System becomes unresponsive
- "Out of memory" error messages

**Solutions**:
1. **Reduce tile count**: Use fewer total tiles
2. **Limit materials**: Reduce max materials setting
3. **Smaller images**: Use lower resolution material images
4. **Close other apps**: Free up system memory
5. **Check system RAM**: Ensure adequate memory available

#### Slow Processing

**Symptoms**:
- Generation takes extremely long
- Progress bar moves very slowly
- System becomes sluggish

**Solutions**:
1. **Use release builds**: 
   ```bash
   cargo build --bin mosaic-gui --release
   ./target/release/mosaic-gui
   ```
2. **Reduce settings**:
   - Lower total tiles (500-1000)
   - Reduce max materials (200-500)
   - Disable optimization for testing
   - Reduce optimization iterations

3. **System optimization**:
   - Close unnecessary applications
   - Ensure adequate free disk space
   - Monitor system resources

#### Generation Hangs

**Symptoms**:
- Progress bar stops updating
- Application becomes unresponsive
- No error messages or logs

**Solutions**:
1. **Enable verbose logging**: Get detailed progress information
2. **Check system resources**: Monitor CPU and memory usage
3. **Restart application**: Force close and restart
4. **Reduce complexity**: Try simpler settings first

### Quality and Results Issues

#### Poor Mosaic Quality

**Symptoms**:
- Blurry or pixelated results
- Colors don't match target image
- Obvious repetition patterns

**Solutions**:
1. **Increase tile count**: Use more tiles for better detail
2. **Improve materials**: Use higher quality, more diverse images
3. **Adjust color settings**: Increase color adjustment strength
4. **Enable optimization**: Use optimization for better patterns

#### Repetitive Patterns

**Symptoms**:
- Same tiles appear frequently
- Obvious clustering of similar tiles
- Unnatural-looking patterns

**Solutions**:
1. **Increase material variety**: Use more diverse material images
2. **Adjust usage limits**: Reduce max usage per image
3. **Enable adjacency penalty**: Increase adjacency penalty weight
4. **Enable optimization**: Use optimization to improve patterns

#### Color Mismatches

**Symptoms**:
- Tiles don't match target colors well
- Mosaic looks too different from original
- Artificial-looking color combinations

**Solutions**:
1. **Increase color adjustment**: Use higher color adjustment values
2. **Better material selection**: Use materials with colors similar to target
3. **More materials**: Increase max materials for better matching
4. **Check material diversity**: Ensure wide range of colors in materials

### Interface Issues

#### Theme Problems

**Symptoms**:
- Theme doesn't switch properly
- Colors appear wrong
- Text hard to read

**Solutions**:
1. **Restart application**: Theme changes may require restart
2. **Check system theme**: Ensure system theme is compatible
3. **Try different theme**: Switch between light and dark modes
4. **Update system**: Ensure OS is up to date

#### Log Display Issues

**Symptoms**:
- Logs don't appear
- Log text is truncated
- Cannot scroll through logs

**Solutions**:
1. **Enable verbose logging**: Ensure logging is enabled
2. **Scroll manually**: Use scrollbar to navigate logs
3. **Resize window**: Make window larger to see more logs
4. **Clear logs**: Restart application to clear old logs

### Advanced Debugging

#### Enable Debug Mode

For detailed debugging information:

```bash
# Linux/macOS
RUST_LOG=debug ./target/release/mosaic-gui

# Windows (PowerShell)
$env:RUST_LOG="debug"
./target/release/mosaic-gui.exe

# Windows (Command Prompt)
set RUST_LOG=debug
./target/release/mosaic-gui.exe
```

#### Collect Debug Information

When reporting issues, include:

1. **System information**:
   - Operating system and version
   - Rust version (`rustc --version`)
   - Application version
   - System specifications (RAM, CPU)

2. **Error details**:
   - Exact error messages
   - Steps to reproduce
   - Debug logs (with verbose logging enabled)
   - Screenshots of error dialogs

3. **Configuration**:
   - Settings used when error occurred
   - File paths and directory structure
   - Material image details (count, formats, sizes)

#### Performance Profiling

For performance issues:

```bash
# Monitor resource usage
top -p $(pgrep mosaic-gui)

# Check memory usage
ps aux | grep mosaic-gui

# Monitor file system usage
iostat -x 1
```

### Platform-Specific Issues

#### Windows-Specific Issues

**Issue**: Console window appears alongside GUI
**Solution**: Ensure using the correct GUI binary, not CLI binary

**Issue**: File dialogs look non-native
**Solution**: Update Windows to latest version

**Issue**: Antivirus blocks execution
**Solution**: Add exception for mosaic-gui executable

#### macOS-Specific Issues

**Issue**: "App is damaged and can't be opened"
**Solution**: 
```bash
# Remove quarantine attribute
xattr -d com.apple.quarantine mosaic-gui
```

**Issue**: File dialogs don't appear
**Solution**: Grant file system access permissions in System Preferences

#### Linux-Specific Issues

**Issue**: GTK errors on startup
**Solution**:
```bash
# Install GTK development packages
sudo apt-get install libgtk-3-dev

# For Wayland users
export GDK_BACKEND=wayland
```

**Issue**: Font rendering issues
**Solution**:
```bash
# Install font packages
sudo apt-get install fonts-dejavu-core fonts-freefont-ttf
```

## Getting Help

### Self-Help Resources

1. **Enable verbose logging**: Always start with detailed logs
2. **Check system requirements**: Verify your platform is supported
3. **Try simpler settings**: Reduce complexity to isolate issues
4. **Search documentation**: Check other sections for related information

### Community Support

1. **GitHub Issues**: Report bugs and request features
2. **Documentation**: Comprehensive guides and examples
3. **Discord/Forums**: Community discussion and support
4. **Wiki**: User-contributed tips and solutions

### Reporting Bugs

When reporting issues:

1. **Use GitHub Issues**: Primary bug tracking location
2. **Include details**: System info, error messages, steps to reproduce
3. **Provide logs**: Debug logs with verbose logging enabled
4. **Add screenshots**: Visual evidence of issues
5. **Be specific**: Exact settings and conditions that cause issues

### Feature Requests

For new features:

1. **Check existing issues**: Avoid duplicates
2. **Describe use case**: Explain why the feature is needed
3. **Provide examples**: Show how the feature would be used
4. **Consider alternatives**: Suggest implementation approaches

## Prevention Tips

### System Maintenance

1. **Keep software updated**: Update OS and Rust toolchain regularly
2. **Monitor resources**: Keep adequate free disk space and RAM
3. **Regular backups**: Backup important material collections
4. **Clean builds**: Occasionally clean and rebuild the application

### Best Practices

1. **Start simple**: Begin with basic settings and gradually increase complexity
2. **Test incrementally**: Make small changes and test results
3. **Document successful settings**: Keep notes of working configurations
4. **Prepare materials**: Organize and verify material images before use

### Performance Optimization

1. **Use release builds**: Always use optimized builds for production
2. **Optimize materials**: Use consistent, high-quality material images
3. **Monitor resources**: Keep system resources available during generation
4. **Batch processing**: Process multiple mosaics with similar settings

By following this troubleshooting guide, you should be able to resolve most common issues with the GUI application. When in doubt, enable verbose logging and start with simpler settings to isolate problems.