mod format;
mod util;

use format::ThemeFormat;
use hyper::{
    header,
    service::{make_service_fn, service_fn},
    Body, Error, Method, Request, Response, Result, Server, StatusCode,
};
use std::{fmt::Display, path::Path};
use tokio::fs;
use url::Url;

const ADDRESS: &str = "127.0.0.1:3001";
const THEME_DIR: &str = "themes";

#[tokio::main]
async fn main() -> Result<()> {
    let make_service = make_service_fn(|_| async { Ok::<_, Error>(service_fn(fetch_response)) });

    let addr = ADDRESS.parse().unwrap();

    let server = Server::bind(&addr).serve(make_service);

    server.await?;

    Ok(())
}

async fn fetch_response(req: Request<Body>) -> Result<Response<Body>> {
    let url = Url::parse(&format!("http://{ADDRESS}{}", req.uri())).unwrap();

    let mut path_segments = url.path_segments().unwrap();

    match (req.method(), path_segments.next()) {
        (&Method::GET, Some("theme")) => {
            if let Some(theme) = path_segments.next() {
                // Theme was specified
                match ThemeFormat::try_from(&url) {
                    Ok(format) => Ok(send_theme(theme, format).await),
                    Err(err) => Ok(send_internal_server_error(&err)),
                }
            } else {
                // No theme specified, send not found
                Ok(send_not_found())
            }
        }
        (&Method::GET, Some("themes")) => Ok(send_themes_list().await),
        _ => Ok(send_not_found()),
    }
}

fn send_not_found() -> Response<Body> {
    Response::builder()
        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .status(StatusCode::NOT_FOUND)
        .body("Not Found".into())
        .unwrap()
}

fn send_internal_server_error<T: Display>(custom_message: &T) -> Response<Body> {
    Response::builder()
        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(format!("Internal Server Error: {custom_message}").into())
        .unwrap()
}

async fn send_themes_list() -> Response<Body> {
    match fs::read_dir(THEME_DIR).await {
        Ok(mut themes_dir) => {
            let mut themes_vec = Vec::new();

            // Fill themes_vec
            while let Some(theme) = themes_dir.next_entry().await.unwrap() {
                let theme = theme.file_name().into_string().unwrap();
                if let Some(theme_toml) = create_theme_toml(&theme).await {
                    themes_vec.push(theme_toml);
                }
            }

            let themes_vec = toml::Value::Array(themes_vec);

            let json_string = serde_json::to_string(&util::convert(themes_vec)).unwrap();

            Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                .body(json_string.into())
                .unwrap()
        }
        Err(err) => {
            // THEME_DIR does not exist
            send_internal_server_error(&err)
        }
    }
}

async fn send_theme(theme: &str, formatter: ThemeFormat) -> Response<Body> {
    if let Some(theme_toml) = create_theme_toml(theme).await {
        let body = formatter.run(&theme_toml).into();
        Response::builder()
            .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .body(body)
            .unwrap()
    } else {
        send_not_found()
    }
}

async fn create_theme_toml(theme: &str) -> Option<toml::Value> {
    let theme_path = Path::new(THEME_DIR).join(theme);
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

    #[tokio::test]
    async fn themes() {
        let request = Request::builder()
            .method(Method::GET)
            .uri("/themes")
            .body(Body::default())
            .unwrap();

        assert_eq!(
            fetch_response(request).await.unwrap().status(),
            StatusCode::OK
        );
    }

    #[tokio::test]
    async fn not_found() {
        let request = Request::builder()
            .method(Method::GET)
            .uri("/does-not-exist")
            .body(Body::default())
            .unwrap();

        assert_eq!(
            fetch_response(request).await.unwrap().status(),
            StatusCode::NOT_FOUND
        );
    }

    #[tokio::test]
    async fn invalid_format() {
        let request = Request::builder()
            .method(Method::GET)
            .uri("/theme/theme.toml?format=this-format-does-not-exist")
            .body(Body::default())
            .unwrap();

        assert_eq!(
            fetch_response(request).await.unwrap().status(),
            StatusCode::INTERNAL_SERVER_ERROR
        )
    }

    #[tokio::test]
    async fn no_theme_specified() {
        let request = Request::builder()
            .method(Method::GET)
            .uri("/theme")
            .body(Body::default())
            .unwrap();

        assert_eq!(
            fetch_response(request).await.unwrap().status(),
            StatusCode::NOT_FOUND
        );
    }

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
