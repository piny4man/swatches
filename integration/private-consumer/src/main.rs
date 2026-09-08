use blueprint::{resolve, AppearancePatch, Rgb, Theme};

fn main() -> Result<(), blueprint::Error> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../themes/blueprint.toml");
    let theme = Theme::load(path)?;
    // In an adapter, these defaults come from the host application.
    let mut defaults = theme.appearance();
    defaults.accent = Rgb::new(1, 2, 3);
    let standalone = resolve(&defaults, None, &AppearancePatch::default());
    assert_eq!(standalone, defaults);
    let themed = resolve(&defaults, Some(&theme), &AppearancePatch::default());
    assert_eq!(themed.accent, theme.colors().accent);
    let overridden = resolve(
        &defaults,
        Some(&theme),
        &AppearancePatch {
            accent: Some(defaults.accent),
            ..Default::default()
        },
    );
    assert_eq!(overridden.accent, defaults.accent);
    println!(
        "private path dependency OK; shared accent {}; explicit accent {}; standalone defaults preserved",
        themed.accent, overridden.accent
    );
    Ok(())
}
