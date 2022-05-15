use serde_json::Value;
use std::borrow::Cow;
use url::Url;

pub enum ThemeFormatter {
    Json,
    Xresources,
}

impl Default for ThemeFormatter {
    fn default() -> Self {
        Self::Json
    }
}

impl ThemeFormatter {
    fn from_str(value: &str) -> Option<Self> {
        match value {
            "json" => Some(ThemeFormatter::Json),
            "xresources" => Some(ThemeFormatter::Xresources),
            _ => None,
        }
    }

    fn format_xresources(data: &Value) -> String {
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

    pub fn run(&self, theme_object: &Value) -> String {
        match self {
            ThemeFormatter::Json => serde_json::to_string(theme_object).unwrap(),
            ThemeFormatter::Xresources => Self::format_xresources(theme_object),
        }
    }
}

impl From<&Url> for ThemeFormatter {
    fn from(url: &Url) -> Self {
        let mut query_pairs = url.query_pairs();
        if let Some(pair) = query_pairs.find(|pair| pair.0 == Cow::Borrowed("format")) {
            ThemeFormatter::from_str(&pair.1)
        } else {
            None
        }
        .unwrap_or_default()
    }
}
