use hyper::{
    header,
    service::{make_service_fn, service_fn},
    Body, Error, Method, Request, Response, Result, Server, StatusCode,
};
use serde_json::{Map, Value};
use std::path::Path;
use tokio::{
    fs::{self, File},
    io::{AsyncBufReadExt, BufReader},
};

const THEME_DIR: &str = "themes";

#[tokio::main]
async fn main() {
    let make_service = make_service_fn(|_| async { Ok::<_, Error>(service_fn(fetch_response)) });

    let addr = ([127, 0, 0, 1], 3001).into();
    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("Server error: {e}");
    }
}

async fn fetch_response(req: Request<Body>) -> Result<Response<Body>> {
    match (req.method(), req.uri().path(), req.uri().query()) {
        (&Method::GET, "/theme", Some(theme)) => Ok(send_theme(theme).await),
        (&Method::GET, "/themes", None) => Ok(send_themes_list().await),
        _ => Ok(send_not_found()),
    }
}

fn send_not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body("Not Found".into())
        .unwrap()
}

async fn send_themes_list() -> Response<Body> {
    let mut themes = fs::read_dir(THEME_DIR).await.unwrap();

    let mut themes_vec = Vec::new();
    while let Some(theme) = themes.next_entry().await.unwrap() {
        themes_vec.push(Value::String(theme.file_name().into_string().unwrap()));
    }
    let body = serde_json::to_string_pretty(&Value::Array(themes_vec))
        .unwrap()
        .into();
    Response::builder()
        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .body(body)
        .unwrap()
}

async fn send_theme(theme: &str) -> Response<Body> {
    // Define file location of theme
    let theme_path = Path::new(THEME_DIR).join(theme);

    if let Ok(file) = File::open(theme_path).await {
        // Map to store the theme's color values
        let mut theme_map = Map::new();

        let mut buf_reader = BufReader::new(file).lines();
        while let Some(line) = buf_reader.next_line().await.unwrap() {
            let line: Vec<&str> = line.split(' ').collect();

            theme_map.insert(
                // Color property name
                line[0].to_string(),
                // Color value
                Value::String(line[1].to_string()),
            );
        }

        // Format JSON and convert to body
        let body = serde_json::to_string_pretty(&Value::Object(theme_map))
            .unwrap()
            .into();

        Response::builder()
            .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .body(body)
            .unwrap()
    } else {
        send_not_found()
    }
}
