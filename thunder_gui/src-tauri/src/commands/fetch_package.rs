use thunder_manager_common::utils::thunderstore::package::{self, FetchPackageResult};

#[tauri::command]
pub async fn fetch_package(name: &str, namespace: &str) -> Result<FetchPackageResult, String> {
    let data = package::fetch_package(name, namespace).await;

    if let Ok(data) = data {
        Ok(data)
    } else {
        data.map_err(|e| e.to_string())
    }
}
