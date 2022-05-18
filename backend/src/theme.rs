use crate::util;
use crate::{cli::Args, template::Template};
use hyper::{header, Body, Response};
use std::path::Path;
use tokio::fs;

pub async fn send_themes_list(themes_path: &Path) -> Response<Body> {
    match fs::read_dir(themes_path).await {
        Ok(mut themes_dir) => {
            let mut themes_vec = Vec::new();

            // Fill themes_vec
            while let Some(theme) = themes_dir.next_entry().await.unwrap() {
                let theme = theme.file_name().into_string().unwrap();
                if let Some(theme_toml) = create_theme_toml(themes_path, &theme).await {
                    themes_vec.push(theme_toml);
                }
            }

            let themes_array = util::convert(toml::Value::Array(themes_vec));

            let json_string = serde_json::to_string(&themes_array).unwrap();

            Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                .body(json_string.into())
                .unwrap()
        }
        Err(err) => {
            // THEME_DIR does not exist
            crate::send_internal_server_error(&err)
        }
    }
}

pub async fn send_theme(args: &Args, theme: &str, template: Template) -> Response<Body> {
    if let Some(theme_toml) = create_theme_toml(&args.themes_path, theme).await {
        let body = template
            .format(&args.templates_path, &theme_toml)
            .await
            .into();
        Response::builder()
            .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .body(body)
            .unwrap()
    } else {
        crate::send_not_found()
    }
}

async fn create_theme_toml(themes_path: &Path, theme: &str) -> Option<toml::Value> {
    let theme_path = Path::new(themes_path).join(theme);
    if let Ok(file_content) = fs::read_to_string(theme_path).await {
        Some(deserialize_theme_toml(theme, &file_content))
    } else {
        None
    }
}

fn deserialize_theme_toml(theme: &str, file_content: &str) -> toml::Value {
    let mut toml: toml::Value = toml::from_str(file_content).unwrap();

    // All themes must have a name entry
    // If the theme already has a name entry, it should be overwritten for consistency
    if let Some(table) = toml.as_table_mut() {
        if table.get("name").is_some() {
            // Overwriting existing name entry
            table["name"] = toml::Value::String(theme.to_string());
        } else {
            // Insert name entry
            table.insert(String::from("name"), toml::Value::String(theme.to_string()));
        }
    }

    toml
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_already_exists() {
        let file_content = r#"
            name = "test"
        "#;

        let theme_toml = deserialize_theme_toml("foo.toml", file_content);
        assert_eq!(
            theme_toml["name"],
            toml::Value::String(String::from("foo.toml"))
        );
    }

    #[test]
    fn name_does_not_exist() {
        let file_content = "";

        let theme_toml = deserialize_theme_toml("bar.toml", file_content);
        assert_eq!(
            theme_toml["name"],
            toml::Value::String(String::from("bar.toml"))
        );
    }
}
