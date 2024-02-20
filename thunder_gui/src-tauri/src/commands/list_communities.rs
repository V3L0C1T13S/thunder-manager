use thunder_manager_common::utils::thunderstore::community::{self, ListCommunitiesResponse};

#[tauri::command]
pub async fn list_communities(cursor: Option<&str>) -> Result<ListCommunitiesResponse, String> {
    let communities = community::list_communities(cursor).await;

    if let Ok(communities) = communities {
        Ok(communities)
    } else {
        communities.map_err(|e| e.to_string())
    }
}
