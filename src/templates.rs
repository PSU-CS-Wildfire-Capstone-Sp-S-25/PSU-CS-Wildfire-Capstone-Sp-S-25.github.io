use crate::*;
use askama::Template;

use axum::response::{IntoResponse, Response, Html};
use axum::http::StatusCode;


/// A wrapper type that we'll use to encapsulate HTML parsed by askama into valid HTML for axum to serve.
pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
