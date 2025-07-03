#!/bin/bash

echo "Starting parallel image conversion..."
echo "Total images to process: $(find ../../sozai -mindepth 2 -type f \( -name "*.png" -o -name "*.jpg" -o -name "*.jpeg" -o -name "*.webp" \) | wc -l)"

# Function to convert a single image
convert_image() {
    local file="$1"
    
    # Get just the filename without extension
    filename=$(basename "$file")
    basename_no_ext="${filename%.*}"
    
    # Get directory name for uniqueness
    dirname=$(dirname "$file")
    parent_dir=$(basename "$dirname")
    
    # Create unique filename with parent directory name
    output_filename="${parent_dir}_${basename_no_ext}.png"
    output_path="../../sozai/${output_filename}"
    
    # Convert with ffmpeg to 3840x2160 PNG
    ffmpeg -i "$file" -vf "scale=3840:2160:force_original_aspect_ratio=decrease,pad=3840:2160:(ow-iw)/2:(oh-ih)/2:black" -y "$output_path" 2>/dev/null
    
    if [ $? -eq 0 ]; then
        echo "✓ $output_filename"
    else
        echo "✗ Failed: $file"
    fi
}

export -f convert_image

# Use xargs with parallel processing (4 concurrent jobs)
find ../../sozai -mindepth 2 -type f \( -name "*.png" -o -name "*.jpg" -o -name "*.jpeg" -o -name "*.webp" \) | \
    xargs -I {} -P 4 bash -c 'convert_image "$@"' _ {}

echo "Conversion completed!"

# Count results
converted_count=$(find ../../sozai -maxdepth 1 -name "*.png" | wc -l)
echo "Total converted images: $converted_count"