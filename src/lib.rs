//! Shared appearance specifications, with no renderer or global configuration.
//!
//! ```
//! use blueprint_theme::{Theme, AppearancePatch};
//! let theme = Theme::parse(include_str!("../themes/blueprint.toml"))?;
//! let appearance = theme.appearance().with_overrides(&AppearancePatch::default());
//! assert_eq!(appearance.background.channels(), [16, 37, 63]);
//! # Ok::<(), blueprint_theme::Error>(())
//! ```

use serde::{Deserialize, Deserializer};
use std::{
    fmt, fs,
    path::{Path, PathBuf},
    str::FromStr,
};

/// An opaque, sRGB color. Alpha is owned by the application.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rgb([u8; 3]);

impl Rgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self([r, g, b])
    }
    pub const fn channels(self) -> [u8; 3] {
        self.0
    }
}

impl FromStr for Rgb {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let bytes = value.as_bytes();
        if bytes.len() != 7 || bytes[0] != b'#' || !bytes[1..].iter().all(u8::is_ascii_hexdigit) {
            return Err("expected an RGB color in #RRGGBB format");
        }
        let nibble = |b: u8| match b {
            b'0'..=b'9' => b - b'0',
            _ => b.to_ascii_lowercase() - b'a' + 10,
        };
        Ok(Self::new(
            nibble(bytes[1]) * 16 + nibble(bytes[2]),
            nibble(bytes[3]) * 16 + nibble(bytes[4]),
            nibble(bytes[5]) * 16 + nibble(bytes[6]),
        ))
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.0[0], self.0[1], self.0[2])
    }
}

impl<'de> Deserialize<'de> for Rgb {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        String::deserialize(d)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}

/// A nonempty family name; resolution to an installed font belongs to the app.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FontFamily(String);

impl FontFamily {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for FontFamily {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.trim().is_empty() || value.chars().any(char::is_control) {
            return Err("font family must be nonempty and contain no control characters");
        }
        Ok(Self(value.trim().to_owned()))
    }
}

impl<'de> Deserialize<'de> for FontFamily {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        String::deserialize(d)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Colors {
    pub background: Rgb,
    pub foreground: Rgb,
    pub accent: Rgb,
    pub muted: Rgb,
    pub selection_background: Rgb,
    pub selection_foreground: Rgb,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Font {
    pub family: FontFamily,
}

/// A complete v1 document; incomplete themes are rejected rather than invented.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Theme {
    colors: Colors,
    font: Font,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Document {
    version: u32,
    colors: Colors,
    font: Font,
}

impl Theme {
    pub fn parse(text: &str) -> Result<Self, Error> {
        let doc: Document = toml::from_str(text).map_err(Error::parse)?;
        if doc.version != 1 {
            return Err(Error::invalid(format!(
                "unsupported version {}; expected version = 1",
                doc.version
            )));
        }
        Ok(Self {
            colors: doc.colors,
            font: doc.font,
        })
    }
    /// Read once; callers choose whether to retain a previously loaded theme.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref();
        let text = fs::read_to_string(path).map_err(|source| Error {
            path: Some(path.to_owned()),
            kind: ErrorKind::Io(source),
        })?;
        Self::parse(&text).map_err(|mut error| {
            error.path = Some(path.to_owned());
            error
        })
    }
    pub fn colors(&self) -> &Colors {
        &self.colors
    }
    pub fn font(&self) -> &Font {
        &self.font
    }
    pub fn appearance(&self) -> Appearance {
        Appearance {
            background: self.colors.background,
            foreground: self.colors.foreground,
            accent: self.colors.accent,
            muted: self.colors.muted,
            selection_background: self.colors.selection_background,
            selection_foreground: self.colors.selection_foreground,
            font_family: self.font.family.clone(),
        }
    }
}

/// Renderer-independent resolved values. Apps provide their own defaults.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Appearance {
    pub background: Rgb,
    pub foreground: Rgb,
    pub accent: Rgb,
    pub muted: Rgb,
    pub selection_background: Rgb,
    pub selection_foreground: Rgb,
    pub font_family: FontFamily,
}

/// Explicit app values only. Do not default-fill app config before mapping it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AppearancePatch {
    pub background: Option<Rgb>,
    pub foreground: Option<Rgb>,
    pub accent: Option<Rgb>,
    pub muted: Option<Rgb>,
    pub selection_background: Option<Rgb>,
    pub selection_foreground: Option<Rgb>,
    pub font_family: Option<FontFamily>,
}

impl Appearance {
    pub fn with_overrides(mut self, patch: &AppearancePatch) -> Self {
        macro_rules! apply { ($($field:ident),+) => { $(if let Some(value) = patch.$field { self.$field = value; })+ }; }
        apply!(
            background,
            foreground,
            accent,
            muted,
            selection_background,
            selection_foreground
        );
        if let Some(family) = &patch.font_family {
            self.font_family = family.clone();
        }
        self
    }
}

/// App defaults → optional complete shared theme → explicit app fields.
pub fn resolve(
    defaults: &Appearance,
    theme: Option<&Theme>,
    overrides: &AppearancePatch,
) -> Appearance {
    theme
        .map(Theme::appearance)
        .unwrap_or_else(|| defaults.clone())
        .with_overrides(overrides)
}

/// Resolve a configured path without consulting the process environment.
///
/// `config_dir` must be absolute. Relative paths (including `..`) are relative
/// to it. `~/` requires an absolute `home`. No variable or `~user` expansion.
/// The result is not canonicalized: symlink identity is preserved for watchers.
pub fn resolve_theme_path(
    value: &str,
    config_dir: &Path,
    home: Option<&Path>,
) -> Result<PathBuf, Error> {
    if value.trim().is_empty() || value.chars().any(char::is_control) {
        return Err(Error::invalid(
            "appearance.theme_file must be a nonempty path without control characters",
        ));
    }
    if !config_dir.is_absolute() {
        return Err(Error::invalid("config_dir must be absolute"));
    }
    if let Some(rest) = value.strip_prefix("~/") {
        let home = home
            .filter(|p| p.is_absolute())
            .ok_or_else(|| Error::invalid("~/ requires an absolute home directory"))?;
        if rest.starts_with('/') {
            return Err(Error::invalid("use exactly one slash after ~"));
        }
        return Ok(home.join(rest));
    }
    if value.starts_with('~') {
        return Err(Error::invalid("only ~/ home expansion is supported"));
    }
    let path = Path::new(value);
    Ok(if path.is_absolute() {
        path.to_owned()
    } else {
        config_dir.join(path)
    })
}

#[derive(Debug)]
pub struct Error {
    path: Option<PathBuf>,
    kind: ErrorKind,
}
#[derive(Debug)]
enum ErrorKind {
    Io(std::io::Error),
    Parse(toml::de::Error),
    Invalid(String),
}
impl Error {
    fn parse(error: toml::de::Error) -> Self {
        Self {
            path: None,
            kind: ErrorKind::Parse(error),
        }
    }
    fn invalid(message: impl Into<String>) -> Self {
        Self {
            path: None,
            kind: ErrorKind::Invalid(message.into()),
        }
    }
    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(path) = &self.path {
            write!(f, "{}: ", path.display())?;
        }
        match &self.kind {
            ErrorKind::Io(e) => e.fmt(f),
            ErrorKind::Parse(e) => e.fmt(f),
            ErrorKind::Invalid(s) => f.write_str(s),
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ErrorKind::Io(e) => Some(e),
            ErrorKind::Parse(e) => Some(e),
            ErrorKind::Invalid(_) => None,
        }
    }
}
