use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use url::Url;

#[derive(Serialize, Deserialize)]
struct ThemeData {
    name: String,
    color0: String,
    color1: String,
    color2: String,
    color3: String,
    color4: String,
    color5: String,
    color6: String,
    color7: String,
    color8: String,
    color9: String,
    color10: String,
    color11: String,
    color12: String,
    color13: String,
    color14: String,
    color15: String,
    background: String,
    selection_foreground: String,
    cursor: String,
    foreground: String,
    selection_background: String,
}

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

    fn format_xresources(data: &ThemeData) -> String {
        format!(
            "! Generated with theme-repo
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
*.color15:     {}",
            data.name,
            data.foreground,
            data.background,
            data.cursor,
            data.color0,
            data.color8,
            data.color1,
            data.color9,
            data.color2,
            data.color10,
            data.color3,
            data.color11,
            data.color4,
            data.color12,
            data.color5,
            data.color13,
            data.color6,
            data.color14,
            data.color7,
            data.color15,
        )
    }

    pub fn run(&self, theme_object: &Value) -> String {
        let theme_object_string = theme_object.to_string();
        let data: ThemeData = serde_json::from_str(&theme_object_string).unwrap();

        match self {
            ThemeFormatter::Json => theme_object_string,
            ThemeFormatter::Xresources => Self::format_xresources(&data),
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
