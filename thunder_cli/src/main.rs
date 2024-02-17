use std::env;
use thunder_manager_common::utils::installer::install_file;

#[tokio::main]
async fn main() -> Result<(), ()> {
    if let Some(file_path) = env::args().nth(1) {
        println!("ok so file is {}", file_path);

        install_file(file_path.as_str())
            .await
            .expect("Couldn't install mods");
    }

    Ok(())
}
