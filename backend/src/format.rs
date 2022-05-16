use std::{
    borrow::Cow,
    error::Error,
    fmt::{self, Display, Formatter},
};
use url::Url;

#[derive(Debug)]
pub enum FormatError {
    InvalidFormat(String),
}

impl Error for FormatError {}

impl Display for FormatError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FormatError::InvalidFormat(s) => write!(f, "Invalid Format: {s}"),
        }
    }
}

pub enum ThemeFormat {
    Toml,
    Json,
    Xresources,
}

impl Default for ThemeFormat {
    fn default() -> Self {
        Self::Toml
    }
}

impl ThemeFormat {
    fn format_xresources(data: &toml::Value) -> String {
        format!(
            r#"
! Generated with theme-repo
! Theme: {}
*.foreground:  {}
*.background:  {}
*.cursorColor: {}
*.color0:      {}
*.color8:      {}
*.color1:      {}
*.color9:      {}
*.color2:      {}
*.color10:     {}
*.color3:      {}
*.color11:     {}
*.color4:      {}
*.color12:     {}
*.color5:      {}
*.color13:     {}
*.color6:      {}
*.color14:     {}
*.color7:      {}
*.color15:     {}
"#,
            data["name"].as_str().unwrap_or_default(),
            data["foreground"].as_str().unwrap_or_default(),
            data["background"].as_str().unwrap_or_default(),
            data["cursor"].as_str().unwrap_or_default(),
            data["color0"].as_str().unwrap_or_default(),
            data["color8"].as_str().unwrap_or_default(),
            data["color1"].as_str().unwrap_or_default(),
            data["color9"].as_str().unwrap_or_default(),
            data["color2"].as_str().unwrap_or_default(),
            data["color10"].as_str().unwrap_or_default(),
            data["color3"].as_str().unwrap_or_default(),
            data["color11"].as_str().unwrap_or_default(),
            data["color4"].as_str().unwrap_or_default(),
            data["color12"].as_str().unwrap_or_default(),
            data["color5"].as_str().unwrap_or_default(),
            data["color13"].as_str().unwrap_or_default(),
            data["color6"].as_str().unwrap_or_default(),
            data["color14"].as_str().unwrap_or_default(),
            data["color7"].as_str().unwrap_or_default(),
            data["color15"].as_str().unwrap_or_default(),
        )
    }

    pub fn run(&self, theme_object: &toml::Value) -> String {
        match self {
            ThemeFormat::Toml => toml::to_string(theme_object).unwrap(),
            ThemeFormat::Json => serde_json::to_string(theme_object).unwrap(),
            ThemeFormat::Xresources => Self::format_xresources(theme_object),
        }
    }
}

impl TryFrom<&str> for ThemeFormat {
    type Error = FormatError;

    fn try_from(format: &str) -> Result<Self, Self::Error> {
        match format {
            "toml" => Ok(ThemeFormat::Toml),
            "json" => Ok(ThemeFormat::Json),
            "xresources" => Ok(ThemeFormat::Xresources),
            _ => Err(FormatError::InvalidFormat(format.to_string())),
        }
    }
}

impl TryFrom<&Url> for ThemeFormat {
    type Error = FormatError;

    fn try_from(url: &Url) -> Result<Self, Self::Error> {
        let mut query_pairs = url.query_pairs();
        if let Some(pair) = query_pairs.find(|pair| pair.0 == Cow::Borrowed("format")) {
            ThemeFormat::try_from(pair.1.as_ref())
        } else {
            Ok(ThemeFormat::default())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_format() {
        let format = "invalid-format";

        assert!(matches!(
            ThemeFormat::try_from(format),
            Err(FormatError::InvalidFormat(_))
        ));
    }

    #[test]
    fn invalid_format_in_url() {
        let url = Url::parse("https://example.com?format=invalid-format").unwrap();

        assert!(matches!(
            ThemeFormat::try_from(&url),
            Err(FormatError::InvalidFormat(_))
        ));
    }
}
