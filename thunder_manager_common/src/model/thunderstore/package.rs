use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum StringOri32 {
    String(String),
    I32(i32)
}

pub type RatingScore = StringOri32;
pub type DownloadCount = StringOri32;

#[derive(Deserialize, Serialize)]
pub struct PackageVersionExperimental {
    namespace: Option<String>,
    name: String,
    version_number: String,
    full_name: Option<String>,
    description: String,
}

#[derive(Deserialize, Serialize)]
pub struct ExperimentalPackage {
    pub namespace: Option<String>,
    pub name: String,
    pub full_name: Option<String>,
    pub owner: Option<String>,
    pub package_url: Option<String>,
    pub date_created: Option<String>,
    pub date_updated: Option<String>,
    pub rating_score: Option<RatingScore>,
    pub is_pinned: Option<bool>,
    pub is_deprecated: Option<bool>,
    pub total_downloads: Option<DownloadCount>,
    pub latest: PackageVersionExperimental,
}