use std::fs;

pub fn is_excluded_file(name: &str) -> bool {
    let excluded_files = ["icon.png", "manifest.json", "README.md"];

    excluded_files.into_iter().any(|v| name == v)
}

pub fn create_dir_all_or_fail<P: AsRef<std::path::Path>>(path: P, name: Option<&str>) {
    fs::create_dir_all(path).expect(
        format!(
            "failed to create path {name}",
            name = name.unwrap_or_default()
        )
        .as_str(),
    )
}
