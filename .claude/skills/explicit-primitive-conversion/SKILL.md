---
name: explicit-primitive-conversion
description: Use when converting between numeric types in this photomosaic generator, converting string↔number, or about to write `as` — u8 pixel channel ↔ f32 [0,1] color math, f32→u8 narrowing with clamp, usize↔u32 grid/tile index math, pixel counts (width * height) widened to f64 for averaging, or Lab/ΔE float math.
---

# Explicit Primitive Conversion

## Overview

The `as` cast hides two different failure modes behind one keyword. Float→int `as` **saturates**: `300.0_f64 as u8` is `255`, and `f64::NAN as u8` is `0` — not a panic, not a wraparound, a silent clamp. Int→int narrowing `as` **truncates to the low bits**: `300_i64 as u8` is `44`, and `-1_i32 as u32` is `4294967295` — the sign is reinterpreted, not preserved. Both are legal Rust and exactly what `clippy::as_conversions` and the `clippy::cast_possible_truncation` family already deny in this workspace. Those lints stop you at the call site; they cannot tell you **which** explicit conversion says what you meant. That choice is this skill.

**`as` for numeric/string conversion is banned outside the two narrow exceptions in "Still allowed" below. No other exceptions.**

**RELATED:** a value that was never modeled as a concrete type in the first place (a bare `String`/`serde_json::Value` standing in for a grid coordinate or a tile count) isn't this skill's job to fix — model it with `precise-type-modeling` first, then convert the modeled value explicitly.

## The rule

| Goal | ❌ banned (implicit/lossy) | ✅ required (explicit) |
|---|---|---|
| widen a number (lossless) | `grid_w as u64`, `x as f64` | `u64::from(grid_w)` / `x.into()` |
| narrow a number (fallible) | `(grid_w * grid_h) as usize`, `index as u16` | `usize::try_from(tiles)?` — handle or propagate the error |
| string → number | `s.parse().unwrap()` | `s.parse::<u32>()?` (turbofish or `let x: u32 = ...?`), error handled |
| number/value → string | ad-hoc `format!("{}", pos)` at every call site | `pos.to_string()`, or `impl Display for GridPosition` once |
| float → int pixel math | `(width as f64 * scale) as u32` inline | name the rounding — `.round()`/`.floor()`/`.trunc()` — then convert in one named, audited helper |
| bool from a count/flag | `(completed_tiles as u8) == 1`, `count as bool`-style tricks | a real comparison: `count != 0`, `neighbor.is_some()` |

**Still allowed — exactly two cases, both grounded in the per-pixel hot loop in `color_adjustment.rs`:**

1. **u8 widening that the type system, not an assumption, guarantees is lossless.** `pixel[0] as f32 / 255.0` (`color_adjustment.rs:55`) and `lut[pixel[0] as usize]` (`lib.rs:132`) run once per channel per pixel — millions of times per image. `u8`'s full range (0–255) always fits `f32` and `usize` on every platform Rust supports, so `f32::from(pixel[0])`/`usize::from(pixel[0])` would be equally zero-cost, but the bare `as` here is not a bug. This does **not** extend to `usize`/`u32` widening: those widths are platform-dependent, so even a "provably lossless" cast like `grid_w as u64` should still go through `u64::from(grid_w)` — the proof is an assumption a future platform can break, and `From` cannot be wrong.
2. **f32 → u8 narrowing immediately guarded by `.clamp()`.** `(final_color.red * 255.0).clamp(0.0, 255.0) as u8` (`color_adjustment.rs:83-85`) is fine as written: the `.clamp(0.0, 255.0)` right before the cast is the guard that makes the narrowing safe. Remove the clamp and the identical cast — `(final_color.red * 255.0) as u8` — becomes exactly the banned pattern, because sRGB round-trips and hue/saturation math can push a channel slightly outside `[0.0, 1.0]` before the multiply.

Also allowed: the `as` that std structurally requires for `f64 → i64` (there is no `TryFrom<f64>` in std), *if* it is isolated in a single named helper directly after `.round()`, not repeated inline.

## Before → after

```rust
// ❌ before — the multiply and the narrow both hide a failure mode
fn total_pixel_count(width: u32, height: u32) -> f64 {
    (width * height) as f64      // width * height overflows u32 (~4.29B px) before this cast runs
}

fn total_tile_count(grid_w: u32, grid_h: u32) -> usize {
    (grid_w * grid_h) as usize   // same overflow risk, plus u32 -> usize has no infallible `From`
}
```

```rust
// ✅ after — widen before multiplying, so the multiply can't overflow;
// narrow only through TryFrom, since u64 -> usize isn't guaranteed lossless
fn total_pixel_count(width: u32, height: u32) -> f64 {
    f64::from(width) * f64::from(height)   // u32 -> f64 is always exact
}

fn total_tile_count(grid_w: u32, grid_h: u32) -> Result<usize, TryFromIntError> {
    let tiles = u64::from(grid_w) * u64::from(grid_h);
    usize::try_from(tiles)
}
```

```rust
// ❌ before — a raw grid delta reinterpreted as an unsigned index
fn neighbor_index(x: i64, grid_width: u32) -> u32 {
    x as u32   // a negative x (off the left edge of the grid) becomes a huge index
}
```

```rust
// ✅ after — the fallibility is named, not swallowed
fn neighbor_index(x: i64) -> Result<u32, TryFromIntError> {
    u32::try_from(x)   // negative x is now a named error, not a huge tile index
}
```

## Why int→int `as` is the one that hides bugs

`as` between integer types keeps the low bits and reinterprets the sign — it never asks permission. A tile-usage counter that overflows `u32`, or a grid offset that went negative a few call sites away (walking off row 0 in `GridPosition::get_adjacent_positions`), becomes a wrong-but-plausible number instead of a caught error. `try_from`/`try_into` turns the same bug into a `Result` at the exact line it happens:

- "this tile count fits in a smaller counter" → `u32::try_from(tiles)?`, not `tiles as u32`
- "this f64 is a valid pixel count" → round it, then `try_from`, never a bare cast
- "this grid offset should never be negative" → `try_from` returns `Err`; `as` returns a wrong `usize`

## Common mistakes

| Mistake | Fix |
|---|---|
| `x as u32` "because the value is always in range" | Prove it: `u32::try_from(x)?` and propagate, or `.expect("documented invariant: ...")` at worst. `try_from` costs nothing on the happy path. |
| `(grid_w * grid_h) as usize` inline at a call site | Isolate the widen-then-multiply in one named helper (see `total_tile_count` above); never repeat the raw multiply-then-cast. |
| `s.parse().unwrap()` on a value read from outside the type system (a config string, a similarity-DB field) | `s.parse::<f32>()` and propagate/handle `ParseFloatError` — anything that arrived as text is untrusted, same as a network payload. |
| `format!("{}", grid_position)` scattered at every call site | `impl Display for GridPosition` once; call `.to_string()` or `{}` through the trait everywhere else. |
| `#[allow(clippy::as_conversions)]` on a whole file "to unblock the build" | Scope the allow to the one function that needs it — e.g. the two lines in `color_adjustment.rs` covered above — with a comment naming the invariant. |

## Red Flags — STOP

- About to write `as` between two numeric types outside the two cases in "Still allowed" → widening or narrowing? Use `From` or `TryFrom`, not `as`.
- About to write `.unwrap()` after `.parse()` → is the input a literal (infallible), or untrusted — a config value or file field? Untrusted needs `Result` handling.
- About to write `(x as f64 * y) as u32` or any float math ending in `as <int>` → name the rounding, then push the cast into one audited helper.
- About to compare a numeric value to `0`/`1` to fake a boolean (`x as u8 == 1`) → write the comparison directly; it already returns `bool`.
- About to write `f32 → u8 as u8` **without** a `.clamp()` immediately before it → add the clamp or use `try_from`-style range handling; an unguarded narrowing cast is exactly what this skill bans.

`clippy::as_conversions` and the `clippy::cast_possible_truncation`/`cast_sign_loss`/`cast_precision_loss` family already deny the lossy call sites; this skill exists for the part they can't check — *which explicit conversion (`From`, `TryFrom`, `parse`, `to_string`, a named rounding helper) expresses the intent*.
