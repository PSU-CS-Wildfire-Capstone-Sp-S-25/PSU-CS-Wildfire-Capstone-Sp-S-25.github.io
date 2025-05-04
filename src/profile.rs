use crate::*;

// Defining a struct to represent a teammate profile.
/// A struct to represent a teammate profile.
/// # Fields
///
/// * 'fullname'
/// * 'pronouns'
/// * 'role'
/// * 'description'
#[derive(Clone, Debug)]
pub struct Profile {
    pub fullname: String,
    pub pronouns: String,
    pub role: String,
    pub description: String,
}
// Implementing a new method for the Profile struct
impl Profile {
    /// Creates a new instance of Profile.
    ///
    /// # Arguments
    ///
    /// * 'fullname'
    /// * 'pronouns'
    /// * 'role'
    /// * 'description'
    ///
    /// # Returns
    ///
    /// A new instance of Profile.
    pub fn new(fullname: &str, pronouns: &str, role: &str, description: &str) -> Self {
        let fullname = fullname.into();
        let pronouns = pronouns.into();
        let role = role.into();
        let description = description.into();

        Self {
            fullname,
            pronouns,
            role,
            description,
        }
    }
}

impl IntoResponse for &Profile {
    /// Converts a reference to a Profile into a Response.
    ///
    /// # Returns
    ///
    /// A Response with a status code of OK and the profile data in JSON format.
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(&self)).into_response()
    }
}
