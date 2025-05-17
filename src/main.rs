#![warn(clippy::all)]

mod templates;
use templates::*;
mod routes;
use routes::*;

use anyhow::Context;
use askama::Template;
use axum::{
    // Handle requests to our web app.
    Json,
    Router,
    extract::{Path, State},
    http::{Method, StatusCode, Uri, request::Parts},
    response::{Html, IntoResponse, Response},
    routing::{delete, get, post, put},
};
use clap::Parser; // Parse command line arguments.
use std::net::SocketAddr;
use tokio::{self, net::TcpListener, sync::RwLock};
use tower_http::{cors, services::ServeDir};

/// Attempting to look at pulldown
use pulldown_cmark::{html, Parser as MarkdownParser};
use std::{fs, path::PathBuf};

/// Represents the arguments passed to the program. TODO add command line args to app :P
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The IP address and port to serve the application on. Defaults to 0.0.0.0:3000.
    #[clap(short, long, default_value = "0.0.0.0:3000")]
    serve: String,
}

#[derive(Clone)]
pub struct AppState {
    markdown_dir: PathBuf
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let serve_addr = args.serve.parse::<SocketAddr>()?;

    let markdown_dir = PathBuf::from("resources/markdown");

    // Defining where the markdown files go
    let app_state = AppState { markdown_dir };

    let cors = cors::CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(cors::Any);

    let app = Router::new()
        .route("/", get(routes::home_with_markdown_nav))
        .route("/md/{filename}", get(routes::serve_markdown_as_html))
        .nest_service("/resources", ServeDir::new("resources"))
        .fallback(fallback)
        .with_state(app_state)
        .layer(cors);

    //let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(serve_addr).await?;
    println!("listening on {}", serve_addr);

    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

/// Returns a html document representing a 404 page, and NOT_FOUND status code.
async fn fallback(uri: Uri) -> Response {
    println!("CTEST - fallback, uri: {:#?}", uri);
    (
        StatusCode::NOT_FOUND,
        Html(include_str!("../resources/html/404.html")),
    )
        .into_response()
}


