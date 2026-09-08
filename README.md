# Blueprint

Shared appearance specifications for independent desktop tools.

**Private foundation, v0.1.0.** Rust package: `blueprint-theme`. Publishing is disabled in Cargo.toml. No registry release has been made.

Blueprint defines six semantic RGB colors and a font family in one versioned TOML file. Tablero, Hyprburst and Crabture will consume these through application-specific adapters. Those integrations are not included yet.

## Theme

See [themes/blueprint.toml](themes/blueprint.toml) for a blue technical-drawing-inspired example. It is an optional example, not a forced palette.

```toml
version = 1

[colors]
background = "#10253F"
foreground = "#EAF3FF"
accent = "#80D4FF"
muted = "#A4B8CF"
selection_background = "#244A70"
selection_foreground = "#FFFFFF"

[font]
family = "JetBrainsMono Nerd Font Mono"
```

Every field is required. Colors are `#RRGGBB`, case insensitive. Unknown keys, duplicate keys, unsupported versions, malformed colors and empty/control-containing font names are errors. Font family is trimmed. Blueprint validates the name, not font installation or glyph coverage.

No colors are derived automatically. Selection foreground/background are explicit so all adapters receive the same design choice. Contrast must be reviewed when designing a theme.

## Rust API

Use a local path dependency while this project is private and unpublished:

```toml
[dependencies]
blueprint = { package = "blueprint-theme", path = "../blueprint" }
```

```rust,no_run
use blueprint::{Theme, AppearancePatch};

fn main() -> Result<(), blueprint::Error> {
    let theme = Theme::load("/home/me/.config/blueprint/theme.toml")?;
    let app_overrides = AppearancePatch::default();
    let appearance = theme.appearance().with_overrides(&app_overrides);
    let [r, g, b] = appearance.accent.channels();
    Ok(())
}
```

`resolve(&app_defaults, optional_theme, &explicit_overrides)` preserves application defaults when no theme is selected. A complete theme replaces the seven shared roles; explicit app values are applied last, including values equal to the old defaults. There is intentionally no global Default appearance.

Apps whose native appearance is richer or has unspecified font defaults may instead consume `Theme::colors()` and `Theme::font()` directly, applying shared fallbacks before resolving their own raw optional fields. Never coerce an app's absent system font into a made-up family merely to use the helper.

## Paths and loading

Proposed app config (adapters must implement this):

```toml
[appearance]
theme_file = "../blueprint/theme.toml"
```

`resolve_theme_path` takes the configured string, absolute app configuration directory and optional absolute home directory. Relative paths resolve against that config directory. `~/` is supported; variables and `~user` are not expanded. Paths are not canonicalized, so watches can follow the configured symlink path. This helper is not a filesystem sandbox.

`Theme::load` is read-only and returns errors containing the path. TOML diagnostics include field/context and source location where available. Missing files are errors; opt-out is represented by not calling load. The library does not silently choose defaults or modify application configuration.

Tablero should keep the previous resolved config when reload fails. Hyprburst and Crabture should load once at opening initially. Their adapters choose startup fallback behavior and diagnostics. No file watcher, daemon, environment mutation, font discovery or renderer is included.

## Adapter plan

| Token | Tablero | Hyprburst | Crabture |
|---|---|---|---|
| background/foreground | Global theme fallback | Default GUI colors | Panel/text colors, retain role alpha |
| accent | Widget emphasis fallback | Prompt/banner | Active controls and selection highlight |
| muted | Future secondary-text role | Empty-state text | Secondary toolbar labels |
| selection colors | Where a native selected surface exists | Selected row | Applicable active/selected controls |
| font family | Existing family setting | New family resolver after explicit path/environment | New resolver with bundled fallback |

An adapter may leave a role unused where no corresponding surface exists. Preserve explicit native overrides, Tablero monitor/widget specificity, and each app's geometry, alpha and font sizes. Do not apply shared opacity to captured screenshot pixels.

Crabture layout measurement, paint and hit-testing must use the same resolved font. Hyprburst's Rio parent and TUI child must resolve the same theme; terminal fallback fonts remain terminal-owned. Tablero must watch both app and theme paths.

## Development

See [Private integration strategy](docs/private-integration.md) for the private
patch workflow and the independent consumer that proves local path builds.

```sh
cargo fmt --check
cargo test --locked
cargo clippy --all-targets --locked -- -D warnings
```

See [docs/review.md](docs/review.md) for scope and review notes. Application integration and laptop measurements remain future work. No distribution license has been chosen while this repository is private.
