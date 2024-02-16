use std::{env, fs, path::PathBuf};

use crate::{model::mod_container::ModContainer, utils::download::download_mod};

mod constants;
mod model;
mod utils;

fn create_dir_all_or_fail<P: AsRef<std::path::Path>>(path: P, name: Option<&str>) {
    fs::create_dir_all(path).expect(
        format!(
            "failed to create path {name}",
            name = name.unwrap_or_default()
        )
        .as_str(),
    )
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args: Vec<_> = env::args().collect();
    dbg!(args);

    if let Some(file_path) = env::args().nth(1) {
        println!("ok so file is {}", file_path);

        let contents = fs::read_to_string(&file_path).expect(format!("file not found").as_str());

        println!("contents: {}", contents);

        let container: ModContainer = if file_path.as_str().ends_with(".json") {
            serde_json::from_str(contents.as_str()).expect("couldn't parse json")
        } else {
            serde_yaml::from_str(contents.as_str()).expect("couldn't parse yaml")
        };

        let container_path = PathBuf::from(constants::MODS_DIR).join(&container.name);
        let download_dir = &container_path.join("downloads");
        let output_dir = &container_path.join("output");
        let workdir = &container_path.join("work");

        println!("container name: {}", &container.name);
        println!("container version: {}", &container.version.unwrap_or(0));
        create_dir_all_or_fail(&container_path, Some("container"));
        create_dir_all_or_fail(&download_dir, Some("download"));
        create_dir_all_or_fail(&output_dir, Some("output"));
        create_dir_all_or_fail(&workdir, Some("work"));

        let tasks: Vec<_> = container
            .mods
            .clone()
            .into_iter()
            .map(|thunderstore_mod| {
                tokio::spawn(download_mod(thunderstore_mod, container_path.clone()))
            })
            .collect();

        for task in tasks {
            task.await.expect("failed to run task");
        }

        println!(
            "ok ur mods are in \"{}\" kthxbye",
            output_dir.to_str().unwrap_or_default()
        );
    }

    Ok(())
}
