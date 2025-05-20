use crate::*;

use comrak::{Options, markdown_to_html};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{collections::HashMap, fs, path::Path}; // For storing a collection of Profiles; Make sure fs and Path are imported // Make sure these are imported
type Profiles = HashMap<String, Profile>; // TODO good K type?

// type html_Profiles = HashMap<String, String>; // K is identifier string, V is strings containing HTML representing a profile.

// Defining a struct to represent a teammate profile.
/// A struct to represent a teammate profile.
/// # Fields
///
/// * 'fullname'
/// * 'pronouns'
/// * 'role'
/// * 'description'
#[derive(Clone, Debug, Deserialize, Serialize)]
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
        let j = Json(&self);
        (StatusCode::OK, j).into_response();
    }
}

/// Produce `Profiles` from the MarkDown in `/resources/profiles`.
///
/// # Returns
///
/// A collection of `Profile` structs.
pub async fn load_profiles_from_md_files() -> anyhow::Result<Profiles> {
    let mut profiles = Profiles::new();
    let profile_dir = Path::new("PSU-CS-Wildfire-Capstone-Sp-S-25.github.io/resources/profiles");

    if profile_dir.is_dir() {
        for entry in fs::read_dir(profile_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
                let file_content = fs::read_to_string(&path)?;
                // let parse_result = parse_markdown_profile_to_html(&file_content);
                // let key =

                // match parse_markdown_profile(&file_content) {
                //     Ok(profile) => {
                //         if let Some(filename_stem) = path.file_stem().and_then(|s| s.to_str()) {
                //             profiles.insert(filename_stem.to_string(), profile);
                //         }
                //     }
                //     Err(e) => {
                //         // Log or handle parsing errors for individual files
                //         eprintln!("Failed to parse profile from {:?}: {}", path, e);
                //     }
                // }
            }
        }
    }
    Ok(profiles)
}

// HELPERS

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Read a txt containing a member profile and populate a Profile for it.
fn read_profile_txt<P>(filename: P) -> Result<Profile, &'static str>
where
    P: AsRef<Path>,
{
    if let Ok(lines) = read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            println!("{}", line);
        }
    }
    Err("not implemented")
}
