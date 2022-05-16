mod template;
mod theme;
mod util;

use hyper::{
    header,
    service::{make_service_fn, service_fn},
    Body, Error, Method, Request, Response, Result, Server, StatusCode,
};
use std::fmt::Display;
use template::Template;
use url::Url;

const ADDRESS: &str = "127.0.0.1:3001";

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

    match (req.method(), path_segments.next(), path_segments.next()) {
        (&Method::GET, Some("themes"), Some(theme)) => match Template::from_url(&url).await {
            Ok(template) => Ok(theme::send_theme(theme, template).await),
            Err(err) => Ok(send_internal_server_error(&err)),
        },
        (&Method::GET, Some("themes"), None) => Ok(theme::send_themes_list().await),
        (&Method::GET, Some("templates"), None) => Ok(template::send_templates_list().await),
        _ => Ok(send_not_found()),
    }
}

pub fn send_not_found() -> Response<Body> {
    Response::builder()
        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .status(StatusCode::NOT_FOUND)
        .body("Not Found".into())
        .unwrap()
}

pub fn send_internal_server_error<T: Display>(custom_message: &T) -> Response<Body> {
    Response::builder()
        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(format!("Internal Server Error: {custom_message}").into())
        .unwrap()
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
    async fn request_with_invalid_template() {
        let request = Request::builder()
            .method(Method::GET)
            .uri("/themes/theme.toml?template=this-template-does-not-exist")
            .body(Body::default())
            .unwrap();

        assert_eq!(
            fetch_response(request).await.unwrap().status(),
            StatusCode::INTERNAL_SERVER_ERROR
        )
    }

    #[tokio::test]
    async fn request_with_no_theme_specified() {
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
}
