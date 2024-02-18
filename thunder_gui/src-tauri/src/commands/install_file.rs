use thunder_manager_common::utils::installer::{install_from_contents, ContainerFileType};

#[tauri::command]
pub async fn install_file(contents: String, file_type: ContainerFileType) -> Result<(), String> {
    if install_from_contents(contents, file_type).await.is_ok() {
        Ok(())
    } else {
        Err(String::from("Couldn't create container"))
    }
}
