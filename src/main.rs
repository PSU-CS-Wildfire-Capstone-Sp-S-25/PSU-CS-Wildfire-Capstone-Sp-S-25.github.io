#![warn(clippy::all)]

mod templates;
use templates::*;
mod routes;
use routes::*;

// mod web;

// use web::*;

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

/// Represents the arguments passed to the program.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The IP address and port to serve the application on. Defaults to 0.0.0.0:3000.
    #[clap(short, long, default_value = "0.0.0.0:3000")]
    serve: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let args = Args::parse();

    let cors = cors::CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(cors::Any);

    let app = Router::new()
        .route("/", get(routes::home))
        .nest_service("/resources", ServeDir::new("resources")) // Serves anything requested from /assets;
        .fallback(fallback)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("listening on {}", addr);

    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
    // // startup(args.serve).await
}

// async fn handler() -> Html<&'static str> {
//     // `std::include_str` macro can be used to include an utf-8 file as `&'static str` in compile
//     // time. This method is relative to current `main.rs` file.
//     Html(include_str!("../index.html"))
// }

// pub async fn startup(ip: String) {
//     // TODO handle CORS issues? using cors::CorsLayer::new()...

//     // Initializes a new router for the web server app.
//     // Includes routes for the home page and serving static assets from "/resources".
//     let app = Router::new()
//         .route("/", get(handler_index))
//         // .route(path, method_router)
//         .nest_service("/resouces", ServeDir::new("resources")) // Serves anything requested from /assets
//         .fallback(fallback);
//     // .layer(cors)
//     let listener = tokio::net::TcpListener::bind(ip.clone()).await.unwrap();
//     println!("Listening on {}", ip);
//     axum::serve(listener, app).await.unwrap();
// }

/// Returns a html document representing a 404 page, and NOT_FOUND status code.
async fn fallback(uri: Uri) -> Response {
    println!("CTEST - fallback, uri: {:#?}", uri);
    (
        StatusCode::NOT_FOUND,
        Html(include_str!("../resources/html/404.html")),
    )
        .into_response()
}
