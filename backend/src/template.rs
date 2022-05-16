use hyper::{header, Body, Response};
use std::{
    borrow::Cow,
    error::Error,
    fmt::{self, Display, Formatter},
    io,
    path::Path,
};
use tinytemplate::TinyTemplate;
use tokio::fs;
use url::Url;

const TEMPLATES_DIR: &str = "templates";

#[derive(Debug)]
pub enum TemplateError {
    InvalidTemplate(String),
    Io(io::Error),
}

impl Error for TemplateError {}

impl Display for TemplateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTemplate(s) => write!(f, "Invalid Template: {s}"),
            Self::Io(err) => write!(f, "{err}"),
        }
    }
}

impl From<io::Error> for TemplateError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

async fn fetch_template_files() -> io::Result<Vec<String>> {
    let mut templates_dir = fs::read_dir(TEMPLATES_DIR).await?;
    let mut templates = Vec::new();
    while let Some(template) = templates_dir.next_entry().await.unwrap() {
        let template = template.file_name().into_string().unwrap();
        templates.push(template);
    }
    Ok(templates)
}

pub async fn send_templates_list() -> Response<Body> {
    match fetch_template_files().await {
        Ok(templates) => {
            let mut templates: Vec<serde_json::Value> = templates
                .iter()
                .map(|template| serde_json::Value::String(String::from(template)))
                .collect();

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
    Other(String),
}

impl Default for Template {
    fn default() -> Self {
        Self::Toml
    }
}

impl Template {
    async fn from_str(template: &str) -> Result<Self, TemplateError> {
        match template {
            "toml" => Ok(Template::Toml),
            "json" => Ok(Template::Json),
            template => {
                let template = String::from(template);
                let templates = fetch_template_files().await?;
                if templates.contains(&template) {
                    Ok(Template::Other(template))
                } else {
                    Err(TemplateError::InvalidTemplate(template))
                }
            }
        }
    }

    pub async fn from_url(url: &Url) -> Result<Self, TemplateError> {
        let mut query_pairs = url.query_pairs();
        if let Some(pair) = query_pairs.find(|pair| pair.0 == Cow::Borrowed("template")) {
            Self::from_str(pair.1.as_ref()).await
        } else {
            Ok(Self::default())
        }
    }

    pub async fn format(&self, theme_object: &toml::Value) -> String {
        match self {
            Template::Toml => toml::to_string(theme_object).unwrap(),
            Template::Json => serde_json::to_string(theme_object).unwrap(),
            Template::Other(template) => {
                let mut tt = TinyTemplate::new();
                let template_path = Path::new(TEMPLATES_DIR).join(template);
                let template_text = std::fs::read_to_string(template_path).unwrap();
                tt.add_template(template, &template_text).unwrap();
                tt.render(template, theme_object).unwrap()
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
            Template::from_str(template).await,
            Err(TemplateError::InvalidTemplate(_))
        ));
    }

    #[tokio::test]
    async fn invalid_template_from_url() {
        let url = Url::parse("https://example.com?template=invalid-template").unwrap();

        assert!(matches!(
            Template::from_url(&url).await,
            Err(TemplateError::InvalidTemplate(_))
        ));
    }
}
