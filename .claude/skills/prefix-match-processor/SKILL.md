---
name: prefix-match-processor
description: Use when branching on a file's extension or another string-token family with a growing match!/if-chain — e.g. supported material-image extensions "png"|"jpg"|"jpeg"|"webp" — especially when the same token-family test is duplicated across call sites (src/main.rs vs src/gui/app_full.rs) and has already drifted out of sync, or when adding several new format tokens to an existing check.
---

# Prefix-Match Processor

## Overview

A growing if/`matches!` (or `starts_with`/`strip_prefix`) chain that tests which string-token family an input belongs to — device URI schemes, CLI flags, or, in this repo, supported material-image file extensions — is a **processor registry** waiting to happen (a.k.a. the plugin / strategy registry, chain-of-responsibility). Replace the chain with a trait, a `Vec<Box<dyn Processor>>` of self-contained implementers, and a runner that returns the first match. Adding a family becomes "write a struct, register it"; the dispatcher never grows, and — critically — there is only **one** dispatcher, so it can't be copy-pasted into a second call site and drift.

The input is typically a tagged value — model it with `precise-type-modeling`. Composing the `Result` inside `run` (`.map`, `.map_err`) follows `chaining-result-combinators`.

**A caveat on exact-equality tokens (the extension case):** unlike URI prefixes, extensions are compared for exact equality after lowercasing (`matches!(ext, "png" | "jpg" | ...)`), not `starts_with`. If every branch really is a one-line "is this token in the set" boolean with no per-family logic, a plain `HashMap<&str, Format>` (or even one shared `fn`) is a legitimate, simpler fix — reach for the trait + registry once per-family handling needs its own logic or its own test (a feature-gated format, format-specific validation), or once the test has already been duplicated and drifted, as below.

## When to extract — the threshold

| Signal | Keep inline `if`/`match` | Enum + exhaustive `match` | Processor registry (`Vec<Box<dyn Processor>>`) |
|---|---|---|---|
| Family count | ≤ ~5, one-line mapping each | Known, fixed set — no new family expected | > ~5 families, **or** growing over time, **or** already duplicated across call sites |
| Branch complexity | trivial | multi-line logic OK, all variants live in one file | multi-line logic owned/tested per family |
| Extensibility | n/a | adding a variant means editing the `match` — the compiler forces exhaustiveness, which is a feature when the set really is closed | adding a family = new struct + one registration line, no central edit, no second copy to forget |

The enum route is `branching-modeled-state-with-match`'s territory: when every family is fixed at compile time, an exhaustive `match` is simpler and compiler-checked. The registry earns its place once families are added independently over time, need isolated tests, or — as below — the same test already got copy-pasted into more than one place and drifted.

## Before (a duplicated, drifted token-family test) ❌

The same "is this a supported material image" check exists twice — once for the CLI loader, once for the GUI loader — and they have already fallen out of sync:

```rust
// src/main.rs:181 (load_tiles)
path.extension()
    .and_then(|ext| ext.to_str())
    .map(|ext| matches!(ext.to_lowercase().as_str(), "png" | "jpg" | "jpeg" | "webp"))
    .unwrap_or(false)

// src/gui/app_full.rs:1653 — the same test, copied by hand, missing "webp"
path.extension()
    .and_then(|ext| ext.to_str())
    .map(|ext| matches!(ext.to_lowercase().as_str(), "png" | "jpg" | "jpeg"))
    .unwrap_or(false)
```

Nobody decided the GUI should reject WebP materials — the two lists just drifted apart because each call site owns its own copy. Add AVIF or TIFF next and there are now three lists to remember to edit in lockstep.

## After (the canonical shape) ✅

**The processor contract** — `None` means "not mine, try the next processor"; `Some(Err(_))` means "mine, but rejected" — the two must never be conflated:

```rust
pub trait MaterialFormatProcessor {
    /// `None`      -> this processor does not claim `ext`.
    /// `Some(Ok)`  -> claimed, accepted as this format.
    /// `Some(Err)` -> claimed, but this format is currently rejected.
    fn run(&self, ext: &str) -> Option<Result<MaterialFormat, FormatError>>;
}
```

**One self-contained processor per family** — the token test lives inside `run`:

```rust
struct WebpProcessor;

impl MaterialFormatProcessor for WebpProcessor {
    fn run(&self, ext: &str) -> Option<Result<MaterialFormat, FormatError>> {
        if ext != "webp" {
            return None; // not mine
        }
        Some(Ok(MaterialFormat::Webp))
    }
}

struct JpegProcessor; // claims both "jpg" and "jpeg" — one processor, one family

impl MaterialFormatProcessor for JpegProcessor {
    fn run(&self, ext: &str) -> Option<Result<MaterialFormat, FormatError>> {
        match ext {
            "jpg" | "jpeg" => Some(Ok(MaterialFormat::Jpeg)),
            _ => None,
        }
    }
}
```

**The registry + runner — first `Some` wins** (the only place that lists families):

```rust
pub struct FormatRegistry {
    processors: Vec<Box<dyn MaterialFormatProcessor>>,
}

impl FormatRegistry {
    pub fn dispatch(&self, ext: &str) -> Result<MaterialFormat, FormatError> {
        let ext = ext.to_lowercase();
        for processor in &self.processors {
            if let Some(result) = processor.run(&ext) {
                return result;
            }
        }
        Err(FormatError::Unsupported(ext))
    }
}

let registry = FormatRegistry {
    processors: vec![
        Box::new(PngProcessor),
        Box::new(JpegProcessor),
        Box::new(WebpProcessor),
    ],
};
```

Both `src/main.rs`'s `load_tiles` and `src/gui/app_full.rs`'s material-scan filter call `registry.dispatch(ext)` instead of each keeping its own `matches!` list — one table, so the CLI and GUI can no longer drift apart. Adding a format: write `AvifProcessor` (returning `Some(Err(FormatError::FeatureDisabled))` if the build lacks the AVIF feature), push it into the `vec!`. Test it alone: `WebpProcessor.run("webp")` — no registry, no other formats, no second call site to remember.

## Order matters

Extension tokens are matched by exact equality, not `starts_with`, so — unlike overlapping prefixes (`thumb_` vs `thumb_small_`) — one family can't accidentally shadow another; ordering the `vec!` is mostly cosmetic here. The rule still applies to any family whose test is fuzzier than equality — a MIME sniff on file-header bytes, a filename convention like `thumb_*` — register the more specific test before the broader one.

## Common mistakes

| Mistake | Fix |
|---|---|
| Processor panics, or returns `None` for both "not mine" and "mine but invalid" | Non-match MUST be `None`; claimed-but-rejected MUST be `Some(Err(_))`. Conflating them lets the runner silently fall through to the wrong processor. |
| Re-implementing the token-family test ad hoc at each call site (as `src/main.rs` and `src/gui/app_full.rs` already have, out of sync) | One shared `FormatRegistry::dispatch`; both CLI and GUI call it, so a fix or a new format lands in exactly one place. |
| Format-specific validation placed in the dispatch loop | Keep the equality test + any per-format validation **inside** `run`; the registry loop stays dumb. |
| Reaching for `Vec<Box<dyn Processor>>` when every branch is a trivial one-line equality check | If there's no per-family logic to own or test, a `HashMap<&str, Format>` (or one shared `fn`) is simpler and equally correct for exact-token families. |
| Extracting 2-3 one-liners into processors | Premature — keep the inline check, or reach for an enum `match`, until the registry threshold is met. |

## Red Flags — STOP

- A `matches!`/`starts_with` chain testing a string-token family that keeps gaining tokens, or whose branches are gaining their own multi-line logic → extract to a processor registry (or an enum `match` if the family set is closed and lives in one place).
- The same token-family test duplicated across more than one call site (the CLI's `src/main.rs` and the GUI's `src/gui/app_full.rs` extension checks already have) → one shared dispatcher both call, never two independently-maintained lists.
- About to invent a new dispatch shape (`HashMap<token, fn>` for a fuzzy/prefix family, `Vec<(&str, fn)>`, a `match` over `starts_with` guards) → use the `Processor` trait + `Vec<Box<dyn Processor>>` + first-`Some` runner instead — unless every branch really is a trivial equality check with no per-family logic, in which case a plain `HashMap` is fine.
- A processor returning `None` to mean "claimed but failed" → it must be `Some(Err(_))`; `None` is reserved for "not mine".
- Per-family logic that can only be tested by driving the whole registry → each processor must be unit-testable on its own.
