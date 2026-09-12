use std::{fs, path::Path};
use swatches::{resolve, resolve_theme_path, AppearancePatch, FontFamily, Rgb, Theme};

const EXAMPLE: &str = include_str!("../themes/swatches.toml");

#[test]
fn example_has_explicit_semantic_roles() {
    let theme = Theme::parse(EXAMPLE).unwrap();
    assert_eq!(theme.colors().background.channels(), [16, 37, 63]);
    assert_eq!(theme.colors().selection_foreground.to_string(), "#FFFFFF");
    assert_eq!(theme.font().family.as_str(), "JetBrainsMono Nerd Font Mono");
}

#[test]
fn rejects_incomplete_unknown_duplicate_and_future_documents() {
    for text in [
        String::new(),
        EXAMPLE.replace("version = 1", "version = 2"),
        EXAMPLE.replace("version = 1", "version = 1\nextra = true"),
        EXAMPLE.replace("[colors]", "[colors]\nextra = 0"),
        EXAMPLE.replace("[font]", "[font]\nsize = 16"),
        EXAMPLE.replace("accent = \"#80D4FF\"", ""),
        EXAMPLE.replace("version = 1", "version = 1\nversion = 1"),
        EXAMPLE.replace("version = 1", "version = -1"),
    ] {
        assert!(Theme::parse(&text).is_err(), "accepted {text}");
    }
}

#[test]
fn strict_rgb_handles_unicode_without_panicking() {
    for color in [
        "",
        "#fff",
        "112233",
        "#11223344",
        "red",
        "#GG1122",
        "#é1234",
        "#１２３",
        " #123456",
    ] {
        assert!(color.parse::<Rgb>().is_err(), "accepted {color}");
    }
    assert_eq!("#aAbBcC".parse::<Rgb>().unwrap().to_string(), "#AABBCC");
}

#[test]
fn invalid_color_reports_field_and_source_path() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("theme.toml");
    fs::write(&path, EXAMPLE.replace("#80D4FF", "#bad")).unwrap();
    let error = Theme::load(&path).unwrap_err();
    assert_eq!(error.path(), Some(path.as_path()));
    let message = error.to_string();
    assert!(message.contains("accent"), "{message}");
    assert!(message.contains("theme.toml"));
}

#[test]
fn font_family_is_validated_and_normalized() {
    assert_eq!(
        "  Example Mono  ".parse::<FontFamily>().unwrap().as_str(),
        "Example Mono"
    );
    for name in ["", "   ", "Mono\nOther", "Mono\0"] {
        assert!(name.parse::<FontFamily>().is_err());
    }
}

#[test]
fn no_theme_preserves_app_defaults_and_explicit_values_win() {
    let theme = Theme::parse(EXAMPLE).unwrap();
    let mut defaults = theme.appearance();
    defaults.accent = Rgb::new(1, 2, 3);
    assert_eq!(
        resolve(&defaults, None, &AppearancePatch::default()),
        defaults
    );
    assert_eq!(
        resolve(&defaults, Some(&theme), &AppearancePatch::default()),
        theme.appearance()
    );
    // Explicitly choosing the OLD default must still beat the shared theme.
    let overrides = AppearancePatch {
        accent: Some(defaults.accent),
        ..Default::default()
    };
    assert_eq!(
        resolve(&defaults, Some(&theme), &overrides).accent,
        defaults.accent
    );
    assert_eq!(
        resolve(&defaults, Some(&theme), &overrides).background,
        theme.colors().background
    );
}

#[test]
fn every_override_is_applied_independently() {
    let theme = Theme::parse(EXAMPLE).unwrap();
    let value = Rgb::new(0, 0, 0);
    let patch = AppearancePatch {
        background: Some(value),
        foreground: Some(value),
        accent: Some(value),
        muted: Some(value),
        selection_background: Some(value),
        selection_foreground: Some(value),
        font_family: Some("Other Mono".parse().unwrap()),
    };
    let a = theme.appearance().with_overrides(&patch);
    assert_eq!(
        [
            a.background,
            a.foreground,
            a.accent,
            a.muted,
            a.selection_background,
            a.selection_foreground
        ],
        [value; 6]
    );
    assert_eq!(a.font_family.as_str(), "Other Mono");
}

#[test]
fn paths_are_relative_to_config_not_cwd() {
    let config = Path::new("/home/test/.config/tablero");
    assert_eq!(
        resolve_theme_path("../swatches/theme.toml", config, None).unwrap(),
        config.join("../swatches/theme.toml")
    );
    assert_eq!(
        resolve_theme_path("/themes/a.toml", config, None).unwrap(),
        Path::new("/themes/a.toml")
    );
    assert_eq!(
        resolve_theme_path("~/themes/a.toml", config, Some(Path::new("/home/test"))).unwrap(),
        Path::new("/home/test/themes/a.toml")
    );
    for value in ["", "  ", "~someone/theme", "~", "x\0y"] {
        assert!(resolve_theme_path(value, config, None).is_err());
    }
    assert!(resolve_theme_path("~/a", config, None).is_err());
    assert!(resolve_theme_path("~//etc/file", config, Some(Path::new("/home/test"))).is_err());
    assert!(resolve_theme_path("a", Path::new("relative"), None).is_err());
    assert!(resolve_theme_path("~/a", config, Some(Path::new("relative"))).is_err());
}

#[test]
fn load_is_read_only_and_failed_reload_can_keep_previous_theme() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("theme.toml");
    assert!(Theme::load(&path).is_err());
    assert!(!path.exists());
    fs::write(&path, EXAMPLE).unwrap();
    let active = Theme::load(&path).unwrap();
    fs::write(&path, "partial edit").unwrap();
    assert!(Theme::load(&path).is_err());
    assert_eq!(active, Theme::parse(EXAMPLE).unwrap());
    fs::write(&path, EXAMPLE.replace("#80D4FF", "#FFAA00")).unwrap();
    assert_ne!(Theme::load(&path).unwrap(), active);
}
