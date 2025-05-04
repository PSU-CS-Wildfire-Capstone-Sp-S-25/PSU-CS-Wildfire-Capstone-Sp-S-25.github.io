use crate::*;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

#[derive(Template)]
#[template(path = "teammate_profile.html")]
pub struct TeammateProfile<'a> {
    fullname: &'a str,
    pronouns: &'a str, // TODO custom type? lol
    role: &'a str,
    description: &'a str,
}
impl<'a> TeammateProfile<'a> {
    fn teammate_profile(profile: &'a Profile) -> Self {
        fullname: Some(profile)
    }
}

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
