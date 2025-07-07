---
description: Generate mosaic art CLI command with optimal grid dimensions
---

# Mosaic Art Command Generator

Generate the optimal mosaic-rust CLI command based on:
- Target aspect ratio (e.g., 16:9, 4:3, 1:1, 9:16)
- Total number of tiles desired
- Number of available material images

## Usage
`/project:mosaic <aspect_ratio> <total_tiles> <materials_count> [target_path] [material_path] [output_path]`

Examples:
- `/project:mosaic 16:9 16000 2500`
- `/project:mosaic 16:9 16000 2500 ./yoko.png ../sozai`
- `/project:mosaic 4:3 12000 2000 ./portrait.jpg ./materials ./output.png`
- `/project:mosaic 9:16 16000 2500` (portrait orientation)

## Parameters
1. **aspect_ratio**: Target aspect ratio (format: "width:height")
2. **total_tiles**: Total number of tiles to use in the mosaic
3. **materials_count**: Number of available material images
4. **(optional) target_path**: Path to target image (default: "./yoko.png")
5. **(optional) material_path**: Path to material images directory (default: "../sozai")
6. **(optional) output_path**: Output path (auto-generated if not specified)

## Task
Calculate the optimal grid dimensions (grid_w × grid_h) that:
1. Produces approximately the requested total_tiles
2. Maintains the target aspect ratio as closely as possible
3. Calculates appropriate max_usage_per_image based on total_tiles/materials_count

Then output the complete mosaic-rust CLI command with these settings:
- Calculated grid dimensions (--grid-w and --grid-h)
- Max materials set to materials_count (--max-materials)
- Max usage per image (--max-usage-per-image, ceiling of total_tiles/materials_count)
- Standard optimization settings:
  - --adjacency-penalty-weight 0.25
  - --optimization-iterations 1500
  - --color-adjustment-strength 0.4

Show the calculation details before the command:
- Grid dimensions and actual tile count
- Max usage per image calculation
- Then the formatted CLI command

## Available CLI Arguments

Required arguments:
- `--target` / `-t`: Target image path
- `--material-src` / `-m`: Material images directory
- `--output` / `-o`: Output file path

Optional arguments:
- `--grid-w`: Number of tiles horizontally (default: 50)
- `--grid-h`: Number of tiles vertically (default: 28)
- `--max-materials`: Maximum number of materials to use (default: 500)
- `--aspect-tolerance`: Aspect ratio tolerance (default: 0.1)
- `--max-usage-per-image`: Maximum times each image can be used (default: 3)
- `--adjacency-penalty-weight`: Weight for adjacency penalty (default: 0.3, 0.0 to disable)
- `--enable-optimization`: Enable post-placement optimization (default: true)
- `--optimization-iterations`: Maximum optimization iterations (default: 1000)
- `--similarity-db`: Path to similarity database (default: "similarity_db.json")
- `--rebuild-similarity-db`: Force rebuild similarity database
- `--color-adjustment-strength`: Enable color adjustment for better matching (0.0 to 1.0, default: 0.3)
- `--show-time`: Show time tracking information (default: true)
- `--show-grid`: Show grid visualization during processing (default: true)