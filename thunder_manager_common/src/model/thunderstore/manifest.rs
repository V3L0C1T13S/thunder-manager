use serde::Deserialize;

#[derive(Deserialize)]
pub struct ThunderstoreManifest {
    pub name: String,
    pub version_number: String,
    pub website_url: String,
    pub description: String,
    // pub dependencies: Vec<String>, // TODO
}
