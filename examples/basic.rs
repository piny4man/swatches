use std::{env, path::PathBuf};
use swatches::{resolve, Appearance, AppearancePatch, FontFamily, Rgb, Theme};

fn main() -> Result<(), swatches::Error> {
    let path = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("theme.toml"));

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
    let appearance = resolve(&defaults, Some(&theme), &AppearancePatch::default());
    println!(
        "background={} foreground={} accent={} font={}",
        appearance.background,
        appearance.foreground,
        appearance.accent,
        appearance.font_family.as_str()
    );
    Ok(())
}
