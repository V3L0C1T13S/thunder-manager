use crate::model::thunderstore::package::ExperimentalPackage;

pub type FetchPackageResult = ExperimentalPackage;
pub type FetchPackageError = reqwest::Error;

pub async fn fetch_package(
    name: &str,
    namespace: &str,
) -> Result<FetchPackageResult, FetchPackageError> {
    let url = format!("https://thunderstore.io/api/experimental/package/{namespace}/{name}");
    let data = reqwest::get(url)
        .await?
        .json::<ExperimentalPackage>()
        .await?;

    Ok(data)
}
