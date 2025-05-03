use crate::*;

#[derive(Template)]
#[template(path = "index.html")]

/// Handles the index page request.
///
///

/// Handles the index page request.
///
/// This function retrieves a random question from the question base and returns an `IndexTemplate`
/// with the question or an error message, if any.
///
/// # Arguments
///
/// * `question_base` - A state reference to a question base.
///
/// # Returns
///
/// A `Response` containing an `IndexTemplate` with the question or an error message.
pub async fn handler_index(State(question_base): State<Arc<RwLock<QuestionBase>>>) -> Response {
    // Attempt to retrieve a random question from the question base
    match question_base.read().await.get_random().await {
        // If successful, return an `IndexTemplate` with the question and a status code of OK
        Ok(question) => (StatusCode::OK, IndexTemplate::question(&question)).into_response(),
        // If an error occurs, return an `IndexTemplate` with the error message and a status code of NOT_FOUND
        Err(e) => (
            StatusCode::NOT_FOUND,
            IndexTemplate::error(e.to_string()),
            // Html(include_str!("../assets/static/404.html")),
        )
            .into_response(),
    }
}
