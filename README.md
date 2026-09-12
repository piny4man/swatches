# Swatches

Swatches is a small, renderer-independent Rust library for sharing appearance
themes between unrelated applications. It parses a strict TOML schema and
resolves values with explicit precedence; rendering, global configuration, and
reload orchestration remain application concerns.

## Installation

Add the library dependency, not a binary:

```sh
cargo add swatches
```

Swatches requires Rust 1.85 or newer.

## Theme format

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
family = "Example Sans"
```

Every field is required. Colors use case-insensitive `#RRGGBB` notation.
Unknown or duplicate keys, unsupported versions, malformed colors, and empty
or control-containing font names are errors. Font family whitespace is trimmed.
Swatches validates a family name but does not discover installed fonts or check
glyph coverage.

## Usage

```rust
use swatches::{resolve, Appearance, AppearancePatch, FontFamily, Rgb, Theme};

fn load_appearance(path: &std::path::Path) -> Result<Appearance, swatches::Error> {
    let defaults = Appearance {
        background: Rgb::new(255, 255, 255),
        foreground: Rgb::new(20, 20, 20),
        accent: Rgb::new(0, 100, 220),
        muted: Rgb::new(100, 100, 100),
        selection_background: Rgb::new(0, 100, 220),
        selection_foreground: Rgb::new(255, 255, 255),
        font_family: "Example Sans".parse::<FontFamily>().expect("valid default"),
    };

    let theme = Theme::load(path)?;
    Ok(resolve(&defaults, Some(&theme), &AppearancePatch::default()))
}
```

The complete example in [`examples/basic.rs`](examples/basic.rs) accepts a
theme path and prints values suitable for passing to any renderer.

Resolution order is deterministic:

1. Application-owned defaults are the base.
2. A selected complete theme replaces all shared roles.
3. Explicit application fields replace matching theme values.

There is deliberately no global `Default` appearance. Applications with richer
models can consume `Theme::colors()` and `Theme::font()` directly instead.
Application adapters, configuration, and documentation belong in those
applications, not in this repository.

## Application responsibilities

- Error handling: `Theme::load` reports I/O and TOML errors and associates them
  with the source path. Applications decide whether startup fails, defaults are
  used, or a previously valid theme remains active.
- Reloading: Swatches reads only when called. File watching, debounce behavior,
  and reload policy belong to the application.
- Fonts: The schema stores a family, not a font size, weight, file, fallback
  chain, or installed-font resolution policy.
- Alpha and geometry: Colors are opaque sRGB values. Opacity, spacing, radius,
  dimensions, animation, and layout stay under application control.
- Paths: `resolve_theme_path` resolves absolute, configuration-relative, and
  `~/` paths without environment-variable expansion or canonicalization. It is
  a convenience helper, not a filesystem sandbox.

## Compatibility

The crate version and TOML schema version are independent. Swatches `0.1.0`
supports only `version = 1` documents and rejects other schema versions rather
than guessing. Before crate 1.0, Rust API changes may occur in minor releases;
schema changes still require an explicit document version and migration notes.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or the
[MIT license](LICENSE-MIT), at your option.
