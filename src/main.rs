use std::{
    env, fs,
    io::{self, Cursor},
    path::{self, PathBuf},
};

use crate::{
    model::{mod_container::ModContainer, thunderstore::manifest::ThunderstoreManifest},
    utils::files::is_excluded_file,
};

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

        let tasks: Vec<_> = container.mods.clone().into_iter().map(|thunderstore_mod| async move {
            let file_path = download_dir.join(format!(
                "v{version}-{name}",
                name = &thunderstore_mod.file_name,
                version = &thunderstore_mod.version
            ));

            // p1. download file (skip if already downloaded)

            if path::Path::new(file_path.to_str().unwrap()).exists() {
                println!(
                    "already downloaded {name} - skipping",
                    name = &thunderstore_mod.file_name
                );
            } else {
                let url = &thunderstore_mod.url;

                println!(
                    "version: {version}, url: {url}",
                    version = &thunderstore_mod.version
                );
                let bytes = reqwest::get(url)
                    .await
                    .expect(format!("failed to download mod from {url}").as_str())
                    .bytes()
                    .await
                    .expect("couldn't get response data");

                let file: Result<fs::File, io::Error> =
                    Ok(fs::File::create(&file_path).expect("couldn't open file"));

                io::copy(&mut Cursor::new(bytes), &mut file.unwrap())
                    .expect("couldn't write data to file");
            }

            // p2 - we've got the file, sweet. now what do we do with it?

            if thunderstore_mod.file_name.ends_with(".zip") {
                println!("zip");

                let file_name = &thunderstore_mod.file_name;
                let data = fs::read(file_path).expect("couldn't read zip archive.");
                let target_path = PathBuf::from(&workdir).join(file_name);
                // extract files
                zip_extract::extract(Cursor::new(data), &target_path, false)
                    .expect(format!("couldn't extract {file_name}").as_str());

                let manifest_path = target_path.join("manifest.json");
                let manifest_data =
                    fs::read_to_string(manifest_path).expect("couldn't load manifest.json");
                let thunderstore_manifest: ThunderstoreManifest =
                    serde_json::from_str(manifest_data.as_str())
                        .expect("couldn't deserialize manifest.json");

                println!(
                    "ok we got the {name} mod",
                    name = &thunderstore_manifest.name
                );
                let resolved_dir = match thunderstore_mod
                    .mod_type
                    .clone()
                    .unwrap_or_default()
                    .as_str()
                {
                    "root" => target_path,
                    _ => target_path.join(&thunderstore_manifest.name),
                };
                assert_eq!(path::Path::exists(resolved_dir.as_path()), true, "Could not resolve mod directory. The manifest.json may be corrupt or incorrectly formatted.");

                let target_output_dir = &output_dir;
                // p3. move files to output dir
                let entries = fs::read_dir(resolved_dir)
                    .expect("couldn't read file")
                    .filter(|v: &Result<fs::DirEntry, io::Error>| {
                        !is_excluded_file(
                            v.as_ref()
                                .expect("invalid file")
                                .file_name()
                                .to_str()
                                .unwrap_or_default(),
                        )
                    })
                    .map(|v| v.expect("invalid file").path());

                let move_options = &fs_extra::dir::CopyOptions::new().overwrite(true);
                let paths = std::vec::Vec::from_iter(entries);
                fs_extra::move_items(&paths, target_output_dir, move_options)
                    .expect("failed to move mods to output dir");
            } else {
                println!("unknown");
            }
        })
        .collect();

        for task in tasks {
            task.await;
        }

        println!(
            "ok ur mods are in \"{}\" kthxbye",
            output_dir.to_str().unwrap_or_default()
        );
    }

    Ok(())
}
