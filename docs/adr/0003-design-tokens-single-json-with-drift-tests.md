---
status: accepted
---

# Design tokens live in one JSON file, mirrored by hand and guarded by drift tests

The egui theme (`desktop/gw-theme`) and the Compose theme (`mobile/`) must use the same colours, type and sizes as the design canvas, yet the two sides share no code (ADR 0001). `docs/design/tokens.json` is the source of truth. Each side keeps a hand-maintained mirror (`tokens.rs`, `Tokens.kt`), and a unit test on each side reads the JSON and fails when a value or a key differs. To change a token, edit the JSON, then both mirrors; CI rejects a partial change.

The JSON is flat (`"color.text_2": "#9AA3B5"`), not nested like the W3C design-tokens format, so the Kotlin test parses it with one regex and needs no JSON library.

The font files exist once, in `mobile/app/src/main/res/font/` (where Android requires them), and the Rust crate embeds them from there with `include_bytes!`.

## Considered Options

- **Code generation** (`build.rs` plus a Gradle task): a true single source, but two build steps to maintain for about 120 values that rarely change. Revisit if the token count or the number of platforms grows.
- **Duplicate by hand with a table in the docs**: simplest, but nothing stops the two sides from drifting apart silently.
