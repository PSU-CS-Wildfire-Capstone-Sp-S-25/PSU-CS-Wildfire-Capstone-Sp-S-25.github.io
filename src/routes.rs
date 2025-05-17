use crate::*;

use askama::Template;
use pulldown_cmark::{html, Parser};
use std::{fs, path::PathBuf};

use axum::{
    Extension, Form,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response, Sse, sse::Event},
};

//define how we render out main HTML page
//NavLink is a struct that is used to generate the nav bar
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub nav_links: Vec<NavLink>,
}

// Setting up navLink
pub struct NavLink {
    pub title: String,
    pub url: String,
}

pub async fn home_with_markdown_nav(State(state): State<AppState>) -> impl IntoResponse {

    let markdown_files = list_markdown_files(&state.markdown_dir).unwrap_or_default();

    let nav_links: Vec<NavLink> = markdown_files
        .iter()
        .map(|f| {
            let filename = f.file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned();
            let title = filename.replace('-', " ").to_string();
            let url = format!("/md/{}.html", filename);
            NavLink { title, url }
        })
        .collect();

    let template = IndexTemplate { nav_links };

    HtmlTemplate(template)
}

// handles requests to urls
pub async fn serve_markdown_as_html(Path(filename): Path<String>, State(state): State<AppState>) -> impl IntoResponse {
    if !filename.ends_with(".html") {
        return StatusCode::BAD_REQUEST.into_response();
    }

    let markdown_filename = filename.replace(".html", ".md");

    let markdown_path = state.markdown_dir.join(&markdown_filename);

    match fs::read_to_string(markdown_path) {
        Ok(markdown_content)=> {
            let parser = Parser::new(&markdown_content);

            let mut html_output = String::new();

            html::push_html(&mut html_output, parser);

            Html(html_output).into_response()
        }

        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

fn list_markdown_files(dir: &PathBuf) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md"){
                files.push(path);
            }
        }
    }
    Ok(files)
}