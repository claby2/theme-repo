use hyper::{header, Body, Response};
use serde_json::Error as JsonError;
use std::{
    borrow::Cow,
    error::Error,
    fmt::{self, Display, Formatter},
    io,
    path::Path,
};
use tinytemplate::{error::Error as TinyTemplateError, TinyTemplate};
use tokio::fs;
use toml::ser::Error as TomlError;
use url::Url;

#[derive(Debug)]
pub enum TemplateError {
    InvalidTemplate(String),
    Io(io::Error),
    TinyTemplate(TinyTemplateError),
    Toml(TomlError),
    Json(JsonError),
}

impl Error for TemplateError {}

impl Display for TemplateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTemplate(s) => write!(f, "Invalid Template: {s}"),
            Self::Io(err) => write!(f, "Io Error: {err}"),
            Self::TinyTemplate(err) => write!(f, "TinyTemplate Error: {err}"),
            Self::Toml(err) => write!(f, "TOML Error: {err}"),
            Self::Json(err) => write!(f, "JSON Error: {err}"),
        }
    }
}

impl From<io::Error> for TemplateError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<TinyTemplateError> for TemplateError {
    fn from(err: TinyTemplateError) -> Self {
        Self::TinyTemplate(err)
    }
}

impl From<TomlError> for TemplateError {
    fn from(err: TomlError) -> Self {
        Self::Toml(err)
    }
}

impl From<JsonError> for TemplateError {
    fn from(err: JsonError) -> Self {
        Self::Json(err)
    }
}

/// Retrieves a list of locally-stored templates
async fn fetch_template_files(templates_path: &Path) -> io::Result<Vec<String>> {
    let mut templates_dir = fs::read_dir(templates_path).await?;
    let mut templates = Vec::new();

    // Iterate through entries in templates directory and push them into templates vector
    while let Some(template) = templates_dir.next_entry().await? {
        if let Ok(template) = template.file_name().into_string() {
            templates.push(template);
        }
    }

    Ok(templates)
}

/// Sends a JSON list of all available templates
pub async fn send_templates_list(templates_path: &Path) -> Response<Body> {
    match fetch_template_files(templates_path).await {
        Ok(templates) => {
            let mut templates: Vec<serde_json::Value> = templates
                .iter()
                .map(|template| serde_json::Value::String(String::from(template)))
                .collect();

            // JSON and TOML templates are builtin templates, so they must be appended manually
            templates.append(&mut vec![
                serde_json::Value::String(String::from("json")),
                serde_json::Value::String(String::from("toml")),
            ]);

            let templates_array = serde_json::Value::Array(templates);

            let json_string = serde_json::to_string(&templates_array).unwrap();

            Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                .body(json_string.into())
                .unwrap()
        }
        Err(err) => crate::send_internal_server_error(&err),
    }
}

pub enum Template {
    Toml,
    Json,

    // Holds name of template in templates directory
    Custom(String),
}

impl Default for Template {
    fn default() -> Self {
        Self::Toml
    }
}

impl Template {
    /// Converts a string slice into a template.
    async fn from_str(templates_path: &Path, template: &str) -> Result<Self, TemplateError> {
        match template {
            "toml" => Ok(Template::Toml),
            "json" => Ok(Template::Json),
            template => {
                let template = String::from(template);
                let templates = fetch_template_files(templates_path).await?;
                if templates.contains(&template) {
                    Ok(Template::Custom(template))
                } else {
                    // The given template string does not correspond with an existing file
                    Err(TemplateError::InvalidTemplate(template))
                }
            }
        }
    }

    /// Converts a URL into a template.
    pub async fn from_url(templates_path: &Path, url: &Url) -> Result<Self, TemplateError> {
        let mut query_pairs = url.query_pairs();
        if let Some(pair) = query_pairs.find(|pair| pair.0 == Cow::Borrowed("template")) {
            Self::from_str(templates_path, pair.1.as_ref()).await
        } else {
            Ok(Self::default())
        }
    }

    /// Formats a theme toml value according to a template.
    pub async fn format(
        &self,
        templates_path: &Path,
        theme_object: &toml::Value,
    ) -> Result<String, TemplateError> {
        match self {
            Template::Toml => Ok(toml::to_string(theme_object)?),
            Template::Json => Ok(serde_json::to_string(theme_object)?),
            Template::Custom(template) => {
                // Get template as string from file
                let template_path = Path::new(templates_path).join(template);
                let template_text = std::fs::read_to_string(template_path)?;

                // Perform template render
                let mut tt = TinyTemplate::new();
                tt.add_template(template, &template_text)?;
                Ok(tt.render(template, theme_object)?)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn invalid_template_from_str() {
        let template = "invalid-template";

        assert!(matches!(
            Template::from_str(Path::new("./templates"), template).await,
            Err(TemplateError::InvalidTemplate(_))
        ));
    }

    #[tokio::test]
    async fn invalid_template_from_url() {
        let url = Url::parse("https://example.com?template=invalid-template").unwrap();

        assert!(matches!(
            Template::from_url(Path::new("./templates"), &url).await,
            Err(TemplateError::InvalidTemplate(_))
        ));
    }
}
