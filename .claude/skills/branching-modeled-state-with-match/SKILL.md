---
name: branching-modeled-state-with-match
description: Use when writing Rust that branches on an owned enum modeling state — TileStatus (NotStarted/InProgress/Completed) in the grid visualizer, a filled-vs-empty grid cell, a cache hit vs miss, or a simulated-annealing accept/reject decision — to choose behavior or a value, or when adding a variant to such an enum. Also when reaching for an if-let chain, a let-else, matches!, or a `_` wildcard arm to consume an enum this crate owns.
---

# Branching Modeled State With match

## Overview

When a value models state as an enum, **branch on it with `match` listing every variant, and never add a `_` wildcard arm or `..` catch-all on an enum you own.** A forgotten variant then becomes a compiler error that points at the exact `match`, not a silent runtime fallthrough — exhaustiveness checking is built into the language here, not something you have to simulate.

This skill is the *consume* side of a modeled type. **REQUIRED SUB-SKILL:** use `precise-type-modeling` to model the enum in the first place (every state a real variant, no stringly-typed status field). See also `early-return-guards` for guard clauses that short-circuit on a single variant.

## The recipe (NEW and existing code)

`TileStatus` in `src/grid_visualizer.rs` is this crate's modeled state: each grid cell is `NotStarted`, `InProgress`, or `Completed` while the mosaic fills in. `GridVisualizer::draw_grid` picks the display glyph with an exhaustive `match`:

```rust
pub enum TileStatus {
    NotStarted,
    InProgress,
    Completed,
}

let symbol = match self.tile_status[y][x] {
    TileStatus::NotStarted => '□',
    TileStatus::InProgress => '●',
    TileStatus::Completed => '■',
    // no `_` arm — every variant above is named
};
```

Add a variant `Skipped` to `TileStatus` and this `match` no longer compiles: `error[E0004]: non-exhaustive patterns`, pointing straight at `draw_grid`. Add the arm (and to every other `match` on `TileStatus`, including `get_progress_summary`'s filters), the build goes green. That is the entire payoff, and a `_` arm anywhere in the match set silently defeats it — the compiler stops telling you where to look.

Grouping variants with `|` stays exhaustive and is fine when two variants share behavior:

```rust
fn is_symbol_final(status: &TileStatus) -> bool {
    match status {
        TileStatus::Completed => true,
        TileStatus::NotStarted | TileStatus::InProgress => false,
    }
}
```

## `if let` / `let else`, and `matches!`

`if let` or `let ... else` on a multi-variant enum is only safe when the function genuinely cares about **one** variant (or one case, like `Option`'s `Some`) and every other case is uniformly "do nothing." `src/main.rs`'s adjacency check on the placement grid (`Vec<Vec<Option<PathBuf>>>`) is exactly this shape — an empty cell (`None`) means "nothing to check yet":

```rust
// OK: None (no tile placed at this neighbor yet) means "nothing to check"
if let Some(neighbor_path) = &self.placed_tiles[ny][nx] {
    if neighbor_path == tile_path {
        return false;
    }
}
```

The moment two variants need *different* handling, that's dispatch, not a guard — go back to `match` with every variant named, the way `draw_grid` does for `TileStatus`. `matches!` is for boolean predicates only (`let is_completed = matches!(status, TileStatus::Completed);`); it discards fields and can't drive per-variant behavior, so never reach for it to pick a return value.

## The `#[non_exhaustive]` exemption

A `_` arm is required — and only there — for enums you do **not** own, when the defining crate marks them `#[non_exhaustive]`. `image::DynamicImage` (a direct dependency here) is one:

```rust
match img {
    image::DynamicImage::ImageRgb8(_) => Format::Rgb8,
    image::DynamicImage::ImageRgba8(_) => Format::Rgba8,
    _ => return Err(anyhow::anyhow!("unsupported pixel format")), // required: image crate marks DynamicImage #[non_exhaustive]
}
```

This does not license a `_` arm on `TileStatus` or any other enum this crate defines — those are yours, model them fully.

## Why not the alternatives

| Anti-pattern | Why it fails |
|---|---|
| `match status { X => …, _ => fallback }` on an enum you own | The wildcard absorbs every future variant — adding one compiles clean and ships silently. |
| `if let X = status { .. } else { fallback }` when other variants need distinct handling | The `else` branch conflates all other variants into one fallback with no compiler signal when a new one needs its own case. |
| `matches!(status, X | Y)` used to pick a return value | `matches!` only yields a `bool` and discards per-variant fields — it can't drive behavior, only a predicate. |
| Matching on a raw `String`/`u8` status code instead of the modeled enum | No exhaustiveness at all; typos and missing cases compile fine. Model first (`precise-type-modeling`), then match. |

## Common mistakes

- Adding a `_ => fallback` "to be safe" — it disables exhaustiveness checking entirely.
- Reaching for `if let` when two or more non-matched variants actually need different behavior — that's dispatch, use `match`.
- Copying the `image::DynamicImage`-style `#[non_exhaustive]` wildcard onto this crate's own enums (`TileStatus` and friends), where nothing forces it.
- Matching on a stringly- or numerically-tagged field instead of the modeled enum.

## Red Flags — STOP

- About to write `match status { … , _ => … }` on an enum defined in this crate → name the remaining variants instead.
- About to write `if let` where the "else" case actually needs its own logic per variant → use `match`.
- About to use `matches!` to choose a return value or side effect → use `match`; `matches!` is a predicate only.
- About to add a variant and a `_` arm swallows it with no compiler error → remove the wildcard before it ships silently.
