# Changelog

All notable changes to Swatches are documented in this file.

## 0.1.0 - Unreleased

- Define and strictly parse version-1 TOML themes with six semantic RGB colors
  and a font family.
- Resolve application defaults, an optional complete theme, and explicit
  overrides in a deterministic order.
- Provide source-aware loading errors and deterministic theme path resolution.
- Publish a renderer-independent API with no daemon, watcher, CLI, rendering
  dependency, or bundled application adapters.
