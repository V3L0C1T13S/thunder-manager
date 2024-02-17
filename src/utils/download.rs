use std::{
    fs,
    io::{self, Cursor},
    path::{self, PathBuf},
};

use super::files::is_excluded_file;
use crate::{
    model::thunderstore::manifest::ThunderstoreManifest, utils::files::create_dir_all_or_fail,
};

use crate::model::game_mod::GameMod;

pub async fn download_mod(thunderstore_mod: GameMod, container_path: PathBuf) {
    let download_dir = container_path.join("downloads");
    let output_dir = container_path.join("output");
    let workdir = container_path.join("work");

    let file_path = download_dir.join(format!(
        "v{version}-{name}",
        name = thunderstore_mod.file_name,
        version = thunderstore_mod.version
    ));

    // p1. download file (skip if already downloaded)

    if path::Path::new(file_path.to_str().unwrap()).exists() {
        println!(
            "already downloaded {name} - skipping",
            name = thunderstore_mod.file_name
        );
    } else {
        let url = &thunderstore_mod.url;

        println!(
            "Downloading {name} v{version} from url: {url}",
            name = thunderstore_mod.file_name,
            version = thunderstore_mod.version
        );
        let bytes = reqwest::get(url)
            .await
            .unwrap_or_else(|_| panic!("failed to download mod from {url}"))
            .bytes()
            .await
            .expect("couldn't get response data");

        let file: Result<fs::File, io::Error> = fs::File::create(&file_path);

        io::copy(&mut Cursor::new(bytes), &mut file.unwrap()).expect("couldn't write data to file");
        println!(
            "Finished downloading {name}.",
            name = thunderstore_mod.file_name
        );
    }

    // p2 - we've got the file, sweet. now what do we do with it?

    if thunderstore_mod.file_name.ends_with(".zip") {
        let file_name = &thunderstore_mod.file_name;
        let data = fs::read(file_path).expect("couldn't read zip archive.");
        let target_path = PathBuf::from(&workdir).join(file_name);

        // extract files
        println!("Extracting {name}...", name = thunderstore_mod.file_name);
        zip_extract::extract(Cursor::new(data), &target_path, false)
            .unwrap_or_else(|_| panic!("couldn't extract {file_name}"));

        let manifest_path = target_path.join("manifest.json");
        let manifest_data = fs::read_to_string(manifest_path);

        let thunderstore_manifest: Option<ThunderstoreManifest> =
            if let Ok(manifest_data) = manifest_data {
                serde_json::from_str(manifest_data.as_str())
                    .expect("couldn't deserialize manifest data")
            } else {
                None
            };

        if thunderstore_manifest.is_some() {
            println!(
                "Resolved {file_name} to {name}.",
                file_name = thunderstore_mod.file_name,
                name = thunderstore_manifest.as_ref().unwrap().name
            );
        }
        let resolved_dir = match thunderstore_mod
            .mod_type
            .clone()
            .unwrap_or_default()
            .as_str()
        {
            "root" => {
                if thunderstore_mod.root.is_some() {
                    target_path.join(thunderstore_mod.root.unwrap())
                } else {
                    target_path
                }
            }
            _ => {
                if let Some(thunderstore_manifest) = thunderstore_manifest {
                    target_path.join(thunderstore_manifest.name)
                } else {
                    target_path
                }
            }
        };
        assert!(
            path::Path::exists(resolved_dir.as_path()),
            "Couldn't find root dir for {}",
            thunderstore_mod.file_name
        );

        // p3. move files to output dir

        let target_output_dir = if thunderstore_mod.extract_to.is_some() {
            output_dir.join(thunderstore_mod.extract_to.unwrap())
        } else {
            output_dir
        };
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

        create_dir_all_or_fail(
            &target_output_dir,
            target_output_dir.file_name().unwrap().to_str(),
        );
        fs_extra::move_items(&paths, target_output_dir, move_options)
            .expect("failed to move mods to output dir");

        println!("Installed {name}.", name = thunderstore_mod.file_name);
    } else {
        println!("unknown file type");
    }
}
