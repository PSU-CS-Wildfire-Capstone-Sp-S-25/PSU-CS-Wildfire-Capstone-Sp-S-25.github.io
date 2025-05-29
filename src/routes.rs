use crate::*;

use axum::{
    Extension, Form,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response, Sse, sse::Event},
};

pub async fn home() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}

// pub async fn publishing -> impl IntoResponse{
//     let template = IndexTemplate{};

// }
