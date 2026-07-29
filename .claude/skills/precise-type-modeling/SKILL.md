---
name: precise-type-modeling
description: Use when authoring or converting Rust types in this photomosaic generator — defining a struct, enum, or function signature for grid cells, tile-placement outcomes, Lab colors, usage counts, adjacency penalties, or optimization config; typing several correlated fields as Option<T>; or about to reach for String, serde_json::Value, HashMap<String, String>, an over-permissive Vec<u8>, an `as` cast, or unwrap()/expect() to avoid modeling a value.
---

# Precise Type Modeling

## Overview

A type must model reality **exactly**: every argument and every state, no looser than the truth. Stringly-typed fields, bare `serde_json::Value`, `HashMap<String, String>` for owned data, over-permissive `Vec<u8>` blobs, `as` casts, and `unwrap()`/`expect()` are where models go to die — each says "I gave up modeling here." The goal is not "it compiles" but "impossible states are unrepresentable," which is also why clippy denies `unwrap_used`, `expect_used`, and `as_conversions` here.

## The rules (minimum bar)

1. **No escape-hatch types for data you control.** Ban `String`/stringly-typed values, `serde_json::Value`, and `HashMap<String, String>` for "arbitrary" tile options, flags, or metadata — "arbitrary" almost always means "I didn't enumerate it yet." Enumerate the fields in a struct, or model the variants as an `enum`.
2. **Untyped data only at a true external boundary, and only in transit.** Raw JSON loaded from a similarity-DB cache file, bytes read from an image file, or CLI arg strings may arrive untyped — parse it immediately into a concrete type (`serde::Deserialize`, `TryFrom`, a `parse_*` function), chaining with `?`/`.map_err` rather than nested matches (see `chaining-result-combinators`). `serde_json::Value`/`Vec<u8>` must never rest in a field or return type — it's a doorway, not a room.
3. **Model every state as an enum — the enum IS the state-transition diagram.** N possible states = N variants, each carrying only that state's data, so illegal field access (reading a tile's assigned path off a usage-limit failure) is a compile error. Consuming it exhaustively via `match` is the job of `branching-modeled-state-with-match` — reference it, don't re-derive it here.
4. **Optional fields are a smell — usually a collapsed enum.** If `usage_count` is only meaningful when assignment failed on the usage limit, and `lab_distance` is only meaningful when assignment succeeded, that's two variants, not two `Option<T>` fields on one struct. Reserve `Option<T>` for a field that is genuinely, independently absent (`total_tiles: Option<u32>` in `MosaicSettings`, where auto-calculation may not have run yet), not for state.
5. **Compiler-checked construction over `as` casts or `unsafe` transmutes.** Build values with type-annotated `let` bindings and struct/enum literals so the compiler checks every field. Numeric narrowing/widening belongs to `explicit-primitive-conversion` — follow that skill, don't duplicate it here.
6. **Newtype wrap primitive units and IDs.** `GridX(usize)`, `GridWidth(usize)` vs `PixelWidth(u32)` — when two arguments share a primitive type, a caller can swap them and the compiler stays silent. A newtype per unit turns that mix-up into a compile error instead of a transposed grid coordinate or a mis-sized tile.

## Before / after

### Optionals hiding a state machine → enum (rules 3, 4)

```rust
// ❌ every field optional; nothing stops reading `.lab_distance` on a failed assignment
struct TileAssignment {
    ok: bool,
    path: Option<PathBuf>, lab_distance: Option<f32>,             // success only
    usage_count: Option<usize>,                                    // usage-limit failure only
    image_aspect: Option<f32>, target_aspect: Option<f32>, tolerance: Option<f32>, // aspect mismatch only
}
```

```rust
// ✅ the enum names each state; each variant carries ONLY its own data
enum TileAssignment {
    Assigned { path: PathBuf, lab_distance: f32 },
    UsageLimitReached { path: PathBuf, usage_count: usize },
    NoAspectRatioMatch { image_aspect: f32, target_aspect: f32, tolerance: f32 },
}

fn summarize(assignment: &TileAssignment) -> String {
    match assignment {
        TileAssignment::Assigned { path, lab_distance } =>
            format!("assigned {} (\u{394}E {lab_distance:.2})", path.display()),
        TileAssignment::UsageLimitReached { path, usage_count } =>
            format!("{} already used {usage_count} times", path.display()),
        TileAssignment::NoAspectRatioMatch { image_aspect, target_aspect, tolerance } =>
            format!("aspect {image_aspect:.2} not within {tolerance} of {target_aspect:.2}"),
        // add a 4th variant -> this match fails to compile until handled
    }
}
```

### "Arbitrary pass-through" → enumerated concrete type (rules 1, 2)

```rust
// ❌ HashMap<String, String> is a giveaway: the optimizer knobs were never modeled
struct OptimizerOptions {
    label: Option<String>,
    use_greedy_fallback: bool,
    flags: HashMap<String, String>,   // "callers can pass anything"
}
```

```rust
// ✅ name the knobs; a new one is a one-line type change, reviewable at compile time
// (this is the real shape of OptimizationConfig in src/optimizer.rs)
struct OptimizationConfig {
    max_iterations: usize,
    initial_temperature: f32,
    temperature_decay: f32,
    report_interval: usize,
}
```

Parse the similarity-DB cache straight into the model at the boundary — never store the raw JSON: `let db = SimilarityDatabase::load_from_file(&cache_path)?;` (see `src/similarity.rs`, which returns `anyhow::Result<Self>`, never a bare `serde_json::Value`).

### Newtypes over bare primitives (rule 6)

```rust
// ❌ four usize args in the same position are trivially swappable
fn cell_bounds(x: usize, y: usize, width: usize, height: usize) -> Rect { /* ... */ }

fn call_site(x: usize, y: usize, grid_width: usize, grid_height: usize) {
    cell_bounds(grid_height, grid_width, x, y); // compiles, silently addresses the wrong grid cell
}
```

```rust
// ✅ newtypes make the mix-up a compile error, not a wrongly filled cell
struct GridX(usize);
struct GridY(usize);
struct GridWidth(usize);
struct GridHeight(usize);
fn cell_bounds(x: GridX, y: GridY, width: GridWidth, height: GridHeight) -> Rect { /* ... */ }
```

## Quick reference

| You're about to write | Do instead |
|---|---|
| `String`/stringly-typed field for data you own | model it: enum or struct; if truly external, parse at the boundary |
| `serde_json::Value`/`HashMap<String, String>` as a field or return type | enumerate fields as concrete variants; parse via `Deserialize`/`TryFrom` first |
| struct with many `Option<T>` fields | split into an `enum` by state |
| `bool` flag + optional payload | one variant per state, payload non-optional inside it |
| `x as T` | a checked conversion — see `explicit-primitive-conversion` |
| two same-typed positional args (`usize, usize, ...`) | newtypes per unit/axis |
| `unwrap()`/`expect()` to "make it compile" | model the failure as a variant, or return `Result` |

## Common rationalizations

| Excuse | Reality |
|---|---|
| "The optimizer flags are arbitrary, I need `HashMap<String, String>`" | "Arbitrary" = unenumerated. List them; add a field when a flag is born. |
| "Optionals are simpler than an enum" | They push every `if let Some` check onto every reader, forever. `match` checks once. |
| "`as` is fine, I know the range" | `TryFrom` proves it for the compiler; `as` truncates silently the day you're wrong. |
| "`serde_json::Value` is the safe version of untyped data" | Only if narrowed immediately — as a resting field it's untyped data with extra steps. |
| "I'll model it properly later" | The `Option`-soup struct ships and never gets revisited. Model it now. |

## Red Flags — STOP

- Reaching for `String`, `serde_json::Value`, `HashMap<String, String>`, or `Vec<u8>` for "config"/"options"/"metadata"/"payload" you actually control.
- A struct where most fields are `Option<T>`, or a `bool`/`status` field that decides which *other* fields are meaningful — both are a discriminant; make it an enum.
- An `as` cast on anything other than a range already proven at compile time, or `unwrap()`/`expect()` papering over a state that should be its own variant — clippy denies both anyway.
- Two same-typed primitive parameters that could be transposed without a compile error.
