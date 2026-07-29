---
name: three-word-naming
description: Use when naming or renaming a Rust function, method, struct, enum, trait, or module in this photomosaic crate — especially when a name is heading past three words (find_and_use_best_tile_with_position, process_tile_no_aspect_filter), contains and/or/with/if_missing (load_or_new), or describes several tile-selection or optimization steps at once.
---

# Three-Word Naming

## Overview

Functions, methods, structs, enums, traits, and modules get **at most three words**. A name that needs a fourth word is not a naming problem — it is a scope problem: the code is doing more than one thing, or it is missing the namespace that should carry part of the name. Fix the scope, and the short name falls out.

**Never fix a too-long name by abbreviating, dropping vowels, or fusing words. Shorten the *responsibility*, not the spelling.**

## Counting

- Split snake_case on `_`, CamelCase on capitals: `calculate_average_lab` = 3, `AdjacencyPenaltyCalculator` = 3, `find_and_use_best_tile_with_position` = 7 ❌.
- **Every token counts** — suffixes, negations, prepositions all count: `process_tile_no_aspect_filter` = 5 ❌ (process, tile, no, aspect, filter).
- An acronym or digit group is one word (`lab` = 1, `2000` = 1) — but `calculate_delta_e_2000` still lands at 4 (calculate, delta, e, 2000) ❌.
- Idiomatic constructors (`new`, `from_x`, `with_x`) count normally but rarely trip the limit: `SimilarityDatabase::new` is fine.
- **Scope:** functions, methods, structs/enums/traits, modules. Exempt: `#[test]` functions (names narrate behavior — `test_similarity_database_load_or_new` is fine even though the production `load_or_new` it tests is not), constants, local variables, trait method implementations (the trait fixed the name), and derive/trait-mandated names (`fmt`, `from`, `try_from`).

## Over three words? Two remedies

**1. Raise the abstraction — split and compose.** A 4+ word name is usually a step list. Each step gets its own ≤3-word function; the composition point keeps a short name describing the *outcome*, not the steps.

**2. Move a word into a namespace.** If the extra words are a noun phrase repeated across the module, they are the module's (or type's) name, not each function's. Callers read `tile_selector::find_best(...)` or `tracker.tick()` — the context words are written once.

```rust
// ❌ 7 words — find + use(mark-used) + best + tile + with + position crammed into one name
fn find_and_use_best_tile_with_position(
    &mut self, target_lab: &Lab, x: usize, y: usize,
) -> Option<Arc<Tile>> { .. }

// ✅ remedy 2 — module tile_selector carries the noun phrase, position drops to a plain arg
mod tile_selector {
    fn find_best(state: &State, target_lab: &Lab, x: usize, y: usize) -> Option<Arc<Tile>> { .. } // tile_selector::find_best(...)
    fn mark_used(state: &mut State, tile: &Tile) { .. }                                           // tile_selector::mark_used(...)
}

// ✅ remedy 1 — or split by responsibility where a module isn't warranted
fn find_best_tile(&self, target_lab: &Lab, x: usize, y: usize) -> Option<Arc<Tile>> { .. }
fn mark_tile_used(&mut self, tile: &Tile) { .. }
```

The same move works with a struct as the namespace: `TileCache::get()`, `GridVisualizer::complete_tile()`, or a method on the value itself: `tracker.tick()`.

Rust convention pushes this further: **don't repeat the module or type name inside the function** — `TileCache::get_tile` ❌ → `TileCache::get` ✅ (this repo already gets it right). That repetition is remedy 2 done halfway; clippy's `module_name_repetitions` lint catches the type-name version of the same mistake.

## Name smells that predict a 4th word

| Smell in the name | What it reveals | Fix |
|---|---|---|
| `_and_` (`find_and_use_best_tile_with_position`) | two responsibilities fused: finding a tile and marking it used | split: `find_best_tile` + `mark_tile_used`, compose at the caller |
| `_with_` tail on the same name (`..._with_position`) | a parameter leaking into the name | position is already an argument — drop it from the name: `find_best_tile(x, y)` |
| negated variant suffix (`process_tile_no_aspect_filter`) sitting next to `process_tile`) | a guard/filter fused into a whole separate function instead of a parameter | fold it in: `process_tile(path, aspect_filter: Option<AspectTolerance>)` |
| `_or_` fused branches (`SimilarityDatabase::load_or_new`) | "load, else construct new" is two responsibilities wearing one verb | keep `load_from_file` fallible, let the caller do `.unwrap_or_else(Self::new)` — or rename to say what it actually promises |
| repeated noun phrase across functions (`process_tile`, `process_tile_no_aspect_filter`) | a missing parameter/variant, not a new function | fold the variant into the base function via a parameter |

## Common mistakes

| Mistake | Fix |
|---|---|
| Abbreviating to sneak under the limit (`find_use_tile`) | Still multiple responsibilities. Split or namespace — never compress spelling. |
| Vague 1-worder to dodge the limit (`process`, `handle`, `do_it`) | Under-specific is as bad as over-long. Three *precise* words beat one vague one. |
| Counting digit/formula-name groups as "free" | They count. `calculate_delta_e_2000` → namespace it: `color_diff::delta_e_2000` (the module absorbs "calculate"). |
| Renaming without re-scoping (`find_and_use_best_tile_with_position` → `find_best_tile_with_position`) | Now the name lies — it no longer promises the tile gets marked used. Split first, then name what's left. |
| Applying the limit to tests, constants, or trait impls | Test names narrate behavior (`test_similarity_database_load_or_new`); constants encode ranges; trait-mandated names (`fmt`, `from`, `try_from`) aren't yours to shorten. All exempt. |

## Red Flags — STOP

- The name has a 4th `_` segment (or 4th capitalized word in CamelCase) → stop, split or namespace. (`find_and_use_best_tile_with_position` has seven.)
- You wrote `_and_`, `_with_`, `_or_` inside a function name → the scope is too broad (`load_or_new` is doing two things).
- You're about to abbreviate a word to fit → wrong axis; shrink the responsibility.
- Two or more functions share a noun prefix and differ only by a filter/variant suffix (`process_tile` vs `process_tile_no_aspect_filter`) → fold the variant into a parameter instead of a new function.
- You typed the module or type name again inside the function (`TileCache::get_tile`) → clippy's `module_name_repetitions` is about to fire; drop the repeat.
