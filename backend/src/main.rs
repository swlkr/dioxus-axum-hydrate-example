use anyhow::Result;
use axum::{
    http::StatusCode,
    response::Html,
    routing::{get, get_service},
    Router,
};
use dioxus::prelude::*;
use frontend::app::app;
use std::{env, net::SocketAddr};
use tokio::fs;
use tower_http::services::ServeDir;

const DIST: &str = "frontend/dist";
const ASSETS: &str = "frontend/dist/assets";

#[tokio::main]
async fn main() -> Result<()> {
    let addr: SocketAddr = env::var("SERVER_ADDR")?.parse()?;
    println!("listening on http://{}", addr);

    let serve_dir = get_service(ServeDir::new(ASSETS)).handle_error(|_| async move {
        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
    });
    let app = Router::new()
        .route("/", get(app_endpoint))
        .nest_service("/assets", serve_dir);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    return Ok(());
}

async fn app_endpoint() -> Html<String> {
    let index = fs::read_to_string(format!("{DIST}/index.html"))
        .await
        .unwrap();
    let (prefix, suffix) = index.split_once(r#"<div id="main">"#).unwrap();

    let mut app = VirtualDom::new(app);
    let _ = app.rebuild();

    let html = dioxus_ssr::pre_render(&app);
    Html(format!(r#"{prefix}<div id="main">{html}{suffix}"#))
}
