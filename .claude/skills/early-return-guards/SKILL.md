---
name: early-return-guards
description: Use when writing or modifying a Rust function with conditional branches — writing a per-tile or per-material loop that filters candidates, nesting a bounds check inside a usage-limit check, threading a `let mut best_match` through branches to return at the end, or wrapping a whole grid-loop body in a condition.
---

# Early Return Guards

## Overview

The happy path stays at indentation level zero. Every precondition, edge case, and failure exits **at the top** with `return` / `?` / `continue` / `break`; what remains reads straight down. Nesting and `else`-after-exit are how a 5-line function becomes a pyramid nobody can review.

**A branch that ends in `return` / `continue` / `break` (or propagates via `?`) is never followed by `else`. Preconditions are checked as guards at the top, inverted. No exceptions.** The star tools are `let Some(x) = opt else { return None; };` / `let Ok(x) = res else { ... };` — a let-else keeps the happy path flat instead of nesting the rest of the function inside `if let` — and the `?` operator, the ultimate guard for `Result`/`Option` propagation (see `chaining-result-combinators` for composing the fallible chain itself once past the guards).

## The rule

| Situation | ❌ banned | ✅ required |
|---|---|---|
| precondition / edge case | wrap the body in `if valid { .. }` | invert and exit: `if !valid { return None; }` |
| branch ends in return/continue/break | `if c { return a; } else { return b; }` | drop the `else` — the code after the `if` *is* the else branch |
| result decided by branches | `let mut result = None;` set in if/else arms, single `return result;` at the end | `return` directly from each arm; last line returns the default |
| loop item filtering | nest the loop body in `if keep { .. }` | `if !keep { continue; }`, body stays flat |
| unwrapping an `Option`/`Result` before the real body | `if let Some(x) = opt { <rest of function> }` nesting everything | `let Some(x) = opt else { return None; };` then flat code |
| adding a condition to an already-nested function | add one more nesting level "to keep the diff small" | that's the signal — invert the function you're touching to guards first |

**Still allowed:** `if/else` where *neither* arm exits (two live continuations); `match` arms (see `branching-modeled-state-with-match`); expression-position `if/else` producing a value (`let x = if c { a } else { b };`); iterator chains (`.filter`/`.find`) that replace the loop entirely — often an even flatter form than a guard loop.

## Before → after

```rust
// ❌ before — result threaded through a pyramid
fn pick_tile<'a>(
    tiles: &'a [Tile],
    preferred_path: Option<&Path>,
    usage: &UsageTracker,
    min_aspect: f32,
) -> Option<&'a Tile> {
    let mut result = None;
    if !tiles.is_empty() {
        if let Some(preferred) = preferred_path {
            for tile in tiles {
                if tile.path == preferred {
                    if tile.aspect_ratio >= min_aspect && usage.can_use_image(&tile.path) {
                        result = Some(tile);
                    }
                }
            }
        }
        if result.is_none() {
            for tile in tiles {
                if tile.aspect_ratio >= min_aspect && usage.can_use_image(&tile.path) {
                    result = Some(tile);
                    break;
                }
            }
        }
    }
    result
}
```

```rust
// ✅ after — guards exit, happy path reads straight down
fn is_eligible(tile: &Tile, usage: &UsageTracker, min_aspect: f32) -> bool {
    tile.aspect_ratio >= min_aspect && usage.can_use_image(&tile.path)
}

fn pick_tile<'a>(
    tiles: &'a [Tile],
    preferred_path: Option<&Path>,
    usage: &UsageTracker,
    min_aspect: f32,
) -> Option<&'a Tile> {
    if tiles.is_empty() {
        return None;
    }
    if let Some(preferred) = preferred_path {
        for tile in tiles {
            if tile.path != preferred {
                continue;
            }
            if !is_eligible(tile, usage, min_aspect) {
                continue;
            }
            return Some(tile);
        }
    }
    tiles.iter().find(|t| is_eligible(t, usage, min_aspect))
}
```

Note the shape of the first loop: disqualify with `continue`, then return — this is the same two-pass shape as `MosaicGenerator::fallback_tile_selection` in `src/main.rs` (strict pass, then a relaxed pass once the strict one yields nothing). The eligibility condition, needed twice, became its own function instead of being copy-pasted into two nests. The second loop had no state to carry across iterations beyond "first match wins," so it collapses further, from a guard loop into `.find` — the iterator chain *is* the guard here, one step flatter than `continue`/`return`.

One wrinkle specific to this codebase: in a `rayon` `par_iter().filter_map(...)` closure (as in `load_tiles`'s tile loading), `continue`/`break` don't exist and a bare `return` exits the closure, not the enclosing function — the guard shape is right (`if !ok { return None; }` at the top of the closure), just don't mistake it for an early return from `load_tiles` itself.

## Common mistakes

| Mistake | Fix |
|---|---|
| `else { return default; }` after an `if .. { return ..; }` | Delete the `else`, dedent: `return default;` as the last line. |
| Keeping a nest "because the hotfix diff should be minimal" | Guard-inverting the one function you touch *is* minimal. Adding a 4th nesting level is the churn. |
| `let mut result: Option<T> = None;` at the top of a branching function | That variable exists only to escape the nest. Return from the arms directly. |
| Guarding with `if ok { <20 lines> }` | Invert: `if !ok { return; }`/`continue;`, then the 20 lines dedent. |
| `if let Some(x) = opt { <rest of function> } else { return None; }` | `let Some(x) = opt else { return None; };` flattens it in place. |
| Manual `if opt.is_none() { return None; } let x = opt.unwrap();` | `let x = opt?;` — that's what `?` is for. |

## Red Flags — STOP

- About to add a brace and indent the function body under a condition → invert the condition and exit instead.
- About to write `else { .. }` under a branch whose last statement is `return`/`continue`/`break` → delete it, dedent.
- About to declare `let mut result = None;` before an `if` → return from the branches.
- Editing a function and your cursor is at nesting depth ≥ 3 → flatten that function to guards before adding your change.
- About to write `if let Some(x) = opt { <everything else> }` → check whether `let Some(x) = opt else { return ..; };` says the same thing flat.

Clippy flags the mechanical part: `clippy::redundant_else` (an `else` after a diverging `if` branch), `clippy::manual_let_else` (an `if let`/`else` that should be a `let-else`), and `clippy::question_mark` (a manual `is_none`/`return None` that `?` replaces). `clippy::needless_bool` catches the related `if c { true } else { false }` case. None of these catch a guard `return` at the top of a function being written *as* a guard — that structuring is this skill's job, not the linter's.
