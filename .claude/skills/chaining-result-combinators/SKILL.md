---
name: chaining-result-combinators
description: Use when composing or chaining Result values across the mosaic pipeline — sequencing image-load, decode, resize, and Lab-conversion steps (src/tile_cache.rs, src/similarity.rs); tempted by unwrap()/expect() on a missing material file or a corrupt similarity-cache JSON; discarding a save/load Result with let _ = ...; or deciding whether a failure becomes a CLI exit code (src/main.rs) or a rendered ProcessingState::Error (src/gui/app_full.rs).
---

# Chaining Result combinators

## Overview

Once a value is a `Result`, **keep it one.** Compose every step with combinators (`.and_then` / `.map` / `.map_err` / `.or_else` / `.inspect_err`) or the `?` operator, and collapse exactly once, at the consumption edge, into the outside-world value — a process exit code, a GUI error state, a log line. The error channel stays `anyhow::Result` (or a narrower `Option`) the whole way through; the edge is where — and the only place where — it collapses.

The library modules (`src/similarity.rs`, `src/tile_cache.rs`, `src/adjacency.rs`, `src/optimizer.rs`) never collapse: they return `Result<T, anyhow::Error>` or `Option<T>` outward and let `src/main.rs` (the `mosaic-rust` CLI) or `src/gui/app_full.rs` (the `mosaic-gui` app) decide what a failure means to a human. This split is not cosmetic here: `generate_mosaic_internal` runs the same decode/resize/Lab pipeline as the CLI, but inside `mosaic-gui` it executes via `tokio::task::spawn_blocking` on the same process as the render loop — a panic anywhere in that pipeline that *isn't* wrapped by `spawn_blocking` takes down the whole GUI window, not just one generation run. **RELATED:** `precise-type-modeling` owns the error/type design itself; `branching-modeled-state-with-match` owns the exhaustive match at the edge (e.g. over `ProcessingState`); `early-return-guards` owns validating preconditions (missing target file, non-existent material directory) before the fallible chain starts.

## The combinators (the whole vocabulary)

| Combinator | Use for |
|---|---|
| `?` | Inside a function returning `Result`: propagate the next fallible step straight-line, converting via `From`/anyhow's blanket impl. The idiomatic backbone — prefer it over `.and_then` in a function body. |
| `.and_then(fn)` | Next step that can itself fail, as an expression (`fn` returns a `Result`). Short-circuits on `Err`. Use when chaining at expression level. |
| `.map(fn)` | Transform the success value (cannot fail) — e.g. turning a decoded `DynamicImage` into a `Lab` via `MosaicGeneratorImpl::calculate_average_lab`, which never fails. |
| `.map_err(fn)` | Normalize the error — e.g. `.map_err(\|e\| format!("Failed to load target image: {e}"))`, the real conversion `generate_mosaic_internal` uses (`src/gui/app_full.rs`) so the final match stays exhaustive. |
| `.or_else(fn)` | **Recover** from an error: return `Ok(fallback)` for the variant you handle, `Err(e)` to re-propagate the rest. |
| `.inspect(fn)` / `.inspect_err(fn)` | Fire a side-effect (log, progress message) without changing the value — the analog of `andTee`/`tap`. |

Both keep the value a `Result`; pick `?` for linear propagation inside a `Result`-returning function, combinators when transforming or normalizing at expression level. Either way the banned move is the same: collapsing to a bare value before the edge.

## The recipe

Loading a material tile — decode, resize, measure its average Lab color — and loading the similarity-cache database: each step can fail; the chain never unwraps mid-flow.

```rust
// Real signatures — src/tile_cache.rs, src/similarity.rs, src/lib.rs
pub fn resize_image(img: &DynamicImage, width: u32, height: u32) -> anyhow::Result<ImageBuffer<Rgb<u8>, Vec<u8>>>;
fn calculate_average_lab(img: &DynamicImage) -> Lab; // infallible — a plain .map step
pub fn save_to_file(&self, path: &Path) -> anyhow::Result<()>;
pub fn load_from_file(path: &Path) -> anyhow::Result<Self>;

// Expression-chain style: decode -> resize -> measure, never unwrap mid-flow
fn measure_resized_tile(path: &Path, tile_w: u32, tile_h: u32) -> anyhow::Result<Lab> {
    image::open(path)
        .map_err(anyhow::Error::from)
        .and_then(|img| resize_image(&img, tile_w, tile_h))
        .map(|resized| MosaicGeneratorImpl::calculate_average_lab(&DynamicImage::ImageRgb8(resized)))
}

// `?`-style: equivalent, and how src/main.rs::process_tile actually reads (real code)
fn process_tile(path: &Path, target_aspect: f32, aspect_tolerance: f32) -> anyhow::Result<Option<Tile>> {
    let img = image::open(path)?;
    let (width, height) = img.dimensions();
    let aspect_ratio = width as f32 / height as f32;
    if !MosaicGeneratorImpl::is_aspect_ratio_match(aspect_ratio, target_aspect, aspect_tolerance) {
        return Ok(None);
    }
    let lab_color = MosaicGeneratorImpl::calculate_average_lab(&img);
    Ok(Some(Tile { path: path.to_path_buf(), lab_color, aspect_ratio }))
}
```

The similarity-cache load already shows the one deliberate recovery point in this codebase — real code, `src/similarity.rs`:

```rust
/// Try to load from file, or create new if file doesn't exist
pub fn load_or_new(path: &Path) -> Self {
    match Self::load_from_file(path) {
        Ok(db) => { println!("Loaded similarity database from {path:?}"); db }
        Err(_) => { println!("Creating new similarity database"); Self::new() }
    }
}
```

A missing file and a corrupt-JSON file both collapse to `Self::new()` here — and *only* here. Elsewhere in `MosaicGenerator::new`, a failed save doesn't get swallowed the same way; it's handled once, explicitly:

```rust
// src/main.rs — real code: handle, don't discard
if let Err(e) = db.save_to_file(similarity_db_path) {
    eprintln!("Warning: Failed to save similarity database: {e}");
}
```

Consume once, at the edge — the CLI's `main` (`src/main.rs`, real code):

```rust
fn main() -> Result<()> {
    let args = Args::parse();
    let target_img = image::open(&args.target)?;
    // ...
    let mut generator = MosaicGenerator::new(/* .. */)?;
    generator.generate_mosaic(/* .. */)?;
    println!("Mosaic saved to {:?}", args.output);
    Ok(())
}
```

`?` propagates every fallible step to `main`'s return type; a top-level `Err` prints via anyhow's `Display` and the process exits non-zero. In `mosaic-gui`, the analogous collapse point is the `Message::MosaicGenerationCompleted` arm (`src/gui/app_full.rs`, real code) — never deeper in `update`/`view`:

```rust
Message::MosaicGenerationCompleted(result) => {
    self.progress_receiver = None;
    match result {
        Ok(output_path) => {
            self.processing_state = ProcessingState::Completed;
            self.log_messages.push(format!("💾 Saved to: {output_path}"));
        }
        Err(error) => {
            self.processing_state = ProcessingState::Error(error.clone());
            self.log_messages.push(format!("❌ Error: {error}"));
        }
    }
}
```

## Where the edge is

The collapse belongs only where the `Result` leaves the crate for the outside world: `src/main.rs`'s `main` (an `anyhow::Result<()>` becoming a process exit code), `src/gui/app_full.rs`'s `Message::MosaicGenerationCompleted` arm (the only place a generation `Result<String, String>` becomes a rendered `ProcessingState::Error`), or a test assertion (`#[cfg(test)]` may `.unwrap()`/`.expect()` freely — a failing assertion is the point). The library modules — tile decode/resize, similarity-database load/save, adjacency and optimization — return `Result`/`Option` onward and never print a final user-facing error, exit, or panic on an expected failure (missing material directory, undecodable tile image, corrupt similarity-cache JSON).

## Anti-patterns

| Instead of | Do |
|---|---|
| `match r { Ok(v) => .., Err(e) => .. }` mid-pipeline just to rewrap in `Ok`/`Err` | `.and_then` (success path) / `.or_else` (recovery) |
| `.unwrap()` / `.expect("...")` on an expected failure (missing material directory, undecodable tile image, corrupt similarity-cache JSON) | Carry the `Result`; collapse only at the edge. Clippy's `unwrap_used`/`expect_used` deny this outside tests. |
| `let _ = db.save_to_file(path);` silently discarding a fallible save | `.and_then` / `?`, or the real `if let Err(e) = ... { eprintln!(...) }` pattern already used in `MosaicGenerator::new` — handle or propagate, never drop |
| `if r.is_err() { return Err(...) } let v = r.unwrap();` mid-flow | `?` or `.and_then` |
| `match image::open(path) { ... }` wrapping a foreign/io call mid-pipeline | `.map_err(anyhow::Error::from)` or `?` (anyhow's blanket `From`), once, at the wrap point |
| Hand-rolled `as` cast to coerce an error/value across a boundary | A named conversion (`From`/`TryFrom`) — `as_conversions` is denied for a reason |

## Red Flags — STOP

- About to write `match` on a `Result` somewhere that is not the consumption edge (`main`'s top level, or the `Message::MosaicGenerationCompleted` arm) → use `.and_then`/`.or_else`, or `?` if you're inside a `Result`-returning function.
- About to call `.unwrap()` or `.expect(...)` outside `#[cfg(test)]` — e.g. on `image::open(path)` for a material file, or on `SimilarityDatabase::load_from_file` — → that's the compiler telling you the failure is unhandled, not that it can't happen.
- About to write `let _ = fallible_call();` → the `Result` is telling you something; handle it or propagate with `?`.
- About to reach for `panic!`/`unreachable!` on a value that came from I/O, a material file, or a similarity-cache file → model it as `anyhow::Result`/`Option` instead. Remember: a panic in `src/gui/app_full.rs` outside `spawn_blocking` takes the whole `mosaic-gui` window down, not just one generation run.
- A `match`/`if let Err` wrapping code that already returns a `Result`, just to re-throw the same error → let `?` or `.map_err` do it.
