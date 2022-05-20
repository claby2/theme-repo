mod cli;
mod template;
mod theme;
mod util;

use clap::Parser;
use cli::Args;
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
    let args = Args::parse();

    let make_service = make_service_fn(|_| {
        let args = args.clone();
        async { Ok::<_, Error>(service_fn(move |req| fetch_response(req, args.to_owned()))) }
    });

    let addr = ADDRESS.parse().unwrap();

    let server = Server::bind(&addr).serve(make_service);

    server.await?;

    Ok(())
}

async fn fetch_response(req: Request<Body>, args: Args) -> Result<Response<Body>> {
    let url = Url::parse(&format!("http://{ADDRESS}{}", req.uri())).unwrap();

    let path_segments: Vec<&str> = url.path_segments().unwrap().collect();

    match (req.method(), &path_segments[..]) {
        (&Method::GET, ["themes", theme]) => {
            // Return a specific theme
            match Template::from_url(&args.templates_path, &url).await {
                Ok(template) => Ok(theme::send_theme(&args, theme, template).await),
                Err(err) => Ok(send_internal_server_error(&err)),
            }
        }

        (&Method::GET, ["themes"]) => {
            // Return all themes
            Ok(theme::send_themes_list(&args.themes_path).await)
        }

        (&Method::GET, ["templates"]) => {
            // Return an array of templates
            Ok(template::send_templates_list(&args.templates_path).await)
        }

        _ => Ok(send_not_found()),
    }
}

/// Sends a NOT_FOUND status code
pub fn send_not_found() -> Response<Body> {
    Response::builder()
        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .status(StatusCode::NOT_FOUND)
        .body("Not Found".into())
        .unwrap()
}

/// Sends an internal server error response with a custom message
pub fn send_internal_server_error<T: Display>(custom_message: &T) -> Response<Body> {
    Response::builder()
        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(format!("Internal Server Error: {custom_message}").into())
        .unwrap()
}
