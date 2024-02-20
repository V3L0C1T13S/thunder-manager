use thunder_manager_common::utils::thunderstore::community::{
    fetch_community_packages, FetchCommunityPackagesResponse,
};

#[tauri::command]
pub async fn fetch_community(identifier: &str) -> Result<FetchCommunityPackagesResponse, ()> {
    let data = fetch_community_packages(identifier).await;

    if let Ok(data) = data {
        Ok(data)
    } else {
        Err(())
    }
}
